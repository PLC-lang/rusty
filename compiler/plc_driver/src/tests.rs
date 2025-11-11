use std::{fmt::Debug, path::PathBuf};

use plc::{
    lowering::{
        calls::AggregateTypeLowerer, polymorphism::PolymorphicCallLowerer, property::PropertyLowerer,
        vtable::VirtualTableGenerator,
    },
    DebugLevel,
};
use plc_diagnostics::{diagnostician::Diagnostician, diagnostics::Diagnostic};
use plc_index::GlobalContext;
use plc_lowering::inheritance::InheritanceLowerer;
use project::project::Project;
use serde::{Deserialize, Serialize};
use source_code::SourceContainer;

use crate::{
    pipelines::{
        self,
        participant::{InitParticipant, PipelineParticipantMut},
        AnnotatedProject, IndexedProject, ParsedProject,
    },
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
    let mut diagnostician = Diagnostician::null_diagnostician();
    let project = construct_project_from_sources_and_includes(sources, includes);
    let context = GlobalContext::new()
        .with_source(project.get_sources(), None)
        .expect("Failed to generate global context with sources!")
        .with_source(project.get_includes(), None)
        .expect("Failed to generate global context with includes!");

    let parsed_project = pipelines::ParsedProject::parse(&context, &project, &mut diagnostician)?;

    Ok(ParsedProjectWrapper { parsed_project, context })
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
    let indexed_project =
        index(parsed_project_wrapper.parsed_project, project, &parsed_project_wrapper.context)?;

    Ok(IndexedProjectWrapper { indexed_project, context: parsed_project_wrapper.context })
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
    let annotated_project =
        annotate(indexed_project_wrapper.indexed_project, project, &indexed_project_wrapper.context)?;

    Ok(AnnotatedProjectWrapper { annotated_project, context: indexed_project_wrapper.context })
}

fn construct_project_from_sources_and_includes<S, T>(sources: T, includes: T) -> Project<S>
where
    S: SourceContainer + Debug,
    T: IntoIterator<Item = S>,
{
    // Create a project
    Project::new("TestProject".into()).with_sources(sources).with_source_includes(includes)
}

// ---------- //
// -- TODO -- //
// ---------- //
// These should probably be accessed via the pipeline, for now I am replicating the behaviour from outside of it.
// Solution found, see runner.rs ...

fn index<S>(
    project: ParsedProject,
    project_info: Project<S>,
    context: &GlobalContext,
) -> Result<IndexedProject, Diagnostic>
where
    S: SourceContainer + Debug,
{
    let mut mutable_participants = get_mutable_participants(project_info, context);
    let project = mutable_participants.iter_mut().fold(project, |project, p| p.pre_index(project));
    let indexed_project = project.index(context.provider());
    let project = mutable_participants.iter_mut().fold(indexed_project, |project, p| p.post_index(project));

    Ok(project)
}

fn annotate<S>(
    project: IndexedProject,
    project_info: Project<S>,
    context: &GlobalContext,
) -> Result<AnnotatedProject, Diagnostic>
where
    S: SourceContainer + Debug,
{
    let mut mutable_participants = get_mutable_participants(project_info, context);
    let project = mutable_participants.iter_mut().fold(project, |project, p| p.pre_annotate(project));
    let annotated_project = project.annotate(context.provider());
    let annotated_project =
        mutable_participants.iter_mut().fold(annotated_project, |project, p| p.post_annotate(project));

    Ok(annotated_project)
}

fn get_mutable_participants<S>(
    project: Project<S>,
    context: &GlobalContext,
) -> Vec<Box<dyn PipelineParticipantMut>>
where
    S: SourceContainer + Debug,
{
    let mut_participants: Vec<Box<dyn PipelineParticipantMut>> = vec![
        Box::new(VirtualTableGenerator::new(context.provider())),
        Box::new(PolymorphicCallLowerer::new(context.provider())),
        Box::new(PropertyLowerer::new(context.provider())),
        Box::new(InitParticipant::new(project.get_init_symbol_name(), context.provider())),
        Box::new(AggregateTypeLowerer::new(context.provider())),
        Box::new(InheritanceLowerer::new(context.provider())),
    ];

    mut_participants
}
