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
