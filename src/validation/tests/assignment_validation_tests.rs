use insta::assert_snapshot;

use crate::test_utils::tests::parse_and_validate_buffered;

#[test]
fn constant_assignment_validation() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    VAR_GLOBAL CONSTANT
        v_global : BOOL;
    END_VAR

    FUNCTION main : DINT
    // CONSTANT assignment
    v_global := TRUE; // INVALID
    END_FUNCTION
    "#,
    );

    assert_snapshot!(diagnostics)
}

#[test]
fn real_assignment_validation() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    FUNCTION main : DINT
    VAR
        v_real : REAL;
        v_lreal : LREAL;

        v_udint : UDINT;

        v_dint : DINT;

        v_time : TIME;

        v_word : WORD;

        v_string : STRING;

        v_char : CHAR;

        v_tod : TOD;

        v_ptr_int : REF_TO INT;
        v_ptr_string : REF_TO STRING;

        v_arr_int_3 : ARRAY[0..3] OF INT;

        v_arr_string_3 : ARRAY[0..3] OF STRING;
    END_VAR
    // REAL
    v_real := v_lreal; // valid
    v_real := REAL#2.0; // valid
    v_real := v_udint; // valid
    v_real := UDINT#10; // valid
    v_real := v_dint; // valid
    v_real := DINT#20; // valid
    v_real := v_time; // valid
    v_real := TIME#10h20m30s; // valid
    v_real := v_word; // valid
    v_real := WORD#16#ffff; // valid
    v_real := v_string; // INVALID
    v_real := STRING#'string'; // INVALID
    v_real := 'string'; // INVALID
    v_real := v_char; // INVALID
    v_real := CHAR#'c'; // INVALID
    v_real := v_tod; // valid
    v_real := TOD#15:36:30; // valid
    v_real := v_ptr_int^; // valid
    v_real := v_ptr_string^; // INVALID
    v_real := v_arr_int_3[0]; // valid
    v_real := v_arr_string_3[0]; // INVALID
    END_FUNCTION
    "#,
    );

    assert_snapshot!(diagnostics)
}

#[test]
fn int_assignment_validation() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    FUNCTION main : DINT
    VAR
        v_lreal : LREAL;

        v_udint : UDINT;
        v_ulint : ULINT;

        v_dint : DINT;
        v_lint : LINT;

        v_time : TIME;
        v_ltime : LTIME;

        v_word : WORD;

        v_string : STRING;

        v_char : CHAR;

        v_tod : TOD;

        v_ptr_int : REF_TO INT;
        v_ptr_string : REF_TO STRING;

        v_arr_int_3 : ARRAY[0..3] OF INT;

        v_arr_string_3 : ARRAY[0..3] OF STRING;
    END_VAR
    // UNSIGNED
    v_udint := v_lreal; // valid
    v_udint := REAL#2.0; // valid
    v_udint := v_ulint; // valid
    v_udint := ULINT#10; // valid
    v_udint := v_dint; // valid
    v_udint := DINT#20; // valid
    v_udint := v_time; // valid
    v_udint := TIME#10h20m30s; // valid
    v_udint := v_word; // valid
    v_udint := WORD#16#ffff; // valid
    v_udint := v_string; // INVALID
    v_udint := STRING#'string'; // INVALID
    v_udint := 'string'; // INVALID
    v_udint := v_char; // INVALID
    v_udint := CHAR#'c'; // INVALID
    v_udint := v_tod; // valid
    v_udint := TOD#15:36:30; // valid
    v_udint := v_ptr_int^; // valid
    v_udint := v_ptr_string^; // INVALID
    v_udint := v_arr_int_3[0]; // valid
    v_udint := v_arr_string_3[0]; // INVALID

    // SIGNED
    v_dint := v_lreal; // valid
    v_dint := REAL#2.0; // valid
    v_dint := v_udint; // valid
    v_dint := UDINT#10; // valid
    v_dint := v_lint; // valid
    v_dint := LINT#20; // valid
    v_dint := v_time; // valid
    v_dint := TIME#10h20m30s; // valid
    v_dint := v_word; // valid
    v_dint := WORD#16#ffff; // valid
    v_dint := v_string; // INVALID
    v_dint := STRING#'string'; // INVALID
    v_dint := 'string'; // INVALID
    v_dint := v_char; // INVALID
    v_dint := CHAR#'c'; // INVALID
    v_dint := v_tod; // valid
    v_dint := TOD#15:36:30; // valid
    v_dint := v_ptr_int^; // valid
    v_dint := v_ptr_string^; // INVALID
    v_dint := v_arr_int_3[0]; // valid
    v_dint := v_arr_string_3[0]; // INVALID
    END_FUNCTION
    "#,
    );

    assert_snapshot!(diagnostics)
}

#[test]
fn duration_assignment_validation() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    FUNCTION main : DINT
    VAR
        v_lreal : LREAL;

        v_udint : UDINT;

        v_dint : DINT;

        v_time : TIME;
        v_ltime : LTIME;

        v_word : WORD;

        v_string : STRING;

        v_char : CHAR;

        v_tod : TOD;

        v_ptr_int : REF_TO INT;
        v_ptr_string : REF_TO STRING;

        v_arr_int_3 : ARRAY[0..3] OF INT;

        v_arr_string_3 : ARRAY[0..3] OF STRING;
    END_VAR
    // TIME
    v_time := v_lreal; // valid
    v_time := REAL#2.0; // valid
    v_time := v_udint; // valid
    v_time := UDINT#10; // valid
    v_time := v_dint; // valid
    v_time := DINT#20; // valid
    v_time := v_ltime; // valid
    v_time := LTIME#10h20m30s; // valid
    v_time := v_word; // valid
    v_time := WORD#16#ffff; // valid
    v_time := v_string; // INVALID
    v_time := STRING#'string'; // INVALID
    v_time := 'string'; // INVALID
    v_time := v_char; // INVALID
    v_time := CHAR#'c'; // INVALID
    v_time := v_tod; // valid
    v_time := TOD#15:36:30; // valid
    v_time := v_ptr_int^; // valid
    v_time := v_ptr_string^; // INVALID
    v_time := v_arr_int_3[0]; // valid
    v_time := v_arr_string_3[0]; // INVALID
    END_FUNCTION
    "#,
    );

    assert_snapshot!(diagnostics)
}

#[test]
fn bit_assignment_validation() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    FUNCTION main : DINT
    VAR
        v_lreal : LREAL;

        v_udint : UDINT;

        v_dint : DINT;

        v_time : TIME;

        v_byte : BYTE;
        v_word : WORD;

        v_string : STRING;

        v_char : CHAR;

        v_tod : TOD;

        v_ptr_int : REF_TO INT;
        v_ptr_string : REF_TO STRING;

        v_arr_int_3 : ARRAY[0..3] OF INT;

        v_arr_string_3 : ARRAY[0..3] OF STRING;
    END_VAR
    // BIT
    v_byte := v_lreal; // valid
    v_byte := REAL#2.0; // valid
    v_byte := v_udint; // valid
    v_byte := UDINT#10; // valid
    v_byte := v_dint; // valid
    v_byte := DINT#20; // valid
    v_byte := v_time; // valid
    v_byte := TIME#10h20m30s; // valid
    v_byte := v_word; // valid
    v_byte := WORD#16#ffff; // valid
    v_byte := v_string; // INVALID
    v_byte := STRING#'string'; // INVALID
    v_byte := 'string'; // INVALID
    v_byte := v_char; // INVALID
    v_byte := CHAR#'c'; // INVALID
    v_byte := v_tod; // valid
    v_byte := TOD#15:36:30; // valid
    v_byte := v_ptr_int^; // valid
    v_byte := v_ptr_string^; // INVALID
    v_byte := v_arr_int_3[0]; // valid
    v_byte := v_arr_string_3[0]; // INVALID
    END_FUNCTION
    "#,
    );

    assert_snapshot!(diagnostics)
}

#[test]
fn string_assignment_validation() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    FUNCTION main : DINT
    VAR
        v_lreal : LREAL;

        v_udint : UDINT;

        v_dint : DINT;

        v_time : TIME;

        v_word : WORD;

        v_string : STRING;
        v_str : STRING;
        v_string1 : STRING[1];
        v_wstring : WSTRING;

        v_char : CHAR;

        v_tod : TOD;

        v_ptr_int : REF_TO INT;
        v_ptr_string : REF_TO STRING;

        v_arr_int_3 : ARRAY[0..3] OF INT;

        v_arr_string_3 : ARRAY[0..3] OF STRING;
    END_VAR
    // STRING
    v_string := v_lreal; // INVALID
    v_string := REAL#2.0; // INVALID
    v_string := v_udint; // INVALID
    v_string := UDINT#10; // INVALID
    v_string := v_dint; // INVALID
    v_string := DINT#20; // INVALID
    v_string := v_time; // INVALID
    v_string := TIME#10h20m30s; // INVALID
    v_string := v_word; // INVALID
    v_string := WORD#16#ffff; // INVALID
    v_string := v_str; // valid
    v_string := STRING#'string'; // valid
    v_string := 'string'; // valid
    v_string := v_wstring; // INVALID
    v_string := WSTRING#"wstring"; // INVALID
    v_string := "wstring"; // INVALID
    v_string := v_char; // INVALID
    v_string := CHAR#'c'; // INVALID
    v_string := v_tod; // INVALID
    v_string := TOD#15:36:30; // INVALID
    v_string := v_ptr_int^; // INVALID
    v_string := v_ptr_string^; // valid
    v_string := v_arr_int_3[0]; // INVALID
    v_string := v_arr_string_3[0]; // valid
    END_FUNCTION
    "#,
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn char_assignment_validation() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    FUNCTION main : DINT
    VAR
        v_lreal : LREAL;

        v_udint : UDINT;

        v_dint : DINT;

        v_time : TIME;

        v_word : WORD;

        v_string : STRING;
        v_str : STRING;
        v_string1 : STRING[1];
        v_wstring : WSTRING;

        v_char : CHAR;
        v_wchar : WCHAR;

        v_tod : TOD;

        v_ptr_int : REF_TO INT;
        v_ptr_string : REF_TO STRING;

        v_arr_int_3 : ARRAY[0..3] OF INT;

        v_arr_string_3 : ARRAY[0..3] OF STRING;
    END_VAR
    // CHAR
    v_char := v_lreal; // INVALID
    v_char := REAL#2.0; // INVALID
    v_char := v_udint; // INVALID
    v_char := UDINT#10; // INVALID
    v_char := v_dint; // INVALID
    v_char := DINT#20; // INVALID
    v_char := v_time; // INVALID
    v_char := TIME#10h20m30s; // INVALID
    v_char := v_word; // INVALID
    v_char := WORD#16#ffff; // INVALID
    v_char := v_string1; // INVALID -> should work
    v_char := STRING#'a'; // INVALID -> should work
    v_char := 'a'; // valid
    v_char := "a"; // INVALID
    v_char := v_string; // INVALID
    v_char := STRING#'string'; // INVALID
    v_char := 'string'; // INVALID
    v_char := v_char; // valid
    v_char := CHAR#'c'; // valid
    v_char := v_wchar; // INVALID
    v_char := WCHAR#"c"; // INVALID
    v_char := v_tod; // INVALID
    v_char := TOD#15:36:30; // INVALID
    v_char := v_ptr_int^; // INVALID
    v_char := v_ptr_string^; // INVALID
    v_char := v_arr_int_3[0]; // INVALID
    v_char := v_arr_string_3[0]; // INVALID
    END_FUNCTION
    "#,
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn date_assignment_validation() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    FUNCTION main : DINT
    VAR
        v_lreal : LREAL;

        v_udint : UDINT;

        v_dint : DINT;

        v_time : TIME;

        v_word : WORD;

        v_string : STRING;

        v_char : CHAR;

        v_date : DATE;
        v_tod : TOD;

        v_ptr_int : REF_TO INT;
        v_ptr_string : REF_TO STRING;

        v_arr_int_3 : ARRAY[0..3] OF INT;

        v_arr_string_3 : ARRAY[0..3] OF STRING;
    END_VAR
    // DATE
    v_date := v_lreal; // valid
    v_date := REAL#2.0; // valid
    v_date := v_udint; // valid
    v_date := UDINT#10; // valid
    v_date := v_dint; // valid
    v_date := DINT#20; // valid
    v_date := v_time; // valid
    v_date := TIME#10h20m30s; // valid
    v_date := v_word; // valid
    v_date := WORD#16#ffff; // valid
    v_date := v_string; // INVALID
    v_date := STRING#'string'; // INVALID
    v_date := 'string'; // INVALID
    v_date := v_char; // INVALID
    v_date := CHAR#'c'; // INVALID
    v_date := v_tod; // valid
    v_date := TOD#15:36:30; // valid
    v_date := v_ptr_int^; // valid
    v_date := v_ptr_string^; // INVALID
    v_date := v_arr_int_3[0]; // valid
    v_date := v_arr_string_3[0]; // INVALID
    END_FUNCTION
    "#,
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn pointer_assignment_validation() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    FUNCTION main : DINT
    VAR
        v_real : REAL;

        v_udint : UDINT;

        v_dint : DINT;

        v_time : TIME;

        v_word : WORD;
        v_lword : LWORD;

        v_string : STRING;

        v_char : CHAR;

        v_date : DATE;

        v_ptr_int : REF_TO INT;

        v_arr_int_3 : ARRAY[0..3] OF INT;

        v_arr_string_3 : ARRAY[0..3] OF STRING;
    END_VAR
    // POINTER
    v_dint := v_ptr_int; // INVALID
    v_word := v_ptr_int; // INVALID
    v_lword := v_ptr_int; // valid
    v_ptr_int := REF(v_real); // INVALID -> TODO: should be valid
    v_ptr_int^ := v_real; // valid
    v_ptr_int := REF(v_udint); // valid
    v_ptr_int^ := v_udint; // valid
    v_ptr_int := REF(v_dint); // valid
    v_ptr_int^ := v_dint; // valid
    v_ptr_int := REF(v_time); // valid
    v_ptr_int^ := v_time; // valid
    v_ptr_int := REF(v_word); // valid
    v_ptr_int^ := v_word; // valid
    v_ptr_int := REF(v_string); // INVALID
    v_ptr_int^ := v_string; // INVALID
    v_ptr_int := REF(v_char); // INVALID -> TODO: missing validation
    v_ptr_int^ := v_char; // INVALID
    v_ptr_int := REF(v_date); // valid
    v_ptr_int^ := v_date; // valid
    v_ptr_int^ := v_arr_int_3[0]; // valid
    v_ptr_int^ := v_arr_string_3[0]; // INVALID
    END_FUNCTION
    "#,
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn array_assignment_validation() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    FUNCTION main : DINT
    VAR
        v_real : REAL;

        v_dint : DINT;

        v_string : STRING;

        v_char : CHAR;

        v_ptr_int : REF_TO INT;
        v_ptr_string : REF_TO STRING;

        v_arr_int_2 : ARRAY[0..2] OF INT;
        v_arr_int_3 : ARRAY[0..3] OF INT;
        v_arr_int_4 : ARRAY[0..4] OF INT;

        v_arr_real_3 : ARRAY[0..3] OF REAL;

        v_arr_string_3 : ARRAY[0..3] OF STRING;
        v_arr_sized_string : ARRAY[0..3] OF STRING[256];
        v_arr_sized_string1 : ARRAY[0..3] OF STRING[256];
        v_arr_sized_string2 : ARRAY[0..8] OF STRING[1256];

        v_arr_char_3 : ARRAY[0..3] OF CHAR;
    END_VAR
    // ARRAY
    v_arr_sized_string := v_arr_sized_string1; // valid
    v_arr_sized_string := v_arr_sized_string2; // INVALID
    v_arr_int_3 := v_arr_int_2; // INVALID
    v_arr_int_3 := v_arr_int_3; // valid
    v_arr_int_3 := v_arr_int_4; // INVALID
    v_arr_int_3 := v_arr_real_3; // INVALID
    v_arr_int_3 := v_arr_string_3; // INVALID
    v_arr_int_3 := v_arr_char_3; // INVALID
    v_arr_int_3 := 1, 2, 3, 4; // INVALID
    v_arr_int_3 := (1, 2, 3, 4); // valid
    v_arr_int_3 := (1, 2, 3, 4, 5, 6); // INVALID -> missing
    v_arr_int_3[0] := v_dint; // valid
    v_arr_int_3[0] := DINT#10; // valid
    v_arr_int_3[0] := v_real; // valid
    v_arr_int_3[0] := REAL#2.0; // valid
    v_arr_int_3[0] := v_string; // INVALID
    v_arr_int_3[0] := STRING#'string'; // INVALID
    v_arr_int_3[0] := 'string'; // INVALID
    v_arr_int_3[0] := v_char; // INVALID
    v_arr_int_3[0] := CHAR#'a'; // INVALID
    v_arr_int_3[0] := v_ptr_int^; // valid
    v_arr_int_3[0] := v_ptr_string^; // INVALID
    v_arr_int_3 := v_dint; // INVALID
    v_dint := v_arr_int_3; // INVALID
    END_FUNCTION
    "#,
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn struct_assignment_validation() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    TYPE STRUCT1 :
        STRUCT
            param1 : BOOL;
        END_STRUCT
    END_TYPE

    TYPE STRUCT2 :
        STRUCT
            param1 : BOOL;
            param2 : BOOL;
        END_STRUCT
    END_TYPE

    TYPE STRUCT3 :
        STRUCT
            var_struct1 : STRUCT1;
        END_STRUCT
    END_TYPE

    FUNCTION_BLOCK fb
    VAR_IN_OUT
        var_inout_struct1 : STRUCT1;
    END_VAR
    END_FUNCTION_BLOCK

    FUNCTION main : DINT
    VAR
        v_real : REAL;

        v_string : STRING;

        v_char : CHAR;

        v_struct1 : STRUCT1;
        v_struct1_2 : STRUCT1;
        v_ref_to_struct1 : REF_TO STRUCT1;

        v_struct2 : STRUCT2;

        v_struct3 : STRUCT3;

        myFB : fb;
    END_VAR
    // STRUCT
    v_real := v_struct1; // INVALID
    v_struct1 := v_real; // INVALID

    v_struct1 := v_struct1_2; // valid
    v_struct1 := v_struct2; // INVALID

    v_struct3 := (var_struct1 := v_struct1); // valid
    v_struct3 := (var_struct1 := v_struct2); // INVALID

    myFB(var_inout_struct1 := v_struct1); // valid
    myFB(var_inout_struct1 := v_struct2); // INVALID


    v_ref_to_struct1 := REF(v_struct1); // valid
    v_ref_to_struct1 := ADR(v_struct1); // valid

    v_ref_to_struct1 := ADR(v_real); // valid
    v_ref_to_struct1 := ADR(v_string); // valid
    v_ref_to_struct1 := ADR(v_char); // valid

    v_ref_to_struct1 := REF(v_real); // INVALID
    v_ref_to_struct1 := REF(v_string); // INVALID
    v_ref_to_struct1 := REF(v_char); // INVALID
    END_FUNCTION
    "#,
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn assigning_literal_with_incompatible_encoding_to_char_is_validated() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION main : DINT
        VAR
            x : CHAR;
            y : WCHAR;
        END_VAR
            x := "A";
            y := 'B';
        END_FUNCTION"#,
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn invalid_action_call_assignments_are_validated() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK fb_t
        VAR
            var1 : ARRAY[0..10] OF WSTRING;
            var2 : ARRAY[0..10] OF WSTRING;
        END_VAR
        VAR_INPUT
            in1 : DINT;
            in2 : STRING;
        END_VAR
        VAR_IN_OUT
            auto : WSTRING;
        END_VAR
        VAR_OUTPUT
            out : ARRAY[0..10] OF WSTRING;
        END_VAR
        END_FUNCTION_BLOCK

        ACTIONS fb_t
        ACTION foo
        END_ACTION
        END_ACTIONS

        FUNCTION main : DINT
        VAR
            fb: fb_t;
            arr: ARRAY[0..10] OF WSTRING;
            wstr: WSTRING;
        END_VAR
            fb.foo(auto := wstr, in1 := 12, in2 := 'hi', out => arr); // valid
            fb.foo(auto := arr, in1 := arr, in2 := arr, out => wstr); // invalid
        END_FUNCTION
        "#,
    );
    insta::assert_snapshot!(&diagnostics)
}

#[test]
fn action_call_parameters_are_only_validated_outside_of_parent_pou_contexts() {
    let diagnostics = parse_and_validate_buffered(
        "FUNCTION_BLOCK FOO_T
        VAR_IN_OUT
            arr: ARRAY[0..1] OF DINT;
        END_VAR
        VAR_TEMP
            i: DINT;
        END_VAR
            BAR(); // associated action call here does not require parameters to be passed.
        END_FUNCTION_BLOCK

        ACTIONS
        ACTION BAR
            FOR i := 0 TO 2 DO
                arr[i] := i;
            END_FOR;

            BAZ(); // we are still in the parent-pou context, no validation required
        END_ACTION

        ACTION BAZ
            FOR i := 0 TO 2 DO
                arr[i] := 0;
            END_FOR;
        END_ACTION
        END_ACTIONS

        FUNCTION main: DINT
        VAR
            fb: FOO_T;
            arr: ARRAY[0..1] OF DINT;
        END_VAR
            fb(arr);
            fb.bar(); // INVALID - we are not in the parent context and while we use a qualified call, we don't know if the variable has been initialized
        END_FUNCTION",
    );

    insta::assert_snapshot!(diagnostics);
}

#[test]
fn implicit_invalid_action_call_assignments_are_validated() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK fb_t
        VAR
            var1 : ARRAY[0..10] OF WSTRING;
            var2 : ARRAY[0..10] OF WSTRING;
        END_VAR
        VAR_INPUT
            in1 : DINT;
            in2 : STRING;
        END_VAR
        END_FUNCTION_BLOCK

        ACTIONS fb_t
        ACTION foo
        END_ACTION
        END_ACTIONS

        FUNCTION main : DINT
        VAR
            fb: fb_t;
            arr: ARRAY[0..10] OF WSTRING;
        END_VAR
            fb.foo(12, 'hi'); // valid
            fb.foo(arr, arr); // invalid
        END_FUNCTION
        "#,
    );

    assert_snapshot!(&diagnostics)
}

#[test]
fn invalid_method_call_assignments_are_validated() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        CLASS cl_t
        VAR
            x : INT := 10;
        END_VAR

        METHOD foo : DINT
        VAR_INPUT
            a : DINT;
            b : STRING;
        END_VAR
            foo := a + x;
        END_METHOD
        END_CLASS

        FUNCTION main : DINT
        VAR
            cl: cl_t;
            arr: ARRAY[0..10] OF WSTRING;
        END_VAR
            cl.foo(12, 'hi'); // valid
            cl.foo(arr, arr); // invalid
        END_FUNCTION
        "#,
    );

    assert_snapshot!(&diagnostics)
}

#[test]
fn invalid_function_block_instantiation_is_validated() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK fb_t
        VAR_INPUT
            ws : WSTRING;
        arr_32 : ARRAY[0..1] OF DINT;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM prog
        VAR
            s : STRING := 'HELLO';
            arr_64 : ARRAY[0..1] OF LINT;
            fb : fb_t;
        END_VAR
            fb(ws := s, arr_32 := arr_64); // invalid explicit
            fb(s, arr_64); // invalid implicit
        END_PROGRAM"#,
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn implicit_action_downcasts_are_validated() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK fb_t
        VAR
            var1 : ARRAY[0..10] OF WSTRING;
            var2 : ARRAY[0..10] OF WSTRING;
        END_VAR
        VAR_INPUT
            in1 : DINT;
            in2 : DWORD;
            in3 : BYTE;
        END_VAR
        END_FUNCTION_BLOCK

        ACTIONS fb_t
        ACTION foo
        END_ACTION
        END_ACTIONS

        FUNCTION main : DINT
        VAR
            fb: fb_t;
            var1 : LINT;
            var2 : LWORD;
            var3 : STRING;
        END_VAR
            fb.foo(var1, var2, var3);
        END_FUNCTION
        "#,
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn assigning_to_input_by_ref_should_deliver_improvment_suggestion() {
    let diagnostics = parse_and_validate_buffered(
            "
            FUNCTION fn : DINT
                VAR_INPUT
                    a : DINT;
                END_VAR

                VAR_INPUT {ref}
                    b : REAL;
                    c : REAL;
                END_VAR

                VAR_IN_OUT
                    d : LREAL;
                END_VAR

                a := 1;
                b := 1.0;   // This should trigger an improvment suggestion, because we are assigning a value
                c;          // This should NOT trigger an improvment suggestion, because we are NOT assigning a value
                d := 1.0;
            END_FUNCTION

            FUNCTION main : DINT
                VAR
                    a : DINT := 3;
                    b : REAL := 3.14;
                    c : REAL := 3.14;
                    d : LREAL := 3.14;
                END_VAR

                fn(a, b, c, d);
            END_FUNCTION
            ",
        );

    assert_snapshot!(diagnostics);
}

#[test]
fn string_type_alias_assignment_can_be_validated() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE MY_STR : STRING; END_TYPE
        TYPE MY_OTHER_STR: STRING[256]; END_TYPE

        PROGRAM main
        VAR
            my_str : MY_STR;
            my_other_str: MY_OTHER_STR;
            i : INT;
        END_VAR
            my_str := i;
            my_other_str := i;
        END_PROGRAM
        ",
    );

    assert_snapshot!(diagnostics);
}

#[test]
fn integral_promotion_in_expression_does_not_cause_downcast_warning() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : INT
        VAR
            a, b, c : SINT;
        END_VAR

            a := a + b + c + 100;
        END_FUNCTION
        ",
    );

    assert!(diagnostics.is_empty())
}

#[test]
fn downcast_will_report_bigger_types_in_expression() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : INT
        VAR
            a, b, c : SINT;
            d, e : DINT;
        END_VAR
            a := a + b + c + d + e;
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics)
}

#[test]
fn literals_out_of_range_for_lhs_will_result_in_downcast_warning() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : INT
        VAR
            a : SINT;
            b : USINT;
        END_VAR

            a := a + 255 + 1000;    // literals out of range of i8 -> warning
            b := b + 255;           // 255 is in range of u8 -> no warning, but will silently overflow for anything other than b == 0. Not sure if there is a better way to guard against this
            a := a + b;             // B is same size as a, but unsigned -> warning
            a := a + USINT#100;     // will fit into a, but is cast to unsigned type -> warning
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics)
}

#[test]
fn literals_out_of_range_inside_unary_expressions_will_cause_no_warning() {
    // GIVEN literals behind unary expressions (which will be annotated as DINT)
    // WHEN we validate
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : INT
        VAR
            a : INT;
        END_VAR
            a := a;
            a := a + 254;
            a := a + 254 - 20;
            a := a + 254 + (-(10+10));  //rewrite this as a unary expression
        END_FUNCTION
        ",
    );
    //WE EXPECT NO VALIDATION PROBLEMS
    assert!(diagnostics.is_empty())
}

#[test]
fn literals_out_of_range_in_a_modulo_operation_cannot_exceed_the_left_operand() {
    // GIVEN an expression INT MOD DINT
    // WHEN we validate
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : INT
        VAR
            a : INT;
        END_VAR
            a := a + (a MOD 40000);
        END_FUNCTION
        ",
    );
    //THEN we expect no validation problems, since a mod d should remain in a's datatype
    assert!(diagnostics.is_empty())
}

#[test]
fn modulo_operation_validates_if_the_left_hand_type_fits_into_the_target_type() {
    // GIVEN an expression INT MOD DINT
    // WHEN we validate
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : INT
        VAR
            a : INT;
            d : DINT := 9;
        END_VAR
            a := a + (d MOD a);
        END_FUNCTION
        ",
    );
    //THEN we expect a downcast warning, since d mod a might overflow INT
    assert_snapshot!(diagnostics)
}

#[test]
fn rhs_of_a_mod_operation_is_ignored_in_downcast_validation() {
    // GIVEN an expression INT MOD DINT
    // WHEN we validate
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : INT
        VAR
            a : INT;
            d : DINT := 9;
            l : LINT;
        END_VAR
            a := a + (a MOD d);
            a := a + (a MOD LINT#10);
            a := a + (a MOD l);
        END_FUNCTION
        ",
    );
    //THEN we expect a downcast warning, since d mod a might overflow INT
    assert!(diagnostics.is_empty())
}

#[test]
fn builtin_sel_does_not_report_false_positive_downcasts_for_literals() {
    // GIVEN an expression INT MOD DINT
    // WHEN we validate
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : INT
        VAR
            a : INT;
            b : INT;
        END_VAR
            a := SEL(TRUE, a, b);
            a := SEL(FALSE, a, 1000);
        END_FUNCTION
        ",
    );
    //THEN we expect no validation problems, since all arguments to SEL fit into the target type
    assert!(diagnostics.is_empty())
}

#[test]
fn builtin_mux_does_not_report_false_positive_downcasts() {
    // GIVEN an expression INT MOD DINT
    // WHEN we validate
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : INT
        VAR
            a : INT;
            b : INT;
        END_VAR
            a := MUX(0, a, b);
            a := MUX(2, a, b, 1000);
        END_FUNCTION
        ",
    );
    //THEN we expect no validation problems, since all arguments to MUX fit into the target type
    assert!(diagnostics.is_empty())
}

#[test]
fn builtins_report_downcasts_depending_on_parameters() {
    // GIVEN an expression INT MOD DINT
    // WHEN we validate
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : INT
        VAR
            a : INT;
            b : DINT;
        END_VAR
            a := SEL(TRUE, a, b); // b: DINT => INT
            a := MUX(0, a, b); // b: DINT => INT
            a := MUX(b, a, 1000); // selector arg is ignored
        END_FUNCTION
        ",
    );
    //THEN we expect individual parameters to be validated and the selector argument to be ignored
    assert_snapshot!(diagnostics)
}

#[test]
fn call_results_are_validated_for_downcasts() {
    // GIVEN a call-result assignment DINT to INT
    // WHEN we validate
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION foo : DINT
            foo := 9;
        END_FUNCTION
        FUNCTION main : INT
        VAR
            a : INT;
        END_VAR
            a := FOO();
        END_FUNCTION
        ",
    );
    //THEN we expect a downcast validation
    assert_snapshot!(diagnostics)
}

#[test]
fn void_assignment_validation() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION foo
            VAR_INPUT
                x: LINT;
            END_VAR
        END_FUNCTION

        FUNCTION main : DINT
            VAR
                x : LINT;
            END_VAR

            x := foo(x);
            x := foo(foo(x));
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E037]: Invalid assignment: cannot assign 'VOID' to 'LINT'
       ┌─ <internal>:13:13
       │
    13 │             x := foo(x);
       │             ^^^^^^^^^^^ Invalid assignment: cannot assign 'VOID' to 'LINT'

    error[E037]: Invalid assignment: cannot assign 'VOID' to 'LINT'
       ┌─ <internal>:14:22
       │
    14 │             x := foo(foo(x));
       │                      ^^^^^^ Invalid assignment: cannot assign 'VOID' to 'LINT'

    error[E037]: Invalid assignment: cannot assign 'VOID' to 'LINT'
       ┌─ <internal>:14:13
       │
    14 │             x := foo(foo(x));
       │             ^^^^^^^^^^^^^^^^ Invalid assignment: cannot assign 'VOID' to 'LINT'
    ")
}

#[test]
fn ref_assignments() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main
            VAR
                localINT    : INT;
                localDINT   : DINT;
                localSTRING : STRING;

                localRefTo          : REF_TO        DINT;
                localRefToINT          : REF_TO     INT;
                localReferenceTo    : REFERENCE TO  DINT;
            END_VAR

            localRefTo          REF= localDINT;
            localReferenceTo    REF= localDINT;
            localRefTo          REF= 0; // This is allowed, assigning to a null pointer
            localRefToINT       REF= 0; // This is allowed, assigning to a null pointer

            // The following are invalid
            1                   REF= localDINT;
            localINT            REF= localDINT;
            localRefTo          REF= 1;
            localReferenceTo    REF= 1;

            localReferenceTo    REF= localINT;
            localReferenceTo    REF= localSTRING;
            localReferenceTo    REF= 'howdy';
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E098]: Invalid assignment, expected a pointer reference
       ┌─ <internal>:19:13
       │
    19 │             1                   REF= localDINT;
       │             ^ Invalid assignment, expected a pointer reference

    error[E098]: Invalid assignment, expected a pointer reference
       ┌─ <internal>:20:13
       │
    20 │             localINT            REF= localDINT;
       │             ^^^^^^^^ Invalid assignment, expected a pointer reference

    error[E037]: Invalid assignment: cannot assign 'DINT' to 'INT'
       ┌─ <internal>:20:13
       │
    20 │             localINT            REF= localDINT;
       │             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Invalid assignment: cannot assign 'DINT' to 'INT'

    error[E098]: Invalid assignment, expected a reference
       ┌─ <internal>:21:38
       │
    21 │             localRefTo          REF= 1;
       │                                      ^ Invalid assignment, expected a reference

    error[E098]: Invalid assignment, expected a reference
       ┌─ <internal>:22:38
       │
    22 │             localReferenceTo    REF= 1;
       │                                      ^ Invalid assignment, expected a reference

    error[E037]: Invalid assignment: cannot assign 'INT' to 'DINT'
       ┌─ <internal>:24:13
       │
    24 │             localReferenceTo    REF= localINT;
       │             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Invalid assignment: cannot assign 'INT' to 'DINT'

    error[E037]: Invalid assignment: cannot assign 'STRING' to 'DINT'
       ┌─ <internal>:25:13
       │
    25 │             localReferenceTo    REF= localSTRING;
       │             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Invalid assignment: cannot assign 'STRING' to 'DINT'

    error[E098]: Invalid assignment, expected a reference
       ┌─ <internal>:26:38
       │
    26 │             localReferenceTo    REF= 'howdy';
       │                                      ^^^^^^^ Invalid assignment, expected a reference
    ");
}

#[test]
fn ref_assignment_with_global_local_variables_and_aliased_types() {
    let diagnostics = parse_and_validate_buffered(
        "
        VAR_GLOBAL
            fooGlobal : DINT;
        END_VAR

        FUNCTION main
            VAR
                fooLocal                        : DINT;
                referenceToFooFirstOfHisName    : REFERENCE TO DINT;
                referenceToFooSecondOfHisName   : REFERENCE TO DINT;
                referenceToAlias                : REFERENCE TO AliasedDINT;

                intLocal            : INT;
                stringLocal         : STRING;

                // Invalid, types should be referenced rather than literals or variables
                invalidA : REFERENCE TO fooLocal;
                invalidB : REFERENCE TO fooGlobal;
                invalidC : REFERENCE TO DINT := 5;
            END_VAR

            referenceToFooFirstOfHisName  REF= fooLocal;
            referenceToFooFirstOfHisName  REF= fooGlobal;
            referenceToFooFirstOfHisName  REF= referenceToFooFirstOfHisName; // Valid, albeit questionable
            referenceToFooFirstOfHisName  REF= referenceToFooSecondOfHisName;

            // Invalid, type mismatch
            referenceToFooFirstOfHisName  REF= intLocal;
            referenceToFooFirstOfHisName  REF= stringLocal;
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E099]: REFERENCE TO variables can not reference other variables
       ┌─ <internal>:17:28
       │
    17 │                 invalidA : REFERENCE TO fooLocal;
       │                            ^^^^^^^^^^^^^^^^^^^^^ REFERENCE TO variables can not reference other variables

    error[E099]: REFERENCE TO variables can not reference other variables
       ┌─ <internal>:18:28
       │
    18 │                 invalidB : REFERENCE TO fooGlobal;
       │                            ^^^^^^^^^^^^^^^^^^^^^^ REFERENCE TO variables can not reference other variables

    error[E052]: Unknown type: AliasedDINT
       ┌─ <internal>:11:64
       │
    11 │                 referenceToAlias                : REFERENCE TO AliasedDINT;
       │                                                                ^^^^^^^^^^^ Unknown type: AliasedDINT

    error[E052]: Unknown type: fooLocal
       ┌─ <internal>:17:41
       │
    17 │                 invalidA : REFERENCE TO fooLocal;
       │                                         ^^^^^^^^ Unknown type: fooLocal

    error[E052]: Unknown type: fooGlobal
       ┌─ <internal>:18:41
       │
    18 │                 invalidB : REFERENCE TO fooGlobal;
       │                                         ^^^^^^^^^ Unknown type: fooGlobal

    error[E037]: Invalid assignment: cannot assign 'INT' to 'DINT'
       ┌─ <internal>:28:13
       │
    28 │             referenceToFooFirstOfHisName  REF= intLocal;
       │             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Invalid assignment: cannot assign 'INT' to 'DINT'

    error[E037]: Invalid assignment: cannot assign 'STRING' to 'DINT'
       ┌─ <internal>:29:13
       │
    29 │             referenceToFooFirstOfHisName  REF= stringLocal;
       │             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Invalid assignment: cannot assign 'STRING' to 'DINT'
    ");
}

#[test]
fn ref_assignment_with_reference_to_array_variable() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main
            VAR
                arrSTRING           :               ARRAY[1..6] OF STRING;
                arrDINT             :               ARRAY[1..5] OF DINT;
                arrReferenceDINT    : REFERENCE TO  ARRAY[1..5] OF DINT;
            END_VAR

            arrReferenceDINT REF= arrDINT;
            arrReferenceDINT REF= arrSTRING;
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E037]: Invalid assignment: cannot assign 'ARRAY[1..6] OF STRING' to 'REFERENCE TO ARRAY[1..5] OF DINT'
       ┌─ <internal>:10:13
       │
    10 │             arrReferenceDINT REF= arrSTRING;
       │             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Invalid assignment: cannot assign 'ARRAY[1..6] OF STRING' to 'REFERENCE TO ARRAY[1..5] OF DINT'
    ");
}

#[test]
fn ref_assignment_with_reference_to_string_variable() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main
            VAR
                localCHAR           : CHAR;
                localSTRING         : STRING;
                localWSTRING        : WSTRING;
                referenceToString   : REFERENCE TO STRING;
            END_VAR

            referenceToString REF= localCHAR;
            referenceToString REF= localSTRING;
            referenceToString REF= localWSTRING;
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E037]: Invalid assignment: cannot assign 'CHAR' to 'STRING'
       ┌─ <internal>:10:13
       │
    10 │             referenceToString REF= localCHAR;
       │             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Invalid assignment: cannot assign 'CHAR' to 'STRING'

    error[E037]: Invalid assignment: cannot assign 'WSTRING' to 'STRING'
       ┌─ <internal>:12:13
       │
    12 │             referenceToString REF= localWSTRING;
       │             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Invalid assignment: cannot assign 'WSTRING' to 'STRING'
    ");
}

// TODO(volsa): Improve the error messages here; these are the default messages returned by the parser
//              without any modifications.
#[test]
fn invalid_reference_to_declaration() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION foo
            VAR
                bar : ARRAY[1..5] OF REFERENCE TO DINT;
                baz : REFERENCE TO REFERENCE TO DINT;
                qux : REF_TO REFERENCE TO DINT;
            END_VAR
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E007]: Unexpected token: expected DataTypeDefinition but found KeywordReferenceTo
      ┌─ <internal>:4:38
      │
    4 │                 bar : ARRAY[1..5] OF REFERENCE TO DINT;
      │                                      ^^^^^^^^^^^^ Unexpected token: expected DataTypeDefinition but found KeywordReferenceTo

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'REFERENCE TO DINT'
      ┌─ <internal>:4:38
      │
    4 │                 bar : ARRAY[1..5] OF REFERENCE TO DINT;
      │                                      ^^^^^^^^^^^^^^^^^ Unexpected token: expected KeywordSemicolon but found 'REFERENCE TO DINT'

    error[E007]: Unexpected token: expected DataTypeDefinition but found KeywordReferenceTo
      ┌─ <internal>:5:36
      │
    5 │                 baz : REFERENCE TO REFERENCE TO DINT;
      │                                    ^^^^^^^^^^^^ Unexpected token: expected DataTypeDefinition but found KeywordReferenceTo

    error[E007]: Unexpected token: expected KeywordEndVar but found 'REFERENCE TO DINT;
                    qux : REF_TO REFERENCE TO DINT;'
      ┌─ <internal>:5:36
      │  
    5 │                   baz : REFERENCE TO REFERENCE TO DINT;
      │ ╭────────────────────────────────────^
    6 │ │                 qux : REF_TO REFERENCE TO DINT;
      │ ╰───────────────────────────────────────────────^ Unexpected token: expected KeywordEndVar but found 'REFERENCE TO DINT;
                    qux : REF_TO REFERENCE TO DINT;'
    ");
}

#[test]
fn alias_variable_type_check() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION foo
            VAR
                dintVar                         : DINT;
                sintVar                         : SINT;
                stringVar                       : STRING;
                arrayDintVar                    : ARRAY[1..5] OF DINT;

                dintVarRefA AT dintVar          : DINT; // Valid
                dintVarRefB AT sintVar          : DINT; // Invalid
                dintVarRefC AT stringVar        : DINT; // Invalid
                dintVarRefD AT arrayDintVar     : DINT; // Invalid

                sintVarRefA AT dintVar          : SINT; // Invalid
                sintVarRefB AT sintVar          : SINT; // Valid
                sintVarRefC AT stringVar        : SINT; // Invalid
                sintVarRefD AT arrayDintVar     : SINT; // Invalid

                stringVarRefA AT dintVar        : STRING; // Invalid
                stringVarRefB AT sintVar        : STRING; // Invalid
                stringVarRefC AT stringVar      : STRING; // Valid
                stringVarRefD AT arrayDintVar   : STRING; // Invalid

                arrayDintVarRefA AT dintVar        : ARRAY[1..5] OF DINT; // Invalid
                arrayDintVarRefB AT sintVar        : ARRAY[1..5] OF DINT; // Invalid
                arrayDintVarRefC AT stringVar      : ARRAY[1..5] OF DINT; // Invalid
                arrayDintVarRefD AT arrayDintVar   : ARRAY[1..5] OF DINT; // Valid
            END_VAR
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E037]: Invalid assignment: cannot assign 'SINT' to 'DINT'
       ┌─ <internal>:10:32
       │
    10 │                 dintVarRefB AT sintVar          : DINT; // Invalid
       │                                ^^^^^^^ Invalid assignment: cannot assign 'SINT' to 'DINT'

    error[E037]: Invalid assignment: cannot assign 'STRING' to 'DINT'
       ┌─ <internal>:11:32
       │
    11 │                 dintVarRefC AT stringVar        : DINT; // Invalid
       │                                ^^^^^^^^^ Invalid assignment: cannot assign 'STRING' to 'DINT'

    error[E037]: Invalid assignment: cannot assign 'ARRAY[1..5] OF DINT' to 'DINT'
       ┌─ <internal>:12:32
       │
    12 │                 dintVarRefD AT arrayDintVar     : DINT; // Invalid
       │                                ^^^^^^^^^^^^ Invalid assignment: cannot assign 'ARRAY[1..5] OF DINT' to 'DINT'

    error[E037]: Invalid assignment: cannot assign 'DINT' to 'SINT'
       ┌─ <internal>:14:32
       │
    14 │                 sintVarRefA AT dintVar          : SINT; // Invalid
       │                                ^^^^^^^ Invalid assignment: cannot assign 'DINT' to 'SINT'

    error[E037]: Invalid assignment: cannot assign 'STRING' to 'SINT'
       ┌─ <internal>:16:32
       │
    16 │                 sintVarRefC AT stringVar        : SINT; // Invalid
       │                                ^^^^^^^^^ Invalid assignment: cannot assign 'STRING' to 'SINT'

    error[E037]: Invalid assignment: cannot assign 'ARRAY[1..5] OF DINT' to 'SINT'
       ┌─ <internal>:17:32
       │
    17 │                 sintVarRefD AT arrayDintVar     : SINT; // Invalid
       │                                ^^^^^^^^^^^^ Invalid assignment: cannot assign 'ARRAY[1..5] OF DINT' to 'SINT'

    error[E037]: Invalid assignment: cannot assign 'DINT' to 'STRING'
       ┌─ <internal>:19:34
       │
    19 │                 stringVarRefA AT dintVar        : STRING; // Invalid
       │                                  ^^^^^^^ Invalid assignment: cannot assign 'DINT' to 'STRING'

    error[E037]: Invalid assignment: cannot assign 'SINT' to 'STRING'
       ┌─ <internal>:20:34
       │
    20 │                 stringVarRefB AT sintVar        : STRING; // Invalid
       │                                  ^^^^^^^ Invalid assignment: cannot assign 'SINT' to 'STRING'

    error[E037]: Invalid assignment: cannot assign 'ARRAY[1..5] OF DINT' to 'STRING'
       ┌─ <internal>:22:34
       │
    22 │                 stringVarRefD AT arrayDintVar   : STRING; // Invalid
       │                                  ^^^^^^^^^^^^ Invalid assignment: cannot assign 'ARRAY[1..5] OF DINT' to 'STRING'

    error[E037]: Invalid assignment: cannot assign 'DINT' to 'ARRAY[1..5] OF DINT'
       ┌─ <internal>:24:37
       │
    24 │                 arrayDintVarRefA AT dintVar        : ARRAY[1..5] OF DINT; // Invalid
       │                                     ^^^^^^^ Invalid assignment: cannot assign 'DINT' to 'ARRAY[1..5] OF DINT'

    error[E037]: Invalid assignment: cannot assign 'SINT' to 'ARRAY[1..5] OF DINT'
       ┌─ <internal>:25:37
       │
    25 │                 arrayDintVarRefB AT sintVar        : ARRAY[1..5] OF DINT; // Invalid
       │                                     ^^^^^^^ Invalid assignment: cannot assign 'SINT' to 'ARRAY[1..5] OF DINT'

    error[E037]: Invalid assignment: cannot assign 'STRING' to 'ARRAY[1..5] OF DINT'
       ┌─ <internal>:26:37
       │
    26 │                 arrayDintVarRefC AT stringVar      : ARRAY[1..5] OF DINT; // Invalid
       │                                     ^^^^^^^^^ Invalid assignment: cannot assign 'STRING' to 'ARRAY[1..5] OF DINT'
    ");
}

#[test]
fn reassignment_of_alias_variables_is_disallowed() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION main
            VAR
                foo AT bar : DINT;
                bar : DINT;
                baz : DINT;
            END_VAR

            foo := bar;         // Valid, the dereferenced value of `foo` is being changed
            foo := baz;         // Valid, same reason as above
            foo REF= bar;       // Invalid, the address of `foo` is being changed
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E100]: foo is an immutable alias variable, can not change the address
       ┌─ <internal>:11:13
       │
    11 │             foo REF= bar;       // Invalid, the address of `foo` is being changed
       │             ^^^^^^^^^^^^ foo is an immutable alias variable, can not change the address
    ");
}

#[test]
fn assigning_arrays_with_same_size_and_type_class_but_different_inner_type_is_an_error() {
    // When assigning arrays of matching size and type-class,
    // if they have a different intrinsic inner type,
    // then we expect an assignment error.
    let diagnostic = parse_and_validate_buffered(
        r"
        FUNCTION foo: DINT
        VAR_INPUT
            in1: ARRAY[0..1] OF LINT;
            in3: STRING[100];
        END_VAR
        END_FUNCTION

        FUNCTION main: DINT
        VAR
            var1: ARRAY[0..1] OF ULINT; // Err
            var2: ARRAY[0..1] OF LTIME; // Err
            var3: ARRAY[0..1] OF LWORD; // Err
            var4: STRING;               // OK
        END_VAR
            foo(var1, var4);
            foo(var2, var4);
            foo(var3, var4);
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostic, @r"
    error[E037]: Invalid assignment: cannot assign 'ARRAY[0..1] OF ULINT' to 'ARRAY[0..1] OF LINT'
       ┌─ <internal>:16:17
       │
    16 │             foo(var1, var4);
       │                 ^^^^ Invalid assignment: cannot assign 'ARRAY[0..1] OF ULINT' to 'ARRAY[0..1] OF LINT'

    error[E037]: Invalid assignment: cannot assign 'ARRAY[0..1] OF LTIME' to 'ARRAY[0..1] OF LINT'
       ┌─ <internal>:17:17
       │
    17 │             foo(var2, var4);
       │                 ^^^^ Invalid assignment: cannot assign 'ARRAY[0..1] OF LTIME' to 'ARRAY[0..1] OF LINT'

    error[E037]: Invalid assignment: cannot assign 'ARRAY[0..1] OF LWORD' to 'ARRAY[0..1] OF LINT'
       ┌─ <internal>:18:17
       │
    18 │             foo(var3, var4);
       │                 ^^^^ Invalid assignment: cannot assign 'ARRAY[0..1] OF LWORD' to 'ARRAY[0..1] OF LINT'
    ");
}
