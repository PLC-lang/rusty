use rusty::runner::compile_and_run;

#[test]
fn variable_length_array_single_dimension_access() {
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
        VAR_INPUT
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
        arr : ARRAY[-5..5, -10..15, -1..1, 0..3] OF LINT;
    END_VAR

    foo(arr);
    a := arr[-5,   5, -1, 3];
    b := arr[ 5,  11, -1, 3];
    c := arr[-1,  10, -1, 3];
    d := arr[ 0,   0, -1, 3];
END_PROGRAM

FUNCTION foo : DINT
    VAR_INPUT
        vla : ARRAY[ *, *, *, * ] OF LINT;
    END_VAR

    vla[-5,   5, -1, 3] := 10;
    vla[ 5,  11, -1, 3] := -7;
    vla[-1,  10, -1, 3] := 4;
    vla[ 0,   0, -1, 3] := 8;
END_FUNCTION
    "#;

    let _: i32 = compile_and_run(src.to_string(), &mut main_type);
    assert_eq!(10, main_type.a);
    assert_eq!(-7, main_type.b);
    assert_eq!(4, main_type.c);
    assert_eq!(8, main_type.d);
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
