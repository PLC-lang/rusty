use insta::assert_snapshot;

use crate::test_utils::tests::codegen_with_debug;

#[test]
fn implementation_added_as_subroutine() {
    //GIVEN 3 POUs
    //When compiling for debug
    let (result, _) = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
        END_FUNCTION
        PROGRAM myPrg
        END_PROGRAM
        FUNCTION_BLOCK myFb
        END_FUNCTION_BLOCK
        ",
    );
    //The POUs has a subroutine entry in the debug info
    assert_snapshot!(result);
}

#[test]
fn external_impl_added_as_external_subroutine() {
    //GIVEN 3 external POUs
    //When compiling for debug
    let (result, _) = codegen_with_debug(
        "
        {external} FUNCTION myFunc : DINT
        END_FUNCTION
        {external} PROGRAM myPrg
        END_PROGRAM
        {external} FUNCTION_BLOCK myFb
        END_FUNCTION_BLOCK
        ",
    );
    //The POUs has a subroutine entry in the debug info
    assert_snapshot!(result);
}

#[test]
fn var_and_vartemp_variables_in_pous_added_as_local() {
    //GIVEN 2 POUs
    //When compiling for debug
    let (result, _) = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
        VAR a,b,c: DINT; END_VAR
        END_FUNCTION
        PROGRAM myPrg
        VAR_TEMP a,b,c: DINT; END_VAR
        END_PROGRAM
        FUNCTION_BLOCK myFb
        VAR_TEMP a,b,c: DINT; END_VAR
        END_FUNCTION_BLOCK
        ",
    );
    //The POUs has a subroutine entry in the debug info
    assert_snapshot!(result);
}

#[test]
fn var_in_out_inout_in_function_added_as_params() {
    // Let a function with an assignment
    let (result, _) = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
        VAR_IN_OUT
            x : INT;
        END_VAR
            myFunc := x + 2;
        END_FUNCTION
        ",
    );
    //The asignment should recieve a debug info entry
    assert_snapshot!(result);
}

#[test]
fn non_function_pous_have_struct_as_param() {
    // Let a function with an assignment
    let (result, _) = codegen_with_debug(
        "
        PROGRAM myProg
        VAR_INPUT
            x : DINT;
        END_VAR
            x := x + 2;
        END_PROGRAM

        FUNCTION_BLOCK fb
        VAR_INPUT
            x : DINT;
        END_VAR
            x := x + 2;
        END_FUNCTION_BLOCK
        ",
    );
    //The asignment should recieve a debug info entry
    assert_snapshot!(result);
}

#[test]
fn assignment_statement_have_location() {
    // Let a function with an assignment
    let (result, _) = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
            myFunc := 1 + 2;
        END_FUNCTION
        ",
    );
    //The asignment should recieve a debug info entry
    assert_snapshot!(result);
}

#[test]
fn function_calls_have_location() {
    // Let a function with a call statement
    let (result, _) = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
            myFunc();
        END_FUNCTION
        ",
    );
    //The asignment should recieve a debug info entry
    assert_snapshot!(result);
}

#[test]
fn function_calls_in_expressions_have_location() {
    // Let a function with a call statement in an addition
    let (result, _) = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
            1 + myFunc();
        END_FUNCTION
        ",
    );
    //The call should recieve a debug info entry
    assert_snapshot!(result);
}

#[test]
fn nested_function_calls_get_location() {
    // Let a function with a call statement in an addition
    let (result, _) = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
        VAR_INPUT x : DINT; END_VAR
            myFunc(myFunc(1));
        END_FUNCTION
        ",
    );
    //The call should recieve a debug info entry
    assert_snapshot!(result);
}

#[test]
fn non_callable_expressions_have_no_location() {
    // Let a function with an addition and a reference
    let (result, _) = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
            1 + 2;
            myFunc;
        END_FUNCTION
        ",
    );
    // No line information should be added on the statements
    assert_snapshot!(result);
}

#[test]
fn return_statement_have_location() {
    // Let a function with an addition and a reference
    let (result, _) = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
            RETURN;
        END_FUNCTION
        ",
    );
    // No line information should be added on the statements
    assert_snapshot!(result);
}

#[test]
fn aggregate_return_value_variable_in_function() {
    // Let a function with an addition and a reference
    let (result, _) = codegen_with_debug(
        "
        FUNCTION myFunc : STRING
            myFunc := 'hello';
        END_FUNCTION
        ",
    );
    // No line information should be added on the statements
    assert_snapshot!(result);
    
}

#[test]
#[ignore]
fn exit_statement_have_location() {
    todo!("Exits belong in loops")
}
