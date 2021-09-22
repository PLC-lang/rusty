// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::super::*;
use num::{Float, NumCast};


#[derive(Default)]
#[allow(dead_code)]
#[repr(C)]
struct MainType {
    di1 : i32,
    i1 : i16,
    i2 : i16,
    r1 : f32,
    r2 : f32,
    t1 : i64,
}

/*
"
FUNCTION main : DINT
VAR
    di1 : DINT;
    i1 : INT;
    i2 : INT;
    r1 : INT;
    r2 : INT;
    t1 : TIME;
END_VAR
END_FUNCTION
"
*/


//addition tests

#[test]
fn adds_in_result_dint_type() {
    let prog = "
    FUNCTION main : DINT
    VAR
        i1 : DINT;
    END_VAR
        main := 10 + 50;
        i1 := 22 + 18;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res : i32 = compile_and_run(prog.to_string(), &mut main); //&mut main
    assert_eq!(res, 60);

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
        r1, r2 : REAL;
    END_VAR
        main := (10*x2) + 50 + 30 + x1;
    END_FUNCTION
    ";

    let res : i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    
    assert_eq!(res, 480);
}


#[test]
fn adds_multiple_w_vars_real_type() {
    let prog = "
    FUNCTION main : REAL
    VAR
        x1 : REAL := 20.8;
        x2 : REAL := 0.2;
        r1, r2 : REAL;
    END_VAR
        main := x1 + 2 + 5 + x2;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res : f32 = compile_and_run(prog.to_string(), &mut main);
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

    let res : f64 = compile_and_run(prog.to_string(), &mut main);
    assert!(approx_equal(res,28.0f64,5));
    //let expected : f64 = 20.8f64 + 2f64 + 5f64 + 0.2f64;
    //assert_eq!(res, expected);
    
}


#[test]
fn adds_w_decimals() {
    let prog = "
    FUNCTION main : DINT
        main := 1.1 + 5.3 + 3.4 + 2.2;
    END_FUNCTION
    ";

    let res : i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 12)
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

    let res : i64 = compile_and_run(prog.to_string(), &mut main);
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

    let res : u64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 1610236800000);
}


#[test]
fn adds_array_basic() {
    let prog = "
    FUNCTION main : DINT
    VAR
        myArr : ARRAY[1..20] OF INT;
        myArr2 : ARRAY[0..8] OF INT := [1,2,3,4,5,6,7,8,9];
    END_VAR
        myArr[20] := 20;
        main := myArr[20] + myArr2[5] + 10;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res : i32 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 35);
}



//--------------------------------------------------------------
//substraction tests

#[test]
fn subtraction_basic() {
    let prog = "
    FUNCTION main : DINT
        main := 30 - 5;
    END_FUNCTION
    ";

    let res : i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 25)
}


#[test]
fn subtraction_decimals() {
    let prog = "
    FUNCTION main : DINT
        main := (3.5 * 3) - 1.5;
    END_FUNCTION
    ";

    let res : i32 = compile_and_run(prog.to_string(), &mut MainType::default());
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

    let res : f32 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, -10.099998);
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

    let res : i64 = compile_and_run(prog.to_string(), &mut main);
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

    let res : u64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 777600000);
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

    let res : i32 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 10);
}


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

    let res : i32 = compile_and_run(prog.to_string(), &mut MainType::default());
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

    let res : i32 = compile_and_run(prog.to_string(), &mut MainType::default());
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

    let res : f32 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 35.4);
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

    let res : i64 = compile_and_run(prog.to_string(), &mut main);
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

    let res : u64 = compile_and_run(prog.to_string(), &mut main);
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

    let res : i32 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 2);
}



//--------------------------------------------------------------
//multiplication tests

#[test]
fn multiplication_basic() {
    let prog = "
    FUNCTION main : DINT
        main := 6 * 100;
    END_FUNCTION
    ";

    let res : i32 = compile_and_run(prog.to_string(), &mut MainType::default());
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

    let res : f32 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 100.6);
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

    let res : i64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 10000000000);
}



#[test]
fn multiplication_date_basic() {
    let prog = "
    FUNCTION main : DATE
    VAR
        date_var : DATE := D#2021-01-01;
        date_10_days : DATE := 777600000;
        result,div_result : DATE;
    END_VAR
        div_result := date_10_days * 2;
        result := date_var + div_result;
        main := result;
    END_FUNCTION
    ";

    let mut main = MainType::default();

    let res : u64 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 1611014400000);
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

    let res : i32 = compile_and_run(prog.to_string(), &mut main);
    assert_eq!(res, 60);
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

    let res : i32 = compile_and_run(prog.to_string(), &mut MainType::default());
    assert_eq!(res, 0)
}



#[test]
fn math_mixed_decimals() {
    let prog = "
    FUNCTION main : REAL
    VAR
        di : DINT;
        i1, i2 : INT;
        x1 : REAL := 1.2;
        x2 : REAL := 5.5;
    END_VAR
        main := (6 * x2) + (x1 / 2) - 0.2 + (x2 / 2) - 1.15;
    END_FUNCTION
    ";
    let mut main = MainType::default();

    let res : f32 = compile_and_run(prog.to_string(), &mut main);
    assert!(approx_equal(res,35.0f32,4));
    let expected : f32 = (6f32 * 5.5f32) + (1.2f32 / 2f32) - 0.2 + (5.5f32 / 2f32) - 1.15;
    assert_eq!(res, expected);
    print!("{}",res);
}


fn approx_equal<T : Float>(a: T, b: T, decimal_places: u16) -> bool {
    let factor : T = NumCast::from(10.0.powi(decimal_places as i32)).unwrap();
    let a = (a * factor).round();
    let b = (b * factor).round();
    a == b
}