use super::super::*;
#[allow(dead_code)]
#[repr(C)]
struct MainType {
    bool_1: bool,
    bool_2: bool,
    bool_3: bool,
    byte_1: u8,
    byte_2: u8,
    byte_3: u8,
    sint_1: i16,
    sint_2: i16,
    sint_3: i16,
    usint_1: u16,
    usint_2: u16,
    usint_3: u16,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    int_1: i32,
    int_2: i32,
    int_3: i32,
    uint_1: u32,
    uint_2: u32,
    uint_3: u32,
    dword_1: u64,
    dword_2: u64,
    dword_3: u64,
    dint_1: i64,
    dint_2: i64,
    dint_3: i64,
    udint_1: u64,
    udint_2: u64,
    udint_3: u64,
    lword_1: u128,
    lword_2: u128,
    lword_3: u128,
    lint_1: i128,
    lint_2: i128,
    lint_3: i128,
    ulint_1: u128,
    ulint_2: u128,
    ulint_3: u128,
}

fn new() -> MainType {
    MainType {
        bool_1: false,
        bool_2: false,
        bool_3: false,
        byte_1: 0,
        byte_2: 0,
        byte_3: 0,
        sint_1: 0,
        sint_2: 0,
        sint_3: 0,
        usint_1: 0,
        usint_2: 0,
        usint_3: 0,
        word_1: 0,
        word_2: 0,
        word_3: 0,
        int_1: 0,
        int_2: 0,
        int_3: 0,
        uint_1: 0,
        uint_2: 0,
        uint_3: 0,
        dword_1: 0,
        dword_2: 0,
        dword_3: 0,
        dint_1: 0,
        dint_2: 0,
        dint_3: 0,
        udint_1: 0,
        udint_2: 0,
        udint_3: 0,
        lword_1: 0,
        lword_2: 0,
        lword_3: 0,
        lint_1: 0,
        lint_2: 0,
        lint_3: 0,
        ulint_1: 0,
        ulint_2: 0,
        ulint_3: 0,
    }
}
#[test]
fn same_type_addition() {
    let function = r"
        PROGRAM main
        VAR
            bool_1   : BOOL;
            bool_2   : BOOL;
            bool_3   : BOOL;
            byte_1   : BYTE;
            byte_2   : BYTE;
            byte_3   : BYTE;
            sint_1   : SINT;
            sint_2   : SINT;
            sint_3   : SINT;
            usint_1  : USINT;
            usint_2  : USINT;
            usint_3  : USINT;
            word_1   : WORD;
            word_2   : WORD;
            word_3   : WORD;
            int_1    : INT;
            int_2    : INT;
            int_3    : INT;
            uint_1   : UINT;
            uint_2   : UINT;
            uint_3   : UINT;
            dword_1  : DWORD;
            dword_2  : DWORD;
            dword_3  : DWORD;
            dint_1   : DINT;
            dint_2   : DINT;
            dint_3   : DINT;
            udint_1 : UDINT;
            udint_2 : UDINT;
            udint_3 : UDINT;
            lword_1 : LWORD;
            lword_2 : LWORD;
            lword_3 : LWORD;
            lint_1  : LINT;
            lint_2  : LINT;
            lint_3  : LINT;
            ulint_1 : ULINT;
            ulint_2 : ULINT;
            ulint_3 : ULINT;
        END_VAR
            bool_1  := 0 + 0;
            bool_2  := 0 + 1;
            bool_3  := 1 + 1;
            byte_1  := 1 + 1;
            byte_2  := 0 + 0;
            byte_3  := 255 + 255;
            sint_1  := -3 + 1;
            sint_2  := -32767 + 2;
            sint_3  := -32767 - 10;
            usint_1 := 0 + 1;
            usint_2 := 0 + 0;
            usint_3 := 65535 + 10;
            word_1  := 0 + 1;
            word_2  := 4294967296 - 10;
            word_3  := 4294967296 + 10;
            int_1   := 0 + 10;
            int_2   := 2147483648 - 10;
            int_3   := 2147483647 + 10;
            uint_1  := 0 + 1;
            uint_2  := 4294967296 - 10;
            uint_3  := 4294967296 + 10;
            dword_1 := 65535 + 10;
            dint_1  := 65535 + 10;
            udint_1 := 65535 + 10;
            lword_1 := 65535 + 10;
            lint_1  := 65535 + 10;
            ulint_1 := 65535 + 10;


        END_PROGRAM
        ";

    let mut maintype = new();

    compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(false, maintype.bool_1);
    assert_eq!(true, maintype.bool_2);
    assert_eq!(false, maintype.bool_3); //Overflow
    
    assert_eq!(2,maintype.byte_1);
    assert_eq!(0,maintype.byte_2);
    assert_eq!(254,maintype.byte_3); //overflow
    
    assert_eq!(-2, maintype.sint_1);
    assert_eq!(-32765, maintype.sint_2);
    assert_eq!(32759, maintype.sint_3); //Overflow
    
    assert_eq!(1, maintype.usint_1);
    assert_eq!(0, maintype.usint_2);
    assert_eq!(9, maintype.usint_3); //Overflow
    
    assert_eq!(1,maintype.word_1);
    assert_eq!(4294967286,maintype.word_2);
    assert_eq!(10,maintype.word_3); //overflow
    
    assert_eq!(10,maintype.int_1);
    assert_eq!(2147483638,maintype.int_2);
    assert_eq!(-2147483639,maintype.int_3); //overflow
    
    assert_eq!(1,maintype.uint_1);
    assert_eq!(4294967286,maintype.uint_2);
    assert_eq!(10,maintype.uint_3); //overflow
    
    assert_eq!(65545,maintype.dword_1);
    assert_eq!(65545,maintype.dint_1);
    assert_eq!(65545,maintype.udint_1);
    assert_eq!(65545,maintype.lword_1);
    assert_eq!(65545,maintype.lint_1);
    assert_eq!(65545,maintype.ulint_1);
    
}

#[test]
#[ignore] //Does not work yet
fn mixed_type_addition() {
    let function = r"
        PROGRAM main
        VAR
            bool_1   : BOOL;
            bool_2   : BOOL;
            bool_3   : BOOL;
            byte_1   : BYTE;
            byte_2   : BYTE;
            byte_3   : BYTE;
            sint_1   : SINT;
            sint_2   : SINT;
            sint_3   : SINT;
            usint_1  : USINT;
            usint_2  : USINT;
            usint_3  : USINT;
            word_1   : WORD;
            word_2   : WORD;
            word_3   : WORD;
            int_1    : INT;
            int_2    : INT;
            int_3    : INT;
            uint_1   : UINT;
            uint_2   : UINT;
            uint_3   : UINT;
            dword_1  : DWORD;
            dword_2  : DWORD;
            dword_3  : DWORD;
            dint_1   : DINT;
            dint_2   : DINT;
            dint_3   : DINT;
            udint_1 : UDINT;
            udint_2 : UDINT;
            udint_3 : UDINT;
            lword_1 : LWORD;
            lword_2 : LWORD;
            lword_3 : LWORD;
            lint_1  : LINT;
            lint_2  : LINT;
            lint_3  : LINT;
            ulint_1 : ULINT;
            ulint_2 : ULINT;
            ulint_3 : ULINT;
        END_VAR
            bool_1  := 0 + 0;
            bool_2  := 10 + 1;
            bool_3  := 1 + TRUE;
            
            sint_1  := 50;
            byte_1  := 1 + 300;
            byte_2  := 300 + sint_1;
            byte_3  := 255 + bool_3;

            int_1 := 10;
            sint_1  := byte_1 + 5000;
            sint_2  := int_1 + 2;
            sint_3  := 65599 - 10;
            
            usint_1  := sint_1 + 5000;
            usint_2  := int_1 + 2;
            usint_3  := 65599 - 10;

            
            dword_1 := 1234; 
            word_1  := usint_1 + 1;
            word_2  := dword_1 + 10;
            
            int_1   := 0 + 10;
            int_2   := 2147483648 - 10;
            int_3   := 2147483647 + 10;

            uint_1  := 0 + 1;
            uint_2  := 4294967296 - 10;
            uint_3  := 4294967296 + 10;
           
            dint_1 := -10;
            lword_1 := 5;
            dword_1 := uint1 + 10;
            dword_2 := dint_1 + 10;
            dword_3 := lword1 + 10;
           
            udint_1 := 5;
            dint_1  := int + 10;
            dint_2  := udint_1 + 10;
            dint_3  := lword + 10;
            
            udint_1 := sint_1 + 10;
            udint_2 := dint_1 + 10;
            udint_3 := lword_1 + 10;
           
            lint_1 := 1234;
            lword_1 := udint_1 + 10;
            lword_2 := sint_1 + 10;
            lword_3 := lint_1 + 10;

            lint_1  := udint_1 + 10;
            lint_2  := int_1 + 10;
            lint_3  := lword_1 + 10;
            
            ulint_1 := udint_1 + 10;
            ulint_2 := lint_1 + 10;
            ulint_3 := sint_1 + 10;


        END_PROGRAM
        ";

    let mut maintype = new();

    compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(false, maintype.bool_1);
    assert_eq!(true, maintype.bool_2);
    assert_eq!(false, maintype.bool_3); //Overflow
    
    assert_eq!(2,maintype.byte_1);
    assert_eq!(0,maintype.byte_2);
    assert_eq!(254,maintype.byte_3); //overflow
    
    assert_eq!(-2, maintype.sint_1);
    assert_eq!(-32765, maintype.sint_2);
    assert_eq!(32759, maintype.sint_3); //Overflow
    
    assert_eq!(1, maintype.usint_1);
    assert_eq!(0, maintype.usint_2);
    assert_eq!(9, maintype.usint_3); //Overflow
    
    assert_eq!(1,maintype.word_1);
    assert_eq!(4294967286,maintype.word_2);
    assert_eq!(10,maintype.word_3); //overflow
    
    assert_eq!(10,maintype.int_1);
    assert_eq!(2147483638,maintype.int_2);
    assert_eq!(-2147483639,maintype.int_3); //overflow
    
    assert_eq!(1,maintype.uint_1);
    assert_eq!(4294967286,maintype.uint_2);
    assert_eq!(10,maintype.uint_3); //overflow
    
    assert_eq!(65545,maintype.dword_1);
    assert_eq!(65545,maintype.dint_1);
    assert_eq!(65545,maintype.udint_1);
    assert_eq!(65545,maintype.lword_1);
    assert_eq!(65545,maintype.lint_1);
    assert_eq!(65545,maintype.ulint_1);
    
}
