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

    let res: i32 = compile_and_run(
        func_true,
        &mut MainType {
            inc: 0,
            cond: false,
            ret: 0,
        },
    );
    assert_eq!(res, 10);
    let res: i32 = compile_and_run(
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
fn for_continue_test() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        ret: i32,
    }

    let function = r#"
    FUNCTION main : DINT
    FOR main := 1 TO 10 BY 1 DO
        main := 10;
        CONTINUE;
        main := 200; 
    END_FOR
    END_FUNCTION
    "#;

    let res: i32 = compile_and_run(function.to_string(), &mut MainType { ret: 0 });
    assert_eq!(res, 11);
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
fn for_loop_exit_test() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        ret: i32,
    }

    let function = r#"
    FUNCTION main : DINT
    FOR main := 100 TO 1000 BY 7 DO
        EXIT;
        main := 200; 
    END_FOR
    "#;

    let res: i32 = compile_and_run(function.to_string(), &mut MainType { ret: 0 });
    assert_eq!(res, 100);
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
fn loop_exit_test() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        ret: i32,
    }

    let function = r#"
    FUNCTION main : DINT
    FOR main := 100 TO 1000 BY 7 DO
        EXIT;
        main := 200; 
    END_FOR
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

    let res: i32 = compile_and_run(function.to_string(), &mut MainType { i: 0, ret: 0 });
    assert_eq!(res, 110);
}

#[test]
fn for_loop_and_decrement_10_times() {
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
    FOR i:= 10 TO 1 BY -1 DO
        main := main + 1;
    END_FOR
    END_FUNCTION
    "#;

    let res: i32 = compile_and_run(function.to_string(), &mut MainType { i: 0, ret: 0 });
    assert_eq!(res, 110);
}

#[test]
fn for_loop_and_increment_10_times_change_vars() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        i: i16,
        ret: i32,
    }

    let function = r#"
    FUNCTION main : DINT
    VAR
        i, : INT := 0;
    END_VAR
    VAR_TEMP
        start, end : INT; 
    END_VAR
    main := 100;
    start := 1;
    end := 2;
    FOR i:= start TO end BY 1 DO
        main := main + 1;
        end := 10;
    END_FOR
    END_FUNCTION
    "#;

    let res: i32 = compile_and_run(function.to_string(), &mut MainType { i: 0, ret: 0 });
    assert_eq!(res, 110);
}

#[test]
fn for_loop_overflow() {
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
    FOR i:= 1 TO 0 BY -2 DO
        main := main + 1;
    END_FOR
    END_FUNCTION
    "#;

    let mut main = MainType { i: 0, ret: 0 };

    let res: i32 = compile_and_run(function.to_string(), &mut main);
    assert_eq!(res, 101);
    assert_eq!(-1, main.i);
}

#[test]
fn for_loop_variable_does_not_affect_other_variables_dint() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        a: i32,
        i: i32,
        b: i32,
    }

    let function = r#"
    FUNCTION main : DINT
    VAR
        a,i,b : DINT;
    END_VAR
    a := 16#FFFF_FFFF;
    b := 16#FFFF_FFFF;
    main := 100;
    FOR i:= 1 TO 10 DO
        main := main + 1;
    END_FOR
    END_FUNCTION
    "#;

    let mut main = MainType { a: 0, i: 0, b: 0 };

    let res: i32 = compile_and_run(function.to_string(), &mut main);
    assert_eq!(res, 110);
    assert_eq!(-1, main.a);
    assert_eq!(-1, main.b);
}

#[test]
fn for_loop_variable_does_not_affect_other_variables_int() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        a: i16,
        i: i16,
        b: i16,
    }

    let function = r#"
    FUNCTION main : DINT
    VAR
        a,i,b : INT;
    END_VAR
    a := 16#FFFF;
    b := 16#FFFF;
    main := 100;
    FOR i:= 1 TO 10 DO
        main := main + 1;
    END_FOR
    END_FUNCTION
    "#;

    let mut main = MainType { a: 0, i: 0, b: 0 };

    let res: i32 = compile_and_run(function.to_string(), &mut main);
    assert_eq!(res, 110);
    assert_eq!(-1, main.a);
    assert_eq!(-1, main.b);
}

#[test]
fn for_loop_variable_does_not_affect_other_variables_sint() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        a: i8,
        i: i8,
        b: i8,
    }

    let function = r#"
    FUNCTION main : DINT
    VAR
        a,i,b : SINT;
    END_VAR
    a := 16#FF;
    b := 16#FF;
    main := 100;
    FOR i:= 1 TO 10 DO
        main := main + 1;
    END_FOR
    END_FUNCTION
    "#;

    let mut main = MainType { a: 0, i: 0, b: 0 };

    let res: i32 = compile_and_run(function.to_string(), &mut main);
    assert_eq!(res, 110);
    assert_eq!(-1, main.a);
    assert_eq!(-1, main.b);
}

#[test]
fn for_loop_variable_does_not_affect_other_variables_lint() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        a: i64,
        i: i64,
        b: i64,
    }

    let function = r#"
    FUNCTION main : DINT
    VAR
        a,i,b : LINT;
    END_VAR
    a := 16#FFFF_FFFF_FFFF_FFFF;
    b := 16#FFFF_FFFF_FFFF_FFFF;
    main := 100;
    FOR i:= 1 TO 10 DO
        main := main + 1;
    END_FOR
    END_FUNCTION
    "#;

    let mut main = MainType { a: 0, i: 0, b: 0 };

    let res: i32 = compile_and_run(function.to_string(), &mut main);
    assert_eq!(res, 110);
    assert_eq!(-1, main.a);
    assert_eq!(-1, main.b);
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

    let res: i32 = compile_and_run(function.to_string(), &mut MainType { i: 0, ret: 0 });
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

    let res: i32 = compile_and_run(function.to_string(), &mut MainType { i: 0, ret: 0 });
    assert_eq!(res, 5);
}

#[test]
fn exit_in_if_in_while_loop() {
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

    let res: i32 = compile_and_run(function.to_string(), &mut MainType { i: 0, ret: 0 });
    assert_eq!(res, 10);
}

#[test]
fn exit_in_for_loop_in_while_loop() {
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
    main := 0;
    WHILE main < 20 DO
        main := main+1;
        FOR i := 0 TO 10 BY 1 DO
            EXIT;
        END_FOR 
    END_WHILE
    main := i + main;
    END_FUNCTION
    "#;

    let res: i32 = compile_and_run(function.to_string(), &mut MainType { i: 0, ret: 0 });
    assert_eq!(res, 20);
}

#[test]
fn continue_in_for_loop_in_while_loop() {
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
    main := 0;
    WHILE main < 20 DO
        main := main+1;
        FOR i := 0 TO 10 BY 1 DO
            CONTINUE;
            main := 200;
        END_FOR 
    END_WHILE
    main := i + main;
    END_FUNCTION
    "#;

    let res: i32 = compile_and_run(function.to_string(), &mut MainType { i: 0, ret: 0 });
    assert_eq!(res, 31);
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

    let res: i32 = compile_and_run(function.to_string(), &mut MainType { i: 0, ret: 0 });
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

    let res: i32 = compile_and_run(function.to_string(), &mut MainType { i: 0, ret: 0 });
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

    let res: i32 = compile_and_run(function.to_string(), &mut MainType { i: 0, ret: 0 });
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
        let res: i32 = compile_and_run(function.to_string(), &mut MainType { i });
        assert_eq!(res, 101);
    });

    (10..19).for_each(|i| {
        let res: i32 = compile_and_run(function.to_string(), &mut MainType { i });
        assert_eq!(res, 201);
    });

    (20..29).for_each(|i| {
        let res: i32 = compile_and_run(function.to_string(), &mut MainType { i });
        assert_eq!(res, 301);
    });

    let res: i32 = compile_and_run(function.to_string(), &mut MainType { i: 999 });
    assert_eq!(res, 7);
}
