// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::test_utils::tests::{
    parse_and_report_parse_errors_buffered, parse_and_validate_buffered, parse_buffered,
};
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
    let var_block = &compilation_unit.pous[0].variable_blocks[0];
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

#[test]
fn super_is_a_reserved_keyword() {
    let src = "
    INTERFACE super END_INTERFACE
    PROGRAM super
        VAR
            super : INT;
        END_VAR
        METHOD super END_METHOD
    END_PROGRAM
    ";

    // TODO(mhasel):    the parser produces a lot of noise for keyword errors,
    //                  we need to find a way to handle keywords as identifiers.
    //                  Also, half the diagnostics in this file are moot since
    //                  the pipeline will abort compilation on parse errors,
    //                  i.e. the validation stage is never reached.
    //                  Related: https://github.com/PLC-lang/rusty/issues/1408

    let diagnostics = parse_and_report_parse_errors_buffered(src);
    assert_snapshot!(diagnostics, @r"
    error[E006]: Expected a name for the interface definition but got nothing
      ┌─ <internal>:2:5
      │
    2 │     INTERFACE super END_INTERFACE
      │     ^^^^^^^^^ Expected a name for the interface definition but got nothing

    error[E006]: Missing expected Token KeywordEndInterface
      ┌─ <internal>:2:15
      │
    2 │     INTERFACE super END_INTERFACE
      │               ^^^^^ Missing expected Token KeywordEndInterface

    error[E007]: Unexpected token: expected StartKeyword but found super
      ┌─ <internal>:2:15
      │
    2 │     INTERFACE super END_INTERFACE
      │               ^^^^^ Unexpected token: expected StartKeyword but found super

    error[E007]: Unexpected token: expected StartKeyword but found END_INTERFACE
      ┌─ <internal>:2:21
      │
    2 │     INTERFACE super END_INTERFACE
      │                     ^^^^^^^^^^^^^ Unexpected token: expected StartKeyword but found END_INTERFACE

    error[E007]: Unexpected token: expected Identifier but found super
      ┌─ <internal>:3:13
      │
    3 │     PROGRAM super
      │             ^^^^^ Unexpected token: expected Identifier but found super

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'VAR
                super'
      ┌─ <internal>:4:9
      │  
    4 │ ╭         VAR
    5 │ │             super : INT;
      │ ╰─────────────────^ Unexpected token: expected KeywordSemicolon but found 'VAR
                super'

    error[E007]: Unexpected token: expected Literal but found END_VAR
      ┌─ <internal>:6:9
      │
    6 │         END_VAR
      │         ^^^^^^^ Unexpected token: expected Literal but found END_VAR

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'END_VAR
            METHOD super END_METHOD'
      ┌─ <internal>:6:9
      │  
    6 │ ╭         END_VAR
    7 │ │         METHOD super END_METHOD
      │ ╰───────────────────────────────^ Unexpected token: expected KeywordSemicolon but found 'END_VAR
            METHOD super END_METHOD'

    error[E006]: Missing expected Token [KeywordSemicolon, KeywordColon]
      ┌─ <internal>:8:5
      │
    8 │     END_PROGRAM
      │     ^^^^^^^^^^^ Missing expected Token [KeywordSemicolon, KeywordColon]

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'END_PROGRAM'
      ┌─ <internal>:8:5
      │
    8 │     END_PROGRAM
      │     ^^^^^^^^^^^ Unexpected token: expected KeywordSemicolon but found 'END_PROGRAM'
    ");
}

#[test]
fn this_is_a_reserved_keyword() {
    let src = "
    INTERFACE this END_INTERFACE
    PROGRAM this
        VAR
            this : INT;
        END_VAR
        METHOD this END_METHOD
    END_PROGRAM
    ";

    // TODO(mhasel):    the parser produces a lot of noise for keyword errors,
    //                  we need to find a way to handle keywords as identifiers
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics, @r"
    error[E006]: Expected a name for the interface definition but got nothing
      ┌─ <internal>:2:5
      │
    2 │     INTERFACE this END_INTERFACE
      │     ^^^^^^^^^ Expected a name for the interface definition but got nothing

    error[E006]: Missing expected Token KeywordEndInterface
      ┌─ <internal>:2:15
      │
    2 │     INTERFACE this END_INTERFACE
      │               ^^^^ Missing expected Token KeywordEndInterface

    error[E007]: Unexpected token: expected StartKeyword but found this
      ┌─ <internal>:2:15
      │
    2 │     INTERFACE this END_INTERFACE
      │               ^^^^ Unexpected token: expected StartKeyword but found this

    error[E007]: Unexpected token: expected StartKeyword but found END_INTERFACE
      ┌─ <internal>:2:20
      │
    2 │     INTERFACE this END_INTERFACE
      │                    ^^^^^^^^^^^^^ Unexpected token: expected StartKeyword but found END_INTERFACE

    error[E007]: Unexpected token: expected Identifier but found this
      ┌─ <internal>:3:13
      │
    3 │     PROGRAM this
      │             ^^^^ Unexpected token: expected Identifier but found this

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'VAR
                this'
      ┌─ <internal>:4:9
      │  
    4 │ ╭         VAR
    5 │ │             this : INT;
      │ ╰────────────────^ Unexpected token: expected KeywordSemicolon but found 'VAR
                this'

    error[E007]: Unexpected token: expected Literal but found END_VAR
      ┌─ <internal>:6:9
      │
    6 │         END_VAR
      │         ^^^^^^^ Unexpected token: expected Literal but found END_VAR

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'END_VAR
            METHOD this END_METHOD'
      ┌─ <internal>:6:9
      │  
    6 │ ╭         END_VAR
    7 │ │         METHOD this END_METHOD
      │ ╰──────────────────────────────^ Unexpected token: expected KeywordSemicolon but found 'END_VAR
            METHOD this END_METHOD'

    error[E006]: Missing expected Token [KeywordSemicolon, KeywordColon]
      ┌─ <internal>:8:5
      │
    8 │     END_PROGRAM
      │     ^^^^^^^^^^^ Missing expected Token [KeywordSemicolon, KeywordColon]

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'END_PROGRAM'
      ┌─ <internal>:8:5
      │
    8 │     END_PROGRAM
      │     ^^^^^^^^^^^ Unexpected token: expected KeywordSemicolon but found 'END_PROGRAM'

    error[E079]: Case condition used outside of case statement! Did you mean to use ';'?
      ┌─ <internal>:3:13
      │
    3 │     PROGRAM this
      │             ^^^^ Case condition used outside of case statement! Did you mean to use ';'?

    error[E120]: Invalid use of `THIS`. Usage is only allowed within `FUNCTION_BLOCK` and its `METHOD`s and `ACTION`s.
      ┌─ <internal>:3:13
      │
    3 │     PROGRAM this
      │             ^^^^ Invalid use of `THIS`. Usage is only allowed within `FUNCTION_BLOCK` and its `METHOD`s and `ACTION`s.
    ");
}
