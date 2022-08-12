use crate::ast::SourceRange;
use crate::test_utils::tests::parse_and_validate;
use crate::Diagnostic;

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
            Diagnostic::unresolved_constant("cx", None, SourceRange::new(340..342,Some(18),Some(17),Some(18),Some(19))),
            Diagnostic::unresolved_constant("cgX", None, SourceRange::new(128..131,Some(7),Some(13),Some(7),Some(16))),
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
            Diagnostic::unresolved_constant("cx", None, SourceRange::new(392..394,Some(18),Some(29),Some(18),Some(31))),
            Diagnostic::unresolved_constant("cai", None, SourceRange::new(473..474,Some(20),Some(17),Some(20),Some(20))),
            Diagnostic::unresolved_constant("gX", None, SourceRange::new(45..52, Some(2),Some(25),Some(2),Some(32))),
            Diagnostic::unresolved_constant("cgX", None, SourceRange::new(154..157, Some(7), Some(13), Some(7), Some(16))),
        ]
    );
}

#[test]
fn constant_on_illegal_var_blocks_cause_validation_issue() {
    // GIVEN different variable block types with the CONSTANT modifier
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        "
        VAR_GLOBAL CONSTANT //OK
        END_VAR

        PROGRAM prg
            VAR_INPUT CONSTANT //illegal
            END_VAR

            VAR_OUTPUT CONSTANT //illegal
            END_VAR

            VAR_IN_OUT CONSTANT //illegal
            END_VAR

            VAR CONSTANT //ok
            END_VAR

        END_PROGRAM

        CLASS cls
            VAR CONSTANT //ok
            END_VAR

             METHOD testMethod
                VAR_INPUT CONSTANT //illegal
                END_VAR

                VAR_OUTPUT CONSTANT //illegal
                END_VAR

                VAR_IN_OUT CONSTANT //illegal
                END_VAR

                VAR CONSTANT //ok
                END_VAR
            END_METHOD
        END_CLASS
       ",
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_constant_block(SourceRange::new(83..92,Some(5),Some(13),Some(5),Some(22))), // VAR_INPUT
            Diagnostic::invalid_constant_block(SourceRange::new(145..155,Some(8),Some(13),Some(8),Some(23))), // VAR_OUTPUT
            Diagnostic::invalid_constant_block(SourceRange::new(208..218,Some(11),Some(13),Some(11),Some(23))), // VAR_IN_OUT
            Diagnostic::invalid_constant_block(SourceRange::new(447..456, Some(24),Some(17),Some(24),Some(26))), // VAR_INPUT
            Diagnostic::invalid_constant_block(SourceRange::new(517..527, Some(27),Some(17),Some(27),Some(27))), // VAR_OUTPUT
            Diagnostic::invalid_constant_block(SourceRange::new(588..598, Some(30),Some(17),Some(30),Some(27))), // VAR_IN_OUT
        ]
    );
}

#[test]
fn constant_fb_instances_are_illegal() {
    // GIVEN a couple of constants, including FB instances and class-instances
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        "
        FUNCTION_BLOCK MyFb
            ;
        END_FUNCTION_BLOCK

        CLASS cls
            METHOD testMethod : INT
                VAR_INPUT myMethodArg : INT; END_VAR
                testMethod := 1;
            END_METHOD
        END_CLASS
 
        VAR_GLOBAL CONSTANT
            x : INT := 1;
            y : MyFb;
            z : cls;
        END_VAR
      ",
    );

    // THEN everything but VAR and VAR_GLOBALS are reported
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::unresolved_constant("y", None, SourceRange::new(320..321,Some(14),Some(13),Some(14),Some(14))),
            Diagnostic::invalid_constant("y", SourceRange::new(320..321,Some(14),Some(13),Some(14),Some(14))),
            Diagnostic::unresolved_constant("z", None, SourceRange::new(342..343,Some(15),Some(13),Some(15),Some(14))),
            Diagnostic::invalid_constant("z", SourceRange::new(342..343,Some(15),Some(13),Some(15),Some(14))),
        ]
    );
}

#[test]
fn sized_varargs_require_type() {
    // GIVEN a function with a untyped sized variadic argument
    // WHEN it is validated
    let diagnostics = parse_and_validate(
        "
        FUNCTION f_with_var : INT
        VAR_INPUT
            in   : INT;
            args : {sized}...;
        END_VAR
        END_FUNCTION
      ",
    );

    assert_eq!(
        diagnostics,
        vec![Diagnostic::missing_datatype(
            Some(": Sized Variadics require a known datatype."),
            SourceRange::new(103..106, Some(4), Some(27), Some(4), Some(29))
        ),]
    )
}
