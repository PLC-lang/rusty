use crate::{
    parser::{parse, tests::lex},
    Diagnostic, SourceRange,
};

#[test]
fn simple_class_without_name() {
    let lexer = lex("CLASS END_CLASS");
    let diagnostics = parse(lexer).1;

    assert_eq!(
        diagnostics.first().unwrap(),
        &Diagnostic::unexpected_token_found(
            "Identifier".into(),
            "END_CLASS".into(),
            SourceRange::new(6..15)
        )
    );
}

#[test]
fn method_with_invalid_return_type() {
    let lexer = lex("CLASS TestClass METHOD foo : ABSTRACT END_METHOD END_CLASS");
    let diagnostics = parse(lexer).1;

    assert_eq!(
        diagnostics.first().unwrap(),
        &Diagnostic::unexpected_token_found(
            "Datatype".into(),
            "ABSTRACT".into(),
            SourceRange::new(29..37),
        )
    );
}
