// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::test_utils::tests::{parse_and_validate_buffered, parse_buffered};
use insta::{assert_debug_snapshot, assert_snapshot};

/*
 * These tests deal with parsing-behavior of containers like POUs, VAR-containers and Actions
 * in the presence of errors.
 * following scenarios will be tested:
 *  - missing pou names, missing function's return variable, illegal return-variable declaration
 *  - incomplete variable-declarations
 *  - incomplete containers, illegal close-keywords
 */

#[test]
fn missing_pou_name() {
    let src = r"
            PROGRAM
            VAR END_VAR
            a;
            END_PROGRAM
            ";

    let (compilation_unit, diagnostics) = parse_buffered(src);
    //expected end of statement (e.g. ;), but found KeywordEndProgram at line: 1 offset: 14..25"
    //Expecting a missing semicolon message
    assert_snapshot!(diagnostics);

    let pou = &compilation_unit.implementations[0];
    assert_debug_snapshot!(pou.statements);
}

#[test]
fn missing_pou_name_2() {
    // in this case, a becomes the POU's name
    let src = r"
            PROGRAM
            a := 2;
            x;
            END_PROGRAM
            ";

    let (compilation_unit, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics);

    let pou = &compilation_unit.implementations[0];
    assert_debug_snapshot!(pou.statements);
}

#[test]
fn illegal_end_pou_keyword() {
    let src = r"
            PROGRAM foo
            a;
            END_FUNCTION
            PROGRAM baz
            b;
            END_PROGRAM
            ";

    let (compilation_unit, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics);

    //check if baz was parsed successfully
    let pou = &compilation_unit.implementations[1];
    assert_debug_snapshot!(pou.statements);
}

#[test]
#[ignore = "https://github.com/PLC-lang/rusty/issues/491"]
fn function_without_return_variable_declaration() {
    // GIVEN a function without a return type
    let src = r"
        FUNCTION foo
        a;
        END_FUNCTION
        ";

    // WHEN the function is parsed
    let (compilation_unit, diagnostics) = parse_buffered(src);

    // THEN I expect a diagnostic complaining about a missing return type
    assert_snapshot!(diagnostics);

    // AND I expect the body to be parsed successfully
    let pou = &compilation_unit.implementations[0];
    assert_debug_snapshot!(pou.statements);
}

#[test]
fn function_with_illegal_return_variable_declaration() {
    let src = r"
            FUNCTION foo :
            VAR END_VAR
            a;
            END_FUNCTION
            ";

    let (compilation_unit, diagnostics) = parse_buffered(src);
    //expected end of statement (e.g. ;), but found KeywordEndProgram at line: 1 offset: 14..25"
    //Expecting a missing semicolon message
    assert_snapshot!(diagnostics);

    //check if a was parsed successfully
    let pou = &compilation_unit.implementations[0];
    assert_debug_snapshot!(pou.statements);
}

#[test]
fn function_return_type_with_initializer() {
    let src = r"
            FUNCTION foo : INT := 3
            VAR END_VAR
            a;
            END_FUNCTION
            ";

    let (compilation_unit, diagnostics) = parse_buffered(src);
    //expected end of statement (e.g. ;), but found KeywordEndProgram at line: 1 offset: 14..25"
    //Expecting a missing semicolon message
    assert_snapshot!(diagnostics);

    //check if a was parsed successfully
    let pou = &compilation_unit.implementations[0];
    assert_debug_snapshot!(pou.statements);
}

#[test]
fn unclosed_var_container() {
    let src = r"
                PROGRAM foo
                    VAR a : INT;
                    VAR b : INT; END_VAR
                END_PROGRAM
                ";

    let (compilation_unit, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics);

    //check if b was parsed successfully
    let var_block = &compilation_unit.units[0].variable_blocks[0];
    assert_debug_snapshot!(var_block)
}

#[test]
fn test_unexpected_type_declaration_error_message() {
    let src = "TYPE MyType:
                PROGRAM
                END_PROGRAM
            END_TYPE
    ";
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics)
}

#[test]
fn a_program_needs_to_end_with_end_program() {
    let src = "PROGRAM buz ";
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics)
}

#[test]
fn a_variable_declaration_block_needs_to_end_with_endvar() {
    let src = "PROGRAM buz VAR END_PROGRAM ";
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics)
}
