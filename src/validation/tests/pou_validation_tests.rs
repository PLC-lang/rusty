use crate::test_utils::tests::{
    parse_and_validate_buffered, temp_make_me_generic_but_for_now_validate_property,
};
use insta::assert_snapshot;

#[test]
fn actions_container_no_name() {
    // GIVEN ACTIONS without a name
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered("ACTIONS ACTION myAction END_ACTION END_ACTIONS");
    // THEN there should be one diagnostic -> missing action container name
    assert_snapshot!(&diagnostics);
}

#[test]
fn class_has_implementation() {
    // GIVEN CLASS with an implementation
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        "
        CLASS myCLASS
        VAR
            LIGHT: BOOL;
        END_VAR
            LIGHT := TRUE;
        END_CLASS
    ",
    );
    // THEN there should be one diagnostic -> Class cannot have implementation
    assert_snapshot!(&diagnostics);
}

#[test]
fn program_has_super_class() {
    // GIVEN PROGRAM with a super class
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        "
        CLASS cls
        END_CLASS

        PROGRAM prog EXTENDS cls
        END_PROGRAM
    ",
    );
    // THEN there should be one diagnostic -> Program cannot have super class
    assert_snapshot!(&diagnostics);
}

#[test]
fn function_has_super_class() {
    // GIVEN FUNCTION with a super class
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        "
        CLASS cls
        END_CLASS

        FUNCTION func EXTENDS cls
        END_FUNCTION
    ",
    );
    // THEN there should be one diagnostic -> Function cannot have super class
    assert_snapshot!(&diagnostics);
}

#[test]
fn class_with_return_type() {
    // GIVEN class with a return type
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        "
        CLASS cls : INT
        END_CLASS
    ",
    );
    // THEN there should be one diagnostic -> Class cannot have a return type
    assert_snapshot!(&diagnostics);
}

#[test]
fn in_out_variable_not_allowed_in_class() {
    // GIVEN class with a VAR_IN_OUT
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        "
        CLASS cls
        VAR_IN_OUT
            var1 : BOOL;
        END_VAR
        END_CLASS
    ",
    );
    // THEN there should be one diagnostic -> Class cannot have a var in/out/inout block
    assert_snapshot!(&diagnostics);
}

#[test]
fn input_variable_not_allowed_in_class() {
    // GIVEN class with a VAR_INPUT
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        "
        CLASS cls
        VAR_INPUT
            var1 : BOOL;
        END_VAR
        END_CLASS
    ",
    );
    // THEN there should be one diagnostic -> Class cannot have a var in/out/inout block
    assert_snapshot!(&diagnostics);
}

#[test]
fn output_variable_not_allowed_in_class() {
    // GIVEN class with a VAR_OUTPUT
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        "
        CLASS cls
        VAR_OUTPUT
            var1 : BOOL;
        END_VAR
        END_CLASS
    ",
    );
    // THEN there should be one diagnostic -> Class cannot have a var in/out/inout block
    assert_snapshot!(&diagnostics);
}

#[test]
fn local_variable_allowed_in_class() {
    // GIVEN class with a VAR
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate_buffered(
        "
        CLASS cls
        VAR
            var1 : BOOL;
        END_VAR
        END_CLASS
    ",
    );
    // THEN there should be no diagnostic -> Class can have local var block
    assert_snapshot!(&diagnostics);
}

#[test]
fn do_not_validate_external() {
    // GIVEN an external program with a simple assignment
    // for this kind of assignment our validator would report
    // potential loss of information (assigning bigger to smaller type)
    // WHEN ...
    let diagnostics = parse_and_validate_buffered(
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
    let diagnostics = parse_and_validate_buffered(
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
        myVar   : BOOL;
    END_VAR
    VAR_INPUT
        myInput : USINT;
    END_VAR
    VAR_IN_OUT
        myInOut : BOOL;
    END_VAR
    VAR_OUTPUT
        myOut   : BOOL;
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

    assert_snapshot!(diagnostics);
}

#[test]
fn assigning_return_value_to_void_functions_returns_error() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION foo
        foo := 1;
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @r###"
    warning[E093]: Function declared as VOID, but trying to assign a return value
      ┌─ <internal>:3:9
      │
    3 │         foo := 1;
      │         ^^^^^^^^ Function declared as VOID, but trying to assign a return value

    "###);
}

#[test]
fn method_input_arguments_are_not_optional() {
    let diagnostic = parse_and_validate_buffered(
        "
        FUNCTION_BLOCK fb
            METHOD foo
                VAR_INPUT
                    in1 : BOOL;
                    in2 : BOOL;
                END_VAR
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                fbInstance : fb;
            END_VAR

            // All of these are invalid because they are missing arguments
            fbInstance.foo();
            fbInstance.foo(in1 := TRUE);
            fbInstance.foo(in2 := TRUE);

            // These are valid
            fbInstance.foo(in1 := TRUE, in2 := TRUE);
            fbInstance.foo(in2 := TRUE, in1 := TRUE);
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostic, @r###"
    error[E030]: Argument `in1` is missing
       ┌─ <internal>:17:13
       │
    17 │             fbInstance.foo();
       │             ^^^^^^^^^^^^^^ Argument `in1` is missing

    error[E030]: Argument `in2` is missing
       ┌─ <internal>:17:13
       │
    17 │             fbInstance.foo();
       │             ^^^^^^^^^^^^^^ Argument `in2` is missing

    error[E030]: Argument `in2` is missing
       ┌─ <internal>:18:13
       │
    18 │             fbInstance.foo(in1 := TRUE);
       │             ^^^^^^^^^^^^^^ Argument `in2` is missing

    error[E030]: Argument `in1` is missing
       ┌─ <internal>:19:13
       │
    19 │             fbInstance.foo(in2 := TRUE);
       │             ^^^^^^^^^^^^^^ Argument `in1` is missing

    "###);
}
#[test]
fn property_within_function_pou() {
    let diagnostics = temp_make_me_generic_but_for_now_validate_property(
        r"
        FUNCTION foo : DINT
            PROPERTY prop : DINT
            END_PROPERTY
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @r"");
}

#[test]
fn property_with_more_than_one_get_block() {
    let diagnostics = temp_make_me_generic_but_for_now_validate_property(
        r"
        FUNCTION_BLOCK foo
            PROPERTY prop : DINT
                GET
                END_Get
                GET
                END_Get
            END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );
    assert_snapshot!(diagnostics, @r"
    ");
}

#[test]
fn property_TMP() {
    let diagnostics = temp_make_me_generic_but_for_now_validate_property(
        r"
        FUNCTION_BLOCK foo
            PROPERTY prop : DINT
              SET
                  1 + 2;
                END_SET
            END_PROPERTY
            PROPERTY prop : DINT
              SET
                  1 + 2;
                END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );

    assert_snapshot!(diagnostics, @r"
    ");
}

#[test]
fn property_with_var_output_in_get_block() {
    let diagnostics = temp_make_me_generic_but_for_now_validate_property(
        r"
        FUNCTION_BLOCK foo
            PROPERTY prop : DINT
              GET
                  VAR_OUTPUT
                    out : DINT;
                  END_VAR
              END_Get
            END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );

    assert_snapshot!(diagnostics, @r"
    ");
}

#[test]
fn method_inout_arguments_are_not_optional() {
    let diagnostic = parse_and_validate_buffered(
        "
        FUNCTION_BLOCK fb
            METHOD foo
                VAR_IN_OUT
                    in1 : BOOL;
                    in2 : BOOL;
                END_VAR
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                fbInstance : fb;
                localIn1 : BOOL;
                localIn2 : BOOL;
            END_VAR

            // All of these are invalid because they are missing arguments
            fbInstance.foo();
            fbInstance.foo(in1 := localIn1);
            fbInstance.foo(in2 := localIn2);

            // These are valid
            fbInstance.foo(in1 := localIn1, in2 := localIn2);
            fbInstance.foo(in2 := localIn2, in1 := localIn1);
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostic, @r###"
    error[E030]: Argument `in1` is missing
       ┌─ <internal>:19:13
       │
    19 │             fbInstance.foo();
       │             ^^^^^^^^^^^^^^ Argument `in1` is missing

    error[E030]: Argument `in2` is missing
       ┌─ <internal>:19:13
       │
    19 │             fbInstance.foo();
       │             ^^^^^^^^^^^^^^ Argument `in2` is missing

    error[E030]: Argument `in2` is missing
       ┌─ <internal>:20:13
       │
    20 │             fbInstance.foo(in1 := localIn1);
       │             ^^^^^^^^^^^^^^ Argument `in2` is missing

    error[E030]: Argument `in1` is missing
       ┌─ <internal>:21:13
       │
    21 │             fbInstance.foo(in2 := localIn2);
       │             ^^^^^^^^^^^^^^ Argument `in1` is missing

    "###);
}
