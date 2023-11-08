// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::*;
use pretty_assertions::assert_eq;

#[allow(dead_code)]
#[repr(C)]
#[derive(Default, PartialEq, Debug)]
struct MainType {
    variable: u64,
    access_var: i16,
    bit_target: bool,
    bit_target2: bool,
    byte_target: u8,
    word_target: u16,
    dword_target: u32,
}

#[test]
fn bitaccess_assignment() {
    let prog = "
    PROGRAM main 
    VAR
        a : BYTE := 2#0000_0101;
        b : WORD := 0;
        c : DWORD := 0;
        d : LWORD := 0;
        two : INT := 2;
    END_VAR
    a.%X0       := FALSE;   //2#0000_0100
    a.1         := TRUE;    //2#0000_0110
    a.%Xtwo     := FALSE;   //2#0000_0010
    b.%B1       := a;       //2#0000_0010_0000_0000
    c.%W1       := b;       //2#0000_0010_0000_0000_0000_0000_0000_0000
    d.%D1       := c;       //2#0000_0010_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000
    END_PROGRAM";

    #[allow(dead_code)]
    #[repr(C)]
    #[derive(Default, Debug)]
    struct Type {
        a: u8,
        b: u16,
        c: u32,
        d: u64,
        two: i16,
    }
    let mut param = Type::default();

    let _: i32 = compile_and_run(prog, &mut param);

    assert_eq!(0b0000_0010, param.a);
    assert_eq!(0b0000_0010_0000_0000, param.b);
    assert_eq!(0b0000_0010_0000_0000_0000_0000_0000_0000, param.c);
    assert_eq!(0b0000_0010_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000, param.d);
}

#[test]
fn bitaccess_chained_assignment() {
    let prog = "
    FUNCTION main: LWORD
    VAR
        d : LWORD := 2#0000_0101_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
        two : INT := 2;
    END_VAR
    d.%D1.%W1.%B1.%X0 := FALSE;
    d.%D1.%W1.%B1.1 := TRUE;
    d.%D1.%W1.%B1.%Xtwo := FALSE;
    main := d;       //2#0000_0010_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000
    END_FUNCTION";

    let res: u64 = compile_and_run(prog, &mut crate::MainType::default());

    assert_eq!(0b0000_0010_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000, res);
}

#[test]
fn qualified_reference_assignment() {
    let prog = "
        TYPE myStruct : STRUCT x : BYTE := 1; END_STRUCT END_TYPE

        FUNCTION main : BYTE
        VAR
            str : myStruct;
        END_VAR
        str.x := 1;
        str.x.%X0 := FALSE;
        str.x.%X1 := TRUE;
        main := str.x;
        END_FUNCTION

        ";

    let res: u8 = compile_and_run(prog, &mut crate::MainType::default);
    assert_eq!(2, res);
}

#[test]
fn bitaccess_test() {
    let prog = "
    PROGRAM main 
    VAR 
        variable    : LWORD; 
        access_var  : INT;
        bitTarget   : BOOL;
        bitTarget2  : BOOL;
        byteTarget  : BYTE;
        wordTarget  : WORD;
        dwordTarget : DWORD;
    END_VAR
    variable    := 16#AB_CD_EF_12_34_56_78_90;
    bitTarget   := variable.%X63; (*Access last bit*)
    byteTarget  := variable.%B7; (*Access last byte*)
    wordTarget  := variable.%W3; (*Access last word*)
    dwordTarget := variable.%D1; (*Access last dword*)
    (*Chaining an access is also allowed *)
    bitTarget2  := variable.%D1.%W1.%B1.%X1;
    END_PROGRAM
    ";
    let mut main_type = MainType::default();

    let _: i32 = compile_and_run(prog, &mut main_type);
    assert_eq!(
        main_type,
        MainType {
            variable: 0xAB_CD_EF_12_34_56_78_90,
            access_var: 0,
            bit_target: true,
            byte_target: 0xAB,
            word_target: 0xAB_CD,
            dword_target: 0xAB_CD_EF_12,
            bit_target2: true,
        }
    )
}

#[test]
fn bitaccess_with_var_test() {
    let prog = "
    PROGRAM main
    VAR 
        variable    : LWORD; 
        access_var  : INT;
        bitTarget   : BOOL;
        bitTarget2  : BOOL;
        byteTarget  : BYTE;
        wordTarget  : WORD;
        dwordTarget : DWORD;
    END_VAR
    variable    := 16#AB_CD_EF_12_34_56_78_90;
    access_var := 63;
    bitTarget   := variable.%Xaccess_var; (*Access last bit*)
    access_var := 7;
    byteTarget  := variable.%Baccess_var; (*Access last byte*)
    access_var := 3;
    wordTarget  := variable.%Waccess_var; (*Access last word*)
    access_var := 1;
    dwordTarget := variable.%Daccess_var; (*Access last dword*)
    (*Chaining an access is also allowed *)
    bitTarget2  := variable.%Daccess_var.%Waccess_var.%Baccess_var.%Xaccess_var;
    END_PROGRAM
    ";
    let mut main_type = MainType::default();

    let _: i32 = compile_and_run(prog, &mut main_type);
    assert_eq!(
        main_type,
        MainType {
            variable: 0xAB_CD_EF_12_34_56_78_90,
            access_var: 1,
            bit_target: true,
            byte_target: 0xAB,
            word_target: 0xAB_CD,
            dword_target: 0xAB_CD_EF_12,
            bit_target2: true,
        }
    )
}

#[test]
fn bitaccess_assignment_should_not_override_current_values() {
    let prog = "
    FUNCTION main : DINT
    VAR_TEMP
        a,b : BYTE := 0;
        c : BOOL := TRUE;
    END_VAR
    b := 1;
    a.%Xb := c;
    b := 0;
    a.%Xb := c;
    b := 2;
    a.%Xb := c;
    main := a;
    END_FUNCTION
    ";
    let res: i32 = compile_and_run(prog, &mut crate::MainType::default);
    assert_eq!(res, 7);
}

#[test]
fn byteaccess_assignment_should_not_override_current_values() {
    let prog = "
    FUNCTION main : DINT
    VAR_TEMP
        a : DINT := 0;
    END_VAR
    a.%B1 := 2#1010_1010;
    a.%B0 := 2#0101_0101;
    a.%B2 := 2#1100_0011;
    main := a;
    END_FUNCTION
    ";
    let res: i32 = compile_and_run(prog, &mut crate::MainType::default);
    assert_eq!(res, 0b0000_0000_1100_0011_1010_1010_0101_0101);
}
