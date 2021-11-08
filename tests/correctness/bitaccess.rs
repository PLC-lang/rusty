use pretty_assertions::assert_eq;

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
    FUNCTION main : INT
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
    END_FUNCTION";

    #[allow(dead_code)]
    #[repr(C)]
    #[derive(Default, Debug)]
    struct Type {
        a: u8,
        b: u16,
        c: u32,
        d: u64,
    }
    let mut param = Type::default();

    compile_and_run::<_, i32>(prog.to_string(), &mut param);

    assert_eq!(0b0000_0010, param.a);
    assert_eq!(0b0000_0010_0000_0000, param.b);
    assert_eq!(0b0000_0010_0000_0000_0000_0000_0000_0000, param.c);
    assert_eq!(
        0b0000_0010_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        param.d
    );
}

#[test]
fn bitaccess_chained_assignment() {
    let prog = "
    FUNCTION main : LWORD
    VAR
        d : LWORD := 2#0000_0101_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
        two : INT := 2;
    END_VAR
    d.%D1.%W1.%B1.%X0 := FALSE;
    d.%D1.%W1.%B1.1 := TRUE;
    d.%D1.%W1.%B1.%Xtwo := FALSE;
    main := d;       //2#0000_0010_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000
    END_FUNCTION";

    struct Type {}

    let res: u64 = compile_and_run(prog.to_string(), &mut Type {});

    assert_eq!(
        0b0000_0010_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        res
    );
}

#[test]
fn qualified_reference_assignment() {
    let prog = "
        TYPE myStruct : STRUCT x : BYTE := 1; END_STRUCT END_TYPE

        FUNCTION main : BYTE
        VAR
            str : myStruct;
        END_VAR
        str.x.%X0 := FALSE;
        str.x.%X1 := TRUE;
        main := str.x;
        END_FUNCTION

        ";
    struct Type {}
    let res: u8 = compile_and_run(prog.to_string(), &mut Type {});
    assert_eq!(2, res);
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
