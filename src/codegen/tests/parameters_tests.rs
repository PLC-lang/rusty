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
            Diagnostic::codegen_error(
                "Cannot generate Literal for EmptyStatement",
                (238..239).into(),
            ),
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
#[ignore = "https://github.com/PLC-lang/rusty/issues/562, currently we only handle output/inout"]
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
#[ignore = "https://github.com/PLC-lang/rusty/issues/562"]
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
            Diagnostic::codegen_error(
                "Cannot generate Literal for EmptyStatement",
                (231..232).into(),
            ),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn program_empty_output_assignment() {
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
			prog(input1 := var1, output1 => , inout1 := var3);
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
#[ignore = "https://github.com/PLC-lang/rusty/issues/562, currently we only handle output/inout"]
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
fn program_missing_inout_assignment() {
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
			prog(input1 := var1, output1 => var2);
		END_PROGRAM
		",
    );
    // THEN
    insta::assert_snapshot!(result);
}
