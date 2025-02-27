use plc::DebugLevel;
use plc_source::SourceCode;

pub fn codegen(src: &str) -> String {
    codegen_without_unwrap(src).map_err(|it| panic!("{it}")).unwrap()
}

pub fn codegen_with_debug(src: &str) -> String {
    codegen_debug_without_unwrap(src, DebugLevel::Full(5)).map_err(|it| panic!("{it}")).unwrap()
}

pub fn codegen_without_unwrap(src: &str) -> Result<String, String> {
    codegen_debug_without_unwrap(src, DebugLevel::None)
}

pub fn codegen_debug_without_unwrap(src: &str, debug_level: DebugLevel) -> Result<String, String> {
    let src: SourceCode = src.into();
    match debug_level {
        DebugLevel::None => driver::generate_to_string("Test", vec![src]),
        DebugLevel::VariablesOnly(_) | DebugLevel::Full(_) => {
            driver::generate_to_string_debug("Test", vec![src])
        }
    }
    .map_err(|it| it.to_string())
}

pub fn parse_and_validate_buffered(src: &str) -> String {
    let source: SourceCode = src.into();
    driver::parse_and_validate("TestProject", vec![source])
}

// Disabled because it does not generate the correct LLVM IR if debug is enabled
// pub fn codegen_debug_without_unwrap(src: &str, debug_level: DebugLevel) -> Result<String, String> {
//     //Create a temp file to store the result
//     let mut output = NamedTempFile::new().map_err(|it| it.to_string())?;
//     let src: SourceCode = src.into();
//     let project = Project::new("Test".to_string())
//         .with_sources(vec![src])
//         .with_output_name(output.path().to_str().map(ToString::to_string));
//     let context =
//         GlobalContext::new().with_source(project.get_sources(), None).map_err(|it| it.to_string())?;
//     let diagnostician = Diagnostician::default();
//     let mut params = cli::CompileParameters::parse(&[
//         "plc",
//         "--ir",
//         "--single-module",
//         "-O",
//         "none",
//         "-o",
//         output.path().to_str().unwrap(),
//     ])
//     .map_err(|e| e.to_string())?;
//     params.generate_debug = debug_level != DebugLevel::None;
//     let pipeline = BuildPipeline {
//         context,
//         project,
//         diagnostician,
//         compile_parameters: Some(params),
//         linker: LinkerType::Internal,
//         mutable_participants: Vec::default(),
//         participants: Vec::default(),
//         module_name: Some("<internal>".to_string()),
//     };
//
//     driver::compile_with_pipeline(pipeline).map_err(|it| it.to_string())?;
//
//     let mut res = String::new();
//     output.read_to_string(&mut res).map_err(|it| it.to_string())?;
//     Ok(res)
// }
