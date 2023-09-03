use insta::{assert_snapshot, assert_debug_snapshot};
use plc_ast::{
    ast::{AstFactory, DataType, UserTypeDeclaration},
    literals::AstLiteral,
};
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
