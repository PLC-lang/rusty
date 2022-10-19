use crate::{test_utils::tests::parse, Diagnostic, SourceRange};

#[test]
fn simple_class_without_name() {
    let src = "CLASS END_CLASS";
    let diagnostics = parse(src).1;

    assert_eq!(
        diagnostics.first().unwrap(),
        &Diagnostic::unexpected_token_found("Identifier", "END_CLASS", SourceRange::new(6..15, None))
    );
}

#[test]
fn method_with_invalid_return_type() {
    let src = "CLASS TestClass METHOD foo : ABSTRACT END_METHOD END_CLASS";
    let diagnostics = parse(src).1;

    assert_eq!(
        diagnostics.first().unwrap(),
        &Diagnostic::unexpected_token_found(
            "DataTypeDefinition",
            "KeywordAbstract",
            SourceRange::new(29..37, None),
        )
    );
}
