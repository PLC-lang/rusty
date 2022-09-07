use std::{cell::RefCell, collections::HashMap, ops::Range};

use inkwell::{
    debug_info::{
        AsDIScope, DIBasicType, DICompileUnit, DICompositeType, DIDerivedType, DIFlags,
        DIFlagsConstants, DIType, DWARFEmissionKind, DebugInfoBuilder,
    },
    module::Module,
    values::GlobalValue,
};

use crate::{
    ast::SourceRange,
    datalayout::Byte,
    diagnostics::Diagnostic,
    index::Index,
    typesystem::{DataType, DataTypeInformation, StringEncoding, BOOL_SIZE, CHAR_TYPE, WCHAR_TYPE},
    DebugLevel, OptimizationLevel,
};

#[derive(PartialEq, Eq)]
#[allow(non_camel_case_types)]
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

pub trait Debug<'ink> {
    fn register_debug_type<'idx>(
        &self,
        name: &str,
        datatype: &'idx DataType,
        index: &'idx Index,
    ) -> Result<(), Diagnostic>;

    fn create_global_variable(
        &self,
        name: &str,
        type_name: &str,
        global_variable: GlobalValue<'ink>,
    ) -> Result<(), Diagnostic>;

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

pub struct DebugObj<'ink> {
    // context: ContextRef<'ink>,
    debug_info: DebugInfoBuilder<'ink>,
    compile_unit: DICompileUnit<'ink>,
    types: RefCell<HashMap<String, DebugType<'ink>>>,
}

pub fn new<'ink>(
    module: &Module<'ink>,
    optimization: OptimizationLevel,
    debug_level: DebugLevel,
) -> DebugObj<'ink> {
    let (debug_info, compile_unit) = module.create_debug_info_builder(
        true,
        inkwell::debug_info::DWARFSourceLanguage::C,
        module.get_source_file_name().to_str().unwrap_or(""),
        "",
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
    DebugObj {
        // context: module.get_context(),
        debug_info,
        compile_unit,
        types: Default::default(),
    }
}

impl<'ink> DebugObj<'ink> {
    fn register_concrete_type(&self, name: &str, di_type: DebugType<'ink>) {
        self.types.borrow_mut().insert(name.to_lowercase(), di_type);
    }

    fn create_int_type(&self, name: &str, size: u32, is_signed: bool) -> Result<(), Diagnostic> {
        let encoding = match is_signed {
            true => DebugEncoding::DW_ATE_signed,
            false => DebugEncoding::DW_ATE_unsigned,
        };
        let res = self
            .debug_info
            .create_basic_type(name, size as u64, encoding as u32, DIFlagsConstants::PUBLIC)
            .map_err(|err| Diagnostic::codegen_error(err, SourceRange::undefined()))?;
        self.register_concrete_type(name, DebugType::Basic(res));
        Ok(())
    }

    fn create_bool_type(&self, name: &str) -> Result<(), Diagnostic> {
        let res = self
            .debug_info
            .create_basic_type(
                name,
                BOOL_SIZE as u64,
                DebugEncoding::DW_ATE_boolean as u32,
                DIFlagsConstants::PUBLIC,
            )
            .map_err(|err| Diagnostic::codegen_error(err, SourceRange::undefined()))?;

        self.register_concrete_type(name, DebugType::Basic(res));
        Ok(())
    }

    fn create_float_type(&self, name: &str, size: u32) -> Result<(), Diagnostic> {
        let encoding = DebugEncoding::DW_ATE_float;
        let res = self
            .debug_info
            .create_basic_type(name, size as u64, encoding as u32, DIFlagsConstants::PUBLIC)
            .map_err(|err| Diagnostic::codegen_error(err, SourceRange::undefined()))?;

        self.register_concrete_type(name, DebugType::Basic(res));
        Ok(())
    }

    fn create_char_type(&self, name: &str, size: u32) -> Result<(), Diagnostic> {
        let res = self
            .debug_info
            .create_basic_type(
                name,
                size as u64,
                DebugEncoding::DW_ATE_UTF as u32,
                DIFlagsConstants::PUBLIC,
            )
            .map_err(|err| Diagnostic::codegen_error(err, SourceRange::undefined()))?;
        self.register_concrete_type(name, DebugType::Basic(res));
        Ok(())
    }

    fn create_struct_type<T: AsRef<str>>(
        &self,
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
        let mut running_offset = Byte::new(0);
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
        &self,
        array: &DataTypeInformation,
        index: &Index,
    ) -> Result<(), Diagnostic> {
        let (name, inner_type, dimensions) = if let DataTypeInformation::Array {
            name,
            inner_type_name,
            dimensions,
            ..
        } = array
        {
            (name, inner_type_name, dimensions)
        } else {
            unreachable!("Type info should be an array")
        };
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
            array.get_size_in_bits(index).into(),
            array.get_alignment(index).bits(),
            subscript.as_slice(),
        );
        self.register_concrete_type(name, DebugType::Composite(array_type));
        Ok(())
    }

    fn create_pointer_type(
        &self,
        pointer: &DataTypeInformation,
        index: &Index,
    ) -> Result<(), Diagnostic> {
        if let DataTypeInformation::Pointer {
            name,
            inner_type_name,
            ..
        } = pointer
        {
            let inner_type = index.get_type(inner_type_name)?;
            let inner_type = self.get_or_create_debug_type(inner_type, index)?;
            let pointer_type = self.debug_info.create_pointer_type(
                name,
                inner_type.into(),
                pointer.get_size_in_bits(index).into(),
                pointer.get_alignment(index).bits(),
                inkwell::AddressSpace::Global,
            );
            self.register_concrete_type(name, DebugType::Derived(pointer_type));
        } else {
            unreachable!("Type should be pointer")
        }
        Ok(())
    }

    fn get_or_create_debug_type(
        &self,
        dt: &DataType,
        index: &Index,
    ) -> Result<DebugType<'ink>, Diagnostic> {
        //Try to find a type in the types
        let dt_name = dt.get_name().to_lowercase();
        //Attempt to re-register the type, this will do nothing if the type exists.
        //TODO: This will crash on recursive datatypes
        self.register_debug_type(&dt_name, dt, index)?;
        self.types
            .borrow()
            .get(&dt_name)
            .ok_or_else(|| {
                Diagnostic::debug_error(format!("Cannot find debug information for type {dt_name}"))
            })
            .map(|it| it.to_owned())
    }

    fn create_string_type(
        &self,
        name: &str,
        string: &DataTypeInformation,
        index: &Index,
    ) -> Result<(), Diagnostic> {
        //Get encoding
        let (size, encoding) = if let DataTypeInformation::String { size, encoding, .. } = string {
            (
                size.as_int_value(index)
                    .map_err(|err| Diagnostic::codegen_error(&err, SourceRange::undefined()))?,
                encoding,
            )
        } else {
            unreachable!("Should be string")
        };
        //Calculate target size
        let string_size = string.get_size_in_bits(index);
        // Register a utf8 or 16 basic type
        let inner_type = match encoding {
            StringEncoding::Utf8 => index.get_effective_type_or_void_by_name(CHAR_TYPE),
            StringEncoding::Utf16 => index.get_effective_type_or_void_by_name(WCHAR_TYPE),
        };
        let inner_type = self.get_or_create_debug_type(inner_type, index)?;
        //Register an array
        let array_type = self.debug_info.create_array_type(
            inner_type.into(),
            string_size.into(),
            string.get_alignment(index).bits(),
            &[(0..(size - 1))],
        );
        self.register_concrete_type(name, DebugType::Composite(array_type));
        Ok(())
    }

    fn create_typedef_type(
        &self,
        dt: &DataTypeInformation,
        index: &Index,
    ) -> Result<(), Diagnostic> {
        if let DataTypeInformation::Alias {
            name,
            referenced_type,
        } = dt
        {
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
        } else {
            unreachable!()
        }

        Ok(())
    }
}

impl<'ink> Debug<'ink> for DebugObj<'ink> {
    fn register_debug_type<'idx>(
        &self,
        name: &str,
        datatype: &'idx DataType,
        index: &'idx Index,
    ) -> Result<(), Diagnostic> {
        //check if the type is currently registered
        if !self.types.borrow().contains_key(&name.to_lowercase()) {
            let type_info = datatype.get_type_information();
            match type_info {
                DataTypeInformation::Struct { member_names, .. } => {
                    self.create_struct_type(name, member_names.as_slice(), index)
                }
                DataTypeInformation::Array { .. } => self.create_array_type(type_info, index),
                DataTypeInformation::Pointer { .. } => self.create_pointer_type(type_info, index),
                DataTypeInformation::Integer { signed, size, .. } => {
                    if type_info.is_bool() {
                        self.create_bool_type(name)
                    } else if type_info.is_character() {
                        self.create_char_type(name, *size)
                    } else {
                        self.create_int_type(name, *size, *signed)
                    }
                }
                DataTypeInformation::Float { size, .. } => self.create_float_type(name, *size),
                DataTypeInformation::String { .. } => {
                    self.create_string_type(name, type_info, index)
                }
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
        if let Some(debug_type) = self.types.borrow().get(&type_name.to_lowercase()) {
            self.debug_info.create_global_variable_expression(
                self.compile_unit.get_file().as_debug_info_scope(),
                name,
                "",
                self.compile_unit.get_file(),
                0,
                (*debug_type).into(),
                true,
                None,
                None,
                global_variable.get_alignment(),
            );
        }

        Ok(())
    }

    fn finalize(&self) -> Result<(), Diagnostic> {
        // if self
        //     .types
        //     .borrow()
        //     .values()
        //     .any(|it| matches!(it, DebugType::Placeholder(_)))
        // {
        //     Err(Diagnostic::debug_error(
        //         "Not all types were resolved by the type for finalize",
        //     ))
        // } else {
        self.debug_info.finalize();
        Ok(())
        // }
    }
}

impl<'ink, T: Debug<'ink>> Debug<'ink> for Option<T> {
    fn register_debug_type<'idx>(
        &self,
        name: &str,
        datatype: &'idx DataType,
        index: &'idx Index,
    ) -> Result<(), Diagnostic> {
        match self {
            Self::None => Ok(()),
            Self::Some(debug) => debug.register_debug_type(name, datatype, index),
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
            Self::Some(obj) => obj.create_global_variable(name, type_name, global_variable),
        }
    }

    fn finalize(&self) -> Result<(), Diagnostic> {
        match self {
            Self::None => Ok(()),
            Self::Some(obj) => obj.finalize(),
        }
    }
}
