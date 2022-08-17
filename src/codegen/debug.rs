use inkwell::{
    debug_info::{
        AsDIScope, DICompileUnit, DIFlagsConstants, DIGlobalVariableExpression, DIType,
        DWARFEmissionKind, DebugInfoBuilder,
    },
    module::Module,
    values::GlobalValue,
};

use crate::{
    ast::SourceRange, diagnostics::Diagnostic, typesystem::BOOL_SIZE, DebugLevel, OptimizationLevel,
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
    fn create_int_type(
        &self,
        name: &str,
        size: u32,
        is_signed: bool,
    ) -> Result<Option<DIType<'ink>>, Diagnostic>;
    fn create_bool_type(&self, name: &str) -> Result<Option<DIType<'ink>>, Diagnostic>;
    fn create_float_type(&self, name: &str, size: u32) -> Result<Option<DIType<'ink>>, Diagnostic>;
    fn create_global_variable(
        &self,
        name: &str,
        debug_type: DIType<'ink>,
        global_variable: GlobalValue<'ink>,
    ) -> Result<Option<DIGlobalVariableExpression<'ink>>, Diagnostic>;

    fn finalize(&self);
}

pub struct DebugObj<'ink> {
    debug_info: DebugInfoBuilder<'ink>,
    compile_unit: DICompileUnit<'ink>,
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
        debug_info,
        compile_unit,
    }
}

impl<'ink> Debug<'ink> for DebugObj<'ink> {
    fn create_int_type(
        &self,
        name: &str,
        size: u32,
        is_signed: bool,
    ) -> Result<Option<DIType<'ink>>, Diagnostic> {
        let encoding = match is_signed {
            true => DebugEncoding::DW_ATE_signed,
            false => DebugEncoding::DW_ATE_unsigned,
        };
        self.debug_info
            .create_basic_type(name, size as u64, encoding.into(), DIFlagsConstants::PUBLIC)
            .map(|it| Some(it.as_type()))
            .map_err(|err| Diagnostic::codegen_error(err, SourceRange::undefined()))
    }

    fn create_bool_type(&self, name: &str) -> Result<Option<DIType<'ink>>, Diagnostic> {
        self.debug_info
            .create_basic_type(
                name,
                BOOL_SIZE as u64,
                DebugEncoding::DW_ATE_boolean.into(),
                DIFlagsConstants::PUBLIC,
            )
            .map(|it| Some(it.as_type()))
            .map_err(|err| Diagnostic::codegen_error(err, SourceRange::undefined()))
    }

    fn create_float_type(&self, name: &str, size: u32) -> Result<Option<DIType<'ink>>, Diagnostic> {
        let encoding = DebugEncoding::DW_ATE_float;
        self.debug_info
            .create_basic_type(name, size as u64, encoding.into(), DIFlagsConstants::PUBLIC)
            .map(|it| Some(it.as_type()))
            .map_err(|err| Diagnostic::codegen_error(err, SourceRange::undefined()))
    }

    fn create_global_variable(
        &self,
        name: &str,
        debug_type: DIType<'ink>,
        global_variable: GlobalValue<'ink>,
    ) -> Result<Option<DIGlobalVariableExpression<'ink>>, Diagnostic> {
        let gv = self.debug_info.create_global_variable_expression(
            self.compile_unit.get_file().as_debug_info_scope(),
            name,
            "",
            self.compile_unit.get_file(),
            0,
            debug_type,
            true,
            None,
            None,
            global_variable.get_alignment(),
        );

        Ok(Some(gv))
    }

    fn finalize(&self) {
        self.debug_info.finalize()
    }
}

impl<'ink, T: Debug<'ink>> Debug<'ink> for Option<T> {
    fn create_int_type(
        &self,
        name: &str,
        size: u32,
        is_signed: bool,
    ) -> Result<Option<DIType<'ink>>, Diagnostic> {
        match self {
            Self::None => Ok(None),
            Self::Some(obj) => obj.create_int_type(name, size, is_signed),
        }
    }

    fn create_bool_type(&self, name: &str) -> Result<Option<DIType<'ink>>, Diagnostic> {
        match self {
            Self::None => Ok(None),
            Self::Some(obj) => obj.create_bool_type(name),
        }
    }

    fn create_float_type(&self, name: &str, size: u32) -> Result<Option<DIType<'ink>>, Diagnostic> {
        match self {
            Self::None => Ok(None),
            Self::Some(obj) => obj.create_float_type(name, size),
        }
    }

    fn create_global_variable(
        &self,
        name: &str,
        debug_type: DIType<'ink>,
        global_variable: GlobalValue<'ink>,
    ) -> Result<Option<DIGlobalVariableExpression<'ink>>, Diagnostic> {
        match self {
            Self::None => Ok(None),
            Self::Some(obj) => obj.create_global_variable(name, debug_type, global_variable),
        }
    }

    fn finalize(&self) {
        match self {
            Self::None => {}
            Self::Some(obj) => obj.finalize(),
        }
    }
}
