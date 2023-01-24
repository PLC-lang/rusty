use crate::{
    diagnostics::Diagnostic,
    test_utils::tests::{codegen, codegen_without_unwrap},
};

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
    let result = codegen_without_unwrap(
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
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::codegen_error("Cannot generate Literal for EmptyStatement", (238..239).into(),),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
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
fn program_empty_input_assignment() {
    // GIVEN
    let result = codegen_without_unwrap(
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
			prog(input1 := , output1 => var2, inout1 := var3);
		END_PROGRAM
		",
    );
    // THEN
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::codegen_error("Cannot generate Literal for EmptyStatement", (231..232).into(),),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn program_empty_output_assignment() {
    // GIVEN
    let result = codegen_without_unwrap(
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
			prog(input1 := var1, output1 => , inout1 := var3);
		END_PROGRAM
		",
    );
    // THEN
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::codegen_error("Cannot generate a LValue for EmptyStatement", (248..249).into(),),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
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
fn program_accepts_empty_statement_as_param() {
    // GIVEN
    let result = codegen(
        "
		FUNCTION prog
		VAR_INPUT
			in1: DINT;
			in2: DINT := 2;
		END_VAR
		VAR_OUTPUT
			out1 : DINT;
			out2 : DINT;
		END_VAR
		END_PROGRAM

		PROGRAM main
		VAR_TEMP
			x: DINT;
		END_VAR
			prog(in1 := 1, out1 => x);
		END_PROGRAM
		",

		// TODO: in assignments for stateless POUs (x := ), see how omitting the parameter is handled and copy
		// out assignments for stateful POUs: skip assignment, similar to empty input statements
    );

    // THEN
    insta::assert_snapshot!(result);
}
