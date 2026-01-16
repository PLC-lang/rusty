// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::test_utils::tests::codegen;
use plc_util::filtered_assert_snapshot;
#[test]
fn pointers_in_function_return() {
    let result = codegen(
        r#"FUNCTION func : REF_TO INT
        END_FUNCTION"#,
    );
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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

    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
            a := ADR(IN := b);
        END_PROGRAM
        ",
    );
    // WHEN compiled
    // We expect the same behaviour as if REF was called, due to the assignee being a pointer
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %main = type { ptr, i32 }

    @main_instance = global %main zeroinitializer

    define void @main(ptr %0) {
    entry:
      %a = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 1
      store ptr %b, ptr %a, align 8
      store ptr %b, ptr %a, align 8
      ret void
    }
    "#);
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
            a := REF(IN := b);
        END_PROGRAM
        ",
    );
    // WHEN compiled
    // We expect a direct conversion and subsequent assignment to pointer(no call)
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %main = type { ptr, i32 }

    @main_instance = global %main zeroinitializer

    define void @main(ptr %0) {
    entry:
      %a = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 1
      store ptr %b, ptr %a, align 8
      store ptr %b, ptr %a, align 8
      ret void
    }
    "#);
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

    filtered_assert_snapshot!(result);
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

    filtered_assert_snapshot!(result);
}

#[test]
fn builtin_function_call_sel() {
    let result = codegen(
        "PROGRAM main
        VAR
            a,b,c : DINT;
        END_VAR
            a := SEL(TRUE, b,c);
            a := SEL(G := TRUE, IN0 := b, IN1 := c);
        END_PROGRAM",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %main = type { i32, i32, i32 }

    @main_instance = global %main zeroinitializer

    define void @main(ptr %0) {
    entry:
      %a = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 1
      %c = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 2
      %load_b = load i32, ptr %b, align 4
      %load_c = load i32, ptr %c, align 4
      %1 = select i1 true, i32 %load_c, i32 %load_b
      store i32 %1, ptr %a, align 4
      %load_b1 = load i32, ptr %b, align 4
      %load_c2 = load i32, ptr %c, align 4
      %2 = select i1 true, i32 %load_c2, i32 %load_b1
      store i32 %2, ptr %a, align 4
      ret void
    }
    "#);
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

    filtered_assert_snapshot!(result);
}

#[test]
fn builtin_function_call_move() {
    let result = codegen(
        "PROGRAM main
        VAR
            a,b : DINT;
        END_VAR
            a := MOVE(b);
            a := MOVE(IN := b);
        END_PROGRAM",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %main = type { i32, i32 }

    @main_instance = global %main zeroinitializer

    define void @main(ptr %0) {
    entry:
      %a = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 1
      %load_b = load i32, ptr %b, align 4
      store i32 %load_b, ptr %a, align 4
      %load_b1 = load i32, ptr %b, align 4
      store i32 %load_b1, ptr %a, align 4
      ret void
    }
    "#);
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
            a := SIZEOF(IN := b);
        END_PROGRAM",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %main = type { i32, i64 }

    @main_instance = global %main zeroinitializer

    define void @main(ptr %0) {
    entry:
      %a = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 1
      store i32 ptrtoint (ptr getelementptr (i64, ptr null, i32 1) to i32), ptr %a, align 4
      store i32 ptrtoint (ptr getelementptr (i64, ptr null, i32 1) to i32), ptr %a, align 4
      ret void
    }
    "#);
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
            foo := LOWER_BOUND(arr := vla, dim := 1);
        END_VAR
        END_FUNCTION
        ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %main = type { [2 x i32], i32 }
    %__foo_vla = type { ptr, [2 x i32] }

    @main_instance = global %main zeroinitializer
    @____foo_vla__init = unnamed_addr constant %__foo_vla zeroinitializer

    define void @main(ptr %0) {
    entry:
      %a = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 1
      %auto_deref = load [2 x i32], ptr %a, align 4
      %outer_arr_gep = getelementptr inbounds [2 x i32], ptr %a, i32 0, i32 0
      %vla_struct = alloca %__foo_vla, align 8
      %vla_array_gep = getelementptr inbounds nuw %__foo_vla, ptr %vla_struct, i32 0, i32 0
      %vla_dimensions_gep = getelementptr inbounds nuw %__foo_vla, ptr %vla_struct, i32 0, i32 1
      store [2 x i32] [i32 0, i32 1], ptr %vla_dimensions_gep, align 4
      store ptr %outer_arr_gep, ptr %vla_array_gep, align 8
      %1 = load %__foo_vla, ptr %vla_struct, align 8
      %vla_struct_ptr = alloca %__foo_vla, align 8
      store %__foo_vla %1, ptr %vla_struct_ptr, align 8
      %call = call i32 @foo(ptr %vla_struct_ptr)
      store i32 %call, ptr %b, align 4
      ret void
    }

    define i32 @foo(ptr %0) {
    entry:
      %foo = alloca i32, align 4
      %vla = alloca ptr, align 8
      store ptr %0, ptr %vla, align 8
      store i32 0, ptr %foo, align 4
      %deref = load ptr, ptr %vla, align 8
      %dim = getelementptr inbounds nuw %__foo_vla, ptr %deref, i32 0, i32 1
      %1 = getelementptr inbounds [2 x i32], ptr %dim, i32 0, i32 0
      %2 = load i32, ptr %1, align 4
      store i32 %2, ptr %foo, align 4
      %deref1 = load ptr, ptr %vla, align 8
      %dim2 = getelementptr inbounds nuw %__foo_vla, ptr %deref1, i32 0, i32 1
      %3 = getelementptr inbounds [2 x i32], ptr %dim2, i32 0, i32 0
      %4 = load i32, ptr %3, align 4
      store i32 %4, ptr %foo, align 4
      %foo_ret = load i32, ptr %foo, align 4
      ret i32 %foo_ret
    }
    "#);
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
            foo := UPPER_BOUND(arr := vla, dim := 1);
        END_VAR
        END_FUNCTION
        ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %main = type { [2 x i32], i32 }
    %__foo_vla = type { ptr, [2 x i32] }

    @main_instance = global %main zeroinitializer
    @____foo_vla__init = unnamed_addr constant %__foo_vla zeroinitializer

    define void @main(ptr %0) {
    entry:
      %a = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 1
      %auto_deref = load [2 x i32], ptr %a, align 4
      %outer_arr_gep = getelementptr inbounds [2 x i32], ptr %a, i32 0, i32 0
      %vla_struct = alloca %__foo_vla, align 8
      %vla_array_gep = getelementptr inbounds nuw %__foo_vla, ptr %vla_struct, i32 0, i32 0
      %vla_dimensions_gep = getelementptr inbounds nuw %__foo_vla, ptr %vla_struct, i32 0, i32 1
      store [2 x i32] [i32 0, i32 1], ptr %vla_dimensions_gep, align 4
      store ptr %outer_arr_gep, ptr %vla_array_gep, align 8
      %1 = load %__foo_vla, ptr %vla_struct, align 8
      %vla_struct_ptr = alloca %__foo_vla, align 8
      store %__foo_vla %1, ptr %vla_struct_ptr, align 8
      %call = call i32 @foo(ptr %vla_struct_ptr)
      store i32 %call, ptr %b, align 4
      ret void
    }

    define i32 @foo(ptr %0) {
    entry:
      %foo = alloca i32, align 4
      %vla = alloca ptr, align 8
      store ptr %0, ptr %vla, align 8
      store i32 0, ptr %foo, align 4
      %deref = load ptr, ptr %vla, align 8
      %dim = getelementptr inbounds nuw %__foo_vla, ptr %deref, i32 0, i32 1
      %1 = getelementptr inbounds [2 x i32], ptr %dim, i32 0, i32 1
      %2 = load i32, ptr %1, align 4
      store i32 %2, ptr %foo, align 4
      %deref1 = load ptr, ptr %vla, align 8
      %dim2 = getelementptr inbounds nuw %__foo_vla, ptr %deref1, i32 0, i32 1
      %3 = getelementptr inbounds [2 x i32], ptr %dim2, i32 0, i32 1
      %4 = load i32, ptr %3, align 4
      store i32 %4, ptr %foo, align 4
      %foo_ret = load i32, ptr %foo, align 4
      ret i32 %foo_ret
    }
    "#);
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
            foo := UPPER_BOUND(arr := vla, dim := MY_CONST - (2 * 3));
        END_VAR
        END_FUNCTION
        ",
    );

    filtered_assert_snapshot!(result);
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

    filtered_assert_snapshot!(result);
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

    filtered_assert_snapshot!(result);
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

    filtered_assert_snapshot!(result);
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

    filtered_assert_snapshot!(res);
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

    filtered_assert_snapshot!(res);
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

    filtered_assert_snapshot!(res);
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

    filtered_assert_snapshot!(res);
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

    filtered_assert_snapshot!(res);
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

    filtered_assert_snapshot!(res);
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

    filtered_assert_snapshot!(res);
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

    filtered_assert_snapshot!(res);
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

    filtered_assert_snapshot!(res);
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

    filtered_assert_snapshot!(res);
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

    filtered_assert_snapshot!(res);
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

    filtered_assert_snapshot!(res);
}

#[test]
fn builtin_div_with_named_arguments() {
    let src = r#"
        FUNCTION main : DINT
        VAR
            x : DINT := 20;
            y : DINT := 4;
        END_VAR
            DIV(IN1 := x, IN2 := y);
        END_FUNCTION
    "#;

    let res = codegen(src);

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %x = alloca i32, align 4
      %y = alloca i32, align 4
      store i32 20, ptr %x, align 4
      store i32 4, ptr %y, align 4
      store i32 0, ptr %main, align 4
      %load_x = load i32, ptr %x, align 4
      %load_y = load i32, ptr %y, align 4
      %tmpVar = sdiv i32 %load_x, %load_y
      %main_ret = load i32, ptr %main, align 4
      ret i32 %main_ret
    }
    "#);
}

#[test]
fn builtin_sub_with_named_arguments() {
    let src = r#"
        FUNCTION main : DINT
        VAR
            x : DINT := 20;
            y : DINT := 4;
        END_VAR
            SUB(IN1 := x, IN2 := y);
        END_FUNCTION
    "#;

    let res = codegen(src);

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %x = alloca i32, align 4
      %y = alloca i32, align 4
      store i32 20, ptr %x, align 4
      store i32 4, ptr %y, align 4
      store i32 0, ptr %main, align 4
      %load_x = load i32, ptr %x, align 4
      %load_y = load i32, ptr %y, align 4
      %tmpVar = sub i32 %load_x, %load_y
      %main_ret = load i32, ptr %main, align 4
      ret i32 %main_ret
    }
    "#);
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
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %main = type { i32 }

    @foo = global i32 0
    @main_instance = global %main zeroinitializer

    define void @main(ptr %0) {
    entry:
      %foo = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
      %load_foo = load i32, ptr @foo, align 4
      store i32 %load_foo, ptr %foo, align 4
      %load_foo1 = load i32, ptr @foo, align 4
      %tmpVar = add i32 %load_foo1, 1
      store i32 %tmpVar, ptr %foo, align 4
      %load_foo2 = load i32, ptr @foo, align 4
      %load_foo3 = load i32, ptr @foo, align 4
      %tmpVar4 = add i32 %load_foo2, %load_foo3
      store i32 %tmpVar4, ptr %foo, align 4
      %load_foo5 = load i32, ptr %foo, align 4
      store i32 %load_foo5, ptr @foo, align 4
      %load_foo6 = load i32, ptr @foo, align 4
      %tmpVar7 = add i32 %load_foo6, 1
      store i32 %tmpVar7, ptr @foo, align 4
      ret void
    }
    "#);
}

#[test]
fn unary_plus_expression_test() {
    let result = codegen(
        "
        PROGRAM exp
        VAR
            x : DINT;
        END_VAR
            +x;
            x := +x + 4;
            x := +-4 + 5;
            +-x;
            x := +foo(+x);
        END_PROGRAM

        FUNCTION foo : DINT
        VAR_INPUT
            x : DINT;
        END_VAR
            foo := +x;
        END_FUNCTION
    ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %exp = type { i32 }

    @exp_instance = global %exp zeroinitializer

    define void @exp(ptr %0) {
    entry:
      %x = getelementptr inbounds nuw %exp, ptr %0, i32 0, i32 0
      %load_x = load i32, ptr %x, align 4
      %load_x1 = load i32, ptr %x, align 4
      %tmpVar = add i32 %load_x1, 4
      store i32 %tmpVar, ptr %x, align 4
      store i32 1, ptr %x, align 4
      %load_x2 = load i32, ptr %x, align 4
      %tmpVar3 = sub i32 0, %load_x2
      %load_x4 = load i32, ptr %x, align 4
      %call = call i32 @foo(i32 %load_x4)
      store i32 %call, ptr %x, align 4
      ret void
    }

    define i32 @foo(i32 %0) {
    entry:
      %foo = alloca i32, align 4
      %x = alloca i32, align 4
      store i32 %0, ptr %x, align 4
      store i32 0, ptr %foo, align 4
      %load_x = load i32, ptr %x, align 4
      store i32 %load_x, ptr %foo, align 4
      %foo_ret = load i32, ptr %foo, align 4
      ret i32 %foo_ret
    }
    "#)
}
