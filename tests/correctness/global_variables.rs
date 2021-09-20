// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::super::*;

#[allow(dead_code)]
#[repr(C)]
struct MainType {
    x: i32,
    ret: i32,
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
    let (res, _) = compile_and_run(function.to_string(), &mut MainType { x: 0, ret: 0 });
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

    let (res, _) = run(&exec_engine, "main", &mut MainType { x: 0, ret: 0 });
    assert_eq!(res, 30);
    let (res2, _) = run(&exec_engine, "two", &mut MainType { x: 0, ret: 0 });
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

    let mut params = MainGlobalsType {
        x: 0,
        y: false,
        z: 0.0,
    };
    run(&exec_engine, "main", &mut params);
    assert_eq!(
        params,
        MainGlobalsType {
            x: 77,
            y: true,
            z: 9.1415,
        }
    );
}
