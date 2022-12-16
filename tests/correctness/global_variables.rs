// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::super::*;

#[allow(dead_code)]
#[repr(C)]
struct MainType {
    x: i32,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
struct MainGlobalsType {
    x: i16,
    y: bool,
    z: f32,
}

#[test]
fn global_variable_can_be_referenced_in_fn() {
    let function = r"
    VAR_GLOBAL
        gX : INT;
    END_VAR
    FUNCTION main : DINT
    VAR
        x : INT;
    END_VAR

    x := 10;
    gX := 20;

    gX := x + gX;

    main := gX;
    END_FUNCTION
    ";
    let res: i32 = compile_and_run(function.to_string(), &mut MainType { x: 0 });
    assert_eq!(res, 30);
}

#[test]
fn global_variable_can_be_referenced_in_two_functions() {
    let function = r"
    VAR_GLOBAL
        gX : INT;
    END_VAR
    FUNCTION main : DINT
    VAR
        x : INT;
    END_VAR

    x := 10;
    gX := 20;

    gX := x + gX;

    main := gX;
    END_FUNCTION

    FUNCTION two : DINT
    two := gX;
    END_FUNCTION
    ";
    let context = inkwell::context::Context::create();
    let exec_engine = compile(&context, function.to_string());

    let res: i32 = run(&exec_engine, "main", &mut MainType { x: 0 });
    assert_eq!(res, 30);
    let res2: i32 = run(&exec_engine, "two", &mut MainType { x: 0 });
    assert_eq!(res2, 30)
}

#[test]
fn global_variables_with_initialization() {
    let function = r"
    VAR_GLOBAL CONSTANT
        c_X : INT   := 77;
        c_Y : BOOL  := TRUE;
        c_Z : REAL  := 9.1415;
    END_VAR

    VAR_GLOBAL
        gX : INT := c_X;
        gY : BOOL := c_Y;
        gZ : REAL := c_Z;
    END_VAR
    PROGRAM main
        VAR
            x : INT;
            y : BOOL;
            z : REAL;
        END_VAR
        x := gX;
        y := gY;
        z := gZ;
    END_PROGRAM
    ";
    let context = inkwell::context::Context::create();
    let exec_engine = compile(&context, function.to_string());

    let mut params = MainGlobalsType { x: 0, y: false, z: 0.0 };
    run::<_, i32>(&exec_engine, "main", &mut params);
    assert_eq!(params, MainGlobalsType { x: 77, y: true, z: 9.1415 });
}

#[test]
fn uninitialized_global_array() {
    let function = r"
        VAR_GLOBAL
            gX : ARRAY[0..2] OF INT;  /* this should be zero-initialized */
            gZ : INT;
        END_VAR
        FUNCTION main : REAL
            VAR
                x,y : INT;
                z : INT;
            END_VAR
            gX[0] := 10;
            gX[1] := 21;
            gZ := 5;
            x := gX[0];
            y := gX[1];
            z := gZ;
            main := (x + y) / z;
        END_FUNCTION
    ";

    struct MainType {}
    let mut maintype = MainType {};
    let res: f32 = compile_and_run(function.to_string(), &mut maintype);
    assert!((res - 31f32 / 5f32) <= f32::EPSILON);
}

#[test]
fn uninitialized_global_struct() {
    let function = r"
        TYPE Point : STRUCT
            x : INT;
            y : INT;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL
            gX : Point; /* this should be zero-initialized */
            gZ : INT;
        END_VAR
        FUNCTION main : REAL
            VAR
                x,y : INT;
                z : INT;
            END_VAR
            gX.x := 10;
            gX.y := 21;
            gZ := 5;
            x := gX.x;
            y := gX.y;
            z := gZ;
            main := (x + y) / z;
        END_FUNCTION
    ";

    struct MainType {}
    let mut maintype = MainType {};
    let res: f32 = compile_and_run(function.to_string(), &mut maintype);
    assert!((res - 31f32 / 5f32) <= f32::EPSILON);
}
