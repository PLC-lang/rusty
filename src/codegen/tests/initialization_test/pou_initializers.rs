use crate::test_utils::tests::codegen;
use plc_util::filtered_assert_snapshot;

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

    filtered_assert_snapshot!(result)
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

    filtered_assert_snapshot!(result)
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

    filtered_assert_snapshot!(result)
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

    filtered_assert_snapshot!(result)
}

#[test]
fn default_values_for_not_initialized_function_vars() {
    let result = codegen(
        "
        FUNCTION func : INT
        VAR
            int_var : INT;
            arr_var : ARRAY[-1..2] OF DINT;
            ptr_var : REF_TO DINT;
            float_var   : REAL;
        END_VAR
        END_FUNCTION
        ",
    );
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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

    filtered_assert_snapshot!(function)
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
    filtered_assert_snapshot!(function)
}

#[test]
fn function_return_value_is_initialized() {
    let function = codegen(
        r"
        TYPE MyStruct: STRUCT
          a: DINT;
          b: INT;
        END_STRUCT
        END_TYPE

        FUNCTION foo_int : INT
        END_FUNCTION

        FUNCTION foo_str : STRING[10]
        END_FUNCTION

        FUNCTION foo_arr : ARRAY[0..9] OF REAL
        END_FUNCTION

        FUNCTION foo_struct : MyStruct
        END_FUNCTION
        ",
    );
    //expect 0-initialization
    filtered_assert_snapshot!(function)
}

#[test]
fn function_return_value_is_initialized_with_type_initializer() {
    let function = codegen(
        r"
          TYPE myArray : ARRAY[0..3] OF DINT := [1,2,3,4]; END_TYPE

          FUNCTION target : myArray
            target[2] := 7;
          END_FUNCTION

          PROGRAM main
            VAR
              x : ARRAY[0..3] OF DINT;
              y : ARRAY[0..3] OF DINT;
            END_VAR
            x := target();
            y := x;
          END_PROGRAM
        ",
    );
    //expect [1,2,3,4]-initialization of function's out pointer
    filtered_assert_snapshot!(function)
}

#[test]
fn function_return_value_with_initializers_is_initialized() {
    // GIVEN a custom Int, a custom String, a custom Array and a custom struct with initializers
    // AND functions that return these types without touching them
    let function = codegen(
        r"
        TYPE MyInt : INT := 7; END_TYPE;
        TYPE MyStr : STRING[10] := 'init'; END_TYPE;
        TYPE MyArr : ARRAY[0..9] OF REAL := [0.0, 1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9]; END_TYPE;
        TYPE MyStrct : STRUCT a : DINT := 1; b : DINT := 2; c : DINT := 3; END_STRUCT END_TYPE;

        FUNCTION foo_int : MyInt
        END_FUNCTION

        FUNCTION foo_str : MyStr
        END_FUNCTION

        FUNCTION foo_arr : MyArr
        END_FUNCTION

        FUNCTION foo_strct : MyStrct
        END_FUNCTION
        ",
    );
    //THEN I expect datatype's initials as declared
    // store 7 to foo_int return
    // memcpy from MyStr__init to foo_str
    // memcpy from MyArr__init to foo_arr
    // memcpy from MyStrct__init to foo_strct
    filtered_assert_snapshot!(function)
}

#[test]
fn function_return_value_without_initializers_is_initialized() {
    // GIVEN a custom Int, a custom String, a custom Array and a custom struct without initializers
    // AND functions that return these types without touching them
    let function = codegen(
        r"
        TYPE MyInt : INT; END_TYPE;
        TYPE MyStr : STRING[10]; END_TYPE;
        TYPE MyArr : ARRAY[0..9] OF REAL; END_TYPE;
        TYPE MyStrct : STRUCT a : DINT; b : DINT; c : DINT; END_STRUCT END_TYPE;

        FUNCTION foo_int : MyInt
        END_FUNCTION

        FUNCTION foo_str : MyStr
        END_FUNCTION

        FUNCTION foo_arr : MyArr
        END_FUNCTION

        FUNCTION foo_strct : MyStrct
        END_FUNCTION
        ",
    );
    //THEN I expect returns are initialized to 0
    // store 0 to foo_int return
    // memset 0 to foo_str
    // memset 0 to foo_arr
    // memcpy from zeroinitializer global to foo_strct
    filtered_assert_snapshot!(function)
}

#[test]
fn two_identical_enums_in_different_functions_are_referenced_correctly() {
    let function = codegen(
        r"
        FUNCTION foo : DINT
            VAR
                position : (x := 1, y := 2) := x;
            END_VAR
        END_FUNCTION

        FUNCTION bar : DINT
            VAR
                position : (x := 3, y := 4) := x;
            END_VAR
        END_FUNCTION
       ",
    );

    // We want to ensure that the `position` variable in bar has a value of 3 instead of 1.
    // Previously this was not the case, because the index wouldn't find the locally defined `x`
    // variant in `bar` and instead referenced the `x` in `foo`.
    // See also https://github.com/PLC-lang/rusty/pull/1092
    filtered_assert_snapshot!(function)
}

#[test]
fn two_identical_enums_in_different_functions_with_similar_names_are_referenced_correctly() {
    let function = codegen(
        r"
        FUNCTION a : DINT
            VAR position : (x := 1, y := 5) := x;   END_VAR
        END_FUNCTION

        FUNCTION aa : DINT
            VAR position : (x := 2, y := 5) := x;   END_VAR
        END_FUNCTION

        FUNCTION bb : DINT
            VAR position : (x := 3, y := 5) := x;   END_VAR
        END_FUNCTION

        FUNCTION b : DINT
            VAR position : (x := 4, y := 5) := x;   END_VAR
        END_FUNCTION
       ",
    );

    // We want to ensure that each local `position` enum gets a correct `x` value assigned, i.e.
    // a.x == 1, aa.x == 2, bb.x == 3, b.x == 4
    filtered_assert_snapshot!(function)
}

#[test]
fn enum_variants_have_precedence_over_global_variables_in_inline_assignment() {
    let function = codegen(
        r"
        VAR_GLOBAL
            x : DINT := 10;
        END_VAR

        FUNCTION foo : DINT
            VAR
                position : (x := 1, y := 2) := x;
            END_VAR
        END_FUNCTION

        FUNCTION bar : DINT
            VAR
                position : (x := 3, y := 4) := x;
            END_VAR
        END_FUNCTION
       ",
    );

    // We want to ensure that both the `position` assignment in `foo` and `bar` references
    // the enum variant `x` rather than the global variable `x`
    filtered_assert_snapshot!(function)
}

#[test]
fn unary_plus_in_initializer() {
    let result = codegen(
        "
        VAR_GLOBAL CONSTANT g1 : INT := 5; END_VAR
        PROGRAM exp
        VAR
            x : INT := +g1;
            y : REAL := +3.14;
        END_VAR
        END_PROGRAM
        ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %exp = type { i16, float }

    @g1 = unnamed_addr constant i16 5
    @exp_instance = global %exp { i16 5, float 0x40091EB860000000 }

    define void @exp(ptr %0) {
    entry:
      %x = getelementptr inbounds nuw %exp, ptr %0, i32 0, i32 0
      %y = getelementptr inbounds nuw %exp, ptr %0, i32 0, i32 1
      ret void
    }
    "#);
}
