use crate::correctness::math_operators::addition::approx_equal;
use driver::runner::compile_and_run_no_params;

#[test]
fn builtin_add_with_ints() {
    let prog = r#"
    FUNCTION main : LINT
    VAR_TEMP
        x1 : ARRAY[0..3] OF DINT := (1, 2, 3, 4);
        l1 : LINT := 1000;
        s1 : SINT := 1;
    END_VAR
        main := ADD(x1[0], x1[1], x1[2], x1[3], l1, s1);
    END_FUNCTION
    "#;

    let res: i64 = compile_and_run_no_params(prog.to_string());
    assert_eq!(res, 1011);
}

#[test]
fn builtin_add_with_floats() {
    let prog = r#"
    FUNCTION main : LREAL
    VAR_TEMP
        x1 : ARRAY[0..3] OF REAL := (1.0, 2.2, 3.4, 4.1);
        x2 : LREAL := 1000.9;
    END_VAR
        main := ADD(x1[0], x1[1], x1[2], x1[3], x2);
    END_FUNCTION
    "#;

    let res: f64 = compile_and_run_no_params(prog.to_string());
    assert!(approx_equal(1011.6, res, 1));
}

#[test]
fn builtin_add_with_ints_and_floats() {
    let prog = r#"
    FUNCTION main : LREAL
    VAR_TEMP
        x1 : ARRAY[0..3] OF DINT := (1, 2, 3, 4);
        x2 : LREAL := 1000.9;
    END_VAR
        main := ADD(x1[0], x1[1], x1[2], x1[3], x2);
    END_FUNCTION
    "#;

    let res: f64 = compile_and_run_no_params(prog.to_string());
    assert!(approx_equal(1010.9, res, 1));
}
