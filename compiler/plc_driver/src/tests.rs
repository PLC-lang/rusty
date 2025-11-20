use std::{fmt::Debug, path::PathBuf};

use plc::DebugLevel;
use plc_diagnostics::{diagnostician::Diagnostician, diagnostics::Diagnostic};
use plc_index::GlobalContext;
use project::project::Project;
use serde::{Deserialize, Serialize};
use source_code::SourceContainer;

use crate::{
    pipelines::{self, AnnotatedProject, BuildPipeline, IndexedProject, ParsedProject, Pipeline},
    CompileOptions,
};

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
        .extend_with_init_units(project.get_init_symbol_name(), ctxt.provider())
        //Resolve
        .annotate(ctxt.provider())
        //Codegen
        .codegen_to_string(&compile_options)
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

    pipeline.register_default_participants();

    pipeline
}
