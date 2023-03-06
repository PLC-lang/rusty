use crate::test_utils::tests::parse_and_validate;
use crate::Diagnostic;

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

    assert_eq!(
        diagnostics,
        vec![
            // CONSTANT
            Diagnostic::cannot_assign_to_constant("v_global", (1246..1254).into()),
            // REAL
            Diagnostic::invalid_assignment("STRING", "REAL", (1627..1645).into()),
            Diagnostic::invalid_assignment("STRING", "REAL", (1662..1687).into()),
            Diagnostic::invalid_assignment("STRING", "REAL", (1704..1722).into()),
            Diagnostic::invalid_assignment("CHAR", "REAL", (1739..1755).into()),
            Diagnostic::invalid_assignment("CHAR", "REAL", (1772..1790).into()),
            Diagnostic::invalid_assignment("STRING", "REAL", (1909..1931).into()),
            Diagnostic::invalid_assignment("STRING", "REAL", (1988..2014).into()),
            // UDINT
            Diagnostic::invalid_assignment("STRING", "UDINT", (2394..2413).into()),
            Diagnostic::invalid_assignment("STRING", "UDINT", (2430..2456).into()),
            Diagnostic::invalid_assignment("STRING", "UDINT", (2473..2492).into()),
            Diagnostic::invalid_assignment("CHAR", "UDINT", (2509..2526).into()),
            Diagnostic::invalid_assignment("CHAR", "UDINT", (2543..2562).into()),
            Diagnostic::invalid_assignment("STRING", "UDINT", (2684..2707).into()),
            Diagnostic::invalid_assignment("STRING", "UDINT", (2765..2792).into()),
            // DINT
            Diagnostic::invalid_assignment("STRING", "DINT", (3160..3178).into()),
            Diagnostic::invalid_assignment("STRING", "DINT", (3195..3220).into()),
            Diagnostic::invalid_assignment("STRING", "DINT", (3237..3255).into()),
            Diagnostic::invalid_assignment("CHAR", "DINT", (3272..3288).into()),
            Diagnostic::invalid_assignment("CHAR", "DINT", (3305..3323).into()),
            Diagnostic::invalid_assignment("STRING", "DINT", (3442..3464).into()),
            Diagnostic::invalid_assignment("STRING", "DINT", (3521..3547).into()),
            // TIME
            Diagnostic::invalid_assignment("STRING", "TIME", (3915..3933).into()),
            Diagnostic::invalid_assignment("STRING", "TIME", (3950..3975).into()),
            Diagnostic::invalid_assignment("STRING", "TIME", (3992..4010).into()),
            Diagnostic::invalid_assignment("CHAR", "TIME", (4027..4043).into()),
            Diagnostic::invalid_assignment("CHAR", "TIME", (4060..04078).into()),
            Diagnostic::invalid_assignment("STRING", "TIME", (4197..4219).into()),
            Diagnostic::invalid_assignment("STRING", "TIME", (4276..4302).into()),
            // BYTE
            Diagnostic::invalid_assignment("STRING", "BYTE", (4667..4685).into()),
            Diagnostic::invalid_assignment("STRING", "BYTE", (4702..4727).into()),
            Diagnostic::invalid_assignment("STRING", "BYTE", (4744..4762).into()),
            Diagnostic::invalid_assignment("CHAR", "BYTE", (4779..4795).into()),
            Diagnostic::invalid_assignment("CHAR", "BYTE", (4812..04830).into()),
            Diagnostic::invalid_assignment("STRING", "BYTE", (4949..4971).into()),
            Diagnostic::invalid_assignment("STRING", "BYTE", (5028..5054).into()),
            // STRING
            Diagnostic::invalid_assignment("LREAL", "STRING", (5091..5110).into()),
            Diagnostic::invalid_assignment("REAL", "STRING", (5127..5147).into()),
            Diagnostic::invalid_assignment("UDINT", "STRING", (5164..5183).into()),
            Diagnostic::invalid_assignment("UDINT", "STRING", (5200..5220).into()),
            Diagnostic::invalid_assignment("DINT", "STRING", (5237..5255).into()),
            Diagnostic::invalid_assignment("DINT", "STRING", (5272..5291).into()),
            Diagnostic::invalid_assignment("TIME", "STRING", (5308..5326).into()),
            Diagnostic::invalid_assignment("TIME", "STRING", (5343..5369).into()),
            Diagnostic::invalid_assignment("WORD", "STRING", (5386..5404).into()),
            Diagnostic::invalid_assignment("WORD", "STRING", (5421..5445).into()),
            Diagnostic::invalid_assignment("WSTRING", "STRING", (5571..5592).into()),
            Diagnostic::invalid_assignment("WSTRING", "STRING", (5609..5638).into()),
            Diagnostic::invalid_assignment("WSTRING", "STRING", (5655..5676).into()),
            Diagnostic::invalid_assignment("CHAR", "STRING", (5693..5711).into()),
            Diagnostic::invalid_assignment("CHAR", "STRING", (5728..5748).into()),
            Diagnostic::invalid_assignment("TIME_OF_DAY", "STRING", (5765..5782).into()),
            Diagnostic::invalid_assignment("TIME_OF_DAY", "STRING", (5799..5823).into()),
            Diagnostic::invalid_assignment("INT", "STRING", (5840..5861).into()),
            Diagnostic::invalid_assignment("INT", "STRING", (5919..5944).into()),
            // CHAR
            Diagnostic::invalid_assignment("LREAL", "CHAR", (6023..6040).into()),
            Diagnostic::invalid_assignment("REAL", "CHAR", (6057..6075).into()),
            Diagnostic::invalid_assignment("UDINT", "CHAR", (6092..6109).into()),
            Diagnostic::invalid_assignment("UDINT", "CHAR", (6126..6144).into()),
            Diagnostic::invalid_assignment("DINT", "CHAR", (6161..6177).into()),
            Diagnostic::invalid_assignment("DINT", "CHAR", (6194..6211).into()),
            Diagnostic::invalid_assignment("TIME", "CHAR", (6228..6244).into()),
            Diagnostic::invalid_assignment("TIME", "CHAR", (6261..6285).into()),
            Diagnostic::invalid_assignment("WORD", "CHAR", (6302..6318).into()),
            Diagnostic::invalid_assignment("WORD", "CHAR", (6335..6357).into()),
            Diagnostic::invalid_assignment("STRING", "CHAR", (6374..6393).into()),
            Diagnostic::invalid_assignment("STRING", "CHAR", (6425..6445).into()),
            Diagnostic::invalid_assignment("STRING", "CHAR", (6505..6523).into()),
            Diagnostic::invalid_assignment("STRING", "CHAR", (6540..6565).into()),
            Diagnostic::syntax_error("Value: 'string' exceeds length for type: CHAR", (6582..6600).into()),
            Diagnostic::invalid_assignment("STRING", "CHAR", (6582..6600).into()),
            Diagnostic::invalid_assignment("WCHAR", "CHAR", (6681..6698).into()),
            Diagnostic::invalid_assignment("WCHAR", "CHAR", (6715..6734).into()),
            Diagnostic::invalid_assignment("TIME_OF_DAY", "CHAR", (6751..6766).into()),
            Diagnostic::invalid_assignment("TIME_OF_DAY", "CHAR", (6783..6805).into()),
            Diagnostic::invalid_assignment("INT", "CHAR", (6822..6841).into()),
            Diagnostic::invalid_assignment("STRING", "CHAR", (6859..6881).into()),
            Diagnostic::invalid_assignment("INT", "CHAR", (6899..6922).into()),
            Diagnostic::invalid_assignment("STRING", "CHAR", (6940..6966).into()),
            // DATE
            Diagnostic::invalid_assignment("STRING", "DATE", (7332..7350).into()),
            Diagnostic::invalid_assignment("STRING", "DATE", (7367..7392).into()),
            Diagnostic::invalid_assignment("STRING", "DATE", (7409..7427).into()),
            Diagnostic::invalid_assignment("CHAR", "DATE", (7444..7460).into()),
            Diagnostic::invalid_assignment("CHAR", "DATE", (7477..7495).into()),
            Diagnostic::invalid_assignment("STRING", "DATE", (7614..7636).into()),
            Diagnostic::invalid_assignment("STRING", "DATE", (7693..7719).into()),
            // POINTER
            Diagnostic::incompatible_type_size("DINT", 32, "hold a", (7757..7776).into()),
            Diagnostic::invalid_assignment("__main_v_ptr_int", "DINT", (7757..7776).into()),
            // __POINTER_TO_REAL should work
            Diagnostic::invalid_assignment("__POINTER_TO_REAL", "__main_v_ptr_int", (7793..7813).into()),
            Diagnostic::invalid_assignment("__POINTER_TO_STRING", "__main_v_ptr_int", (8172..8194).into()),
            Diagnostic::invalid_assignment("STRING", "INT", (8211..8233).into()),
            // missing __POINTER_TO_CHAR validation
            Diagnostic::invalid_assignment("CHAR", "INT", (8315..8335).into()),
            Diagnostic::invalid_assignment("STRING", "INT", (8465..8495).into()),
            // ARRAY
            Diagnostic::invalid_assignment("__main_v_arr_int_2", "__main_v_arr_int_3", (8531..8557).into()),
            Diagnostic::invalid_assignment("__main_v_arr_int_4", "__main_v_arr_int_3", (8615..8641).into()),
            Diagnostic::invalid_assignment("__main_v_arr_real_3", "__main_v_arr_int_3", (8658..8685).into()),
            Diagnostic::invalid_assignment(
                "__main_v_arr_string_3",
                "__main_v_arr_int_3",
                (8702..8731).into()
            ),
            Diagnostic::invalid_assignment("__main_v_arr_char_3", "__main_v_arr_int_3", (8748..8775).into()),
            Diagnostic::array_expected_initializer_list((8792..8803).into()),
            Diagnostic::array_expected_identifier_or_round_bracket((8810..8811).into()),
            Diagnostic::array_expected_identifier_or_round_bracket((8813..8814).into()),
            Diagnostic::array_expected_identifier_or_round_bracket((8816..8817).into()),
            Diagnostic::invalid_assignment("STRING", "INT", (9096..9122).into()),
            Diagnostic::invalid_assignment("STRING", "INT", (9139..9172).into()),
            Diagnostic::invalid_assignment("STRING", "INT", (9189..9215).into()),
            Diagnostic::invalid_assignment("CHAR", "INT", (9232..9256).into()),
            Diagnostic::invalid_assignment("CHAR", "INT", (9273..9299).into()),
            Diagnostic::invalid_assignment("STRING", "INT", (9359..9389).into()),
            Diagnostic::invalid_assignment("DINT", "__main_v_arr_int_3", (9407..9428).into()),
            Diagnostic::invalid_assignment("__main_v_arr_int_3", "DINT", (9445..9466).into()),
            // STRUCT POINTER
            Diagnostic::invalid_assignment(
                "__POINTER_TO_REAL",
                "__main_v_ref_to_struct",
                (9785..9815).into()
            ),
            Diagnostic::invalid_assignment(
                "__POINTER_TO_STRING",
                "__main_v_ref_to_struct",
                (9832..9861).into()
            ),
            Diagnostic::invalid_assignment(
                "__POINTER_TO_CHAR",
                "__main_v_ref_to_struct",
                (9878..9908).into()
            ),
            Diagnostic::invalid_assignment(
                "__POINTER_TO_REAL",
                "__main_v_ref_to_struct",
                (9930..9957).into()
            ),
            Diagnostic::invalid_assignment(
                "__POINTER_TO_STRING",
                "__main_v_ref_to_struct",
                (9975..10001).into()
            ),
            Diagnostic::invalid_assignment(
                "__POINTER_TO_CHAR",
                "__main_v_ref_to_struct",
                (10019..10046).into()
            ),
            Diagnostic::incompatible_type_size("WORD", 16, "hold a", (10069..10094).into()),
            Diagnostic::invalid_assignment("__main_v_ref_to_struct", "WORD", (10069..10094).into()),
        ]
    );
}
