// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::super::*;

#[test]
fn max_function() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        the_a: i16,
        the_b: i16,
    }

    let function = r#"

    FUNCTION MAX : DINT 
    VAR_INPUT 
        a : INT;
        b : INT;
    END_VAR

    IF a > b THEN
        MAX := a;
    ELSE 
        MAX := b;
    END_IF
    END_FUNCTION

    FUNCTION main : DINT
    VAR_INPUT
        theA : INT;
        theB : INT;
    END_VAR

    main := MAX(theA, theB);

    END_FUNCTION

    "#
    .to_string();

    let context: Context = Context::create();
    let engine = compile(&context, function);
    let mut case1 = MainType { the_a: 4, the_b: 7 };
    let mut case2 = MainType {
        the_a: 9,
        the_b: -2,
    };

    let res: i32 = run(&engine, "main", &mut case1);
    assert_eq!(res, 7);
    let res: i32 = run(&engine, "main", &mut case2);
    assert_eq!(res, 9);
}

#[test]
fn nested_function_call() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {}

    let mut main_data = MainType {};

    let function = r#"
            FUNCTION bar : DINT
            VAR_INPUT
                x : DINT;
            END_VAR
                bar := x;
            END_FUNCTION

            FUNCTION foo : DINT
            VAR_INPUT
                y : DINT;
            END_VAR
                foo := y;
            END_FUNCTION


            FUNCTION main : DINT
                main := foo(bar(1000));
            END_FUNCTION
        "#;

    let res: i32 = compile_and_run(function.to_string(), &mut main_data);
    assert_eq!(1000, res);
}

#[test]
fn test_or_sideeffects() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x: bool,
    }

    let function = r#"
    VAR_GLOBAL
        res_or : INT;
    END_VAR

    FUNCTION OR_BRANCH : BOOL 
    VAR_INPUT 
        a : BOOL;
        b : INT;
    END_VAR

    OR_BRANCH := a;
    res_or := res_or + b;

    END_FUNCTION

    FUNCTION main : DINT
    VAR
        x : BOOL;
    END_VAR

    x := OR_BRANCH(TRUE,1) OR OR_BRANCH(FALSE,2);
    x := OR_BRANCH(FALSE,10) OR OR_BRANCH(TRUE,20) OR OR_BRANCH(FALSE,50);
    main := res_or;

    END_FUNCTION

    "#
    .to_string();

    let context: Context = Context::create();
    let engine = compile(&context, function);
    let mut case1 = MainType { x: false };
    let res: i32 = run(&engine, "main", &mut case1);
    assert_eq!(res, 31);
}

#[test]
fn test_and_sideeffects() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x: bool,
    }

    let function = r#"
    VAR_GLOBAL
        res_and : INT;
    END_VAR

    FUNCTION AND_BRANCH : BOOL 
    VAR_INPUT 
        a : BOOL;
        b : INT;
    END_VAR

    AND_BRANCH := a;
    res_and := res_and + b;

    END_FUNCTION

    FUNCTION main : DINT
    VAR
        y : BOOL;
    END_VAR

    y := AND_BRANCH(FALSE,1) AND AND_BRANCH(TRUE,2);
    y := AND_BRANCH(TRUE,10) AND AND_BRANCH(FALSE,20) AND AND_BRANCH(TRUE,50);
    main := res_and;

    END_FUNCTION

    "#
    .to_string();

    let context: Context = Context::create();
    let engine = compile(&context, function);
    let mut case1 = MainType { x: false };
    let res: i32 = run(&engine, "main", &mut case1);
    assert_eq!(res, 31);
}

#[test]
fn function_block_instances_save_state_per_instance() {
    #[allow(dead_code)]
    #[repr(C)]
    struct FooType {
        i: i16,
    }

    struct MainType {
        f: FooType,
        j: FooType,
    }
    let function = r#"
    FUNCTION_BLOCK foo
    VAR_INPUT
        i : INT;
    END_VAR
    i := i + 1;
    END_FUNCTION_BLOCK

    PROGRAM main
    VAR 
        f : foo;
        j : foo;
    END_VAR 
    f();
    f();
    j(4);
    j();
    j();
    END_PROGRAM
    "#;

    let mut interface = MainType {
        f: FooType { i: 0 },
        j: FooType { i: 0 },
    };
    let _: i32 = compile_and_run(function.to_string(), &mut interface);
    assert_eq!(interface.f.i, 2);
    assert_eq!(interface.j.i, 7);
}
#[test]
fn program_instances_save_state_per() {
    #[allow(dead_code)]
    #[repr(C)]
    struct FooType {
        i: i16,
    }

    struct MainType {
        f: FooType,
    }
    let function = r#"
    PROGRAM main
    VAR_INPUT
        i : INT;
    END_VAR
    i := i + 1;
    END_PROGRAM
    "#;

    let mut interface = MainType {
        f: FooType { i: 4 },
    };
    let context = inkwell::context::Context::create();
    let exec_engine = compile(&context, function.to_string());
    run::<_, i32>(&exec_engine, "main", &mut interface);
    run::<_, i32>(&exec_engine, "main", &mut interface);
    assert_eq!(interface.f.i, 6);
}

#[test]
fn functions_can_be_called_out_of_order() {
    struct MainType {
        f: i16,
    }
    let function = r#"

    FUNCTION foo : INT
      foo := bar();
    END_FUNCTION

    FUNCTION bar : INT
        bar := 7;
    END_FUNCTION

    PROGRAM main
        VAR 
            r : INT;
        END_VAR 
        r:= foo();
    END_PROGRAM
    "#;

    let mut interface = MainType { f: 0 };
    let _: i32 = compile_and_run(function.to_string(), &mut interface);

    assert_eq!(7, interface.f);
}

#[test]
fn function_block_instances_save_state_per_instance_2() {
    #[allow(dead_code)]
    #[repr(C)]
    struct BazType {
        i: i16,
    }

    #[allow(dead_code)]
    #[repr(C)]
    struct FooType {
        i: i16,
        baz: BazType,
    }

    struct MainType {
        f: FooType,
        j: FooType,
    }
    let function = r#"
    FUNCTION_BLOCK Baz 
    VAR_INPUT
        i : INT;
    END_VAR
    i := i+1;
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK foo
    VAR_INPUT
        i : INT;
        baz: Baz; 
    END_VAR
    
    END_FUNCTION_BLOCK

    PROGRAM main
    VAR 
        f : foo;
        j : foo;
    END_VAR 
    f.baz.i := f.baz.i + 1;
    f.baz.i := f.baz.i + 1;


    j.baz.i := j.baz.i + 1;
    j.baz.i := j.baz.i + 1;
    j.baz.i := j.baz.i + 1;
    j.baz.i := j.baz.i + 1;
    END_PROGRAM
    "#;

    let mut interface = MainType {
        f: FooType {
            i: 0,
            baz: BazType { i: 0 },
        },
        j: FooType {
            i: 0,
            baz: BazType { i: 0 },
        },
    };
    let _: i32 = compile_and_run(function.to_string(), &mut interface);

    assert_eq!(2, interface.f.baz.i);
    assert_eq!(4, interface.j.baz.i);
}

#[test]
fn function_call_inout_variable() {
    #[repr(C)]
    struct MainType {
        baz: i32,
    }
    let function = r#"
        PROGRAM multiply
            VAR_IN_OUT
                param: DINT;
            END_VAR
            VAR_INPUT
                factor : DINT;
            END_VAR

            param := param * factor;
        END_PROGRAM

        PROGRAM foo 
            VAR_IN_OUT
            inout : DINT;
            END_VAR

            inout := inout + 1;
            multiply(param := inout, factor := inout);
        END_PROGRAM

        PROGRAM main
            VAR
                baz : DINT;
            END_VAR
            foo(inout := baz);
        END_PROGRAM
    "#;

    let mut interface = MainType { baz: 7 };
    let _: i32 = compile_and_run(function.to_string(), &mut interface);

    assert_eq!(64, interface.baz);
}

#[test]
fn inouts_behave_like_pointers() {
    #[repr(C)]
    struct MainType {
        p1: i32,
        p2: i32,
        p3: i32,
    }
    let function = r#"
        VAR_GLOBAL 
            snap1 : DINT;
            snap2 : DINT;
            snap3 : DINT;
        END_VAR

        PROGRAM takeSnapshot
            VAR_IN_OUT
                param: DINT;
            END_VAR
            VAR_INPUT
                data : DINT;
            END_VAR
            param := data;
        END_PROGRAM

        PROGRAM main
            VAR
                p1 : DINT;
                p2 : DINT;
                p3 : DINT;
            END_VAR

            takeSnapshot(param := snap1, data := 7);
            p1 := snap1;
            takeSnapshot(param := snap2, data := 8);
            p2 := snap2;
            takeSnapshot(param := snap3, data := 9);
            p3 := snap3;
        END_PROGRAM
    "#;

    let mut interface = MainType {
        p1: 0,
        p2: 0,
        p3: 0,
    };
    let _: i32 = compile_and_run(function.to_string(), &mut interface);

    assert_eq!(7, interface.p1);
    assert_eq!(8, interface.p2);
    assert_eq!(9, interface.p3);
}
