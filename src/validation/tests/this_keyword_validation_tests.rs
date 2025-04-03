use insta::assert_snapshot;
use test_utils::parse_and_validate_buffered;

#[test]
fn pointer_arithmetic_with_this() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    FUNCTION_BLOCK parent
    VAR
        x : LINT := 10;
        y : LINT := 20;
    END_VAR
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK child EXTENDS parent
    VAR
        a : INT;
    END_VAR
        // Pointer arithmetic with SUPER
        a := (THIS + 1)^ + 5;
    END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}
