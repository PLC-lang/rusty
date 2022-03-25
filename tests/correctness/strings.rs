use pretty_assertions::assert_eq;

use super::super::*;

#[test]
fn string_assignment_from_smaller_literal() {
    let src = "
        PROGRAM main
            VAR x : STRING[6]; END_VAR
            x := 'hello';
        END_PROGRAM
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 7],
    }
    let mut main_type = MainType { x: [0; 7] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("hello\0\0".as_bytes(), &main_type.x);
}

#[test]
fn string_assignment_from_bigger_literal() {
    let src = "
        PROGRAM main
            VAR x : STRING[4];END_VAR
            x := 'hello';
        END_PROGRAM
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 5],
    }
    let mut main_type = MainType { x: [0; 5] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("hell\0".as_bytes(), &main_type.x);
}
#[test]
fn string_assignment_from_smaller_string() {
    let src = "
        PROGRAM main 
            VAR x : STRING[6]; y : STRING[5]; END_VAR
            y := 'hello';
            x := y;
        END_PROGRAM
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 7],
        y: [u8; 6],
    }
    let mut main_type = MainType {
        x: [0; 7],
        y: [0; 6],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("hello\0\0".as_bytes(), &main_type.x);
}

#[test]
fn string_assignment_from_bigger_string() {
    let src = "
        PROGRAM main
            VAR x : STRING[4]; y : STRING[5]; END_VAR
            y := 'hello';
            x := y;
        END_PROGRAM
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 5],
        y: [u8; 6],
    }
    let mut main_type = MainType {
        x: [0; 5],
        y: [0; 6],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("hell\0".as_bytes(), &main_type.x);
}

#[test]
fn string_assignment_from_smaller_function() {
    let src = "
        TYPE ReturnString : STRING[5] := ''; END_TYPE

        FUNCTION hello : ReturnString
        hello := 'hello';
        END_FUNCTION

        PROGRAM main
            VAR x : STRING[6]; END_VAR
            x := hello();
        END_PROGRAM
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 7],
    }
    let mut main_type = MainType { x: [0; 7] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("hello\0\0".as_bytes(), &main_type.x);
}

#[test]
fn string_assignment_from_bigger_function() {
    let src = "
        FUNCTION hello : STRING[5]
        hello := 'hello';
        END_FUNCTION

        PROGRAM main
            VAR x : STRING[4]; END_VAR
            x := hello();
        END_PROGRAM
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 5],
    }
    let mut main_type = MainType { x: [0; 5] };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!("hell\0".as_bytes(), &main_type.x);
}

#[test]
fn string_assignment_from_bigger_literal_do_not_leak() {
    let src = "
        FUNCTION main : DINT
            VAR x,y : STRING[4]; END_VAR
            x := 'hello';
        END_FUNCTION
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 5],
        y: [u8; 5],
    }
    let mut main_type = MainType {
        x: [0; 5],
        y: [0; 5],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!(&[0; 5], &main_type.y);
}

#[test]
fn string_assignment_from_bigger_string_does_not_leak() {
    let src = "
        FUNCTION main : DINT
            VAR x,y : STRING[4]; z : STRING[10]; END_VAR
            z := 'hello foo';
            x := z;
        END_FUNCTION
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 5],
        y: [u8; 5],
        z: [u8; 11],
    }
    let mut main_type = MainType {
        x: [0; 5],
        y: [0; 5],
        z: [0; 11],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!(&[0; 5], &main_type.y);
}

#[test]
fn string_assignment_from_bigger_function_does_not_leak() {
    let src = "
        FUNCTION hello : STRING[10]
        hello := 'hello foo';
        END_FUNCTION

        FUNCTION main : DINT
            VAR x,y : STRING[4]; END_VAR
            x := hello();
        END_FUNCTION
    ";

    #[allow(dead_code)]
    struct MainType {
        x: [u8; 5],
        y: [u8; 5],
    }
    let mut main_type = MainType {
        x: [0; 5],
        y: [0; 5],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!(&[0; 5], &main_type.y);
}

#[test]
fn initialization_of_string_arrays() {
    let src = "
        VAR_GLOBAL
            texts: ARRAY[0..2] OF STRING[10] := ['hello', 'world', 'ten chars!']
        END_VAR

        PROGRAM main
            VAR x,y,z : STRING[10]; END_VAR
        
            x := texts[0];
            y := texts[1];
            z := texts[2];
        
        END_PROGRAM
    ";

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x: [u8; 11],
        y: [u8; 11],
        z: [u8; 11],
    }
    let mut main_type = MainType {
        x: [0; 11],
        y: [0; 11],
        z: [0; 11],
    };

    let _: i32 = compile_and_run(src, &mut main_type);
    assert_eq!(main_type.x, "hello\0\0\0\0\0\0".as_bytes());
    assert_eq!(main_type.y, "world\0\0\0\0\0\0".as_bytes());
    assert_eq!(main_type.z, "ten chars!\0".as_bytes());
}
