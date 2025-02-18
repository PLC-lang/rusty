use plc::DebugLevel;
use plc_source::SourceCode;

pub fn codegen(src: &str) -> String {
    codegen_without_unwrap(src).map_err(|it| panic!("{it}")).unwrap()
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
