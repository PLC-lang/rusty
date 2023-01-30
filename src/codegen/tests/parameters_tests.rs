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
		END_VAR
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
