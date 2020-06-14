use super::super::*;
#[allow(dead_code)]
#[repr(C)]
struct MainType {
    bool_1: bool,
    bool_2: bool,
    bool_3: bool,
    lword_1: u64,
    lint_1: i64,
    ulint_1: u64,
}

fn new() -> MainType {
    MainType {
        bool_1: false,
        bool_2: false,
        bool_3: false,
        lword_1: 0,
        lint_1: 0,
        ulint_1: 0,
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
            lword_1 : LWORD;
            lint_1  : LINT;
            ulint_1 : ULINT;
        END_VAR
            bool_1  := 0 + 0;
            bool_2  := 0 + 1;
            bool_3  := 1 + 1;
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

    assert_eq!(65545,maintype.lword_1);
    assert_eq!(65545,maintype.lint_1);
    assert_eq!(65545,maintype.ulint_1);
    
}

#[test]
fn byte_addition(){

    #[repr(C)]
    struct Type {
        byte_1 : u8,
        byte_2 : u8,
        byte_3 : u8,
    }

    let program = r#"
        PROGRAM main
        VAR
            byte_1 : BYTE;
            byte_2 : BYTE;
            byte_3 : BYTE;
        END_VAR
        byte_1  := 1 + 1;
        byte_2  := 0 + 0;
        byte_3  := 255 + 255;
        END_PROGRAM
        "#;
    
    let mut maintype = Type {
        byte_1 : 0,
        byte_2 : 0,
        byte_3 : 0,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(2,maintype.byte_1);
    assert_eq!(0,maintype.byte_2);
    assert_eq!(254,maintype.byte_3); //Overflow
}

#[test]
fn byte_mixed_addition(){

    #[repr(C)]
    struct Type {
        byte_1 : u8,
        sint_1 : i8,
        int_1 : i16,
    }

    let program = r#"
        PROGRAM main
        VAR
            byte_1 : BYTE;
            sint_1 : SINT;
            int_1 : INT;
        END_VAR
        byte_1  := sint_1 + int_1;
        END_PROGRAM
        "#;
    
    let mut maintype = Type {
        byte_1 : 0,
        sint_1 : -10,
        int_1 : 275,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(9,maintype.byte_1); //Overflow
    
    let mut maintype = Type {
        byte_1 : 0,
        sint_1 : 10,
        int_1 : 10,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(20,maintype.byte_1);
}

#[test]
fn usint_addition(){

    #[repr(C)]
    struct Type {
        usint_1 : u8,
        usint_2 : u8,
        usint_3 : u8,
    }

    let program = r#"
        PROGRAM main
        VAR
            usint_1 : USINT;
            usint_2 : USINT;
            usint_3 : USINT;
        END_VAR
        usint_1  := 1 + 1;
        usint_2  := 0 + 0;
        usint_3  := 255 + 255;
        END_PROGRAM
        "#;
    
    let mut maintype = Type {
        usint_1 : 0,
        usint_2 : 0,
        usint_3 : 0,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(2,maintype.usint_1);
    assert_eq!(0,maintype.usint_2);
    assert_eq!(254,maintype.usint_3); //Overflow
}

#[test]
fn usint_mixed_addition(){

    #[repr(C)]
    struct Type {
        usint_1 : u8,
        sint_1 : i8,
        int_1 : i16,
    }

    let program = r#"
        PROGRAM main
        VAR
            usint_1 : BYTE;
            sint_1 : SINT;
            int_1 : INT;
        END_VAR
        usint_1  := sint_1 + int_1;
        END_PROGRAM
        "#;
    
    let mut maintype = Type {
        usint_1 : 0,
        sint_1 : -10,
        int_1 : 275,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(9,maintype.usint_1); //Overflow
    
    let mut maintype = Type {
        usint_1 : 0,
        sint_1 : 10,
        int_1 : 10,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(20,maintype.usint_1);
}

#[test]
fn sint_additions(){

    #[repr(C)]
    struct Type {
        sint_1 : u8,
        sint_2 : u8,
        sint_3 : u8,
    }

    let program = r#"
        PROGRAM main
        VAR
            sint_1 : SINT;
            sint_2 : SINT;
            sint_3 : SINT;
        END_VAR
        sint_1  := 1 + 1;
        sint_2  := 0 + 0;
        sint_3  := -127 - 10;
        END_PROGRAM
        "#;
    
    let mut maintype = Type {
        sint_1 : 0,
        sint_2 : 0,
        sint_3 : 0,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(2,maintype.sint_1);
    assert_eq!(0,maintype.sint_2);
    assert_eq!(119,maintype.sint_3); //Overflow
}

#[test]
fn sint_mixed_addition(){

    #[repr(C)]
    struct Type {
        usint_1 : u8,
        sint_1 : i8,
        int_1 : i16,
    }

    let program = r#"
        PROGRAM main
        VAR
            usint_1 : BYTE;
            sint_1 : SINT;
            int_1 : INT;
        END_VAR
        sint_1  := usint_1 + int_1;
        END_PROGRAM
        "#;
    
    let mut maintype = Type {
        usint_1 : 250,
        sint_1 : 0,
        int_1 : 0,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(-6,maintype.sint_1);
    
    let mut maintype = Type {
        usint_1 : 10,
        sint_1 : 0,
        int_1 : 300,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(54,maintype.sint_1);
}

#[test]
fn word_addition(){

    #[repr(C)]
    struct Type {
        byte_1 : u16,
        byte_2 : u16,
        byte_3 : u16,
    }

    let program = r#"
        PROGRAM main
        VAR
            word_1 : WORD;
            word_2 : WORD;
            word_3 : WORD;
        END_VAR
            word_1  := 0 + 1;
            word_2  := 65535 - 10;
            word_3  := 65535 + 10;
        END_PROGRAM
        "#;
    
    let mut maintype = Type {
        byte_1 : 0,
        byte_2 : 0,
        byte_3 : 0,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(1, maintype.byte_1);
    assert_eq!(65525, maintype.byte_2);
    assert_eq!(9, maintype.byte_3); //Overflow
}

#[test]
fn word_mixed_addition(){

    #[repr(C)]
    struct Type {
        word_1 : u16,
        word_2 : u16,
        byte_1 : u8,
        int_1 : i16,
        dint_1 : i32,
    }

    let program = r#"
        PROGRAM main
        VAR
            word_1 : WORD;
            word_2 : WORD;
            byte_1 : BYTE;
            int_1  : INT;
            dint_1  : DINT;
        END_VAR
          word_1 := byte_1 + dint_1;
          word_2 := int_1 + dint_1;
        END_PROGRAM
        "#;
    
    let mut maintype = Type {
        word_1 : 0,
        word_2 : 0,
        byte_1 : 255,
        int_1  : -10,
        dint_1 : -0xFFFFFFF,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(256, maintype.word_1);
    assert_eq!(65527, maintype.word_2);
}

#[test]
fn int_addition(){

    #[repr(C)]
    struct Type {
        byte_1 : i16,
        byte_2 : i16,
        byte_3 : i16,
    }

    let program = r#"
        PROGRAM main
        VAR
            int_1 : INT;
            int_2 : INT;
            int_3 : INT;
        END_VAR
            int_1  := 0 + 1;
            int_2  := 65535 - 10;
            int_2  := -32767 + 10;
            int_3  := -32767 - 10;
        END_PROGRAM
        "#;
    
    let mut maintype = Type {
        byte_1 : 0,
        byte_2 : 0,
        byte_3 : 0,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(1, maintype.byte_1);
    assert_eq!(-32757, maintype.byte_2);
    assert_eq!(32759, maintype.byte_3); //Overflow
}

#[test]
fn uint_addition(){

    #[repr(C)]
    struct Type {
        byte_1 : u16,
        byte_2 : u16,
        byte_3 : u16,
    }

    let program = r#"
        PROGRAM main
        VAR
            uint_1 : UINT;
            uint_2 : UINT;
            uint_3 : UINT;
        END_VAR
            uint_1  := 0 + 1;
            uint_2  := 65535 - 10;
            uint_3  := 65535 + 10;
        END_PROGRAM
        "#;
    
    let mut maintype = Type {
        byte_1 : 0,
        byte_2 : 0,
        byte_3 : 0,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(1, maintype.byte_1);
    assert_eq!(65525, maintype.byte_2);
    assert_eq!(9, maintype.byte_3); //Overflow
}

#[test]
fn dword_addition(){

    #[repr(C)]
    struct Type {
        byte_1 : u32,
        byte_2 : u32,
        byte_3 : u32,
    }

    let program = r#"
        PROGRAM main
        VAR
            dword_1 : DWORD;
            dword_2 : DWORD;
            dword_3 : DWORD;
        END_VAR
            dword_1  := 0 + 1;
            dword_2  := 4294967296 - 10;
            dword_3  := 4294967296 + 10;
        END_PROGRAM
        "#;
    
    let mut maintype = Type {
        byte_1 : 0,
        byte_2 : 0,
        byte_3 : 0,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(1, maintype.byte_1);
    assert_eq!(4294967286, maintype.byte_2);
    assert_eq!(10, maintype.byte_3); //Overflow
}

#[test]
fn dint_addition(){

    #[repr(C)]
    struct Type {
        byte_1 : i32,
        byte_2 : i32,
        byte_3 : i32,
    }

    let program = r#"
        PROGRAM main
        VAR
            dint_1 : DINT;
            dint_2 : DINT;
            dint_3 : DINT;
        END_VAR
            dint_1  := 0 + 1;
            dint_2   := 2147483648 - 10;
            dint_3   := 2147483647 + 10;
        END_PROGRAM
        "#;
    
    let mut maintype = Type {
        byte_1 : 0,
        byte_2 : 0,
        byte_3 : 0,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(1,maintype.byte_1);
    assert_eq!(2147483638,maintype.byte_2);
    assert_eq!(-2147483639,maintype.byte_3); //overflow
}

#[test]
fn udint_addition(){

    #[repr(C)]
    struct Type {
        byte_1 : u32,
        byte_2 : u32,
        byte_3 : u32,
    }

    let program = r#"
        PROGRAM main
        VAR
            udint_1 : UDINT;
            udint_2 : UDINT;
            udint_3 : UDINT;
        END_VAR
            udint_1  := 0 + 1;
            udint_2  := 4294967295 - 10;
            udint_3  := 4294967295 + 10;
        END_PROGRAM
        "#;
    
    let mut maintype = Type {
        byte_1 : 0,
        byte_2 : 0,
        byte_3 : 0,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(1, maintype.byte_1);
    assert_eq!(4294967285, maintype.byte_2);
    assert_eq!(9, maintype.byte_3); //Overflow
}

// #[test]
// fn mixed_type_addition() {
//     let function = r"
//         PROGRAM main
//         VAR
//             bool_1   : BOOL;
//             bool_2   : BOOL;
//             bool_3   : BOOL;
//             byte_1   : BYTE;
//             byte_2   : BYTE;
//             byte_3   : BYTE;
//             sint_1   : SINT;
//             sint_2   : SINT;
//             sint_3   : SINT;
//             usint_1  : USINT;
//             usint_2  : USINT;
//             usint_3  : USINT;
//             word_1   : WORD;
//             word_2   : WORD;
//             word_3   : WORD;
//             int_1    : INT;
//             int_2    : INT;
//             int_3    : INT;
//             uint_1   : UINT;
//             uint_2   : UINT;
//             uint_3   : UINT;
//             dword_1  : DWORD;
//             dword_2  : DWORD;
//             dword_3  : DWORD;
//             dint_1   : DINT;
//             dint_2   : DINT;
//             dint_3   : DINT;
//             udint_1 : UDINT;
//             udint_2 : UDINT;
//             udint_3 : UDINT;
//             lword_1 : LWORD;
//             lword_2 : LWORD;
//             lword_3 : LWORD;
//             lint_1  : LINT;
//             lint_2  : LINT;
//             lint_3  : LINT;
//             ulint_1 : ULINT;
//             ulint_2 : ULINT;
//             ulint_3 : ULINT;
//         END_VAR
//             bool_1  := 0 + 0;
//             bool_2  := 10 + 1;
//             bool_3  := 1 + TRUE;
            
//             sint_1  := 50;
//             byte_1  := 1 + 300;
//             byte_2  := 300 + sint_1;
//             byte_3  := 255 + bool_2;

//             int_1 := 10;
//             sint_1  := byte_1 + 5000;
//             sint_2  := int_1 + 2;
//             sint_3  := 65599 - 10;
            
//             usint_1  := sint_1 + 5000;
//             usint_2  := int_1 + 2;
//             usint_3  := 65599 - 10;

            
//             dword_1 := 4294967295; 
//             word_1  := usint_1 + 1;
//             word_2  := dword_1 + 10;
            
//             int_1   := 0 + 10;
//             int_2   := 2147483648 - 10;
//             int_3   := 2147483647 + 10;

//             uint_1  := 0 + 1;
//             uint_2  := 4294967296 - 10;
//             uint_3  := 4294967296 + 10;
           
//             dint_1 := -10;
//             lword_1 := 5;
//             dword_1 := uint_1 + 10;
//             dword_2 := dint_1 + 10;
//             dword_3 := lword_1 + 10;
           
//             udint_1 := 5;
//             dint_1  := int_1 + 10;
//             dint_2  := udint_1 + 10;
//             dint_3  := lword_1 + 10;
            
//             udint_1 := sint_1 + 10;
//             udint_2 := dint_1 + 10;
//             udint_3 := lword_1 + 10;
           
//             lint_1 := 1234;
//             lword_1 := udint_1 + 10;
//             lword_2 := sint_1 + 10;
//             lword_3 := lint_1 + 10;

//             lint_1  := udint_1 + 10;
//             lint_2  := int_1 + 10;
//             lint_3  := lword_1 + 10;
            
//             ulint_1 := udint_1 + 10;
//             ulint_2 := lint_1 + 10;
//             ulint_3 := sint_1 + 10;


//         END_PROGRAM
//         ";

//     let mut maintype = new();

//     compile_and_run(function.to_string(), &mut maintype);
//     assert_eq!(false, maintype.bool_1);
//     assert_eq!(true, maintype.bool_2);
//     assert_eq!(false, maintype.bool_3); //Overflow
    
//     assert_eq!(45,maintype.byte_1);
//     assert_eq!(94,maintype.byte_2);
//     assert_eq!(254,maintype.byte_3); //overflow
    
//     assert_eq!(5045, maintype.sint_1);
//     assert_eq!(12, maintype.sint_2);
//     assert_eq!(53, maintype.sint_3); //Overflow
    
//     assert_eq!(10045, maintype.usint_1);
//     assert_eq!(12, maintype.usint_2);
//     assert_eq!(53, maintype.usint_3); //Overflow
    
//     assert_eq!(10046,maintype.word_1);
//     assert_eq!(9,maintype.word_2);
    
//     assert_eq!(10,maintype.int_1);
//     assert_eq!(2147483638,maintype.int_2);
//     assert_eq!(-2147483639,maintype.int_3); //overflow
    
//     assert_eq!(1,maintype.uint_1);
//     assert_eq!(4294967286,maintype.uint_2);
//     assert_eq!(10,maintype.uint_3); //overflow
    
//     assert_eq!(11,maintype.dword_1);
//     assert_eq!(0,maintype.dword_2);
//     assert_eq!(15,maintype.dword_3);
    
//     assert_eq!(20,maintype.dint_1);
//     assert_eq!(15,maintype.dint_2);
//     assert_eq!(15,maintype.dint_3);
    
//     assert_eq!(5055,maintype.udint_1);
//     assert_eq!(30,maintype.udint_2);
//     assert_eq!(15,maintype.udint_3);

//     assert_eq!(5065,maintype.lword_1);
//     assert_eq!(5055,maintype.lword_2);
//     assert_eq!(1244,maintype.lword_3);
    
//     assert_eq!(5065,maintype.lint_1);
//     assert_eq!(20,maintype.lint_2);
//     assert_eq!(5075,maintype.lint_3);

//     assert_eq!(5065,maintype.ulint_1);
//     assert_eq!(5075,maintype.ulint_2);
//     assert_eq!(5055,maintype.ulint_3);
    
// }

#[test]
fn unsinged_byte_expansion() {
    #[repr(C)]
    struct Type {
        byte_1 : u8,
        int_1 : i32,
    }

    let program = r#"
        PROGRAM main
        VAR
            byte_1 : BYTE;
            int_1 : DINT;
        END_VAR
        byte_1 := 255;
        int_1 := byte_1 + 10;
        END_PROGRAM
        "#;
    
    let mut maintype = Type {
        byte_1 : 0,
        int_1 : 0,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(265,maintype.int_1);
}


#[test]
fn unsinged_byte_expansion2() {
    #[repr(C)]
    struct Type {
        byte_1 : u8,
        byte_2 : i16,
        byte_3 : u16,
        int_1 : i32,
        int_2 : i32,
    }

    let program = r#"
        PROGRAM main
        VAR
            u_byte_1 : BYTE;
            s_byte_2 : INT;
            u_byte_3 : UINT;
            u_int_1 : DWORD;
            u_int_2 : DWORD;
        END_VAR
        u_byte_1 := 255;
        s_byte_2 := -10;
        u_byte_3 := 65525;
        u_int_1 := u_byte_1 + s_byte_2;
        u_int_2 := u_byte_1 + u_byte_3;
        END_PROGRAM
        "#;
    
    let mut maintype = Type {
        byte_1 : 0,
        byte_2: 0,
        byte_3: 0,
        int_1 : 0,
        int_2 : 0,
    };

    compile_and_run(program.to_string(), &mut maintype);
    assert_eq!(245,maintype.int_1);
    assert_eq!(65780,maintype.int_2);
}


#[test]
fn unsinged_byte_expansion3() {
    #[repr(C)]
    struct Type {
        arg1 : u32,
        arg2 : u32,
        arg3 : u64,
        result : u64,
    }

    let program = r#"
        PROGRAM main
        VAR
            arg1 : UDINT;
            arg2 : UDINT;
            arg3 : ULINT;
            result : ULINT;
        END_VAR
        
        result := arg1 + (arg2 + arg3) + (arg2 + arg3);
        END_PROGRAM
        "#;
    
/*
 *              +
 *      arg1        +
 *              arg2    arg3
 * 
 */

    let mut maintype = Type {
        arg1 : 10000,
        arg2 : 0xFFFF_FFFF,
        arg3 : 10,
        result : 0,
    };
    compile_and_run(program.to_string(), &mut maintype);
    let arg1 : u64 = maintype.arg1.into();
    let arg2 : u64 = maintype.arg2.into();
    let arg3 : u64 = maintype.arg3.into();
    let expected : u64 = arg1 + (arg2+arg3)+(arg2+arg3);
    assert_eq!(expected,
                maintype.result);
}
