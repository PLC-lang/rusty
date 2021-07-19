use crate::parser::parse;
use crate::Diagnostic;
use pretty_assertions::*;

#[test]
fn test_unexpected_token_error_message() {
    let source = "PROGRAM prg
                VAR ;
                END_VAR
            END_PROGRAM
    ";
    let lexer = super::super::lex(source);
    let (_, diagnostics) = parse(lexer).unwrap();

    assert_eq!(
        format!("{:?}", diagnostics),
        format!(
            "{:?}",
            vec![Diagnostic::unexpected_token_found(
                "KeywordEndVar".into(),
                "';'".into(),
                (32..33).into()
            ),]
        )
    );
}

#[test]
fn test_unexpected_token_error_message2() {
    let lexer = super::super::lex(
        "SOME PROGRAM prg
                VAR ;
                END_VAR
            END_PROGRAM
    ",
    );
    let parse_result = parse(lexer);

    if let Err { 0: msg } = parse_result {
        assert_eq!(
            Diagnostic::syntax_error("Unexpected token: 'SOME'".into(), (0..4).into()),
            msg
        );
    } else {
        panic!("Expected parse error but didn't get one.");
    }
}

#[test]
fn test_unclosed_body_error_message() {
    let lexer = super::super::lex(
        "
            
            PROGRAM My_PRG

    ",
    );
    let (_, diagnostics) = parse(lexer).unwrap();

    assert_eq!(
        diagnostics,
        vec![Diagnostic::unexpected_token_found(
            "KeywordEndProgram".into(),
            "''".into(),
            (46..46).into()
        )]
    );
}
