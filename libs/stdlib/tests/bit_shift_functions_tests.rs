// Import common functionality into the integration tests
mod common;

use common::add_std;
use plc_source::SourceCode;

use crate::common::compile_and_run;

#[derive(Default, Debug)]
#[repr(C)]
struct MainType {
    byte: u8,
    word: u16,
    dword: u32,
    lword: u64,
}

#[test]
fn shift_left_test() {
    let src = "
        PROGRAM main
        VAR
           b : BYTE;
           w : WORD;
           d : DWORD;
           l : LWORD;
        END_VAR
        b := SHL(BYTE#2#0001_1001,3);
        w := SHL(WORD#2#0001_1001,11);
        d := SHL(DWORD#2#0001_1001,27);
        l := SHL(LWORD#2#0001_1001,59);
        END_PROGRAM
        ";
    let sources = SourceCode::new(src, "main.st");
    let mut maintype = MainType::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.byte, 0b1100_1000);
    assert_eq!(maintype.word, 0b1100_1000_0000_0000);
    assert_eq!(maintype.dword, 0b1100_1000_0000_0000_0000_0000_0000_0000);
    assert_eq!(
        maintype.lword,
        0b1100_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000
    );
}

#[test]
fn shift_right_test() {
    let src = "
        PROGRAM main
        VAR
           b : BYTE;
           w : WORD;
           d : DWORD;
           l : LWORD;
        END_VAR
        b := SHR(BYTE#16#11,3);
        w := SHR(WORD#16#101,3);
        d := SHR(DWORD#16#1_0001,3);
        l := SHR(LWORD#16#1_0000_0000_0001,3);
        END_PROGRAM
        ";
    let sources = SourceCode::new(src, "main.st");
    let mut maintype = MainType::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.byte, 0x2);
    assert_eq!(maintype.word, 0x20);
    assert_eq!(maintype.dword, 0x2000);
    assert_eq!(maintype.lword, 0x2000_0000_0000);
}

#[test]
fn rotate_left_test() {
    let src = "
        PROGRAM main
        VAR
           b : BYTE;
           w : WORD;
           d : DWORD;
           l : LWORD;
        END_VAR
        b := ROL(BYTE#16#81,3);
        w := ROL(WORD#16#8001,3);
        d := ROL(DWORD#16#8000_0001,3);
        l := ROL(LWORD#16#8000_0000_0000_0001,3);
        END_PROGRAM
        ";
    let sources = add_std!(src, "bit_shift_functions.st");
    let mut maintype = MainType::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.byte, 0xC);
    assert_eq!(maintype.word, 0xC);
    assert_eq!(maintype.dword, 0xC);
    assert_eq!(maintype.lword, 0xC);
}

#[test]
fn rotate_right_test() {
    let src = "
        PROGRAM main
        VAR
           b : BYTE;
           w : WORD;
           d : DWORD;
           l : LWORD;
        END_VAR
        b := ROR(BYTE#16#81,3);
        w := ROR(WORD#16#8001,3);
        d := ROR(DWORD#16#8000_0001,3);
        l := ROR(LWORD#16#8000_0000_0000_0001,3);
        END_PROGRAM
        ";
    let sources = add_std!(src, "bit_shift_functions.st");
    let mut maintype = MainType::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.byte, 0x30);
    assert_eq!(maintype.word, 0x3000);
    assert_eq!(maintype.dword, 0x3000_0000);
    assert_eq!(maintype.lword, 0x3000_0000_0000_0000);
}
