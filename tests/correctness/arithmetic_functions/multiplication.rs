use crate::correctness::math_operators::addition::approx_equal;
use driver::runner::compile_and_run_no_params;

#[test]
fn mul_with_ints() {
    let prog = r#"
    FUNCTION main : LINT
    VAR
        x1 : ARRAY[0..3] OF DINT := (1, 2, 3, 4);
        l1 : LINT := 1000;
        s1 : SINT := 5;
    END_VAR
        main := MUL(x1[0], x1[1], x1[2], x1[3], l1, s1);
    END_FUNCTION
    "#;

    let expected = 1 * 2 * 3 * 4 * 1000 * 5;
    let res: i64 = compile_and_run_no_params(prog.to_string());
    assert_eq!(expected, res);
}

#[test]
fn mul_with_floats() {
    let prog = r#"
    FUNCTION main : LREAL
    VAR
        x1 : ARRAY[0..3] OF REAL := (1.0, 2.2, 3.4, 4.1);
        x2 : LREAL := 1000.9;
    END_VAR
        main := MUL(x1[0], x1[1], x1[2], x1[3], x2);
    END_FUNCTION
    "#;

    let expected = 1.0 * 2.2 * 3.4 * 4.1 * 1000.9;
    let res: f64 = compile_and_run_no_params(prog.to_string());
    assert!(approx_equal(expected, res, 1));
}

#[test]
fn builtin_mul_with_ints_and_floats() {
    let prog = r#"
    FUNCTION main : LREAL
    VAR
        x1 : ARRAY[0..3] OF DINT := (1, 2, 3, 4);
        x2 : LREAL := 0.5;
    END_VAR
        main := MUL(x1[0], x1[1], x1[2], x1[3], x2);
    END_FUNCTION
    "#;

    let res: f64 = compile_and_run_no_params(prog.to_string());
    assert!(approx_equal(12.0, res, 1));
}
