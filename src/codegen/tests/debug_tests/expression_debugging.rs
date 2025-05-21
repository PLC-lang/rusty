use plc_util::filtered_snapshot;

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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
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
    filtered_snapshot!(result);
}

#[test]
fn zero_sized_types_offset_and_size_are_correct() {
    let result = codegen_with_debug(
        "
        PROGRAM mainProg
        VAR
            i : UINT;
            arr1 : ARRAY [0..10] OF BYTE;
            fb : zeroSize;
            arr2 : ARRAY [0..10] OF BYTE;
        END_VAR
        END_PROGRAM

        FUNCTION_BLOCK zeroSize
            VAR
            END_VAR
        END_FUNCTION_BLOCK
        ",
    );
    // We expect the element after the zero sized member to have the offset same offset as the zero sized member
    assert!(result.contains(r#"!DIDerivedType(tag: DW_TAG_member, name: "fb", scope: !2, file: !2, line: 6, baseType: !13, align: 64, offset: 104, flags: DIFlagPublic)"#));
    assert!(result.contains(r#"!DIDerivedType(tag: DW_TAG_member, name: "arr2", scope: !2, file: !2, line: 7, baseType: !8, size: 88, align: 8, offset: 104, flags: DIFlagPublic)"#));
    // We also expect the zero sized type to not have a size set in the debug info
    assert!(result.contains(r#"!DICompositeType(tag: DW_TAG_structure_type, name: "zeroSize", scope: !2, file: !2, line: 11, align: 64, flags: DIFlagPublic, elements: !14, identifier: "zeroSize")"#));
}
