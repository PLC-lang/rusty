use std::{ops::Range, path::Path};

use inkwell::{
    basic_block::BasicBlock,
    context::Context,
    debug_info::{
        AsDIScope, DIBasicType, DICompileUnit, DICompositeType, DIDerivedType, DIFile, DIFlags,
        DIFlagsConstants, DILocalVariable, DIScope, DISubprogram, DISubroutineType, DIType,
        DWARFEmissionKind, DebugInfoBuilder,
    },
    module::Module,
    targets::TargetData,
    types::BasicTypeEnum,
    values::{BasicMetadataValueEnum, FunctionValue, GlobalValue, PointerValue},
};
use rustc_hash::{FxHashMap, FxHashSet};

use plc_ast::ast::LinkageType;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;

use crate::{
    index::{Index, PouIndexEntry, VariableIndexEntry},
    typesystem::{
        DataType, DataTypeInformation, Dimension, StringEncoding, TypeSize, CHAR_TYPE, VOID_INTERNAL_NAME,
        WCHAR_TYPE,
    },
    DebugLevel, OptimizationLevel,
};

use super::{
    generators::{llvm::Llvm, statement_generator::FunctionContext, ADDRESS_SPACE_GLOBAL},
    llvm_index::LlvmTypedIndex,
};

#[derive(PartialEq, Eq)]
#[allow(non_camel_case_types)]
/// Represents the DWARF (attribute) encodings for basic types
enum DebugEncoding {
    // DW_ATE_address = 0x01,
    DW_ATE_boolean = 0x02,
    DW_ATE_float = 0x04,
    DW_ATE_signed = 0x05,
    DW_ATE_unsigned = 0x07,
    DW_ATE_UTF = 0x10,
}

impl From<DebugLevel> for DWARFEmissionKind {
    fn from(level: DebugLevel) -> Self {
        match level {
            DebugLevel::Full(_) | DebugLevel::VariablesOnly(_) => DWARFEmissionKind::Full,
            _ => DWARFEmissionKind::None,
        }
    }
}

/// A trait that represents a Debug builder
/// An implementor of this trait will be called during various codegen phases to generate debug
/// information
pub trait Debug<'ink> {
    /// Set the debug info source location of the instruction currently pointed at by the builder
    fn set_debug_location(
        &self,
        llvm: &Llvm,
        scope: &FunctionContext,
        //Current line starts with 0
        line: usize,
        column: usize,
    );

    //Unsets the current debug location allowing the debug info to be skipped for variable
    //initializations
    fn unset_debug_location(&self, llvm: &Llvm);

    /// Registers a new function for debugging, this method is responsible for registering a
    /// function's stub as well as its interface (variables/parameters)
    fn register_function<'idx>(
        &mut self,
        indices: (&Index, &LlvmTypedIndex<'ink>),
        func: &FunctionContext<'ink, 'idx>,
        return_type: Option<&'idx DataType>,
        parent_function: Option<FunctionValue<'ink>>,
        parameter_types: &[&'idx DataType],
        implementation_start: usize,
    );

    /// Registers a new datatype for debugging
    fn register_debug_type<'idx>(
        &mut self,
        name: &str,
        datatype: &'idx DataType,
        index: &'idx Index,
        types_index: &'idx LlvmTypedIndex,
    ) -> Result<(), Diagnostic>;

    /// Creates a globally accessible variable with the given datatype.
    fn create_global_variable(
        &mut self,
        name: &str,
        type_name: &str,
        global_variable: GlobalValue<'ink>,
        location: &SourceLocation,
    );

    /// Creates a locally accessible variable.
    fn register_local_variable(
        &mut self,
        variable: &VariableIndexEntry,
        alignment: u32,
        scope: &FunctionContext<'ink, '_>,
    );

    /// Creates a debug entry for a function parameter
    fn register_parameter(
        &mut self,
        variable: &VariableIndexEntry,
        arg_no: usize,
        scope: &FunctionContext<'ink, '_>,
    );

    /// Create the debug entry for an Function POU entry
    fn register_struct_parameter(&mut self, pou: &str, scope: &FunctionContext<'ink, '_>);

    fn add_variable_declaration(
        &self,
        name: &str,
        value: PointerValue<'ink>,
        scope: &FunctionContext,
        block: BasicBlock<'ink>,
        line: usize,
        column: usize,
    );

    /// When code generation is done, this method needs to be called to ensure the inner LLVM state
    /// of the debug builder has been finalized.
    fn finalize(&self);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DebugType<'ink> {
    Basic(DIBasicType<'ink>),
    Struct(DICompositeType<'ink>),
    Derived(DIDerivedType<'ink>),
    Composite(DICompositeType<'ink>),
}

impl<'ink> From<DebugType<'ink>> for DIType<'ink> {
    fn from(t: DebugType<'ink>) -> Self {
        match t {
            DebugType::Basic(t) => t.as_type(),
            DebugType::Struct(t) => t.as_type(),
            DebugType::Derived(t) => t.as_type(),
            DebugType::Composite(t) => t.as_type(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableKey {
    name: String,
    parent: Option<String>,
}

impl VariableKey {
    pub fn new(name: &str, parent: Option<&str>) -> Self {
        Self { name: name.to_string(), parent: parent.map(|it| it.to_string()) }
    }
}

/// Represents the debug builder and information for a compilation unit.
pub struct DebugBuilder<'ink> {
    context: &'ink Context,
    debug_info: DebugInfoBuilder<'ink>,
    compile_unit: DICompileUnit<'ink>,
    /// Registered debug types, keyed by lowercased type name.
    types: FxHashMap<String, DebugType<'ink>>,
    /// Tracks types currently being processed to detect and handle recursive type definitions.
    /// When a type references itself (e.g., a struct containing a pointer to itself), we detect
    /// this cycle by checking if the type is already in this set. If so, we return a forward
    /// declaration instead of recursing infinitely.
    processing: FxHashSet<String>,
    variables: FxHashMap<VariableKey, DILocalVariable<'ink>>,
    optimization: OptimizationLevel,
    files: FxHashMap<&'static str, DIFile<'ink>>,
    target_data: TargetData,
}

/// A wrapper that redirects to correct debug builder implementation based on the debug context.
/// It internally holds a DebugBuilder to do the actual actions, but abstacts it from the caller by
/// implementing the Debug trait
pub enum DebugBuilderEnum<'ink> {
    None,
    VariablesOnly(DebugBuilder<'ink>),
    Full(DebugBuilder<'ink>),
}

impl<'ink> DebugBuilderEnum<'ink> {
    pub fn new(
        context: &'ink Context,
        module: &Module<'ink>,
        root: Option<&Path>,
        optimization: OptimizationLevel,
        debug_level: DebugLevel,
    ) -> Self {
        match debug_level {
            DebugLevel::None => DebugBuilderEnum::None,
            DebugLevel::VariablesOnly(version) | DebugLevel::Full(version) => {
                let dwarf_version: BasicMetadataValueEnum<'ink> =
                    context.i32_type().const_int(version as u64, false).into();
                module.add_metadata_flag(
                    "Dwarf Version",
                    inkwell::module::FlagBehavior::Warning,
                    context.metadata_node(&[dwarf_version]),
                );
                // `LLVMParseIRInContext` expects "Debug Info Version" metadata, with the specified version
                // matching the LLVM version or otherwise it will emit a warning and strip DI from the IR.
                // These metadata flags are not mutually exclusive.
                let dwarf_version: BasicMetadataValueEnum<'ink> = context
                    .i32_type()
                    .const_int(inkwell::debug_info::debug_metadata_version() as u64, false)
                    .into();
                module.add_metadata_flag(
                    "Debug Info Version",
                    inkwell::module::FlagBehavior::Warning,
                    context.metadata_node(&[dwarf_version]),
                );

                let path = Path::new(module.get_source_file_name().to_str().unwrap_or("")).to_path_buf();
                let root = root.unwrap_or_else(|| Path::new(""));
                let filename = &path.strip_prefix(root).unwrap_or(&path).to_str().unwrap_or_default();
                let (debug_info, compile_unit) = module.create_debug_info_builder(
                    true,
                    inkwell::debug_info::DWARFSourceLanguage::C, //TODO: Own lang
                    filename,
                    root.to_str().unwrap_or_default(),
                    "RuSTy Structured text Compiler",
                    optimization.is_optimized(),
                    "",
                    0,
                    "",
                    debug_level.into(),
                    0,
                    false,
                    false,
                    "",
                    "",
                );

                let data_layout = module.get_data_layout();
                let data_layout = data_layout.as_str().to_str().expect("Data layout is valid");
                let target_data = TargetData::create(data_layout);
                let dbg_obj = DebugBuilder {
                    context,
                    debug_info,
                    compile_unit,
                    types: Default::default(),
                    processing: Default::default(),
                    variables: Default::default(),
                    optimization,
                    files: Default::default(),
                    target_data,
                };
                match debug_level {
                    DebugLevel::VariablesOnly(_) => DebugBuilderEnum::VariablesOnly(dbg_obj),
                    DebugLevel::Full(_) => DebugBuilderEnum::Full(dbg_obj),
                    _ => unreachable!("Only variables or full debug can reach this"),
                }
            }
        }
    }
}

impl<'ink> DebugBuilder<'ink> {
    fn register_concrete_type(&mut self, name: &str, di_type: DebugType<'ink>) {
        self.types.insert(name.to_lowercase(), di_type);
    }

    // Apply `DW_TAG_const_type` wrapper for constant variables
    fn apply_const_type_if_needed(&self, debug_type: DIType<'ink>, is_constant: bool) -> DIType<'ink> {
        if is_constant {
            let const_type =
                self.debug_info.create_reference_type(debug_type, 38 /* DW_TAG_const_type */);
            const_type.as_type()
        } else {
            debug_type
        }
    }

    fn create_basic_type(
        &mut self,
        name: &str,
        size: u64,
        encoding: DebugEncoding,
        location: &SourceLocation,
    ) -> Result<(), Diagnostic> {
        let res = self
            .debug_info
            .create_basic_type(name, size, encoding as u32, DIFlagsConstants::PUBLIC)
            .map_err(|err| Diagnostic::codegen_error(err.to_string(), location))?;
        self.register_concrete_type(name, DebugType::Basic(res));
        Ok(())
    }

    fn create_struct_type(
        &mut self,
        name: &str,
        members: &[VariableIndexEntry],
        location: &SourceLocation,
        index: &Index,
        types_index: &LlvmTypedIndex,
    ) -> Result<(), Diagnostic> {
        if location.is_internal() {
            return Ok(());
        }

        let struct_type = match types_index.get_associated_type(name)? {
            BasicTypeEnum::StructType(value) => value,
            _ => self.context.opaque_struct_type(name),
        };

        // Can't compute offsets for opaque structs - register as forward declaration
        if struct_type.is_opaque() {
            self.register_concrete_type(name, self.create_forward_declaration(name));
            return Ok(());
        }

        let file = location
            .get_file_name()
            .map(|it| self.get_or_create_debug_file(it))
            .unwrap_or_else(|| self.compile_unit.get_file());

        let super_ty_name = index.find_pou(name).and_then(|it| it.get_super_class());

        // Collect member debug info
        let member_types: Vec<_> = members
            .iter()
            .filter(|it| !(it.is_temp() || it.is_variadic() || it.is_var_external()))
            .enumerate()
            .filter_map(|(element_index, member)| {
                let dt = index.get_type(member.get_type_name().as_ref()).ok()?;
                let di_type = self.get_or_create_debug_type(dt, index, types_index).ok()?;
                let di_type = self.apply_const_type_if_needed(di_type.into(), member.is_constant());

                let llvm_type = types_index.find_associated_type(dt.get_name());
                let align_bits =
                    llvm_type.map(|ty| self.target_data.get_preferred_alignment(&ty) * 8).unwrap_or(0);
                let size_bits = llvm_type.map(|ty| self.target_data.get_bit_size(&ty)).unwrap_or(0);
                let offset_bits = self
                    .target_data
                    .offset_of_element(&struct_type, element_index as u32)
                    .map(|offset| offset * 8)
                    .unwrap_or(0);

                let member_name = super_ty_name
                    .filter(|sname| member.get_name() == format!("__{sname}"))
                    .map_or(member.get_name(), |_| "SUPER");

                Some(
                    self.debug_info
                        .create_member_type(
                            file.as_debug_info_scope(),
                            member_name,
                            file,
                            member.source_location.get_line_plus_one() as u32,
                            size_bits,
                            align_bits,
                            offset_bits,
                            DIFlags::PUBLIC,
                            di_type,
                        )
                        .as_type(),
                )
            })
            .collect();

        let llvm_size = self.target_data.get_bit_size(&struct_type);
        let struct_align_bits = self.target_data.get_preferred_alignment(&struct_type) * 8;

        let debug_struct = self.debug_info.create_struct_type(
            file.as_debug_info_scope(),
            name,
            file,
            location.get_line_plus_one() as u32,
            llvm_size,
            struct_align_bits,
            DIFlags::PUBLIC,
            None,
            member_types.as_slice(),
            0,
            None,
            name,
        );

        self.register_concrete_type(name, DebugType::Struct(debug_struct));
        Ok(())
    }

    fn create_array_type(
        &mut self,
        name: &str,
        inner_type: &str,
        dimensions: &[Dimension],
        size: u64,
        index: &Index,
        types_index: &LlvmTypedIndex,
    ) -> Result<(), Diagnostic> {
        //find the inner type debug info
        let inner_type = index.get_type(inner_type)?;
        //Find the dimensions as ranges
        let subscript = dimensions
            .iter()
            .map(|it| it.get_range_plus_one(index))
            .collect::<Result<Vec<Range<i64>>, _>>()
            .map_err(|err| Diagnostic::codegen_error(err, SourceLocation::undefined()))?;
        let inner_type = self.get_or_create_debug_type(inner_type, index, types_index)?;
        let llvm_type = types_index.get_associated_type(name)?;
        let align_bits = self.target_data.get_preferred_alignment(&llvm_type) * 8;
        let array_type =
            self.debug_info.create_array_type(inner_type.into(), size, align_bits, subscript.as_slice());

        self.register_concrete_type(name, DebugType::Composite(array_type));
        Ok(())
    }

    /// Creates a forward-declared struct type for use when resolving recursive type references.
    /// This is used when a pointer type references a struct that is currently being processed,
    /// to break the infinite recursion cycle.
    fn create_forward_declaration(&self, name: &str) -> DebugType<'ink> {
        let file = self.compile_unit.get_file();
        let fwd_struct = self.debug_info.create_struct_type(
            file.as_debug_info_scope(),
            name,
            file,
            0,
            0, // size unknown for forward declaration
            0, // align unknown for forward declaration
            DIFlagsConstants::FWD_DECL,
            None,
            &[], // empty elements for forward declaration
            0,
            None,
            name,
        );
        DebugType::Struct(fwd_struct)
    }

    fn create_pointer_type(
        &mut self,
        name: &str,
        inner_type_name: &str,
        size: u64,
        index: &Index,
        types_index: &LlvmTypedIndex,
    ) -> Result<(), Diagnostic> {
        let inner_type_entry = index.get_type(inner_type_name)?;

        let inner_type = if inner_type_entry.is_void() {
            DebugType::Basic(
                self.debug_info
                    .create_basic_type(
                        VOID_INTERNAL_NAME,
                        0,
                        DebugEncoding::DW_ATE_unsigned as u32,
                        DIFlagsConstants::PUBLIC,
                    )
                    .map_err(|err| Diagnostic::codegen_error(err.to_string(), SourceLocation::undefined()))?,
            )
        } else {
            self.get_or_create_debug_type(inner_type_entry, index, types_index)?
        };

        let llvm_type = types_index.get_associated_type(name)?;
        let align_bits = self.target_data.get_preferred_alignment(&llvm_type) * 8;
        let pointer_type = self.debug_info.create_pointer_type(
            name,
            inner_type.into(),
            size,
            align_bits,
            inkwell::AddressSpace::from(ADDRESS_SPACE_GLOBAL),
        );

        // For pointer types, we create a typedef to represent the pointer type in DWARF.
        // This allows other tools to distinguish between different kinds of pointer semantics (e.g., reference vs. auto-deref, type-safety, ...)
        // XXX: This is a workaround - gdb will show the mangled type name in the debugger, which is not ideal
        let typedef_name = match index.get_type(name).map(|it| it.get_type_information())? {
            DataTypeInformation::Pointer { auto_deref: Some(auto_deref), .. } => match auto_deref {
                plc_ast::ast::AutoDerefType::Reference => format!("__REFERENCE_TO__{name}"),
                _ => format!("__AUTO_DEREF__{name}"),
            },
            DataTypeInformation::Pointer { type_safe, .. } => {
                type_safe.then_some(format!("__REF_TO__{name}")).unwrap_or(format!("__POINTER_TO__{name}"))
            }
            _ => unreachable!("Only pointer types reach this"),
        };

        let file = self.compile_unit.get_file();
        let ty = self.debug_info.create_typedef(
            pointer_type.as_type(),
            &typedef_name,
            file,
            0, // Line 0 for built-in types
            file.as_debug_info_scope(),
            align_bits,
        );

        self.register_concrete_type(name, DebugType::Derived(ty));

        Ok(())
    }

    fn get_or_create_debug_type(
        &mut self,
        dt: &DataType,
        index: &Index,
        types_index: &LlvmTypedIndex,
    ) -> Result<DebugType<'ink>, Diagnostic> {
        let dt_name = dt.get_name();
        let key = dt_name.to_lowercase();

        // Already registered - return it
        if let Some(debug_type) = self.types.get(&key) {
            return Ok(*debug_type);
        }

        // Currently being processed - return forward declaration to break recursion
        if self.processing.contains(&key) {
            log::trace!("Type {dt_name} is being processed, returning forward declaration");
            return Ok(self.create_forward_declaration(dt_name));
        }

        // Register the type (this will add it to self.types)
        self.register_debug_type(dt_name, dt, index, types_index)?;

        self.types
            .get(&key)
            .ok_or_else(|| {
                Diagnostic::new(format!("Cannot find debug information for type {dt_name}"))
                    .with_error_code("E076")
            })
            .copied()
    }

    /// Creates debug information for string types using an array + typedef approach.
    ///
    /// This function generates DWARF debug metadata for string types by creating:
    /// 1. A character array type based on the string's encoding (CHAR for UTF-8, WCHAR for UTF-16)
    /// 2. A typedef with a unique name based on encoding and length (e.g., "__STRING__81", "__WSTRING__26")
    ///
    /// ## Typedef Naming
    ///
    /// The typedef uses the `__STRING__<grapheme count>` or `__WSTRING__<grapheme count>` pattern where:
    /// - Double underscore prefix avoids clashing with user-defined types (reserved for compiler internals)
    /// - Length suffix ensures each string type has a unique DWARF reference
    /// - Consistent pattern enables easy detection by DWARF parsers
    fn create_string_type(
        &mut self,
        name: &str,
        length: i64,
        encoding: StringEncoding,
        size: u64,
        index: &Index,
        types_index: &LlvmTypedIndex,
    ) -> Result<(), Diagnostic> {
        let char_datatype = match encoding {
            StringEncoding::Utf8 => index.get_effective_type_or_void_by_name(CHAR_TYPE),
            StringEncoding::Utf16 => index.get_effective_type_or_void_by_name(WCHAR_TYPE),
        };

        let char_debug_type = self.get_or_create_debug_type(char_datatype, index, types_index)?;
        let array_align_bits =
            self.target_data.get_preferred_alignment(&types_index.get_associated_type(name)?) * 8;
        let array_type = self.debug_info.create_array_type(
            char_debug_type.into(),
            size,
            array_align_bits,
            #[allow(clippy::single_range_in_vec_init)]
            &[0..length],
        );
        let typedef_name = match encoding {
            StringEncoding::Utf8 => format!("__STRING__{}", length),
            StringEncoding::Utf16 => format!("__WSTRING__{}", length),
        };

        let file = self.compile_unit.get_file();
        let string_typedef = self.debug_info.create_typedef(
            array_type.as_type(),
            &typedef_name,
            file,
            0, // Line 0 for built-in types
            file.as_debug_info_scope(),
            array_align_bits,
        );

        self.register_concrete_type(name, DebugType::Derived(string_typedef));
        Ok(())
    }

    fn create_typedef_type(
        &mut self,
        name: &str,
        referenced_type: &str,
        location: &SourceLocation,
        index: &Index,
        types_index: &LlvmTypedIndex,
    ) -> Result<(), Diagnostic> {
        let inner_dt = index.get_effective_type_by_name(referenced_type)?;
        let inner_type = self.get_or_create_debug_type(inner_dt, index, types_index)?;
        let file = location
            .get_file_name()
            .map(|it| self.get_or_create_debug_file(it))
            .unwrap_or_else(|| self.compile_unit.get_file());

        let llvm_type = types_index.get_associated_type(name)?;
        let align_bits = self.target_data.get_preferred_alignment(&llvm_type) * 8;
        let typedef = self.debug_info.create_typedef(
            inner_type.into(),
            name,
            file,
            location.get_line_plus_one() as u32,
            file.as_debug_info_scope(),
            align_bits,
        );
        self.register_concrete_type(name, DebugType::Derived(typedef));

        Ok(())
    }

    fn create_subrange_type(
        &mut self,
        name: &str,
        referenced_type: &str,
        sub_range: &std::ops::Range<TypeSize>,
        location: &SourceLocation,
        index: &Index,
        types_index: &LlvmTypedIndex,
    ) -> Result<(), Diagnostic> {
        // Resolve the range bounds to create a unique typedef name based on backing type
        let start_val = sub_range
            .start
            .as_int_value(index)
            .map_err(|err| Diagnostic::codegen_error(err, location.clone()))?;
        let end_val = sub_range
            .end
            .as_int_value(index)
            .map_err(|err| Diagnostic::codegen_error(err, location.clone()))?;
        let typedef_name = format!("__SUBRANGE_{start_val}_{end_val}__{referenced_type}");

        // Check if we already have a debug type for this subrange signature
        if let Some(existing_type) = self.types.get(&typedef_name.to_lowercase()).cloned() {
            self.register_concrete_type(name, existing_type);
            return Ok(());
        }

        let inner_dt = index.get_effective_type_by_name(referenced_type)?;
        let inner_type = self.get_or_create_debug_type(inner_dt, index, types_index)?;
        let file = location
            .get_file_name()
            .map(|it| self.get_or_create_debug_file(it))
            .unwrap_or_else(|| self.compile_unit.get_file());

        let llvm_type = types_index.get_associated_type(name)?;
        let align_bits = self.target_data.get_preferred_alignment(&llvm_type) * 8;

        let typedef = self.debug_info.create_typedef(
            inner_type.into(),
            &typedef_name,
            file,
            location.get_line_plus_one() as u32,
            file.as_debug_info_scope(),
            align_bits,
        );

        // Register under the canonical name for future reuse
        self.register_concrete_type(&typedef_name, DebugType::Derived(typedef));
        // Also register under this specific type name for lookups
        self.register_concrete_type(name, DebugType::Derived(typedef));

        Ok(())
    }

    fn create_subroutine_type(
        &mut self,
        return_type: Option<&DataType>,
        parameter_types: &[&DataType],
        file: DIFile<'ink>,
    ) -> DISubroutineType<'ink> {
        let return_type = return_type
            .as_ref()
            .filter(|return_type| !return_type.is_aggregate_type())
            .and_then(|dt| self.types.get(dt.get_name()))
            .map(|return_type| return_type.to_owned())
            .map(Into::into);

        let parameter_types = parameter_types
            .iter()
            .map(|dt| {
                self.types
                    .get(dt.get_name().to_lowercase().as_str())
                    .copied()
                    .map(Into::into)
                    .unwrap_or_else(|| panic!("Cound not find debug type information for {}", dt.get_name()))
                //Types should be created by this stage
            })
            .collect::<Vec<DIType>>();

        self.debug_info.create_subroutine_type(file, return_type, &parameter_types, DIFlagsConstants::PUBLIC)
    }

    fn create_function(
        &mut self,
        scope: DIScope<'ink>,
        pou: &PouIndexEntry,
        return_type: Option<&DataType>,
        parameter_types: &[&DataType],
        implementation_start: usize,
    ) -> DISubprogram<'_> {
        let location = pou.get_location();
        let file = location
            .get_file_name()
            .map(|it| self.get_or_create_debug_file(it))
            .unwrap_or_else(|| self.compile_unit.get_file());
        let is_external = matches!(pou.get_linkage(), LinkageType::External);
        let ditype = self.create_subroutine_type(return_type, parameter_types, file);
        self.debug_info.create_function(
            scope,
            pou.get_name(),
            Some(pou.get_name()), // for generics e.g. NAME__TYPE
            file,
            location.get_line_plus_one() as u32,
            // entry for the function
            ditype,
            false,
            !is_external,
            (implementation_start + 1) as u32,
            DIFlagsConstants::PUBLIC,
            self.optimization.is_optimized(),
        )
    }

    ///Creates the debug information for function variables
    ///For a `Function` these will be all VAR_INPUT, VAR_OUTPUT and VAR_IN_OUT in addition to
    ///entries for VAR and VAR_TEMP
    ///For other POUs we create entries in VAR_TEMP and an additional single parameter at position 0
    ///(the struct)
    fn create_function_variables(
        &mut self,
        pou: &PouIndexEntry,
        func: &FunctionContext<'ink, '_>,
        index: &Index,
        types_index: &LlvmTypedIndex<'ink>,
    ) {
        let mut param_offset = 0;
        //Register the return and local variables for debugging
        for variable in index
            .get_variables_for_pou(pou)
            .iter()
            .filter(|it| it.is_local() || it.is_temp() || it.is_return())
        {
            let align_bits = types_index
                .get_associated_type(&variable.data_type_name)
                .map(|it| self.target_data.get_preferred_alignment(&it))
                .unwrap_or(0)
                * 8;
            self.register_local_variable(variable, align_bits, func);
        }

        let implementation = pou.find_implementation(index).expect("A POU will have an impl at this stage");
        if implementation.get_implementation_type().has_self_parameter() {
            self.register_struct_parameter(pou.get_parent_pou_name().unwrap_or_else(|| pou.get_name()), func);
            param_offset += 1;
        }
        if implementation.get_implementation_type().is_function_method_or_init() {
            let declared_params = index.get_available_parameters(implementation.get_call_name());
            // Register all parameters for debugging
            for (index, variable) in declared_params.iter().enumerate() {
                self.register_parameter(variable, index + param_offset, func);
            }
        }
    }

    fn get_or_create_debug_file(&mut self, location: &'static str) -> DIFile<'ink> {
        let path = Path::new(location);
        let directory = path.parent().and_then(|it| it.to_str()).unwrap_or("");
        let filename = path.file_name().and_then(|it| it.to_str()).unwrap_or(location);
        *self.files.entry(location).or_insert_with(|| {
            //split to dir and file
            self.debug_info.create_file(filename, directory)
        })
    }

    fn get_debug_file(&self, location: &'static str) -> Option<DIFile<'ink>> {
        self.files.get(location).copied()
    }
}

impl<'ink> Debug<'ink> for DebugBuilder<'ink> {
    fn set_debug_location(&self, llvm: &Llvm, scope: &FunctionContext, line: usize, column: usize) {
        let file = scope
            .linking_context
            .get_location()
            .get_file_name()
            .and_then(|it| self.get_debug_file(it))
            .unwrap_or_else(|| self.compile_unit.get_file());
        let scope = scope
            .function
            .get_subprogram()
            .map(|it| it.as_debug_info_scope())
            .unwrap_or_else(|| file.as_debug_info_scope());
        let location =
            self.debug_info.create_debug_location(self.context, line as u32, column as u32, scope, None);
        llvm.builder.set_current_debug_location(location);
    }

    fn unset_debug_location(&self, llvm: &Llvm) {
        llvm.builder.unset_current_debug_location();
    }

    fn register_function<'idx>(
        &mut self,
        indices: (&Index, &LlvmTypedIndex<'ink>),
        func: &FunctionContext<'ink, 'idx>,
        return_type: Option<&'idx DataType>,
        parent_function: Option<FunctionValue<'ink>>,
        parameter_types: &[&'idx DataType],
        implementation_start: usize,
    ) {
        let (index, types_index) = indices;
        let pou = index.find_pou(func.linking_context.get_call_name()).expect("POU is available");
        if matches!(pou.get_linkage(), LinkageType::External) || pou.get_location().is_internal() {
            return;
        }
        let file = pou
            .get_location()
            .get_file_name()
            .map(|it| self.get_or_create_debug_file(it))
            .unwrap_or_else(|| self.compile_unit.get_file());
        let scope = if let Some(function) = parent_function.and_then(|it| it.get_subprogram()) {
            function.as_debug_info_scope()
        } else {
            file.as_debug_info_scope()
        };
        let subprogram = self.create_function(scope, pou, return_type, parameter_types, implementation_start);
        func.function.set_subprogram(subprogram);
        //Create function parameters
        self.create_function_variables(pou, func, index, types_index);
    }

    fn register_debug_type<'idx>(
        &mut self,
        name: &str,
        datatype: &'idx DataType,
        index: &'idx Index,
        types_index: &LlvmTypedIndex,
    ) -> Result<(), Diagnostic> {
        let key = name.to_lowercase();

        // Already registered
        if self.types.contains_key(&key) {
            log::trace!("Type {name} already has debug info");
            return Ok(());
        }

        // Track that we're processing this type (for recursion detection in get_or_create_debug_type)
        self.processing.insert(key.clone());

        let type_info = datatype.get_type_information();
        let size = types_index
            .find_associated_type(name)
            .or_else(|| types_index.find_associated_pou_type(name))
            .map(|llvm_type| self.target_data.get_bit_size(&llvm_type))
            .unwrap_or(0);
        let location = &datatype.location;
        log::trace!("Creating debug info for type {name} with size {size} and info {type_info:?}");

        let result = match type_info {
            DataTypeInformation::Struct { members, .. } => {
                self.create_struct_type(name, members.as_slice(), location, index, types_index)
            }
            DataTypeInformation::Array { name, inner_type_name, dimensions, .. } => {
                self.create_array_type(name, inner_type_name, dimensions, size, index, types_index)
            }
            DataTypeInformation::Pointer { name, inner_type_name, .. } => {
                self.create_pointer_type(name, inner_type_name, size, index, types_index)
            }
            DataTypeInformation::Integer { signed, size, .. } => {
                let encoding = if type_info.is_bool() {
                    DebugEncoding::DW_ATE_boolean
                } else if type_info.is_character() {
                    DebugEncoding::DW_ATE_UTF
                } else {
                    match *signed {
                        true => DebugEncoding::DW_ATE_signed,
                        false => DebugEncoding::DW_ATE_unsigned,
                    }
                };
                self.create_basic_type(name, *size as u64, encoding, location)
            }
            DataTypeInformation::Float { size, .. } => {
                self.create_basic_type(name, *size as u64, DebugEncoding::DW_ATE_float, location)
            }
            DataTypeInformation::String { size: string_size, encoding, .. } => {
                let length = string_size
                    .as_int_value(index)
                    .map_err(|err| Diagnostic::codegen_error(err, SourceLocation::undefined()))?;
                self.create_string_type(name, length, *encoding, size, index, types_index)
            }
            DataTypeInformation::Alias { name, referenced_type }
            | DataTypeInformation::Enum { name, referenced_type, .. } => {
                self.create_typedef_type(name, referenced_type, location, index, types_index)
            }
            DataTypeInformation::SubRange { name, referenced_type, sub_range } => {
                self.create_subrange_type(name, referenced_type, sub_range, location, index, types_index)
            }
            // Other types are just derived basic types
            _ => {
                log::debug!("Type {name} has unsupported debug info generation for {type_info:?}");
                Ok(())
            }
        };

        self.processing.remove(&key);
        result
    }

    fn create_global_variable(
        &mut self,
        name: &str,
        type_name: &str,
        global_variable: GlobalValue<'ink>,
        location: &SourceLocation,
    ) {
        if let Some(debug_type) = self.types.get(&type_name.to_lowercase()) {
            let debug_type =
                self.apply_const_type_if_needed((*debug_type).into(), global_variable.is_constant());

            let file = location
                .get_file_name()
                .map(|it| self.get_or_create_debug_file(it))
                .unwrap_or_else(|| self.compile_unit.get_file());
            let debug_variable = self.debug_info.create_global_variable_expression(
                file.as_debug_info_scope(),
                name,
                "",
                file,
                location.get_line_plus_one() as u32,
                debug_type,
                false,
                None,
                None,
                0, // Global variable alignment does not need to be forced/set. See https://llvm.org/docs/LangRef.html#global-variables
            );
            let gv_metadata = debug_variable.as_metadata_value(self.context);

            global_variable.set_metadata(gv_metadata, 0);
            self.context.metadata_node(&[gv_metadata.into()]);
        }
    }

    fn register_local_variable(
        &mut self,
        variable: &VariableIndexEntry,
        alignment: u32,
        function_scope: &FunctionContext<'ink, '_>,
    ) {
        let type_name = variable.get_type_name();
        let location = &variable.source_location;
        let file = location
            .get_file_name()
            .map(|it| self.get_or_create_debug_file(it))
            .unwrap_or_else(|| self.compile_unit.get_file());
        let line = location.get_line_plus_one() as u32;

        let scope = function_scope
            .function
            .get_subprogram()
            .map(|it| it.as_debug_info_scope())
            .unwrap_or_else(|| file.as_debug_info_scope());
        if let Some(debug_type) = self.types.get(&type_name.to_lowercase()) {
            let debug_type = self.apply_const_type_if_needed((*debug_type).into(), variable.is_constant());

            let debug_variable = self.debug_info.create_auto_variable(
                scope,
                variable.get_name(),
                file,
                line,
                debug_type,
                false,
                DIFlagsConstants::ZERO,
                alignment,
            );

            let variable_key = VariableKey::new(
                variable.get_qualified_name(),
                Some(&function_scope.linking_context.get_call_name_for_ir()),
            );
            self.variables.insert(variable_key, debug_variable);
        }
    }

    fn register_parameter(
        &mut self,
        variable: &VariableIndexEntry,
        arg_no: usize,
        function_scope: &FunctionContext<'ink, '_>,
    ) {
        let type_name = variable.get_type_name();
        let location = &variable.source_location;
        let file = location
            .get_file_name()
            .map(|it| self.get_or_create_debug_file(it))
            .unwrap_or_else(|| self.compile_unit.get_file());
        let line = location.get_line_plus_one() as u32;
        let scope = function_scope
            .function
            .get_subprogram()
            .map(|it| it.as_debug_info_scope())
            .unwrap_or_else(|| file.as_debug_info_scope());

        if let Some(debug_type) = self.types.get(&type_name.to_lowercase()) {
            let debug_type = self.apply_const_type_if_needed((*debug_type).into(), variable.is_constant());

            let debug_variable = self.debug_info.create_parameter_variable(
                scope,
                variable.get_name(),
                arg_no as u32,
                file,
                line,
                debug_type,
                false,
                DIFlagsConstants::ZERO,
            );

            let variable_key = VariableKey::new(
                variable.get_qualified_name(),
                Some(&function_scope.linking_context.get_call_name_for_ir()),
            );
            self.variables.insert(variable_key, debug_variable);
        }
    }

    fn register_struct_parameter(&mut self, name: &str, function_scope: &FunctionContext<'ink, '_>) {
        let file = function_scope
            .linking_context
            .get_location()
            .get_file_name()
            .map(|it| self.get_or_create_debug_file(it))
            .unwrap_or_else(|| self.compile_unit.get_file());
        let scope = function_scope
            .function
            .get_subprogram()
            .map(|it| it.as_debug_info_scope())
            .unwrap_or_else(|| file.as_debug_info_scope());
        let variable_key =
            VariableKey::new(name, Some(&function_scope.linking_context.get_call_name_for_ir()));
        if let Some(debug_type) = self.types.get(&name.to_lowercase()) {
            let debug_type = *debug_type;
            let line = function_scope.linking_context.get_location().get_line_plus_one() as u32;
            let debug_variable = self.debug_info.create_parameter_variable(
                scope,
                name,
                0,
                file,
                line,
                debug_type.into(),
                false,
                DIFlagsConstants::ZERO,
            );
            self.variables.insert(variable_key, debug_variable);
        }
    }

    fn add_variable_declaration(
        &self,
        name: &str,
        value: PointerValue<'ink>,
        function_scope: &FunctionContext,
        block: BasicBlock<'ink>,
        line: usize,
        column: usize,
    ) {
        let file = function_scope
            .linking_context
            .get_location()
            .get_file_name()
            .and_then(|it| self.get_debug_file(it))
            .unwrap_or_else(|| self.compile_unit.get_file());
        let scope = function_scope
            .function
            .get_subprogram()
            .map(|it| it.as_debug_info_scope())
            .unwrap_or_else(|| file.as_debug_info_scope());

        let location = self.debug_info.create_debug_location(
            self.context,
            (line + 1) as u32,
            column as u32,
            scope,
            None,
        );

        let key = VariableKey::new(name, Some(&function_scope.linking_context.get_call_name_for_ir()));
        let variable = self.variables.get(&key);

        if variable.is_none() {
            // LLVM 19+ crashes with a segfault when `LLVMDIBuilderInsertDeclareRecordAtEnd` receives
            // a null DILocalVariable pointer. Skip inserting the debug declare if the variable
            // wasn't registered. To test, the unit test`actions_debug` should segfault when not
            // early-returning here.
            return;
        }

        // Workaround until inkwell 0.9 is released, see https://github.com/TheDan64/inkwell/issues/613
        fn insert_declare_at_end_raw(
            debug_info: &DebugInfoBuilder,
            storage: PointerValue,
            var_info: Option<DILocalVariable>,
            expr: Option<inkwell::debug_info::DIExpression>,
            debug_loc: inkwell::debug_info::DILocation,
            block: BasicBlock,
        ) {
            // When expr is None, we create an empty expression (same as inkwell does internally).
            let expr = expr.unwrap_or_else(|| debug_info.create_expression(vec![]));
            unsafe {
                inkwell::llvm_sys::debuginfo::LLVMDIBuilderInsertDeclareRecordAtEnd(
                    debug_info.as_mut_ptr(),
                    inkwell::values::AsValueRef::as_value_ref(&storage),
                    var_info.map(|v| v.as_mut_ptr()).unwrap_or(std::ptr::null_mut()),
                    expr.as_mut_ptr(),
                    debug_loc.as_mut_ptr(),
                    block.as_mut_ptr(),
                );
            }
        }

        // TODO: Replace function call below with `self.debug_info.insert_declare_at_end(value, variable.copied(), None, location, block);`
        // once inkwell 0.9 is released.
        insert_declare_at_end_raw(&self.debug_info, value, variable.copied(), None, location, block);
    }

    fn finalize(&self) {
        self.debug_info.finalize();
    }
}

impl<'ink> Debug<'ink> for DebugBuilderEnum<'ink> {
    fn set_debug_location(&self, llvm: &Llvm, scope: &FunctionContext, line: usize, column: usize) {
        match self {
            Self::None | Self::VariablesOnly(..) => {}
            Self::Full(obj) => obj.set_debug_location(llvm, scope, line, column),
        };
    }

    fn unset_debug_location(&self, llvm: &Llvm) {
        match self {
            Self::None | Self::VariablesOnly(..) => {}
            Self::Full(obj) => obj.unset_debug_location(llvm),
        };
    }

    fn register_function<'idx>(
        &mut self,
        indices: (&Index, &LlvmTypedIndex<'ink>),
        func: &FunctionContext<'ink, 'idx>,
        return_type: Option<&'idx DataType>,
        parent_function: Option<FunctionValue<'ink>>,
        parameter_types: &[&'idx DataType],
        implementation_start: usize,
    ) {
        match self {
            Self::None | Self::VariablesOnly(..) => {}
            Self::Full(obj) => obj.register_function(
                indices,
                func,
                return_type,
                parent_function,
                parameter_types,
                implementation_start,
            ),
        };
    }

    fn register_debug_type<'idx>(
        &mut self,
        name: &str,
        datatype: &'idx DataType,
        index: &'idx Index,
        types_index: &'idx LlvmTypedIndex,
    ) -> Result<(), Diagnostic> {
        match self {
            Self::None => Ok(()),
            Self::VariablesOnly(obj) | Self::Full(obj) => {
                obj.register_debug_type(name, datatype, index, types_index)
            }
        }
    }

    fn create_global_variable(
        &mut self,
        name: &str,
        type_name: &str,
        global_variable: GlobalValue<'ink>,
        location: &SourceLocation,
    ) {
        match self {
            Self::None => {}
            Self::VariablesOnly(obj) | Self::Full(obj) => {
                obj.create_global_variable(name, type_name, global_variable, location)
            }
        }
    }

    fn register_local_variable(
        &mut self,
        variable: &VariableIndexEntry,
        alignment: u32,
        scope: &FunctionContext<'ink, '_>,
    ) {
        match self {
            Self::None | Self::VariablesOnly(_) => {}
            Self::Full(obj) => obj.register_local_variable(variable, alignment, scope),
        }
    }

    fn register_parameter(
        &mut self,
        variable: &VariableIndexEntry,
        arg_no: usize,
        scope: &FunctionContext<'ink, '_>,
    ) {
        match self {
            Self::None | Self::VariablesOnly(_) => {}
            Self::Full(obj) => obj.register_parameter(variable, arg_no, scope),
        }
    }

    fn register_struct_parameter(&mut self, pou: &str, scope: &FunctionContext<'ink, '_>) {
        match self {
            Self::None | Self::VariablesOnly(_) => {}
            Self::Full(obj) => obj.register_struct_parameter(pou, scope),
        }
    }

    fn add_variable_declaration(
        &self,
        name: &str,
        value: PointerValue<'ink>,
        scope: &FunctionContext,
        block: BasicBlock<'ink>,
        line: usize,
        column: usize,
    ) {
        match self {
            Self::None | Self::VariablesOnly(_) => {}
            Self::Full(obj) => obj.add_variable_declaration(name, value, scope, block, line, column),
        }
    }

    fn finalize(&self) {
        match self {
            Self::None => {}
            Self::VariablesOnly(obj) | Self::Full(obj) => obj.finalize(),
        }
    }
}
