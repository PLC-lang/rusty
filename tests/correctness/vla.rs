use std::ffi::CStr;

use rusty::runner::compile_and_run;

#[test]
fn variable_length_array_single_dimension_access() {
    #[derive(Default)]
    struct MainType {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
    }

    let mut main_type = MainType::default();
    let src = r#"
    PROGRAM main
        VAR
            a, b, c, d, e : LINT;
        END_VAR
        VAR_TEMP
            arr : ARRAY[-21..4] OF LINT;
        END_VAR

        foo(arr);
        a := arr[-21];
        b := arr[1];
        c := arr[2];
        d := arr[3];
        e := arr[4];
    END_PROGRAM

    FUNCTION foo : DINT
        VAR_INPUT
            vla : ARRAY[ * ] OF LINT;
        END_VAR

        vla[-21] := 2;
        vla[1] := 4;
        vla[2] := 6;
        vla[3] := 8;
        vla[4] := 10;
    END_FUNCTION
    "#;

    let _: i32 = compile_and_run(src.to_string(), &mut main_type);
    assert_eq!(2, main_type.a);
    assert_eq!(4, main_type.b);
    assert_eq!(6, main_type.c);
    assert_eq!(8, main_type.d);
    assert_eq!(10, main_type.e);
}

#[test]
fn variable_length_array_multi_dimension_access() {
    #[derive(Default)]
    struct MainType {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }

    let mut main_type = MainType::default();
    let src = r#"
    PROGRAM main
        VAR
            a, b, c, d: DINT;
        END_VAR
        VAR_TEMP
            arr : ARRAY[0..1, 0..1] OF DINT;
        END_VAR

        foo(arr);
        a := arr[0, 0];
        b := arr[0, 1];
        c := arr[1, 0];
        d := arr[1, 1];   
    END_PROGRAM

    FUNCTION foo : DINT
        VAR_INPUT
            vla : ARRAY[ *, * ] OF DINT;
        END_VAR

        vla[0, 0] := 0;
        vla[0, 1] := 2;
        vla[1, 0] := 4;
        vla[1, 1] := 8;  
    END_FUNCTION
    "#;

    let _: i32 = compile_and_run(src.to_string(), &mut main_type);
    assert_eq!(0, main_type.a);
    assert_eq!(2, main_type.b);
    assert_eq!(4, main_type.c);
    assert_eq!(8, main_type.d);
}

#[test]
fn variable_length_array_multi_dimension_read_write() {
    #[derive(Default)]
    struct MainType {
        a: i64,
        b: i64,
        c: i64,
    }

    let mut main_type = MainType::default();
    let src = r#"
    PROGRAM main
        VAR
            a, b, c:  LINT;
        END_VAR
        VAR_TEMP
            arr : ARRAY[0..3, 0..2, 0..1, 0..10] OF LINT;
        END_VAR

        foo(arr);
        a := arr[1, 2, 1, 8];
        b := arr[2, 1, 1, 6];
        c := arr[3, 1, 1, 4];
    END_PROGRAM

    FUNCTION foo : DINT
        VAR_INPUT
            vla : ARRAY[ *, *, *, *] OF LINT;
        END_VAR

        vla[1, 2, 1, 8] := -7; 
        vla[2, 1, 1, 6] := 72; 
        vla[3, 1, 1, 4] := 11; 
    END_FUNCTION
    "#;

    let _: i32 = compile_and_run(src.to_string(), &mut main_type);
    assert_eq!(-7, main_type.a);
    assert_eq!(72, main_type.b);
    assert_eq!(11, main_type.c);
}

#[test]
fn variable_length_array_multi_dimension_read_write_with_offsets() {
    #[derive(Default)]
    struct MainType {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }

    let mut main_type = MainType::default();
    let src = r#"
    PROGRAM main
    VAR
        a, b, c, d: LINT;
    END_VAR
    VAR_TEMP
        arr : ARRAY[-5..5, -21..15, -21..1, 0..3] OF LINT;
    END_VAR

    foo(arr);
    a := arr[  5, -11, -17, 3];
    b := arr[ -2, -21,  -0, 2];
    c := arr[ -5,  15,   1, 1];
    d := arr[  0,   0,   0, 0];
END_PROGRAM

FUNCTION foo : DINT
    VAR_INPUT 
        vla : ARRAY[ *, *, *, * ] OF LINT;
    END_VAR

    vla[  5, -11, -17, 3] := 10;
    vla[ -2, -21, -0,  2] := -7;
    vla[ -5,  15,  1,  1] := 4;
    vla[  0,   0,  0,  0] := 8;
END_FUNCTION
    "#;

    let _: i32 = compile_and_run(src.to_string(), &mut main_type);
    assert_eq!(10, main_type.a);
    assert_eq!(-7, main_type.b);
    assert_eq!(4, main_type.c);
    assert_eq!(8, main_type.d);
}

#[test]
fn consecutive_calls_with_differently_sized_arrays() {
    #[derive(Default)]
    struct MainType {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
        f: i64,
        g: i64,
        h: i64,
    }

    let mut main_type = MainType::default();
    let src = r#"
    PROGRAM main
    VAR
        a, b, c, d, e, f, g, h: LINT;
    END_VAR
    VAR_TEMP
        arr : ARRAY[-5..5, -1..15] OF LINT;
        arr2 : ARRAY[-21..20, -13..13] OF LINT;
    END_VAR

    foo(arr);
    foo(arr2);

    a := arr [  5, -11 ];
    b := arr [ -1,   1 ];
    c := arr [ -2,  13 ];
    d := arr [  0,   0 ];

    e := arr2 [  5, -11 ];
    f := arr2 [ -1,   1 ];
    g := arr2 [ -2,  13 ];
    h := arr2 [  0,   0 ];
    END_PROGRAM

    FUNCTION foo : DINT
        VAR_INPUT 
            vla : ARRAY[ *, * ] OF LINT;
        END_VAR

        vla [  5, -11 ] := 10;
        vla [ -1,   1 ] := -7;
        vla [ -2,  13 ] :=  4;
        vla [  0,   0 ] :=  8;
    END_FUNCTION
    "#;

    let _: i32 = compile_and_run(src.to_string(), &mut main_type);
    assert_eq!(10, main_type.a);
    assert_eq!(-7, main_type.b);
    assert_eq!(4, main_type.c);
    assert_eq!(8, main_type.d);

    assert_eq!(10, main_type.e);
    assert_eq!(-7, main_type.f);
    assert_eq!(4, main_type.g);
    assert_eq!(8, main_type.h);
}

#[test]
fn variable_length_array_single_dimension_access_with_offset() {
    #[derive(Default)]
    struct MainType {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
    }

    let mut main_type = MainType::default();
    let src = r#"
    PROGRAM main
        VAR
            a, b, c, d, e : DINT;
        END_VAR
        VAR_TEMP
            arr : ARRAY[5..5+4] OF DINT;
        END_VAR

        foo(arr);
        a := arr[5];
        b := arr[6];
        c := arr[7];
        d := arr[8];
        e := arr[9];
    END_PROGRAM

    FUNCTION foo : DINT
        VAR_INPUT
            vla : ARRAY[ * ] OF DINT;
        END_VAR

        vla[5] := 2;
        vla[6] := 4;
        vla[7] := 6;
        vla[8] := 8;
        vla[9] := 10;
    END_FUNCTION
    "#;

    let _: i32 = compile_and_run(src.to_string(), &mut main_type);
    assert_eq!(2, main_type.a);
    assert_eq!(4, main_type.b);
    assert_eq!(6, main_type.c);
    assert_eq!(8, main_type.d);
    assert_eq!(10, main_type.e);
}

#[test]
fn variable_length_array_var_input_ref() {
    #[derive(Default)]
    struct MainType {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
    }

    let mut main_type = MainType::default();
    let src = r#"
    PROGRAM main
        VAR
            a, b, c, d, e : DINT;
        END_VAR
        VAR_TEMP
            arr : ARRAY[-2..2] OF DINT;
        END_VAR

        foo(arr);
        a := arr[-2];
        b := arr[-1];
        c := arr[0] ;
        d := arr[1] ;
        e := arr[2] ;
    END_PROGRAM

    FUNCTION foo : DINT
        VAR_INPUT {ref}
            vla : ARRAY[ * ] OF DINT;
        END_VAR

        vla[-2] := 2;
        vla[-1] := 4;
        vla[0] := 6;
        vla[1] := 8;
        vla[2] := 10;
    END_FUNCTION
    "#;

    let _: i32 = compile_and_run(src.to_string(), &mut main_type);
    assert_eq!(2, main_type.a);
    assert_eq!(4, main_type.b);
    assert_eq!(6, main_type.c);
    assert_eq!(8, main_type.d);
    assert_eq!(10, main_type.e);
}

#[test]
fn variable_length_array_by_ref_param_access() {
    #[derive(Default)]
    struct MainType {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
    }

    let mut main_type = MainType::default();
    let src = r#"
    PROGRAM main
        VAR
            a, b, c, d, e : DINT;
        END_VAR
        VAR_TEMP
            arr : ARRAY[0..4] OF DINT;
        END_VAR

        foo(arr);
        a := arr[0];
        b := arr[1];
        c := arr[2];
        d := arr[3];
        e := arr[4];
    END_PROGRAM

    FUNCTION foo : DINT    
        VAR_IN_OUT
            vla : ARRAY[ * ] OF DINT;
        END_VAR

        vla[0] := 2;
        vla[1] := 4;
        vla[2] := 6;
        vla[3] := 8;
        vla[4] := 10;
    END_FUNCTION
    "#;

    let _: i32 = compile_and_run(src.to_string(), &mut main_type);
    assert_eq!(2, main_type.a);
    assert_eq!(4, main_type.b);
    assert_eq!(6, main_type.c);
    assert_eq!(8, main_type.d);
    assert_eq!(10, main_type.e);
}

#[test]
#[ignore = "not yet implemented"]
fn variable_length_array_output_param_access() {
    #[derive(Default)]
    struct MainType {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
    }

    let mut main_type = MainType::default();
    let src = r#"
    PROGRAM main
    VAR
        a, b, c, d, e : DINT;
    END_VAR
    VAR_TEMP
        arr: ARRAY[0..4] OF DINT;
    END_VAR

        foo(arr);
        a := arr[0];
        b := arr[1];
        c := arr[2];
        d := arr[3];
        e := arr[4];
    END_PROGRAM

    FUNCTION foo : DINT    
        VAR_OUTPUT
            vla : ARRAY[ * ] OF DINT;
        END_VAR

        vla[0] := 2;
        vla[1] := 4;
        vla[2] := 6;
        vla[3] := 8;
        vla[4] := 10;
    END_FUNCTION
    "#;

    let _: i32 = compile_and_run(src.to_string(), &mut main_type);
    assert_eq!(2, main_type.a);
    assert_eq!(4, main_type.b);
    assert_eq!(6, main_type.c);
    assert_eq!(8, main_type.d);
    assert_eq!(10, main_type.e);
}

#[test]
fn variable_length_array_with_global_array() {
    #[derive(Default)]
    struct MainType {
        a: i32,
    }

    let mut main_type = MainType::default();
    let src = r#"
    VAR_GLOBAL
        arr : ARRAY[0..1] OF INT;
    END_VAR

    FUNCTION foo : INT
    VAR_INPUT
        vla : ARRAY[*] OF INT;
    END_VAR
        vla[0] := 20;
    END_FUNCTION

    PROGRAM main
    VAR
        a : DINT;
    END_VAR
        foo(arr);
        a := arr[0];
    END_FUNCTION
    "#;

    let _: i32 = compile_and_run(src.to_string(), &mut main_type);
    assert_eq!(20, main_type.a);
}

#[test]
fn variable_length_array_multi_dimension_with_strings() {
    #[repr(C)]
    struct MainType {
        a: [u8; 81],
        b: [u8; 81],
        c: [u8; 81],
        d: [u8; 81],
    }

    let mut maintype = MainType { a: [0_u8; 81], b: [0_u8; 81], c: [0_u8; 81], d: [0_u8; 81] };
    let src = r#"
    PROGRAM main
        VAR
            a, b, c, d: STRING;
        END_VAR
        VAR_TEMP
            arr : ARRAY[0..1, 0..1] OF STRING;
        END_VAR

        foo(arr);
        a := arr[0, 0];
        b := arr[0, 1];
        c := arr[1, 0];
        d := arr[1, 1];   
    END_PROGRAM

    FUNCTION foo : DINT
        VAR_INPUT
            vla : ARRAY[ *, * ] OF STRING;
        END_VAR

        vla[0, 0] := 'brave ';
        vla[0, 1] := 'new ';
        vla[1, 0] := 'world ';
        vla[1, 1] := '📖';  
    END_FUNCTION
    "#;

    let _: i32 = compile_and_run(src.to_string(), &mut maintype);
    let expected = "brave new world 📖";
    let result = format!(
        "{}{}{}{}",
        unsafe { CStr::from_bytes_with_nul_unchecked(&maintype.a) }.to_str().unwrap().trim_end_matches('\0'),
        unsafe { CStr::from_bytes_with_nul_unchecked(&maintype.b) }.to_str().unwrap().trim_end_matches('\0'),
        unsafe { CStr::from_bytes_with_nul_unchecked(&maintype.c) }.to_str().unwrap().trim_end_matches('\0'),
        unsafe { CStr::from_bytes_with_nul_unchecked(&maintype.d) }.to_str().unwrap().trim_end_matches('\0'),
    );
    assert_eq!(expected, result);
}

#[test]
#[ignore = "not yet implemented"]
fn variable_length_array_of_array() {
    #[derive(Default)]
    struct MainType {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32,
        g: i32,
        h: i32,
    }

    let mut main_type = MainType::default();
    let src = r#"
    PROGRAM main
        VAR
            a, b, c, d, e, f, g, h: DINT;
        END_VAR
        VAR_TEMP
            arr : ARRAY[0..1, 0..1] OF ARRAY[0..1] OF DINT;
        END_VAR

        foo(arr);
        a := arr[0, 0][0];
        b := arr[0, 0][1];
        c := arr[0, 1][0];
        d := arr[0, 1][1];   
        e := arr[1, 0][0];   
        f := arr[1, 0][1];   
        g := arr[1, 1][0];   
        h := arr[1, 1][1];   
        
    END_PROGRAM

    FUNCTION foo : DINT
        VAR_INPUT
            vla : ARRAY[ *, * ] OF ARRAY[ * ] OF DINT;
        END_VAR

        vla[0, 0][0] := 0;
        vla[0, 0][1] := 1;
        vla[0, 1][0] := 2;
        vla[0, 1][1] := 3;
        vla[1, 0][0] := 4; 
        vla[1, 0][1] := 5; 
        vla[1, 1][0] := 6; 
        vla[1, 1][1] := 7; 
    END_FUNCTION
    "#;

    let _: i32 = compile_and_run(src.to_string(), &mut main_type);
    assert_eq!(0, main_type.a);
    assert_eq!(1, main_type.b);
    assert_eq!(2, main_type.c);
    assert_eq!(3, main_type.d);
    assert_eq!(4, main_type.e);
    assert_eq!(5, main_type.f);
    assert_eq!(6, main_type.g);
    assert_eq!(7, main_type.h);
}
