use crate::correctness::math_operators::addition::approx_equal;
use driver::runner::{compile_and_run, MainType};

#[test]
fn mul_with_ints() {
    let prog = r#"
    FUNCTION main : DINT
    VAR
        x1 : ARRAY[0..3] OF DINT := (1, 2, 3, 4);
        l1 : LINT := 1000;
        s1 : SINT := 5;
    END_VAR
        main := MUL(x1[0], x1[1], x1[2], x1[3], l1, s1);
    END_FUNCTION
    "#;

    let mut main = MainType::default();
    let expected = 1 * 2 * 3 * 4 * 1000 * 5;
    let res: i32 = compile_and_run(prog.to_string(), &mut main);
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

    let mut main = MainType::default();
    let expected = 1.0 * 2.2 * 3.4 * 4.1 * 1000.9;
    let res: f64 = compile_and_run(prog.to_string(), &mut main);
    assert!(approx_equal(expected, res, 1));
}
