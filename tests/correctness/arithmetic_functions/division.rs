use crate::correctness::math_operators::addition::approx_equal;
use driver::runner::{compile_and_run, MainType};

#[test]
fn builtin_div_with_ints() {
    let prog = r#"
    FUNCTION main : DINT
    VAR
        x1 : DINT := 1000;
        l1 : LINT := 333;
    END_VAR
        main := DIV(x1, l1);
    END_FUNCTION
    "#;

    let mut main = MainType::default();

    let res: i32 = compile_and_run(prog.to_string(), &mut main);
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

    let mut main = MainType::default();

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

    let mut main = MainType::default();

    let res: f64 = compile_and_run(prog.to_string(), &mut main);
    assert!(approx_equal(0.02, res, 2));
}

#[test]
fn builtin_div_with_dates() {
    let prog = r#"
    FUNCTION main : LINT
    VAR
        x1 : ARRAY[0..3] OF DATE := (DATE#1970-01-02, DATE#2000-01-02);
    END_VAR
        main := DIV(x1[0], x1[1]);
    END_FUNCTION
    "#;

    let mut main = MainType::default();
    let get_timestamp = |year, month, day| {
        chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_nano_opt(0, 0, 0, 0)
            .unwrap()
            .timestamp_nanos()
    };

    let expected = get_timestamp(1970, 1, 1)
        / get_timestamp(2000, 1, 2)
        / get_timestamp(2023, 5, 30)
        / get_timestamp(1999, 12, 31);

    let res: i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(expected, res);
}
