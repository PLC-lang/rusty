use crate::correctness::math_operators::addition::approx_equal;
use driver::runner::compile_and_run;

struct MainType;

#[test]
fn builtin_div_with_ints() {
    let prog = r#"
    FUNCTION main : LINT
    VAR
        x1 : DINT := 1000;
        l1 : LINT := 333;
    END_VAR
        main := DIV(x1, l1);
    END_FUNCTION
    "#;

    let mut main = MainType {};

    let res: i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 3);
}

#[test]
fn builtin_div_with_floats() {
    let prog = r#"
    FUNCTION main : LREAL
    VAR
        x1 : REAL :=  10.0;
        x2 : LREAL := 1000.0;
    END_VAR
        main := DIV(x1, x2);
    END_FUNCTION
    "#;

    let mut main = MainType {};

    let res: f64 = compile_and_run(prog.to_string(), &mut main);
    assert!(approx_equal(0.01, res, 2));
}

#[test]
fn builtin_div_with_ints_and_floats() {
    let prog = r#"
    FUNCTION main : LREAL
    VAR
        x1 : DINT := 20;
        x2 : LREAL := 1000.0;
    END_VAR
        main := DIV(x1, x2);
    END_FUNCTION
    "#;

    let mut main = MainType {};

    let res: f64 = compile_and_run(prog.to_string(), &mut main);
    assert!(approx_equal(0.02, res, 2));
}
