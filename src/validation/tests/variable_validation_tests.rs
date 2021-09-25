use crate::{validation::tests::parse_and_validate, Diagnostic};

#[test]
fn uninitialized_constants_are_reported() {
    let diagnostics = parse_and_validate(
        "
        VAR_GLOBAL 
            gX : INT;
            gXi : INT := 7;
        END_VAR

        VAR_GLOBAL CONSTANT
            cgX : INT;
            cgXi : INT := 7;
        END_VAR

        PROGRAM prg
            VAR 
                x : INT;
                xi : INT := 7;
            END_VAR

            VAR CONSTANT
                cx : INT;
                cxi : INT := 7;
            END_VAR
        END_PROGRAM
       ",
    );

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::unresolved_constant("cx", None, (340..342).into()),
            Diagnostic::unresolved_constant("cgX", None, (128..131).into()),
        ]
    );
}

#[test]
fn unresolvable_variables_are_reported() {
    let diagnostics = parse_and_validate(
        "
        VAR_GLOBAL 
            gX : INT := 7 + cgX; //unresolvable
            gXi : INT := 7;
        END_VAR

        VAR_GLOBAL CONSTANT
            cgX : INT;  //unresolved
            cgXi : INT := 7;
        END_VAR

        PROGRAM prg
            VAR 
                x : INT;
                xi : INT := 7;
            END_VAR

            VAR CONSTANT
                cx : INT := cx;  //unresolvable
                cxi : INT := 7;
                cai : INT := a;
            END_VAR
        END_PROGRAM
       ",
    );

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::unresolved_constant("cx", None, (392..394).into()),
            Diagnostic::unresolved_constant("cai", None, (473..474).into()),
            Diagnostic::unresolved_constant("gX", None, (45..52).into()),
            Diagnostic::unresolved_constant("cgX", None, (154..157).into()),
        ]
    );
}
