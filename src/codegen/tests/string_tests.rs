use plc_util::filtered_assert_snapshot;

// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::test_utils::tests::{codegen, codegen_without_unwrap};

#[test]
fn variable_string_assignment_test() {
    // GIVEN some string assignments
    let result = codegen(
        r"
PROGRAM prg
   VAR
      y : STRING[15];
      z : STRING[30] := 'xyz';
   END_VAR

   y := z;
   z := y;
END_PROGRAM
    ",
    );

    // THEN we dont want that y := z will overwrite the last byte of the y-vector (null-terminator)
    filtered_assert_snapshot!(result);
}

#[test]
fn casted_string_assignment_uses_memcpy() {
    // GIVEN some string assignments
    let result = codegen(
        r#"
    PROGRAM prg
    VAR
        a : STRING;
        b : WSTRING;
    END_VAR

    a := STRING#"abc";
    a := STRING#'abc';

    b := WSTRING#"abc";
    b := WSTRING#'abc';

    END_PROGRAM
    "#,
    );

    // THEN we expect the assignments to use memcpy, no stores!
    filtered_assert_snapshot!(result);
}

#[test]
fn vartmp_string_init_test() {
    let result = codegen(
        r"
PROGRAM prg
   VAR_TEMP
      y : STRING[15];
      z : STRING[30] := 'xyz';
   END_VAR

END_PROGRAM
    ",
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn simple_string_test() {
    let result = codegen(
        r"
VAR_GLOBAL
    str: STRING[20];
    wstr: WSTRING[20];
END_VAR
    ",
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn program_with_casted_string_assignment() {
    let result = codegen(
        r#"PROGRAM prg
VAR
  y : STRING;
  z : WSTRING;
END_VAR

// cast a WSTRING to a STRING
y := STRING#"im a genius";
// cast a STRING to a WSTRING
z := WSTRING#'im a utf16 genius';
END_PROGRAM
"#,
    );
    filtered_assert_snapshot!(result);
}

#[test]
fn generate_with_invalid_casted_string_assignment() {
    let result = codegen_without_unwrap(
        r#"PROGRAM prg
VAR
  y : INT;
END_VAR
y := INT#"seven";
END_PROGRAM
"#,
    )
    .unwrap_err();
    filtered_assert_snapshot!(result)
}

#[test]
fn program_with_string_type_assignment() {
    let result = codegen(
        r#"
TYPE MyString: STRING[99] := 'abc'; END_TYPE
TYPE MyWString: WSTRING[99] := "abc"; END_TYPE

PROGRAM prg
VAR
y : STRING;
z : MyString;
zz : MyWString;
END_VAR
y := 'im a genius';
z := 'im also a genius';
zz := "im also a genius";
END_PROGRAM
"#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn variable_length_strings_can_be_created() {
    let result = codegen(
        r#"PROGRAM prg
          VAR
          y : STRING[15];
          z : STRING[3] := 'xyz';
          wy : WSTRING[15];
          wz : WSTRING[3] := "xyz";
          END_VAR
          y := 'im a genius';
          wy := "im a genius";
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn function_returns_a_literal_string() {
    let program = codegen(
        r#"
        FUNCTION ret : STRING
            ret := 'abc';
        END_FUNCTION

        PROGRAM main
            VAR
                str: STRING;
            END_VAR
            str := ret();
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(program);
}

#[test]
fn function_takes_string_paramter_and_returns_string() {
    let program = codegen(
        r#"
        FUNCTION read_string : STRING
            VAR_INPUT
                to_read : STRING;
            END_VAR

            read_string := to_read;
        END_FUNCTION

        PROGRAM main
            VAR
                text1 : STRING;
            END_VAR

            text1 := read_string('abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc');
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(program);
}

#[test]
fn variable_length_strings_using_constants_can_be_created() {
    let result = codegen(
        r#"
        VAR_GLOBAL CONSTANT
          LONG_STRING : INT := 15;
          SHORT_STRING : INT := 3;
        END_VAR

        PROGRAM prg
          VAR
          y : STRING[LONG_STRING];
          z : STRING[SHORT_STRING] := 'xyz';
          wy : WSTRING[2 * LONG_STRING];
          wz : WSTRING[2 * SHORT_STRING] := "xyz";
          END_VAR
          y := 'im a genius';
          wy := "im a genius";
        END_PROGRAM
        "#,
    );

    filtered_assert_snapshot!(result);
}

//from OSCAT
#[test]
fn nested_struct_initialization_of_multi_dim_string_arrays() {
    let result = codegen(
        r#"
        TYPE CONSTANTS_LANGUAGE :
        STRUCT
            (* Language Setup *)
            DEFAULT : INT := 1; (* 1=english, 2=german 3=french *)
            LMAX : INT := 3;
            WEEKDAYS : ARRAY[1..3, 1..7] OF STRING[10] :=   ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday',
                                                            'Montag', 'Dienstag', 'Mittwoch', 'Donnerstag', 'Freitag', 'Samstag', 'Sonntag',
                                                            'Lundi', 'Mardi', 'Mercredi', 'Jeudi', 'Vendredi', 'Samedi', 'Dimanche'];
            WEEKDAYS2 : ARRAY[1..3, 1..7] OF STRING[2] :=   ['Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa', 'Su',
                                                            'Mo', 'Di', 'Mi', 'Do', 'Fr', 'Sa', 'So',
                                                            'Lu', 'Ma', 'Me', 'Je', 'Ve', 'Sa', 'Di'];
            MONTHS : ARRAY[1..3, 1..12] OF STRING[10] :=    ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December',
                                                            'Januar', 'Februar', 'März', 'April', 'Mai', 'Juni', 'Juli', 'August', 'September', 'Oktober', 'November', 'Dezember',
                                                            'Janvier', 'Février', 'mars', 'Avril', 'Mai', 'Juin', 'Juillet', 'Août', 'Septembre', 'Octobre', 'Novembre', 'Decembre'];
            MONTHS3 : ARRAY[1..3, 1..12] OF STRING[3] :=    ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec',
                                                            'Jan', 'Feb', 'Mrz', 'Apr', 'Mai', 'Jun', 'Jul', 'Aug', 'Sep', 'Okt', 'Nov', 'Dez',
                                                            'Jan', 'Fev', 'Mar', 'Avr', 'Mai', 'Jun', 'Jul', 'Aou', 'Sep', 'Oct', 'Nov', 'Dec'];
            DIRS : ARRAY[1..3,0..15] OF STRING[3] :=        ['N', 'NNE', 'NE', 'ENE', 'E', 'ESE', 'SE', 'SSE', 'S', 'SSW', 'SW', 'WSW', 'W', 'WNW', 'NW', 'NNW',
                                                            'N', 'NNO', 'NO', 'ONO', 'O', 'OSO', 'SO', 'SSO', 'S', 'SSW', 'SW', 'WSW', 'W', 'WNW', 'NW', 'NNW',
                                                            'N', 'NNO', 'NO', 'ONO', 'O', 'OSO', 'SO', 'SSO', 'S', 'SSW', 'SW', 'WSW', 'W', 'WNW', 'NW', 'NNW'];
        END_STRUCT
        END_TYPE

        VAR_GLOBAL x : CONSTANTS_LANGUAGE; END_VAR
        "#,
    );
    filtered_assert_snapshot!(result);
}

#[test]
fn string_function_parameters() {
    let result = codegen(
        r#"
        FUNCTION foo: INT
            VAR_INPUT
                s : STRING;
            END_VAR

            RETURN 0;
        END_PROGRAM


        PROGRAM prg
            VAR
                s : STRING[10] := 'hello';
                a : STRING;
            END_VAR

            a := s;
            a := 'hello';
            foo(s);
            foo('hello');
        END_PROGRAM

        "#,
    );

    filtered_assert_snapshot!(result);
}

#[test]
fn program_string_output() {
    // GIVEN PROGRAM returning strings
    let result = codegen(
        r#"
        PROGRAM prog
        VAR_OUTPUT
            output1 : STRING;
            output2 : WSTRING;
        END_VAR
            output1 := 'string';
            output2 := "wstring";
        END_PROGRAM

        PROGRAM main
        VAR
            x : STRING[6];
            y : WSTRING[7];
        END_VAR
            prog(x, y);
        END_PROGRAM
    "#,
    );

    // THEN
    filtered_assert_snapshot!(result);
}

#[test]
fn function_returning_generic_string_should_return_by_ref() {
    // GIVEN PROGRAM returning generic strings
    let result = codegen(
        r#"
        FUNCTION MID <T: ANY_STRING> : T
        VAR_INPUT {ref}
            IN : T;
        END_VAR
        VAR_INPUT
            L  : DINT;
            P  : DINT;
        END_VAR
        END_FUNCTION

        FUNCTION MID__STRING : STRING
        VAR_INPUT {ref}
            IN : STRING;
        END_VAR
        VAR_INPUT
            L  : DINT;
            P  : DINT;
        END_VAR
            MID__STRING := 'abc';
        END_FUNCTION

        PROGRAM main
            VAR_INPUT
                fmt : STRING[60];
                x : STRING;
            END_VAR
            x := MID(fmt, 1,2);
        END_PROGRAM
    "#,
    );

    // THEN
    filtered_assert_snapshot!(result);
}

#[test]
fn function_var_constant_strings_should_be_collected_as_literals() {
    // GIVEN FUNCTION with var constant literal initializers
    let result = codegen(
        r#"
        FUNCTION FSTRING_TO_DT : DT
            VAR CONSTANT
                ignore: STRING[1] := '*';  (* ignore character is * *)
            END_VAR
            VAR
                fchar : STRING[1] := '#';  (* format character is # *)
            END_VAR

            fchar := '#';
            ignore := '*';

        END_FUNCTION
    "#,
    );

    // THEN we should see global variables for * and #
    filtered_assert_snapshot!(result);
}

#[test]
fn using_a_constant_var_string_should_be_memcpyable_nonref() {
    //regression test that used to break in IF c = ignore because ignore had troubles
    //when it tried to load the constant string
    let result = codegen(
        r#"
        FUNCTION STRING_EQUAL : BOOL
            VAR_INPUT op1, op2: STRING[1024]; END_VAR
        END_FUNCTION

        FUNCTION FSTRING_TO_DT : DT
            VAR CONSTANT
                ignore: STRING[1] := '*';  (* ignore character is * *)
                fchar : STRING[1] := '#';  (* format character is # *)
            END_VAR
            VAR
                c: STRING[1];
            END_VAR

            IF c = ignore THEN
                (* skip ignore characters *)
            END_IF;

        END_FUNCTION
    "#,
    );

    // THEN
    filtered_assert_snapshot!(result);
}

#[test]
fn using_a_constant_var_string_should_be_memcpyable() {
    //regression test that used to break in IF c = ignore because ignore had troubles
    //when it tried to load the constant string
    let result = codegen(
        r#"
        FUNCTION STRING_EQUAL : BOOL
            VAR_INPUT {ref} op1, op2: STRING[1024] END_VAR
        END_FUNCTION

        FUNCTION FSTRING_TO_DT : DT
            VAR CONSTANT
                ignore: STRING[1] := '*';  (* ignore character is * *)
                fchar : STRING[1] := '#';  (* format character is # *)
            END_VAR
            VAR
                c: STRING[1];
            END_VAR

            IF c = ignore THEN
                (* skip ignore characters *)
            END_IF;

        END_FUNCTION
    "#,
    );

    // THEN
    filtered_assert_snapshot!(result);
}

#[test]
#[ignore = "missing validation for literal string assignments"]
fn assigning_utf8_literal_to_wstring() {
    let result = codegen(
        r#"
        PROGRAM main
        VAR
            ws: WSTRING;
        END_VAR
            ws := 'd';
        END_PROGRAM
    "#,
    );

    // THEN
    filtered_assert_snapshot!(result);
}
