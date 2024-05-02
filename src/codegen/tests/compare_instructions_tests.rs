use crate::test_utils::tests::codegen;

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
fn pointer_compare_instructions() {
    // codegen should be successful for binary expression for pointer<->int / int<->pointer / pointer<->pointer
    let result = codegen(
        "
        PROGRAM main
        VAR
            x : INT := 10;
            y : INT := 20;
            pt : REF_TO INT;
            comp : BOOL;
        END_VAR
        pt := REF(x);

        (* compare pointer-pointer / pointer-int *)
        comp := pt = pt;
        comp := pt <> y;
        comp := pt < pt;
        comp := pt > y;
        comp := pt <= pt;
        comp := y >= pt;
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn pointer_function_call_compare_instructions() {
    // codegen should be successful for binary expression for pointer<->int / int<->pointer / pointer<->pointer
    let result = codegen(
        "
        FUNCTION foo : LINT
        END_FUNCTION

        PROGRAM main
        VAR
            pt : REF_TO INT;
            x : INT;
            comp : BOOL;
        END_VAR
        pt := REF(x);

        (* compare pointer-pointer / pointer-int *)
        comp := pt = foo();
        comp := pt <> foo();
        comp := pt < foo();
        comp := pt > foo();
        comp := pt <= foo();
        comp := pt >= foo();
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn compare_instructions_with_different_types() {
    let result = codegen(
        "
        TYPE MySubRangeInt: INT(0..500); END_TYPE
        TYPE MyDint: DINT; END_TYPE

        FUNCTION foo : LINT
        END_FUNCTION

        PROGRAM main
        VAR
            ptr_int : REF_TO INT;

            a : MySubRangeInt;
            b : MyDint;

            var_sint : SINT;
            var_int  : INT;
            var_dint : DINT;
            var_lint : LINT;

            var_usint : USINT;
            var_uint  : UINT;
            var_udint : UDINT;
            var_ulint : ULINT;
        END_VAR
            ptr_int := REF(var_int);

            var_sint = var_dint;
            var_int < 30;
            10 > var_lint;

            var_usint <> var_udint;
            var_uint <= UDINT#40;
            UDINT#10 >= var_ulint;

            var_sint = var_usint;
            var_uint <= var_lint;
            var_dint >= var_ulint;

            var_lint < a;
            a > var_sint;
            b < var_lint;
            SINT#5 <> b;

            ptr_int <= var_usint;
            a = ptr_int;

            foo() <> 40;
            var_udint <= foo();
            foo() = var_lint;
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn compare_instruction_functions_with_different_types() {
    let result = codegen(
        "
        TYPE MySubRangeInt: INT(0..500); END_TYPE
        TYPE MyDint: DINT; END_TYPE

        FUNCTION foo : REAL
        END_FUNCTION

        PROGRAM main
        VAR
            ptr_float : REF_TO REAL;

            a : MySubRangeInt;
            b : MyDint;

            var_real : REAL;
            var_lreal : LREAL;

            var_sint : SINT;
            var_int  : INT;
            var_dint : DINT;
            var_lint : LINT;

            var_usint : USINT;
            var_uint  : UINT;
            var_udint : UDINT;
            var_ulint : ULINT;
        END_VAR
            ptr_float := REF(var_real);

            EQ(var_sint, var_dint)
            GT(var_int, 30);
            GT(10.5, var_lreal);

            NE(var_usint, var_udint);
            LE(var_uint, UDINT#40);
            GE(UDINT#10, var_ulint);

            EQ(var_sint, var_usint);
            LE(var_uint, var_lint);
            GE(var_dint, var_ulint);

            LT(var_lint, a);
            GT(a, var_sint);
            LT(b, var_lint);
            NE(SINT#5, b, 17);

            LE(ptr_float, var_usint);
            EQ(a, ptr_float);

            NE(foo(), 40.5);
            LE(var_udint, foo());
            EQ(foo(), var_lint);
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn compare_datetime_types() {
    let result = codegen(
        "
        PROGRAM main
        VAR
            var_time: TIME;
            var_time_of_day: TIME_OF_DAY;
            var_date: DATE;
            var_date_and_time: DATE_AND_TIME;
        END_VAR
            GT(var_time, var_time_of_day, var_date, var_date_and_time);
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(result);
}
