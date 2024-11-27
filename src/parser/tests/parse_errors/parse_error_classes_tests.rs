use insta::assert_snapshot;

use crate::test_utils::tests::{parse_and_validate_buffered, parse_buffered};

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

#[test]
fn declaring_methods_in_functions_is_an_error() {
    let src = r#"
    FUNCTION bar
        METHOD anyMethod
        ;
        END_METHOD
        ;
    END_FUNCTION
    "#;
    let (_, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics, @r###"
    error[E001]: Methods cannot be declared in a POU of type 'Function'.
      ┌─ <internal>:2:14
      │
    2 │     FUNCTION bar
      │              ^^^ Methods cannot be declared in a POU of type 'Function'.

    error[E007]: Unexpected token: expected Literal but found METHOD
      ┌─ <internal>:3:9
      │
    3 │         METHOD anyMethod
      │         ^^^^^^ Unexpected token: expected Literal but found METHOD

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'METHOD anyMethod'
      ┌─ <internal>:3:9
      │
    3 │         METHOD anyMethod
      │         ^^^^^^^^^^^^^^^^ Unexpected token: expected KeywordSemicolon but found 'METHOD anyMethod'

    error[E007]: Unexpected token: expected Literal but found END_METHOD
      ┌─ <internal>:5:9
      │
    5 │         END_METHOD
      │         ^^^^^^^^^^ Unexpected token: expected Literal but found END_METHOD

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'END_METHOD'
      ┌─ <internal>:5:9
      │
    5 │         END_METHOD
      │         ^^^^^^^^^^ Unexpected token: expected KeywordSemicolon but found 'END_METHOD'
    "###);
}
