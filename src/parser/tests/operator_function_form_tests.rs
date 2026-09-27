use crate::test_utils::tests::parse;

#[test]
fn operator_keywords_are_accepted_in_function_form() {
    let src = r#"
        PROGRAM exp
        VAR
            a, b, r : DINT;
            x, y, z : BOOL;
        END_VAR
            r := MOD(a, b);
            z := AND(x, y);
            z := OR(x, y);
            z := XOR(x, y);

            r := a MOD b;
            z := x AND y;
            z := x OR y;
            z := x XOR y;
        END_PROGRAM
    "#;

    let (_, diagnostics) = parse(src);
    assert!(diagnostics.is_empty(), "expected no parse diagnostics, got: {diagnostics:#?}");
}

#[test]
fn operator_keywords_are_accepted_in_function_body() {
    let src = r#"
        FUNCTION main : DINT
            main := MOD(17, 5);
        END_FUNCTION
    "#;

    let (_, diagnostics) = parse(src);
    assert!(diagnostics.is_empty(), "expected no parse diagnostics, got: {diagnostics:#?}");
}
