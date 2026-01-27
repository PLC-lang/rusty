use insta::assert_snapshot;
use test_utils::parse_and_validate_buffered;

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

    assert_snapshot!(diagnostics, @r"
    warning[E093]: Function declared as VOID, but trying to assign a return value
      ┌─ <internal>:3:9
      │
    3 │         foo := 1;
      │         ^^^^^^^^ Function declared as VOID, but trying to assign a return value

    error[E037]: Invalid assignment: cannot assign 'DINT' to 'foo'
      ┌─ <internal>:3:9
      │
    3 │         foo := 1;
      │         ^^^^^^^^ Invalid assignment: cannot assign 'DINT' to 'foo'
    ");
}

#[test]
fn method_input_arguments_are_optional() {
    let diagnostic = parse_and_validate_buffered(
        "
        FUNCTION_BLOCK fb
            METHOD foo
                VAR_INPUT
                    in1 : BOOL := true;
                    in2 : BOOL;
                END_VAR
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                fbInstance : fb;
            END_VAR

            // All of these are valid because parameters will default to their initial values if not given
            // explicit arguments
            fbInstance.foo();
            fbInstance.foo(in1 := TRUE);
            fbInstance.foo(in2 := TRUE);
        END_FUNCTION
        ",
    );

    assert!(diagnostic.is_empty())
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

    insta::assert_snapshot!(diagnostic, @r"
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
    ");
}

#[test]
fn methods_in_function_blocks_need_to_match_base() {
    // GIVEN a function block with a method
    // WHEN the method in the function block does not match the base method
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION_BLOCK fb
            METHOD foo
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb2 EXTENDS fb
            METHOD foo
                VAR_INPUT
                    in1 : BOOL;
                END_VAR
            END_METHOD
        END_FUNCTION_BLOCK
        ",
    );
    // THEN there should be one diagnostic -> Method foo in function block fb2 does not match base method
    assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, parameters do not match:
      ┌─ <internal>:8:20
      │
    8 │             METHOD foo
      │                    ^^^ Derived methods with conflicting signatures, parameters do not match:

    note[E118]: `foo` has more parameters than the method defined in `fb`
       ┌─ <internal>:3:20
       │
     3 │             METHOD foo
       │                    --- see also
       ·
    10 │                     in1 : BOOL;
       │                     --- see also
    ");
}

#[test]
fn only_function_blocks_can_use_extends() {
    // GIVEN a program that extends a function block
    // WHEN the program extends a function block
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION_BLOCK fb
        END_FUNCTION_BLOCK

        PROGRAM prog EXTENDS fb
        END_PROGRAM
        ",
    );
    // THEN there should be one diagnostic -> Only function blocks can use EXTENDS
    assert_snapshot!(diagnostics, @r"
    error[E110]: Subclassing is only allowed in `CLASS` and `FUNCTION_BLOCK`
      ┌─ <internal>:5:17
      │
    5 │         PROGRAM prog EXTENDS fb
      │                 ^^^^ Subclassing is only allowed in `CLASS` and `FUNCTION_BLOCK`
    ");
}

#[test]
fn redeclaration_of_variables_from_super_is_an_error() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION_BLOCK fb
            VAR
                var1 : BOOL;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb2 EXTENDS fb
            VAR
                var1 : BOOL;
            END_VAR
        END_FUNCTION_BLOCK
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E021]: Variable `var1` is already declared in parent POU `fb`
       ┌─ <internal>:10:17
       │
     4 │                 var1 : BOOL;
       │                 ---- see also
       ·
    10 │                 var1 : BOOL;
       │                 ^^^^ Variable `var1` is already declared in parent POU `fb`
    ");
}

#[test]
fn redeclaration_of_variables_from_super_super_is_an_error() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION_BLOCK fb
            VAR
                var1 : BOOL;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb2 EXTENDS fb
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb3 EXTENDS fb2
            VAR
                var1 : BOOL;
            END_VAR
        END_FUNCTION_BLOCK
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E021]: Variable `var1` is already declared in parent POU `fb`
       ┌─ <internal>:13:17
       │
     4 │                 var1 : BOOL;
       │                 ---- see also
       ·
    13 │                 var1 : BOOL;
       │                 ^^^^ Variable `var1` is already declared in parent POU `fb`
    ");
}

#[test]
fn underscore_separated_name_repetition_does_not_overflow_the_stack() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION_BLOCK great_grandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK grandparent EXTENDS great_grandparent
        VAR
            x : INT := 10;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent EXTENDS grandparent
            x := 100;
        END_FUNCTION_BLOCK
        ",
    );

    assert_snapshot!(diagnostics, @r"
    warning[E049]: Illegal access to private member grandparent.x
       ┌─ <internal>:12:13
       │
    12 │             x := 100;
       │             ^ Illegal access to private member grandparent.x
    ");
}

#[test]
fn signature_mismatch_between_base_and_interface() {
    let diagnostics = parse_and_validate_buffered(
        "
        INTERFACE intf
            METHOD foo
            END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK fb
            METHOD foo
                VAR_INPUT
                    in1 : BOOL;
                END_VAR
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb3 EXTENDS fb IMPLEMENTS intf
        END_FUNCTION_BLOCK
        ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, parameters do not match:
       ┌─ <internal>:15:24
       │
    15 │         FUNCTION_BLOCK fb3 EXTENDS fb IMPLEMENTS intf
       │                        ^^^ Derived methods with conflicting signatures, parameters do not match:

    note[E118]: `foo` has more parameters than the method defined in `intf`
       ┌─ <internal>:3:20
       │
     3 │             METHOD foo
       │                    --- see also
       ·
    10 │                     in1 : BOOL;
       │                     --- see also
    ");
}

#[test]
fn interface_method_declared_in_parent_is_allowed() {
    let diagnostics = parse_and_validate_buffered(
        "
        INTERFACE intf
            METHOD foo
            END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK fb
            METHOD foo
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb3 EXTENDS fb IMPLEMENTS intf
        END_FUNCTION_BLOCK
        ",
    );

    assert_snapshot!(diagnostics, @"");
}
