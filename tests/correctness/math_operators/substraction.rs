use crate::{compile_and_run, MainType};
use chrono::TimeZone;
use num::{Float, NumCast};

//--------------------------------------------------------------
//substraction tests

#[test]
fn subtraction_basic() {
    let prog = "
    FUNCTION main : DINT
        main := 30 - 5;
    END_FUNCTION
    ";

    let res: i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 25)
}

#[test]
fn subtraction_decimals() {
    let prog = "
    FUNCTION main : DINT
        main := (3.5 * 3) - 1.5;
    END_FUNCTION
    ";

    let res: i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 9)
}

#[test]
fn substraction_vars_real_type() {
    let prog = "
    FUNCTION main : DINT
    VAR
        x1 : REAL := 100.5;
        x2 : REAL := 110.6;
        r1, r2 : REAL;
    END_VAR
        main := x1 - x2;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: f32 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, -10.099998);
}

#[test]
fn substraction_multiple_vars_lreal_type() {
    let prog = "
    FUNCTION main : LREAL
    VAR
        r1 : LREAL := 20.8;
        r2 : LREAL := 0.2;
    END_VAR
        main := r1 - 2 - 5 - r2;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: f64 = compile_and_run(prog.to_string(), &mut main);
    assert!(approx_equal(res, 13.6f64, 5));
    //let expected : f64 = 20.8f64 + 2f64 + 5f64 + 0.2f64;
    //assert_eq!(res, expected);
}

#[test]
fn double_negative() {
    let prog = "
    FUNCTION main : DINT
        main := --5;
    END_FUNCTION
    ";

    let res: i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 5)
}

#[test]
fn double_negative_parenthesized() {
    let prog = "
    FUNCTION main : DINT
        main := -(-15);
    END_FUNCTION
    ";

    let res: i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 15)
}

#[test]
fn division_lint_type() {
    let prog = "
    FUNCTION main : LINT
        main := 9223372036854775809 - 2;
    END_FUNCTION
    ";

    let res: i64 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 9223372036854775807)
}

#[test]
fn division_sint_type() {
    let prog = "
    FUNCTION main : SINT
        main := 250 - 123;
    END_FUNCTION
    ";

    let res: i8 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 127)
}

#[test]
fn substraction_ulint_type() {
    let prog = "
    FUNCTION main : ULINT
        main := -2 - 3 + 5;
    END_FUNCTION
    ";

    let res: i64 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 0)
}

#[test]
fn substract_udint_type() {
    let prog = "
    FUNCTION main : UDINT
        main := 2000167484 - 200;
    END_FUNCTION
    ";

    let res: i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 2000167284)
}

#[test]
fn substract_uint_type_basic() {
    let prog = "
    FUNCTION main : UINT
        main := 42155 - 10001;
    END_FUNCTION
    ";

    let res: i16 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 32154)
}

#[test]
fn substract_usint_type() {
    let prog = "
    FUNCTION main : USINT
        main := 260 - 145;
    END_FUNCTION
    ";

    let res: i8 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 115)
}

#[test]
fn substract_time_basic() {
    let prog = "
    FUNCTION main : TIME
    VAR
        time_var : TIME := T#25s;
    END_VAR
        main := time_var - 10000000000;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 15000000000);
}

#[test]
fn substract_dt_type_basic() {
    let prog = "
    FUNCTION main : DT
    VAR
        i3 : TIME := T#25s;
    END_VAR
        main := i3 - 10000000000;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 15000000000);
}

#[test]
fn substract_tod_type_basic() {
    let prog = "
    FUNCTION main : TOD
    VAR
        i3 : TIME := T#25s;
    END_VAR
        main := i3 - 10000000000;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 15000000000);
}

#[test]
fn substract_date_basic() {
    let prog = "
    FUNCTION main : DATE
    VAR
        date_var : DATE := D#2021-01-01;
        date_temp : DATE := D#2021-01-10;
        result : DATE;
    END_VAR
        result := date_temp - date_var;
        main := result;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: u64 = compile_and_run(prog.to_string(), &mut main);
    let date_var = chrono::Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap().timestamp_nanos() as u64;
    let date_temp = chrono::Utc.with_ymd_and_hms(2021, 1, 10, 0, 0, 0).unwrap().timestamp_nanos() as u64;
    assert_eq!(res, date_temp - date_var);
}

#[test]
fn substract_array_basic() {
    let prog = "
    FUNCTION main : DINT
    VAR
        myArr : ARRAY[1..20] OF INT;
    END_VAR
        myArr[20] := 20;
        main := myArr[20] - 10;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i32 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 10);
}

//--------------------------

fn approx_equal<T: Float>(a: T, b: T, decimal_places: u16) -> bool {
    let factor: T = NumCast::from(10.0.powi(decimal_places as i32)).unwrap();
    let a = (a * factor).round();
    let b = (b * factor).round();
    a == b
}
