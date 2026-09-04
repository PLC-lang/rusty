use plc_ast::{ast::LinkageType, provider::IdProvider};
use plc_source::{source_location::SourceLocationFactory, SourceCode, SourceContainer};
use plc_xmlgen::xml_gen::{
    get_omron_template, parse_project_into_nodetree, GenerationParameters, OMRON_SCHEMA,
};

use crate::{lexer, parser};

#[test]
fn external_var_blocks_from_source_text_are_generated() {
    let src = r"
PROGRAM ExternalProg
    VAR_EXTERNAL
        currentState : INT;
        reset : BOOL;
    END_VAR

    VAR_EXTERNAL CONSTANT
        MAX_SPEED : INT;
    END_VAR

    reset := FALSE;
END_PROGRAM
";

    let temp_dir = std::env::temp_dir();
    let st_path = temp_dir.join("test_external_blocks_source.st");
    std::fs::write(&st_path, src).unwrap();

    let source = SourceCode { source: String::from(src), path: Some(st_path.clone()) };
    let file_name = source.get_location_str();

    let (unit, diagnostics) = parser::parse(
        lexer::lex_with_ids(
            &source.source,
            IdProvider::default(),
            SourceLocationFactory::for_source(&source),
        ),
        LinkageType::Internal,
        file_name,
    );
    assert!(diagnostics.is_empty(), "unexpected parse diagnostics: {diagnostics:?}");

    let mut params = GenerationParameters::new();
    params.output_xml_omron = true;

    let output_path = temp_dir.join("test_external_blocks_output.xml");
    let units = vec![&unit];
    let result =
        parse_project_into_nodetree(&params, &units, OMRON_SCHEMA, &output_path, get_omron_template());
    assert!(result.is_ok());

    let contents = std::fs::read_to_string(&output_path).unwrap();
    let compact = contents.lines().map(|line| line.trim()).collect::<Vec<&str>>().join("");

    assert!(compact.contains(r#"<Program name="ExternalProg">"#));
    assert!(compact.contains(r#"<ExternalVars><Variable name="currentState">"#));
    assert!(compact.contains(r#"<Variable name="reset">"#));
    assert!(compact.contains(r#"<ExternalVars constant="true"><Variable name="MAX_SPEED">"#));
    assert!(compact.contains("reset := FALSE;"));

    let _ = std::fs::remove_file(&output_path);
    let _ = std::fs::remove_file(&st_path);
}
