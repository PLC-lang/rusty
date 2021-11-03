// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::super::*;

#[test]
fn fb_vars_can_be_accessed() {
    let src = "
FUNCTION main : DINT 
    prg();
    main := prg.y;
END_FUNCTION

PROGRAM prg
    VAR x : myFB; END_VAR
    VAR_OUTPUT y : DINT; END_VAR
    y := x.test(32);
END_PROGRAM

FUNCTION_BLOCK myFB
VAR
    x : INT := 10;
END_VAR
METHOD test : DINT
VAR_INPUT a : DINT; END_VAR
x := a;
test := x;
END_METHOD
END_FUNCTION_BLOCK
    ";

    struct Main {}

    let res: i32 = compile_and_run(src.into(), &mut Main {});
    //Expecting it not to fail
    assert_eq!(res, 42);
}

#[test]
fn class_vars_can_be_accessed() {
    let src = "
FUNCTION main : DINT 
    prg();
    main := prg.y;
END_FUNCTION

PROGRAM prg
    VAR x : myClass; END_VAR
    VAR_OUTPUT y : DINT; END_VAR
    y := x.test(32);
END_PROGRAM

CLASS myClass
VAR
    x : INT := 10;
END_VAR
METHOD test : DINT
VAR_INPUT a : DINT; END_VAR
x := a;
test := x;
END_METHOD
END_CLASS
    ";

    struct Main {}

    let res: i32 = compile_and_run(src.into(), &mut Main {});
    //Expecting it not to fail
    assert_eq!(res, 42);
}
