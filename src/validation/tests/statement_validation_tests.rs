use crate::test_utils::tests::parse_and_validate;
use crate::Diagnostic;

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
