// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{ast::SourceRange, parser::parse, Diagnostic};
use pretty_assertions::*;

#[test]
fn missing_semicolon_reported_as_diagnostic() {
    let lexer = super::lex(
        r"
                PROGRAM foo 
                    buz()
                    foo();
                END_PROGRAM
    ",
    );

    let (compilation_unit, _, diagnostics) = parse(lexer).unwrap();
    //expected end of statement (e.g. ;), but found KeywordEndProgram at line: 1 offset: 14..25"
    //Expecting a missing semicolon message
    let expected = Diagnostic::unexpected_token_found(
        "KeywordSemicolon".into(),
        "'foo', (Identifier)".into(),
        SourceRange::new("", 76..79),
    );
    assert_eq!(diagnostics[0], expected);

    let pou = &compilation_unit.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements),
        r#"[
    CallStatement {
        operator: Reference {
            name: "buz",
        },
        parameters: None,
    },
    CallStatement {
        operator: Reference {
            name: "foo",
        },
        parameters: None,
    },
]"#
    );
}

#[test]
fn illegal_literal_time_missing_segments_test() {
    let lexer = super::lex(
        "
        PROGRAM exp 
            T#;
        END_PROGRAM
        ",
    );
    let (_, _, diagnostics) = parse(lexer).unwrap();
    let expected = Diagnostic::unexpected_token_found(
        "KeywordSemicolon".into(),
        "'#', (Error)".into(),
        SourceRange::new("", 35..36),
    );
    assert_eq!(diagnostics[0], expected);

    let expected = Diagnostic::illegal_token("#", SourceRange::new("", 35..36));
    assert_eq!(diagnostics[1], expected);
}

#[test]
fn time_literal_problems_can_be_recovered_from_during_parsing() {
    let lexer = super::lex(
        "
        PROGRAM exp 
            T#1d4d2h3m;
            x;
        END_PROGRAM
        ",
    );
    let (cu, ..) = parse(lexer).unwrap();

    let actual_statements = cu.implementations[0].statements.len();
    assert_eq!(actual_statements, 2);
}

#[test]
fn illegal_literal_time_double_segments_test() {
    let lexer = super::lex(
        "
        PROGRAM exp 
            T#1d4d2h3m;
        END_PROGRAM
        ",
    );

    let (_, _, diagnostics) = parse(lexer).unwrap();
    assert_eq!(
        diagnostics[0],
        Diagnostic::unexpected_token(
            "Invalid TIME Literal: segments must be unique".into(),
            SourceRange::new("", 34..45)
        )
    );
}

#[test]
fn illegal_literal_time_out_of_order_segments_test() {
    let lexer = super::lex(
        "
        PROGRAM exp 
            T#1s2h3d;
        END_PROGRAM
        ",
    );

    let (_, _, diagnostics) = parse(lexer).unwrap();
    assert_eq!(
        diagnostics[0],
        Diagnostic::unexpected_token(
            "Invalid TIME Literal: segments out of order, use d-h-m-s-ms".into(),
            SourceRange::new("", 34..43)
        )
    );
}

#[test]
fn incomplete_statement_test() {
    let lexer = super::lex(
        "
        PROGRAM exp 
            1 + 2 +;
            x;
        END_PROGRAM
        ",
    );

    let (cu, _, diagnostics) = parse(lexer).unwrap();
    let pou = &cu.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements),
        r#"[
    EmptyStatement,
    Reference {
        name: "x",
    },
]"#
    );

    assert_eq!(
        diagnostics[0],
        Diagnostic::unexpected_token(
            "unexpected token: ';' [KeywordSemicolon] at line: 3 offset: 20..21".into(),
            SourceRange::new("", 34..42)
        )
    );
}

#[test]
fn incomplete_statement_in_parantheses_recovery_test() {
    let lexer = super::lex(
        "
        PROGRAM exp 
            (1 + 2 - ) + 3;
            x;
        END_PROGRAM
        ",
    );

    let (cu, _, diagnostics) = parse(lexer).unwrap();
    let pou = &cu.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements),
        r#"[
    BinaryExpression {
        operator: Plus,
        left: EmptyStatement,
        right: LiteralInteger {
            value: "3",
        },
    },
    Reference {
        name: "x",
    },
]"#
    );

    assert_eq!(
        diagnostics[0],
        Diagnostic::unexpected_token(
            "unexpected token: ')' [KeywordParensClose] at line: 3 offset: 22..23".into(),
            SourceRange::new("", 35..44)
        )
    );
}

#[test]
fn mismatched_parantheses_recovery_test() {
    let lexer = super::lex(
        "
        PROGRAM exp 
            (1 + 2;
            x;
        END_PROGRAM
        ",
    );

    let (cu, _, diagnostics) = parse(lexer).unwrap();
    let pou = &cu.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements),
        r#"[
    BinaryExpression {
        operator: Plus,
        left: LiteralInteger {
            value: "1",
        },
        right: LiteralInteger {
            value: "2",
        },
    },
    Reference {
        name: "x",
    },
]"#
    );

    assert_eq!(
        diagnostics[0],
        Diagnostic::unexpected_token_found(
            "KeywordParensClose".into(),
            "';', (KeywordSemicolon)".into(),
            SourceRange::new("", 40..41)
        )
    );
    // assert_eq!(
    //     diagnostics[0],
    //     Diagnostic::unexpected_token("expected 'KeywordParensClose' but found ';' (KeywordSemicolon)".into(), SourceRange::new("", 40..41))
    // );
}
