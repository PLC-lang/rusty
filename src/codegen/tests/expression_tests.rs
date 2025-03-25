// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::test_utils::tests::codegen;

#[test]
fn pointers_in_function_return() {
    let result = codegen(
        r#"FUNCTION func : REF_TO INT
        END_FUNCTION"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn structs_in_function_return() {
    let result = codegen(
        r#"
        TYPE myStruct : STRUCT
            x : INT;
            END_STRUCT
        END_TYPE
        FUNCTION func : myStruct
            VAR_OUTPUT
                xxx : myStruct;
            END_VAR
        END_FUNCTION"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn strings_in_function_return() {
    let result = codegen(
        r#"
       FUNCTION func : STRING
            VAR_INPUT
                myout : REF_TO STRING;
            END_VAR

            func := 'hello';
            myout^ := 'hello';
       END_FUNCTION"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn calling_strings_in_function_return() {
    let result = codegen(
        r#"
       FUNCTION func : STRING
            func := 'hello';
       END_FUNCTION

       PROGRAM main
            VAR
                x : STRING;
            END_VAR

            x := func();
       END_PROGRAM
       "#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn unary_expressions_can_be_real() {
    let result = codegen(
        r#"
            PROGRAM prg
            VAR
                a,b : REAL;
            END_VAR
                b := -2.0;
                a := -b;
            END_PROGRAM
        "#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn type_mix_in_call() {
    let result = codegen(
        "
        FUNCTION foo : INT
        VAR_INPUT
            in : INT;
        END_VAR
        END_FUNCTION
        FUNCTION baz : INT
            foo(1.5);
        END_FUNCTION
    ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn cast_pointer_to_lword() {
    let result = codegen(
        r#"
        FUNCTION baz : INT
            VAR
                ptr_x : POINTER TO INT;
                y : LWORD;
            END_VAR;

            y := ptr_x;
        END_FUNCTION
    "#,
    );

    //should result in normal number-comparisons
    insta::assert_snapshot!(result);
}

#[test]
fn cast_lword_to_pointer() {
    let result = codegen(
        r#"
        FUNCTION baz : INT
            VAR
                ptr_x : POINTER TO INT;
                y : LWORD;
            END_VAR;

            ptr_x := y;
        END_FUNCTION
    "#,
    );

    //should result in normal number-comparisons
    insta::assert_snapshot!(result);
}

#[test]
fn cast_between_pointer_types() {
    let result = codegen(
        r#"
        PROGRAM baz
            VAR
                ptr_x : POINTER TO BYTE;
                y : WORD;
            END_VAR;

            ptr_x := REF(y);
        END_PROGRAM
    "#,
    );

    //should result in bitcast conversion when assigning to ptr_x
    insta::assert_snapshot!(result);
}

#[test]
fn unnecessary_casts_between_pointer_types() {
    let result = codegen(
        r#"
        TYPE MyByte : BYTE; END_TYPE

        PROGRAM baz
            VAR
                ptr : POINTER TO BYTE;
                b : BYTE;
                si : SINT;
                mb : MyByte;
            END_VAR;

            ptr := REF(b); //no cast necessary
            ptr := REF(si); //no cast necessary
            ptr := REF(mb); //no cast necessary
        END_PROGRAM
    "#,
    );

    //should not result in bitcast
    insta::assert_snapshot!(result);
}

#[test]
fn access_string_via_byte_array() {
    let result = codegen(
        r#"
        TYPE MyByte : BYTE; END_TYPE

        PROGRAM baz
            VAR
                str: STRING[10];
                ptr : POINTER TO BYTE;
                bytes : POINTER TO ARRAY[0..9] OF BYTE;
            END_VAR;

            ptr := REF(str); //bit-cast expected
            bytes := REF(str);
        END_PROGRAM
    "#,
    );

    //should result in bitcasts
    insta::assert_snapshot!(result);
}

#[test]
fn pointer_arithmetics() {
    // codegen should be successful for binary expression for pointer<->int / int<->pointer / pointer<->pointer
    let result = codegen(
        "
        PROGRAM main
        VAR
            x : INT := 10;
            y : INT := 20;
            pt : REF_TO INT;
        END_VAR
        pt := REF(x);

        (* +/- *)
        pt := pt + 1;
        pt := pt + 1 + 1;
        pt := 1 + pt;
        pt := pt - y;
        pt := 1 + pt + 1;
        pt := pt - y - 1;
        pt := 1 + 1 + pt ;
        pt := y + pt - y ;
        pt := y + y + pt ;
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn pointer_arithmetics_function_call() {
    // codegen should be successful for binary expression for pointer<->int / int<->pointer / pointer<->pointer
    let result = codegen(
        "
        FUNCTION foo : LINT
        END_FUNCTION

        PROGRAM main
        VAR
            pt : REF_TO INT;
            x : INT;
        END_VAR
        pt := REF(x);

        (* +/- *)
        pt := pt + foo();
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn nested_call_statements() {
    // GIVEN some nested call statements
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            a : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
            foo(foo(2));
        END_PROGRAM
        ",
    );
    // WHEN compiled
    // WE expect a flat sequence of calls, no regions and branching
    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_adr() {
    // GIVEN some nested call statements
    let result = codegen(
        "
        PROGRAM main
        VAR
            a : REF_TO DINT;
            b : DINT;
        END_VAR
            a := ADR(b);
        END_PROGRAM
        ",
    );
    // WHEN compiled
    // We expect the same behaviour as if REF was called, due to the assignee being a pointer
    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_ref() {
    // GIVEN some nested call statements
    let result = codegen(
        "
        PROGRAM main
        VAR
            a : REF_TO DINT;
            b : DINT;
        END_VAR
            a := REF(b);
        END_PROGRAM
        ",
    );
    // WHEN compiled
    // We expect a direct conversion and subsequent assignment to pointer(no call)
    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_mux() {
    let result = codegen(
        "PROGRAM main
        VAR
            a,b,c,d,e : DINT;
        END_VAR
            a := MUX(3, b,c,d,e); // 3 = e
        END_PROGRAM",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_mux_with_aggregate_type() {
    let result = codegen(
        "PROGRAM main
        VAR
            str1 : STRING;
        END_VAR
            str1 := MUX(3, 'lorem', 'ipsum', 'dolor', 'sit'); // 3 = sit
        END_PROGRAM",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_sel() {
    let result = codegen(
        "PROGRAM main
        VAR
            a,b,c : DINT;
        END_VAR
            a := SEL(TRUE, b,c);
        END_PROGRAM",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_sel_as_expression() {
    let result = codegen(
        "PROGRAM main
        VAR
            a,b,c : DINT;
        END_VAR
            a := SEL(TRUE, b,c) + 10;
        END_PROGRAM",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_move() {
    let result = codegen(
        "PROGRAM main
        VAR
            a,b : DINT;
        END_VAR
            a := MOVE(b);
        END_PROGRAM",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_sizeof() {
    let result = codegen(
        "PROGRAM main
        VAR
            a: DINT;
            b: LINT;
        END_VAR
            a := SIZEOF(b);
        END_PROGRAM",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_lower_bound() {
    let result = codegen(
        "PROGRAM main
        VAR
            a: ARRAY[0..1] OF DINT;
            b: DINT;
        END_VAR
            b := foo(a);
        END_PROGRAM

        FUNCTION foo : DINT
        VAR_IN_OUT
            vla: ARRAY[*] OF DINT;
        END_VAR
            foo := LOWER_BOUND(vla, 1);
        END_VAR
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_upper_bound() {
    let result = codegen(
        "PROGRAM main
        VAR
            a: ARRAY[0..1] OF DINT;
            b: DINT;
        END_VAR
            b := foo(a);
        END_PROGRAM

        FUNCTION foo : DINT
        VAR_IN_OUT
            vla: ARRAY[*] OF DINT;
        END_VAR
            foo := UPPER_BOUND(vla, 1);
        END_VAR
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_upper_bound_expr() {
    let result = codegen(
        "
        VAR_GLOBAL CONSTANT
            MY_CONST : DINT := 10;
        END_VAR
        PROGRAM main
        VAR
            a: ARRAY[0..1, 1..2, 2..3, 3..4] OF DINT;
            b: DINT;
        END_VAR
            b := foo(a);
        END_PROGRAM

        FUNCTION foo : DINT
        VAR_IN_OUT
            vla: ARRAY[*] OF DINT;
        END_VAR
            // upper bound of 4th dimension => 8th element in dimension array
            foo := UPPER_BOUND(vla, MY_CONST - (2 * 3));
        END_VAR
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn test_max_int() {
    let result = codegen(
        r"
    {external}
    FUNCTION MAX<U : ANY> : U
    VAR_INPUT in : {sized} U...; END_VAR
    END_FUNCTION

    FUNCTION main : INT
    main := MAX(INT#5,INT#2,INT#1,INT#3,INT#4,INT#7,INT#-1);
    END_FUNCTION",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn compare_date_time_literals() {
    let result = codegen(
        "
    PROGRAM main
    VAR_TEMP
        cmp1, cmp2, cmp3, cmp4, cmp5, cmp6, cmp7, cmp8 : BOOL;
    END_VAR
        cmp1 := TIME#2d4h6m8s10ms11us300ns < TIME#1d8h43m23s55ms;
        cmp2 := LTIME#2d4h6m8s10ms11us300ns > LTIME#1d8h43m23s55ms;

        cmp3 := TOD#23:59:59.999 < TOD#10:32:59;
        cmp4 := LTOD#23:59:59.999 > LTOD#10:32:59;

        cmp5 := DATE#2022-10-20 < DATE#1999-01-01;
        cmp6 := LDATE#2022-10-20 > LDATE#1999-01-01;

        cmp7 := DT#2022-10-20-23:59:59.999 < DT#1999-01-01-10:32;
        cmp8 := LDT#2022-10-20-23:59:59.999 > LDT#1999-01-01-10:32;
    END_PROGRAM
    ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn allowed_assignable_types() {
    let result = codegen(
        r#"
        PROGRAM main
        VAR
            v : INT;
            x : ARRAY[0..1] OF INT;
            y : REF_TO INT;
            z : REF_TO ARRAY[0..1] OF INT;
        END_VAR
            v := 0;
            x[0] := 1;
            y^ := 2;
            y^.1 := 3;
            z^[0] := 4;
            z^[1].1 := 5;
        END_PROGRAM
        "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn builtin_add_ints() {
    let src = r#"
        FUNCTION main : DINT
        VAR
            x1, x2, x3 : DINT;
            l1 : LINT;
            s1 : SINT;
        END_VAR
            ADD(x1, x2, x3, l1, s1);
        END_FUNCTION
    "#;

    let res = codegen(src);

    insta::assert_snapshot!(res);
}

#[test]
fn builtin_add_float() {
    let src = r#"
        FUNCTION main : DINT
        VAR
            x1, x2, x3 : REAL;
            l1 : LREAL;
        END_VAR
            ADD(x1, x2, x3, l1);
        END_FUNCTION
    "#;

    let res = codegen(src);

    insta::assert_snapshot!(res);
}

#[test]
fn builtin_add_mixed() {
    let src = r#"
        FUNCTION main : DINT
        VAR
            x1, x2, x3 : REAL;
            l1 : LINT;
        END_VAR
            ADD(x1, x2, x3, l1);
        END_FUNCTION
    "#;

    let res = codegen(src);

    insta::assert_snapshot!(res);
}

#[test]
fn builtin_mul_ints() {
    let src = r#"
        FUNCTION main : DINT
        VAR
            x1, x2, x3 : DINT;
            l1 : LINT;
            s1 : SINT;
        END_VAR
            MUL(x1, x2, x3, l1, s1);
        END_FUNCTION
    "#;

    let res = codegen(src);

    insta::assert_snapshot!(res);
}

#[test]
fn builtin_mul_float() {
    let src = r#"
        FUNCTION main : DINT
        VAR
            x1, x2, x3 : REAL;
            l1 : LREAL;
        END_VAR
            MUL(x1, x2, x3, l1);
        END_FUNCTION
    "#;

    let res = codegen(src);

    insta::assert_snapshot!(res);
}

#[test]
fn builtin_mul_mixed() {
    let src = r#"
        FUNCTION main : DINT
        VAR
            x1, x2, x3 : REAL;
            l1 : LINT;
        END_VAR
            MUL(x1, x2, x3, l1);
        END_FUNCTION
    "#;

    let res = codegen(src);

    insta::assert_snapshot!(res);
}

#[test]
fn builtin_sub_ints() {
    let src = r#"
        FUNCTION main : DINT
        VAR
            x1 : DINT;
            l1 : LINT;
        END_VAR
            SUB(x1, l1);
        END_FUNCTION
    "#;

    let res = codegen(src);

    insta::assert_snapshot!(res);
}

#[test]
fn builtin_sub_float() {
    let src = r#"
        FUNCTION main : DINT
        VAR
            x1 : REAL;
            l1 : LREAL;
        END_VAR
            SUB(x1, l1);
        END_FUNCTION
    "#;

    let res = codegen(src);

    insta::assert_snapshot!(res);
}

#[test]
fn builtin_sub_mixed() {
    let src = r#"
        FUNCTION main : DINT
        VAR
            x1 : REAL;
            l1 : LINT;
        END_VAR
            SUB(x1, l1);
        END_FUNCTION
    "#;

    let res = codegen(src);

    insta::assert_snapshot!(res);
}

#[test]
fn builtin_div_ints() {
    let src = r#"
        FUNCTION main : DINT
        VAR
            x1 : DINT;
            l1 : LINT;
        END_VAR
            DIV(x1, l1);
        END_FUNCTION
    "#;

    let res = codegen(src);

    insta::assert_snapshot!(res);
}

#[test]
fn builtin_div_float() {
    let src = r#"
        FUNCTION main : DINT
        VAR
            x1 : REAL;
            l1 : LREAL;
        END_VAR
            DIV(x1, l1);
        END_FUNCTION
    "#;

    let res = codegen(src);

    insta::assert_snapshot!(res);
}

#[test]
fn builtin_div_mixed() {
    let src = r#"
        FUNCTION main : DINT
        VAR
            x1 : REAL;
            l1 : LINT;
        END_VAR
            DIV(x1, l1);
        END_FUNCTION
    "#;

    let res = codegen(src);

    insta::assert_snapshot!(res);
}

#[test]
fn global_namespace_operator() {
    let src = r#"
    VAR_GLOBAL
        foo : DINT;
    END_VAR

    PROGRAM main
    VAR
        foo : DINT;
    END_VAR
        foo := .foo;
        foo := .foo + 1;
        foo := .foo + .foo;

        .foo := foo;
        .foo := .foo + 1;
    END_PROGRAM
    "#;

    let res = codegen(src);
    insta::assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %main = type { i32 }

    @foo = global i32 0
    @main_instance = global %main zeroinitializer

    define void @main(%main* %0) {
    entry:
      %foo = getelementptr inbounds %main, %main* %0, i32 0, i32 0
      %load_ = load i32, i32* @foo, align 4
      store i32 %load_, i32* %foo, align 4
      %load_1 = load i32, i32* @foo, align 4
      %tmpVar = add i32 %load_1, 1
      store i32 %tmpVar, i32* %foo, align 4
      %load_2 = load i32, i32* @foo, align 4
      %load_3 = load i32, i32* @foo, align 4
      %tmpVar4 = add i32 %load_2, %load_3
      store i32 %tmpVar4, i32* %foo, align 4
      %load_foo = load i32, i32* %foo, align 4
      store i32 %load_foo, i32* @foo, align 4
      %load_5 = load i32, i32* @foo, align 4
      %tmpVar6 = add i32 %load_5, 1
      store i32 %tmpVar6, i32* @foo, align 4
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %main = type { i32 }

    @main_instance = external global %main

    define void @__init_main(%main* %0) {
    entry:
      %self = alloca %main*, align 8
      store %main* %0, %main** %self, align 8
      ret void
    }

    declare void @main(%main*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %main = type { i32 }

    @main_instance = external global %main
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      call void @__init_main(%main* @main_instance)
      ret void
    }

    declare void @__init_main(%main*)

    declare void @main(%main*)
    "#);
}
