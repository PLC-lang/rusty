use crate::{assert_almost_eq, compile_and_run, MainType};
use chrono::TimeZone;
use num::{Float, NumCast};

//--------------------------------------------------------------
//multiplication tests

#[test]
fn multiplication_basic() {
    let prog = "
    FUNCTION main : DINT
        main := 6 * 100;
    END_FUNCTION
    ";

    let res: i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 600)
}

#[test]
fn order_of_operations_mul() {
    let prog = "
    FUNCTION main : DINT
    main := 10 * 10 / 5 / 2;
    END_FUNCTION
    ";
    let res: i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 10)
}

#[test]
fn multiplication_vars_real_type() {
    let prog = "
    FUNCTION main : REAL
    VAR
        i1, i2 : INT;
        x1 : REAL := 50.3;
        x2 : REAL := 2.0;
    END_VAR
        main := x1 * x2;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: f32 = compile_and_run(prog.to_string(), &mut main);
    assert_almost_eq!(100.6, res, f32::EPSILON);
}

#[test]
fn multiplication_multiple_vars_lreal_type() {
    let prog = "
    FUNCTION main : LREAL
    VAR
        r1 : LREAL := 20.8;
        r2 : LREAL := 0.2;
    END_VAR
        main := r1 * 3 * r2;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: f64 = compile_and_run(prog.to_string(), &mut main);
    assert!(approx_equal(res, 12.48f64, 4));
}

#[test]
fn division_lint_type() {
    let prog = "
    FUNCTION main : LINT
        main := 4611686018427387903 * 2;
    END_FUNCTION
    ";

    let res: i64 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 9223372036854775806)
}

#[test]
fn division_sint_type() {
    let prog = "
    FUNCTION main : SINT
        main := 42 * 3;
    END_FUNCTION
    ";

    let res: i8 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 126)
}

#[test]
fn multiplication_ulint_type() {
    let prog = "
    FUNCTION main : ULINT
        main := 20000000000000 * 5;
    END_FUNCTION
    ";

    let res: i64 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 100000000000000)
}

#[test]
fn multiplication_udint_type() {
    let prog = "
    FUNCTION main : UDINT
        main := 1000083642 * 2;
    END_FUNCTION
    ";

    let res: i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 2000167284)
}

#[test]
fn multiplication_uint_type_basic() {
    let prog = "
    FUNCTION main : UINT
        main := 16005 * 2;
    END_FUNCTION
    ";

    let res: i16 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 32010)
}

#[test]
fn multiplication_usint_type() {
    let prog = "
    FUNCTION main : USINT
        main := 56 * 2;
    END_FUNCTION
    ";

    let res: i8 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 112)
}

#[test]
fn multiplication_time_basic() {
    let prog = "
    FUNCTION main : TIME
    VAR
        time_var : TIME := T#5s;
    END_VAR
        main := time_var * 2;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 10000000000);
}

#[test]
fn multiplication_date_basic() {
    let prog = "
    FUNCTION main : DATE
    VAR
        date_var : DATE := D#2021-01-01;
        date_10_days : DATE := 777600000000000;
        result,mul_result : DATE;
    END_VAR
        mul_result := date_10_days * 2;
        result := date_var + mul_result;
        main := result;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: u64 = compile_and_run(prog.to_string(), &mut main);
    let date_var = chrono::Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap().timestamp_nanos() as u64;
    let date_10_days = chrono::Utc.with_ymd_and_hms(1970, 1, 10, 0, 0, 0).unwrap().timestamp_nanos() as u64;
    assert_eq!(res, date_var + date_10_days * 2);
}

#[test]
fn multiplication_dt_type_basic() {
    let prog = "
    FUNCTION main : DT
    VAR
        i3 : TIME := T#25s;
    END_VAR
        main := i3 * 2;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 50000000000);
}

#[test]
fn multiplication_tod_type_basic() {
    let prog = "
    FUNCTION main : TOD
    VAR
        i3 : TIME := T#25s;
    END_VAR
        main := i3 * 2;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 50000000000);
}

#[test]
fn multiplication_array_basic() {
    let prog = "
    FUNCTION main : DINT
    VAR
        myArr : ARRAY[1..20] OF INT;
    END_VAR
        myArr[20] := 20;
        main := myArr[20] * 3;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i32 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 60);
}

//-----------------------------

fn approx_equal<T: Float>(a: T, b: T, decimal_places: u16) -> bool {
    let factor: T = NumCast::from(10.0.powi(decimal_places as i32)).unwrap();
    let a = (a * factor).round();
    let b = (b * factor).round();
    a == b
}
