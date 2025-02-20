use std::{fmt::Debug, path::PathBuf};

use plc::DebugLevel;
use plc_diagnostics::{diagnostician::Diagnostician, diagnostics::Diagnostic, reporter::DiagnosticReporter};
use plc_index::GlobalContext;
use project::project::Project;
use source_code::SourceContainer;

use crate::{
    pipelines::{self, BuildPipeline, Pipeline},
    CompileOptions,
};

mod external_files;
mod multi_files;
mod validation;

pub fn compile_with_root<S, T>(
    sources: T,
    includes: T,
    root: &str,
    debug_level: DebugLevel,
) -> Result<Vec<String>, Diagnostic>
where
    S: SourceContainer + Debug,
    T: IntoIterator<Item = S>,
{
    compile_to_string(sources, includes, Some(root), debug_level)
}

pub fn compile_to_string<S, T>(
    sources: T,
    includes: T,
    root: Option<&str>,
    debug_level: DebugLevel,
) -> Result<Vec<String>, Diagnostic>
where
    S: SourceContainer + Debug,
    T: IntoIterator<Item = S>,
{
    let path: Option<PathBuf> = root.map(|it| it.into());
    let mut diagnostician = Diagnostician::null_diagnostician();
    //Create a project
    let project = Project::new("TestProject".into()).with_sources(sources).with_source_includes(includes);
    let ctxt = GlobalContext::new()
        .with_source(project.get_sources(), None)?
        .with_source(project.get_includes(), None)?;
    //Parse
    let compile_options = CompileOptions {
        root: path,
        debug_level,
        optimization: plc::OptimizationLevel::None,
        ..Default::default()
    };
    //TODO: participants
    pipelines::ParsedProject::parse(&ctxt, &project, &mut diagnostician)?
        //Index
        .index(ctxt.provider())
        .extend_with_init_units(&project.get_init_symbol_name(), ctxt.provider())
        //Resolve
        .annotate(ctxt.provider())
        //Codegen
        .codegen_to_string(&compile_options)
}

/// Parses and validates the given source with the `BuildPipeline` and default participants.
/// This function is meant to be used for integration tests to ensure our validations behave as expected.
pub fn parse_and_validate_buffered<T: SourceContainer>(source: T) -> String {
    let diagnostician = Diagnostician::buffered();
    let project = Project::new("TestProject".into()).with_sources(vec![source]);
    let context = GlobalContext::new().with_source(project.get_sources(), None).unwrap();

    let mut pipeline = BuildPipeline {
        context,
        project,
        diagnostician,
        compile_parameters: None,
        linker: plc::linker::LinkerType::Internal,
        mutable_participants: Default::default(),
        participants: Default::default(),
    };

    pipeline.register_default_participants();

    let project = pipeline.parse().unwrap();
    let project = pipeline.index(project).unwrap();
    let project = pipeline.annotate(project).unwrap();
    let _ = project.validate(&pipeline.context, &mut pipeline.diagnostician);

    pipeline.diagnostician.buffer().unwrap_or_default()
}
