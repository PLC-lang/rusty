use insta::assert_snapshot;

use crate::test_utils::tests::codegen_with_debug;

#[test]
fn implementation_added_as_subroutine() {
    //GIVEN 3 POUs
    //When compiling for debug
    let result = codegen_with_debug(
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
fn external_impl_is_not_added_as_external_subroutine() {
    //GIVEN 3 external POUs
    //When compiling for debug
    let result = codegen_with_debug(
        "
        {external} FUNCTION myFunc : DINT
        END_FUNCTION
        {external} PROGRAM myPrg
        END_PROGRAM
        {external} FUNCTION_BLOCK myFb
        END_FUNCTION_BLOCK
        ",
    );
    //The POUs don't have a subroutine entry in the debug info
    assert_snapshot!(result);
}

#[test]
fn var_and_vartemp_variables_in_pous_added_as_local() {
    //GIVEN 2 POUs
    //When compiling for debug
    let result = codegen_with_debug(
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
    let result = codegen_with_debug(
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
    let result = codegen_with_debug(
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
    let result = codegen_with_debug(
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
    let result = codegen_with_debug(
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
    let result = codegen_with_debug(
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
    let result = codegen_with_debug(
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
    let result = codegen_with_debug(
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
    let result = codegen_with_debug(
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
    let result = codegen_with_debug(
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
fn exit_statement_have_location() {
    // Let a function with an addition and a reference
    let result = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
            WHILE TRUE THEN
                EXIT;
            END_WHILE
            myFunc := 1;
        END_FUNCTION
        ",
    );
    // No line information should be added on the statements
    assert_snapshot!(result);
}

#[test]
fn if_conditions_location_marked() {
    // Let a function with an addition and a reference
    let result = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
            IF TRUE THEN
                myFunc := 1;
            ELSIF FALSE THEN
                myFunc := 1;
            ELSE
                myFunc := 1;
            END_IF
            myFunc := 1;
        END_FUNCTION
        ",
    );
    // No line information should be added on the statements
    assert_snapshot!(result);
}

#[test]
fn case_conditions_location_marked() {
    // Let a function with an addition and a reference
    let result = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
            CASE myFunc OF
            1:
                myFunc := 1;
            2:
                myFunc := 1;
            ELSE
                myFunc := 1;
            END_CASE
            myFunc := 1;
        END_FUNCTION
        ",
    );
    // No line information should be added on the statements
    assert_snapshot!(result);
}

#[test]
fn while_conditions_location_marked() {
    // Let a function with an addition and a reference
    let result = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
            WHILE myFunc > 1 DO
                myFunc := 1;
            END_WHILE
            myFunc := 1;
        END_FUNCTION
        ",
    );
    // No line information should be added on the statements
    assert_snapshot!(result);
}

#[test]
fn repeat_conditions_location_marked() {
    // Let a function with an addition and a reference
    let result = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
            REPEAT
                myFunc := 1;
            UNTIL myFunc > 10 END_REPEAT
            myFunc := 1;
        END_FUNCTION
        ",
    );
    // No line information should be added on the statements
    assert_snapshot!(result);
}

#[test]
fn for_conditions_location_marked() {
    // Let a function with an addition and a reference
    let result = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
            FOR myFunc := 1 TO 20 BY 2 DO
                myFunc := 1;
            END_FOR
        END_FUNCTION
        ",
    );
    // No line information should be added on the statements
    assert_snapshot!(result);
}

#[test]
fn array_size_correctly_set_in_dwarf_info() {
    // Let a function with an addition and a reference
    let result = codegen_with_debug(
        "
        FUNCTION foo : DINT
        VAR
            a : ARRAY[1..64] OF DINT;
        END_VAR
        END_FUNCTION
        ",
    );

    // We expect the array to have a size of 64 in the debug info, specifically for this snapshot
    // we want to make sure the entry "!DISubrange(count: 64, lowerBound: 1)" has count equal 64
    assert_snapshot!(result);
}
#[test]
fn string_size_correctly_set_in_dwarf_info() {
    // Let a function with an addition and a reference
    let result = codegen_with_debug(
        "
        VAR_GLOBAL
            a : STRING[64];
        END_VAR
        ",
    );

    // We expect the string to have a size of 65 in the debug info, specifically for this snapshot
    // we want to make sure the entry "!DISubrange(count: 65, lowerBound: 1)" has count equal 65
    //
    // Note: 65 and not 64 because of the null terminator
    assert_snapshot!(result);
}

#[test]
fn zero_sized_types_do_not_have_alignments() {
    let result = codegen_with_debug(
        "
        FUNCTION_BLOCK nonZeroA
            VAR
                idx : BYTE;
            END_VAR
        END_FUNCTION_BLOCK

        PROGRAM nonZeroB
            VAR
                idx : BYTE;
            END_VAR
        END_PROGRAM

        PROGRAM         zeroB /* empty */ END_PROGRAM
        FUNCTION_BLOCK  zeroA /* empty */ END_FUNCTION_BLOCK
        ",
    );

    // Debugging variables with a zero-sized type between two non-zero sized types will yield incorrect
    // data when displaying their content if they are given a debug-alignment. This test verifies that zero-sized types do not have an alignment in the debug information.
    // The result should have neither a `size` nor a `aligmnent` here:
    assert!(result.contains(r#"!DICompositeType(tag: DW_TAG_structure_type, name: "zeroA", scope: !2, file: !2, line: 15, flags: DIFlagPublic, elements: !15, identifier: "zeroA")"#));
    assert!(result.contains(r#"!DICompositeType(tag: DW_TAG_structure_type, name: "zeroB", scope: !2, file: !2, line: 14, flags: DIFlagPublic, elements: !15, identifier: "zeroB")"#));
}
