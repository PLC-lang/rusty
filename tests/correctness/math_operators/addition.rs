use crate::{compile_and_run, MainType};
use num::{Float, NumCast};

//addition tests

#[test]
fn adds_in_result_dint_type() {
    #[allow(dead_code)]
    #[derive(Default)]
    #[repr(C)]
    struct MainType {
        di1: i32,
        di2: i32,
    }

    let prog = "
    FUNCTION foo : DINT
        foo := 10 + 50;
    END_FUNCTION

    PROGRAM main 
    VAR i1, i2: DINT; END_VAR

    i1 := 22 + 18;
    i2 := foo();

    END_PROGRAM
    ";

    let mut main = MainType::default();
    let _: i32 = compile_and_run(prog.to_string(), &mut main); //&mut main
    assert_eq!(60, main.di2);
    assert_eq!(40, main.di1);
}

#[test]
fn adds_multiple_w_vars_dint_type() {
    let prog = "
    FUNCTION main : DINT
    VAR
        di1 : DINT;
        x1 : INT := 200;
        x2 : INT := 20;
    END_VAR
        main := (10*x2) + 50 + 30 + x1;
    END_FUNCTION
    ";

    let res: i32 = compile_and_run(prog.to_string(), &mut MainType::default());

    assert_eq!(res, 480);
}

#[test]
fn adds_multiple_w_vars_real_type() {
    let prog = "
    FUNCTION main : REAL
    VAR
        r1 : REAL := 20.8;
        r2 : REAL := 0.2;
    END_VAR
        main := r1 + 2 + 5 + r2;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: f32 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 28.0);
}

#[test]
fn adds_multiple_vars_lreal_type() {
    let prog = "
    FUNCTION main : LREAL
    VAR
        x1 : LREAL := 20.8;
        x2 : LREAL := 0.2;
    END_VAR
        main := x1 + 2 + 5 + x2;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: f64 = compile_and_run(prog.to_string(), &mut main);
    assert!(approx_equal(res, 28.0f64, 5));
}

#[test]
fn store_decimal_addition_in_int() {
    let prog = "
    FUNCTION main : DINT
        main := 1.1 + 5.3 + 3.4 + 2.2;
    END_FUNCTION
    ";

    let res: i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 12)
}

#[test]
fn adds_lint_type() {
    let prog = "
    FUNCTION main : LINT
        main := 9223372036854775806 + 1;
    END_FUNCTION
    ";

    let res: i64 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 9223372036854775807)
}

#[test]
fn adds_sint_type() {
    let prog = "
    FUNCTION main : SINT
        main := 126 + 1;
    END_FUNCTION
    ";

    let res: i8 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 127)
}

#[test]
fn adds_ulint_type() {
    let prog = "
    FUNCTION main : ULINT
        main := 34559834657 + 283756423;
    END_FUNCTION
    ";

    let res: u64 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 34843591080)
}

#[test]
fn adds_udint_type() {
    let prog = "
    FUNCTION main : UDINT
        main := 2147483642 + 5;
    END_FUNCTION
    ";

    let res: u32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 2147483647)
}

#[test]
fn adds_uint_type() {
    let prog = "
    FUNCTION main : UINT
        main := 32760 + 7;
    END_FUNCTION
    ";

    let res: u16 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 32767)
}

#[test]
fn adds_usint_type() {
    let prog = "
    FUNCTION main : USINT
        main := 101 + 26;
    END_FUNCTION
    ";

    let res: u8 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 127)
}

#[test]
fn adds_time_basic() {
    let prog = "
    FUNCTION main : TIME
    VAR
        time_var : TIME := T#25s;
    END_VAR
        main := time_var + 10;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 25000000010);
}

#[test]
fn adds_dt_type_basic() {
    let prog = "
    FUNCTION main : DT
    VAR
        i3 : DT := T#25s;
    END_VAR
        main := i3 + 10;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 25000000010);
}

#[test]
fn adds_tod_type_basic() {
    #[allow(dead_code)]
    #[derive(Default)]
    #[repr(C)]
    struct MainType {
        i3: i32,
    }
    let prog = "
    FUNCTION main : TOD
    VAR
        i3 : TOD := T#25s;
    END_VAR
        main := i3 + 10;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 25000000010);
}

#[test]
fn add_date_basic() {
    let prog = "
    FUNCTION main : DATE
    VAR
        date_var : DATE := D#2021-01-01;
        date_10_days : DATE := 777600000;
        result : DATE;
    END_VAR
        result := date_10_days + date_var;
        main := result;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: u64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 1610236800000);
}

#[test]
fn adds_array_basic() {
    let prog = "
    FUNCTION main : DINT
    VAR
        int_array : ARRAY[1..20] OF INT;
        my_arr2 : ARRAY[0..8] OF INT := [1,2,3,4,5,6,7,8,9];
    END_VAR
        int_array[20] := 20;
        //           20       +    6       + 10 == 36
        main := int_array[20] + my_arr2[5] + 10;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i32 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 36);
}

//--------------------------

fn approx_equal<T: Float>(a: T, b: T, decimal_places: u16) -> bool {
    let factor: T = NumCast::from(10.0.powi(decimal_places as i32)).unwrap();
    let a = (a * factor).round();
    let b = (b * factor).round();
    a == b
}
