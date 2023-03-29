use crate::{assert_validation_snapshot, test_utils::tests::parse_and_validate};

#[test]
fn uninitialized_constants_fall_back_to_the_default() {
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

    assert_eq!(diagnostics, vec![]);
}

#[test]
fn unresolvable_variables_are_reported() {
    let diagnostics = parse_and_validate(
        "
        VAR_GLOBAL 
            gX : INT := 7 + cgX;
            gXi : INT := 7;
        END_VAR

        VAR_GLOBAL CONSTANT
            cgX : INT;  //default
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
                cai : INT := a;  //unresolvable
            END_VAR
        END_PROGRAM
       ",
    );

    assert_validation_snapshot!(&diagnostics);
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
    assert_validation_snapshot!(&diagnostics);
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
    assert_validation_snapshot!(&diagnostics);
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

    assert_validation_snapshot!(&diagnostics);
}