// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::compile_and_run;

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

    let res: i32 = compile_and_run(func_true, &mut MainType { inc: 0, cond: false, ret: 0 });
    assert_eq!(res, 10);
    let res: i32 = compile_and_run(func_false, &mut MainType { inc: 0, cond: false, ret: 0 });
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

    let res: i32 = compile_and_run(func_true, &mut MainType { ret: 0 });
    assert_eq!(res, 10);
    let res: i32 = compile_and_run(func_false, &mut MainType { ret: 0 });
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

    let res: i32 = compile_and_run(function.to_string(), &mut MainType { ret: 0 });
    assert_eq!(res, 100);
}

#[test]
fn while_continue_test() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        ret: i32,
    }

    let function = r#"
    FUNCTION main : DINT
    main := 1;
    WHILE main < 10 DO
        main := main + 1;
        CONTINUE;
        main := 200;
    END_WHILE
    END_FUNCTION
    "#;

    let res: i32 = compile_and_run(function.to_string(), &mut MainType { ret: 0 });
    assert_eq!(res, 10);
}

#[test]
fn while_loop_exit_test() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        ret: i32,
    }

    let function = r#"
    FUNCTION main : DINT
    main := 100;
    WHILE main > 50 DO
        EXIT;
        main := 200;
    END_WHILE
    END_FUNCTION
    "#;

    let res: i32 = compile_and_run(function.to_string(), &mut MainType { ret: 0 });
    assert_eq!(res, 100);
}

#[test]
fn while_loop_no_entry() {
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

    let res: i32 = compile_and_run(function.to_string(), &mut crate::MainType::default());
    assert_eq!(res, 5);
}

#[test]
fn exit_in_if_in_while_loop() {
    let function = r#"
    FUNCTION main : DINT
    VAR
        i : INT;
    END_VAR
    i := 0;
    WHILE i < 20 DO
        i := i+1;
        IF i >= 10 THEN
            EXIT;
        END_IF
    END_WHILE
    main := i;
    END_FUNCTION
    "#;

    let res: i32 = compile_and_run(function.to_string(), &mut crate::MainType::default());
    assert_eq!(res, 10);
}

#[test]
fn repeat_loop_no_entry() {
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
    UNTIL i > 0
    END_REPEAT
    main := main + (i * 1000);
    END_FUNCTION
    "#;

    let res: i32 = compile_and_run(function.to_string(), &mut crate::MainType::default());
    assert_eq!(res, 1017);
}
#[test]
fn while_loop_10_times() {
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

    let res: i32 = compile_and_run(function.to_string(), &mut crate::MainType::default());
    assert_eq!(res, 10101);
}

#[test]
fn repeat_loop_10_times() {
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
    UNTIL i > 10
    END_REPEAT
    main := main + (i * 1000);
    END_FUNCTION
    "#;

    let res: i32 = compile_and_run(function.to_string(), &mut crate::MainType::default());

    assert_eq!(res, 11111);
}

#[test]
fn repeat_loop_reference() {
    let function = r#"
    FUNCTION main : DINT
    VAR
        i : DINT;
        b : BOOL;
    END_VAR
    main := 1;
    i := 0;
    REPEAT
        i := i+1;
        main := main + 10;
        b := i > 10;
    UNTIL b
    END_REPEAT
    main := main + (i * 1000);
    END_FUNCTION
    "#;

    let res: i32 = compile_and_run(function.to_string(), &mut crate::MainType::default());

    assert_eq!(res, 11111);
}

#[test]
fn case_statement() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        i: i16,
        ret: i16,
    }
    let function = r#"
    PROGRAM main
    VAR_INPUT
        i : INT;
        ret : INT;
    END_VAR
    ret := 1;

    CASE i OF
        1,2,3,4,5,6,7,8,9: ret := 101;
        10,11,12..19: ret := 201;
        20..24, 25..29: ret := 301;
        ELSE ret := 7;
    END_CASE
    END_PROGRAM
    "#;

    (1..9).for_each(|i| {
        let p = &mut MainType { i, ret: 0 };
        let _: i32 = compile_and_run(function.to_string(), p);
        assert_eq!(p.ret, 101);
    });

    (10..19).for_each(|i| {
        let p = &mut MainType { i, ret: 0 };
        let _: i32 = compile_and_run(function.to_string(), p);
        assert_eq!(p.ret, 201);
    });

    (20..29).for_each(|i| {
        let p = &mut MainType { i, ret: 0 };
        let _: i32 = compile_and_run(function.to_string(), p);
        assert_eq!(p.ret, 301);
    });

    let p = &mut MainType { i: 999, ret: 0 };
    let _: i32 = compile_and_run(function.to_string(), p);
    assert_eq!(p.ret, 7);
}
