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

    insta::assert_snapshot!(result)
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

    insta::assert_snapshot!(result)
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

    insta::assert_snapshot!(result)
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

    insta::assert_snapshot!(result)
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
fn initialized_array_in_function() {
    let result = codegen(
        "
		FUNCTION func : INT
		VAR
			arr_var : ARRAY[-1..2] OF DINT := [1,2,3,4];
		END_VAR
		END_FUNCTION
		",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn initialized_array_type_in_function() {
    let result = codegen(
        "
    TYPE arr : ARRAY[-1..2] OF DINT := [1,2,3,4]; END_TYPE
		FUNCTION func : INT
		VAR
			arr_var : arr;
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

#[test]
fn function_return_value_is_initialized() {
    let function = codegen(
        r"
        FUNCTION foo_int : INT
        END_FUNCTION

        FUNCTION foo_str : STRING[10]
        END_FUNCTION

        FUNCTION foo_arr : ARRAY[0..9] OF REAL
        END_FUNCTION
        ",
    );
    //expect 0-initialization
    insta::assert_snapshot!(function)
}

#[test]
#[ignore = "Default values are not being initialized correctly"]
fn function_return_value_with_initializers_is_initialized() {
    let function = codegen(
        r"
        TYPE MyInt : INT := 7; END_TYPE
        TYPE MyStr : STRING[10] := 'init'; END_TYPE
        TYPE MyArr : ARRAY[0..9] OF REAL := [0.0, 1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9]; END_TYPE

        FUNCTION foo_int : MyInt
        END_FUNCTION

        FUNCTION foo_str : MyStr
        END_FUNCTION

        FUNCTION foo_arr : MyArr
        END_FUNCTION
        ",
    );
    //expect datatype's initials
    insta::assert_snapshot!(function)
}
