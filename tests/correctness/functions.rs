use std::ffi::CStr;

use rusty::codegen::CodegenContext;

// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::*;

#[test]
fn max_function() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        the_a: i16,
        the_b: i16,
        ret: i16,
    }

    let function = r#"

    FUNCTION MAX : INT
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

    PROGRAM main
    VAR
        theA : INT;
        theB : INT;
        ret: INT;
    END_VAR

    ret := MAX(theA, theB);

    END_PROGRAM

    "#
    .to_string();

    let context = CodegenContext::create();
    let module = compile(&context, function);
    let mut case1 = MainType { the_a: 4, the_b: 7, ret: 0 };
    let mut case2 = MainType { the_a: 9, the_b: -2, ret: 0 };

    let _: i32 = module.run("main", &mut case1);
    assert_eq!(case1.ret, 7);
    let _: i32 = module.run("main", &mut case2);
    assert_eq!(case2.ret, 9);
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

    let context = CodegenContext::create();
    let module = compile(&context, function);
    let mut case1 = MainType { x: false };
    let res: i32 = module.run("main", &mut case1);
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

    let context = CodegenContext::create();
    let module = compile(&context, function);
    let mut case1 = MainType { x: false };
    let res: i32 = module.run("main", &mut case1);
    assert_eq!(res, 31);
}

#[test]
fn test_amp_as_and_sideeffects() {
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

        y := AND_BRANCH(FALSE,1) & AND_BRANCH(TRUE,2);
        y := AND_BRANCH(TRUE,10) & AND_BRANCH(FALSE,20) & AND_BRANCH(TRUE,50);
        main := res_and;

    END_FUNCTION

    "#
    .to_string();

    let context = CodegenContext::create();
    let module = compile(&context, function);
    let mut case1 = MainType { x: false };
    let res: i32 = module.run("main", &mut case1);
    assert_eq!(res, 31);
}

#[test]
fn function_block_instances_save_outputs() {
    #[repr(C)]
    struct MainType {
        var: i32,
    }

    let function = r#"
        FUNCTION_BLOCK fb1
            VAR_INPUT
                a : BOOL;
            END_VAR
            VAR_OUTPUT
                b : BOOL;
            END_VAR
            b := a OR b;
        END_FUNCTION_BLOCK

        PROGRAM main
            VAR
                var1 : DINT;
            END_VAR
            VAR_TEMP
                t : BOOL;
                k : BOOL;
                j : fb1;
            END_VAR

            j(a := TRUE, b => t);
            j(a := FALSE, b => k);
            var1 := k;
        END_PROGRAM
    "#;

    let mut interface = MainType { var: 0 };
    let _: i32 = compile_and_run(function.to_string(), &mut interface);

    assert_eq!(1, interface.var);
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

    let mut interface = MainType { f: FooType { i: 4 } };
    let context = CodegenContext::create();
    let module = compile(&context, function.to_string());
    module.run::<_, i32>("main", &mut interface);
    module.run::<_, i32>("main", &mut interface);
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

    let mut interface = MainType { p1: 0, p2: 0, p3: 0 };
    let _: i32 = compile_and_run(function.to_string(), &mut interface);

    assert_eq!(7, interface.p1);
    assert_eq!(8, interface.p2);
    assert_eq!(9, interface.p3);
}

#[test]
fn var_output_assignment() {
    struct MainType {
        var1: i32,
        var2: i32,
    }

    let function = r#"
        PROGRAM foo
            VAR_INPUT
                input1 : DINT;
                input2 : DINT;
            END_VAR
            VAR_OUTPUT
                output1 : DINT;
                output2 : DINT;
            END_VAR
            output1 := input1;
            output2 := input2;
        END_PROGRAM

        PROGRAM main
            VAR
                var1 : DINT;
                var2 : DINT;
            END_VAR
            foo(input1 := 7, input2 := 8, output1 => var1, output2 => var2);
        END_PROGRAM
    "#;

    let mut interface = MainType { var1: 0, var2: 0 };
    let _: i32 = compile_and_run(function.to_string(), &mut interface);

    assert_eq!((7, 8), (interface.var1, interface.var2));
}

#[test]
fn var_output_unassigned() {
    #[repr(C)]
    struct MainType {
        var: i32,
    }

    let function = r#"
        PROGRAM foo
            VAR_OUTPUT
                output1 : DINT;
                output2 : DINT;
            END_VAR
            output1 := 42;
            output2 := output1;
        END_PROGRAM

        PROGRAM main
            VAR
                var1 : DINT;
            END_VAR
            foo(output2 => var1);
        END_PROGRAM
    "#;

    let mut interface = MainType { var: 0 };
    let _: i32 = compile_and_run(function.to_string(), &mut interface);

    assert_eq!(42, interface.var);
}

#[test]
fn var_output_assignment_in_functions() {
    struct MainType {
        var1: i32,
        var2: i32,
    }

    let function = r#"
        FUNCTION foo : INT
            VAR_INPUT
                input1 : DINT;
                input2 : DINT;
            END_VAR
            VAR_OUTPUT
                output1 : DINT;
                output2 : DINT;
            END_VAR
            output1 := input1 + 2;
            output2 := input2 + 3;
        END_FUNCTION

        PROGRAM main
            VAR
                var1 : DINT;
                var2 : DINT;
            END_VAR
            foo(7, 8, output1 => var1, output2 => var2);
        END_PROGRAM
    "#;

    let mut interface = MainType { var1: 0, var2: 0 };
    let _: i32 = compile_and_run(function.to_string(), &mut interface);

    assert_eq!((7 + 2, 8 + 3), (interface.var1, interface.var2));
}

#[test]
fn optional_output_assignment_in_functions() {
    struct MainType {
        var1: i32,
        var2: i32,
    }

    let function = r#"
        FUNCTION foo : INT
            VAR_OUTPUT
                output1 : DINT;
                output2 : DINT;
            END_VAR
            output1 := 1;
            output2 := 2;
        END_FUNCTION

        PROGRAM main
            VAR
                var1 : DINT;
                var2 : DINT;
            END_VAR
            foo(output1 =>, output2 => var2);
        END_PROGRAM
    "#;

    let mut interface = MainType { var1: 0, var2: 0 };
    let _: i32 = compile_and_run(function.to_string(), &mut interface);

    assert_eq!(0, interface.var1);
    assert_eq!(2, interface.var2);
}

#[test]
fn aggregate_var_output_assignment() {
    #[repr(C)]
    struct MainType {
        var1: [u8; 81],
        var2: [i32; 4],
    }

    let main = r#"
        PROGRAM foo
        VAR_OUTPUT
            output1 : STRING;
            output2 : ARRAY[0..3] OF DINT;
        END_VAR
            output1 := 'Hello, world!';
            output2 := [5, 7, 11, 13];
        END_PROGRAM

        PROGRAM main
        VAR
            var1 : STRING;
            var2 : ARRAY[0..3] OF DINT;
        END_VAR
            foo(output1 => var1, output2 => var2);
        END_PROGRAM
    "#;

    let mut interface = MainType { var1: [0; 81], var2: [0; 4] };
    let _: i32 = compile_and_run(main.to_string(), &mut interface);

    let str = CStr::from_bytes_until_nul(&interface.var1).unwrap().to_string_lossy();
    assert_eq!("Hello, world!", str);
    assert_eq!([5, 7, 11, 13], interface.var2);
}

#[test]
fn nested_calls_in_call_statement() {
    #[repr(C)]
    struct MainType {
        var1: i32,
        var2: i32,
    }

    let function = r#"
        FUNCTION seven : DINT
            seven := 7;
        END_FUNCTION

        FUNCTION eight : DINT
            eight := 8;
        END_FUNCTION

        FUNCTION nine : DINT
            nine := 9;
        END_FUNCTION

        FUNCTION sum : DINT
        VAR_INPUT a,b,c : DINT; END_VAR
            sum := a + b + c;
        END_FUNCTION

        PROGRAM main
            VAR
                var1 : DINT;
                var2 : DINT;
            END_VAR

            var1 := sum(seven(), eight(), nine());
            var2 := sum(
                sum(seven(), eight(), nine()),
                sum(seven(), eight(), nine()),
                sum(seven(), eight(), nine()));
        END_PROGRAM
    "#;

    let mut interface = MainType { var1: 0, var2: 0 };
    let _: i32 = compile_and_run(function.to_string(), &mut interface);

    assert_eq!((7 + 8 + 9, 3 * (7 + 8 + 9)), (interface.var1, interface.var2));
}

#[test]
fn nested_calls_passing_aggregate_types() {
    #[repr(C)]
    struct MainType {
        var1: [i32; 2],
    }

    let function = r#"
        TYPE Arr : ARRAY[0..1] OF DINT := [1, 1]; END_TYPE
        FUNCTION getArr : Arr
            getArr[0] := 3;
            getArr[1] := 4;
        END_FUNCTION

        FUNCTION inc : Arr
            VAR_INPUT
                a: Arr;
                index: DINT;
            END_VAR

            a[index] := a[index] + 1;
            inc := a;
        END_FUNCTION

        PROGRAM main
            VAR
                var1 : Arr;
            END_VAR

            var1 := inc(getArr(), 1);
        END_PROGRAM
    "#;

    let mut interface = MainType { var1: [0, 0] };
    let _: i32 = compile_and_run(function.to_string(), &mut interface);

    assert_eq!([3, 5], interface.var1);
}

#[test]
fn by_ref_and_by_val_mixed_function_call() {
    let function = r#"FUNCTION func : DINT
        VAR_INPUT {ref}
            byRef1 : INT;
            byRef2 : DINT;
        END_VAR
        VAR_INPUT
            byVal1 : INT;
            byVal2 : DINT;
        END_VAR
            func := byRef1 + byRef2 + byVal1 + byVal2;
        END_FUNCTION

        FUNCTION main : DINT
            main := func(10,100,1000,10_000); //1 and 2 by ref, 3 and 4 by val
        END_FUNCTION
        "#;

    let res: i32 = compile_and_run(function.to_string(), &mut MainType::default());
    assert_eq!(res, 11_110)
}

#[test]
fn mux_test() {
    let function = r#"
        FUNCTION main : DINT
        VAR
            num,b : DINT := 2;
        END_VAR
            b := num;
            main := MUX(num,b,5,6,7,8); //Result is 6
        END_FUNCTION
        "#;

    let context = CodegenContext::create();
    let module = compile(&context, function);
    let res: i32 = module.run_no_param("main");
    assert_eq!(res, 6)
}

#[test]
fn mux_test_variables() {
    let function = r#"
        FUNCTION main : DINT
        VAR
            num,b,c,d,e,f : DINT;
        END_VAR
            num := 2;
            b := 4;
            c := 5;
            d := 6;
            e := 7;
            f := 8;
            main := MUX(num,b,c,d,e,f); //Result is 6 (d)
        END_FUNCTION
        "#;

    let context = CodegenContext::create();
    let module = compile(&context, function);
    let res: i32 = module.run_no_param("main");
    assert_eq!(res, 6)
}

#[test]
fn mux_array_ref() {
    #[repr(C)]
    #[derive(Default)]
    struct MainType {
        res: [i32; 3],
    }

    let function = r#"
    PROGRAM main
    VAR
        arr1 : ARRAY[0..2] OF DINT;
    END_VAR
    VAR_TEMP
        arr2 : ARRAY[0..2] OF DINT := (0, 1, 2);
        arr3 : ARRAY[0..2] OF DINT := (3, 4, 5);
        arr4 : ARRAY[0..2] OF DINT := (6, 7, 8);
        arr5 : ARRAY[0..2] OF DINT := (9, 9, 9);
    END_VAR
        arr1 := MUX(2, arr2, arr3, arr4, arr5); // arr4
    END_PROGRAM
    "#;

    let mut main = MainType::default();
    let _: i32 = compile_and_run(function.to_string(), &mut main);
    assert_eq!(main.res, [6, 7, 8]);
}

#[test]
fn mux_struct_ref() {
    #[repr(C)]
    #[derive(Default)]
    struct MainType {
        res: MyStruct,
    }

    #[repr(C)]
    #[derive(Default)]
    struct MyStruct {
        a: bool,
        b: bool,
    }

    let function = r#"
    PROGRAM main
    VAR
        struct1 : myStruct;
    END_VAR
    VAR_TEMP
        struct2 : myStruct := (a := FALSE, b := FALSE);
        struct3 : myStruct := (a := FALSE, b := TRUE);
        struct4 : myStruct := (a := TRUE, b := FALSE);
        struct5 : myStruct := (a := TRUE, b := TRUE);
    END_VAR
        struct1 := MUX(2, struct2, struct3, struct4, struct5); // struct4
    END_PROGRAM

    TYPE myStruct : STRUCT
        a : BOOL;
        b : BOOL;
    END_STRUCT
    END_TYPE
    "#;

    let mut main = MainType::default();
    let _: i32 = compile_and_run(function.to_string(), &mut main);
    assert_eq!((true, false), (main.res.a, main.res.b));
}

#[test]
fn mux_string_ref() {
    #[repr(C)]
    #[derive(Default)]
    struct MainType {
        res: [u8; 6],
    }

    let function = r#"
    PROGRAM main
    VAR
        str1 : STRING[5];
    END_VAR
    VAR_TEMP
        str2 : STRING := 'str2 ';
        str3 : STRING := 'str3 ';
        str4 : STRING := 'str4 ';
        str5 : STRING := 'str5 ';
        str6 : STRING := 'str6 ';
    END_VAR
        str1 := MUX(3, str2, str3, str4, str5, str6); // str5
    END_PROGRAM
    "#;

    let mut main = MainType::default();
    let _: i32 = compile_and_run(function.to_string(), &mut main);
    assert_eq!(main.res, "str5 \0".as_bytes());
}

#[test]
fn mux_string_literal() {
    #[repr(C)]
    #[derive(Default)]
    struct MainType {
        res: [u8; 4],
    }

    let function = r#"
    PROGRAM main
    VAR
        str1 : STRING[3];
    END_VAR
        str1 := MUX(3, 'hello', 'world', 'foo', 'baz'); // baz
    END_PROGRAM
    "#;

    let mut main = MainType::default();
    let _: i32 = compile_and_run(function.to_string(), &mut main);
    assert_eq!(main.res, "baz\0".as_bytes());
}

#[test]
fn sel_test_false() {
    let function = r#"
        FUNCTION main : DINT
            main := SEL(FALSE,4,5); //Result is 4
        END_FUNCTION
        "#;

    let context = CodegenContext::create();
    let module = compile(&context, function);
    let res: i32 = module.run_no_param("main");
    assert_eq!(res, 4)
}

#[test]
fn sel_test_true() {
    let function = r#"
        FUNCTION main : DINT
            main := SEL(TRUE,4,5); //Result is 5
        END_FUNCTION
        "#;

    let context = CodegenContext::create();
    let module = compile(&context, function);
    let res: i32 = module.run_no_param("main");
    assert_eq!(res, 5)
}

#[test]
fn sel_test_true_vars() {
    let function = r#"
        FUNCTION main : DINT
        VAR a,b : DINT; END_VAR
            a := 4;
            b := 5;
            main := SEL(TRUE,a,b); //Result is 5
        END_FUNCTION
        "#;

    let context = CodegenContext::create();
    let module = compile(&context, function);
    let res: i32 = module.run_no_param("main");
    assert_eq!(res, 5)
}

#[test]
fn sel_expression_test() {
    let function = r#"
        FUNCTION main : DINT
        VAR a,b : DINT; END_VAR
            a := 4;
            b := 5;
            main := SEL(TRUE,a,b) + 10; //Result is 15
        END_FUNCTION
        "#;

    let context = CodegenContext::create();
    let module = compile(&context, function);
    let res: i32 = module.run_no_param("main");
    assert_eq!(res, 15);
}

#[test]
fn sel_struct_ref() {
    #[repr(C)]
    #[derive(Default)]
    struct MainType {
        res: MyStruct,
    }

    #[repr(C)]
    #[derive(Default)]
    struct MyStruct {
        a: bool,
        b: bool,
    }

    let function = r#"
    PROGRAM main
    VAR
        struct1 : myStruct;
    END_VAR
    VAR_TEMP
        struct2 : myStruct := (a := TRUE, b := FALSE);
        struct3 : myStruct := (a := FALSE, b := TRUE);
    END_VAR
        struct1 := SEL(TRUE, struct2, struct3); // struct3
    END_PROGRAM

    TYPE myStruct : STRUCT
        a : BOOL;
        b : BOOL;
    END_STRUCT
    END_TYPE
    "#;

    let mut main = MainType::default();
    let _: i32 = compile_and_run(function.to_string(), &mut main);
    assert!(!main.res.a);
    assert!(main.res.b);
}

#[test]
fn sel_array_ref() {
    #[repr(C)]
    #[derive(Default)]
    struct MainType {
        res: [i32; 3],
    }

    let function = r#"
    PROGRAM main
    VAR
        arr1 : ARRAY[0..2] OF DINT;
    END_VAR
    VAR_TEMP
        arr2 : ARRAY[0..2] OF DINT := (0, 1, 2);
        arr3 : ARRAY[0..2] OF DINT := (3, 4, 5);
    END_VAR
        arr1 := SEL(TRUE, arr2, arr3); // arr3
    END_PROGRAM
    "#;

    let mut main = MainType::default();
    let _: i32 = compile_and_run(function.to_string(), &mut main);
    assert_eq!(main.res, [3, 4, 5]);
}

#[test]
fn sel_string_ref() {
    #[repr(C)]
    #[derive(Default)]
    struct MainType {
        res: [u8; 6],
    }

    let function = r#"
    PROGRAM main
    VAR
        str1 : STRING[5];
    END_VAR
    VAR_TEMP
        str2 : STRING := 'hello';
        str3 : STRING := 'world';
    END_VAR
        str1 := SEL(TRUE, str2, str3); // str3
    END_PROGRAM
    "#;

    let mut main = MainType::default();
    let _: i32 = compile_and_run(function.to_string(), &mut main);
    assert_eq!(main.res, "world\0".as_bytes());
}

#[test]
fn sel_string_literal() {
    #[repr(C)]
    #[derive(Default)]
    struct MainType {
        res: [u8; 6],
    }

    let function = r#"
    PROGRAM main
    VAR
        str1 : STRING[5];
    END_VAR
        str1 := SEL(TRUE, 'hello', 'world'); // world
    END_PROGRAM
    "#;

    let mut main = MainType::default();
    let _: i32 = compile_and_run(function.to_string(), &mut main);
    assert_eq!(main.res, "world\0".as_bytes());
}

#[test]
fn move_test() {
    let function = r#"
        FUNCTION main : DINT
        VAR a : DINT; END_VAR
            a := 4;
            main := MOVE(a); //Result is 4
        END_FUNCTION
        "#;

    let context = CodegenContext::create();
    let module = compile(&context, function);
    let res: i32 = module.run_no_param("main");
    assert_eq!(res, 4)
}

#[test]
#[ignore = "variable sized arrays not yet implemented"]
fn sizeof_len() {
    let src = r#"
    PROGRAM main
    VAR
        y : ARRAY[0..13] OF INT;
    END_VAR
        len(y);
    END_PROGRAM

    FUNCTION len : DINT
    VAR_INPUT
        arr : ARRAY[*, *] OF INT;
    END_VAR
        len := SIZEOF(arr) / SIZEOFF(arr(0));
    END_FUNCTION
    "#;

    let context = CodegenContext::create();
    let module = compile(&context, src);
    let res: i32 = module.run_no_param("main");

    assert_eq!(13, res);
}

#[test]
fn argument_passed_by_ref_then_by_val() {
    #[repr(C)]
    struct MainType {
        arr: [i32; 5],
    }

    let source = r"
    TYPE MyType : ARRAY[1..5] OF DINT; END_TYPE

    PROGRAM main
        VAR
            arr : MyType;
        END_VAR

        fn_by_ref(arr);
    END_PROGRAM

    FUNCTION fn_by_ref : DINT
        VAR_IN_OUT
            arg_by_ref : MyType;
        END_VAR

        // These SHOULD modify the underlying array passed from main
        arg_by_ref[1] := 1;
        arg_by_ref[2] := 2;
        arg_by_ref[3] := 3;
        arg_by_ref[4] := 4;
        arg_by_ref[5] := 5;

        fn_by_val(arg_by_ref);
    END_FUNCTION

    FUNCTION fn_by_val : DINT
        VAR_INPUT
            arg_by_val : MyType;
        END_VAR

        // These should NOT modify the underlying array passed from main
        arg_by_val[1] := 10;
        arg_by_val[2] := 20;
        arg_by_val[3] := 30;
        arg_by_val[4] := 40;
        arg_by_val[5] := 50;
    END_FUNCTION
    ";

    let mut maintype = MainType { arr: [0; 5] };
    let _: i32 = compile_and_run(source, &mut maintype);

    assert_eq!(maintype.arr, [1, 2, 3, 4, 5]);
}

#[test]
fn function_with_optional_params() {
    #[repr(C)]
    struct MainType {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }
    let function = r#"
        FUNCTION foo : DINT
        VAR_INPUT
            a : DINT := 1;
            b : DINT := 2;
            c : DINT := 3;
        END_VAR
            foo := a + b + c;
        END_FUNCTION

        PROGRAM main
        VAR
            a,b,c,d : DINT;
        END_VAR
            a := foo(); // 1 + 2 + 3
            b := foo(10); // 10 + 2 + 3
            c := foo(10, 20); // 10 + 20 + 3
            d := foo(10, 20, 30); // 10 + 20 + 30
        END_PROGRAM
        "#;

    let mut maintype = MainType { a: 0, b: 0, c: 0, d: 0 };
    let _: i32 = compile_and_run(function.to_string(), &mut maintype);
    assert_eq!(maintype.a, 6);
    assert_eq!(maintype.b, 15);
    assert_eq!(maintype.c, 33);
    assert_eq!(maintype.d, 60);
}
