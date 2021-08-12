// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::super::*;

macro_rules! permutate_conditionals {
    ($code: tt, $condition : tt) => {{
        let true_1 = format!($code, $condition = "TRUE");
        let false_1 = format!($code, $condition = "FALSE");
        (true_1, false_1)
    }};
}

#[test]
fn adding_through_conditions() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        inc: i32,
        cond: bool,
        ret: i32,
    }

    let function = permutate_conditionals!(
        r#"
    FUNCTION main : DINT
    VAR
        inc : DINT;
        cond : BOOL;
    END_VAR

    cond := {cond};
    inc := 0;

    IF cond THEN
        inc := inc + 10;
    ELSE
        inc := inc + 100;
    END_IF

    main := inc;

    END_FUNCTION

    "#,
        cond
    );

    let (func_true, func_false) = function;

    let (res, _) = compile_and_run(
        func_true,
        &mut MainType {
            inc: 0,
            cond: false,
            ret: 0,
        },
    );
    assert_eq!(res, 10);
    let (res, _) = compile_and_run(
        func_false,
        &mut MainType {
            inc: 0,
            cond: false,
            ret: 0,
        },
    );
    assert_eq!(res, 100);
}

#[test]
fn adding_through_conditions_to_function_return() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        ret: i32,
    }

    let function = permutate_conditionals!(
        r#"
    FUNCTION main : DINT
    VAR
    END_VAR
    main := 0;
    IF {cond} THEN
        main := main + 10;
    ELSE
        main := main + 100;
    END_IF

    END_FUNCTION

    "#,
        cond
    );

    let (func_true, func_false) = function;

    let (res, _) = compile_and_run(func_true, &mut MainType { ret: 0 });
    assert_eq!(res, 10);
    let (res, _) = compile_and_run(func_false, &mut MainType { ret: 0 });
    assert_eq!(res, 100);
}

#[test]
fn early_return_test() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        ret: i32,
    }

    let function = r#"
    FUNCTION main : DINT
    main := 100;
    RETURN
    main := 200;
    END_FUNCTION
    "#;

    let (res, _) = compile_and_run(function.to_string(), &mut MainType { ret: 0 });
    assert_eq!(res, 100);
}

#[test]
fn for_loop_and_increment_10_times() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        i: i16,
        ret: i32,
    }

    let function = r#"
    FUNCTION main : DINT
    VAR
        i : INT;
    END_VAR
    main := 100;
    FOR i:= 1 TO 10 DO
        main := main + 1;
    END_FOR
    END_FUNCTION
    "#;

    let (res, _) = compile_and_run(function.to_string(), &mut MainType { i: 0, ret: 0 });
    assert_eq!(res, 110);
}

#[test]
fn for_loop_and_increment_10_times_skipping_1() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        i: i16,
        ret: i16,
    }
    let function = r#"
    FUNCTION main : DINT
    VAR
        i : INT;
    END_VAR
    main := 1000;
    FOR i:= 1 TO 10 BY 2 DO
        main := main + 1;
    END_FOR
    END_FUNCTION
    "#;

    let (res, _) = compile_and_run(function.to_string(), &mut MainType { i: 0, ret: 0 });
    assert_eq!(res, 1005);
}

#[test]
fn while_loop_no_entry() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        i: i16,
        ret: i32,
    }

    let function = r#"
    FUNCTION main : DINT
    VAR
        i : INT;
    END_VAR
    main := 5;
    i := 0;
    WHILE i < 0 DO
        i := i+1;
        main := main + 10;
    END_WHILE
    main := main + (i * 1000);
    END_FUNCTION
    "#;

    let (res, _) = compile_and_run(function.to_string(), &mut MainType { i: 0, ret: 0 });
    assert_eq!(res, 5);
}

#[test]
fn repeat_loop_no_entry() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        i: i16,
        ret: i32,
    }

    let function = r#"
    FUNCTION main : DINT
    VAR
        i : INT;
    END_VAR
    main := 7;
    i := 0;
    REPEAT
        i := i+1;
        main := main + 10;
    UNTIL i < 0 
    END_REPEAT
    main := main + (i * 1000);
    END_FUNCTION
    "#;

    let (res, _) = compile_and_run(function.to_string(), &mut MainType { i: 0, ret: 0 });
    assert_eq!(res, 1017);
}
#[test]
fn while_loop_10_times() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        i: i16,
        ret: i32,
    }
    let function = r#"
    FUNCTION main : DINT
    VAR
        i : DINT;
    END_VAR
    main := 1;
    i := 0;
    WHILE i < 10 DO
        i := i+1;
        main := main + 10;
    END_WHILE
    main := main + (i * 1000);
    END_FUNCTION
    "#;

    let (res, _) = compile_and_run(function.to_string(), &mut MainType { i: 0, ret: 0 });
    assert_eq!(res, 10101);
}

#[test]
fn repeat_loop_10_times() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        i: i16,
        ret: i32,
    }
    let function = r#"
    FUNCTION main : DINT
    VAR
        i : DINT;
    END_VAR
    main := 1;
    i := 0;
    REPEAT
        i := i+1;
        main := main + 10;
    UNTIL i < 10
    END_REPEAT
    main := main + (i * 1000);
    END_FUNCTION
    "#;

    let (res, _) = compile_and_run(function.to_string(), &mut MainType { i: 0, ret: 0 });
    assert_eq!(res, 10101);
}

#[test]
fn case_statement() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        i: i16,
    }
    let function = r#"
    FUNCTION main : DINT
    VAR
        i : INT;
    END_VAR
    main := 1;
    
    CASE i OF
        1,2,3,4,5,6,7,8,9: main := 101;
        10,11,12..19: main := 201;
        20..24, 25..29: main := 301;
        ELSE main := 7;
    END_CASE
    END_FUNCTION
    "#;

    (1..9).for_each(|i| {
        let (res, _) = compile_and_run(function.to_string(), &mut MainType { i });
        assert_eq!(res, 101);
    });

    (10..19).for_each(|i| {
        let (res, _) = compile_and_run(function.to_string(), &mut MainType { i });
        assert_eq!(res, 201);
    });

    (20..29).for_each(|i| {
        let (res, _) = compile_and_run(function.to_string(), &mut MainType { i });
        assert_eq!(res, 301);
    });

    let (res, _) = compile_and_run(function.to_string(), &mut MainType { i: 999 });
    assert_eq!(res, 7);
}
