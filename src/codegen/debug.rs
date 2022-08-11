use inkwell::{
    debug_info::{DICompileUnit, DWARFEmissionKind, DebugInfoBuilder},
    module::Module,
};

use crate::OptimizationLevel;

pub enum DebugLevel {
    None,
    VariablesOnly,
    Full,
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

pub struct Debug<'ink> {
    debug_info: DebugInfoBuilder<'ink>,
    compile_unit: DICompileUnit<'ink>,
}

impl<'ink> Debug<'ink> {
    pub fn new(
        module: &Module<'ink>,
        optimization: OptimizationLevel,
        debug_level: DebugLevel,
    ) -> Debug<'ink> {
        /*
        allow_unresolved: bool,
        language: DWARFSourceLanguage,
        filename: &str,
        directory: &str,
        producer: &str,
        is_optimized: bool,
        flags: &str,
        runtime_ver: libc::c_uint,
        split_name: &str,
        kind: DWARFEmissionKind,
        dwo_id: libc::c_uint,
        split_debug_inlining: bool,
        debug_info_for_profiling: bool,
         */
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
        Debug {
            debug_info,
            compile_unit,
        }
    }
}
