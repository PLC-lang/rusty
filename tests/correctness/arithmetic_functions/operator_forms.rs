use driver::runner::compile_and_run_no_params;

#[test]
fn builtin_mod_function_form_matches_operator_form() {
    let prog = r#"
    FUNCTION main : DINT
        main := MOD(17, 5) * 10 + (17 MOD 5);
    END_FUNCTION
    "#;

    let res: i32 = compile_and_run_no_params(prog.to_string());
    assert_eq!(res, 22);
}

#[test]
fn builtin_and_function_form_is_variadic_and_keeps_operator_form() {
    let prog = r#"
    FUNCTION main : BOOL
        main := AND(TRUE, TRUE, TRUE) AND (TRUE AND TRUE);
    END_FUNCTION
    "#;

    let res: bool = compile_and_run_no_params(prog.to_string());
    assert!(res);
}

#[test]
fn builtin_or_function_form_is_variadic_and_keeps_operator_form() {
    let prog = r#"
    FUNCTION main : BOOL
        main := OR(FALSE, FALSE, TRUE) AND (FALSE OR TRUE);
    END_FUNCTION
    "#;

    let res: bool = compile_and_run_no_params(prog.to_string());
    assert!(res);
}

#[test]
fn builtin_xor_function_form_is_variadic_and_keeps_operator_form() {
    let prog = r#"
    FUNCTION main : BOOL
        main := XOR(TRUE, FALSE, TRUE) = (TRUE XOR FALSE XOR TRUE);
    END_FUNCTION
    "#;

    let res: bool = compile_and_run_no_params(prog.to_string());
    assert!(res);
}
