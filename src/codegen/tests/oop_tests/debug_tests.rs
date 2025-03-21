use test_utils::codegen_with_debug as codegen;

#[test]
fn members_from_base_class_are_available_in_subclasses() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK foo
        VAR
            a : INT;
            b : STRING;
            c : ARRAY[0..10] OF STRING;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar EXTENDS foo
        END_FUNCTION_BLOCK
        "#,
    );
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { i16, [81 x i8], [11 x [81 x i8]] }
    %bar = type { %foo }

    @__foo__init = constant %foo zeroinitializer, !dbg !0
    @__bar__init = constant %bar zeroinitializer, !dbg !16
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @foo(%foo* %0) !dbg !25 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !29, metadata !DIExpression()), !dbg !30
      %a = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %b = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %c = getelementptr inbounds %foo, %foo* %0, i32 0, i32 2
      ret void, !dbg !30
    }

    define void @bar(%bar* %0) !dbg !31 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !34, metadata !DIExpression()), !dbg !35
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void, !dbg !35
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %__foo = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %__foo)
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!21, !22}
    !llvm.dbg.cu = !{!23}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 7792, flags: DIFlagPublic, elements: !4, identifier: "foo")
    !4 = !{!5, !7, !12}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 4, baseType: !6, size: 16, flags: DIFlagPublic)
    !6 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 5, baseType: !8, size: 648, offset: 16, flags: DIFlagPublic)
    !8 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 648, elements: !10)
    !9 = !DIBasicType(name: "char", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !10 = !{!11}
    !11 = !DISubrange(count: 81, lowerBound: 0)
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 6, baseType: !13, size: 7128, offset: 664, flags: DIFlagPublic)
    !13 = !DICompositeType(tag: DW_TAG_array_type, baseType: !8, size: 7128, elements: !14)
    !14 = !{!15}
    !15 = !DISubrange(count: 11, lowerBound: 0)
    !16 = !DIGlobalVariableExpression(var: !17, expr: !DIExpression())
    !17 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 10, type: !18, isLocal: false, isDefinition: true)
    !18 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 10, size: 7792, flags: DIFlagPublic, elements: !19, identifier: "bar")
    !19 = !{!20}
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !3, size: 7792, flags: DIFlagPublic)
    !21 = !{i32 2, !"Dwarf Version", i32 5}
    !22 = !{i32 2, !"Debug Info Version", i32 3}
    !23 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !24, splitDebugInlining: false)
    !24 = !{!0, !16}
    !25 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !26, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !23, retainedNodes: !28)
    !26 = !DISubroutineType(flags: DIFlagPublic, types: !27)
    !27 = !{null, !3}
    !28 = !{}
    !29 = !DILocalVariable(name: "foo", scope: !25, file: !2, line: 8, type: !3)
    !30 = !DILocation(line: 8, column: 8, scope: !25)
    !31 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 10, type: !32, scopeLine: 11, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !23, retainedNodes: !28)
    !32 = !DISubroutineType(flags: DIFlagPublic, types: !33)
    !33 = !{null, !18}
    !34 = !DILocalVariable(name: "bar", scope: !31, file: !2, line: 11, type: !18)
    !35 = !DILocation(line: 11, column: 8, scope: !31)
    "#);
}

#[test]
fn write_to_parent_variable_qualified_access() {
    let res = codegen(
        "
        FUNCTION_BLOCK fb
        VAR
            x : INT;
            y : INT;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb2 EXTENDS fb
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK foo
        VAR
            myFb : fb2;
        END_VAR
            myFb.x := 1;
        END_FUNCTION_BLOCK
       ",
    );

    insta::assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %fb2 = type { %fb }
    %fb = type { i16, i16 }
    %foo = type { %fb2 }

    @__fb2__init = constant %fb2 zeroinitializer, !dbg !0
    @__fb__init = constant %fb zeroinitializer, !dbg !11
    @__foo__init = constant %foo zeroinitializer, !dbg !13
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @fb(%fb* %0) !dbg !22 {
    entry:
      call void @llvm.dbg.declare(metadata %fb* %0, metadata !26, metadata !DIExpression()), !dbg !27
      %x = getelementptr inbounds %fb, %fb* %0, i32 0, i32 0
      %y = getelementptr inbounds %fb, %fb* %0, i32 0, i32 1
      ret void, !dbg !27
    }

    define void @fb2(%fb2* %0) !dbg !28 {
    entry:
      call void @llvm.dbg.declare(metadata %fb2* %0, metadata !31, metadata !DIExpression()), !dbg !32
      %__fb = getelementptr inbounds %fb2, %fb2* %0, i32 0, i32 0
      ret void, !dbg !32
    }

    define void @foo(%foo* %0) !dbg !33 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !36, metadata !DIExpression()), !dbg !37
      %myFb = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %__fb = getelementptr inbounds %fb2, %fb2* %myFb, i32 0, i32 0, !dbg !37
      %x = getelementptr inbounds %fb, %fb* %__fb, i32 0, i32 0, !dbg !37
      store i16 1, i16* %x, align 2, !dbg !37
      ret void, !dbg !38
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init_fb2(%fb2* %0) {
    entry:
      %self = alloca %fb2*, align 8
      store %fb2* %0, %fb2** %self, align 8
      %deref = load %fb2*, %fb2** %self, align 8
      %__fb = getelementptr inbounds %fb2, %fb2* %deref, i32 0, i32 0
      call void @__init_fb(%fb* %__fb)
      ret void
    }

    define void @__init_fb(%fb* %0) {
    entry:
      %self = alloca %fb*, align 8
      store %fb* %0, %fb** %self, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %myFb = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      call void @__init_fb2(%fb2* %myFb)
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!18, !19}
    !llvm.dbg.cu = !{!20}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__fb2__init", scope: !2, file: !2, line: 9, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb2", scope: !2, file: !2, line: 9, size: 32, flags: DIFlagPublic, elements: !4, identifier: "fb2")
    !4 = !{!5}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__fb", scope: !2, file: !2, baseType: !6, size: 32, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb", scope: !2, file: !2, line: 2, size: 32, flags: DIFlagPublic, elements: !7, identifier: "fb")
    !7 = !{!8, !10}
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 4, baseType: !9, size: 16, flags: DIFlagPublic)
    !9 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 5, baseType: !9, size: 16, offset: 16, flags: DIFlagPublic)
    !11 = !DIGlobalVariableExpression(var: !12, expr: !DIExpression())
    !12 = distinct !DIGlobalVariable(name: "__fb__init", scope: !2, file: !2, line: 2, type: !6, isLocal: false, isDefinition: true)
    !13 = !DIGlobalVariableExpression(var: !14, expr: !DIExpression())
    !14 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 12, type: !15, isLocal: false, isDefinition: true)
    !15 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 12, size: 32, flags: DIFlagPublic, elements: !16, identifier: "foo")
    !16 = !{!17}
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "myFb", scope: !2, file: !2, line: 14, baseType: !3, size: 32, flags: DIFlagPublic)
    !18 = !{i32 2, !"Dwarf Version", i32 5}
    !19 = !{i32 2, !"Debug Info Version", i32 3}
    !20 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !21, splitDebugInlining: false)
    !21 = !{!11, !0, !13}
    !22 = distinct !DISubprogram(name: "fb", linkageName: "fb", scope: !2, file: !2, line: 2, type: !23, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !20, retainedNodes: !25)
    !23 = !DISubroutineType(flags: DIFlagPublic, types: !24)
    !24 = !{null, !6}
    !25 = !{}
    !26 = !DILocalVariable(name: "fb", scope: !22, file: !2, line: 7, type: !6)
    !27 = !DILocation(line: 7, column: 8, scope: !22)
    !28 = distinct !DISubprogram(name: "fb2", linkageName: "fb2", scope: !2, file: !2, line: 9, type: !29, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !20, retainedNodes: !25)
    !29 = !DISubroutineType(flags: DIFlagPublic, types: !30)
    !30 = !{null, !3}
    !31 = !DILocalVariable(name: "fb2", scope: !28, file: !2, line: 10, type: !3)
    !32 = !DILocation(line: 10, column: 8, scope: !28)
    !33 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 12, type: !34, scopeLine: 16, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !20, retainedNodes: !25)
    !34 = !DISubroutineType(flags: DIFlagPublic, types: !35)
    !35 = !{null, !15}
    !36 = !DILocalVariable(name: "foo", scope: !33, file: !2, line: 16, type: !15)
    !37 = !DILocation(line: 16, column: 12, scope: !33)
    !38 = !DILocation(line: 17, column: 8, scope: !33)
    "#);
}

#[test]
fn write_to_parent_variable_in_instance() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK foo
        VAR
            s : STRING;
        END_VAR
        METHOD baz
            s := 'hello';
        END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar EXTENDS foo
            s := 'world';
        END_FUNCTION_BLOCK

        FUNCTION main
        VAR
            s: STRING;
            fb: bar;
        END_VAR
            fb.baz();
            fb();
        END_FUNCTION
    "#,
    );
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %bar = type { %foo }
    %foo = type { [81 x i8] }

    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"world\00"
    @__bar__init = constant %bar zeroinitializer, !dbg !0
    @__foo__init = constant %foo zeroinitializer, !dbg !13
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @foo(%foo* %0) !dbg !19 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !23, metadata !DIExpression()), !dbg !24
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void, !dbg !24
    }

    define void @foo.baz(%foo* %0) !dbg !25 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !26, metadata !DIExpression()), !dbg !27
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %1 = bitcast [81 x i8]* %s to i8*, !dbg !27
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_0, i32 0, i32 0), i32 6, i1 false), !dbg !27
      ret void, !dbg !28
    }

    define void @bar(%bar* %0) !dbg !29 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !32, metadata !DIExpression()), !dbg !33
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %__foo, i32 0, i32 0, !dbg !33
      %1 = bitcast [81 x i8]* %s to i8*, !dbg !33
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_1, i32 0, i32 0), i32 6, i1 false), !dbg !33
      ret void, !dbg !34
    }

    define void @main() !dbg !35 {
    entry:
      %s = alloca [81 x i8], align 1
      %fb = alloca %bar, align 8
      call void @llvm.dbg.declare(metadata [81 x i8]* %s, metadata !38, metadata !DIExpression()), !dbg !39
      %0 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %bar* %fb, metadata !40, metadata !DIExpression()), !dbg !41
      %1 = bitcast %bar* %fb to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 getelementptr inbounds (%bar, %bar* @__bar__init, i32 0, i32 0, i32 0, i32 0), i64 ptrtoint (%bar* getelementptr (%bar, %bar* null, i32 1) to i64), i1 false)
      call void @__init_bar(%bar* %fb), !dbg !42
      %__foo = getelementptr inbounds %bar, %bar* %fb, i32 0, i32 0, !dbg !42
      call void @foo.baz(%foo* %__foo), !dbg !43
      call void @bar(%bar* %fb), !dbg !44
      ret void, !dbg !45
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #1

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #2

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1

    define void @__init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %__foo = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %__foo)
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn }
    attributes #2 = { argmemonly nofree nounwind willreturn writeonly }

    !llvm.module.flags = !{!15, !16}
    !llvm.dbg.cu = !{!17}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 11, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 11, size: 648, flags: DIFlagPublic, elements: !4, identifier: "bar")
    !4 = !{!5}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !6, size: 648, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 648, flags: DIFlagPublic, elements: !7, identifier: "foo")
    !7 = !{!8}
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !2, file: !2, line: 4, baseType: !9, size: 648, flags: DIFlagPublic)
    !9 = !DICompositeType(tag: DW_TAG_array_type, baseType: !10, size: 648, elements: !11)
    !10 = !DIBasicType(name: "char", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !11 = !{!12}
    !12 = !DISubrange(count: 81, lowerBound: 0)
    !13 = !DIGlobalVariableExpression(var: !14, expr: !DIExpression())
    !14 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !6, isLocal: false, isDefinition: true)
    !15 = !{i32 2, !"Dwarf Version", i32 5}
    !16 = !{i32 2, !"Debug Info Version", i32 3}
    !17 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !18, splitDebugInlining: false)
    !18 = !{!13, !0}
    !19 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !20, scopeLine: 9, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !17, retainedNodes: !22)
    !20 = !DISubroutineType(flags: DIFlagPublic, types: !21)
    !21 = !{null, !6}
    !22 = !{}
    !23 = !DILocalVariable(name: "foo", scope: !19, file: !2, line: 9, type: !6)
    !24 = !DILocation(line: 9, column: 8, scope: !19)
    !25 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !19, file: !2, line: 6, type: !20, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !17, retainedNodes: !22)
    !26 = !DILocalVariable(name: "foo", scope: !25, file: !2, line: 7, type: !6)
    !27 = !DILocation(line: 7, column: 12, scope: !25)
    !28 = !DILocation(line: 8, column: 8, scope: !25)
    !29 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 11, type: !30, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !17, retainedNodes: !22)
    !30 = !DISubroutineType(flags: DIFlagPublic, types: !31)
    !31 = !{null, !3}
    !32 = !DILocalVariable(name: "bar", scope: !29, file: !2, line: 12, type: !3)
    !33 = !DILocation(line: 12, column: 12, scope: !29)
    !34 = !DILocation(line: 13, column: 8, scope: !29)
    !35 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 15, type: !36, scopeLine: 15, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !17, retainedNodes: !22)
    !36 = !DISubroutineType(flags: DIFlagPublic, types: !37)
    !37 = !{null}
    !38 = !DILocalVariable(name: "s", scope: !35, file: !2, line: 17, type: !9)
    !39 = !DILocation(line: 17, column: 12, scope: !35)
    !40 = !DILocalVariable(name: "fb", scope: !35, file: !2, line: 18, type: !3)
    !41 = !DILocation(line: 18, column: 12, scope: !35)
    !42 = !DILocation(line: 0, scope: !35)
    !43 = !DILocation(line: 20, column: 12, scope: !35)
    !44 = !DILocation(line: 21, column: 12, scope: !35)
    !45 = !DILocation(line: 22, column: 8, scope: !35)
    "#);
}

#[test]
fn array_in_parent_generated() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK grandparent
        VAR
            y : ARRAY[0..5] OF INT;
            a : INT;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent extends grandparent
            VAR
                x : ARRAY[0..10] OF INT;
                b : INT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                z : ARRAY[0..10] OF INT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION main
        VAR
            arr: ARRAY[0..10] OF child;
        END_VAR
            arr[0].a := 10;
            arr[0].y[0] := 20;
            arr[1].b := 30;
            arr[1].x[1] := 40;
            arr[2].z[2] := 50;
        END_FUNCTION
        "#,
    );
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %child = type { %parent, [11 x i16] }
    %parent = type { %grandparent, [11 x i16], i16 }
    %grandparent = type { [6 x i16], i16 }

    @__child__init = constant %child zeroinitializer, !dbg !0
    @__parent__init = constant %parent zeroinitializer, !dbg !23
    @__grandparent__init = constant %grandparent zeroinitializer, !dbg !25
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @grandparent(%grandparent* %0) !dbg !31 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !35, metadata !DIExpression()), !dbg !36
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      ret void, !dbg !36
    }

    define void @parent(%parent* %0) !dbg !37 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !40, metadata !DIExpression()), !dbg !41
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void, !dbg !41
    }

    define void @child(%child* %0) !dbg !42 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !45, metadata !DIExpression()), !dbg !46
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      ret void, !dbg !46
    }

    define void @main() !dbg !47 {
    entry:
      %arr = alloca [11 x %child], align 8
      call void @llvm.dbg.declare(metadata [11 x %child]* %arr, metadata !50, metadata !DIExpression()), !dbg !52
      %0 = bitcast [11 x %child]* %arr to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([11 x %child]* getelementptr ([11 x %child], [11 x %child]* null, i32 1) to i64), i1 false)
      %tmpVar = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !53
      %__parent = getelementptr inbounds %child, %child* %tmpVar, i32 0, i32 0, !dbg !53
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !53
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 1, !dbg !53
      store i16 10, i16* %a, align 2, !dbg !53
      %tmpVar1 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !54
      %__parent2 = getelementptr inbounds %child, %child* %tmpVar1, i32 0, i32 0, !dbg !54
      %__grandparent3 = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0, !dbg !54
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent3, i32 0, i32 0, !dbg !54
      %tmpVar4 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 0, !dbg !54
      store i16 20, i16* %tmpVar4, align 2, !dbg !54
      %tmpVar5 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !55
      %__parent6 = getelementptr inbounds %child, %child* %tmpVar5, i32 0, i32 0, !dbg !55
      %b = getelementptr inbounds %parent, %parent* %__parent6, i32 0, i32 2, !dbg !55
      store i16 30, i16* %b, align 2, !dbg !55
      %tmpVar7 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !56
      %__parent8 = getelementptr inbounds %child, %child* %tmpVar7, i32 0, i32 0, !dbg !56
      %x = getelementptr inbounds %parent, %parent* %__parent8, i32 0, i32 1, !dbg !56
      %tmpVar9 = getelementptr inbounds [11 x i16], [11 x i16]* %x, i32 0, i32 1, !dbg !56
      store i16 40, i16* %tmpVar9, align 2, !dbg !56
      %tmpVar10 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 2, !dbg !57
      %z = getelementptr inbounds %child, %child* %tmpVar10, i32 0, i32 1, !dbg !57
      %tmpVar11 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 2, !dbg !57
      store i16 50, i16* %tmpVar11, align 2, !dbg !57
      ret void, !dbg !58
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #1

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__init_grandparent(%grandparent* %__grandparent)
      ret void
    }

    define void @__init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn writeonly }

    !llvm.module.flags = !{!27, !28}
    !llvm.dbg.cu = !{!29}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 480, flags: DIFlagPublic, elements: !4, identifier: "child")
    !4 = !{!5, !22}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !6, size: 304, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 304, flags: DIFlagPublic, elements: !7, identifier: "parent")
    !7 = !{!8, !17, !21}
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "__grandparent", scope: !2, file: !2, baseType: !9, size: 112, flags: DIFlagPublic)
    !9 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 112, flags: DIFlagPublic, elements: !10, identifier: "grandparent")
    !10 = !{!11, !16}
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !12, size: 96, flags: DIFlagPublic)
    !12 = !DICompositeType(tag: DW_TAG_array_type, baseType: !13, size: 96, elements: !14)
    !13 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !14 = !{!15}
    !15 = !DISubrange(count: 6, lowerBound: 0)
    !16 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !13, size: 16, offset: 96, flags: DIFlagPublic)
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !18, size: 176, offset: 112, flags: DIFlagPublic)
    !18 = !DICompositeType(tag: DW_TAG_array_type, baseType: !13, size: 176, elements: !19)
    !19 = !{!20}
    !20 = !DISubrange(count: 11, lowerBound: 0)
    !21 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !13, size: 16, offset: 288, flags: DIFlagPublic)
    !22 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !18, size: 176, offset: 304, flags: DIFlagPublic)
    !23 = !DIGlobalVariableExpression(var: !24, expr: !DIExpression())
    !24 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !6, isLocal: false, isDefinition: true)
    !25 = !DIGlobalVariableExpression(var: !26, expr: !DIExpression())
    !26 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !9, isLocal: false, isDefinition: true)
    !27 = !{i32 2, !"Dwarf Version", i32 5}
    !28 = !{i32 2, !"Debug Info Version", i32 3}
    !29 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !30, splitDebugInlining: false)
    !30 = !{!25, !23, !0}
    !31 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !32, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !34)
    !32 = !DISubroutineType(flags: DIFlagPublic, types: !33)
    !33 = !{null, !9}
    !34 = !{}
    !35 = !DILocalVariable(name: "grandparent", scope: !31, file: !2, line: 7, type: !9)
    !36 = !DILocation(line: 7, column: 8, scope: !31)
    !37 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !38, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !34)
    !38 = !DISubroutineType(flags: DIFlagPublic, types: !39)
    !39 = !{null, !6}
    !40 = !DILocalVariable(name: "parent", scope: !37, file: !2, line: 14, type: !6)
    !41 = !DILocation(line: 14, column: 8, scope: !37)
    !42 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !43, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !34)
    !43 = !DISubroutineType(flags: DIFlagPublic, types: !44)
    !44 = !{null, !3}
    !45 = !DILocalVariable(name: "child", scope: !42, file: !2, line: 20, type: !3)
    !46 = !DILocation(line: 20, column: 8, scope: !42)
    !47 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 22, type: !48, scopeLine: 26, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !34)
    !48 = !DISubroutineType(flags: DIFlagPublic, types: !49)
    !49 = !{null}
    !50 = !DILocalVariable(name: "arr", scope: !47, file: !2, line: 24, type: !51)
    !51 = !DICompositeType(tag: DW_TAG_array_type, baseType: !3, size: 5280, elements: !19)
    !52 = !DILocation(line: 24, column: 12, scope: !47)
    !53 = !DILocation(line: 26, column: 12, scope: !47)
    !54 = !DILocation(line: 27, column: 12, scope: !47)
    !55 = !DILocation(line: 28, column: 12, scope: !47)
    !56 = !DILocation(line: 29, column: 12, scope: !47)
    !57 = !DILocation(line: 30, column: 12, scope: !47)
    !58 = !DILocation(line: 31, column: 8, scope: !47)
    "#);
}

#[test]
fn complex_array_access_generated() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK grandparent
        VAR
            y : ARRAY[0..5] OF INT;
            a : INT;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent extends grandparent
            VAR
                x : ARRAY[0..10] OF INT;
                b : INT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                z : ARRAY[0..10] OF INT;
            END_VAR
            y[b + z[b*2] - a] := 20;
        END_FUNCTION_BLOCK
        "#,
    );

    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %parent = type { %grandparent, [11 x i16], i16 }
    %grandparent = type { [6 x i16], i16 }
    %child = type { %parent, [11 x i16] }

    @__parent__init = constant %parent zeroinitializer, !dbg !0
    @__grandparent__init = constant %grandparent zeroinitializer, !dbg !19
    @__child__init = constant %child zeroinitializer, !dbg !21
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @grandparent(%grandparent* %0) !dbg !31 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !35, metadata !DIExpression()), !dbg !36
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      ret void, !dbg !36
    }

    define void @parent(%parent* %0) !dbg !37 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !40, metadata !DIExpression()), !dbg !41
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void, !dbg !41
    }

    define void @child(%child* %0) !dbg !42 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !45, metadata !DIExpression()), !dbg !46
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !46
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 0, !dbg !46
      %b = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2, !dbg !46
      %load_b = load i16, i16* %b, align 2, !dbg !46
      %1 = sext i16 %load_b to i32, !dbg !46
      %b1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2, !dbg !46
      %load_b2 = load i16, i16* %b1, align 2, !dbg !46
      %2 = sext i16 %load_b2 to i32, !dbg !46
      %tmpVar = mul i32 %2, 2, !dbg !46
      %tmpVar3 = mul i32 1, %tmpVar, !dbg !46
      %tmpVar4 = add i32 %tmpVar3, 0, !dbg !46
      %tmpVar5 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 %tmpVar4, !dbg !46
      %load_tmpVar = load i16, i16* %tmpVar5, align 2, !dbg !46
      %3 = sext i16 %load_tmpVar to i32, !dbg !46
      %tmpVar6 = add i32 %1, %3, !dbg !46
      %__grandparent7 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !46
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent7, i32 0, i32 1, !dbg !46
      %load_a = load i16, i16* %a, align 2, !dbg !46
      %4 = sext i16 %load_a to i32, !dbg !46
      %tmpVar8 = sub i32 %tmpVar6, %4, !dbg !46
      %tmpVar9 = mul i32 1, %tmpVar8, !dbg !46
      %tmpVar10 = add i32 %tmpVar9, 0, !dbg !46
      %tmpVar11 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 %tmpVar10, !dbg !46
      store i16 20, i16* %tmpVar11, align 2, !dbg !46
      ret void, !dbg !47
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__init_grandparent(%grandparent* %__grandparent)
      ret void
    }

    define void @__init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!27, !28}
    !llvm.dbg.cu = !{!29}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 304, flags: DIFlagPublic, elements: !4, identifier: "parent")
    !4 = !{!5, !14, !18}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__grandparent", scope: !2, file: !2, baseType: !6, size: 112, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 112, flags: DIFlagPublic, elements: !7, identifier: "grandparent")
    !7 = !{!8, !13}
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !9, size: 96, flags: DIFlagPublic)
    !9 = !DICompositeType(tag: DW_TAG_array_type, baseType: !10, size: 96, elements: !11)
    !10 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !11 = !{!12}
    !12 = !DISubrange(count: 6, lowerBound: 0)
    !13 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !10, size: 16, offset: 96, flags: DIFlagPublic)
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !15, size: 176, offset: 112, flags: DIFlagPublic)
    !15 = !DICompositeType(tag: DW_TAG_array_type, baseType: !10, size: 176, elements: !16)
    !16 = !{!17}
    !17 = !DISubrange(count: 11, lowerBound: 0)
    !18 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !10, size: 16, offset: 288, flags: DIFlagPublic)
    !19 = !DIGlobalVariableExpression(var: !20, expr: !DIExpression())
    !20 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !6, isLocal: false, isDefinition: true)
    !21 = !DIGlobalVariableExpression(var: !22, expr: !DIExpression())
    !22 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !23, isLocal: false, isDefinition: true)
    !23 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 480, flags: DIFlagPublic, elements: !24, identifier: "child")
    !24 = !{!25, !26}
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !3, size: 304, flags: DIFlagPublic)
    !26 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !15, size: 176, offset: 304, flags: DIFlagPublic)
    !27 = !{i32 2, !"Dwarf Version", i32 5}
    !28 = !{i32 2, !"Debug Info Version", i32 3}
    !29 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !30, splitDebugInlining: false)
    !30 = !{!19, !0, !21}
    !31 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !32, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !34)
    !32 = !DISubroutineType(flags: DIFlagPublic, types: !33)
    !33 = !{null, !6}
    !34 = !{}
    !35 = !DILocalVariable(name: "grandparent", scope: !31, file: !2, line: 7, type: !6)
    !36 = !DILocation(line: 7, column: 8, scope: !31)
    !37 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !38, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !34)
    !38 = !DISubroutineType(flags: DIFlagPublic, types: !39)
    !39 = !{null, !3}
    !40 = !DILocalVariable(name: "parent", scope: !37, file: !2, line: 14, type: !3)
    !41 = !DILocation(line: 14, column: 8, scope: !37)
    !42 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !43, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !34)
    !43 = !DISubroutineType(flags: DIFlagPublic, types: !44)
    !44 = !{null, !23}
    !45 = !DILocalVariable(name: "child", scope: !42, file: !2, line: 20, type: !23)
    !46 = !DILocation(line: 20, column: 12, scope: !42)
    !47 = !DILocation(line: 21, column: 8, scope: !42)
    "#);
}

#[test]
fn function_block_method_debug_info() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK foo
        METHOD baz
        END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar EXTENDS foo
        END_FUNCTION_BLOCK
    "#,
    );
    insta::assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type {}
    %bar = type { %foo }

    @__foo__init = constant %foo zeroinitializer, !dbg !0
    @__bar__init = constant %bar zeroinitializer, !dbg !5
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @foo(%foo* %0) !dbg !14 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !17, metadata !DIExpression()), !dbg !18
      ret void, !dbg !18
    }

    define void @foo.baz(%foo* %0) !dbg !19 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !20, metadata !DIExpression()), !dbg !21
      ret void, !dbg !21
    }

    define void @bar(%bar* %0) !dbg !22 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !25, metadata !DIExpression()), !dbg !26
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void, !dbg !26
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %__foo = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %__foo)
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!10, !11}
    !llvm.dbg.cu = !{!12}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, flags: DIFlagPublic, elements: !4, identifier: "foo")
    !4 = !{}
    !5 = !DIGlobalVariableExpression(var: !6, expr: !DIExpression())
    !6 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 7, type: !7, isLocal: false, isDefinition: true)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 7, flags: DIFlagPublic, elements: !8, identifier: "bar")
    !8 = !{!9}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !3, flags: DIFlagPublic)
    !10 = !{i32 2, !"Dwarf Version", i32 5}
    !11 = !{i32 2, !"Debug Info Version", i32 3}
    !12 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !13, splitDebugInlining: false)
    !13 = !{!0, !5}
    !14 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !15, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !12, retainedNodes: !4)
    !15 = !DISubroutineType(flags: DIFlagPublic, types: !16)
    !16 = !{null, !3}
    !17 = !DILocalVariable(name: "foo", scope: !14, file: !2, line: 5, type: !3)
    !18 = !DILocation(line: 5, column: 8, scope: !14)
    !19 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !14, file: !2, line: 3, type: !15, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !12, retainedNodes: !4)
    !20 = !DILocalVariable(name: "foo", scope: !19, file: !2, line: 4, type: !3)
    !21 = !DILocation(line: 4, column: 8, scope: !19)
    !22 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 7, type: !23, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !12, retainedNodes: !4)
    !23 = !DISubroutineType(flags: DIFlagPublic, types: !24)
    !24 = !{null, !7}
    !25 = !DILocalVariable(name: "bar", scope: !22, file: !2, line: 8, type: !7)
    !26 = !DILocation(line: 8, column: 8, scope: !22)
    "###);
}
