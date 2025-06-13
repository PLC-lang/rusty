use plc::DebugLevel;
use source_code::SourceCode;

use crate::tests::compile_with_root;
use plc_util::filtered_assert_snapshot;

#[test]
fn multiple_source_files_generated() {
    //Given 2 sources
    let src1 = SourceCode::new(
        "
    FUNCTION main : INT
    VAR_INPUT

    END_VAR

    VAR

    END_VAR
    mainProg();
    END_FUNCTION

    ",
        "external_file1.st",
    );
    let src2 = SourceCode::new(
        "
    PROGRAM mainProg
    VAR_TEMP
    END_VAR
    END_PROGRAM
    ",
        "external_file2.st",
    );
    //When the are generated
    let results = compile_with_root(vec![src1, src2], vec![], "root", DebugLevel::None).unwrap();
    assert_eq!(results.len(), 4);
    //The datatypes do not conflics
    //The functions are defined correctly
    filtered_assert_snapshot!(results.join("\n"));
}

#[test]
fn multiple_files_with_debug_info() {
    //Given 2 sources
    let src1: SourceCode = SourceCode::new(
        "
    FUNCTION main : INT
    VAR_INPUT

    END_VAR

    VAR

    END_VAR
    mainProg();
    END_FUNCTION

    ",
        "file1.st",
    );

    let src2: SourceCode = SourceCode::new(
        "
    PROGRAM mainProg
    VAR_TEMP
    END_VAR
    END_PROGRAM
    ",
        "file2.st",
    );
    //When the are generated
    let results =
        compile_with_root(vec![src1, src2], vec![], "root", DebugLevel::Full(plc::DEFAULT_DWARF_VERSION))
            .unwrap();
    assert_eq!(results.len(), 4);
    //The datatypes do not conflics
    //The functions are defined correctly
    filtered_assert_snapshot!(results.join("\n"));
}

#[test]
fn multiple_files_in_different_locations_with_debug_info() {
    //Given 2 sources
    let src1: SourceCode = SourceCode::new(
        "
    FUNCTION main : INT
    VAR_INPUT

    END_VAR

    VAR

    END_VAR
    mainProg();
    END_FUNCTION

    ",
        "app/file1.st",
    );

    let src2: SourceCode = SourceCode::new(
        "
    PROGRAM mainProg
    VAR_TEMP
    END_VAR
    END_PROGRAM
    ",
        "lib/file2.st",
    );
    //When the are generated
    let results =
        compile_with_root(vec![src1, src2], vec![], "root", DebugLevel::Full(plc::DEFAULT_DWARF_VERSION))
            .unwrap();
    assert_eq!(results.len(), 4);
    //The datatypes do not conflics
    //The functions are defined correctly
    filtered_assert_snapshot!(results.join("\n"));
}

#[test]
fn forward_declared_constant_is_also_marked_constant() {
    // GIVEN 2 sources, one with a forward declaration of a constant
    // and the other with the definition of that constant.
    let src1 = SourceCode::new(
        "
    FUNCTION main : INT
    VAR
        f: foo;
    END_VAR
        mainProg(f.something_to_initialize);
    END_FUNCTION

    ",
        "external_file1.st",
    );
    let src2 = SourceCode::new(
        "
    VAR_GLOBAL CONSTANT
        a: INT := 10;
    END_VAR

    PROGRAM mainProg
    VAR_INPUT
        a: INT;
    END_VAR
    END_PROGRAM

    FUNCTION_BLOCK foo
    VAR
        something_to_initialize: INT := 10 + a;
    END_VAR
    END_FUNCTION_BLOCK
    ",
        "external_file2.st",
    );

    // WHEN they are generated
    let results = compile_with_root(vec![src1, src2], vec![], "root", DebugLevel::Full(5)).unwrap();

    // THEN the constant is marked as constant in the generated code
    filtered_assert_snapshot!(results.join("\n"), @r###"
    ; ModuleID = 'external_file1.st'
    source_filename = "external_file1.st"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { i16 }
    %mainProg = type { i16 }

    @__foo__init = external unnamed_addr constant %foo, !dbg !0
    @mainProg_instance = external global %mainProg, !dbg !8

    define i16 @main() !dbg !18 {
    entry:
      %main = alloca i16, align 2
      %f = alloca %foo, align 8
      call void @llvm.dbg.declare(metadata %foo* %f, metadata !23, metadata !DIExpression()), !dbg !24
      %0 = bitcast %foo* %f to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%foo* @__foo__init to i8*), i64 ptrtoint (%foo* getelementptr (%foo, %foo* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata i16* %main, metadata !25, metadata !DIExpression()), !dbg !26
      store i16 0, i16* %main, align 2
      call void @__init_foo(%foo* %f), !dbg !27
      call void @__user_init_foo(%foo* %f), !dbg !27
      %something_to_initialize = getelementptr inbounds %foo, %foo* %f, i32 0, i32 0, !dbg !27
      %load_something_to_initialize = load i16, i16* %something_to_initialize, align 2, !dbg !27
      store i16 %load_something_to_initialize, i16* getelementptr inbounds (%mainProg, %mainProg* @mainProg_instance, i32 0, i32 0), align 2, !dbg !27
      call void @mainProg(%mainProg* @mainProg_instance), !dbg !28
      %main_ret = load i16, i16* %main, align 2, !dbg !29
      ret i16 %main_ret, !dbg !29
    }

    declare !dbg !30 void @foo(%foo*)

    declare void @__init_foo(%foo*)

    declare !dbg !33 void @__user_init_foo(%foo*)

    declare !dbg !38 void @mainProg(%mainProg*)

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn }

    !llvm.module.flags = !{!13, !14}
    !llvm.dbg.cu = !{!15}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 12, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "external_file2.st", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 12, size: 16, align: 64, flags: DIFlagPublic, elements: !5, identifier: "foo")
    !5 = !{!6}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "something_to_initialize", scope: !2, file: !2, line: 14, baseType: !7, size: 16, align: 16, flags: DIFlagPublic)
    !7 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !8 = !DIGlobalVariableExpression(var: !9, expr: !DIExpression())
    !9 = distinct !DIGlobalVariable(name: "mainProg", scope: !2, file: !2, line: 6, type: !10, isLocal: false, isDefinition: true)
    !10 = !DICompositeType(tag: DW_TAG_structure_type, name: "mainProg", scope: !2, file: !2, line: 6, size: 16, align: 64, flags: DIFlagPublic, elements: !11, identifier: "mainProg")
    !11 = !{!12}
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 8, baseType: !7, size: 16, align: 16, flags: DIFlagPublic)
    !13 = !{i32 2, !"Dwarf Version", i32 5}
    !14 = !{i32 2, !"Debug Info Version", i32 3}
    !15 = distinct !DICompileUnit(language: DW_LANG_C, file: !16, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !17, splitDebugInlining: false)
    !16 = !DIFile(filename: "external_file1.st", directory: "root")
    !17 = !{!0, !8}
    !18 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !19, file: !19, line: 2, type: !20, scopeLine: 2, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !15, retainedNodes: !22)
    !19 = !DIFile(filename: "external_file1.st", directory: "")
    !20 = !DISubroutineType(flags: DIFlagPublic, types: !21)
    !21 = !{null}
    !22 = !{}
    !23 = !DILocalVariable(name: "f", scope: !18, file: !19, line: 4, type: !4, align: 64)
    !24 = !DILocation(line: 4, column: 8, scope: !18)
    !25 = !DILocalVariable(name: "main", scope: !18, file: !19, line: 2, type: !7, align: 16)
    !26 = !DILocation(line: 2, column: 13, scope: !18)
    !27 = !DILocation(line: 0, scope: !18)
    !28 = !DILocation(line: 6, column: 8, scope: !18)
    !29 = !DILocation(line: 7, column: 4, scope: !18)
    !30 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 12, type: !31, scopeLine: 16, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !15, retainedNodes: !22)
    !31 = !DISubroutineType(flags: DIFlagPublic, types: !32)
    !32 = !{null, !4}
    !33 = distinct !DISubprogram(name: "__user_init_foo", linkageName: "__user_init_foo", scope: !34, file: !34, type: !35, scopeLine: 1, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !15, retainedNodes: !22)
    !34 = !DIFile(filename: "__initializers", directory: "")
    !35 = !DISubroutineType(flags: DIFlagPublic, types: !36)
    !36 = !{null, !37}
    !37 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_foo", baseType: !4, size: 64, align: 64, dwarfAddressSpace: 1)
    !38 = distinct !DISubprogram(name: "mainProg", linkageName: "mainProg", scope: !2, file: !2, line: 6, type: !39, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !15, retainedNodes: !22)
    !39 = !DISubroutineType(flags: DIFlagPublic, types: !40)
    !40 = !{null, !10, !7}

    ; ModuleID = 'external_file2.st'
    source_filename = "external_file2.st"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %mainProg = type { i16 }
    %foo = type { i16 }

    @a = unnamed_addr constant i16 10, !dbg !0
    @mainProg_instance = global %mainProg zeroinitializer, !dbg !5
    @__foo__init = unnamed_addr constant %foo { i16 20 }, !dbg !10

    define void @mainProg(%mainProg* %0) !dbg !21 {
    entry:
      call void @llvm.dbg.declare(metadata %mainProg* %0, metadata !25, metadata !DIExpression()), !dbg !26
      %a = getelementptr inbounds %mainProg, %mainProg* %0, i32 0, i32 0
      ret void, !dbg !26
    }

    define void @foo(%foo* %0) !dbg !27 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !30, metadata !DIExpression()), !dbg !31
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %something_to_initialize = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void, !dbg !31
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!16, !17}
    !llvm.dbg.cu = !{!18}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "a", scope: !2, file: !2, line: 3, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "external_file2.st", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !5 = !DIGlobalVariableExpression(var: !6, expr: !DIExpression())
    !6 = distinct !DIGlobalVariable(name: "mainProg", scope: !2, file: !2, line: 6, type: !7, isLocal: false, isDefinition: true)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "mainProg", scope: !2, file: !2, line: 6, size: 16, align: 64, flags: DIFlagPublic, elements: !8, identifier: "mainProg")
    !8 = !{!9}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 8, baseType: !4, size: 16, align: 16, flags: DIFlagPublic)
    !10 = !DIGlobalVariableExpression(var: !11, expr: !DIExpression())
    !11 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 12, type: !12, isLocal: false, isDefinition: true)
    !12 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !13)
    !13 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 12, size: 16, align: 64, flags: DIFlagPublic, elements: !14, identifier: "foo")
    !14 = !{!15}
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "something_to_initialize", scope: !2, file: !2, line: 14, baseType: !4, size: 16, align: 16, flags: DIFlagPublic)
    !16 = !{i32 2, !"Dwarf Version", i32 5}
    !17 = !{i32 2, !"Debug Info Version", i32 3}
    !18 = distinct !DICompileUnit(language: DW_LANG_C, file: !19, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !20, splitDebugInlining: false)
    !19 = !DIFile(filename: "external_file2.st", directory: "root")
    !20 = !{!0, !5, !10}
    !21 = distinct !DISubprogram(name: "mainProg", linkageName: "mainProg", scope: !2, file: !2, line: 6, type: !22, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !18, retainedNodes: !24)
    !22 = !DISubroutineType(flags: DIFlagPublic, types: !23)
    !23 = !{null, !7, !4}
    !24 = !{}
    !25 = !DILocalVariable(name: "mainProg", scope: !21, file: !2, line: 10, type: !7)
    !26 = !DILocation(line: 10, column: 4, scope: !21)
    !27 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 12, type: !28, scopeLine: 16, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !18, retainedNodes: !24)
    !28 = !DISubroutineType(flags: DIFlagPublic, types: !29)
    !29 = !{null, !13}
    !30 = !DILocalVariable(name: "foo", scope: !27, file: !2, line: 16, type: !13)
    !31 = !DILocation(line: 16, column: 4, scope: !27)

    ; ModuleID = '__initializers'
    source_filename = "__initializers"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %mainProg = type { i16 }
    %foo = type { i16 }

    @mainProg_instance = external global %mainProg
    @__foo__init = external unnamed_addr constant %foo

    define void @__init_mainprog(%mainProg* %0) {
    entry:
      %self = alloca %mainProg*, align 8
      store %mainProg* %0, %mainProg** %self, align 8
      ret void
    }

    declare void @mainProg(%mainProg*)

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    declare void @foo(%foo*)

    define void @__user_init_mainProg(%mainProg* %0) {
    entry:
      %self = alloca %mainProg*, align 8
      store %mainProg* %0, %mainProg** %self, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    ; ModuleID = '__init___TestProject'
    source_filename = "__init___TestProject"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %mainProg = type { i16 }

    @mainProg_instance = external global %mainProg
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___TestProject, i8* null }]

    define void @__init___TestProject() {
    entry:
      call void @__init_mainprog(%mainProg* @mainProg_instance)
      call void @__user_init_mainProg(%mainProg* @mainProg_instance)
      ret void
    }

    declare void @__init_mainprog(%mainProg*)

    declare void @mainProg(%mainProg*)

    declare void @__user_init_mainProg(%mainProg*)
    "###);
}
