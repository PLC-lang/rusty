use crate::compile_and_run;
use num::{Float, NumCast};

#[derive(Default)]
#[allow(dead_code)]
#[repr(C)]
struct MainType {
    di1: i32,
    i1: i16,
    i2: i16,
    r1: f32,
    r2: f32,
    t1: i64,
}

//--------------------------------------------------------------
//math mixed tests

#[test]
fn math_mixed() {
    let prog = "
    FUNCTION main : DINT
        main := (6 * 100) + (600 / 6) - 500 + (200 / 20) - 210;
    END_FUNCTION
    ";

    let res: i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 0)
}

#[test]
fn math_mixed_decimals() {
    let prog = "
    FUNCTION main : REAL
    VAR
        di : DINT;
        i1, i2 : INT;
        r1 : REAL := 1.2;
        r2 : REAL := 5.5;
    END_VAR
        main := (6 * r2) + (r1 / 2) - 0.2 + (r2 / 2) - 1.15;
    END_FUNCTION
    ";
    let mut main = MainType::default();

    let res: f32 = compile_and_run(prog.to_string(), &mut main);
    assert!(approx_equal(res, 35.0f32, 4));
    let expected: f32 = (6f32 * 5.5f32) + (1.2f32 / 2f32) - 0.2 + (5.5f32 / 2f32) - 1.15;
    assert_eq!(res, expected);
    print!("{}", res);
}

#[test]
fn mixed_math_lint_type() {
    let prog = "
    FUNCTION main : LINT
        main := 3948576349876 * 2 - 6548700 + 987077788 / 2;
    END_FUNCTION
    ";

    let res: i64 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 7897639689946)
}

#[test]
fn mixed_math_sint_type() {
    let prog = "
    FUNCTION main : SINT
        main := 50 * 2 + 27 - 8 + 16 / 2;
    END_FUNCTION
    ";

    let res: i8 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 127)
}

#[test]
fn mixed_math_ulint_type() {
    let prog = "
    FUNCTION main : ULINT
        main := 5555 * 222 + 27000 - 80 + 1600000000 / 2;
    END_FUNCTION
    ";

    let res: i64 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 801260130)
}

#[test]
fn mixed_math_udint_type() {
    let prog = "
    FUNCTION main : UDINT
        main := 5555 * 222 + 27000 - 80 + 2199999000 / 2;
    END_FUNCTION
    ";

    let res: i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 1101259630)
}

#[test]
fn mixed_math_uint_type() {
    let prog = "
    FUNCTION main : UINT
        main := 9867 * 10 + 870 - 95000 + 125 / 4;
    END_FUNCTION
    ";

    let res: i16 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 4571)
}

#[test]
fn mixed_math_usint_type() {
    let prog = "
    FUNCTION main : USINT
        main := 123 * 8 + 210 - 1115 + 126 / 6;
    END_FUNCTION
    ";

    let res: i8 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 100)
}

#[test]
fn mixed_math_time_basic() {
    let prog = "
    FUNCTION main : TIME
    VAR
        t1 : TIME := T#5s;
        time_var2 : TIME := T#6s;
        time_var3 : TIME := T#10s;
    END_VAR
        main := t1 + time_var2 * 3 / 2 - time_var3;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 4_000_000_000);
}

#[test]
fn mixed_math_tod_basic() {
    let prog = "
    FUNCTION main : TOD
    VAR
        t1 : TOD := T#5s;
        t2 : TOD := T#6s;
        t3 : TOD := T#10s;
    END_VAR
        main := t1 + t2 * 3 / 2 - t3;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 4000000000);
}

#[test]
fn mixed_math_date_basic() {
    let prog = "
    FUNCTION main : DATE
    VAR
        date_var : DATE := D#2021-01-01;
        date_10_days : DATE := 777600000;
        date_1_day : DATE := 86400;
        result : DATE;
    END_VAR
        result := date_var + date_10_days * 2 - date_1_day / 2;
        main := result;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: u64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 1611014356800);
}

#[test]
fn mixed_math_dt_basic() {
    let prog = "
    FUNCTION main : DT
    VAR
        date_var : DT := D#2021-01-01;
        date_10_days : DT := 777600000;
        date_1_day : DT := 86400;
        result : DT;
    END_VAR
        result := date_var + date_10_days * 2 - date_1_day / 2;
        main := result;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: u64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 1611014356800);
}

#[test]
fn lreal_mixed_decimals() {
    let prog = "
    FUNCTION main : LREAL
    VAR
        r1 : LREAL := 1.2;
        r2 : LREAL := 5.5;
    END_VAR
        main := (2 * r2) + (r1 / 3) - 0.8 + (r2 / 2) - 1.99;
    END_FUNCTION
    ";
    let mut main = MainType::default();

    let res: f64 = compile_and_run(prog.to_string(), &mut main);
    assert!(approx_equal(res, 11.36f64, 2));
}

#[test]
fn mixed_array_math_basic() {
    let prog = "
    FUNCTION main : DINT
    VAR
        int_array : ARRAY[1..10] OF INT;
    END_VAR
        int_array[3] := 20;
        int_array[4] := 11;
        int_array[5] := 15;
        int_array[6] := 60;
        int_array[7] := 3;
        main := int_array[3] + int_array[5] * int_array[4] - int_array[6] / int_array[7];
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i32 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 165);
}

//-----------------------------

fn approx_equal<T: Float>(a: T, b: T, decimal_places: u16) -> bool {
    let factor: T = NumCast::from(10.0.powi(decimal_places as i32)).unwrap();
    let a = (a * factor).round();
    let b = (b * factor).round();
    a == b
}
