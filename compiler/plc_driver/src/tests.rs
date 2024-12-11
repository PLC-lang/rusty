use std::{fmt::Debug, path::PathBuf, sync::Arc};

use plc::{codegen::CodegenContext, DebugLevel};
use plc_diagnostics::{diagnostician::Diagnostician, diagnostics::Diagnostic};
use plc_index::GlobalContext;
use project::project::Project;
use source_code::SourceContainer;

use crate::{
    pipelines::{self, BuildPipeline},
    CompileOptions,
};

mod external_files;
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
    let diagnostician = Diagnostician::null_diagnostician();
    //Create a project
    let project = Project::new("TestProject".into()).with_sources(sources).with_source_includes(includes);
    let project = if let Some(path) = root { project.with_location(path) } else { project };
    let ctxt = GlobalContext::new()
        .with_source(project.get_sources(), None)?
        .with_source(project.get_includes(), None)?;

    // pipelines::ParsedProject::parse(&ctxt, &project, &mut diagnostician)?
    //     //Index
    //     .index(ctxt.provider())
    //     // .extend_with_init_units(&project.get_init_symbol_name(), ctxt.provider())
    //     //Resolve
    //     .annotate(ctxt.provider())
    //     //Codegen
    //     .codegen_to_string(&compile_options)

    let debug_level = match debug_level {
        DebugLevel::None => None,
        DebugLevel::VariablesOnly(_) => Some("--debug_variables"),
        DebugLevel::Full(_) => Some("-g"),
    };
    let mut args = vec!["-O", "none"];
    args.extend(debug_level);
    let mut pipeline = BuildPipeline::new(&args).unwrap();
    // let params = crate::cli::CompileParameters::parse(&args).unwrap();

    let target = pipeline.compile_parameters.as_ref().and_then(|it| it.target.clone()).unwrap_or_default();
    let codegen_participant = pipelines::participant::CodegenParticipant {
        compile_options: pipeline.get_compile_options().unwrap(),
        link_options: pipeline.get_link_options().unwrap(),
        target: target.clone(),
        objects: Arc::new(std::sync::RwLock::new(pipelines::GeneratedProject {
            target,
            objects: pipeline.project.get_objects().to_vec(),
        })),
        got_layout: Default::default(),
        compile_dirs: Default::default(),
        libraries: pipeline.project.get_libraries().to_vec(),
    };
    let init_participant =
        pipelines::participant::InitParticipant::new(&project.get_init_symbol_name(), ctxt.provider());

    pipeline.register_participant(Box::new(codegen_participant));

    pipeline.register_mut_participant(Box::new(init_participant));
    // let mut pipeline = crate::pipelines::BuildPipeline {
    //     context: ctxt,
    //     project,
    //     diagnostician,
    //     compile_parameters: Some(params),
    //     linker: plc::linker::LinkerType::Internal,
    //     mutable_participants: vec![init_participant],
    //     participants: Vec::default(),
    // };
    let project = crate::pipelines::Pipeline::parse(&mut pipeline).unwrap();
    let project = crate::pipelines::Pipeline::index(&mut pipeline, project).unwrap();
    let project = crate::pipelines::Pipeline::annotate(&mut pipeline, project).unwrap();

    let context = CodegenContext::create();
    project
        .generate_modules(&context, pipeline.get_compile_options().as_ref().unwrap())
        .map(|it| it.into_iter().map(|it| it.persist_to_string()).collect::<Vec<_>>())
}
