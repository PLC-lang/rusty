use std::{collections::HashMap, ops::Range, path::Path};

use inkwell::{
    basic_block::BasicBlock,
    context::Context,
    debug_info::{
        AsDIScope, DIBasicType, DICompileUnit, DICompositeType, DIDerivedType, DIFile, DIFlags,
        DIFlagsConstants, DILocalVariable, DISubprogram, DISubroutineType, DIType, DWARFEmissionKind,
        DebugInfoBuilder,
    },
    module::Module,
    values::{BasicMetadataValueEnum, FunctionValue, GlobalValue, PointerValue},
};
use plc_ast::ast::LinkageType;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;

use crate::{
    datalayout::{Bytes, DataLayout, MemoryLocation},
    index::{ImplementationType, Index, PouIndexEntry, VariableIndexEntry},
    typesystem::{DataType, DataTypeInformation, Dimension, StringEncoding, CHAR_TYPE, WCHAR_TYPE},
    DebugLevel, OptimizationLevel,
};

use super::generators::{llvm::Llvm, ADDRESS_SPACE_GLOBAL};

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
        scope: &FunctionValue,
        //Current line starts with 0
        line: usize,
        column: usize,
    );

    /// Registers a new function for debugging, this method is responsible for registering a
    /// function's stub as well as its interface (variables/parameters)
    fn register_function<'idx>(
        &mut self,
        index: &Index,
        func: FunctionValue<'ink>,
        pou: &PouIndexEntry,
        return_type: Option<&'idx DataType>,
        parameter_types: &[&'idx DataType],
        implementation_start: usize,
    );

    /// Registers a new datatype for debugging
    fn register_debug_type<'idx>(
        &mut self,
        name: &str,
        datatype: &'idx DataType,
        index: &'idx Index,
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
        scope: FunctionValue<'ink>,
    );

    /// Creates a debug entry for a function parameter
    fn register_parameter(
        &mut self,
        variable: &VariableIndexEntry,
        arg_no: usize,
        scope: FunctionValue<'ink>,
    );

    /// Create the debug entry for an Function POU entry
    fn register_struct_parameter(&mut self, pou: &PouIndexEntry, scope: FunctionValue<'ink>);

    fn add_variable_declaration(
        &self,
        name: &str,
        value: PointerValue<'ink>,
        scope: FunctionValue<'ink>,
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

/// Represents the debug builder and information for a compilation unit.
pub struct DebugBuilder<'ink> {
    context: &'ink Context,
    debug_info: DebugInfoBuilder<'ink>,
    compile_unit: DICompileUnit<'ink>,
    types: HashMap<String, DebugType<'ink>>,
    variables: HashMap<String, DILocalVariable<'ink>>,
    optimization: OptimizationLevel,
    files: HashMap<&'static str, DIFile<'ink>>,
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

                let path = Path::new(module.get_source_file_name().to_str().unwrap_or(""));
                let root = root.unwrap_or_else(|| Path::new(""));
                let filename = path.strip_prefix(root).unwrap_or(path).to_str().unwrap_or_default();
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

                let dbg_obj = DebugBuilder {
                    context,
                    debug_info,
                    compile_unit,
                    types: Default::default(),
                    variables: Default::default(),
                    optimization,
                    files: Default::default(),
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
            .map_err(|err| Diagnostic::codegen_error(err, location.clone()))?;
        self.register_concrete_type(name, DebugType::Basic(res));
        Ok(())
    }

    fn create_struct_type(
        &mut self,
        name: &str,
        members: &[VariableIndexEntry],
        index: &Index,
        location: &SourceLocation,
    ) -> Result<(), Diagnostic> {
        //Create each type
        let index_types = members
            .iter()
            .map(|it| (it.get_name(), it.get_type_name(), &it.source_location))
            .map(|(name, type_name, location)| {
                index.get_type(type_name.as_ref()).map(|dt| (name, dt, location))
            })
            .collect::<Result<Vec<_>, Diagnostic>>()?;

        let file = location
            .get_file_name()
            .map(|it| self.get_or_create_debug_file(it))
            .unwrap_or_else(|| self.compile_unit.get_file());

        let mut types = vec![];
        let mut running_offset = MemoryLocation::new(0);
        for (member_name, dt, location) in index_types.into_iter() {
            let di_type = self.get_or_create_debug_type(dt, index)?;
            //Adjust the offset based on the field alignment
            let type_info = dt.get_type_information();
            let alignment = type_info.get_alignment(index);
            let size = type_info.get_size(index);
            running_offset = running_offset.align_to(alignment);
            types.push(
                self.debug_info
                    .create_member_type(
                        file.as_debug_info_scope(),
                        member_name,
                        file,
                        location.get_line_plus_one() as u32,
                        size.bits().into(),
                        alignment.bits(),
                        running_offset.bits().into(),
                        DIFlags::PUBLIC,
                        di_type.into(),
                    )
                    .as_type(),
            );
            running_offset += size;
        }

        let struct_dt = index.get_type_information_or_void(name);

        //Create a struct type
        let struct_type = self.debug_info.create_struct_type(
            file.as_debug_info_scope(),
            name,
            file,
            location.get_line_plus_one() as u32,
            running_offset.bits().into(),
            struct_dt.get_alignment(index).bits(),
            DIFlags::PUBLIC,
            None,
            types.as_slice(),
            0,
            None,
            name,
        );

        self.register_concrete_type(name, DebugType::Struct(struct_type));
        Ok(())
    }

    fn create_array_type(
        &mut self,
        name: &str,
        inner_type: &str,
        dimensions: &[Dimension],
        size: Bytes,
        alignment: Bytes,
        index: &Index,
    ) -> Result<(), Diagnostic> {
        //find the inner type debug info
        let inner_type = index.get_type(inner_type)?;
        //Find the dimenstions as ranges
        let subscript = dimensions
            .iter()
            .map(|it| it.get_range(index))
            //Convert to normal range
            .collect::<Result<Vec<Range<i64>>, _>>()
            .map_err(|err| Diagnostic::codegen_error(err, SourceLocation::undefined()))?;
        let inner_type = self.get_or_create_debug_type(inner_type, index)?;
        let array_type = self.debug_info.create_array_type(
            inner_type.into(),
            size.bits().into(),
            alignment.bits(),
            subscript.as_slice(),
        );
        self.register_concrete_type(name, DebugType::Composite(array_type));
        Ok(())
    }

    fn create_pointer_type(
        &mut self,
        name: &str,
        inner_type: &str,
        size: Bytes,
        alignment: Bytes,
        index: &Index,
    ) -> Result<(), Diagnostic> {
        let inner_type = index.get_type(inner_type)?;
        let inner_type = self.get_or_create_debug_type(inner_type, index)?;
        let pointer_type = self.debug_info.create_pointer_type(
            name,
            inner_type.into(),
            size.bits().into(),
            alignment.bits(),
            inkwell::AddressSpace::from(ADDRESS_SPACE_GLOBAL),
        );
        self.register_concrete_type(name, DebugType::Derived(pointer_type));
        Ok(())
    }

    fn get_or_create_debug_type(
        &mut self,
        dt: &DataType,
        index: &Index,
    ) -> Result<DebugType<'ink>, Diagnostic> {
        //Try to find a type in the types
        let dt_name = dt.get_name().to_lowercase();
        //Attempt to re-register the type, this will do nothing if the type exists.
        //TODO: This will crash on recursive datatypes
        self.register_debug_type(&dt_name, dt, index)?;
        self.types
            .get(&dt_name)
            .ok_or_else(|| {
                Diagnostic::new(format!("Cannot find debug information for type {dt_name}"))
                    .with_error_code("E076")
            })
            .map(|it| it.to_owned())
    }

    fn create_string_type(
        &mut self,
        name: &str,
        length: i64,
        encoding: StringEncoding,
        size: Bytes,
        alignment: Bytes,
        index: &Index,
    ) -> Result<(), Diagnostic> {
        // Register a utf8 or 16 basic type
        let inner_type = match encoding {
            StringEncoding::Utf8 => index.get_effective_type_or_void_by_name(CHAR_TYPE),
            StringEncoding::Utf16 => index.get_effective_type_or_void_by_name(WCHAR_TYPE),
        };
        let inner_type = self.get_or_create_debug_type(inner_type, index)?;
        //Register an array
        let array_type = self.debug_info.create_array_type(
            inner_type.into(),
            size.bits().into(),
            alignment.bits(),
            #[allow(clippy::single_range_in_vec_init)]
            &[(0..(length - 1))],
        );
        self.register_concrete_type(name, DebugType::Composite(array_type));
        Ok(())
    }

    fn create_typedef_type(
        &mut self,
        name: &str,
        referenced_type: &str,
        index: &Index,
        location: &SourceLocation,
    ) -> Result<(), Diagnostic> {
        let inner_dt = index.get_effective_type_by_name(referenced_type)?;
        let inner_type = self.get_or_create_debug_type(inner_dt, index)?;
        let file = location
            .get_file_name()
            .map(|it| self.get_or_create_debug_file(it))
            .unwrap_or_else(|| self.compile_unit.get_file());

        let typedef = self.debug_info.create_typedef(
            inner_type.into(),
            name,
            file,
            location.get_line_plus_one() as u32,
            file.as_debug_info_scope(),
            inner_dt.get_type_information().get_alignment(index).bits(),
        );
        self.register_concrete_type(name, DebugType::Derived(typedef));

        Ok(())
    }

    fn create_subroutine_type(
        &self,
        return_type: Option<&DataType>,
        parameter_types: &[&DataType],
        file: DIFile<'ink>,
    ) -> DISubroutineType {
        let return_type = return_type
            .filter(|it| !it.is_aggregate_type())
            .and_then(|dt| self.types.get(dt.get_name()))
            .map(|it| it.to_owned())
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
        pou: &PouIndexEntry,
        return_type: Option<&DataType>,
        parameter_types: &[&DataType],
        implementation_start: usize,
    ) -> DISubprogram {
        let location = pou.get_location();
        let file = location
            .get_file_name()
            .map(|it| self.get_or_create_debug_file(it))
            .unwrap_or_else(|| self.compile_unit.get_file());
        let is_external = matches!(pou.get_linkage(), LinkageType::External);
        let ditype = self.create_subroutine_type(return_type, parameter_types, file);
        self.debug_info.create_function(
            file.as_debug_info_scope(),
            pou.get_name(),
            Some(pou.get_name()), // for generics e.g. NAME__TYPE
            file,
            location.get_line_plus_one() as u32,
            // entry for the function
            ditype,
            false, // TODO: what is this
            !is_external,
            (implementation_start + 1) as u32,
            DIFlagsConstants::PUBLIC,
            self.optimization.is_optimized(),
        )
    }

    ///Creates the debug information for function variables
    ///For a `Function` these will be all VAR_INPUT, VAR_OUTPUT and VAR_IN_OUT in addition to
    ///entries for VAR and VAR_TEMP
    ///For other POUs we create enties in VAR_TEMP and an additional single parameter at position 0
    ///(the struct)
    fn create_function_variables(&mut self, pou: &PouIndexEntry, func: FunctionValue<'ink>, index: &Index) {
        let mut param_offset = 0;
        //Register the return and local variables for debugging
        for variable in index
            .get_pou_members(pou.get_name())
            .iter()
            .filter(|it| it.is_local() || it.is_temp() || it.is_return())
        {
            let var_type = index
                .find_effective_type_by_name(variable.get_type_name())
                .expect("Type should exist at this stage");
            let alignment = var_type.get_type_information().get_alignment(index).bits();
            //If the variable is an aggregate return type, register it as first parameter, and
            //increase the param count
            if variable.is_return() && var_type.is_aggregate_type() {
                self.register_aggregate_return(variable, var_type, func);
                param_offset += 1;
            } else {
                self.register_local_variable(variable, alignment, func);
            }
        }
        let implementation = pou.find_implementation(index).expect("A POU will have an impl at this stage");
        if implementation.implementation_type != ImplementationType::Function {
            if implementation.get_implementation_type() == &ImplementationType::Method {
                //Methods ignored for now
            } else {
                self.register_struct_parameter(pou, func);
            }
        } else {
            let declared_params = index.get_declared_parameters(implementation.get_call_name());
            //Register all parameters for debugging
            for (index, variable) in declared_params.iter().enumerate() {
                self.register_parameter(variable, index + param_offset, func);
            }
        }
    }

    fn register_aggregate_return(
        &mut self,
        variable: &VariableIndexEntry,
        var_type: &DataType,
        scope: FunctionValue<'ink>,
    ) {
        let original_type = self
            .types
            .get(&var_type.get_name().to_lowercase())
            .copied()
            .unwrap_or_else(|| panic!("Cannot find type {} in debug types", variable.get_name()))
            .into();
        let data_layout = DataLayout::default();
        let debug_type = self.debug_info.create_pointer_type(
            &format!("__ref_to_{}", variable.get_type_name()), // TODO: Naming convention (see plc_util/src/convention.rs)
            original_type,
            data_layout.p64.bits().into(),
            data_layout.p64.bits(),
            inkwell::AddressSpace::from(ADDRESS_SPACE_GLOBAL),
        );
        let location = &variable.source_location;
        let file = location
            .get_file_name()
            .map(|it| self.get_or_create_debug_file(it))
            .unwrap_or_else(|| self.compile_unit.get_file());
        let line = location.get_line_plus_one() as u32;
        let scope = scope
            .get_subprogram()
            .map(|it| it.as_debug_info_scope())
            .unwrap_or_else(|| self.compile_unit.as_debug_info_scope());
        let debug_variable = self.debug_info.create_parameter_variable(
            scope,
            variable.get_name(),
            0,
            file,
            line,
            debug_type.as_type(),
            false,
            DIFlagsConstants::ZERO,
        );
        self.variables.insert(variable.get_qualified_name().to_string(), debug_variable);
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
}

impl<'ink> Debug<'ink> for DebugBuilder<'ink> {
    fn set_debug_location(&self, llvm: &Llvm, scope: &FunctionValue, line: usize, column: usize) {
        let scope = scope
            .get_subprogram()
            .map(|it| it.as_debug_info_scope())
            .unwrap_or_else(|| self.compile_unit.as_debug_info_scope());
        let location =
            self.debug_info.create_debug_location(self.context, line as u32, column as u32, scope, None);
        llvm.builder.set_current_debug_location(location);
    }

    fn register_function<'idx>(
        &mut self,
        index: &Index,
        func: FunctionValue<'ink>,
        pou: &PouIndexEntry,
        return_type: Option<&'idx DataType>,
        parameter_types: &[&'idx DataType],
        implementation_start: usize,
    ) {
        if matches!(pou.get_linkage(), LinkageType::External) {
            return;
        }
        let subprogram = self.create_function(pou, return_type, parameter_types, implementation_start);
        func.set_subprogram(subprogram);
        //Create function parameters
        self.create_function_variables(pou, func, index);
    }

    fn register_debug_type<'idx>(
        &mut self,
        name: &str,
        datatype: &'idx DataType,
        index: &'idx Index,
    ) -> Result<(), Diagnostic> {
        //check if the type is currently registered
        if !self.types.contains_key(&name.to_lowercase()) {
            let type_info = datatype.get_type_information();
            let size = type_info.get_size(index);
            let alignment = type_info.get_alignment(index);
            let location = &datatype.location;
            match type_info {
                DataTypeInformation::Struct { members, .. } => {
                    self.create_struct_type(name, members.as_slice(), index, location)
                }
                DataTypeInformation::Array { name, inner_type_name, dimensions, .. } => {
                    self.create_array_type(name, inner_type_name, dimensions, size, alignment, index)
                }
                DataTypeInformation::Pointer { name, inner_type_name, .. } => {
                    self.create_pointer_type(name, inner_type_name, size, alignment, index)
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
                    self.create_string_type(name, length, *encoding, size, alignment, index)
                }
                DataTypeInformation::Alias { name, referenced_type }
                | DataTypeInformation::Enum { name, referenced_type, .. } => {
                    self.create_typedef_type(name, referenced_type, index, location)
                }
                // Other types are just derived basic types
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }

    fn create_global_variable(
        &mut self,
        name: &str,
        type_name: &str,
        global_variable: GlobalValue<'ink>,
        location: &SourceLocation,
    ) {
        if let Some(debug_type) = self.types.get(&type_name.to_lowercase()) {
            let debug_type = *debug_type;
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
                debug_type.into(),
                false,
                None,
                None,
                global_variable.get_alignment(),
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
        scope: FunctionValue<'ink>,
    ) {
        let type_name = variable.get_type_name();
        let location = &variable.source_location;
        let file = location
            .get_file_name()
            .map(|it| self.get_or_create_debug_file(it))
            .unwrap_or_else(|| self.compile_unit.get_file());
        let line = location.get_line_plus_one() as u32;

        let scope = scope
            .get_subprogram()
            .map(|it| it.as_debug_info_scope())
            .unwrap_or_else(|| self.compile_unit.as_debug_info_scope());
        if let Some(debug_type) = self.types.get(&type_name.to_lowercase()) {
            let debug_variable = self.debug_info.create_auto_variable(
                scope,
                variable.get_name(),
                file,
                line,
                (*debug_type).into(),
                false,
                DIFlagsConstants::ZERO,
                alignment,
            );

            self.variables.insert(variable.get_qualified_name().to_string(), debug_variable);
        }
    }

    fn register_parameter(
        &mut self,
        variable: &VariableIndexEntry,
        arg_no: usize,
        scope: FunctionValue<'ink>,
    ) {
        let type_name = variable.get_type_name();
        let location = &variable.source_location;
        let file = location
            .get_file_name()
            .map(|it| self.get_or_create_debug_file(it))
            .unwrap_or_else(|| self.compile_unit.get_file());
        let line = location.get_line_plus_one() as u32;
        let scope = scope
            .get_subprogram()
            .map(|it| it.as_debug_info_scope())
            .unwrap_or_else(|| self.compile_unit.as_debug_info_scope());

        if let Some(debug_type) = self.types.get(&type_name.to_lowercase()) {
            let debug_variable = self.debug_info.create_parameter_variable(
                scope,
                variable.get_name(),
                arg_no as u32,
                file,
                line,
                (*debug_type).into(),
                false,
                DIFlagsConstants::ZERO,
            );

            self.variables.insert(variable.get_qualified_name().to_string(), debug_variable);
        }
    }

    fn register_struct_parameter(&mut self, pou: &PouIndexEntry, scope: FunctionValue<'ink>) {
        let scope = scope
            .get_subprogram()
            .map(|it| it.as_debug_info_scope())
            .unwrap_or_else(|| self.compile_unit.as_debug_info_scope());
        if let Some(debug_type) = self.types.get(&pou.get_name().to_lowercase()) {
            let debug_type = *debug_type;
            let file = pou
                .get_location()
                .get_file_name()
                .map(|it| self.get_or_create_debug_file(it))
                .unwrap_or_else(|| self.compile_unit.get_file());
            let line = pou.get_location().get_line_plus_one() as u32;
            let debug_variable = self.debug_info.create_parameter_variable(
                scope,
                pou.get_name(),
                0,
                file,
                line,
                debug_type.into(),
                false,
                DIFlagsConstants::ZERO,
            );
            self.variables.insert(pou.get_name().to_string(), debug_variable);
        }
    }

    fn add_variable_declaration(
        &self,
        name: &str,
        value: PointerValue<'ink>,
        scope: FunctionValue<'ink>,
        block: BasicBlock<'ink>,
        line: usize,
        column: usize,
    ) {
        let scope = scope
            .get_subprogram()
            .map(|it| it.as_debug_info_scope())
            .unwrap_or_else(|| self.compile_unit.as_debug_info_scope());
        let location = self.debug_info.create_debug_location(
            self.context,
            (line + 1) as u32,
            column as u32,
            scope,
            None,
        );
        self.debug_info.insert_declare_at_end(
            value,
            self.variables.get(name).copied(),
            None,
            location,
            block,
        );
    }

    fn finalize(&self) {
        self.debug_info.finalize();
    }
}

impl<'ink> Debug<'ink> for DebugBuilderEnum<'ink> {
    fn set_debug_location(&self, llvm: &Llvm, scope: &FunctionValue, line: usize, column: usize) {
        match self {
            Self::None | Self::VariablesOnly(..) => {}
            Self::Full(obj) => obj.set_debug_location(llvm, scope, line, column),
        };
    }

    fn register_function<'idx>(
        &mut self,
        index: &Index,
        func: FunctionValue<'ink>,
        pou: &PouIndexEntry,
        return_type: Option<&'idx DataType>,
        parameter_types: &[&'idx DataType],
        implementation_start: usize,
    ) {
        match self {
            Self::None | Self::VariablesOnly(..) => {}
            Self::Full(obj) => {
                obj.register_function(index, func, pou, return_type, parameter_types, implementation_start)
            }
        };
    }

    fn register_debug_type<'idx>(
        &mut self,
        name: &str,
        datatype: &'idx DataType,
        index: &'idx Index,
    ) -> Result<(), Diagnostic> {
        match self {
            Self::None => Ok(()),
            Self::VariablesOnly(obj) | Self::Full(obj) => obj.register_debug_type(name, datatype, index),
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
        scope: FunctionValue<'ink>,
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
        scope: FunctionValue<'ink>,
    ) {
        match self {
            Self::None | Self::VariablesOnly(_) => {}
            Self::Full(obj) => obj.register_parameter(variable, arg_no, scope),
        }
    }

    fn register_struct_parameter(&mut self, pou: &PouIndexEntry, scope: FunctionValue<'ink>) {
        match self {
            Self::None | Self::VariablesOnly(_) => {}
            Self::Full(obj) => obj.register_struct_parameter(pou, scope),
        }
    }

    fn add_variable_declaration(
        &self,
        name: &str,
        value: PointerValue<'ink>,
        scope: FunctionValue<'ink>,
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
