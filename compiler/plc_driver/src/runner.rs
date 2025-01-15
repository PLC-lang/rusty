use crate::{
    pipelines::{participant::InitParticipant, BuildPipeline, Pipeline},
    CompileOptions,
};

use plc::{
    codegen::{CodegenContext, GeneratedModule},
    lowering::calls::AggregateTypeLowerer,
};
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
pub fn compile<T: Compilable>(codegen_context: &CodegenContext, source: T) -> GeneratedModule<'_> {
    let source = source.containers();
    let project = Project::new("TestProject".to_string()).with_sources(source);
    let context = GlobalContext::new().with_source(project.get_sources(), None).unwrap();
    let diagnostician = Diagnostician::null_diagnostician();
    let mut pipeline = BuildPipeline {
        context,
        project,
        diagnostician,
        compile_parameters: None,
        linker: plc::linker::LinkerType::Internal,
        mutable_participants: Default::default(),
        participants: Default::default(),
    };

    let init_participant =
        InitParticipant::new(&pipeline.project.get_init_symbol_name(), pipeline.context.provider());
    pipeline.register_mut_participant(Box::new(init_participant));

    let aggregate_return_participant = AggregateTypeLowerer::new(pipeline.context.provider());
    pipeline.register_mut_participant(Box::new(aggregate_return_participant));

    let project = pipeline.parse().unwrap();
    let project = pipeline.index(project).unwrap();
    let project = pipeline.annotate(project).unwrap();

    let compile_options = CompileOptions {
        optimization: plc::OptimizationLevel::None,
        debug_level: plc::DebugLevel::None,
        ..Default::default()
    };

    match project.generate_single_module(codegen_context, &compile_options) {
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
    module.print_to_stderr();
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
