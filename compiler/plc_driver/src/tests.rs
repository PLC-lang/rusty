use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use plc::{codegen::CodegenContext, DebugLevel};
use plc_diagnostics::{diagnostician::Diagnostician, diagnostics::Diagnostic};
use plc_index::GlobalContext;
use project::project::Project;
use serde::{Deserialize, Serialize};
use source_code::SourceContainer;

use crate::{
    pipelines::{self, AnnotatedProject, BuildPipeline, IndexedProject, ParsedProject, Pipeline},
    CompileOptions,
};

mod debug_paths;
mod external_files;
mod header_generator;
mod multi_files;

#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct ParsedProjectWrapper {
    pub parsed_project: ParsedProject,
    pub context: GlobalContext,
}

#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct IndexedProjectWrapper {
    pub indexed_project: IndexedProject,
    pub context: GlobalContext,
}

#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct AnnotatedProjectWrapper {
    pub annotated_project: AnnotatedProject,
    pub context: GlobalContext,
}

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
    let compile_options = CompileOptions {
        root: root.map(PathBuf::from),
        debug_level,
        optimization: plc::OptimizationLevel::None,
        ..Default::default()
    };
    compile_to_string_with_options(sources, includes, compile_options)
}

pub fn compile_to_string_with_options<S, T>(
    sources: T,
    includes: T,
    compile_options: CompileOptions,
) -> Result<Vec<String>, Diagnostic>
where
    S: SourceContainer + Debug,
    T: IntoIterator<Item = S>,
{
    let mut diagnostician = Diagnostician::null_diagnostician();
    //Create a project
    let project = Project::new("TestProject".into()).with_sources(sources).with_source_includes(includes);
    let ctxt = GlobalContext::new()
        .with_source(project.get_sources(), None)?
        .with_source(project.get_includes(), None)?;
    //TODO: participants
    pipelines::ParsedProject::parse(&ctxt, &project, &mut diagnostician)?
        //Index
        .index(ctxt.provider())
        //Resolve
        .annotate(ctxt.provider())
        //Codegen
        .codegen_to_string(&compile_options)
}

pub fn compile_args_to_string(args: &[String]) -> Result<String, Diagnostic> {
    let mut pipeline = BuildPipeline::new(args).map_err(|err| Diagnostic::new(err.to_string()))?;
    pipeline.register_default_mut_participants();
    let project = pipeline.parse()?;
    let project = pipeline.index(project)?;
    let project = pipeline.annotate(project)?;
    project.validate(&pipeline.context, &mut pipeline.diagnostician)?;

    let context = CodegenContext::create();
    let module =
        project.generate_single_module(&context, pipeline.get_compile_options().as_ref().unwrap(), None)?;
    module.map(|it| it.persist_to_string()).ok_or_else(|| Diagnostic::new("Cannot generate module"))
}

pub fn compile_build_config_to_string(
    build_config: &Path,
    extra_args: &[&str],
) -> Result<String, Diagnostic> {
    let mut args = vec![
        "plc".to_string(),
        "build".to_string(),
        build_config.to_string_lossy().to_string(),
        "--ir".to_string(),
        "--single-module".to_string(),
        "-O".to_string(),
        "none".to_string(),
    ];
    args.extend(extra_args.iter().map(|it| it.to_string()));
    compile_args_to_string(&args)
}

pub fn progress_pipeline_to_step_parsed<S, T>(
    sources: T,
    includes: T,
) -> Result<ParsedProjectWrapper, Diagnostic>
where
    S: SourceContainer + Debug,
    T: IntoIterator<Item = S>,
{
    let project = construct_project_from_sources_and_includes(sources, includes);
    let context = GlobalContext::new()
        .with_source(project.get_sources(), None)
        .expect("Failed to generate global context with sources!")
        .with_source(project.get_includes(), None)
        .expect("Failed to generate global context with includes!");
    let mut pipeline = get_pipeline(project, context);

    let parsed_project = pipeline.parse()?;

    Ok(ParsedProjectWrapper { parsed_project, context: pipeline.context })
}

pub fn progress_pipeline_to_step_indexed<S, T>(
    sources: T,
    includes: T,
    parsed_project_wrapper: ParsedProjectWrapper,
) -> Result<IndexedProjectWrapper, Diagnostic>
where
    S: SourceContainer + Debug,
    T: IntoIterator<Item = S>,
{
    let project = construct_project_from_sources_and_includes(sources, includes);
    let mut pipeline = get_pipeline(project, parsed_project_wrapper.context);

    let indexed_project = pipeline.index(parsed_project_wrapper.parsed_project)?;

    Ok(IndexedProjectWrapper { indexed_project, context: pipeline.context })
}

pub fn progress_pipeline_to_step_annotated<S, T>(
    sources: T,
    includes: T,
    indexed_project_wrapper: IndexedProjectWrapper,
) -> Result<AnnotatedProjectWrapper, Diagnostic>
where
    S: SourceContainer + Debug,
    T: IntoIterator<Item = S>,
{
    let project = construct_project_from_sources_and_includes(sources, includes);
    let mut pipeline = get_pipeline(project, indexed_project_wrapper.context);

    let annotated_project = pipeline.annotate(indexed_project_wrapper.indexed_project)?;

    Ok(AnnotatedProjectWrapper { annotated_project, context: pipeline.context })
}

fn construct_project_from_sources_and_includes<S, T>(sources: T, includes: T) -> Project<S>
where
    S: SourceContainer + Debug,
    T: IntoIterator<Item = S>,
{
    // Create a project
    Project::new("TestProject".into()).with_sources(sources).with_source_includes(includes)
}

fn get_pipeline<S>(project: Project<S>, context: GlobalContext) -> BuildPipeline<S>
where
    S: SourceContainer + Debug,
{
    let diagnostician = Diagnostician::null_diagnostician();
    let mut pipeline = BuildPipeline {
        context,
        project,
        diagnostician,
        compile_parameters: None,
        linker: plc::linker::LinkerType::Internal,
        mutable_participants: Default::default(),
        participants: Default::default(),
        module_name: Some("<internal>".to_string()),
    };

    pipeline.register_default_mut_participants();

    pipeline
}
