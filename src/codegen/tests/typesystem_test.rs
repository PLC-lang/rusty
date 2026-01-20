// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::test_utils::tests::codegen;
use plc_util::filtered_assert_snapshot;
//Same size operations remain the same
// Different types smaller than int converted to int (expanded according to sign)
// Different types with one element bigger than int converts all elements to its size
// Unary operator on an element equal to or bigger than int converts it to the next bigger size (if available)
// Expansions follow the sign of the original datatype

/*
                                            x       x
        +-------+-------+-------+-------+-------+-------+
        |       | <=Int | DINT  | LINT  | REAL  | LREAL |
        +-------+-------+-------+-------+-------+-------+
        | <=INT | INT   | DINT  | LINT  | REAL  | LREAL |
        +-------+-------+-------+-------+-------+-------+
        | DINT  | DINT  | DINT  | LINT  | REAL  | LREAL |
        +-------+-------+-------+-------+-------+-------+
        | LINT  | LINT  | LINT  | LINT  | LREAL | LREAL |
        +-------+-------+-------+-------+-------+-------+
      x | REAL  | REAL  | REAL  | LREAL | REAL  | LREAL |
        +-------+-------+-------+-------+-------+-------+
      x | LREAL | LREAL | LREAL | LREAL | LREAL | LREAL |
        +-------+-------+-------+-------+-------+-------++

*/

#[test]
fn even_all_sint_expressions_fallback_to_dint() {
    let result = codegen(
        r#"PROGRAM prg
        VAR
        b : SINT;
        c : SINT;
        x : SINT;
        END_VAR

        x := b + c;

        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn datatypes_smaller_than_dint_promoted_to_dint() {
    let result = codegen(
        r#"PROGRAM prg
        VAR
        b : SINT;
        c : DINT;
        x : DINT;
        END_VAR

        x := b + c;

        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn aliased_datatypes_respect_conversion_rules() {
    let result = codegen(
        r#"
        TYPE MYSINT : SINT; END_TYPE
        TYPE MYDINT : DINT; END_TYPE
        PROGRAM prg
        VAR
        b : MYSINT;
        c : MYDINT;
        x : MYDINT;
        END_VAR

        x := b + c;
        b := c + x;

        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn unsingned_datatypes_smaller_than_dint_promoted_to_dint() {
    let result = codegen(
        r#"PROGRAM prg
        VAR
        b : BYTE;
        c : DINT;
        x : DINT;
        END_VAR

        x := b + c;

        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn datatypes_larger_than_int_promote_the_second_operand() {
    let result = codegen(
        r#"PROGRAM prg
        VAR
        b : DINT;
        c : LINT;
        x : LINT;
        END_VAR

        x := b + c;

        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn float_and_double_mix_converted_to_double() {
    let result = codegen(
        r#"
        PROGRAM prg
        VAR
            a : REAL;
            b : LREAL;
            c : LREAL;
        END_VAR

        c := b + a;
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn float_assinged_to_double_to_double() {
    let result = codegen(
        r#"
        PROGRAM prg
        VAR
            a : REAL;
            b : LREAL;
        END_VAR

        b := a;
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn int_assigned_to_float_is_cast() {
    let result = codegen(
        r#"
        PROGRAM prg
        VAR
            a : INT;
            b : UINT;
            c : REAL;
        END_VAR
        c := a;
        c := b;
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn float_assigned_to_int_is_cast() {
    let result = codegen(
        r#"
        PROGRAM prg
        VAR
            a : INT;
            b : UINT;
            c : REAL;
        END_VAR
        a := c;
        b := c;
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn int_smaller_or_equal_to_float_converted_to_float() {
    let result = codegen(
        r#"
        PROGRAM prg
        VAR
            a : REAL;
            b : INT;
            c : REAL;
        END_VAR

        c := b + a;
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn int_bigger_than_float_converted_to_double() {
    let result = codegen(
        r#"
        PROGRAM prg
        VAR
            a : REAL;
            b : LINT;
        END_VAR

        b + a;
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn int_bigger_than_byte_promoted_on_compare_statement() {
    let result = codegen(
        r#"
        PROGRAM prg
        VAR
            a : BYTE;
            b : LINT;
        END_VAR

        b < a;
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn numerical_promotion_for_variadic_functions_without_declaration() {
    let src = r#"
    {external}
    FUNCTION printf : DINT
    VAR_IN_OUT
        format: STRING;
    END_VAR
    VAR_INPUT
        args: ...;
    END_VAR
    END_FUNCTION

    PROGRAM main
    VAR_TEMP
        s: STRING := '$N numbers: %f %f %f %d $N $N';
        float: REAL := 3.0;
        double: LREAL := 3.0;
        integer: INT := 3;
    END_VAR
        printf(s, REAL#3.0, float, double, integer);
    END_PROGRAM
    "#;

    let result = codegen(src);
    filtered_assert_snapshot!(result);
}

#[test]
fn small_int_varargs_get_promoted_while_32bit_and_higher_keep_their_type() {
    let src = r#"
    {external}
    FUNCTION printf : DINT
    VAR_IN_OUT
        format: STRING;
    END_VAR
    VAR_INPUT
        args: ...;
    END_VAR
    END_FUNCTION

    FUNCTION main : DINT
    VAR
        out1 : INT :=  -1;
        out2 : DINT := -1;
        out3 : LINT := -1;
        out4 : UDINT := 4_294_967_295;
    END_VAR
        printf('(d) result : %d %d %d %u$N', out1, out2, out3, out4);
        printf('(hd) result : %hd %hd %hd$N', out1, out2, out3);
    END_FUNCTION
    "#;

    let result = codegen(src);
    filtered_assert_snapshot!(result);
}

#[test]
fn enum_typed_varargs_get_promoted() {
    let src = r#"
    {external}
    FUNCTION printf : DINT
    VAR_IN_OUT
        format: STRING;
    END_VAR
    VAR_INPUT
        args: ...;
    END_VAR
    END_FUNCTION

    TYPE MyEnum : INT (a := 10, b := 20);
    END_TYPE

    FUNCTION main : DINT
    VAR
        e1 : MyEnum := a;
        i1 : INT := 10;
    END_VAR
        printf('result : %d %d$N', e1, i1);
    END_FUNCTION
    "#;

    let result = codegen(src);
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @MyEnum.a = unnamed_addr constant i16 10
    @MyEnum.b = unnamed_addr constant i16 20
    @utf08_literal_0 = private unnamed_addr constant [16 x i8] c"result : %d %d\0A\00"

    declare i32 @printf(ptr, ...)

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %e1 = alloca i16, align 2
      %i1 = alloca i16, align 2
      store i16 10, ptr %e1, align 2
      store i16 10, ptr %i1, align 2
      store i32 0, ptr %main, align 4
      %load_e1 = load i16, ptr %e1, align 2
      %0 = sext i16 %load_e1 to i32
      %load_i1 = load i16, ptr %i1, align 2
      %1 = sext i16 %load_i1 to i32
      %call = call i32 (ptr, ...) @printf(ptr @utf08_literal_0, i32 %0, i32 %1)
      %main_ret = load i32, ptr %main, align 4
      ret i32 %main_ret
    }
    "#);
}

#[test]
fn self_referential_struct_via_reference_codegen() {
    let result = codegen(
        r#"
        TYPE Node : STRUCT
            data : DINT;
            next : REF_TO Node;
        END_STRUCT END_TYPE

        PROGRAM main
        VAR
            node1 : Node;
            node2 : Node;
        END_VAR
            node1.data := 42;
            node2.data := 84;
            node1.next := REF(node2);
            node2.next := REF(node1);
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %main = type { %Node, %Node }
    %Node = type { i32, ptr }

    @main_instance = global %main zeroinitializer
    @__Node__init = unnamed_addr constant %Node zeroinitializer

    define void @main(ptr %0) {
    entry:
      %node1 = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 0
      %node2 = getelementptr inbounds nuw %main, ptr %0, i32 0, i32 1
      %data = getelementptr inbounds nuw %Node, ptr %node1, i32 0, i32 0
      store i32 42, ptr %data, align 4
      %data1 = getelementptr inbounds nuw %Node, ptr %node2, i32 0, i32 0
      store i32 84, ptr %data1, align 4
      %next = getelementptr inbounds nuw %Node, ptr %node1, i32 0, i32 1
      store ptr %node2, ptr %next, align 8
      %next2 = getelementptr inbounds nuw %Node, ptr %node2, i32 0, i32 1
      store ptr %node1, ptr %next2, align 8
      ret void
    }
    "#);
}

#[test]
fn arrays_and_strings_passed_as_pointers_in_unsized_variadics() {
    // Test that arrays and strings are passed as pointers to unsized variadic functions
    // following C ABI conventions (array/string decay to pointers)
    let src = r#"
    {external}
    FUNCTION printf : DINT
    VAR_INPUT
        format: STRING;
        args: ...;
    END_VAR
    END_FUNCTION

    PROGRAM main
    VAR_TEMP
        myString: STRING := 'hello';
        myArray: ARRAY[0..2] OF INT := [1, 2, 3];
    END_VAR
        // Both STRING and ARRAY should be passed as pointers (i8*)
        printf('String: %s', myString);
        printf('Array: %p', myArray);
    END_PROGRAM
    "#;

    let result = codegen(src);
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %main = type {}

    @main_instance = global %main zeroinitializer
    @__main.myString__init = unnamed_addr constant [81 x i8] c"hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @__main.myArray__init = unnamed_addr constant [3 x i16] [i16 1, i16 2, i16 3]
    @utf08_literal_0 = private unnamed_addr constant [10 x i8] c"Array: %p\00"
    @utf08_literal_1 = private unnamed_addr constant [11 x i8] c"String: %s\00"

    declare i32 @printf(ptr, ...)

    define void @main(ptr %0) {
    entry:
      %myString = alloca [81 x i8], align 1
      %myArray = alloca [3 x i16], align 2
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %myString, ptr align 1 @__main.myString__init, i64 ptrtoint (ptr getelementptr ([81 x i8], ptr null, i32 1) to i64), i1 false)
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %myArray, ptr align 1 @__main.myArray__init, i64 ptrtoint (ptr getelementptr ([3 x i16], ptr null, i32 1) to i64), i1 false)
      %call = call i32 (ptr, ...) @printf(ptr @utf08_literal_1, ptr %myString)
      %call1 = call i32 (ptr, ...) @printf(ptr @utf08_literal_0, ptr %myArray)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}
