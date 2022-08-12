use crate::{test_utils::tests::parse_and_validate, Diagnostic, ast::SourceRange};

#[test]
fn function_no_return_unsupported() {
    // GIVEN FUNCTION with no return type
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate("FUNCTION foo VAR_INPUT END_VAR END_FUNCTION");
    // THEN there should be one diagnostic -> missing return type
    assert_eq!(
        diagnostics,
        vec![Diagnostic::function_return_missing(SourceRange::new(9..12,Some(1),Some(9),Some(1),Some(12)))]
    );
}
