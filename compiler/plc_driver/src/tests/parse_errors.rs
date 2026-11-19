use plc::{codegen::CodegenContext, DebugLevel};
use plc_diagnostics::diagnostician::Diagnostician;
use source_code::SourceCode;

use crate::{
    parse_and_annotate, parse_and_validate,
    pipelines::BuildPipeline,
    runner,
    tests::{compile_to_string, progress_pipeline_to_step_parsed},
};

const BROKEN_BODY: &str = "
        FUNCTION main : DINT
            main := 2 *;
        END_FUNCTION
        ";

#[test]
fn parse_errors_are_reported_once() {
    let source = SourceCode::from(
        "
        FUNCTION foo : BOOL
        VAR_INPUT
            in : BOOL;
            in2
        END_VAR
        ;
        END_FUNCTION
        ",
    );

    let diagnostics = parse_and_validate("TestProject", vec![source]);

    insta::assert_snapshot!(diagnostics, @r"
    error[E006]: Missing expected Token KeywordColon or KeywordComma
      ┌─ <internal>:5:16
      │  
    5 │               in2
      │ ╭───────────────^
    6 │ │         END_VAR
      │ ╰────────^ Missing expected Token KeywordColon or KeywordComma

    error[E006]: Missing expected Token KeywordColon
      ┌─ <internal>:6:9
      │
    6 │         END_VAR
      │         ^^^^^^^ Missing expected Token KeywordColon

    error[E007]: Unexpected token: expected DataTypeDefinition but found KeywordEndVar
      ┌─ <internal>:6:9
      │
    6 │         END_VAR
      │         ^^^^^^^ Unexpected token: expected DataTypeDefinition but found KeywordEndVar

    error[E006]: Missing expected Token [KeywordSemicolon]
      ┌─ <internal>:6:9
      │
    6 │         END_VAR
      │         ^^^^^^^ Missing expected Token [KeywordSemicolon]

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'END_VAR'
      ┌─ <internal>:6:9
      │
    6 │         END_VAR
      │         ^^^^^^^ Unexpected token: expected KeywordSemicolon but found 'END_VAR'
    ");
}

#[test]
#[should_panic(expected = "Unexpected token: expected expression but found ;")]
fn runner_compile_panics_with_the_parse_errors() {
    let context = CodegenContext::create();
    runner::compile(&context, BROKEN_BODY);
}

#[test]
fn compile_to_string_fails_with_the_parse_errors() {
    let source = SourceCode::from(BROKEN_BODY);

    let error = compile_to_string(vec![source], vec![], None, DebugLevel::None).unwrap_err();

    insta::assert_snapshot!(error.to_string(), @r"
    Compilation aborted due to critical parse errors
    error[E007]: Unexpected token: expected expression but found ;
      ┌─ <internal>:3:24
      │
    3 │             main := 2 *;
      │                        ^ Unexpected token: expected expression but found ;
    ");
}

#[test]
fn parsed_pipeline_step_fails_with_the_parse_errors() {
    let source = SourceCode::from(BROKEN_BODY);

    let error = progress_pipeline_to_step_parsed(vec![source], vec![]).err().expect("parsing must fail");

    insta::assert_snapshot!(error.to_string(), @r"
    Compilation aborted due to critical parse errors
    error[E007]: Unexpected token: expected expression but found ;
      ┌─ <internal>:3:24
      │
    3 │             main := 2 *;
      │                        ^ Unexpected token: expected expression but found ;
    ");
}

#[test]
fn parse_and_annotate_fails_with_the_parse_errors() {
    let source = SourceCode::from(BROKEN_BODY);

    let error = parse_and_annotate("TestProject", vec![source]).err().expect("parsing must fail");

    insta::assert_snapshot!(error.to_string(), @r"
    Compilation aborted due to critical parse errors
    error[E007]: Unexpected token: expected expression but found ;
      ┌─ <internal>:3:24
      │
    3 │             main := 2 *;
      │                        ^ Unexpected token: expected expression but found ;
    ");
}

#[test]
fn pipeline_parse_and_annotate_fails_with_the_parse_errors() {
    let source = SourceCode::from(BROKEN_BODY);
    let mut pipeline =
        BuildPipeline::from_sources("TestProject", vec![source], Diagnostician::buffered()).unwrap();
    pipeline.register_default_mut_participants();

    let error = pipeline.parse_and_annotate().err().expect("parsing must fail");

    insta::assert_snapshot!(error.to_string(), @r"
    Compilation aborted due to critical parse errors
    error[E007]: Unexpected token: expected expression but found ;
      ┌─ <internal>:3:24
      │
    3 │             main := 2 *;
      │                        ^ Unexpected token: expected expression but found ;
    ");
}
