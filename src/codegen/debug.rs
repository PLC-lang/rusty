use std::{collections::HashMap, cell::RefCell};

use inkwell::{
    debug_info::{
        AsDIScope, DICompileUnit, DIFlagsConstants, DIType,
        DWARFEmissionKind, DebugInfoBuilder,
    },
    module::Module,
    values::GlobalValue, context::Context,
};

use crate::{
    ast::SourceRange, diagnostics::Diagnostic, typesystem::{BOOL_SIZE, DataTypeInformation}, DebugLevel, OptimizationLevel,
};

#[derive(PartialEq, Eq)]
#[allow(non_camel_case_types)]
enum DebugEncoding {
    DW_ATE_boolean,
    DW_ATE_signed,
    DW_ATE_unsigned,
    DW_ATE_float,
}

impl From<DebugEncoding> for u32 {
    fn from(enc: DebugEncoding) -> Self {
        match enc {
            DebugEncoding::DW_ATE_boolean => 0x02,
            DebugEncoding::DW_ATE_signed => 0x05,
            DebugEncoding::DW_ATE_unsigned => 0x07,
            DebugEncoding::DW_ATE_float => 0x04,
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
        name : &str,
        datatype : &'idx DataTypeInformation,
    ) -> Result<(), Diagnostic>;

    fn expand_struct_type(&self, 
        name: &str,
        members: &[&str]
    );

    fn create_global_variable(
        &self,
        name: &str,
        type_name: &str,
        global_variable: GlobalValue<'ink>,
    ) -> Result<(), Diagnostic>;

    fn finalize(&self);
}

pub struct DebugObj<'ink> {
    context: &'ink Context,
    debug_info: DebugInfoBuilder<'ink>,
    compile_unit: DICompileUnit<'ink>,
    types: RefCell<HashMap<String, DIType<'ink>>>,
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
        context: &module.get_context(),
        debug_info,
        compile_unit,
        types : Default::default(),
    }
}

impl<'ink> DebugObj<'ink> {
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
        let res = self.debug_info
            .create_basic_type(name, size as u64, encoding.into(), DIFlagsConstants::PUBLIC)
            .map_err(|err| Diagnostic::codegen_error(err, SourceRange::undefined()))?;

        self.types.borrow_mut().insert(name.to_lowercase(), res.as_type());
        Ok(())
    }

    fn create_bool_type(&self, name: &str) -> Result<(), Diagnostic> {
        let res = self.debug_info
            .create_basic_type(
                name,
                BOOL_SIZE as u64,
                DebugEncoding::DW_ATE_boolean.into(),
                DIFlagsConstants::PUBLIC,
            )
            .map_err(|err| Diagnostic::codegen_error(err, SourceRange::undefined()))?;

        self.types.borrow_mut().insert(name.to_lowercase(), res.as_type());
        Ok(())
    }

    fn create_float_type(&self, name: &str, size: u32) -> Result<(), Diagnostic> {
        let encoding = DebugEncoding::DW_ATE_float;
        let res = self.debug_info
            .create_basic_type(name, size as u64, encoding.into(), DIFlagsConstants::PUBLIC)
            .map_err(|err| Diagnostic::codegen_error(err, SourceRange::undefined()))?;

        self.types.borrow_mut().insert(name.to_lowercase(), res.as_type());
        Ok(())
    }

    fn create_struct_type(&self, name: &str, members: &[&str]) -> Result<(), Diagnostic> {
        todo!()
    }

    fn create_array_type(&self, name: &str, inner_type: &str, elements: u32) -> Result<(), Diagnostic> {
        //No array support in inkwell yet
        Ok(())
    }

    fn create_pointer_type(&self, name: &str, inner_type: &str) -> Result<(), Diagnostic> {
        todo!()
    }


}

impl<'ink> Debug<'ink> for DebugObj<'ink> {
    fn register_debug_type<'idx>(
        &self,
        name : &str,
        datatype : &'idx DataTypeInformation,
    ) -> Result<(), Diagnostic> {
        match datatype {
            DataTypeInformation::Struct { .. } => {
                // This _needs_ to be replaced once the types are created
                let res = unsafe {
                    self.debug_info.create_placeholder_derived_type(self.context)
                };
                self.types.borrow_mut().insert(name.to_lowercase(), res.as_type());
                Ok(())
            },
            DataTypeInformation::Array { .. } => todo!(),
            DataTypeInformation::Pointer { .. } => todo!(),
            DataTypeInformation::Integer { signed, size, ..} => {
                if datatype.is_bool() {
                    self.create_bool_type(name)
                } else {
                    self.create_int_type(name, *size, *signed)
                }
            },
            DataTypeInformation::Float { size, .. } => self.create_float_type(name, *size),
            DataTypeInformation::String { .. } => todo!(),
            // Other types are just derived basic types
            _ => Ok(())
        }
    }

    fn create_global_variable(
        &self,
        name: &str,
        type_name : &str,
        global_variable: GlobalValue<'ink>,
    ) -> Result<(), Diagnostic> {
        if let Some(debug_type) = self.types.borrow().get(&type_name.to_lowercase()) {
            self.debug_info.create_global_variable_expression(
                self.compile_unit.get_file().as_debug_info_scope(),
                name,
                "",
                self.compile_unit.get_file(),
                0,
                *debug_type,
                true,
                None,
                None,
                global_variable.get_alignment(),
            );

        }

        Ok(())
    }

    fn finalize(&self) {
        self.debug_info.finalize()
    }

    fn expand_struct_type(&self, 
        name: &str,
        members: &[&str],
        index: &Index,
    ) {
        //Find each struct member created previously
        //Create a struct type 
        let res = self.debug_info.create_member_type()
    }
}

impl<'ink, T: Debug<'ink>> Debug<'ink> for Option<T> {
    fn register_debug_type<'idx>(
        &self,
        name : &str,
        datatype : &'idx DataTypeInformation,
    ) -> Result<(), Diagnostic> {
        match self {
            Self::None => Ok(()),
            Self::Some(debug) => debug.register_debug_type(name, datatype),
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

    fn finalize(&self) {
        match self {
            Self::None => {}
            Self::Some(obj) => obj.finalize(),
        }
    }


    fn expand_struct_type(&self, 
        name: &str,
        members: &[&str]
    ) {
        todo!()
    }
}
