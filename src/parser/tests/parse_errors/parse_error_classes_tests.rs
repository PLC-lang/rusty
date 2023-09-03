use insta::assert_snapshot;

use crate::test_utils::tests::parse_and_validate_buffered;

#[test]
fn simple_class_without_name() {
    let src = "CLASS END_CLASS";
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics)
}

#[test]
fn method_with_invalid_return_type() {
    let src = "CLASS TestClass METHOD foo : ABSTRACT END_METHOD END_CLASS";
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics)
}
