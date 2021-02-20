/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::super::*;
#[allow(dead_code)]
#[repr(C)]
struct MainType {
    x: i32,
    x_: i32,
    y: bool,
    y_: bool,
    z: f32,
    z_: f32,
}

struct ThreeInts {
    x: i32,
    y: i32,
    z: i32,
}

fn new() -> MainType {
    MainType {
        x: 0,
        x_: 0,
        y: false,
        y_: false,
        z: 0.0,
        z_: 0.0,
    }
}
#[test]
fn initia_values_of_programs_members() {
    let function = r"
        PROGRAM other
        VAR
            x   : DINT := 77;
            x_  : DINT;
            y   : BOOL := TRUE;
            y_  : BOOL;
            z   : REAL := 3.1415;
            z_  : REAL;
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            x : DINT;
            x_ : DINT;
            y : BOOL;
            y_ : BOOL;
            z : REAL;
            z_ : REAL;
        END_VAR
            x := other.x;
            x_ := other.x_;
            y := other.y;
            y_ := other.y_;
            z := other.z;
            z_ := other.z_;
        END_PROGRAM
        ";

    let mut maintype = new();

    compile_and_run(function.to_string(), &mut maintype);

        assert_eq!(77, maintype.x);
        assert_eq!(0, maintype.x_);
        assert_eq!(true, maintype.y);
        assert_eq!(false, maintype.y_);
        assert_eq!(3.1415, maintype.z);
        assert_eq!(0.0, maintype.z_);
}

#[test]
fn initia_values_of_functionblock_members() {
    let function = r"
        FUNCTION_BLOCK MyFB
        VAR
            x   : DINT := 77;
            x_  : DINT;
            y   : BOOL := TRUE;
            y_  : BOOL;
            z   : REAL := 3.1415;
            z_  : REAL;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM other
            VAR myFB: MyFB; END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            x : DINT;
            x_ : DINT;
            y : BOOL;
            y_ : BOOL;
            z : REAL;
            z_ : REAL;
        END_VAR
            x := other.myFB.x;
            x_ := other.myFB.x_;
            y := other.myFB.y;
            y_ := other.myFB.y_;
            z := other.myFB.z;
            z_ := other.myFB.z_;
        END_PROGRAM
        ";

    let mut maintype = new();

    compile_and_run(function.to_string(), &mut maintype);

        assert_eq!(77, maintype.x);
        assert_eq!(0, maintype.x_);
        assert_eq!(true, maintype.y);
        assert_eq!(false, maintype.y_);
        assert_eq!(3.1415, maintype.z);
        assert_eq!(0.0, maintype.z_);
}



#[test]
fn initia_values_of_function_members() {
    let function = r"
        FUNCTION other : DINT
        VAR
            x   : DINT := 77;
            y   : DINT := 88;
            z   : DINT := 99;
        END_VAR
        VAR_INPUT
            index : INT;
        END_VAR

            IF index = 0 THEN
                other := x;
            ELSIF index = 1 THEN
                other := y;
            ELSE
                other := z;
            END_IF
        END_FUNCTION

        PROGRAM main
        VAR
            x : DINT;
            y : DINT;
            z : DINT;
        END_VAR
            x := other(index := 0);
            y := other(index := 1);
            z := other(index := 2);
        END_PROGRAM
        ";

    let mut maintype = ThreeInts{ x:0, y:0, z:0 };

    compile_and_run(function.to_string(), &mut maintype);

        assert_eq!(77, maintype.x);
        assert_eq!(88, maintype.y);
        assert_eq!(99, maintype.z);
}

#[test]
fn initia_values_of_struct_type_members() {
    let function = r"
        TYPE MyStruct :
            STRUCT
                x   : DINT := 77;
                x_  : DINT;
                y   : BOOL := TRUE;
                y_  : BOOL;
                z   : REAL := 3.1415;
                z_  : REAL;
            END_STRUCT
        END_TYPE

        VAR_GLOBAL
            other: MyStruct;
        END_VAR

        PROGRAM main
        VAR
            x : DINT;
            x_ : DINT;
            y : BOOL;
            y_ : BOOL;
            z : REAL;
            z_ : REAL;
        END_VAR
            x := other.x;
            x_ := other.x_;
            y := other.y;
            y_ := other.y_;
            z := other.z;
            z_ := other.z_;
        END_PROGRAM
        ";

    let mut maintype = new();

    compile_and_run(function.to_string(), &mut maintype);

        assert_eq!(77, maintype.x);
        assert_eq!(0, maintype.x_);
        assert_eq!(true, maintype.y);
        assert_eq!(false, maintype.y_);
        assert_eq!(3.1415, maintype.z);
        assert_eq!(0.0, maintype.z_);
}

#[test]
fn initia_values_of_alias_type() {
    let function = r"
        TYPE MyInt  : DINT := 7; END_TYPE
        TYPE MyBool : BOOL := TRUE; END_TYPE
        TYPE MyReal : REAL := 5.67; END_TYPE

        VAR_GLOBAL
            gx   : MyInt;
            gxx  : MyInt := 8;
            gy   : MyBool;
            gyy   : MyBool := FALSE;
            gz   : MyReal;
            gzz  : MyReal := 1.23;
        END_VAR

        PROGRAM main
        VAR
            x : DINT;
            x_ : DINT;
            y : BOOL;
            y_ : BOOL;
            z : REAL;
            z_ : REAL;
        END_VAR
            x := gx;
            x_ := gxx;
            y := gy;
            y_ := gyy;
            z := gz;
            z_ := gzz;
        END_PROGRAM
        ";

    let mut maintype = new();

    compile_and_run(function.to_string(), &mut maintype);

        assert_eq!(7, maintype.x);
        assert_eq!(8, maintype.x_);
        assert_eq!(true, maintype.y);
        assert_eq!(false, maintype.y_);
        assert_eq!(5.67, maintype.z);
        assert_eq!(1.23, maintype.z_);
}