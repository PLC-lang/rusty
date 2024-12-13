use std::sync::{Arc, RwLock};

use crate::{
    pipelines::{participant::{CodegenParticipant, InitParticipant}, BuildPipeline, GeneratedProject, ParsedProject},
    CompileOptions,
};

use plc::codegen::{CodegenContext, GeneratedModule};
use plc_diagnostics::diagnostician::Diagnostician;
use plc_index::GlobalContext;
use project::project::Project;
use source_code::Compilable;

#[allow(dead_code)]
#[repr(C)]
pub struct MainType {
    a: [usize; 1000],
}

impl Default for MainType {
    fn default() -> Self {
        MainType { a: [0; 1000] }
    }
}

///
/// Compiles and runs the given sources
/// Sources must be `Compilable`, default implementations include `String` and `&str`
/// An implementation is also provided for `Vec<SourceContainer>`
///
pub fn compile<T: Compilable>(context: &CodegenContext, source: T) -> GeneratedModule<'_> {
    let project = Project::new("TestProject".to_string()).with_sources(source.containers());
    let ctxt = GlobalContext::new().with_source(project.get_sources(), None).unwrap();
    let diagnostician = Diagnostician::default();
    let params = crate::cli::CompileParameters::parse(&["--single-module", "-O", "none"]).unwrap();
    let mut pipeline = BuildPipeline {
        context: ctxt,
        project,
        diagnostician,
        compile_parameters: Some(params),
        linker: plc::linker::LinkerType::Internal,
        mutable_participants: Vec::default(),
        participants: Vec::default(),
    };
    let target = pipeline.compile_parameters.as_ref().and_then(|it| it.target.clone()).unwrap_or_default();
    let codegen_participant = CodegenParticipant {
        compile_options: pipeline.get_compile_options().unwrap(),
        link_options: pipeline.get_link_options().unwrap(),
        target: target.clone(),
        objects: Arc::new(RwLock::new(GeneratedProject {
            target,
            objects: pipeline.project.get_objects().to_vec(),
        })),
        got_layout: Default::default(),
        compile_dirs: Default::default(),
        libraries: vec![],
    };
    let init_participant = Box::new(InitParticipant::new(&project.get_init_symbol_name(), ctxt.provider()));
    pipeline.register_participant(Box::new(codegen_participant));
    pipeline.register_par
    let project = crate::pipelines::Pipeline::parse(&mut pipeline).unwrap();
    let project = crate::pipelines::Pipeline::index(&mut pipeline, project).unwrap();
    let project = crate::pipelines::Pipeline::annotate(&mut pipeline, project).unwrap();

    let module = project.generate_single_module(&context, pipeline.get_compile_options().as_ref().unwrap());

    match module {
        Ok(res) => res.unwrap(),
        Err(e) => panic!("{e}"),
    }
}

///
/// A Convenience method to compile and then run the given source
///
pub fn compile_and_run<T, U, S: Compilable>(source: S, params: &mut T) -> U {
    let context: CodegenContext = CodegenContext::create();
    let module = compile(&context, source);
    module.run::<T, U>("main", params)
}

///
/// A Convenience method to compile and then run the given source
/// without external parameters
///
pub fn compile_and_run_no_params<U, S: Compilable>(source: S) -> U {
    let context: CodegenContext = CodegenContext::create();
    let module = compile(&context, source);
    module.print_to_stderr();
    module.run_no_param::<U>("main")
}
