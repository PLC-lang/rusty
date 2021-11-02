// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::super::*;

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
    FUNCTION main : BYTE
    VAR
        a : BYTE := 0;
        b : WORD := 0;
        c : DWORD := 0;
        d : LWORD := 0;
    END_VAR
    a.1 := TRUE; //2#0000_0010
    a.%X0 := TRUE; //2#0000_0011
    b.%B0 := a;
    c.%W0 := b;
    d.%D0 := c;
    main := a;
    END_FUNCTION";

    struct Type {}

    let res :u8 = compile_and_run(prog.to_string(), &mut Type{});
    assert_eq!(3,res);
}

#[test]
fn bitaccess_test() {
    let prog = "
    FUNCTION main : DINT
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
    END_FUNCTION
    ";
    let mut main_type = MainType::default();

    compile_and_run::<_, i32>(prog.to_string(), &mut main_type);
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
    FUNCTION main : DINT
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
    END_FUNCTION
    ";
    let mut main_type = MainType::default();

    compile_and_run::<_, i32>(prog.to_string(), &mut main_type);
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
