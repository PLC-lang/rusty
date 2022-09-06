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
            (204..218).into()
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
            (204..218).into()
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
            Diagnostic::cannot_assign_to_constant("prg.cl", (327..329).into()),
            Diagnostic::cannot_assign_to_constant("ci", (371..373).into()),
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
            Diagnostic::cannot_assign_to_constant("__prg_state.OPEN", (230..234).into()),
            Diagnostic::cannot_assign_to_constant("__global_g_enum.B", (253..254).into()),
            Diagnostic::cannot_assign_to_constant("Color.red", (273..276).into()),
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
                (129..140).into()
            ),
            Diagnostic::syntax_error(
                "Value: '898JKAN' exceeds length for type: WCHAR",
                (162..171).into()
            ),
            Diagnostic::invalid_assignment("WCHAR", "CHAR", (188..195).into()),
            Diagnostic::invalid_assignment("CHAR", "WCHAR", (211..218).into()),
            Diagnostic::invalid_assignment("INT", "CHAR", (247..253).into()),
            Diagnostic::invalid_assignment("DINT", "CHAR", (269..276).into()),
            Diagnostic::invalid_assignment("STRING", "CHAR", (308..314).into()),
            Diagnostic::invalid_assignment("STRING", "WCHAR", (330..337).into()),
            Diagnostic::invalid_assignment("CHAR", "INT", (354..360).into()),
            Diagnostic::invalid_assignment("CHAR", "STRING", (376..382).into()),
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
            Diagnostic::missing_compare_function("STRING_EQUAL", "STRING", (33..43).into()),
            Diagnostic::missing_compare_function("STRING_EQUAL", "STRING", (89..99).into()),
            Diagnostic::missing_compare_function("STRING_LESS", "STRING", (145..155).into()),
            Diagnostic::missing_compare_function("STRING_GREATER", "STRING", (201..211).into()),
            Diagnostic::missing_compare_function("STRING_LESS", "STRING", (257..267).into()),
            Diagnostic::missing_compare_function("STRING_EQUAL", "STRING", (257..267).into()),
            Diagnostic::missing_compare_function("STRING_GREATER", "STRING", (313..323).into()),
            Diagnostic::missing_compare_function("STRING_EQUAL", "STRING", (313..323).into()),
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
            (113..123).into()
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
            Diagnostic::missing_compare_function("WSTRING_EQUAL", "WSTRING", (33..43).into()),
            Diagnostic::missing_compare_function("WSTRING_EQUAL", "WSTRING", (89..99).into()),
            Diagnostic::missing_compare_function("WSTRING_LESS", "WSTRING", (145..155).into()),
            Diagnostic::missing_compare_function("WSTRING_GREATER", "WSTRING", (201..211).into()),
            Diagnostic::missing_compare_function("WSTRING_LESS", "WSTRING", (257..267).into()),
            Diagnostic::missing_compare_function("WSTRING_EQUAL", "WSTRING", (257..267).into()),
            Diagnostic::missing_compare_function("WSTRING_GREATER", "WSTRING", (313..323).into()),
            Diagnostic::missing_compare_function("WSTRING_EQUAL", "WSTRING", (313..323).into()),
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

#[test]
fn switch_case() {
    // GIVEN switch case statement
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        r#"
		VAR_GLOBAL CONSTANT
			BASE : DINT := 1;
		END_VAR

		TYPE myType: ( MYTYPE_A := BASE+1 ); END_TYPE

        PROGRAM
		VAR
			input, res : DINT;
		END_VAR

			CASE input OF
				BASE:
					res := 1;
				MYTYPE_A:
					res := 2;
				MYTYPE_A+1:
					res := 3;
				4:
					res := 4;
				2*2+1:
					res := 5;
			END_CASE
		END_PROGRAM
      "#,
    );

    // THEN no errors should occure
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn switch_case_duplicate_integer_non_const_var_reference() {
    // GIVEN switch case with non constant variables
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        r#"
		VAR_GLOBAL CONSTANT
			CONST : DINT := 8;
		END_VAR

        PROGRAM
		VAR
			input, res, x, y : DINT;
		END_VAR
			x := 2;
			y := x;

			CASE input OF
				x: // x is no constant => error
					res := 1;
				y: // y is no constant => error
					res := 2;
				2+x: // x is no constant => error
					res := 3;
				CONST:
					res := 4;
				CONST+x: // x is no constant => error
					res := 5;
			END_CASE
		END_PROGRAM
      "#,
    );

    // THEN the non constant variables are reported
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::non_constant_case_condition("'x' is no const reference", (160..161).into()),
            Diagnostic::non_constant_case_condition("'y' is no const reference", (211..212).into()),
            Diagnostic::non_constant_case_condition("'x' is no const reference", (262..265).into()),
            Diagnostic::non_constant_case_condition("'x' is no const reference", (341..348).into())
        ]
    );
}

#[test]
fn switch_case_duplicate_integer() {
    // GIVEN switch case with duplicate constant conditions
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        r#"
		VAR_GLOBAL CONSTANT
			BASE : DINT := 2;
			GLOB : DINT := 2;
		END_VAR

		TYPE myType: ( MYTYPE_A := BASE*2 ); END_TYPE

        PROGRAM
		VAR
			input, res : DINT;
		END_VAR
			CASE input OF
				4:
					res := 1;
				BASE*2:
					res := 2;
				BASE+GLOB:
					res := 3;
				MYTYPE_A:
					res := 4;
				2+2:
					res := 5;
			END_CASE
		END_PROGRAM
      "#,
    );

    // THEN the non constant variables are reported
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::duplicate_case_condition(&4, (222..228).into()),
            Diagnostic::duplicate_case_condition(&4, (249..258).into()),
            Diagnostic::duplicate_case_condition(&4, (279..287).into()),
            Diagnostic::duplicate_case_condition(&4, (308..311).into()),
        ]
    );
}
