use crate::{pipelines::ParsedProject, CompileOptions};

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
    let source = source.containers();
    let project = Project::new("TestProject".to_string()).with_sources(source);
    let ctxt = GlobalContext::new().with_source(project.get_sources(), None).unwrap();
    let mut diagnostician = Diagnostician::null_diagnostician();
    let parsed_project = ParsedProject::parse(&ctxt, project, &mut diagnostician).unwrap();
    let indexed_project = parsed_project.index(ctxt.provider());
    let annotated_project = indexed_project
        .annotate(ctxt.provider())
        .lower(ctxt.provider());
    let compile_options = CompileOptions {
        optimization: plc::OptimizationLevel::None,
        debug_level: plc::DebugLevel::None,
        ..Default::default()
    };

    match annotated_project.generate_single_module(context, &compile_options) {
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
