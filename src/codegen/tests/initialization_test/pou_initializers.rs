use crate::test_utils::tests::codegen;

#[test]
fn initial_constant_values_in_pou_variables() {
    let result = codegen(
        r#"
        VAR_GLOBAL CONSTANT
        MAX_LEN : INT := 99;
        MIN_LEN : INT := 10;
        LEN : INT := MIN_LEN + 10;
        END_VAR
 
        PROGRAM prg
      	  VAR_INPUT
            my_len: INT := LEN + 4;
            my_size: INT := MAX_LEN - MIN_LEN;
          END_VAR
        END_PROGRAM
 
        "#,
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%prg_interface = type { i16, i16 }

@MAX_LEN = global i16 99
@MIN_LEN = global i16 10
@LEN = global i16 20
@prg_instance = global %prg_interface { i16 24, i16 89 }

define void @prg(%prg_interface* %0) {
entry:
  %my_len = getelementptr inbounds %prg_interface, %prg_interface* %0, i32 0, i32 0
  %my_size = getelementptr inbounds %prg_interface, %prg_interface* %0, i32 0, i32 1
  ret void
}
"#;

    assert_eq!(result, expected);
}
#[test]
fn initial_values_in_program_pou() {
    let result = codegen(
        "
        PROGRAM Main
        VAR
          x : INT := 7;
          xx : INT;
          y : BOOL := TRUE;
          yy : BOOL;
          z : REAL := 3.1415;
          zz : REAL;
        END_VAR
        END_PROGRAM
        ",
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%Main_interface = type { i16, i16, i1, i1, float, float }

@Main_instance = global %Main_interface { i16 7, i16 0, i1 true, i1 false, float 0x400921CAC0000000, float 0.000000e+00 }

define void @Main(%Main_interface* %0) {
entry:
  %x = getelementptr inbounds %Main_interface, %Main_interface* %0, i32 0, i32 0
  %xx = getelementptr inbounds %Main_interface, %Main_interface* %0, i32 0, i32 1
  %y = getelementptr inbounds %Main_interface, %Main_interface* %0, i32 0, i32 2
  %yy = getelementptr inbounds %Main_interface, %Main_interface* %0, i32 0, i32 3
  %z = getelementptr inbounds %Main_interface, %Main_interface* %0, i32 0, i32 4
  %zz = getelementptr inbounds %Main_interface, %Main_interface* %0, i32 0, i32 5
  ret void
}
"#;

    assert_eq!(result, expected);
}

#[test]
fn initial_values_in_function_block_pou() {
    let result = codegen(
        "
        FUNCTION_BLOCK FB
        VAR
          x : INT := 7;
          xx : INT;
          y : BOOL := TRUE;
          yy : BOOL;
          z : REAL := 3.1415;
          zz : REAL;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
        VAR
          fb : FB;
        END_VAR
        END_PROGRAM
        ",
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%FB_interface = type { i16, i16, i1, i1, float, float }
%main_interface = type { %FB_interface }

@FB__init = global %FB_interface { i16 7, i16 0, i1 true, i1 false, float 0x400921CAC0000000, float 0.000000e+00 }
@main_instance = global %main_interface { %FB_interface { i16 7, i16 0, i1 true, i1 false, float 0x400921CAC0000000, float 0.000000e+00 } }

define void @FB(%FB_interface* %0) {
entry:
  %x = getelementptr inbounds %FB_interface, %FB_interface* %0, i32 0, i32 0
  %xx = getelementptr inbounds %FB_interface, %FB_interface* %0, i32 0, i32 1
  %y = getelementptr inbounds %FB_interface, %FB_interface* %0, i32 0, i32 2
  %yy = getelementptr inbounds %FB_interface, %FB_interface* %0, i32 0, i32 3
  %z = getelementptr inbounds %FB_interface, %FB_interface* %0, i32 0, i32 4
  %zz = getelementptr inbounds %FB_interface, %FB_interface* %0, i32 0, i32 5
  ret void
}

define void @main(%main_interface* %0) {
entry:
  %fb = getelementptr inbounds %main_interface, %main_interface* %0, i32 0, i32 0
  ret void
}
"#;

    assert_eq!(result, expected);
}

#[test]
fn initial_values_in_array_of_array_variable() {
    let result = codegen(
        "
         VAR_GLOBAL 
           a : ARRAY[0..1] OF ARRAY[0..1] OF BYTE  := [[1,2],[3,4]]; 
         END_VAR
         ",
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

@a = global [2 x [2 x i8]] [[2 x i8] c"\01\02", [2 x i8] c"\03\04"]
"#;

    assert_eq!(result, expected);
}

#[test]
fn default_values_for_not_initialized_function_vars() {
    let result = codegen(
        "
		FUNCTION func : INT
		VAR
			int_var : INT;
			arr_var : ARRAY[-1..2] OF DINT;
			ptr_var	: REF_TO DINT;
			float_var	: REAL;
		END_VAR
		END_FUNCTION
		",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn memcpy_for_struct_initialization_in_function() {
    let result = codegen(
        "
		FUNCTION func : INT
		VAR
			a : STRUCT x : INT := 0; END_STRUCT
		END_VAR
		END_FUNCTION
		",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn no_memcpy_for_struct_initialization_in_program() {
    let result = codegen(
        "
		PROGRAM prog
		VAR
			a : STRUCT x : INT := 0; END_STRUCT
		END_VAR
		END_PROGRAM
		",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn function_block_struct_initialized_in_function() {
    let function = codegen(
        r"
        FUNCTION_BLOCK fb
        VAR
        END_VAR
        END_FUNCTION_BLOCK
        FUNCTION func : DINT
        VAR_INPUT
          in  : fb;
        END_VAR
        VAR
          x : fb;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
          fb0 : fb;
        END_VAR
            func(fb0);
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(function)
}

#[test]
fn class_struct_initialized_in_function() {
    let function = codegen(
        r"
        CLASS fb
        VAR
          a : INT := 9;
        END_VAR
        END_CLASS
        FUNCTION func : DINT
        VAR_INPUT
          in : fb;
        END_VAR
        VAR
          x : fb;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
          fb0 : fb;
        END_VAR
          func(fb0);
        END_PROGRAM
        ",
    );
    insta::assert_snapshot!(function)
}