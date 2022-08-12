use crate::ast::SourceRange;
use crate::test_utils::tests::parse_and_validate;
use crate::Diagnostic;

#[test]
fn assign_pointer_to_too_small_type_result_in_an_error() {
    //GIVEN assignment statements to DWORD
    //WHEN it is validated
    let diagnostics: Vec<Diagnostic> = parse_and_validate(
        "
        PROGRAM FOO
            VAR
                ptr : POINTER TO INT;
                address : DWORD;
            END_VAR
            
            address := 16#DEAD_BEEF;              
            address := ptr;         //should throw error as address is too small to store full pointer
        END_PROGRAM
        ",
    );

    //THEN assignment with different type sizes are reported
    assert_eq!(
        diagnostics,
        vec![Diagnostic::incompatible_type_size(
            "DWORD",
            32,
            "hold a",
            SourceRange::new(204..218,Some(8),Some(13),Some(8),Some(27))
        ),]
    );
}

#[test]
fn assign_too_small_type_to_pointer_result_in_an_error() {
    //GIVEN assignment statements to pointer
    //WHEN it is validated
    let diagnostics: Vec<Diagnostic> = parse_and_validate(
        "
        PROGRAM FOO
            VAR
                ptr : POINTER TO INT;
                address : DWORD;
            END_VAR
            
            address := 16#DEAD_BEEF;              
            ptr := address;         //should throw error as address is too small to store full pointer
        END_PROGRAM
        ",
    );

    //THEN assignment with different type sizes are reported
    assert_eq!(
        diagnostics,
        vec![Diagnostic::incompatible_type_size(
            "DWORD",
            32,
            "to be stored in a",
            SourceRange::new(204..218,Some(8),Some(13),Some(8),Some(27))
        ),]
    );
}

#[test]
fn assign_pointer_to_lword() {
    //GIVEN assignment statements to lword
    //WHEN it is validated
    let diagnostics: Vec<Diagnostic> = parse_and_validate(
        "
        PROGRAM FOO
            VAR
                ptr : POINTER TO INT;
                address : LWORD;
            END_VAR
            
            address := 16#DEAD_BEEF;              
            address := ptr;         //should throw error as address is too small to store full pointer
        END_PROGRAM
        ",
    );

    //THEN every assignment is valid
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn assignment_to_constants_result_in_an_error() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        "
        VAR_GLOBAL CONSTANT
            ci: INT := 1;
        END_VAR

        VAR_GLOBAL
            i : INT;
        END_VAR

        PROGRAM prg
            VAR CONSTANT
                cl : INT := 1;
            END_VAR

            VAR
                l : INT := 1;
            END_VAR

            l   := 7;
            cl  := 4;
            i   := 1;
            ci  := 4;
        END_PROGRAM
      ",
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::cannot_assign_to_constant("prg.cl", SourceRange::new(327..329,Some(19),Some(13),Some(19),Some(15))),
            Diagnostic::cannot_assign_to_constant("ci", SourceRange::new(371..373,Some(21),Some(13),Some(21),Some(15))),
        ]
    );
}

#[test]
fn assignment_to_enum_literals_results_in_error() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        "
        TYPE Color: (red, yellow, green); END_TYPE

        VAR_GLOBAL 
            g_enum: (A, B, C);
        END_VAR

        PROGRAM prg
            VAR 
                state: (OPEN, CLOSED);
            END_VAR

            OPEN := 3;
            B := A;
            red := green;
       END_PROGRAM
      ",
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::cannot_assign_to_constant("__prg_state.OPEN", SourceRange::new(230..234,Some(12),Some(13),Some(12),Some(17))),
            Diagnostic::cannot_assign_to_constant("__global_g_enum.B", SourceRange::new(253..254,Some(13),Some(13),Some(13),Some(14))),
            Diagnostic::cannot_assign_to_constant("Color.red", SourceRange::new(273..276,Some(14),Some(13),Some(14),Some(16))),
        ]
    );
}

#[test]
fn invalid_char_assignments() {
    // GIVEN invalid assignments to CHAR/WCHAR
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        r#"
		PROGRAM mainProg
		VAR
			c : CHAR;
			c2 : CHAR;
			wc : WCHAR;
			wc2 : WCHAR;
			i : INT;
			s : STRING;
		END_VAR
			c := 'AJK%&/231'; // invalid
			wc := "898JKAN"; // invalid

			c := wc; // invalid
			wc := c; // invalid

			i := 54;
			c := i; // invalid
			c := 42; // invalid

			s := 'ABC';
			c := s; // invalid
			wc := s; // invalid

			i := c; // invalid
			s := c; // invalid

			c := 'A';
			c2 := 'B';
			c := c2;

			wc := "A";
			wc2 := "B";
			wc := wc2;
		END_PROGRAM"#,
    );

    // THEN every assignment should be reported
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::syntax_error(
                "Value: 'AJK%&/231' exceeds length for type: CHAR",
                SourceRange::new(129..140, Some(10),Some(18),Some(10),Some(29))
            ),
            Diagnostic::syntax_error(
                "Value: '898JKAN' exceeds length for type: WCHAR",
                SourceRange::new(162..171,Some(10),Some(19),Some(10),Some(28))
            ),
            Diagnostic::invalid_assignment("WCHAR", "CHAR", SourceRange::new(188..195,Some(13),Some(13),Some(13),Some(20))),
            Diagnostic::invalid_assignment("CHAR", "WCHAR", SourceRange::new(211..218,Some(14),Some(13),Some(14),Some(20))),
            Diagnostic::invalid_assignment("INT", "CHAR", SourceRange::new(247..253,Some(17),Some(13),Some(17),Some(20))),
            Diagnostic::invalid_assignment("DINT", "CHAR", SourceRange::new(269..276,Some(18),Some(13),Some(18),Some(20))),
            Diagnostic::invalid_assignment("STRING", "CHAR", SourceRange::new(308..314,Some(21),Some(13),Some(21),Some(20))),
            Diagnostic::invalid_assignment("STRING", "WCHAR", SourceRange::new(330..337,Some(22),Some(13),Some(22),Some(20))),
            Diagnostic::invalid_assignment("CHAR", "INT", SourceRange::new(354..360,Some(24),Some(13),Some(24),Some(20))),
            Diagnostic::invalid_assignment("CHAR", "STRING", SourceRange::new(376..382,Some(25),Some(13),Some(25),Some(20))),
        ]
    );
}

#[test]
fn missing_string_compare_function_causes_error() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        "
        PROGRAM prg
            'a' =  'b'; // missing compare function :-(
            'a' <> 'b'; // missing compare function :-(
            'a' <  'b'; // missing compare function :-(
            'a' >  'b'; // missing compare function :-(
            'a' <= 'b'; // missing compare function :-(
            'a' >= 'b'; // missing compare function :-(
        END_PROGRAM
      ",
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_compare_function("STRING_EQUAL", "STRING", SourceRange::new(33..43,Some(2),Some(13),Some(2),Some(23))),
            Diagnostic::missing_compare_function("STRING_EQUAL", "STRING", SourceRange::new(89..99,Some(3),Some(13),Some(3),Some(23))),
            Diagnostic::missing_compare_function("STRING_LESS", "STRING", SourceRange::new(145..155,Some(4),Some(13),Some(4),Some(23))),
            Diagnostic::missing_compare_function("STRING_GREATER", "STRING", SourceRange::new(201..211,Some(5),Some(13),Some(5),Some(23))),
            Diagnostic::missing_compare_function("STRING_LESS", "STRING", SourceRange::new(257..267,Some(6),Some(13),Some(6),Some(23))),
            Diagnostic::missing_compare_function("STRING_EQUAL", "STRING", SourceRange::new(257..267,Some(6),Some(13),Some(6),Some(23))),
            Diagnostic::missing_compare_function("STRING_GREATER", "STRING", SourceRange::new(313..323,Some(7),Some(13),Some(7),Some(23))),
            Diagnostic::missing_compare_function("STRING_EQUAL", "STRING", SourceRange::new(313..323,Some(7),Some(13),Some(7),Some(23))),
        ]
    );
}

#[test]
fn string_compare_function_cause_no_error_if_functions_exist() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        "
        FUNCTION STRING_EQUAL : BOOL VAR_INPUT a,b : STRING END_VAR END_FUNCTION
        FUNCTION STRING_GREATER : BOOL VAR_INPUT a,b : STRING END_VAR END_FUNCTION
        FUNCTION STRING_LESS : BOOL VAR_INPUT a,b : STRING END_VAR END_FUNCTION

        PROGRAM prg
            'a' =  'b'; // missing compare function :-(
            'a' <> 'b'; // missing compare function :-(
            'a' <  'b'; // missing compare function :-(
            'a' >  'b'; // missing compare function :-(
            'a' <= 'b'; // missing compare function :-(
            'a' >= 'b'; // missing compare function :-(
        END_PROGRAM
      ",
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn string_compare_function_with_wrong_signature_causes_error() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        "
        FUNCTION STRING_EQUAL : BOOL VAR_INPUT a : STRING END_VAR END_FUNCTION

        PROGRAM prg
            'a' =  'b'; // missing compare function :-(
        END_PROGRAM
      ",
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert_eq!(
        diagnostics,
        vec![Diagnostic::missing_compare_function(
            "STRING_EQUAL",
            "STRING",
            SourceRange::new(113..123,Some(4),Some(13),Some(4),Some(23))
        ),]
    );
}

#[test]
fn missing_wstring_compare_function_causes_error() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        r#"
        PROGRAM prg
            "a" =  "b"; // missing compare function :-(
            "a" <> "b"; // missing compare function :-(
            "a" <  "b"; // missing compare function :-(
            "a" >  "b"; // missing compare function :-(
            "a" <= "b"; // missing compare function :-(
            "a" >= "b"; // missing compare function :-(
        END_PROGRAM
      "#,
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_compare_function("WSTRING_EQUAL", "WSTRING", SourceRange::new(33..43,Some(2),Some(13),Some(2),Some(23))),
            Diagnostic::missing_compare_function("WSTRING_EQUAL", "WSTRING", SourceRange::new(89..99,Some(3),Some(13),Some(3),Some(23))),
            Diagnostic::missing_compare_function("WSTRING_LESS", "WSTRING", SourceRange::new(145..155,Some(4),Some(13),Some(4),Some(23))),
            Diagnostic::missing_compare_function("WSTRING_GREATER", "WSTRING", SourceRange::new(201..211,Some(5),Some(13),Some(5),Some(23))),
            Diagnostic::missing_compare_function("WSTRING_LESS", "WSTRING", SourceRange::new(257..267,Some(6),Some(13),Some(6),Some(23))),
            Diagnostic::missing_compare_function("WSTRING_EQUAL", "WSTRING", SourceRange::new(257..267,Some(7),Some(13),Some(7),Some(23))),
            Diagnostic::missing_compare_function("WSTRING_GREATER", "WSTRING", SourceRange::new(313..323,Some(8),Some(13),Some(8),Some(23))),
            Diagnostic::missing_compare_function("WSTRING_EQUAL", "WSTRING", SourceRange::new(313..323,Some(8),Some(13),Some(8),Some(23))),
        ]
    );
}

#[test]
fn wstring_compare_function_cause_no_error_if_functions_exist() {
    // GIVEN assignment statements to constants, some to writable variables
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        r#"
        FUNCTION WSTRING_EQUAL : BOOL VAR_INPUT a,b : WSTRING END_VAR END_FUNCTION
        FUNCTION WSTRING_GREATER : BOOL VAR_INPUT a,b : WSTRING END_VAR END_FUNCTION
        FUNCTION WSTRING_LESS : BOOL VAR_INPUT a,b : WSTRING END_VAR END_FUNCTION

        PROGRAM prg
            "a" =  "b"; // missing compare function :-(
            "a" <> "b"; // missing compare function :-(
            "a" <  "b"; // missing compare function :-(
            "a" >  "b"; // missing compare function :-(
            "a" <= "b"; // missing compare function :-(
            "a" >= "b"; // missing compare function :-(
        END_PROGRAM
      "#,
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert_eq!(diagnostics, vec![]);
}
