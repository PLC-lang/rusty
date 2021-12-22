// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::*;
use crate::test_utils::tests::codegen;

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

    insta::assert_snapshot!(result);
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

    insta::assert_snapshot!(result);
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

    insta::assert_snapshot!(result);
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

    insta::assert_snapshot!(result);
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

    insta::assert_snapshot!(result);
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

    insta::assert_snapshot!(result);
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

    insta::assert_snapshot!(result);
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

    insta::assert_snapshot!(result);
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

    insta::assert_snapshot!(result);
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

    insta::assert_snapshot!(result);
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

    insta::assert_snapshot!(result);
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

    insta::assert_snapshot!(result);
}
