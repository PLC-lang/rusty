use driver::runner::compile_and_run_no_params;

#[test]
fn builtin_mod_function_form_widens_its_result() {
    for (expression, expected) in [
        ("MOD(SINT#-17, UINT#256)", -17),
        ("MOD(USINT#255, UINT#256)", 255),
        ("MOD(SINT#17, LINT#5) + LINT#300", 302),
        ("MOD(IN2 := UINT#256, IN1 := SINT#-17)", -17),
        ("MOD(IN2 := UINT#256, SINT#-17)", -17),
        ("MOD(IN1 := (SINT#-17), IN2 := UINT#256)", -17),
        ("SIZEOF(MOD(IN2 := LINT#5, IN1 := SINT#17))", 1),
    ] {
        let prog = format!("FUNCTION main : LINT main := {expression}; END_FUNCTION");
        let res: i64 = compile_and_run_no_params(prog);
        assert_eq!(res, expected, "{expression}");
    }
}

#[test]
fn builtin_bitwise_function_forms_accept_mixed_bit_string_widths() {
    let prog = r#"
    FUNCTION main : BOOL
        main := (AND(BYTE#16#FF, WORD#16#0F, DWORD#16#0A) = DWORD#16#0A)
            AND (OR(BYTE#1, WORD#256, DWORD#65536) = DWORD#65793)
            AND (XOR(BYTE#16#FF, WORD#16#FF, DWORD#0) = DWORD#0);
    END_FUNCTION
    "#;

    let res: bool = compile_and_run_no_params(prog.to_string());
    assert!(res);
}

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
