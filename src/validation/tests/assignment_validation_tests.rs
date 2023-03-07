use insta::assert_snapshot;

use crate::test_utils::tests::parse_and_validate;
use crate::validation::tests::make_readable;

#[test]
fn assignment_validation() {
    let diagnostics = parse_and_validate(
        r#"
    VAR_GLOBAL CONSTANT
        v_global : BOOL;
    END_VAR
    
    TYPE STRUCT_params :
        STRUCT
            param1 : BOOL;
            param2 : BOOL;
            param3 : BOOL;
        END_STRUCT
    END_TYPE
    
    FUNCTION main : DINT
    VAR
        v_real : REAL;
        v_lreal : LREAL;
    
        v_udint : UDINT;
        v_ulint : ULINT;
    
        v_dint : DINT;
        v_lint : LINT;
    
        v_time : TIME;
        v_ltime : LTIME;
    
        v_byte : BYTE;
        v_word : WORD;
        v_lword : LWORD;
    
        v_string : STRING;
        v_str : STRING;
        v_string1 : STRING[1];
        v_wstring : WSTRING;
    
        v_char : CHAR;
        v_wchar : WCHAR;
    
        v_date : DATE;
        v_tod : TOD;
    
        v_ptr_int : REF_TO INT;
        v_ptr_string : REF_TO STRING;
    
        v_arr_int_2 : ARRAY[0..2] OF INT;
        v_arr_int_3 : ARRAY[0..3] OF INT;
        v_arr_int_4 : ARRAY[0..4] OF INT;
    
        v_arr_real_3 : ARRAY[0..3] OF REAL;
    
        v_arr_string_3 : ARRAY[0..3] OF STRING;
    
        v_arr_char_3 : ARRAY[0..3] OF CHAR;
    
        v_struct : STRUCT_params;
        v_ref_to_struct : REF_TO STRUCT_params;
    END_VAR
    // CONSTANT assignment
    v_global := TRUE; // INVALID
    
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
    
    // POINTER
    v_dint := v_ptr_int; // INVALID
    v_ptr_int := &v_real; // INVALID -> TODO: should be valid
    v_ptr_int^ := v_real; // valid
    v_ptr_int := &v_udint; // valid
    v_ptr_int^ := v_udint; // valid
    v_ptr_int := &v_dint; // valid
    v_ptr_int^ := v_dint; // valid
    v_ptr_int := &v_time; // valid
    v_ptr_int^ := v_time; // valid
    v_ptr_int := &v_word; // valid
    v_ptr_int^ := v_word; // valid
    v_ptr_int := &v_string; // INVALID
    v_ptr_int^ := v_string; // INVALID
    v_ptr_int := &v_char; // INVALID -> TODO: missing validation
    v_ptr_int^ := v_char; // INVALID
    v_ptr_int := &v_date; // valid
    v_ptr_int^ := v_date; // valid
    v_ptr_int^ := v_arr_int_3[0]; // valid
    v_ptr_int^ := v_arr_string_3[0]; // INVALID
    
    // ARRAY
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
    
    // STRUCT
    v_ref_to_struct := REF(v_struct); // valid
    v_ref_to_struct := ADR(v_struct); // valid
    v_ref_to_struct := &(v_struct); // valid
    
    v_ref_to_struct := ADR(v_real); // valid
    v_ref_to_struct := ADR(v_str); // valid
    v_ref_to_struct := ADR(v_char); // valid
    
    v_ref_to_struct := REF(v_real); // INVALID
    v_ref_to_struct := REF(v_str); // INVALID
    v_ref_to_struct := REF(v_char); // INVALID
    
    v_ref_to_struct := &(v_real); // INVALID
    v_ref_to_struct := &(v_str); // INVALID
    v_ref_to_struct := &(v_char); // INVALID
    
    v_word := v_ref_to_struct; // INVALID
    v_lword := v_ref_to_struct; // valid
    END_FUNCTION
    "#,
    );

    assert_snapshot!(make_readable(&diagnostics));
}
