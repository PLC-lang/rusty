use crate::{ast::AstStatement::LiteralInteger, ast::*, test_utils::tests::parse, Diagnostic};

#[test]
fn illegal_literal_time_missing_segments_test() {
    let src = "
        PROGRAM exp 
            T#;
        END_PROGRAM
        ";
    let (_, diagnostics) = parse(src);
    assert_eq!(
        diagnostics,
        vec![Diagnostic::unexpected_token_found(
            "Literal",
            ";",
            SourceRange::new(36..37,Some(2),Some(15),Some(2),Some(16))
        )]
    );
}

#[test]
fn time_literal_problems_can_be_recovered_from_during_parsing() {
    let src = "
        PROGRAM exp 
            T#1d4d2h3m;
            x;
        END_PROGRAM
        ";
    let (cu, diagnostics) = parse(src);

    let actual_statements = cu.implementations[0].statements.len();
    assert_eq!(actual_statements, 2);
    assert_eq!(
        diagnostics,
        vec![Diagnostic::syntax_error(
            "Invalid TIME Literal: segments must be unique",
            SourceRange::new(34..44,Some(2),Some(13),Some(2),Some(23))
        )]
    );
}

#[test]
fn illegal_literal_time_double_segments_test() {
    let src = "
        PROGRAM exp 
            T#1d4d2h3m;
        END_PROGRAM
        ";

    let (_, diagnostics) = parse(src);
    assert_eq!(
        diagnostics[0],
        Diagnostic::syntax_error(
            "Invalid TIME Literal: segments must be unique",
            SourceRange::new(34..44,Some(2),Some(13),Some(2),Some(23))
        )
    );
}

#[test]
fn illegal_literal_time_out_of_order_segments_test() {
    let src = "
        PROGRAM exp 
            T#1s2h3d;
        END_PROGRAM
        ";

    let (_, diagnostics) = parse(src);
    assert_eq!(
        diagnostics[0],
        Diagnostic::syntax_error(
            "Invalid TIME Literal: segments out of order, use d-h-m-s-ms",
            SourceRange::new(34..42,Some(2),Some(13),Some(2),Some(21)),
        )
    );
}

#[test]
fn literal_hex_number_with_double_underscores() {
    let src = "PROGRAM exp 16#DEAD__beef; END_PROGRAM";
    let result = parse(src).1;

    assert_eq!(
        result.first().unwrap(),
        &Diagnostic::unexpected_token_found("KeywordSemicolon", "'__beef'", SourceRange::new(19..25,Some(1),Some(19),Some(1),Some(25)))
    );
}

#[test]
fn literal_dec_number_with_double_underscores() {
    let src = "PROGRAM exp 43__000; END_PROGRAM";
    let result = parse(src).1;

    assert_eq!(
        result.first().unwrap(),
        &Diagnostic::unexpected_token_found("KeywordSemicolon", "'__000'", SourceRange::new(14..19,Some(1),Some(14),Some(1),Some(19)))
    );
}

#[test]
fn literal_bin_number_with_double_underscores() {
    let src = "PROGRAM exp 2#01__001_101_01; END_PROGRAM";
    let result = parse(src).1;

    assert_eq!(
        result.first().unwrap(),
        &Diagnostic::unexpected_token_found("KeywordSemicolon", "'__001_101_01'", SourceRange::new(16..28,Some(1),Some(16),Some(1),Some(28)))
    );
}

#[test]
fn literal_oct_number_with_double_underscores() {
    let src = "PROGRAM exp 8#7__7; END_PROGRAM";
    let result = parse(src).1;

    assert_eq!(
        result.first().unwrap(),
        &Diagnostic::unexpected_token_found("KeywordSemicolon", "'__7'", SourceRange::new(15..18,Some(1),Some(15),Some(1),Some(18)))
    );
}

#[test]
fn string_with_round_parens_can_be_parsed() {
    let src = r#"
            TYPE MyString1 : STRING(253); END_TYPE
            TYPE MyString2 : STRING[254) := 'abc'; END_TYPE
            TYPE MyString3 : STRING(255]; END_TYPE
            "#;
    let (result, diagnostics) = parse(src);

    assert_eq!(
        diagnostics,
        vec! [
            Diagnostic::ImprovementSuggestion {
                message: "Unusual type of parentheses around string size expression, consider using square parentheses '[]'".into()
                    ,
                range: SourceRange::new(37..41,Some(2),Some(37),Some(2),Some(41)),
            },
            Diagnostic::ImprovementSuggestion {
                message: "Mismatched types of parentheses around string size expression".into(),
                range: SourceRange::new(88..92,Some(3), Some(37), Some(3),Some(41)),
            },
            Diagnostic::ImprovementSuggestion {
                message: "Mismatched types of parentheses around string size expression".into(),
                range: SourceRange::new(148..152,Some(4), Some(37),Some(4),Some(41)),
            }
        ]
    );

    let ast_string = format!("{:#?}", &result.types);

    let expected_ast = format!(
        "{:#?}",
        vec![
            UserTypeDeclaration {
                data_type: DataType::StringType {
                    name: Some("MyString1".to_string()),
                    size: Some(LiteralInteger {
                        value: 253,
                        location: SourceRange::new(10..11,Some(1),Some(10),Some(1),Some(11)),
                        id: 0,
                    }),
                    is_wide: false,
                },
                initializer: None,
                location: SourceRange::new(18..42,Some(2),Some(18),Some(2),Some(42)),
                scope: None,
            },
            UserTypeDeclaration {
                data_type: DataType::StringType {
                    name: Some("MyString2".to_string()),
                    size: Some(LiteralInteger {
                        value: 254,
                        location: SourceRange::new(10..11,Some(1),Some(10),Some(1),Some(11)),
                        id: 0,
                    }),
                    is_wide: false,
                },
                initializer: Some(AstStatement::LiteralString {
                    is_wide: false,
                    location: SourceRange::new(69..102,Some(3),Some(18),Some(3),Some(42)),
                    value: "abc".into(),
                    id: 0,
                }),
                location: SourceRange::undefined(),
                scope: None,
            },
            UserTypeDeclaration {
                data_type: DataType::StringType {
                    name: Some("MyString3".to_string()),
                    size: Some(LiteralInteger {
                        value: 255,
                        location: SourceRange::new(10..11,Some(1),Some(10),Some(1),Some(11)),
                        id: 0,
                    }),
                    is_wide: false,
                },
                initializer: None,
                location: SourceRange::undefined(),
                scope: None,
            }
        ]
    );

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn literal_cast_with_space() {
    let src = "PROGRAM exp INT# 123; END_PROGRAM";
    let (_, diagnostics) = parse(src);

    assert_eq!(
        vec![Diagnostic::syntax_error(
            "Incomplete statement",
            SourceRange::new(12..16,Some(1),Some(12),Some(1),Some(16))
        )],
        diagnostics
    );
}
