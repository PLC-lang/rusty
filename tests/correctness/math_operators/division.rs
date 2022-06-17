use crate::{compile_and_run, MainType};
use num::{Float, NumCast};

//--------------------------------------------------------------
//division tests

#[test]
fn int_division_in_result() {
    let prog = "
    FUNCTION main : DINT
        //        int division results in 3 * 100
        main := (10 / 3) * 100;
    END_FUNCTION
    ";

    let res: i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 300)
}

#[test]
fn real_division_in_result() {
    let prog = "
    FUNCTION main : DINT
        //        real division results in 3.3333.. * 100
        main := (REAL#10 / 3) * 100;
    END_FUNCTION
    ";

    let res: i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 333)
}

#[test]
fn subs_vars_real_type() {
    let prog = "
    FUNCTION main : DINT
    VAR
        x1 : REAL := 70.8;
        x2 : REAL := 2.0;
        r1, r2 : REAL;
    END_VAR
        main := x1 / x2;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: f32 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 35.4);
}

#[test]
fn division_multiple_vars_lreal_type() {
    let prog = "
    FUNCTION main : LREAL
    VAR
        r1 : LREAL := 20.8;
        r2 : LREAL := 0.2;
    END_VAR
        main := r1 / 3 / r2;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: f64 = compile_and_run(prog.to_string(), &mut main);
    assert!(approx_equal(res, 34.66666f64, 4));
}

#[test]
fn division_lint_type() {
    let prog = "
    FUNCTION main : LINT
        main := 9223372036854775807 / 2;
    END_FUNCTION
    ";

    let res: i64 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 4611686018427387903)
}

#[test]
fn division_sint_type() {
    let prog = "
    FUNCTION main : SINT
        main := 250 / 2;
    END_FUNCTION
    ";

    let res: i8 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 125)
}

#[test]
fn division_ulint_type() {
    let prog = "
    FUNCTION main : ULINT
        main := 2000000000000000 / 20;
    END_FUNCTION
    ";

    let res: i64 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 100000000000000)
}

#[test]
fn division_udint_type() {
    let prog = "
    FUNCTION main : UDINT
        main := 4147483642 / 2;
    END_FUNCTION
    ";

    let res: i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 2073741821)
}

#[test]
fn division_uint_type() {
    let prog = "
    FUNCTION main : UINT
        main := 64002 / 2;
    END_FUNCTION
    ";

    let res: i16 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 32001)
}

#[test]
fn division_usint_type() {
    let prog = "
    FUNCTION main : USINT
        main := 250 / 2;
    END_FUNCTION
    ";

    let res: i8 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 125)
}

#[test]
fn division_time_basic() {
    let prog = "
    FUNCTION main : TIME
    VAR
        time_var : TIME := T#25s;
    END_VAR
        main := time_var / 2;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 12500000000);
}

#[test]
fn division_dt_type_basic() {
    let prog = "
    FUNCTION main : DT
    VAR
        i3 : TIME := T#25s;
    END_VAR
        main := i3 / 2;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 12500000000);
}

#[test]
fn division_tod_type_basic() {
    let prog = "
    FUNCTION main : TOD
    VAR
        i3 : TIME := T#25s;
    END_VAR
        main := i3 / 2;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 12500000000);
}

#[test]
fn division_date_basic() {
    let prog = "
    FUNCTION main : DATE
    VAR
        date_var : DATE := D#2021-01-01;
        date_10_days : DATE := 777600000;
        result,div_result : DATE;
    END_VAR
        div_result := date_10_days / 2;
        result := date_var + div_result;
        main := result;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: u64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 1609848000000);
}

#[test]
fn division_array_basic() {
    let prog = "
    FUNCTION main : DINT
    VAR
        myArr : ARRAY[1..20] OF INT;
    END_VAR
        myArr[20] := 20;
        main := myArr[20] / 10;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res: i32 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 2);
}

#[test]
fn real_division_by_zero() {
    #[derive(Debug, PartialEq)]
    struct MainType {
        r: f64,
        z: f64,
    }

    let prog = "
    PROGRAM main
        VAR
            r : LREAL;
            rZero: LREAL;
        END_VAR
        r := (1.0 / rZero);
    END_PROGRAM
    ";

    let mut main = MainType { r: 0.0, z: 0.0 };

    let _: i32 = compile_and_run(prog.to_string(), &mut main);
    assert!(main.r.is_infinite());
}

//--------------------------

fn approx_equal<T: Float>(a: T, b: T, decimal_places: u16) -> bool {
    let factor: T = NumCast::from(10.0.powi(decimal_places as i32)).unwrap();
    let a = (a * factor).round();
    let b = (b * factor).round();
    a == b
}
