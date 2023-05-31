use driver::runner::{compile_and_run, MainType};
use crate::correctness::math_operators::addition::approx_equal;

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

#[test]
fn mul_with_dates() {
    let prog = r#"
    FUNCTION main : LINT
    VAR
        x1 : ARRAY[0..3] OF DATE := (DATE#1970-01-01, DATE#2000-01-02, DATE#2023-05-30);
        x2 : DATE := DATE#1999-12-31;
    END_VAR
        main := MUL(x1[0], x1[1], x1[2], x1[3], x2);
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
        * get_timestamp(2000, 1, 2)
        * get_timestamp(2023, 5, 30)
        * get_timestamp(1999, 12, 31);

    let res: i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(expected, res);
}
