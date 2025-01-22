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

#[test]
fn property() {
    let diagnostics = temp_make_me_generic_but_for_now_validate_property(
        r"
        FUNCTION_BLOCK fb
            PROPERTY prop : DINT
            END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION main
        END_FUNCTION
    ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E001]: Property has neither a GET nor a SET block
      ┌─ <internal>:3:22
      │
    3 │             PROPERTY prop : DINT
      │                      ^^^^ Property has neither a GET nor a SET block
    ");
}

#[test]
fn property_with_more_than_one_get_block() {
    let diagnostics = temp_make_me_generic_but_for_now_validate_property(
        r"
        FUNCTION_BLOCK fb
            PROPERTY prop : DINT
                GET
                END_GET

                GET
                END_GET
            END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION main
        END_FUNCTION
    ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E001]: Property has more than one GET block
      ┌─ <internal>:3:22
      │
    3 │             PROPERTY prop : DINT
      │                      ^^^^ Property has more than one GET block
    ");
}

#[test]
fn property_with_more_than_one_set_block() {
    let diagnostics = temp_make_me_generic_but_for_now_validate_property(
        r"
        FUNCTION_BLOCK fb
            PROPERTY prop : DINT
                SET
                END_SET

                SET
                END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION main
        END_FUNCTION
    ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E001]: Property has more than one SET block
      ┌─ <internal>:3:22
      │
    3 │             PROPERTY prop : DINT
      │                      ^^^^ Property has more than one SET block
    ");
}

#[test]
fn property_with_unsupported_variable_type_blocks() {
    let mut result = String::new();
    for kind in vec!["VAR_INPUT", "VAR_OUTPUT", "VAR_IN_OUT"] {
        let code = format!(
            "
            FUNCTION_BLOCK fb
                PROPERTY prop : DINT
                    GET
                        {kind}
                        END_VAR
                    END_GET
                END_PROPERTY
            END_FUNCTION_BLOCK

            FUNCTION main
            END_FUNCTION
            "
        );

        let diagnostics = temp_make_me_generic_but_for_now_validate_property(&code);
        result = format!("{result}\n{diagnostics}");
    }

    assert_snapshot!(result, @r"
    error[E001]: Invalid variable block type, only blocks of type VAR are allowed
      ┌─ <internal>:3:26
      │
    3 │                 PROPERTY prop : DINT
      │                          ^^^^ Invalid variable block type, only blocks of type VAR are allowed


    error[E001]: Invalid variable block type, only blocks of type VAR are allowed
      ┌─ <internal>:3:26
      │
    3 │                 PROPERTY prop : DINT
      │                          ^^^^ Invalid variable block type, only blocks of type VAR are allowed


    error[E001]: Invalid variable block type, only blocks of type VAR are allowed
      ┌─ <internal>:3:26
      │
    3 │                 PROPERTY prop : DINT
      │                          ^^^^ Invalid variable block type, only blocks of type VAR are allowed
    ");
}

#[test]
fn property_defined_in_unsupported_pous_yields_an_error() {
    let code = format!(
        "
        CLASS foo
            PROPERTY prop : DINT
                GET
                END_GET
            END_PROPERTY
        END_CLASS

        FUNCTION bar
            PROPERTY prop : DINT
                GET
                END_GET
            END_PROPERTY
        END_CLASS
        "
    );

    let diagnostics = temp_make_me_generic_but_for_now_validate_property(&code);

    // TODO: Also pass parent location as secondary location in diagnostic
    // TODO: Update parser error "Methods cannot be declared in a POU of type"
    assert_snapshot!(diagnostics, @r"
    error[E001]: Methods cannot be declared in a POU of type 'Function'.
      ┌─ <internal>:9:18
      │
    9 │         FUNCTION bar
      │                  ^^^ Methods cannot be declared in a POU of type 'Function'.

    error[E007]: Unexpected token: expected KeywordEndFunction but found END_CLASS
       ┌─ <internal>:14:9
       │
    14 │         END_CLASS
       │         ^^^^^^^^^ Unexpected token: expected KeywordEndFunction but found END_CLASS

    error[E001]: Property only allowed in FunctionBlock or Program
      ┌─ <internal>:3:22
      │
    3 │             PROPERTY prop : DINT
      │                      ^^^^ Property only allowed in FunctionBlock or Program

    error[E001]: Property only allowed in FunctionBlock or Program
       ┌─ <internal>:10:22
       │
    10 │             PROPERTY prop : DINT
       │                      ^^^^ Property only allowed in FunctionBlock or Program

    error[E001]: Property has more than one GET block
       ┌─ <internal>:10:22
       │
    10 │             PROPERTY prop : DINT
       │                      ^^^^ Property has more than one GET block
    ");
}

#[test]
fn duplicate_property_name() {
    let code = r"
        FUNCTION_BLOCK foo
            PROPERTY bar : DINT
            END_PROPERTY

            PROPERTY bar : DINT
            END_PROPERTY

            PROPERTY bar : DINT
            END_PROPERTY

            PROPERTY bar : DINT
            END_PROPERTY
        END_FUNCTION_BLOCK
    ";

    insta::assert_snapshot!(temp_make_me_generic_but_for_now_validate_property(code), @r"
    error[E001]: Property has neither a GET nor a SET block
      ┌─ <internal>:3:22
      │
    3 │             PROPERTY bar : DINT
      │                      ^^^ Property has neither a GET nor a SET block

    error[E001]: Property has neither a GET nor a SET block
      ┌─ <internal>:6:22
      │
    6 │             PROPERTY bar : DINT
      │                      ^^^ Property has neither a GET nor a SET block

    error[E001]: Property has neither a GET nor a SET block
      ┌─ <internal>:9:22
      │
    9 │             PROPERTY bar : DINT
      │                      ^^^ Property has neither a GET nor a SET block

    error[E001]: Property has neither a GET nor a SET block
       ┌─ <internal>:12:22
       │
    12 │             PROPERTY bar : DINT
       │                      ^^^ Property has neither a GET nor a SET block

    error[E001]: Duplicate symbol `foo.bar`
       ┌─ <internal>:3:22
       │
     3 │             PROPERTY bar : DINT
       │                      ^^^ Duplicate symbol `foo.bar`
       ·
     6 │             PROPERTY bar : DINT
       │                      --- see also
       ·
     9 │             PROPERTY bar : DINT
       │                      --- see also
       ·
    12 │             PROPERTY bar : DINT
       │                      --- see also
    ");
}

#[test]
fn duplicate_property_in_one_function_block_but_not_the_other() {
    let code = r"
        FUNCTION_BLOCK prop1
            PROPERTY foo : DINT
            END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK prop2
            PROPERTY foo : DINT
            END_PROPERTY

            PROPERTY foo : DINT
            END_PROPERTY
        END_FUNCTION_BLOCK
    ";

    insta::assert_snapshot!(temp_make_me_generic_but_for_now_validate_property(code), @r"
    error[E001]: Property has neither a GET nor a SET block
      ┌─ <internal>:3:22
      │
    3 │             PROPERTY foo : DINT
      │                      ^^^ Property has neither a GET nor a SET block

    error[E001]: Property has neither a GET nor a SET block
      ┌─ <internal>:8:22
      │
    8 │             PROPERTY foo : DINT
      │                      ^^^ Property has neither a GET nor a SET block

    error[E001]: Property has neither a GET nor a SET block
       ┌─ <internal>:11:22
       │
    11 │             PROPERTY foo : DINT
       │                      ^^^ Property has neither a GET nor a SET block

    error[E001]: Duplicate symbol `prop2.foo`
       ┌─ <internal>:8:22
       │
     8 │             PROPERTY foo : DINT
       │                      ^^^ Duplicate symbol `prop2.foo`
       ·
    11 │             PROPERTY foo : DINT
       │                      --- see also
    ");
}

#[test]
fn same_name_function_block_and_property() {
    let code = r"
        FUNCTION_BLOCK foo
            PROPERTY foo : DINT
            GET END_GET
            END_PROPERTY
        END_FUNCTION_BLOCK
    ";

    // TODO: Is this ok, should there be an error (from the compiler perspective theres no problem, obviously)?
    insta::assert_snapshot!(temp_make_me_generic_but_for_now_validate_property(code), @r"");
}
