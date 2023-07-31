use crate::{pipelines::ParsedProject, CompileOptions};

use ast::provider::IdProvider;
use plc::{
    codegen::{CodegenContext, GeneratedModule},
    diagnostics::Diagnostician,
};
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
    let mut diagnostician = Diagnostician::null_diagnostician();
    let id_provider = IdProvider::default();
    let parsed_project =
        ParsedProject::parse(&project, None, id_provider.clone(), &mut diagnostician).unwrap();
    let indexed_project = parsed_project.index(id_provider.clone()).unwrap();
    let annotated_project = indexed_project.annotate(id_provider, &diagnostician).unwrap();
    let compile_options = CompileOptions {
        optimization: plc::OptimizationLevel::None,
        debug_level: plc::DebugLevel::None,
        ..Default::default()
    };

    annotated_project.generate_single_module(context, &compile_options).unwrap().unwrap()
}

///
/// A Convenience method to compile and then run the given source
///
pub fn compile_and_run<T, U, S: Compilable>(source: S, params: &mut T) -> U {
    let context: CodegenContext = CodegenContext::create();
    let module = compile(&context, source);
    module.run::<T, U>("main", params)
}
