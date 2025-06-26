use std::io::Read;

use driver::{cli, pipelines::BuildPipeline};
use plc::{linker::LinkerType, DebugLevel};
use plc_ast::ast::CompilationUnit;
use plc_diagnostics::{diagnostician::Diagnostician, reporter::DiagnosticReporter};
use plc_index::GlobalContext;
use plc_source::SourceCode;
use project::project::Project;
use tempfile::NamedTempFile;

pub fn codegen(src: &str) -> String {
    codegen_without_unwrap(src).map_err(|it| panic!("{it}")).unwrap()
}

pub fn codegen_with_debug(src: &str) -> String {
    codegen_debug_without_unwrap(src, DebugLevel::Full(5)).map_err(|it| panic!("{it}")).unwrap()
}
pub fn codegen_with_debug_version(src: &str, version: usize) -> String {
    codegen_debug_without_unwrap(src, DebugLevel::Full(version)).map_err(|it| panic!("{it}")).unwrap()
}
pub fn codegen_without_unwrap(src: &str) -> Result<String, String> {
    codegen_debug_without_unwrap(src, DebugLevel::None)
}

pub fn parse_and_validate_buffered(src: &str) -> String {
    let source: SourceCode = src.into();
    driver::parse_and_validate("TestProject", vec![source])
}

pub fn parse_and_validate_buffered_ast(src: &str) -> Vec<CompilationUnit> {
    let source: SourceCode = src.into();

    match driver::parse_and_annotate_with_diagnostics("TestProject", vec![source], Diagnostician::buffered())
    {
        Ok((mut pipeline, project)) => {
            project.validate(&pipeline.context, &mut pipeline.diagnostician).unwrap();
            project.units.into_iter().map(CompilationUnit::from).collect()
        }
        Err(diagnostician) => panic!("{}", diagnostician.buffer().unwrap()),
    }
}

fn get_debug_param(debug_level: DebugLevel) -> Option<String> {
    match debug_level {
        DebugLevel::None => None,
        DebugLevel::VariablesOnly(plc::DEFAULT_DWARF_VERSION) => Some("--debug-variables".to_string()),
        DebugLevel::VariablesOnly(version) => Some(format!("--gdwarf-variables={version}")),
        DebugLevel::Full(plc::DEFAULT_DWARF_VERSION) => Some("--debug".to_string()),
        DebugLevel::Full(version) => Some(format!("--gdwarf={version}")),
    }
}

pub fn codegen_debug_without_unwrap(src: &str, debug_level: DebugLevel) -> Result<String, String> {
    //Create a temp file to store the result
    let mut output = NamedTempFile::new().map_err(|it| it.to_string())?;
    let src: SourceCode = src.into();
    let project = Project::new("Test".to_string())
        .with_sources(vec![src])
        .with_output_name(output.path().to_str().map(ToString::to_string));
    let context =
        GlobalContext::new().with_source(project.get_sources(), None).map_err(|it| it.to_string())?;
    let diagnostician = Diagnostician::default();
    let mut args =
        vec!["plc", "--ir", "--single-module", "-O", "none", "-o", output.path().to_str().unwrap()];
    let debug_level = get_debug_param(debug_level);
    if let Some(debug) = &debug_level {
        args.push(debug);
    };
    let params = cli::CompileParameters::parse(&args).map_err(|e| e.to_string())?;
    let pipeline = BuildPipeline {
        context,
        project,
        diagnostician,
        compile_parameters: Some(params),
        linker: LinkerType::Internal,
        mutable_participants: Vec::default(),
        participants: Vec::default(),
        module_name: Some("<internal>".to_string()),
    };

    driver::compile_with_pipeline(pipeline).map_err(|it| it.to_string())?;

    let mut res = String::new();
    output.read_to_string(&mut res).map_err(|it| it.to_string())?;
    Ok(res)
}
