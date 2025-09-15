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
    %Node = type { i32, %Node* }

    @main_instance = global %main zeroinitializer
    @__Node__init = unnamed_addr constant %Node zeroinitializer

    define void @main(%main* %0) {
    entry:
      %node1 = getelementptr inbounds %main, %main* %0, i32 0, i32 0
      %node2 = getelementptr inbounds %main, %main* %0, i32 0, i32 1
      %data = getelementptr inbounds %Node, %Node* %node1, i32 0, i32 0
      store i32 42, i32* %data, align 4
      %data1 = getelementptr inbounds %Node, %Node* %node2, i32 0, i32 0
      store i32 84, i32* %data1, align 4
      %next = getelementptr inbounds %Node, %Node* %node1, i32 0, i32 1
      store %Node* %node2, %Node** %next, align 8
      %next2 = getelementptr inbounds %Node, %Node* %node2, i32 0, i32 1
      store %Node* %node1, %Node** %next2, align 8
      ret void
    }
    "#);
}
