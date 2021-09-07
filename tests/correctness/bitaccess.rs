// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::super::*;

#[allow(dead_code)]
#[repr(C)]
#[derive(Default, PartialEq, Debug)]
struct MainType {
    variable : u64,
    bit_target : bool,
    bit_target2 : bool,
    byte_target : u8,
    word_target : u16,
    dword_target : u32,
}

#[test]
fn bitaccess_test() {
    let prog = "
    FUNCTION main : DINT
    VAR 
        variable    : LWORD; 
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

    compile_and_run(prog.to_string(), &mut main_type);
    assert_eq!(main_type, MainType {
        variable : 0xAB_CD_EF_12_34_56_78_90,
        bit_target: true,
        byte_target: 0xAB,
        word_target: 0xAB_CD,
        dword_target: 0xAB_CD_EF_12,
        bit_target2: true,
    })

}