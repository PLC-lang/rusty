use std::{cell::RefCell, collections::HashMap};

use inkwell::{
    debug_info::{
        AsDIScope, DIBasicType, DICompileUnit, DICompositeType, DIFlags,
        DIFlagsConstants, DIType, DWARFEmissionKind, DebugInfoBuilder,
    },
    module::Module,
    values::GlobalValue,
};

use crate::{
    ast::SourceRange,
    datalayout::Size,
    diagnostics::Diagnostic,
    index::Index,
    typesystem::{DataType, DataTypeInformation, BOOL_SIZE},
    DebugLevel, OptimizationLevel,
};

#[derive(PartialEq, Eq)]
#[allow(non_camel_case_types)]
enum DebugEncoding {
    // DW_ATE_address,
    DW_ATE_boolean,
    DW_ATE_float,
    DW_ATE_signed,
    DW_ATE_unsigned,
    // DW_ATE_UTF,
}

impl From<DebugEncoding> for u32 {
    fn from(enc: DebugEncoding) -> Self {
        match enc {
            // DebugEncoding::DW_ATE_address => 0x01,
            DebugEncoding::DW_ATE_boolean => 0x02,
            DebugEncoding::DW_ATE_float => 0x04,
            DebugEncoding::DW_ATE_signed => 0x05,
            DebugEncoding::DW_ATE_unsigned => 0x07,
            // DebugEncoding::DW_ATE_UTF => 0x10,
        }
    }
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
    BasicType(DIBasicType<'ink>),
    StructType(DICompositeType<'ink>),
    // DerivedType(DIDerivedType<'ink>),
    // Placeholder {
    //     name: &'ink str,
    //     offset: &'ink str,
    //     derived_type: DIDerivedType<'ink>,
    // },
}

impl<'ink> Into<DIType<'ink>> for DebugType<'ink> {
    fn into(self) -> DIType<'ink> {
        match self {
            DebugType::BasicType(t) => t.as_type(),
            DebugType::StructType(t) => t.as_type(),
            // DebugType::DerivedType(t) => t.as_type(),
            // | DebugType::Placeholder {
            //     derived_type: t, ..
            // } => t.as_type(),
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
        // if let Some(DebugType::Placeholder {
        //     derived_type: placeholder,
        //     ..
        // }) = 
        self.types.borrow_mut().insert(name.to_lowercase(), di_type);
        // {
        //     unsafe {
        //         match di_type {
        //             DebugType::Placeholder { derived_type, .. } => self
        //                 .debug_info
        //                 .replace_placeholder_derived_type(placeholder, derived_type),
        //             _ => {}
        //         }
        //     }
        // }
    }

    fn create_int_type<'idx>(
        &self,
        name: &'idx str,
        size: u32,
        is_signed: bool,
    ) -> Result<(), Diagnostic> {
        let encoding = match is_signed {
            true => DebugEncoding::DW_ATE_signed,
            false => DebugEncoding::DW_ATE_unsigned,
        };
        let res = self
            .debug_info
            .create_basic_type(name, size as u64, encoding.into(), DIFlagsConstants::PUBLIC)
            .map_err(|err| Diagnostic::codegen_error(err, SourceRange::undefined()))?;
        self.register_concrete_type(name, DebugType::BasicType(res));
        Ok(())
    }

    fn create_bool_type(&self, name: &str) -> Result<(), Diagnostic> {
        let res = self
            .debug_info
            .create_basic_type(
                name,
                BOOL_SIZE as u64,
                DebugEncoding::DW_ATE_boolean.into(),
                DIFlagsConstants::PUBLIC,
            )
            .map_err(|err| Diagnostic::codegen_error(err, SourceRange::undefined()))?;

        self.register_concrete_type(name, DebugType::BasicType(res));
        Ok(())
    }

    fn create_float_type(&self, name: &str, size: u32) -> Result<(), Diagnostic> {
        let encoding = DebugEncoding::DW_ATE_float;
        let res = self
            .debug_info
            .create_basic_type(name, size as u64, encoding.into(), DIFlagsConstants::PUBLIC)
            .map_err(|err| Diagnostic::codegen_error(err, SourceRange::undefined()))?;

        self.register_concrete_type(name, DebugType::BasicType(res));
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
            .map(|it| index.find_member(name, it))
            .flatten()
            .map(|it| (it.get_name(), it.get_type_name()))
            .map(|(name, type_name)| index.get_type(type_name.as_ref()).map(|dt| (name, dt)))
            .collect::<Result<Vec<_>, Diagnostic>>()?;

        let mut types = vec![];
        let mut running_offset = Size::from_bytes(0);
        for (member_name, dt) in index_types.into_iter() {
            //Try to find a type in the types
            let dt_name = dt.get_name().to_lowercase();
            //Attempt to re-register the type, this will do nothing if the type exists.
            //TODO: This will crash on recursive datatypes
            self.register_debug_type(&dt_name, dt, index)?;
            let di_type = self
                .types
                .borrow()
                .get(&dt_name)
                .ok_or_else(|| {
                    Diagnostic::debug_error(format!(
                        "Cannot find debug information for type {dt_name}"
                    ))
                })?
                .to_owned();
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
            running_offset = Size::from_bytes(running_offset.bytes() + size.bytes());
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

        self.register_concrete_type(name, DebugType::StructType(struct_type));
        Ok(())
    }

    //fn create_array_type(
    //    &self,
    //    name: &str,
    //    inner_type: &str,
    //    elements: u32,
    //) -> Result<(), Diagnostic> {
    //    //No array support in inkwell yet
    //    Ok(())
    //}

    //fn create_pointer_type(&self, name: &str, inner_type: &str) -> Result<(), Diagnostic> {
    //    //No pointer support in inkwell yet
    //    Ok(())
    //}
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
            match datatype.get_type_information() {
                DataTypeInformation::Struct { member_names, .. } => {
                    self.create_struct_type(name, member_names.as_slice(), index)
                }
                // DataTypeInformation::Array { .. } => todo!(),
                // DataTypeInformation::Pointer { .. } => todo!(),
                DataTypeInformation::Integer { signed, size, .. } => {
                    if datatype.get_type_information().is_bool() {
                        self.create_bool_type(name)
                    } else {
                        self.create_int_type(name, *size, *signed)
                    }
                }
                DataTypeInformation::Float { size, .. } => self.create_float_type(name, *size),
                // DataTypeInformation::String { .. } => !(),
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
