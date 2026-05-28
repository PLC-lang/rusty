//! Tests covering the problem #1404 aims to solve: when a reserved keyword
//! appears in a user-supplied name slot (variable name, POU name, parameter
//! name, type alias name, enum variant name, struct field name), the parser
//! today emits a generic "unexpected token" diagnostic that often cascades
//! into a misleading follow-on. The cases below pin the inputs; the
//! implementation choice for the diagnostic shape is open.
//!
//! Each positive test uses an empty inline snapshot — until the parser change
//! lands, every test fails with the *current* diagnostic captured as the new
//! snapshot, which surfaces the input/output gap. When the change ships, the
//! implementer reviews and accepts the produced snapshots in one pass.

use crate::test_utils::tests::parse_and_validate_buffered;
use insta::assert_snapshot;

#[test]
fn retain_as_variable_name_in_var_block() {
    let source = r"
        FUNCTION main
            VAR
                retain : DINT;
            END_VAR
        END_FUNCTION
    ";
    let diagnostics = parse_and_validate_buffered(source);
    assert_snapshot!(diagnostics, @"
    error[E138]: `RETAIN` is a reserved keyword and cannot be used as a variable name
      ┌─ <internal>:4:17
      │
    4 │                 retain : DINT;
      │                 ^^^^^^ `RETAIN` is a reserved keyword and cannot be used as a variable name
    ");
}

#[test]
fn program_as_variable_name_in_var_block() {
    let source = r"
        FUNCTION main
            VAR
                program : DINT;
            END_VAR
        END_FUNCTION
    ";
    let diagnostics = parse_and_validate_buffered(source);
    assert_snapshot!(diagnostics, @"
    error[E007]: Unexpected token: expected KeywordEndVar but found 'program : DINT;'
      ┌─ <internal>:4:17
      │
    4 │                 program : DINT;
      │                 ^^^^^^^^^^^^^^^ Unexpected token: expected KeywordEndVar but found 'program : DINT;'
    ");
}

#[test]
fn retain_as_function_name() {
    let source = r"
        FUNCTION retain : DINT
        END_FUNCTION
    ";
    let diagnostics = parse_and_validate_buffered(source);
    assert_snapshot!(diagnostics, @"
    error[E138]: `RETAIN` is a reserved keyword and cannot be used as a function name
      ┌─ <internal>:2:18
      │
    2 │         FUNCTION retain : DINT
      │                  ^^^^^^ `RETAIN` is a reserved keyword and cannot be used as a function name
    ");
}

#[test]
fn retain_as_program_name() {
    let source = r"
        PROGRAM retain
        END_PROGRAM
    ";
    let diagnostics = parse_and_validate_buffered(source);
    assert_snapshot!(diagnostics, @"
    error[E138]: `RETAIN` is a reserved keyword and cannot be used as a program name
      ┌─ <internal>:2:17
      │
    2 │         PROGRAM retain
      │                 ^^^^^^ `RETAIN` is a reserved keyword and cannot be used as a program name
    ");
}

#[test]
fn constant_as_function_block_name() {
    let source = r"
        FUNCTION_BLOCK constant
        END_FUNCTION_BLOCK
    ";
    let diagnostics = parse_and_validate_buffered(source);
    assert_snapshot!(diagnostics, @"
    error[E138]: `CONSTANT` is a reserved keyword and cannot be used as a function block name
      ┌─ <internal>:2:24
      │
    2 │         FUNCTION_BLOCK constant
      │                        ^^^^^^^^ `CONSTANT` is a reserved keyword and cannot be used as a function block name
    ");
}

#[test]
fn retain_as_var_input_parameter_name() {
    let source = r"
        FUNCTION foo : DINT
            VAR_INPUT
                retain : DINT;
            END_VAR
        END_FUNCTION
    ";
    let diagnostics = parse_and_validate_buffered(source);
    assert_snapshot!(diagnostics, @"
    error[E138]: `RETAIN` is a reserved keyword and cannot be used as a parameter name
      ┌─ <internal>:4:17
      │
    4 │                 retain : DINT;
      │                 ^^^^^^ `RETAIN` is a reserved keyword and cannot be used as a parameter name
    ");
}

#[test]
fn retain_as_type_alias_name() {
    let source = r"
        TYPE retain : DINT; END_TYPE
    ";
    let diagnostics = parse_and_validate_buffered(source);
    assert_snapshot!(diagnostics, @"
    error[E138]: `RETAIN` is a reserved keyword and cannot be used as a type name
      ┌─ <internal>:2:14
      │
    2 │         TYPE retain : DINT; END_TYPE
      │              ^^^^^^ `RETAIN` is a reserved keyword and cannot be used as a type name
    ");
}

#[test]
fn retain_as_enum_variant_name() {
    let source = r"
        TYPE my_enum : (A, retain, B); END_TYPE
    ";
    let diagnostics = parse_and_validate_buffered(source);
    assert_snapshot!(diagnostics, @"
    error[E138]: `RETAIN` is a reserved keyword and cannot be used as an enum variant name
      ┌─ <internal>:2:28
      │
    2 │         TYPE my_enum : (A, retain, B); END_TYPE
      │                            ^^^^^^ `RETAIN` is a reserved keyword and cannot be used as an enum variant name
    ");
}

#[test]
fn retain_as_struct_field_name() {
    let source = r"
        TYPE s : STRUCT retain : DINT; END_STRUCT END_TYPE
    ";
    let diagnostics = parse_and_validate_buffered(source);
    assert_snapshot!(diagnostics, @"
    error[E138]: `RETAIN` is a reserved keyword and cannot be used as a struct field name
      ┌─ <internal>:2:25
      │
    2 │         TYPE s : STRUCT retain : DINT; END_STRUCT END_TYPE
      │                         ^^^^^^ `RETAIN` is a reserved keyword and cannot be used as a struct field name
    ");
}

#[test]
fn retain_in_valid_modifier_position_is_clean() {
    // Negative case: `RETAIN` in its legitimate variable-block-modifier slot
    // (`VAR RETAIN ... END_VAR`) must parse cleanly with no diagnostic.
    let source = r"
        FUNCTION_BLOCK fb
            VAR RETAIN
                x : DINT;
            END_VAR
        END_FUNCTION_BLOCK
    ";
    let diagnostics = parse_and_validate_buffered(source);
    assert_snapshot!(diagnostics, @"");
}

// --- Regression pins for out-of-scope reference/expression positions. ---
// These keep the *current* "unexpected token" behaviour around uses of reserved
// keywords outside declaration slots. #1404 deliberately does not touch them;
// the empty snapshots capture today's output so a future change is a deliberate
// snapshot update.

#[test]
fn retain_as_variable_reference_in_expression_is_unexpected_token() {
    let source = r"
        FUNCTION main : DINT
            VAR x : DINT; END_VAR
            x := retain;
        END_FUNCTION
    ";
    let diagnostics = parse_and_validate_buffered(source);
    assert_snapshot!(diagnostics, @"
    error[E007]: Unexpected token: expected Literal but found retain
      ┌─ <internal>:4:18
      │
    4 │             x := retain;
      │                  ^^^^^^ Unexpected token: expected Literal but found retain

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'retain'
      ┌─ <internal>:4:18
      │
    4 │             x := retain;
      │                  ^^^^^^ Unexpected token: expected KeywordSemicolon but found 'retain'
    ");
}

#[test]
fn retain_as_call_target_is_unexpected_token() {
    let source = r"
        FUNCTION main
            retain();
        END_FUNCTION
    ";
    let diagnostics = parse_and_validate_buffered(source);
    assert_snapshot!(diagnostics, @"
    error[E007]: Unexpected token: expected Literal but found retain
      ┌─ <internal>:3:13
      │
    3 │             retain();
      │             ^^^^^^ Unexpected token: expected Literal but found retain

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'retain()'
      ┌─ <internal>:3:13
      │
    3 │             retain();
      │             ^^^^^^^^ Unexpected token: expected KeywordSemicolon but found 'retain()'
    ");
}

#[test]
fn retain_as_member_access_is_unexpected_token() {
    let source = r"
        FUNCTION_BLOCK fb VAR x : DINT; END_VAR END_FUNCTION_BLOCK
        FUNCTION main
            VAR f : fb; END_VAR
            f.retain;
        END_FUNCTION
    ";
    let diagnostics = parse_and_validate_buffered(source);
    assert_snapshot!(diagnostics, @"
    error[E007]: Unexpected token: expected Literal but found retain
      ┌─ <internal>:5:15
      │
    5 │             f.retain;
      │               ^^^^^^ Unexpected token: expected Literal but found retain

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'retain'
      ┌─ <internal>:5:15
      │
    5 │             f.retain;
      │               ^^^^^^ Unexpected token: expected KeywordSemicolon but found 'retain'
    ");
}
