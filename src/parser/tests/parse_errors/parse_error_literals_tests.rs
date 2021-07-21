use crate::{
    ast::SourceRange,
    parser::{parse, tests::lex},
    Diagnostic,
};

#[test]
fn illegal_literal_time_missing_segments_test() {
    let lexer = lex("
        PROGRAM exp 
            T#;
        END_PROGRAM
        ");
    let (_, diagnostics) = parse(lexer).unwrap();
    assert_eq!(
        diagnostics,
        vec![Diagnostic::unexpected_token_found(
            "KeywordSemicolon".into(),
            "'#'".into(),
            SourceRange::new(35..36)
        )]
    );
}

#[test]
fn time_literal_problems_can_be_recovered_from_during_parsing() {
    let lexer = lex("
        PROGRAM exp 
            T#1d4d2h3m;
            x;
        END_PROGRAM
        ");
    let (cu, diagnostics) = parse(lexer).unwrap();

    let actual_statements = cu.implementations[0].statements.len();
    assert_eq!(actual_statements, 2);
    assert_eq!(
        diagnostics,
        vec![Diagnostic::syntax_error(
            "Invalid TIME Literal: segments must be unique".into(),
            (34..44).into()
        )]
    );
}

#[test]
fn illegal_literal_time_double_segments_test() {
    let lexer = lex("
        PROGRAM exp 
            T#1d4d2h3m;
        END_PROGRAM
        ");

    let (_, diagnostics) = parse(lexer).unwrap();
    assert_eq!(
        diagnostics[0],
        Diagnostic::syntax_error(
            "Invalid TIME Literal: segments must be unique".into(),
            SourceRange::new(34..44)
        )
    );
}

#[test]
fn illegal_literal_time_out_of_order_segments_test() {
    let lexer = lex("
        PROGRAM exp 
            T#1s2h3d;
        END_PROGRAM
        ");

    let (_, diagnostics) = parse(lexer).unwrap();
    assert_eq!(
        diagnostics[0],
        Diagnostic::syntax_error(
            "Invalid TIME Literal: segments out of order, use d-h-m-s-ms".into(),
            SourceRange::new(34..42)
        )
    );
}
