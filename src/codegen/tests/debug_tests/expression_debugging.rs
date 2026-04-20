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
fn function_with_ctor_triggering_local_still_has_debug_info() {
    // Regression: initializer-lowering used to taint the enclosing function's
    // ImplementationIndexEntry.location with an internal FileMarker after injecting
    // `__<fn>_<var>__ctor` calls at the front of the body. That dropped !DISubprogram
    // and all !dbg metadata for the whole function. The body below has both a
    // REF_TO (ctor-triggering) and a plain statement so we can see both the
    // synthetic ctor call AND the user statement carry sensible debug info.
    let result = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
        VAR
            b : REF_TO DINT;
        END_VAR
            myFunc := 1;
        END_FUNCTION
        ",
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define i32 @myFunc() !dbg !4 {
    entry:
      %myFunc = alloca i32, align [filtered]
      %b = alloca ptr, align [filtered]
        #dbg_declare(ptr %b, !9, !DIExpression(), !13)
      store ptr null, ptr %b, align [filtered]
        #dbg_declare(ptr %myFunc, !14, !DIExpression(), !15)
      store i32 0, ptr %myFunc, align [filtered]
      store i32 1, ptr %myFunc, align [filtered], !dbg !16
      %myFunc_ret = load i32, ptr %myFunc, align [filtered], !dbg !17
      ret i32 %myFunc_ret, !dbg !17
    }

    !llvm.module.flags = !{!0, !1}
    !llvm.dbg.cu = !{!2}

    !0 = !{i32 2, !"Dwarf Version", i32 5}
    !1 = !{i32 2, !"Debug Info Version", i32 3}
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "<internal>", directory: "src")
    !4 = distinct !DISubprogram(name: "myFunc", linkageName: "myFunc", scope: !5, file: !5, line: 2, type: !6, scopeLine: 6, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !8)
    !5 = !DIFile(filename: "<internal>", directory: "")
    !6 = !DISubroutineType(flags: DIFlagPublic, types: !7)
    !7 = !{null}
    !8 = !{}
    !9 = !DILocalVariable(name: "b", scope: !4, file: !5, line: 4, type: !10, align [filtered])
    !10 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REF_TO____myFunc_b", scope: !3, file: !3, baseType: !11, align [filtered])
    !11 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__myFunc_b", baseType: !12, size: 64, align [filtered], dwarfAddressSpace: 1)
    !12 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !13 = !DILocation(line: 4, column: 12, scope: !4)
    !14 = !DILocalVariable(name: "myFunc", scope: !4, file: !5, line: 2, type: !12, align [filtered])
    !15 = !DILocation(line: 2, column: 17, scope: !4)
    !16 = !DILocation(line: 6, column: 12, scope: !4)
    !17 = !DILocation(line: 7, column: 8, scope: !4)
    "#);
}

#[test]
fn function_with_value_initialized_local_still_has_debug_info() {
    // Same regression as `function_with_ctor_triggering_local_still_has_debug_info`, but
    // triggered by a simple value-initialized scalar (`a : DINT := 5;`) which also produces
    // a synthetic `__myFunc_a__ctor` during initializer lowering.
    let result = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
        VAR
            a : DINT := 5;
        END_VAR
            myFunc := a;
        END_FUNCTION
        ",
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define i32 @myFunc() !dbg !4 {
    entry:
      %myFunc = alloca i32, align [filtered]
      %a = alloca i32, align [filtered]
        #dbg_declare(ptr %a, !9, !DIExpression(), !11)
      store i32 5, ptr %a, align [filtered]
        #dbg_declare(ptr %myFunc, !12, !DIExpression(), !13)
      store i32 0, ptr %myFunc, align [filtered]
      %load_a = load i32, ptr %a, align [filtered], !dbg !14
      store i32 %load_a, ptr %myFunc, align [filtered], !dbg !14
      %myFunc_ret = load i32, ptr %myFunc, align [filtered], !dbg !15
      ret i32 %myFunc_ret, !dbg !15
    }

    !llvm.module.flags = !{!0, !1}
    !llvm.dbg.cu = !{!2}

    !0 = !{i32 2, !"Dwarf Version", i32 5}
    !1 = !{i32 2, !"Debug Info Version", i32 3}
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "<internal>", directory: "src")
    !4 = distinct !DISubprogram(name: "myFunc", linkageName: "myFunc", scope: !5, file: !5, line: 2, type: !6, scopeLine: 6, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !8)
    !5 = !DIFile(filename: "<internal>", directory: "")
    !6 = !DISubroutineType(flags: DIFlagPublic, types: !7)
    !7 = !{null}
    !8 = !{}
    !9 = !DILocalVariable(name: "a", scope: !4, file: !5, line: 4, type: !10, align [filtered])
    !10 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !11 = !DILocation(line: 4, column: 12, scope: !4)
    !12 = !DILocalVariable(name: "myFunc", scope: !4, file: !5, line: 2, type: !10, align [filtered])
    !13 = !DILocation(line: 2, column: 17, scope: !4)
    !14 = !DILocation(line: 6, column: 12, scope: !4)
    !15 = !DILocation(line: 7, column: 8, scope: !4)
    "#);
}

#[test]
fn function_with_struct_local_still_has_debug_info() {
    // A struct local triggers a `<StructType>__ctor` call — verifies the fix also covers
    // the non-per-variable ctor case.
    let result = codegen_with_debug(
        "
        TYPE S : STRUCT x : DINT; END_STRUCT END_TYPE

        FUNCTION myFunc : DINT
        VAR
            s : S;
        END_VAR
            myFunc := s.x;
        END_FUNCTION
        ",
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %S = type { i32 }

    define i32 @myFunc() !dbg !4 {
    entry:
      %myFunc = alloca i32, align [filtered]
      %s = alloca %S, align [filtered]
        #dbg_declare(ptr %s, !9, !DIExpression(), !14)
      call void @llvm.memset.p0.i64(ptr align [filtered] %s, i8 0, i64 ptrtoint (ptr getelementptr (%S, ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %myFunc, !15, !DIExpression(), !16)
      store i32 0, ptr %myFunc, align [filtered]
      %x = getelementptr inbounds nuw %S, ptr %s, i32 0, i32 0, !dbg !17
      %load_x = load i32, ptr %x, align [filtered], !dbg !17
      store i32 %load_x, ptr %myFunc, align [filtered], !dbg !17
      %myFunc_ret = load i32, ptr %myFunc, align [filtered], !dbg !18
      ret i32 %myFunc_ret, !dbg !18
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }

    !llvm.module.flags = !{!0, !1}
    !llvm.dbg.cu = !{!2}

    !0 = !{i32 2, !"Dwarf Version", i32 5}
    !1 = !{i32 2, !"Debug Info Version", i32 3}
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "<internal>", directory: "src")
    !4 = distinct !DISubprogram(name: "myFunc", linkageName: "myFunc", scope: !5, file: !5, line: 4, type: !6, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !8)
    !5 = !DIFile(filename: "<internal>", directory: "")
    !6 = !DISubroutineType(flags: DIFlagPublic, types: !7)
    !7 = !{null}
    !8 = !{}
    !9 = !DILocalVariable(name: "s", scope: !4, file: !5, line: 6, type: !10, align [filtered])
    !10 = !DICompositeType(tag: DW_TAG_structure_type, name: "S", scope: !5, file: !5, line: 2, size: 32, align [filtered], flags: DIFlagPublic, elements: !11, identifier: "S")
    !11 = !{!12}
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !5, file: !5, line: 2, baseType: !13, size: 32, align [filtered], flags: DIFlagPublic)
    !13 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !14 = !DILocation(line: 6, column: 12, scope: !4)
    !15 = !DILocalVariable(name: "myFunc", scope: !4, file: !5, line: 4, type: !13, align [filtered])
    !16 = !DILocation(line: 4, column: 17, scope: !4)
    !17 = !DILocation(line: 8, column: 12, scope: !4)
    !18 = !DILocation(line: 9, column: 8, scope: !4)
    "#);
}

#[test]
fn function_with_initialized_array_local_still_has_debug_info() {
    // An initialized array triggers a `__myFunc_arr__ctor` call.
    let result = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
        VAR
            arr : ARRAY[0..2] OF DINT := [1, 2, 3];
        END_VAR
            myFunc := arr[0];
        END_FUNCTION
        ",
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @__myFunc.arr__init = unnamed_addr constant [3 x i32] [i32 1, i32 2, i32 3]

    define i32 @myFunc() !dbg !4 {
    entry:
      %myFunc = alloca i32, align [filtered]
      %arr = alloca [3 x i32], align [filtered]
        #dbg_declare(ptr %arr, !9, !DIExpression(), !14)
      call void @llvm.memcpy.p0.p0.i64(ptr align [filtered] %arr, ptr align [filtered] @__myFunc.arr__init, i64 ptrtoint (ptr getelementptr ([3 x i32], ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %myFunc, !15, !DIExpression(), !16)
      store i32 0, ptr %myFunc, align [filtered]
      %tmpVar = getelementptr inbounds [3 x i32], ptr %arr, i32 0, i32 0, !dbg !17
      %load_tmpVar = load i32, ptr %tmpVar, align [filtered], !dbg !17
      store i32 %load_tmpVar, ptr %myFunc, align [filtered], !dbg !17
      %myFunc_ret = load i32, ptr %myFunc, align [filtered], !dbg !18
      ret i32 %myFunc_ret, !dbg !18
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }

    !llvm.module.flags = !{!0, !1}
    !llvm.dbg.cu = !{!2}

    !0 = !{i32 2, !"Dwarf Version", i32 5}
    !1 = !{i32 2, !"Debug Info Version", i32 3}
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "<internal>", directory: "src")
    !4 = distinct !DISubprogram(name: "myFunc", linkageName: "myFunc", scope: !5, file: !5, line: 2, type: !6, scopeLine: 6, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !8)
    !5 = !DIFile(filename: "<internal>", directory: "")
    !6 = !DISubroutineType(flags: DIFlagPublic, types: !7)
    !7 = !{null}
    !8 = !{}
    !9 = !DILocalVariable(name: "arr", scope: !4, file: !5, line: 4, type: !10, align [filtered])
    !10 = !DICompositeType(tag: DW_TAG_array_type, baseType: !11, size: 96, align [filtered], elements: !12)
    !11 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !12 = !{!13}
    !13 = !DISubrange(count: 3, lowerBound: 0)
    !14 = !DILocation(line: 4, column: 12, scope: !4)
    !15 = !DILocalVariable(name: "myFunc", scope: !4, file: !5, line: 2, type: !11, align [filtered])
    !16 = !DILocation(line: 2, column: 17, scope: !4)
    !17 = !DILocation(line: 6, column: 12, scope: !4)
    !18 = !DILocation(line: 7, column: 8, scope: !4)
    "#);
}

#[test]
fn function_with_reference_to_initializer_still_has_debug_info() {
    // `REFERENCE TO T REF= x` is a declaration-level initializer (distinct from a runtime
    // REF= statement in the body). It also goes through initializer lowering and injects
    // a ctor call, so the enclosing function used to lose its debug info.
    let result = codegen_with_debug(
        "
        FUNCTION myFunc : DINT
        VAR
            a : DINT;
            b : REFERENCE TO DINT REF= a;
        END_VAR
            myFunc := b;
        END_FUNCTION
        ",
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define i32 @myFunc() !dbg !4 {
    entry:
      %myFunc = alloca i32, align [filtered]
      %a = alloca i32, align [filtered]
      %b = alloca ptr, align [filtered]
        #dbg_declare(ptr %a, !9, !DIExpression(), !11)
      store i32 0, ptr %a, align [filtered]
        #dbg_declare(ptr %b, !12, !DIExpression(), !15)
      store ptr null, ptr %b, align [filtered]
        #dbg_declare(ptr %myFunc, !16, !DIExpression(), !17)
      store i32 0, ptr %myFunc, align [filtered]
      %deref = load ptr, ptr %b, align [filtered], !dbg !18
      %load_b = load i32, ptr %deref, align [filtered], !dbg !18
      store i32 %load_b, ptr %myFunc, align [filtered], !dbg !18
      %myFunc_ret = load i32, ptr %myFunc, align [filtered], !dbg !19
      ret i32 %myFunc_ret, !dbg !19
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
    !13 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REFERENCE_TO____myFunc_b", scope: !3, file: !3, baseType: !14, align [filtered])
    !14 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__myFunc_b", baseType: !10, size: 64, align [filtered], dwarfAddressSpace: 1)
    !15 = !DILocation(line: 5, column: 12, scope: !4)
    !16 = !DILocalVariable(name: "myFunc", scope: !4, file: !5, line: 2, type: !10, align [filtered])
    !17 = !DILocation(line: 2, column: 17, scope: !4)
    !18 = !DILocation(line: 7, column: 12, scope: !4)
    !19 = !DILocation(line: 8, column: 8, scope: !4)
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
