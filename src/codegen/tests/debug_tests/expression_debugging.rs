use plc_util::filtered_assert_snapshot;

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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
}

#[test]
fn ref_assignment_statement_have_location() {
    // A REF= assignment must carry !dbg metadata so debuggers can stop on the line.
    let result = codegen_with_debug(
        "
        FUNCTION myFunc
        VAR
            a : DINT;
            b : REF_TO DINT;
        END_VAR
            b REF= a;
        END_FUNCTION
        ",
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define void @myFunc() !dbg !4 {
    entry:
      %a = alloca i32, align [filtered]
      %b = alloca ptr, align [filtered]
        #dbg_declare(ptr %a, !9, !DIExpression(), !11)
      store i32 0, ptr %a, align [filtered]
        #dbg_declare(ptr %b, !12, !DIExpression(), !15)
      store ptr null, ptr %b, align [filtered]
      store ptr %a, ptr %b, align [filtered], !dbg !16
      ret void, !dbg !17
    }

    !llvm.module.flags = !{!0, !1}
    !llvm.dbg.cu = !{!2}

    !0 = !{i32 2, !"Dwarf Version", i32 5}
    !1 = !{i32 2, !"Debug Info Version", i32 3}
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "<internal>", directory: "src")
    !4 = distinct !DISubprogram(name: "myFunc", linkageName: "myFunc", scope: !5, file: !5, line: 2, type: !6, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !8)
    !5 = !DIFile(filename: "<internal>", directory: "")
    !6 = !DISubroutineType(flags: DIFlagPublic, types: !7)
    !7 = !{null}
    !8 = !{}
    !9 = !DILocalVariable(name: "a", scope: !4, file: !5, line: 4, type: !10, align [filtered])
    !10 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !11 = !DILocation(line: 4, column: 12, scope: !4)
    !12 = !DILocalVariable(name: "b", scope: !4, file: !5, line: 5, type: !13, align [filtered])
    !13 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REF_TO____myFunc_b", scope: !3, file: !3, baseType: !14, align [filtered])
    !14 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__myFunc_b", baseType: !10, size: 64, align [filtered], dwarfAddressSpace: 1)
    !15 = !DILocation(line: 5, column: 12, scope: !4)
    !16 = !DILocation(line: 7, column: 12, scope: !4)
    !17 = !DILocation(line: 8, column: 8, scope: !4)
    "#);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
}

#[test]
fn exit_statement_have_location() {
    // Let a function with an addition and a reference
    let result = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
            WHILE TRUE DO
                EXIT;
            END_WHILE
            myFunc := 1;
        END_FUNCTION
        ",
    );
    // No line information should be added on the statements
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
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
    filtered_assert_snapshot!(result);
}
