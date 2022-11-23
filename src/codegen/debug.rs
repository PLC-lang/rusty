use std::{collections::HashMap, ops::Range, path::Path};

use inkwell::{
    context::Context,
    debug_info::{
        AsDIScope, DIBasicType, DICompileUnit, DICompositeType, DIDerivedType, DIFlags,
        DIFlagsConstants, DILocation, DIScope, DISubprogram, DISubroutineType, DIType,
        DWARFEmissionKind, DebugInfoBuilder, DILexicalBlock,
    },
    module::Module,
    values::{FunctionValue, GlobalValue},
};

use crate::{
    ast::{SourceRange, LinkageType},
    datalayout::{Bytes, MemoryLocation},
    diagnostics::Diagnostic,
    index::{Index, PouIndexEntry},
    typesystem::{DataType, DataTypeInformation, Dimension, StringEncoding, CHAR_TYPE, WCHAR_TYPE},
    DebugLevel, OptimizationLevel,
};

use super::generators::llvm::Llvm;

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

impl From<DWARFEmissionKind> for DebugLevel {
    fn from(kind: DWARFEmissionKind) -> Self {
        match kind {
            DWARFEmissionKind::Full => DebugLevel::Full,
            _ => DebugLevel::None,
        }
    }
}

impl From<DebugLevel> for DWARFEmissionKind {
    fn from(level: DebugLevel) -> Self {
        match level {
            DebugLevel::Full | DebugLevel::VariablesOnly => DWARFEmissionKind::Full,
            _ => DWARFEmissionKind::None,
        }
    }
}

/// A trait that represents a Debug builder
/// An implementor of this trais will be called during various codegen phases to generate debug
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
    ) -> Result<(), Diagnostic>;

    /// ...
    fn register_function<'idx>(
        &self,
        func: &FunctionValue,
        pou: &PouIndexEntry,
        return_type: Option<&'idx DataType>,
        parameter_types: &[&'idx DataType],
    ) -> Result<(), Diagnostic>;

    /// Registers a new datatype for debugging
    fn register_debug_type<'idx>(
        &mut self,
        name: &str,
        datatype: &'idx DataType,
        index: &'idx Index,
    ) -> Result<(), Diagnostic>;

    /// Creates a globally accessible variable with the given datatype.
    fn create_global_variable(
        &self,
        name: &str,
        type_name: &str,
        global_variable: GlobalValue<'ink>,
    ) -> Result<(), Diagnostic>;

    /// When code generation is done, this method needs to be called to ensure the inner LLVM state
    /// of the debug builder has been finalized.
    fn finalize(&self) -> Result<(), Diagnostic>;
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
    optimization: OptimizationLevel,
    //A stack of lexical blocks to write locations into
    scopes: Vec<DILexicalBlock<'ink>>,
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
        optimization: OptimizationLevel,
        debug_level: DebugLevel,
    ) -> Self {
        match debug_level {
            DebugLevel::None => DebugBuilderEnum::None,
            DebugLevel::VariablesOnly | DebugLevel::Full => {
                let path = Path::new(module.get_source_file_name().to_str().unwrap_or(""));
                let directory = path.parent().and_then(|it| it.to_str()).unwrap_or("");
                let filename = path.file_name().and_then(|it| it.to_str()).unwrap_or("");
                let (debug_info, compile_unit) = module.create_debug_info_builder(
                    true,
                    inkwell::debug_info::DWARFSourceLanguage::C, //TODO: Own lang
                    filename,
                    directory,
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
                    optimization,
                    scopes: Vec::new(),
                };
                match debug_level {
                    DebugLevel::VariablesOnly => DebugBuilderEnum::VariablesOnly(dbg_obj),
                    DebugLevel::Full => DebugBuilderEnum::Full(dbg_obj),
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
    ) -> Result<(), Diagnostic> {
        let res = self
            .debug_info
            .create_basic_type(name, size, encoding as u32, DIFlagsConstants::PUBLIC)
            .map_err(|err| Diagnostic::codegen_error(err, SourceRange::undefined()))?;
        self.register_concrete_type(name, DebugType::Basic(res));
        Ok(())
    }

    fn create_struct_type<T: AsRef<str>>(
        &mut self,
        name: &str,
        members: &[T],
        index: &Index,
    ) -> Result<(), Diagnostic> {
        //Create each type
        let index_types = members
            .iter()
            .map(|it| it.as_ref())
            .filter_map(|it| index.find_member(name, it))
            .map(|it| (it.get_name(), it.get_type_name()))
            .map(|(name, type_name)| index.get_type(type_name.as_ref()).map(|dt| (name, dt)))
            .collect::<Result<Vec<_>, Diagnostic>>()?;

        let mut types = vec![];
        let mut running_offset = MemoryLocation::new(0);
        for (member_name, dt) in index_types.into_iter() {
            let di_type = self.get_or_create_debug_type(dt, index)?;
            //Adjust the offset based on the field alignment
            let type_info = dt.get_type_information();
            let alignment = type_info.get_alignment(index);
            let size = type_info.get_size(index);
            running_offset = running_offset.align_to(alignment);
            types.push(
                self.debug_info
                    .create_member_type(
                        self.compile_unit.get_file().as_debug_info_scope(),
                        member_name,
                        self.compile_unit.get_file(),
                        0,
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
            self.compile_unit.get_file().as_debug_info_scope(),
            name,
            self.compile_unit.get_file(),
            0,
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
            .map_err(|err| Diagnostic::codegen_error(&err, SourceRange::undefined()))?;
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
            inkwell::AddressSpace::Global,
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
                Diagnostic::debug_error(format!("Cannot find debug information for type {dt_name}"))
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
    ) -> Result<(), Diagnostic> {
        let inner_dt = index.get_effective_type_by_name(referenced_type)?;
        let inner_type = self.get_or_create_debug_type(inner_dt, index)?;

        let typedef = self.debug_info.create_typedef(
            inner_type.into(),
            name,
            self.compile_unit.get_file(),
            0,
            self.compile_unit.get_file().as_debug_info_scope(),
            inner_dt.get_type_information().get_alignment(index).bits(),
        );
        self.register_concrete_type(name, DebugType::Derived(typedef));

        Ok(())
    }

    fn create_debug_location(
        &self,
        scope: DIScope<'ink>,
        line: usize,
        column: usize,
    ) -> Result<DILocation, Diagnostic> {
        Ok(self.debug_info.create_debug_location(
            self.context,
            line as u32,   // try_into() error msg on fail
            column as u32, // not implemented yet
            scope,
            None, // ?
        ))
    }

    fn create_subroutine_type(
        &self,
        return_type: Option<&DataType>,
        parameter_types: &[&DataType],
    ) -> Result<DISubroutineType, Diagnostic> {
        let return_type = return_type
            .and_then(|dt| self.types.get(dt.get_name()))
            .map(|it| it.to_owned())
            .map(Into::into);

        let parameter_types = parameter_types
            .iter()
            .map(|dt| {
                self.types
                    .get(dt.get_name().to_lowercase().as_str())
                    .expect("at this point we should have all types") // TODO: error msg
                    .to_owned()
            })
            .map(Into::into)
            .collect::<Vec<DIType>>();

        Ok(self.debug_info.create_subroutine_type(
            self.compile_unit.get_file(),
            return_type,
            &parameter_types,
            DIFlagsConstants::PUBLIC,
        ))
    }

    fn create_function(
        &self,
        pou: &PouIndexEntry,
        return_type: Option<&DataType>,
        parameter_types: &[&DataType],
    ) -> Result<DISubprogram, Diagnostic> {
        let is_external = matches!(pou.get_linkage(), LinkageType::External);
        let ditype = self.create_subroutine_type(return_type, parameter_types)?;
        Ok(self.debug_info.create_function(
            self.compile_unit.get_file().as_debug_info_scope(),
            pou.get_name(),
            Some(pou.get_name()), // for generics e.g. NAME__TYPE
            self.compile_unit.get_file(),
            pou.get_location().line_number as u32, // try_into() error msg on fail
            ditype,
            false,                                 // TODO: what is this 
            !is_external,
            pou.get_location().line_number as u32, // scope begins at declaration
            DIFlagsConstants::PUBLIC,
            self.optimization.is_optimized(),
        ))
    }
}

impl<'ink> Debug<'ink> for DebugBuilder<'ink> {
    fn set_debug_location(
        &self,
        llvm: &Llvm,
        scope: &FunctionValue,
        //Current line starts with 0
        line: usize,
        column: usize,
    ) -> Result<(), Diagnostic> {
        let scope = scope
            .get_subprogram()
            .map(|it| it.as_debug_info_scope())
            .unwrap_or_else(|| self.compile_unit.as_debug_info_scope());
        let location = self.create_debug_location(scope, line + 1, column)?;
        llvm.builder
            .set_current_debug_location(llvm.context, location);
        Ok(())
    }

    fn register_function<'idx>(
        &self,
        func: &FunctionValue,
        pou: &PouIndexEntry,
        return_type: Option<&'idx DataType>,
        parameter_types: &[&'idx DataType],
    ) -> Result<(), Diagnostic> {
        let subprogram = self.create_function(
            pou,
            return_type,
            parameter_types,
        )?;
        func.set_subprogram(subprogram);
        Ok(())
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
            match type_info {
                DataTypeInformation::Struct { member_names, .. } => {
                    self.create_struct_type(name, member_names.as_slice(), index)
                }
                DataTypeInformation::Array {
                    name,
                    inner_type_name,
                    dimensions,
                    ..
                } => self.create_array_type(
                    name,
                    inner_type_name,
                    dimensions,
                    size,
                    alignment,
                    index,
                ),
                DataTypeInformation::Pointer {
                    name,
                    inner_type_name,
                    ..
                } => self.create_pointer_type(name, inner_type_name, size, alignment, index),
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
                    self.create_basic_type(name, *size as u64, encoding)
                }
                DataTypeInformation::Float { size, .. } => {
                    self.create_basic_type(name, *size as u64, DebugEncoding::DW_ATE_float)
                }
                DataTypeInformation::String {
                    size: string_size,
                    encoding,
                    ..
                } => {
                    let length = string_size
                        .as_int_value(index)
                        .map_err(|err| Diagnostic::codegen_error(&err, SourceRange::undefined()))?;
                    self.create_string_type(name, length, *encoding, size, alignment, index)
                }
                DataTypeInformation::Alias {
                    name,
                    referenced_type,
                }
                | DataTypeInformation::Enum {
                    name,
                    referenced_type,
                    ..
                } => self.create_typedef_type(name, referenced_type, index),
                // Other types are just derived basic types
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }

    fn create_global_variable(
        &self,
        name: &str,
        type_name: &str,
        global_variable: GlobalValue<'ink>,
    ) -> Result<(), Diagnostic> {
        if let Some(debug_type) = self.types.get(&type_name.to_lowercase()) {
            let debug_variable = self.debug_info.create_global_variable_expression(
                self.compile_unit.get_file().as_debug_info_scope(),
                name,
                "",
                self.compile_unit.get_file(),
                0,
                (*debug_type).into(),
                false,
                None,
                None,
                global_variable.get_alignment(),
            );
            let gv_metadata = debug_variable.as_metadata_value(self.context);

            global_variable.set_metadata(gv_metadata, 0);
            self.context.metadata_node(&[gv_metadata.into()]);
        }

        Ok(())
    }

    fn finalize(&self) -> Result<(), Diagnostic> {
        self.debug_info.finalize();
        Ok(())
    }
}

impl<'ink> Debug<'ink> for DebugBuilderEnum<'ink> {
    fn set_debug_location(
        &self,
        llvm: &Llvm,
        scope: &FunctionValue,
        line: usize,
        column: usize,
    ) -> Result<(), Diagnostic> {
        match self {
            Self::None | Self::VariablesOnly(..) => Ok(()),
            Self::Full(obj) => obj.set_debug_location(llvm, scope, line, column),
        }
    }

    fn register_function<'idx>(
        &self,
        func: &FunctionValue,
        pou: &PouIndexEntry,
        return_type: Option<&'idx DataType>,
        parameter_types: &[&'idx DataType],
    ) -> Result<(), Diagnostic> {
        match self {
            Self::None | Self::VariablesOnly(..) => Ok(()),
            Self::Full(obj) => obj.register_function(func, pou, return_type, parameter_types),
        }
    }

    fn register_debug_type<'idx>(
        &mut self,
        name: &str,
        datatype: &'idx DataType,
        index: &'idx Index,
    ) -> Result<(), Diagnostic> {
        match self {
            Self::None => Ok(()),
            Self::VariablesOnly(obj) | Self::Full(obj) => {
                obj.register_debug_type(name, datatype, index)
            }
        }
    }

    fn create_global_variable(
        &self,
        name: &str,
        type_name: &str,
        global_variable: GlobalValue<'ink>,
    ) -> Result<(), Diagnostic> {
        match self {
            Self::None => Ok(()),
            Self::VariablesOnly(obj) | Self::Full(obj) => {
                obj.create_global_variable(name, type_name, global_variable)
            }
        }
    }

    fn finalize(&self) -> Result<(), Diagnostic> {
        match self {
            Self::None => Ok(()),
            Self::VariablesOnly(obj) | Self::Full(obj) => obj.finalize(),
        }
    }
}
