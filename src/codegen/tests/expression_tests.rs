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
        END_FUNCTION"#,
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
fn string_comparison_test() {
    let result = codegen(
        r#"
        FUNCTION STRING_EQUAL : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR

        END_FUNCTION

        FUNCTION baz : INT
            VAR a,b : STRING; END_VAR
            VAR result : BOOL; END_VAR

            result := 'a' = 'b';
            result := a = b;
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn string_equal_with_constant_test() {
    let result = codegen(
        r#"
        FUNCTION STRING_EQUAL : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR
        END_FUNCTION

        FUNCTION baz : INT
            VAR a,b : STRING; END_VAR
            VAR result : BOOL; END_VAR

            result := a = 'b';
            result := 'a' = b;
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn string_less_with_constant_test() {
    let result = codegen(
        r#"
        FUNCTION STRING_LESS : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR
        END_FUNCTION

        FUNCTION baz : INT
            VAR a : STRING; END_VAR
            VAR result : BOOL; END_VAR

            result := a < 'b';
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn string_greater_with_constant_test() {
    let result = codegen(
        r#"
        FUNCTION STRING_GREATER : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR
        END_FUNCTION

        FUNCTION baz : INT
            VAR a : STRING; END_VAR
            VAR result : BOOL; END_VAR

            result := a > 'b';
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn string_not_equal_with_constant_test() {
    let result = codegen(
        r#"
        FUNCTION STRING_EQUAL : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR
        END_FUNCTION

        FUNCTION baz : INT
            VAR a : STRING; END_VAR
            VAR result : BOOL; END_VAR

            result := a <> 'b';
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn string_smaller_or_equal_with_constant_test() {
    let result = codegen(
        r#"
        FUNCTION STRING_LESS : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR
        END_FUNCTION
        FUNCTION STRING_EQUAL : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR
        END_FUNCTION

        FUNCTION baz : INT
            VAR a,b : STRING; END_VAR
            VAR result : BOOL; END_VAR

            result := a <= 'b';
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn string_greater_or_equal_with_constant_test() {
    let result = codegen(
        r#"
        FUNCTION STRING_GREATER : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR
        END_FUNCTION
        FUNCTION STRING_EQUAL : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR
        END_FUNCTION

        FUNCTION baz : INT
            VAR a,b : STRING; END_VAR
            VAR result : BOOL; END_VAR

            result := a >= 'b';
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn ranged_number_type_comparing_test() {
    let result = codegen(
        r#"
        FUNCTION baz : INT
            VAR x,y : INT(0..500); END_VAR;

            x = 3;
            x < y;
            y <= 0;
        END_FUNCTION
    "#,
    );

    //should result in normal number-comparisons
    insta::assert_snapshot!(result);
}

#[test]
fn aliased_ranged_number_type_comparing_test() {
    let result = codegen(
        r#"
        TYPE MyInt: INT(0..500); END_TYPE
        FUNCTION baz : INT
            VAR x,y : MyInt; END_VAR;

            x = 3;
            x < y;
            y <= 0;
        END_FUNCTION
    "#,
    );

    //should result in normal number-comparisons
    insta::assert_snapshot!(result);
}

#[test]
fn aliased_number_type_comparing_test() {
    let result = codegen(
        r#"
        TYPE MyInt: INT; END_TYPE

        FUNCTION baz : INT
            VAR x,y : MyInt; END_VAR;

            x = 3;
            x < y;
            y <= 0;
        END_FUNCTION
    "#,
    );

    //should result in normal number-comparisons
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
fn pointer_addition() {
    let result = codegen(
        "
		PROGRAM main
		VAR
			x : INT := 10;
			pt : REF_TO INT;
		END_VAR
		pt := &(x);
		pt := pt + 1;
		END_PROGRAM
		",
    );
    insta::assert_snapshot!(result);
}
