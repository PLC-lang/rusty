use crate::test_utils::tests::codegen;

#[test]
fn function_all_parameters_assigned() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAprogram_accepts_empty_statement_as_output_paramR
            foo(var1, var2, var3);
            foo(input1 := var1, output1 => var2, inout1 := var3);
            foo(inout1 := var3, input1 := var1, output1 => var2);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_empty_input_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(input1 := , output1 => var2, inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_empty_output_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(input1 := var1, output1 => , inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_empty_output_default_value_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT := 3;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(input1 := var1, output1 => , inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_empty_inout_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(input1 := var1, output1 => var2, inout1 := );
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_missing_input_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(output1 => var2, inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_missing_input_default_value_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT := 10;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(output1 => var2, inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_missing_output_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(input1 := var1, inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_missing_output_default_value_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT := 3;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(input1 := var1, inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_missing_inout_assignment() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(input1 := var1, output1 => var2);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_default_value_parameter_type() {
    // GIVEN
    let result = codegen(
        "
        TYPE myType : DINT := 20; END_TYPE

        FUNCTION foo : DINT
        VAR_INPUT
            input1 : myType;
        END_VAR
        VAR_OUTPUT
            output1 : myType;
            output2 : myType;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            foo(output2 => , inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn program_all_parameters_assigned_explicit() {
    // GIVEN
    let result = codegen(
        "
        PROGRAM prog
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            prog(input1 := var1, output1 => var2, inout1 := var3);
            prog(inout1 := var3, input1 := var1, output1 => var2);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn program_all_parameters_assigned_implicit() {
    // GIVEN
    let result = codegen(
        "
        PROGRAM prog
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            prog(var1, var2, var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn program_empty_inout_assignment() {
    // GIVEN
    let result = codegen(
        "
        PROGRAM prog
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            prog(input1 := var1, output1 => var2, inout1 := );
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn program_missing_input_assignment() {
    // GIVEN
    let result = codegen(
        "
        PROGRAM prog
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            prog(output1 => var2, inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn program_missing_output_assignment() {
    // GIVEN
    let result = codegen(
        "
        PROGRAM prog
        VAR_INPUT
            input1 : DINT;
        END_VAR
        VAR_OUTPUT
            output1 : DINT;
        END_VAR
        VAR_IN_OUT
            inout1 : DINT;
        END_VAR
        END_PROGRAM

        PROGRAM main
        VAR
            var1, var2, var3 : DINT;
        END_VAR
            prog(input1 := var1, inout1 := var3);
        END_PROGRAM
        ",
    );
    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn program_accepts_empty_statement_as_input_param() {
    // GIVEN
    let result = codegen(
        "
        PROGRAM prog
        VAR_INPUT
            in1: DINT;
            in2: DINT;
        END_VAR
        END_PROGRAM

        PROGRAM main
            prog(in1 := 1, in2 := );
        END_PROGRAM
        ",
    );

    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn program_accepts_empty_statement_as_output_param() {
    // GIVEN
    let result = codegen(
        "
        PROGRAM prog
        VAR_OUTPUT
            out1 : DINT;
            out2 : DINT;
        END_VAR
            out1 := 1;
            out2 := 2;
        END_PROGRAM

        PROGRAM main
        VAR
            x : DINT;
        END_VAR
            prog( out1 => x, out2 => );
        END_PROGRAM
        ",
    );

    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn fb_accepts_empty_statement_as_input_param() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION_BLOCK fb_t
        VAR_INPUT
            in1: DINT;
            in2: DINT;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
        VAR
            fb : fb_t;
        END_VAR
            fb(in1 := 1, in2 := );
        END_PROGRAM
        ",
    );

    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn fb_accepts_empty_statement_as_output_param() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION_BLOCK fb_t
        VAR_OUTPUT
            out1 : DINT;
            out2 : DINT;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
        VAR
            fb : fb_t;
            x : DINT;
        END_VAR
            fb( out1 => x, out2 => );
        END_PROGRAM
        ",
    );

    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_accepts_empty_statement_as_input_param() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo
        VAR_INPUT
            in1: DINT;
            in2: DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
            foo(in1 := 1, in2 := );
        END_PROGRAM
        ",
    );

    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn function_accepts_empty_statement_as_output_param() {
    // GIVEN
    let result = codegen(
        "
        FUNCTION foo
        VAR_OUTPUT
            out1 : DINT;
            out2 : DINT;
        END_VAR
        END_FUNCTION

        PROGRAM main
        VAR
            x: DINT;
        END_VAR
            foo( out1 => x, out2 => );
        END_PROGRAM
        ",
    );

    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn parameters_behind_function_block_pointer_are_assigned_to() {
    // GIVEN
    let result = codegen(
        "
        PROGRAM main
        VAR
            file : file_t;
            FileOpen : REF_TO file_t;
        END_VAR
            FileOpen := &file;
            FileOpen^(var2:=TRUE);
        END_PROGRAM

        FUNCTION_BLOCK file_t
        VAR_INPUT
            var1 : BOOL;
            var2 : BOOL;
        END_VAR
        END_FUNCTION_BLOCK
        ",
    );

    // THEN
    insta::assert_snapshot!(result);
}

#[test]
fn var_in_out_params_can_be_out_of_order() {
    let res = codegen(
        "PROGRAM mainProg
    VAR
        fb : fb_t;
        out1, out2 : BOOL;
    END_VAR
        fb(myOtherInOut := out1, myInOut := out2);
        fb(myInOut := out1, myOtherInOut := out2);

        fb.foo(myOtherInOut := out2, myInOut := out1);
        fb.foo(myInOut := out2, myOtherInOut := out1);
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
    END_ACTIONS",
    );

    insta::assert_snapshot!(res);
}
