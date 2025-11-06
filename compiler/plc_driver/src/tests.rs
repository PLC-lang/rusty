use std::{fmt::Debug, path::PathBuf};

use plc::DebugLevel;
use plc_diagnostics::{diagnostician::Diagnostician, diagnostics::Diagnostic};
use plc_index::GlobalContext;
use project::project::Project;
use source_code::SourceContainer;

use crate::{
    pipelines::{self, AnnotatedProject, IndexedProject, ParsedProject},
    CompileOptions,
};

mod external_files;
mod header_generator;
mod multi_files;

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

pub fn progress_pipeline_to_step_parsed<S, T>(sources: T, includes: T) -> Result<ParsedProject, Diagnostic>
where
    S: SourceContainer + Debug,
    T: IntoIterator<Item = S>,
{
    let mut diagnostician = Diagnostician::null_diagnostician();

    let (project, context) = construct_project_and_context_from_sources_and_includes(sources, includes);
    let parsed_project = pipelines::ParsedProject::parse(&context, &project, &mut diagnostician)?;

    Ok(parsed_project)
}

pub fn progress_pipeline_to_step_indexed<S, T>(
    sources: T,
    includes: T,
    parsed_project: ParsedProject,
) -> Result<IndexedProject, Diagnostic>
where
    S: SourceContainer + Debug,
    T: IntoIterator<Item = S>,
{
    let (project, context) = construct_project_and_context_from_sources_and_includes(sources, includes);

    let indexed_project = parsed_project
        .index(context.provider())
        .extend_with_init_units(project.get_init_symbol_name(), context.provider());

    Ok(indexed_project)
}

pub fn progress_pipeline_to_step_annotated<S, T>(
    sources: T,
    includes: T,
    indexed_project: IndexedProject,
) -> Result<AnnotatedProject, Diagnostic>
where
    S: SourceContainer + Debug,
    T: IntoIterator<Item = S>,
{
    let (_, context) = construct_project_and_context_from_sources_and_includes(sources, includes);

    let annotated_project = indexed_project.annotate(context.provider());

    Ok(annotated_project)
}

fn construct_project_and_context_from_sources_and_includes<S, T>(
    sources: T,
    includes: T,
) -> (Project<S>, GlobalContext)
where
    S: SourceContainer + Debug,
    T: IntoIterator<Item = S>,
{
    // Create a project
    let project = Project::new("TestProject".into()).with_sources(sources).with_source_includes(includes);
    let context = GlobalContext::new()
        .with_source(project.get_sources(), None)
        .expect("Failed to generate global context with sources!")
        .with_source(project.get_includes(), None)
        .expect("Failed to generate global context with includes!");

    (project, context)
}
