use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex, RwLock},
};

use crate::pipelines::{
        participant::{InitParticipant, PipelineParticipant},
        BuildPipeline, Pipeline,
    };

use plc::{
    codegen::{CodegenContext, GeneratedModule},
    lowering::calls::AggregateTypeLowerer,
};
use plc_diagnostics::diagnostician::Diagnostician;
use plc_index::GlobalContext;
use project::project::Project;
use source_code::{Compilable, SourceContainer};

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

struct CodegenTestRunner<T> {
    params: Option<T>
}

impl<T : Sync + Send> PipelineParticipant for CodegenTestRunner<T>
{
    fn generate(
        &self,
        _generated_module: &GeneratedModule,
    ) -> Result<(), plc_diagnostics::diagnostics::Diagnostic> {
        Ok(())
    }
}

///
/// Compiles and runs the given sources
/// Sources must be `Compilable`, default implementations include `String` and `&str`
/// An implementation is also provided for `Vec<SourceContainer>`
///
pub fn compile_with_participant<T, S>(context: &CodegenContext, source: T, participant : Option<Box<dyn PipelineParticipant + 'a>>)
where
    T: Compilable<T = S>,
    S: SourceContainer,
{
    let project = Project::new("TestProject".to_string()).with_sources(source.containers());
    let ctxt = GlobalContext::new().with_source(project.get_sources(), None).unwrap();
    let init_participant = InitParticipant::new(&project.get_init_symbol_name(), ctxt.provider());
    let aggregate_type_lowerer = AggregateTypeLowerer::new(ctxt.provider());
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
    pipeline.register_mut_participant(Box::new(init_participant));
    if let Some(participant) = participant {
        pipeline.register_participant(participant);
    }
    pipeline.register_mut_participant(Box::new(aggregate_type_lowerer));
    let project = pipeline.parse().unwrap();
    let project = pipeline.index(project).unwrap();
    let project = pipeline.annotate(project).unwrap();
    pipeline.generate(context, project).unwrap();

    // match module {
    //     Ok(res) => res.unwrap(),
    //     Err(e) => panic!("{e}"),
    // }
}
pub fn compile<T, S>(context: &CodegenContext, source: T)
where
    T: Compilable<T = S>,
    S: SourceContainer,
{
    compile_with_participant(context, source, None)
}

///
/// A Convenience method to compile and then run the given source
///
pub fn compile_and_run<'a, T : Send + Sync, U: Default + Sync + Send, S: Compilable<T = R>, R: SourceContainer>(
    source: S,
    params: &'a mut T,
) -> U {
    let context: CodegenContext = CodegenContext::create();
    let value: Arc<Mutex<U>> = Default::default();
    // let compile_value = value.clone();
    let participant = CodegenTestRunner { params: Some(params) };
    compile_with_participant(&context, source, Some(Box::new(participant)));
        // move |m| {
        // *compile_value.lock().unwrap() = m.run::<T, U>("main", params);
    // });
    let mut lock = value.lock().unwrap();
    std::mem::take(lock.borrow_mut())
}

///
/// A Convenience method to compile and then run the given source
/// without external parameters
///
pub fn compile_and_run_no_params<U: Default + Send + Sync, S: Compilable<T = R>, R: SourceContainer>(source: S) -> U {
    let context: CodegenContext = CodegenContext::create();
    let value: Arc<Mutex<U>> = Default::default();
    let _compile_value = value.clone();
    let participant = CodegenTestRunner { params: None };
    compile_with_participant(&context, source, Some(Box::new(participant)));
    // compile(&context, source, move |m| {
    //     m.print_to_stderr();
    //     *compile_value.lock().unwrap() = m.run_no_param("main");
    // });

    let mut lock = value.lock().unwrap();
    std::mem::take(lock.borrow_mut())
}
