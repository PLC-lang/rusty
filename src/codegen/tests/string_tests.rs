// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{
    diagnostics::Diagnostic,
    test_utils::tests::{codegen, codegen_without_unwrap},
};

#[test]
fn variable_string_assignment_test() {
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

    insta::assert_snapshot!(result);
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
    insta::assert_snapshot!(result);
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
    );

    assert_eq!(
        result,
        Err(Diagnostic::codegen_error(
            "Cannot generate String-Literal for type INT",
            (44..51).into()
        ))
    );
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

    insta::assert_snapshot!(result);
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

    insta::assert_snapshot!(result);
}

#[test]
fn function_parameters_string() {
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
            text2 : STRING;
            text3 : STRING;
        END_VAR

            text1 := read_string('abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc');
            text3 := read_string('hello');
        END_PROGRAM
        "#,
    );

    insta::assert_snapshot!(program);
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

    insta::assert_snapshot!(result);
}
