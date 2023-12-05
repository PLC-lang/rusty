// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::test_utils::tests::{codegen, codegen_debug_without_unwrap, generate_with_empty_program};
use insta::assert_snapshot;

#[test]
fn program_with_variables_and_references_generates_void_function_and_struct_and_body() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : DINT;
y : DINT;
END_VAR
x;
y;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn empty_statements_dont_generate_anything() {
    let result = codegen(
        r#"
        PROGRAM prg
            VAR x : DINT; y : DINT; END_VAR
            x;
            y;
        END_PROGRAM
"#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn external_program_global_var_is_external() {
    let result = codegen(
        r#"
        @EXTERNAL
        PROGRAM prg
            VAR x : DINT; y : DINT; END_VAR
            x;
            y;
        END_PROGRAM
"#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn empty_global_variable_list_generates_nothing() {
    let result = generate_with_empty_program("VAR_GLOBAL END_VAR");
    insta::assert_snapshot!(result);
}

#[test]
fn a_global_variables_generates_in_separate_global_variables() {
    let result = generate_with_empty_program("VAR_GLOBAL gX : INT; gY : BOOL; END_VAR");
    insta::assert_snapshot!(result);
}

#[test]
fn external_global_variable_generates_as_external() {
    let result = generate_with_empty_program("@EXTERNAL VAR_GLOBAL gX : INT; gY : BOOL; END_VAR");
    insta::assert_snapshot!(result);
}

#[test]
fn two_global_variables_generates_in_separate_global_variables() {
    let result =
        generate_with_empty_program("VAR_GLOBAL gX : INT; gY : BOOL; END_VAR VAR_GLOBAL gA : INT; END_VAR");
    insta::assert_snapshot!(result);
}

#[test]
fn global_variable_reference_is_generated() {
    let function = codegen(
        r"
    VAR_GLOBAL
        gX : INT;
    END_VAR
    PROGRAM prg
    VAR
      x : INT;
    END_VAR
    gX := 20;
    x := gX;
    END_PROGRAM
    ",
    );

    insta::assert_snapshot!(function);
}

#[test]
fn empty_program_with_name_generates_void_function() {
    let result = codegen("PROGRAM prg END_PROGRAM");
    insta::assert_snapshot!(result);
}

#[test]
fn empty_function_with_name_generates_int_function() {
    let result = codegen("FUNCTION foo : INT END_FUNCTION");
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_variables_generates_void_function_and_struct() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : DINT;
y : DINT;
END_VAR
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_bool_variables_and_references_generates_void_function_and_struct_and_body() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : BOOL;
y : BOOL;
END_VAR
x;
y;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_variables_and_additions_generates_void_function_and_struct_and_body() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : DINT;
y : DINT;
END_VAR
x + y;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_variable_and_addition_literal_generates_void_function_and_struct_and_body() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : DINT;
END_VAR
x + 7;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn casted_literals_code_gen_test() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : INT;
z : INT;
END_VAR

      // the INT# should prevent this addition
      // to result in an DINT (i32) and then truncated back
      // to i16 again

      z := x + INT#7;

END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn casted_literals_hex_ints_code_gen_test() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : DINT;
END_VAR

      x := INT#16#FFFF;
      x := WORD#16#FFFF;

END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn casted_literals_lreal_code_gen_test() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : REAL;
z : REAL;
END_VAR

      // the LREAL# should fource a double addition
      z := x + LREAL#7.7;

END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn casted_literals_real_code_gen_test() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : INT;
z : REAL;
END_VAR

      // the REAL# should prevent this addition
      // to result in an DINT (i32) and then result
      // in an i32 devision

      z := x / REAL#7;

END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn casted_literals_hex_code_gen_test() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : INT;
z : INT;
END_VAR

      // the INT# should prevent this addition
      // to result in an DINT (i32) and then
      // truncated back to i16

      z := x +  INT#16#D;

END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn min_max_real_and_lreal_values_do_not_result_in_an_under_or_overflow() {
    // See relevant issue https://github.com/PLC-lang/rusty/issues/732
    // TL;DR: The given code snippet should NOT result in under- or overflows as they're the MIN and MAX
    //        values for (l)reals.
    let result = codegen(
        r#"
        PROGRAM main
            VAR
                F32_MIN : REAL  := -3.40282347E+38;
                F32_MAX : REAL  :=  3.40282347E+38;
                F64_MIN : LREAL := -1.7976931348623157E+308;
                F64_MAX : LREAL :=  1.7976931348623157E+308;
            END_VAR
        END_PROGRAM
        "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn casted_literals_bool_code_gen_test() {
    let result = codegen(
        r#"PROGRAM prg
VAR
z : BOOL;
END_VAR

      z := BOOL#TRUE;
      z := BOOL#FALSE;
      z := BOOL#1;
      z := BOOL#0;

END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_variable_assignment_generates_void_function_and_struct_and_body() {
    let result = codegen(
        r#"PROGRAM prg
VAR
y : DINT;
END_VAR
y := 7;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_real_assignment() {
    let result = codegen(
        r#"PROGRAM prg
VAR
y : REAL;
END_VAR
y := 0.15625;
y := 0.1e3;
y := 1e3;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_real_cast_assignment() {
    let result = codegen(
        r#"PROGRAM prg
VAR
y : REAL;
x : INT;
END_VAR
y := x;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_date_assignment() {
    let result = codegen(
        r#"PROGRAM prg
VAR
w : TIME_OF_DAY;
x : TIME;
y : DATE;
z : DATE_AND_TIME;
END_VAR
w := TIME_OF_DAY#15:36:30.123;
w := TOD#15:36:30.123;
x := TIME#100s12ms;
x := T#100s12ms;
y := DATE#1984-10-01;
y := D#1970-01-01;
z := DATE_AND_TIME#1984-10-01-20:15:14;
z := DT#1970-01-01-16:20:04.123;
z := DT#1970-01-01-16:20:04.123456789;
z := DATE_AND_TIME#2000-01-01-20:15:00;
z := DATE_AND_TIME#2000-01-01-20:15;
z := DT#2000-01-01-20:15:08.123;
END_PROGRAM
"#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn program_with_long_date_assignment() {
    let result = codegen(
        r#"PROGRAM prg
VAR
w : LTIME;
x : LDATE;
y : LDT;
z : LTOD;
END_VAR
w := LTIME#100s12ms6us3ns;
w := LTIME#100s12ms6us3ns;
x := LDATE#1984-10-01;
x := LDATE#1970-01-01;
y := LDT#1984-10-01-20:15:14;
y := LDT#1970-01-01-16:20:04.123456789;
z := LTOD#15:36:30.999999999;
z := LTOD#15:36:30.123456;
END_PROGRAM
"#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn program_with_date_assignment_whit_short_datatype_names() {
    let result = codegen(
        r#"PROGRAM prg
VAR
w : TOD;
x : T;
y : D;
z : DT;
END_VAR
w := TIME_OF_DAY#15:36:30.123;
w := TOD#15:36:30.123;
x := TIME#100s12ms;
x := T#100s12ms;
y := DATE#1984-10-01;
y := D#1970-01-01;
z := DATE_AND_TIME#1984-10-01-20:15;
z := DT#1970-01-01-16:20:08.123;
z := DT#1970-01-01-16:20:04.123456789;
END_PROGRAM
"#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn program_with_time_assignment() {
    let result = codegen(
        r#"PROGRAM prg
VAR
y : TIME;

END_VAR
y := T#0d0h0m0s0ms;
y := T#0.5d;
y := T#0d0h0m0.1s;
y := T#0d0h0m100ms;
y := T#1ms;
y := T#-1us;
y := T#1ns;
y := T#-1d0h0m0s1ms;
y := T#100d0h0m0s1ms;
END_PROGRAM
"#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn program_with_time_of_day_assignment() {
    let result = codegen(
        r#"PROGRAM prg
VAR
y : TIME_OF_DAY;

END_VAR
y := TIME_OF_DAY#00:00:00;
y := TOD#01:00:00;
y := TIME_OF_DAY#01:00:00.001;
y := TOD#1:1:1;
y := TIME_OF_DAY#20:15:00;
y := TIME_OF_DAY#20:15;
y := TOD#11:11:00;
y := TOD#11:11;
END_PROGRAM
"#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn time_variables_have_nano_seconds_resolution() {
    let result = codegen(
        r#"PROGRAM prg
VAR
y : TIME;

END_VAR
y := T#1ms;
y := T#0.000001s;
y := T#0.0000001s;
y := T#100d0h0m0s1.125ms;
END_PROGRAM
"#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn date_comparisons() {
    let result = codegen(
        r#"PROGRAM prg
        VAR
          a : DATE;
          b : DATE_AND_TIME;
          c : TIME;
          d : TIME_OF_DAY;
        END_VAR

          a > D#2021-05-01;
          b > DT#2021-05-01-19:29:17;
          c > T#1d19h29m17s;
          d > TOD#19:29:17;
        END_PROGRAM"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn date_invalid_declaration() {
    let msg = codegen_debug_without_unwrap(
        r#"PROGRAM prg
        VAR
          a : DATE := D#2001-02-29; (* feb29 on non-leap year should not pass *)
        END_VAR
        END_PROGRAM"#,
        crate::DebugLevel::None,
    )
    .unwrap_err();
    assert_snapshot!(msg);
}

#[test]
fn program_with_string_assignment() {
    let result = codegen(
        r#"PROGRAM prg
            VAR
            y : STRING;
            z : WSTRING;
            END_VAR
            y := 'im a genius';
            z := "im a utf16 genius";
        END_PROGRAM"#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn program_with_special_chars_in_string() {
    let result = codegen(
        r#"PROGRAM prg
VAR
should_replace_s : STRING;
should_not_replace_s : STRING;

should_replace_ws : WSTRING;
should_not_replace_ws : WSTRING;
END_VAR
should_replace_s := 'a$l$L b$n$N c$p$P d$r$R e$t$T $$ $'single$' $57💖$F0$9F$92$96';
should_not_replace_s := '$0043 $"no replace$"';

should_replace_ws := "a$l$L b$n$N c$p$P d$r$R e$t$T $$ $"double$" $0057💖$D83D$DC96";
should_not_replace_ws := "$43 $'no replace$'";
END_PROGRAM
"#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn different_case_references() {
    let result = codegen(
        r#"
TYPE MyInt: INT := 1; END_TYPE
TYPE MyDInt: DINT := 2; END_TYPE

PROGRAM prg
VAR
y : int;
z : MyInt;
zz : Mydint;
END_VAR
END_PROGRAM
"#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn program_with_real_additions() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : REAL;
y : REAL;
z : REAL;
END_VAR
x := 12.375;
y := 0.25;
z := x + y;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_boolean_assignment_generates_void_function_and_struct_and_body() {
    let result = codegen(
        r#"PROGRAM prg
VAR
y : BOOL;
END_VAR
y := TRUE;
y := FALSE;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_variable_and_arithmatic_assignment_generates_void_function_and_struct_and_body() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : DINT;
y : DINT;
END_VAR
y := x + 1;
y := x - 2;
y := x * 3;
y := x / 4;
y := x MOD 5;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_variable_and_comparison_assignment_generates_void_function_and_struct_and_body() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : DINT;
y : BOOL;
END_VAR
y := x = 1;
y := x > 2;
y := x < 3;
y := x <> 4;
y := x >= 5;
y := x <= 6;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_floats_variable_and_comparison_assignment_generates_correctly() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : REAL;
y : BOOL;
END_VAR
y := x = 1;
y := x > 2;
y := x < 3;
y := x <> 4;
y := x >= 5;
y := x <= 6;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_and_statement() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : BOOL;
y : BOOL;
END_VAR
x AND y;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_or_statement() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : BOOL;
y : BOOL;
END_VAR
x OR y;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_xor_statement() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : BOOL;
y : BOOL;
z : BOOL;
END_VAR
z := x XOR y;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_negated_expressions_generates_void_function_and_struct_and_body() {
    let result = codegen(
        r#"PROGRAM prg
VAR
x : BOOL;
y : BOOL;
END_VAR
NOT x;
x AND NOT y;
END_PROGRAM
"#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn program_with_negated_combined_expressions_generates_void_function_and_struct_and_body() {
    let result = codegen(
        r#"PROGRAM prg
VAR
z : DINT;
y : BOOL;
END_VAR
y AND z >= 5;
NOT (z <= 6) OR y;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_signed_combined_expressions() {
    let result = codegen(
        r#"PROGRAM prg
            VAR
            z : DINT;
            y : DINT;
            END_VAR
            -1 + z;
            2 +-z;
            -y + 3;
            END_PROGRAM
            "#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn if_elsif_else_generator_test() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : DINT;
            y : DINT;
            z : DINT;
            u : DINT;
            b1 : BOOL;
            b2 : BOOL;
            b3 : BOOL;
        END_VAR
        IF b1 THEN
            x;
        ELSIF b2 THEN
            y;
        ELSIF b3 THEN
            z;
        ELSE
            u;
        END_IF
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn if_generator_test() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : DINT;
            b1 : BOOL;
        END_VAR
        IF b1 THEN
            x;
        END_IF
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn if_with_expression_generator_test() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : DINT;
            b1 : BOOL;
        END_VAR
        IF (x > 1) OR b1 THEN
            x;
        END_IF
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn for_statement_with_steps_test() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : DINT;
        END_VAR
        FOR x := 3 TO 10 BY 7 DO
            x;
        END_FOR
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn for_statement_with_continue() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : DINT;
        END_VAR
        FOR x := 3 TO 10 BY 7 DO
            x := x + 1;
            CONTINUE;
            x := x - 1;
        END_FOR
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn for_statement_with_exit() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : DINT;
        END_VAR
        FOR x := 3 TO 10 BY 7 DO
            x := x + 2;
            EXIT;
            x := x + 5;
        END_FOR
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn class_method_in_pou() {
    let result = codegen(
        "
        CLASS MyClass
            VAR
                x, y : INT;
            END_VAR

            METHOD testMethod
                VAR_INPUT myMethodArg : INT; END_VAR
                VAR myMethodLocalVar : INT; END_VAR

                x := myMethodArg;
                y := x;
                myMethodLocalVar = y;
            END_METHOD
        END_CLASS

        PROGRAM prg
        VAR
          cl : MyClass;
          x : INT;
        END_VAR
        x := cl.x;
        cl.testMethod(x);
        cl.testMethod(myMethodArg:= x);
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result)
}

#[test]
fn fb_method_in_pou() {
    let result = codegen(
        "
        FUNCTION_BLOCK MyClass
            VAR
                x, y : INT;
            END_VAR

            METHOD testMethod
                VAR_INPUT myMethodArg : INT; END_VAR
                VAR myMethodLocalVar : INT; END_VAR

                x := myMethodArg;
                y := x;
                myMethodLocalVar = y;
            END_METHOD
        END_FUNCTION_BLOCK

        PROGRAM prg
        VAR
          cl : MyClass;
          x : INT;
        END_VAR
        x := cl.x;
        cl.testMethod(x);
        cl.testMethod(myMethodArg:= x);
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result)
}

#[test]
fn method_codegen_return() {
    let result = codegen(
        "
    CLASS MyClass
        METHOD testMethod : INT
            VAR_INPUT myMethodArg : INT; END_VAR
            testMethod := 1;
        END_METHOD
    END_CLASS
        ",
    );

    insta::assert_snapshot!(result)
}

#[test]
fn method_codegen_void() {
    let result = codegen(
        "
    CLASS MyClass
        METHOD testMethod
            VAR_INPUT myMethodArg : INT; END_VAR
            VAR myMethodLocalVar : INT; END_VAR

            myMethodLocalVar := 1;
        END_METHOD
    END_CLASS
        ",
    );

    insta::assert_snapshot!(result)
}

#[test]
fn class_member_access_from_method() {
    let result = codegen(
        "
    CLASS MyClass
        VAR
            x, y : INT;
        END_VAR

        METHOD testMethod
            VAR_INPUT myMethodArg : INT; END_VAR
            VAR myMethodLocalVar : INT; END_VAR

            x := myMethodArg;
            y := x;
            myMethodLocalVar = y;
        END_METHOD
    END_CLASS
        ",
    );

    insta::assert_snapshot!(result)
}

#[test]
fn while_loop_with_if_exit() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : DINT;
        END_VAR
        WHILE x < 20 DO
          x := x + 1;
          IF x >= 10 THEN
            EXIT;
          END_IF
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn for_statement_without_steps_test() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : DINT;
        END_VAR
        FOR x := 3 TO 10 DO
            x;
        END_FOR
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn for_statement_sint() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : SINT;
        END_VAR
        FOR x := 3 TO 10 DO
            x;
        END_FOR
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn for_statement_int() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : INT;
        END_VAR
        FOR x := 3 TO 10 DO
            x;
        END_FOR
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn for_statement_lint() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : LINT;
        END_VAR
        FOR x := 3 TO 10 DO
            x;
        END_FOR
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn for_statement_continue() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : DINT;
        END_VAR
        FOR x := 3 TO 10 DO
        END_FOR
        x;
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn for_statement_with_references_steps_test() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            step: DINT;
            x : DINT;
            y : DINT;
            z : DINT;
        END_VAR
        FOR x := y TO z BY step DO
            x;
        END_FOR
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn while_statement() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : BOOL;
        END_VAR
        WHILE x DO
            x;
        END_WHILE
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn while_with_expression_statement() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : BOOL;
        END_VAR
        WHILE x = 0 DO
            x;
        END_WHILE
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn repeat_statement() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : BOOL;
        END_VAR
        REPEAT
            x;
        UNTIL x
        END_REPEAT
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn simple_case_statement() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : DINT;
            y : DINT;
        END_VAR
        CASE x OF
        1: y := 1;
        2: y := 2;
        3: y := 3;
        ELSE
            y := -1;
        END_CASE
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn simple_case_i8_statement() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : BYTE;
            y : BYTE;
        END_VAR
        CASE x OF
        1: y := 1;
        2: y := 2;
        3: y := 3;
        ELSE
            y := 0;
        END_CASE
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn case_with_multiple_labels_statement() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : DINT;
            y : DINT;
        END_VAR
        CASE x OF
        1,2: y := 1;
        3,4: y := 2;
        ELSE
            y := -1;
        END_CASE
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn case_with_ranges_statement() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : DINT;
            y : DINT;
        END_VAR
        CASE x OF
        2..3: y := 2;
        END_CASE
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn case_with_constant_expressions_in_case_selectors() {
    let result = codegen(
        r##"
VAR_GLOBAL CONSTANT
    FORWARD     : DINT := 7;
    UP          : DINT := FORWARD + 1;
    DOWN        : DINT := FORWARD + UP;
END_VAR

FUNCTION drive : DINT
    VAR
        input : DINT;
        horiz, depth : DINT;
    END_VAR

    CASE input OF
        FORWARD :
            horiz := horiz + 1;
        FORWARD*2:
            horiz := horiz + 2;
        UP :
            depth := depth - 1;
        DOWN :
            depth := depth + 1;

    END_CASE

END_FUNCTION
"##,
    );

    // WHEN we compile, we want to see propagated constant in the switch statement
    // -> so no references to variables, but int-values (7, 14, 8 and 15)
    insta::assert_snapshot!(result);
}

#[test]
fn case_with_enum_expressions_in_case_selectors() {
    let result = codegen(
        r##"
VAR_GLOBAL CONSTANT
    BASE     : DINT := 7;
END_VAR

TYPE Direction: (
    FORWARD := BASE,
    UP,
    DOWN := BASE * 4);
END_TYPE

FUNCTION drive : DINT
    VAR
        input : DINT;
        horiz, depth : DINT;
    END_VAR

    CASE input OF
        FORWARD :
        horiz := horiz + 1;
    FORWARD*2:
            horiz := horiz + 2;
    UP :
        depth := depth - 1;
    DOWN :
            depth := depth + 1;

    END_CASE

END_FUNCTION
"##,
    );

    // WHEN we compile, we want to see propagated constant in the switch statement
    // -> so no references to variables, but int-values (7, 14, 8 and 28)
    insta::assert_snapshot!(result);
}

#[test]
fn function_called_in_program() {
    let result = codegen(
        "
        FUNCTION foo : DINT
        foo := 1;
        END_FUNCTION

        PROGRAM prg
        VAR
            x : DINT;
        END_VAR
        x := foo();
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn real_function_called_in_program() {
    let result = codegen(
        "
        FUNCTION foo : REAL
        foo := 1.0;
        END_FUNCTION

        PROGRAM prg
        VAR
            x : DINT;
        END_VAR
        x := foo();
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn external_function_called_in_program() {
    let result = codegen(
        "
        @EXTERNAL FUNCTION foo : DINT
        END_FUNCTION

        PROGRAM prg
        foo();
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn nested_function_called_in_program() {
    let result = codegen(
        "
        FUNCTION bar : DINT
        bar := 1;
        END_FUNCTION

        FUNCTION foo : DINT
        VAR_INPUT
            in : DINT;
        END_VAR

        foo := 1;
        END_FUNCTION

        PROGRAM prg
        VAR
            x : DINT;
        END_VAR
        x := foo(bar());
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn function_with_parameters_called_in_program() {
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
          bar : DINT;
        END_VAR
        foo := 1;
        END_FUNCTION

        PROGRAM prg
        VAR
        x : DINT;
        END_VAR
        x := foo(2);
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn function_with_two_parameters_called_in_program() {
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
          bar : DINT;
          buz : BOOL;
        END_VAR
        foo := 1;
        END_FUNCTION

        PROGRAM prg
        VAR
        x : DINT;
        END_VAR
        x := foo(2, TRUE);
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn function_with_local_var_initialization_and_call() {
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
          in1 : DINT;
        END_VAR
        VAR
          x : INT := 7;
          y : INT;
          z : INT := 9;
        END_VAR
        foo := 1;
        END_FUNCTION
        PROGRAM prg
        foo(5);
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result)
}

#[test]
fn function_with_local_temp_var_initialization() {
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
          in1 : DINT;
        END_VAR
        VAR
          x : INT := 7;
        END_VAR
        VAR_TEMP
          y : INT;
          z : INT := 9;
        END_VAR
        y := z + 1;
        END_FUNCTION
        PROGRAM prg
        foo(5);
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(result)
}

#[test]
fn program_with_local_temp_var_initialization() {
    let result = codegen(
        "
        PROGRAM foo
        VAR
          x : INT := 7;
        END_VAR
        VAR_TEMP
          y : INT;
          z : INT := 9;
        END_VAR
        y := z + 1;
        END_PROGRAM
        PROGRAM prg
        foo();
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(result)
}

#[test]
fn program_called_in_program() {
    let result = codegen(
        "
        PROGRAM foo
        END_PROGRAM

        PROGRAM prg
        foo();
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn action_called_in_program() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : DINT;
        END_VAR
        foo();
        END_PROGRAM
        ACTIONS prg
        ACTION foo
            x := 2;
        END_ACTION
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn qualified_local_action_called_in_program() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : DINT;
        END_VAR
        prg.foo();
        END_PROGRAM
        ACTIONS prg
        ACTION foo
            x := 2;
        END_ACTION
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn qualified_foreign_action_called_in_program() {
    let result = codegen(
        "
        PROGRAM bar
            prg.foo();
        END_PROGRAM
        PROGRAM prg
        VAR
            x : DINT;
        END_VAR
        END_PROGRAM
        ACTIONS prg
        ACTION foo
            x := 2;
        END_ACTION
        END_ACTIONS
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn qualified_action_from_fb_called_in_program() {
    let result = codegen(
        "
        PROGRAM bar
        VAR
            fb_inst : fb;
        END_VAR
            fb_inst.foo();
        END_PROGRAM

        FUNCTION_BLOCK fb
        VAR
            x : DINT;
        END_VAR
        END_FUNCTION_BLOCK
        ACTIONS fb
        ACTION foo
            x := 2;
        END_ACTION
        END_ACTIONS
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn program_with_two_parameters_called_in_program() {
    let result = codegen(
        "
        PROGRAM foo
        VAR_INPUT
          bar : DINT;
          buz : BOOL;
        END_VAR
        END_PROGRAM

        PROGRAM prg
          foo(2, TRUE);
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn program_with_two_explicit_parameters_called_in_program() {
    let result = codegen(
        "
        PROGRAM foo
        VAR_INPUT
          bar : DINT;
          buz : BOOL;
        END_VAR
        END_PROGRAM

        PROGRAM prg
          foo(buz := TRUE, bar := 2);
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn program_with_var_out_called_in_program() {
    let result = codegen(
        "
        PROGRAM foo
        VAR_INPUT
          bar : DINT;
        END_VAR
        VAR_OUTPUT
          buz : BOOL;
        END_VAR
        END_PROGRAM

        PROGRAM prg
        VAR
            baz : BOOL;
        END_VAR
          foo(bar := 2, buz => baz);
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn program_with_var_inout_called_in_program() {
    let result = codegen(
        "
        PROGRAM foo
        VAR_IN_OUT
          inout : DINT;
        END_VAR
        inout := inout + 1;
        END_PROGRAM

        PROGRAM prg
        VAR
            baz : DINT;
        END_VAR
          baz := 7;
          foo(inout := baz);
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn pass_inout_to_inout() {
    let result = codegen(
        "
        PROGRAM foo2
        VAR_IN_OUT
          inout : DINT;
        END_VAR
        VAR_INPUT
          in : DINT;
        END_VAR
        END_PROGRAM

        PROGRAM foo
        VAR_IN_OUT
          inout : DINT;
        END_VAR
        foo2(inout := inout, in := inout);
        END_PROGRAM

        PROGRAM prg
        VAR
            baz : DINT;
        END_VAR
          foo(inout := baz);
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn pointers_generated() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            X : BOOL;
            pX : POINTER TO BOOL;
            rX : REF_TO BOOL;
        END_VAR

        //Assign address
        pX := NULL;
        rX := NULL;
        pX := &X;
        rX := &X;

        //Read from pointer
        X := pX^;
        X := rX^;

        //Write in pointer
        pX^ := X;
        rX^ := X;

        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn complex_pointers() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            X : INT;
            arrX : ARRAY[1..10] OF INT;
            arrrX : ARRAY[1..10] OF REF_TO INT;
            rarrX : REF_TO ARRAY[1..10] OF INT;
        END_VAR

        //Assign address
        arrX[1] := X;
        arrrX[2] := &arrX[3];
        rarrX := &arrX;

        //Read from pointer
        X := arrrX[4]^;
        X := rarrX^[5];

        //Write in pointer
        arrrX[6]^ := X;
        rarrX^[7] := arrrX[8]^;

        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn pointer_and_array_access_to_in_out() {
    let result = codegen(
        "
        FUNCTION main : INT
        VAR_IN_OUT
            a : REF_TO INT;
            b : ARRAY[0..1] OF INT;
        END_VAR
        VAR
            c : INT;
        END_VAR
        c := a^;
        c := b[0];
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result)
}

#[test]
fn program_with_var_out_called_mixed_in_program() {
    let result = codegen(
        "
        PROGRAM foo
        VAR_INPUT
          bar : DINT;
        END_VAR
        VAR_OUTPUT
          buz : BOOL;
        END_VAR
        END_PROGRAM

        PROGRAM prg
        VAR
            baz : BOOL;
        END_VAR
          foo(buz => baz, bar := 2);
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn program_called_before_decalaration() {
    codegen(
        "
        PROGRAM foo
          bar();
        END_PROGRAM

        PROGRAM bar
        END_PROGRAM
        ",
    );
    //Expecting no errors
}

#[test]
fn function_called_before_decalaration() {
    codegen(
        "
        FUNCTION foo : INT
          foo := bar();
        END_FUNCTION

        FUNCTION bar : INT
            bar := 7;
        END_FUNCTION
        ",
    );
    //Expecting no errors
}

#[test]
fn function_called_when_shadowed() {
    let result = codegen(
        "
        FUNCTION foo : DINT
        foo := 1;
        END_FUNCTION

        PROGRAM prg
        VAR
            froo : DINT;
        END_VAR
        froo := foo();  //the original test was foo := foo() which cannot work!!!
                        // imagine prg.foo was a FB which can be called.
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn function_block_instance_call() {
    let result = codegen(
        "
        FUNCTION_BLOCK foo
          VAR_INPUT
            x, y : INT;
          END_VAR
        END_FUNCTION_BLOCK

        PROGRAM prg
        VAR
            fb_inst : foo;
        END_VAR
        fb_inst();
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result)
}

#[test]
fn function_block_qualified_instance_call() {
    let result = codegen(
        "
        FUNCTION_BLOCK foo
        VAR
          bar_inst : bar;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar
        END_FUNCTION_BLOCK

        PROGRAM prg
        VAR
          foo_inst : foo;
        END_VAR
          foo_inst.bar_inst();
        END_PROGRAM
      ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn reference_qualified_name() {
    let result = codegen(
        "
        FUNCTION_BLOCK fb
        VAR_INPUT
          x :DINT;
        END_VAR
        END_FUNCTION_BLOCK
        PROGRAM foo
        VAR_INPUT
            x : DINT;
            y : DINT;
            baz : fb;
        END_VAR
        END_PROGRAM
        PROGRAM prg
        VAR
            x : DINT;
        END_VAR
            x := foo.x;
            x := foo.y;
            x := foo.baz.x;
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn structs_are_generated() {
    let result = codegen(
        "
        TYPE MyStruct: STRUCT
          a: DINT;
          b: INT;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL
          x : MyStruct;
          y : STRUCT
            a : BYTE;
            b : BYTE;
          END_STRUCT;
        END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn arrays_are_generated() {
    let result = codegen(
        "
        TYPE MyArray: ARRAY[0..9] OF INT; END_TYPE

        VAR_GLOBAL
          x : MyArray;
          y : ARRAY[0..5] OF REAL;
        END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn arrays_with_global_const_size_are_generated() {
    let result = codegen(
        "
        VAR_GLOBAL CONSTANT
          THREE : INT := 3;
          ZERO  : INT := 0;
          LEN   : INT := THREE * THREE;
        END_VAR

        TYPE MyArray: ARRAY[ZERO..LEN] OF INT; END_TYPE

        VAR_GLOBAL
          x : MyArray;
          y : ARRAY[ZERO .. LEN+1] OF DINT;
          z : ARRAY[-LEN .. THREE * THREE] OF BYTE;
          zz : ARRAY[-LEN .. ZERO, ZERO .. LEN] OF BYTE;
          zzz : ARRAY[-LEN .. ZERO] OF ARRAY[2 .. LEN] OF BYTE;
        END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn structs_members_can_be_referenced() {
    let result = codegen(
        "
        TYPE MyStruct: STRUCT
          a: DINT;
          b: DINT;
        END_STRUCT
        END_TYPE

        PROGRAM MainProg
        VAR
          Cord: MyStruct;
        END_VAR
          Cord.a := 0;
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn enums_are_generated() {
    let result = codegen(
        "
        TYPE MyEnum: (red, yellow, green);
        END_TYPE

        VAR_GLOBAL
          x : MyEnum;
        END_VAR
        ",
    );

    insta::assert_snapshot!(result)
}

#[test]
fn typed_enums_are_generated() {
    let result = codegen(
        "
        TYPE MyEnum: BYTE(red, yellow, green);
        END_TYPE

        TYPE MyEnum2: UINT(red, yellow, green);
        END_TYPE

        TYPE MyEnum3: DINT(red, yellow, green);
        END_TYPE

        VAR_GLOBAL
          x : MyEnum;
          y : MyEnum2;
          z : MyEnum3;
        END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn typed_enums_are_used_properly() {
    let result = codegen(
        "
        TYPE MyEnum: BYTE(red := 5, yellow, green);
        END_TYPE

        TYPE MyEnum2: UINT(red := 15, yellow, green);
        END_TYPE

        TYPE MyEnum3: DINT(red := 25, yellow, green);
        END_TYPE

        PROGRAM prg
            VAR
                x: BYTE;
                y: UINT;
                z: DINT;
            END_VAR

            x := MyEnum#yellow;
            y := MyEnum2#yellow;
            z := MyEnum3#yellow;

        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn typed_enums_with_initializers_are_generated() {
    let result = codegen(
        "
        TYPE MyEnum: BYTE(red := 1, yellow := 2, green := 3);
        END_TYPE

        TYPE MyEnum2: UINT(red := 10, yellow := 11, green := 12);
        END_TYPE

        TYPE MyEnum3: DINT(red := 22, yellow := 33, green := 44);
        END_TYPE

        VAR_GLOBAL
          x : MyEnum;
          y : MyEnum2;
          z : MyEnum3;
        END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn typed_enums_with_partly_initializers_are_generated() {
    let result = codegen(
        "
        VAR_GLOBAL CONSTANT
          twenty : INT := 20;
        END_VAR

        TYPE MyEnum: BYTE(red := 7, yellow, green);
        END_TYPE

        TYPE MyEnum: BYTE(a,b,c:=7,d,e,f:=twenty,g);
        END_TYPE

        VAR_GLOBAL
          x : MyEnum;
        END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn enums_custom_type_are_generated() {
    let result = codegen(
        "
    TYPE TrafficLight:
        (White, Red, Yellow, Green);
    END_TYPE

    PROGRAM main
    VAR
        tf1 : TrafficLight;
    END_VAR
    END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result)
}

#[test]
fn enum_members_can_be_used_in_asignments() {
    let result = codegen(
        "
      TYPE MyEnum: (red, yellow, green);
      END_TYPE

      PROGRAM main
      VAR
        color : MyEnum;
      END_VAR
      color := red;
      color := yellow;
      color := green;
      END_PROGRAM
      ",
    );

    insta::assert_snapshot!(result)
}

#[test]
fn inline_structs_are_generated() {
    let result = codegen(
        "

        VAR_GLOBAL
         x: STRUCT
              a: DINT;
              b: DINT;
            END_STRUCT
        END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn accessing_nested_structs() {
    let result = codegen(
        "
        TYPE InnerStruct:
        STRUCT
          inner1 : INT;
          inner2 : INT;
        END_STRUCT
        END_TYPE

        TYPE OuterStruct:
        STRUCT
          out1 : InnerStruct;
          out2 : InnerStruct;
        END_STRUCT
        END_TYPE

        PROGRAM Main
        VAR
          m : OuterStruct;
        END_VAR

          m.out1.inner1 := 3;
          m.out2.inner2 := 7;
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn inline_enums_are_generated() {
    let result = codegen(
        "
        VAR_GLOBAL
          x : (red, yellow, green);
        END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn basic_datatypes_generated() {
    let result = codegen(
        "
        VAR_GLOBAL
            bool_1   : BOOL;
            byte_2   : BYTE;
            sint_3   : SINT;
            usint_4  : USINT;
            word_5   : WORD;
            int_6    : INT;
            uint_7   : UINT;
            dword_8  : DWORD;
            dint_9   : DINT;
            udint_10 : UDINT;
            lword_11 : LWORD;
            lint_12  : LINT;
            ulint_13 : ULINT;
        END_VAR
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn array_of_int_type_generated() {
    let result = codegen(
        "
        PROGRAM prg
            VAR
                x : ARRAY[0..10] OF INT;
            END_VAR
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn array_of_cast_int_type_generated() {
    let result = codegen(
        "
        PROGRAM prg
            VAR
                x : ARRAY[0..INT#16#A] OF INT;
            END_VAR
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn array_of_int_type_used() {
    let result = codegen(
        "
        PROGRAM prg
            VAR
                x : ARRAY[0..3] OF DINT;
            END_VAR
            x[1] := 3;
            x[2] := x[3] + 3;
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn array_of_int_non_zero_type_generated() {
    let result = codegen(
        "
        PROGRAM prg
            VAR
                x : ARRAY[10..20] OF INT;
            END_VAR
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn array_of_int_type_with_non_zero_start_used() {
    let result = codegen(
        "
        PROGRAM prg
            VAR
                x : ARRAY[1..3] OF DINT;
            END_VAR
            x[1] := 3;
            x[2] := x[3] + 3;
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn array_of_int_non_zero_negative_type_generated() {
    let result = codegen(
        "
        PROGRAM prg
            VAR
                x : ARRAY[-10..20] OF INT;
            END_VAR
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn array_of_int_type_with_non_zero_negative_start_used() {
    let result = codegen(
        "
        PROGRAM prg
            VAR
                x : ARRAY[-2..3] OF DINT;
            END_VAR
            x[-1] := 3;
            x[2] := x[3] + 3;
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn multidim_array_declaration() {
    let result = codegen(
        "
        PROGRAM prg
            VAR
                x : ARRAY[0..1, 2..4] OF INT;
            END_VAR
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn multidim_array_access() {
    let result = codegen(
        "
        PROGRAM prg
            VAR
                x : ARRAY[0..3, 1..2] OF DINT;
            END_VAR
            x[2, 1] := 3;
            x[3, 2] := x[1, 2] + 3;
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn nested_array_declaration() {
    let result = codegen(
        "
        PROGRAM prg
            VAR
                x : ARRAY[2..4] OF ARRAY[0..1] OF INT;
            END_VAR
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn nested_array_access() {
    let result = codegen(
        "
        PROGRAM prg
            VAR
                x : ARRAY[0..3] OF ARRAY[1..2] OF DINT;
            END_VAR
            x[2][1] := 3;
            x[3][2] := x[1][2] + 3;
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn nested_array_cube_writes() {
    let result = codegen(
        r"
            PROGRAM main
            VAR
            x: INT;
            y: INT;
            z: INT;
            cube        : ARRAY[0..4, 0..4, 0..4] OF DINT;
            END_VAR

            cube[x, y, z] := x*y*z;
           END_PROGRAM
            ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn nested_array_cube_writes_negative_start() {
    let result = codegen(
        r"
            PROGRAM main
            VAR
            x: INT;
            y: INT;
            z: INT;
            cube        : ARRAY[-2..2,-2..2,-2..2] OF DINT;
            END_VAR

            cube[x, y, z] := x*y*z;
           END_PROGRAM
            ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn returning_early_in_function() {
    let result = codegen(
        "
        FUNCTION smaller_than_ten: INT
          VAR_INPUT n : SINT; END_VAR
          IF n < 10 THEN
                  RETURN;
          END_IF;
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn returning_early_in_function_block() {
    let result = codegen(
        "
        FUNCTION_BLOCK abcdef
          VAR_INPUT n : SINT; END_VAR
          IF n < 10 THEN
                  RETURN;
          END_IF;
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn accessing_nested_array_in_struct() {
    let result = codegen(
        "
        TYPE MyStruct:
        STRUCT
          field1 : ARRAY[0..4] OF INT;
        END_STRUCT
        END_TYPE

        PROGRAM Main
        VAR
          m : MyStruct;
        END_VAR

          m.field1[3] := 7;
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result)
}

#[test]
fn sub_range_type_calls_check_function_missing() {
    let source = "
            TYPE MyInt: INT(0..100); END_TYPE

            FUNCTION Check_XX_RangeSigned : INT
            VAR_INPUT
              value : INT;
              lower : INT;
              upper : INT;
            END_VAR
            Check_XX_RangeSigned := value;
            END_FUNCTION

            PROGRAM Main
            VAR
              x : MyInt;
            END_VAR

            x := 7;
            END_PROGRAM
           ";
    let result = codegen(source);

    // we expect a normal assignemnt, no check-function call
    insta::assert_snapshot!(result);
}

#[test]
fn sub_range_type_calls_check_function_on_assigment() {
    let source = "
            TYPE MyInt: INT(0..100); END_TYPE

            FUNCTION CheckRangeSigned : INT
            VAR_INPUT
              value : INT;
              lower : INT;
              upper : INT;
            END_VAR
            CheckRangeSigned := value;
            END_FUNCTION

            PROGRAM Main
            VAR
              x : MyInt;
            END_VAR

            x := 7;
            END_PROGRAM
           ";
    let result = codegen(source);

    // we expect no simple assigment, but we expect somehting like x:= CheckRangeSigned(7);
    insta::assert_snapshot!(result);
}

#[test]
fn using_global_consts_in_expressions() {
    //GIVEN some constants used in an expression
    let result = codegen(
        r#"
        VAR_GLOBAL CONSTANT
          cA : INT := 1;
          cB : INT := 2;
          cC : INT := cA + cB;
        END_VAR

        PROGRAM prg
          VAR
            z : DINT;
          END_VAR
          z := cA + cB + cC;
        END_PROGRAM
        "#,
    );
    //WHEN we compile
    // we expect the constants to be inlined
    insta::assert_snapshot!(result);
}

#[test]
fn using_cast_statement_as_const_expression() {
    //GIVEN a array-declaration with an expression using cast-statements
    let result = codegen(
        r#"
        PROGRAM prg
          VAR
            x: ARRAY[0 .. INT#16#B + INT#16#2] OF INT;
          END_VAR
        END_PROGRAM
        "#,
    );

    //THEN the array should be of size 14 (13 + 1 \0 byte)
    insta::assert_snapshot!(result);
}

#[test]
fn using_const_expression_in_range_type() {
    //GIVEN a range statement with an expression as an upper limit
    let result = codegen(
        r#"
        VAR_GLOBAL CONST
          MIN : INT := 7;
        END_VAR

        FUNCTION CheckRangeSigned: INT
          VAR_INPUT
              value : INT;
              lower : INT;
              upper : INT;
          END_VAR
          CheckRangeSigned := value;
        END_FUNCTION

        PROGRAM prg
          VAR
            x: INT(0 .. MIN+1);
          END_VAR
          x := 5;
        END_PROGRAM
        "#,
    );
    //assigning to x should call the range-function with 0 and 8 as parameters
    insta::assert_snapshot!(result);
}

#[test]
fn inlined_array_size_from_local_scoped_constants() {
    // GIVEN some an array with const-expr -dimensions
    // the dimension-constants are defined within the same POU
    // which means that a & b are only visible from within that PROGRAM
    let result = codegen(
        r#"
        VAR_GLOBAL CONSTANT
          a : INT := 0;
          b : INT := 2;
          c : INT := 5;
        END_VAR

        PROGRAM aaa
            VAR CONSTANT
                a : INT := 3;
                b : INT := 7;
            END_VAR

            VAR
                arr : ARRAY[a..b] OF BYTE;
                arr2 : ARRAY[a..c] OF BYTE;
            END_VAR
        END_PROGRAM
       "#,
    );

    // THEN we expect arr to be of size 5, not size 3
    // AND we expect arr2 to be of size 3
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_chars() {
    let result = codegen(
        r#"
        PROGRAM mainPROG
        VAR
            x : CHAR;
            y : WCHAR;
        END_VAR
            x := 'a';
            x := ' ';

            y := "A";
            y := " ";
            y := "'";
            y := "$"";
        END_PROGRAM
        "#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn program_with_casted_chars_assignment() {
    let result = codegen(
        r#"
        PROGRAM mainPROG
        VAR
            x : CHAR;
            y : WCHAR;
        END_VAR
            x := CHAR#"A";
            y := WCHAR#'B';
        END_PROGRAM
        "#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn function_call_with_same_name_as_return_type() {
    let result = codegen(
        "
        FUNCTION TIME : TIME
        END_FUNCTION

        PROGRAM prg
        VAR
        END_VAR
            TIME();
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn variable_with_same_name_as_data_type() {
    let result = codegen(
        "
        FUNCTION func : TIME
        VAR
            TIME : TIME;
        END_VAR
        END_FUNCTION

        PROGRAM prog
        VAR
            TIME : TIME;
        END_VAR
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(result);
}

/// THIS TEST IS MISLEADING!!!
/// THERE IS A CONFLICT BETWEEN TIME.TIME (the VAR)
/// AND TIME.TIME (the return variable) WHICH
/// CANNOT BE HANDLED PROPERLY. VALIDATION SHOULD FAIL!
#[test]
#[ignore = "worked in the past, should fail!"]
fn variable_with_same_name_as_function() {
    let result = codegen(
        "
        FUNCTION TIME : TIME
        VAR
            TIME : TIME;
        END_VAR
        END_FUNCTION
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn expression_list_as_array_initilization() {
    let result = codegen(
        "
        VAR_GLOBAL
            arr : ARRAY[0..3] OF INT := 1, 2, 3;
            b_exp : ARRAY[0..4] OF DINT := 1+3, 2*3, 7-1, 10;
            str : ARRAY[0..2] OF STRING := 'first', 'second';
        END_VAR
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn default_values_for_not_initialized_function_vars() {
    let result = codegen(
        "
        FUNCTION func : INT
        VAR
            int_var : INT;
            arr_var : ARRAY[0..2] OF DINT;
            ptr_var : REF_TO DINT;
            float_var   : REAL;
        END_VAR
        END_FUNCTION
        ",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn order_var_and_var_temp_block() {
    // GIVEN a program with defined VAR_TEMP before VAR block
    let result = codegen(
        "
        PROGRAM main
        VAR_TEMP
            temp : INT;
        END_VAR
        VAR
            var1 : INT;
        END_VAR
        END_PROGRAM
        ",
    );
    // codegen should be successful
    insta::assert_snapshot!(result);
}

#[test]
fn constant_expressions_in_ranged_type_declaration_are_propagated() {
    //GIVEN a ranged type from 0 .. MIN+1 where MIN is a global constant
    //WHEN the code is generated
    let result = codegen(
        "
        VAR_GLOBAL CONSTANT
          MIN : INT := 7;
        END_VAR

        FUNCTION CheckRangeSigned: INT
          VAR_INPUT
              value : INT;
              lower : INT;
              upper : INT;
          END_VAR
          CheckRangeSigned := value;
        END_FUNCTION

        PROGRAM prg
          VAR
            x: INT(0 .. MIN+1);
          END_VAR
          x := 5;
        END_PROGRAM",
    );

    // THEN we expect that the assignment to the range-typed variable (x := 5) will result
    // in a call to CheckRangedSigned where the upper bound is a literal i16 8 - NOT an
    // add-expression that really calculates the upper bound at runtime

    insta::assert_snapshot!(result);
}

#[test]
fn constant_expression_in_function_blocks_are_propagated() {
    //GIVEN a constant in a function block
    //WHEN the code is generated
    let result = codegen(
        "
        FUNCTION_BLOCK fbWithConstant
        VAR
            x : INT;
        END_VAR
        VAR CONSTANT
            const : INT := 2;
        END_VAR
          x := const;
        END_FUNCTION
        ",
    );

    // THEN we expect that the assignment to the variable (x := const) will be replaced
    // With x := 2

    insta::assert_snapshot!(result);
}

#[test]
fn date_and_time_addition_in_var_output() {
    //GIVEN a date and time and a time addition on output variables
    //WHEN the code is generated
    let result = codegen(
        "
        FUNCTION func : DINT
        VAR_OUTPUT
            d_and_t : DT;
            time_var : TIME;
        END_VAR
            d_and_t := d_and_t + time_var;
        END_FUNCTION
        ",
    );

    //Then the time variable is added to the date time variable
    insta::assert_snapshot!(result);
}

#[test]
fn date_and_time_global_constants_initialize() {
    //GIVEN date time constants with each possible prefix
    let src = r#"
    VAR_GLOBAL CONSTANT
        cT          : TIME              := TIME#1s;
        cT_SHORT    : TIME              := T#1s;
        cLT         : LTIME             := LTIME#1000s;
        cLT_SHORT   : LTIME             := LT#1000s;
        cD          : DATE              := DATE#1970-01-01;
        cD_SHORT    : DATE              := D#1975-02-11;
        cLD         : LDATE             := LDATE#1975-02-11;
        cLD_SHORT   : LDATE             := LD#1975-02-11;
        cTOD        : TIME_OF_DAY       := TIME_OF_DAY#00:00:00;
        cTOD_SHORT  : TOD               := TOD#00:00:00;
        cLTOD       : LTOD              := LTIME_OF_DAY#23:59:59.999999999;
        cLTOD_SHORT : LTOD              := LTOD#23:59:59.999999999;
        cDT         : DATE_AND_TIME     := DATE_AND_TIME#1970-01-02-23:59:59;
        cDT_SHORT   : DT                := DT#1970-01-02-23:59:59;
        cLDT        : LDT               := LDATE_AND_TIME#1970-01-02-23:59:59.123;
        cLDT_SHORT  : LDT               := LDT#1970-01-02-23:59:59.123;
    END_VAR

    PROGRAM main
    VAR_TEMP
        t1      : TIME;
        t2      : TIME;
        lt1     : LTIME;
        lt2     : LTIME;
        d1      : DATE;
        d2      : DATE;
        ld1     : LDATE;
        ld2     : LDATE;
        tod1    : TIME_OF_DAY;
        tod2    : TOD;
        ltod1   : LTOD;
        ltod2   : LTOD;
        dt1     : DATE_AND_TIME;
        dt2     : DT;
        ldt1    : LDT;
        ldt2    : LDT;
    END_VAR

        t1      := cT;
        t2      := cT_SHORT;
        lt1     := cLT;
        lt2     := cLT_SHORT;
        d1      := cD;
        d2      := cD_SHORT;
        ld1     := cLD;
        ld2     := cLD_SHORT;
        tod1    := cTOD;
        tod2    := cTOD_SHORT;
        ltod1   := cLTOD;
        ltod2   := cLTOD_SHORT;
        dt1     := cDT;
        dt2     := cDT_SHORT;
        ldt1    := cLDT;
        ldt2    := cLDT_SHORT;
    END_PROGRAM"#;

    let result = codegen(src);
    // THEN the variables are initialized correctly
    insta::assert_snapshot!(result);
}

#[test]
fn contants_in_case_statements_resolved() {
    let result = codegen(
        "
        PROGRAM main
            VAR
                DAYS_IN_MONTH : DINT;
            END_VAR
            VAR CONSTANT
                SIXTY : DINT := 60;
            END_VAR
            CASE DAYS_IN_MONTH OF
              32..SIXTY :   DAYS_IN_MONTH := 29;
              (SIXTY    + 2)..70 :  DAYS_IN_MONTH := 30;
            ELSE
              DAYS_IN_MONTH := 31;
            END_CASE;
        END_PROGRAM
        ",
    );

    // THEN the first case should be 32..60
    // AND the second case should be 62..70
    insta::assert_snapshot!(result);
}

#[test]
fn sub_range_check_functions() {
    // GIVEN
    let result = codegen(
        "
    FUNCTION CheckRangeSigned : DINT
        VAR_INPUT v: DINT; low: DINT; up: DINT; END_VAR
        CheckRangeSigned := -7;
    END_FUNCTION

    FUNCTION CheckRangeUnsigned : UDINT
        VAR_INPUT v: UDINT; low: UDINT; up: UDINT; END_VAR
        CheckRangeUnsigned := 7;
    END_FUNCTION

    FUNCTION CheckLRangeSigned : LINT
        VAR_INPUT v: LINT; low: LINT; up: LINT; END_VAR
        CheckLRangeSigned := -77;
    END_FUNCTION

    FUNCTION CheckLRangeUnsigned : ULINT
        VAR_INPUT v: ULINT; low: ULINT; up: ULINT; END_VAR
        CheckLRangeUnsigned := 77;
    END_FUNCTION

    PROGRAM main
    VAR
        a   : BYTE(0 .. 100);
        b   : SINT(-100 .. 100);
        c   : USINT(0 .. 100);
        d   : WORD(0 .. 100);
        e   : INT(-100 .. 100);
        f   : UINT(0 .. 100);
        g   : DINT(-100 .. 100);
        h   : UDINT(0 .. 100);
        i   : LINT(-100 .. 100);
        j   : ULINT(0 .. 100);
    END_VAR
    a := 1; b := 1; c := 1; d := 1; e := 1;
    f := 1; g := 1; h := 1; i := 1; j := 1;
    END_PROGRAM
    ",
    );

    // THEN for every assignment a check function should be called
    // with the correct type cast for parameters and return type
    insta::assert_snapshot!(result);
}

#[test]
fn reference_to_reference_assignments_in_function_arguments() {
    let result = codegen(
        r#"
    VAR_GLOBAL
        global1 : STRUCT_params;
        global2 : STRUCT_params;
        global3 : STRUCT_params;

        global4 : DINT;
        global5 : STRING;
        global6 : REAL;
    END_VAR

    TYPE STRUCT_params :
        STRUCT
            param1 : BOOL;
            param2 : BOOL;
            param3 : BOOL;
        END_STRUCT
    END_TYPE

    PROGRAM prog
        VAR_INPUT
            input1 : REF_TO STRUCT_params;
            input2 : REF_TO STRUCT_params;
            input3 : REF_TO STRUCT_params;
        END_VAR
    END_PROGRAM

    PROGRAM main
        prog(
            // ALL of these should have an identical IR representation
            input1 := ADR(global1),
            input2 := REF(global2),
            input3 := &global3
        );

        prog(
            // These are not valid but we want to see if there's a cast involved
            input1 := ADR(global4),
            input2 := REF(global5),
            input3 := &global6
        );
    END_PROGRAM
    "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn sizeof_works_in_binary_expression_with_different_size() {
    let result = codegen(
        r#"
        FUNCTION main : DINT
        VAR
            i : DINT;
            j : UINT;
            arr_ptr : REF_TO ARRAY[1..3] OF REAL;
        END_VAR
            i := j - SIZEOF(arr_ptr);
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
#[ignore]
fn function_and_struct_with_same_names() {
    // See description of [`index::get_container_members_filtered`]
    // for reasoning why this test case exists.
    let result = codegen(
        "
        FUNCTION FOO : FOO
        END_FUNCTION

        TYPE FOO : STRUCT
            bar : DINT;
        END_STRUCT
        END_TYPE
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn array_of_struct_as_member_of_another_struct_is_initialized() {
    let res = codegen(
        "
        PROGRAM mainProg
        VAR
            var_str1 : STRUCT1 := ((myInt := 10), (myArr := [(x1 := TRUE, x2 := 128), (x1 := FALSE, x2 := 1024)]);
        END_VAR
        END_PROGRAM

        TYPE STRUCT1 :
            STRUCT
                myInt : INT;
                myArr : ARRAY[0..10] OF STRUCT2;
            END_STRUCT
        END_TYPE

        TYPE STRUCT2 :
            STRUCT
                x1 : BOOL;
                x2 : DINT;
            END_STRUCT
        END_TYPE
       ",
    );

    insta::assert_snapshot!(res);
}

#[test]
fn array_of_struct_as_member_of_another_struct_and_variable_declaration_is_initialized() {
    let res = codegen(
        "
        PROGRAM mainProg
        VAR
            var_str1 : ARRAY[1..5] OF STRUCT1 := [
                (myInt := 1, myArr := [(x1 := TRUE, x2 := 128), (x1 := FALSE, x2 := 1024)]),
                (myInt := 2, myArr := [(x1 := TRUE, x2 := 256), (x1 := FALSE, x2 := 2048)])
            ];
        END_VAR
        END_PROGRAM

        TYPE STRUCT1 :
            STRUCT
                myInt : INT;
                myArr : ARRAY[0..4] OF STRUCT2;
            END_STRUCT
        END_TYPE

        TYPE STRUCT2 :
            STRUCT
                x1 : BOOL;
                x2 : DINT;
            END_STRUCT
        END_TYPE
       ",
    );

    insta::assert_snapshot!(res);
}
