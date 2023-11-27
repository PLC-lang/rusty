use crate::correctness::math_operators::addition::approx_equal;
use driver::runner::{compile_and_run, MainType};

#[test]
fn builtin_sub_with_ints() {
    let prog = r#"
    FUNCTION main : DINT
    VAR
        x1 : DINT := 1000;
        l1 : LINT := 333;
    END_VAR
        main := SUB(x1, l1);
    END_FUNCTION
    "#;

    let mut main = MainType::default();

    let res: i32 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 777);
}

#[test]
fn builtin_sub_with_floats() {
    let prog = r#"
    FUNCTION main : LREAL
    VAR
        x1 : REAL :=  10.0;
        x2 : LREAL := 1000.0;
    END_VAR
        main := SUB(x1, x2);
    END_FUNCTION
    "#;

    let mut main = MainType::default();

    let res: f64 = compile_and_run(prog.to_string(), &mut main);
    assert!(approx_equal(-990.0, res, 2));
}

#[test]
fn builtin_sub_with_ints_and_floats() {
    let prog = r#"
    FUNCTION main : LREAL
    VAR
        x1 : DINT := 20;
        x2 : LREAL := 19.9;
    END_VAR
        main := SUB(x1, x2);
    END_FUNCTION
    "#;

    let mut main = MainType::default();

    let res: f64 = compile_and_run(prog.to_string(), &mut main);
    assert!(approx_equal(0.1, res, 1));
}
