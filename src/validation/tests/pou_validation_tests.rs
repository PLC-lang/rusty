use crate::{assert_validation_snapshot, test_utils::tests::parse_and_validate};

#[test]
fn function_no_return_unsupported() {
    // GIVEN FUNCTION with no return type
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate("FUNCTION foo VAR_INPUT END_VAR END_FUNCTION");
    // THEN there should be one diagnostic -> missing return type
    assert_validation_snapshot!(&diagnostics);
}

#[test]
fn actions_container_no_name() {
    // GIVEN ACTIONS without a name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate("ACTIONS ACTION myAction END_ACTION END_ACTIONS");
    // THEN there should be one diagnostic -> missing action container name
    assert_validation_snapshot!(&diagnostics);
}

#[test]
fn do_not_validate_external() {
    // GIVEN an external program with a simple assignment
    // for this kind of assignment our validator would report
    // potential loss of information (assigning bigger to smaller type)
    // WHEN ...
    let diagnostics = parse_and_validate(
        "
    PROGRAM main
    END_PROGRAM

    {external}
    PROGRAM program_0
    VAR
        x : SINT;
        y : INT;
    END_VAR
        x := y;
    END_PROGRAM
    ",
    );
    // THEN there should not be any reported diagnostic for the external program
    assert!(diagnostics.is_empty());
}

#[test]
fn in_out_variable_out_of_order() {
    let diagnostics = parse_and_validate(
        "
    PROGRAM mainProg
    VAR
        fb : fb_t;
        out1, out2 : BOOL;
    END_VAR
        fb(myOtherInOut := out1, myInOut := out2);  // valid
        fb(myInOut := out1, myOtherInOut := out2);  // valid
        fb(myInOut := out2); // invalid: missing in-out param
        fb(0, TRUE);  // invalid: one in-out is a literal, the other is missing

        fb.foo(myOtherInOut := out2, myInOut := out1); // valid
    END_PROGRAM
    
    FUNCTION_BLOCK fb_t
    VAR
        myVar	: BOOL;
    END_VAR
    VAR_INPUT
        myInput	: USINT;    
    END_VAR
    VAR_IN_OUT
        myInOut	: BOOL;
    END_VAR
    VAR_OUTPUT
        myOut	: BOOL;
    END_VAR
    VAR_IN_OUT
        myOtherInOut : BOOL;
    END_VAR
    END_FUNCTION_BLOCK

    ACTIONS
        ACTION foo
            myInOut := myOtherInOut;
        END_ACTION
    END_ACTIONS
    ",
    );

    assert_validation_snapshot!(diagnostics);
}
