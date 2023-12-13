use crate::{pipelines::ParsedProject, CompileOptions};

use ast::provider::IdProvider;
use plc::codegen::{CodegenContext, GeneratedModule};
use plc_diagnostics::diagnostician::Diagnostician;
use project::project::{LibraryInformation, Project};
use source_code::{Compilable, SourceContainer, SourceMap};

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

    let sm = SourceMap::leaking();
    for source in project.get_sources() {
        let file_path = source.get_location_str();
        let file_content = source.load_source(None).unwrap();
        sm.sources.insert(file_path.to_string(), file_content);
    }

    for source in project.get_includes() {
        let file_path = source.get_location_str();
        let file_content = source.load_source(None).unwrap();
        sm.sources.insert(file_path.to_string(), file_content);
    }

    for source in project.get_libraries().iter().flat_map(LibraryInformation::get_includes) {
        let file_path = source.get_location_str();
        let file_content = source.load_source(None).unwrap();
        sm.sources.insert(file_path.to_string(), file_content);
    }

    let parsed_project = ParsedProject::parse(&project, sm, id_provider.clone(), &mut diagnostician).unwrap();
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
