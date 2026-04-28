use insta::{assert_debug_snapshot, assert_snapshot};

use plc_diagnostics::diagnostics::Diagnostic;

use crate::test_utils::tests::{parse, parse_and_validate_buffered, parse_buffered};

#[test]
fn illegal_literal_time_missing_segments_test() {
    let src = "
        PROGRAM exp
            T#;
        END_PROGRAM
        ";
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics);
}

#[test]
fn time_literal_problems_can_be_recovered_from_during_parsing() {
    let src = "
        PROGRAM exp
            T#1d4d2h3m;
            x;
        END_PROGRAM
        ";
    let (cu, diagnostics) = parse_buffered(src);

    let actual_statements = cu.implementations[0].statements.len();
    assert_eq!(actual_statements, 2);
    assert_snapshot!(diagnostics);
}

#[test]
fn illegal_literal_time_double_segments_test() {
    let src = "
        PROGRAM exp
            T#1d4d2h3m;
        END_PROGRAM
        ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics);
}

#[test]
fn illegal_literal_time_out_of_order_segments_test() {
    let src = "
        PROGRAM exp
            T#1s2h3d;
        END_PROGRAM
        ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics);
}

#[test]
fn literal_hex_number_with_double_underscores() {
    let src = "PROGRAM exp 16#DEAD__beef; END_PROGRAM";
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics);
}

#[test]
fn literal_dec_number_with_double_underscores() {
    let src = "PROGRAM exp 43__000; END_PROGRAM";
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics);
}

#[test]
fn literal_bin_number_with_double_underscores() {
    let src = "PROGRAM exp 2#01__001_101_01; END_PROGRAM";
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics);
}

#[test]
fn literal_oct_number_with_double_underscores() {
    let src = "PROGRAM exp 8#7__7; END_PROGRAM";
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics);
}

#[test]
fn string_with_round_parens_can_be_parsed() {
    let src = r#"
            TYPE MyString1 : STRING(253); END_TYPE
            TYPE MyString2 : STRING[254) := 'abc'; END_TYPE
            TYPE MyString3 : STRING(255]; END_TYPE
            "#;
    let (result, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics);

    let ast_string = format!("{:#?}", &result.user_types);
    assert_debug_snapshot!(ast_string);
}

#[test]
fn literal_cast_with_space() {
    let src = "PROGRAM exp INT# 123; END_PROGRAM";
    let (_, diagnostics) = parse(src);

    // THEN this should work
    assert_eq!(Vec::<Diagnostic>::new(), diagnostics);
}

// ── E124: invalid escape sequences in string literals ─────────────────────────
//
// NOTE: A trailing '$' immediately before the closing delimiter (e.g. 'hello$')
// is NOT reachable as an E124 diagnostic. The lexer regex treats '$X' (dollar
// followed by any character) as an atomic unit, so '$' before the closing quote
// is consumed as the '$'' quote-escape, leaving the string unterminated. The
// compiler rejects these inputs at the lexer level (E007), not via E124.
// The lit tests in tests/lit/single/string_escapes/string_trailing_dollar.st
// and wstring_trailing_dollar.st document this behaviour at the integration level.

#[test]
fn string_with_unrecognized_escape_emits_diagnostic() {
    let src = r#"
        PROGRAM exp
        VAR s : STRING[20]; END_VAR
            s := 'test$Qtest';
        END_PROGRAM
    "#;
    let (_, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics, @"
    error[E124]: Invalid escape sequence in string literal: '$Q' is not a valid escape sequence
      ┌─ <internal>:4:18
      │
    4 │             s := 'test$Qtest';
      │                  ^^^^^^^^^^^^ Invalid escape sequence in string literal: '$Q' is not a valid escape sequence
    ");
}

#[test]
fn string_with_incomplete_hex_escape_emits_diagnostic() {
    let src = r#"
        PROGRAM exp
        VAR s : STRING[10]; END_VAR
            s := '$A';
        END_PROGRAM
    "#;
    let (_, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics, @"
    error[E124]: Invalid escape sequence in string literal: incomplete hex escape, expected 2 hex digits after '$'
      ┌─ <internal>:4:18
      │
    4 │             s := '$A';
      │                  ^^^^ Invalid escape sequence in string literal: incomplete hex escape, expected 2 hex digits after '$'
    ");
}

#[test]
fn wstring_with_unrecognized_escape_emits_diagnostic() {
    let src = r#"
        PROGRAM exp
        VAR s : WSTRING[20]; END_VAR
            s := "test$Qtest";
        END_PROGRAM
    "#;
    let (_, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics, @r#"
    error[E124]: Invalid escape sequence in string literal: '$Q' is not a valid escape sequence
      ┌─ <internal>:4:18
      │
    4 │             s := "test$Qtest";
      │                  ^^^^^^^^^^^^ Invalid escape sequence in string literal: '$Q' is not a valid escape sequence
    "#);
}

#[test]
fn wstring_with_incomplete_hex_escape_emits_diagnostic() {
    let src = r#"
        PROGRAM exp
        VAR s : WSTRING[10]; END_VAR
            s := "$004";
        END_PROGRAM
    "#;
    let (_, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics, @r#"
    error[E124]: Invalid escape sequence in string literal: incomplete hex escape, expected 4 hex digits after '$'
      ┌─ <internal>:4:18
      │
    4 │             s := "$004";
      │                  ^^^^^^ Invalid escape sequence in string literal: incomplete hex escape, expected 4 hex digits after '$'
    "#);
}

#[test]
fn string_with_multiple_invalid_escapes_emits_multiple_diagnostics() {
    // '$Q$Z': two consecutive unrecognised escapes in a properly-terminated string.
    // Note: '$' immediately before the closing delimiter (e.g. 'hello$') is a lexer-level
    // error, not E124, so such cases are not tested here.
    let src = r#"
        PROGRAM exp
        VAR s : STRING[20]; END_VAR
            s := '$Q$Z';
        END_PROGRAM
    "#;
    let (_, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics, @"
    error[E124]: Invalid escape sequence in string literal: '$Q' is not a valid escape sequence
      ┌─ <internal>:4:18
      │
    4 │             s := '$Q$Z';
      │                  ^^^^^^ Invalid escape sequence in string literal: '$Q' is not a valid escape sequence

    error[E124]: Invalid escape sequence in string literal: '$Z' is not a valid escape sequence
      ┌─ <internal>:4:18
      │
    4 │             s := '$Q$Z';
      │                  ^^^^^^ Invalid escape sequence in string literal: '$Z' is not a valid escape sequence
    ");
}
