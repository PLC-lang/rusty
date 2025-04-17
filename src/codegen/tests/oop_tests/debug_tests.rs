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

    %__vtable_foo = type {}
    %__vtable_bar = type { %__vtable_foo }
    %foo = type { i16, [81 x i8], [11 x [81 x i8]] }
    %bar = type { %foo }

    @____vtable_foo__init = constant %__vtable_foo zeroinitializer, !dbg !0
    @____vtable_bar__init = constant %__vtable_bar zeroinitializer, !dbg !5
    @__foo__init = constant %foo zeroinitializer, !dbg !10
    @__bar__init = constant %bar zeroinitializer, !dbg !25
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @foo(%foo* %0) !dbg !34 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !37, metadata !DIExpression()), !dbg !38
      %a = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %b = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %c = getelementptr inbounds %foo, %foo* %0, i32 0, i32 2
      ret void, !dbg !38
    }

    define void @bar(%bar* %0) !dbg !39 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !42, metadata !DIExpression()), !dbg !43
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void, !dbg !43
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init___vtable_foo(%__vtable_foo* %0) {
    entry:
      %self = alloca %__vtable_foo*, align 8
      store %__vtable_foo* %0, %__vtable_foo** %self, align 8
      ret void
    }

    define void @__init___vtable_bar(%__vtable_bar* %0) {
    entry:
      %self = alloca %__vtable_bar*, align 8
      store %__vtable_bar* %0, %__vtable_bar** %self, align 8
      %deref = load %__vtable_bar*, %__vtable_bar** %self, align 8
      %__vtable_foo = getelementptr inbounds %__vtable_bar, %__vtable_bar* %deref, i32 0, i32 0
      call void @__init___vtable_foo(%__vtable_foo* %__vtable_foo)
      ret void
    }

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

    !llvm.module.flags = !{!30, !31}
    !llvm.dbg.cu = !{!32}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "____vtable_foo__init", scope: !2, file: !2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo", scope: !2, file: !2, flags: DIFlagPublic, elements: !4, identifier: "__vtable_foo")
    !4 = !{}
    !5 = !DIGlobalVariableExpression(var: !6, expr: !DIExpression())
    !6 = distinct !DIGlobalVariable(name: "____vtable_bar__init", scope: !2, file: !2, type: !7, isLocal: false, isDefinition: true)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_bar", scope: !2, file: !2, flags: DIFlagPublic, elements: !8, identifier: "__vtable_bar")
    !8 = !{!9}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_foo", scope: !2, file: !2, baseType: !3, flags: DIFlagPublic)
    !10 = !DIGlobalVariableExpression(var: !11, expr: !DIExpression())
    !11 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !12, isLocal: false, isDefinition: true)
    !12 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 7792, flags: DIFlagPublic, elements: !13, identifier: "foo")
    !13 = !{!14, !16, !21}
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 4, baseType: !15, size: 16, flags: DIFlagPublic)
    !15 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !16 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 5, baseType: !17, size: 648, offset: 16, flags: DIFlagPublic)
    !17 = !DICompositeType(tag: DW_TAG_array_type, baseType: !18, size: 648, elements: !19)
    !18 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !19 = !{!20}
    !20 = !DISubrange(count: 81, lowerBound: 0)
    !21 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 6, baseType: !22, size: 7128, offset: 664, flags: DIFlagPublic)
    !22 = !DICompositeType(tag: DW_TAG_array_type, baseType: !17, size: 7128, elements: !23)
    !23 = !{!24}
    !24 = !DISubrange(count: 11, lowerBound: 0)
    !25 = !DIGlobalVariableExpression(var: !26, expr: !DIExpression())
    !26 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 10, type: !27, isLocal: false, isDefinition: true)
    !27 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 10, size: 7792, flags: DIFlagPublic, elements: !28, identifier: "bar")
    !28 = !{!29}
    !29 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !12, size: 7792, flags: DIFlagPublic)
    !30 = !{i32 2, !"Dwarf Version", i32 5}
    !31 = !{i32 2, !"Debug Info Version", i32 3}
    !32 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !33, splitDebugInlining: false)
    !33 = !{!10, !25, !0, !5}
    !34 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !35, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !32, retainedNodes: !4)
    !35 = !DISubroutineType(flags: DIFlagPublic, types: !36)
    !36 = !{null, !12}
    !37 = !DILocalVariable(name: "foo", scope: !34, file: !2, line: 8, type: !12)
    !38 = !DILocation(line: 8, column: 8, scope: !34)
    !39 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 10, type: !40, scopeLine: 11, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !32, retainedNodes: !4)
    !40 = !DISubroutineType(flags: DIFlagPublic, types: !41)
    !41 = !{null, !27}
    !42 = !DILocalVariable(name: "bar", scope: !39, file: !2, line: 11, type: !27)
    !43 = !DILocation(line: 11, column: 8, scope: !39)
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

    %__vtable_fb = type {}
    %__vtable_fb2 = type { %__vtable_fb }
    %__vtable_foo = type {}
    %fb2 = type { %fb }
    %fb = type { i16, i16 }
    %foo = type { %fb2 }

    @____vtable_fb__init = constant %__vtable_fb zeroinitializer, !dbg !0
    @____vtable_fb2__init = constant %__vtable_fb2 zeroinitializer, !dbg !5
    @____vtable_foo__init = constant %__vtable_foo zeroinitializer, !dbg !10
    @__fb2__init = constant %fb2 zeroinitializer, !dbg !13
    @__fb__init = constant %fb zeroinitializer, !dbg !23
    @__foo__init = constant %foo zeroinitializer, !dbg !25
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @fb(%fb* %0) !dbg !34 {
    entry:
      call void @llvm.dbg.declare(metadata %fb* %0, metadata !37, metadata !DIExpression()), !dbg !38
      %x = getelementptr inbounds %fb, %fb* %0, i32 0, i32 0
      %y = getelementptr inbounds %fb, %fb* %0, i32 0, i32 1
      ret void, !dbg !38
    }

    define void @fb2(%fb2* %0) !dbg !39 {
    entry:
      call void @llvm.dbg.declare(metadata %fb2* %0, metadata !42, metadata !DIExpression()), !dbg !43
      %__fb = getelementptr inbounds %fb2, %fb2* %0, i32 0, i32 0
      ret void, !dbg !43
    }

    define void @foo(%foo* %0) !dbg !44 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !47, metadata !DIExpression()), !dbg !48
      %myFb = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %__fb = getelementptr inbounds %fb2, %fb2* %myFb, i32 0, i32 0, !dbg !48
      %x = getelementptr inbounds %fb, %fb* %__fb, i32 0, i32 0, !dbg !48
      store i16 1, i16* %x, align 2, !dbg !48
      ret void, !dbg !49
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init___vtable_fb(%__vtable_fb* %0) {
    entry:
      %self = alloca %__vtable_fb*, align 8
      store %__vtable_fb* %0, %__vtable_fb** %self, align 8
      ret void
    }

    define void @__init___vtable_fb2(%__vtable_fb2* %0) {
    entry:
      %self = alloca %__vtable_fb2*, align 8
      store %__vtable_fb2* %0, %__vtable_fb2** %self, align 8
      %deref = load %__vtable_fb2*, %__vtable_fb2** %self, align 8
      %__vtable_fb = getelementptr inbounds %__vtable_fb2, %__vtable_fb2* %deref, i32 0, i32 0
      call void @__init___vtable_fb(%__vtable_fb* %__vtable_fb)
      ret void
    }

    define void @__init___vtable_foo(%__vtable_foo* %0) {
    entry:
      %self = alloca %__vtable_foo*, align 8
      store %__vtable_foo* %0, %__vtable_foo** %self, align 8
      ret void
    }

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

    !llvm.module.flags = !{!30, !31}
    !llvm.dbg.cu = !{!32}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "____vtable_fb__init", scope: !2, file: !2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_fb", scope: !2, file: !2, flags: DIFlagPublic, elements: !4, identifier: "__vtable_fb")
    !4 = !{}
    !5 = !DIGlobalVariableExpression(var: !6, expr: !DIExpression())
    !6 = distinct !DIGlobalVariable(name: "____vtable_fb2__init", scope: !2, file: !2, type: !7, isLocal: false, isDefinition: true)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_fb2", scope: !2, file: !2, flags: DIFlagPublic, elements: !8, identifier: "__vtable_fb2")
    !8 = !{!9}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_fb", scope: !2, file: !2, baseType: !3, flags: DIFlagPublic)
    !10 = !DIGlobalVariableExpression(var: !11, expr: !DIExpression())
    !11 = distinct !DIGlobalVariable(name: "____vtable_foo__init", scope: !2, file: !2, type: !12, isLocal: false, isDefinition: true)
    !12 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo", scope: !2, file: !2, flags: DIFlagPublic, elements: !4, identifier: "__vtable_foo")
    !13 = !DIGlobalVariableExpression(var: !14, expr: !DIExpression())
    !14 = distinct !DIGlobalVariable(name: "__fb2__init", scope: !2, file: !2, line: 9, type: !15, isLocal: false, isDefinition: true)
    !15 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb2", scope: !2, file: !2, line: 9, size: 32, flags: DIFlagPublic, elements: !16, identifier: "fb2")
    !16 = !{!17}
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "__fb", scope: !2, file: !2, baseType: !18, size: 32, flags: DIFlagPublic)
    !18 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb", scope: !2, file: !2, line: 2, size: 32, flags: DIFlagPublic, elements: !19, identifier: "fb")
    !19 = !{!20, !22}
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 4, baseType: !21, size: 16, flags: DIFlagPublic)
    !21 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !22 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 5, baseType: !21, size: 16, offset: 16, flags: DIFlagPublic)
    !23 = !DIGlobalVariableExpression(var: !24, expr: !DIExpression())
    !24 = distinct !DIGlobalVariable(name: "__fb__init", scope: !2, file: !2, line: 2, type: !18, isLocal: false, isDefinition: true)
    !25 = !DIGlobalVariableExpression(var: !26, expr: !DIExpression())
    !26 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 12, type: !27, isLocal: false, isDefinition: true)
    !27 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 12, size: 32, flags: DIFlagPublic, elements: !28, identifier: "foo")
    !28 = !{!29}
    !29 = !DIDerivedType(tag: DW_TAG_member, name: "myFb", scope: !2, file: !2, line: 14, baseType: !15, size: 32, flags: DIFlagPublic)
    !30 = !{i32 2, !"Dwarf Version", i32 5}
    !31 = !{i32 2, !"Debug Info Version", i32 3}
    !32 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !33, splitDebugInlining: false)
    !33 = !{!23, !13, !25, !0, !5, !10}
    !34 = distinct !DISubprogram(name: "fb", linkageName: "fb", scope: !2, file: !2, line: 2, type: !35, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !32, retainedNodes: !4)
    !35 = !DISubroutineType(flags: DIFlagPublic, types: !36)
    !36 = !{null, !18}
    !37 = !DILocalVariable(name: "fb", scope: !34, file: !2, line: 7, type: !18)
    !38 = !DILocation(line: 7, column: 8, scope: !34)
    !39 = distinct !DISubprogram(name: "fb2", linkageName: "fb2", scope: !2, file: !2, line: 9, type: !40, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !32, retainedNodes: !4)
    !40 = !DISubroutineType(flags: DIFlagPublic, types: !41)
    !41 = !{null, !15}
    !42 = !DILocalVariable(name: "fb2", scope: !39, file: !2, line: 10, type: !15)
    !43 = !DILocation(line: 10, column: 8, scope: !39)
    !44 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 12, type: !45, scopeLine: 16, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !32, retainedNodes: !4)
    !45 = !DISubroutineType(flags: DIFlagPublic, types: !46)
    !46 = !{null, !27}
    !47 = !DILocalVariable(name: "foo", scope: !44, file: !2, line: 16, type: !27)
    !48 = !DILocation(line: 16, column: 12, scope: !44)
    !49 = !DILocation(line: 17, column: 8, scope: !44)
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

    %__vtable_foo = type { i32* }
    %__vtable_bar = type { %__vtable_foo }
    %bar = type { %foo }
    %foo = type { [81 x i8] }

    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"world\00"
    @____vtable_foo__init = constant %__vtable_foo zeroinitializer, !dbg !0
    @____vtable_bar__init = constant %__vtable_bar zeroinitializer, !dbg !8
    @__bar__init = constant %bar zeroinitializer, !dbg !13
    @__foo__init = constant %foo zeroinitializer, !dbg !25
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @foo(%foo* %0) !dbg !31 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !35, metadata !DIExpression()), !dbg !36
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void, !dbg !36
    }

    define void @foo_baz(%foo* %0) !dbg !37 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !38, metadata !DIExpression()), !dbg !39
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %1 = bitcast [81 x i8]* %s to i8*, !dbg !39
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_0, i32 0, i32 0), i32 6, i1 false), !dbg !39
      ret void, !dbg !40
    }

    define void @bar(%bar* %0) !dbg !41 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !44, metadata !DIExpression()), !dbg !45
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %__foo, i32 0, i32 0, !dbg !45
      %1 = bitcast [81 x i8]* %s to i8*, !dbg !45
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_1, i32 0, i32 0), i32 6, i1 false), !dbg !45
      ret void, !dbg !46
    }

    define void @main() !dbg !47 {
    entry:
      %s = alloca [81 x i8], align 1
      %fb = alloca %bar, align 8
      call void @llvm.dbg.declare(metadata [81 x i8]* %s, metadata !50, metadata !DIExpression()), !dbg !51
      %0 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %bar* %fb, metadata !52, metadata !DIExpression()), !dbg !53
      %1 = bitcast %bar* %fb to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 getelementptr inbounds (%bar, %bar* @__bar__init, i32 0, i32 0, i32 0, i32 0), i64 ptrtoint (%bar* getelementptr (%bar, %bar* null, i32 1) to i64), i1 false)
      call void @__init_bar(%bar* %fb), !dbg !54
      %__foo = getelementptr inbounds %bar, %bar* %fb, i32 0, i32 0, !dbg !54
      call void @foo_baz(%foo* %__foo), !dbg !55
      call void @bar(%bar* %fb), !dbg !56
      ret void, !dbg !57
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #1

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #2

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1

    define void @__init___vtable_foo(%__vtable_foo* %0) {
    entry:
      %self = alloca %__vtable_foo*, align 8
      store %__vtable_foo* %0, %__vtable_foo** %self, align 8
      ret void
    }

    define void @__init___vtable_bar(%__vtable_bar* %0) {
    entry:
      %self = alloca %__vtable_bar*, align 8
      store %__vtable_bar* %0, %__vtable_bar** %self, align 8
      %deref = load %__vtable_bar*, %__vtable_bar** %self, align 8
      %__vtable_foo = getelementptr inbounds %__vtable_bar, %__vtable_bar* %deref, i32 0, i32 0
      call void @__init___vtable_foo(%__vtable_foo* %__vtable_foo)
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

    !llvm.module.flags = !{!27, !28}
    !llvm.dbg.cu = !{!29}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "____vtable_foo__init", scope: !2, file: !2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo", scope: !2, file: !2, size: 64, flags: DIFlagPublic, elements: !4, identifier: "__vtable_foo")
    !4 = !{!5}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "foo.baz", scope: !2, file: !2, baseType: !6, size: 64, flags: DIFlagPublic)
    !6 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "VOID_POINTER", baseType: !7, size: 64, dwarfAddressSpace: 1)
    !7 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !8 = !DIGlobalVariableExpression(var: !9, expr: !DIExpression())
    !9 = distinct !DIGlobalVariable(name: "____vtable_bar__init", scope: !2, file: !2, type: !10, isLocal: false, isDefinition: true)
    !10 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_bar", scope: !2, file: !2, size: 64, flags: DIFlagPublic, elements: !11, identifier: "__vtable_bar")
    !11 = !{!12}
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_foo", scope: !2, file: !2, baseType: !3, size: 64, flags: DIFlagPublic)
    !13 = !DIGlobalVariableExpression(var: !14, expr: !DIExpression())
    !14 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 11, type: !15, isLocal: false, isDefinition: true)
    !15 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 11, size: 648, flags: DIFlagPublic, elements: !16, identifier: "bar")
    !16 = !{!17}
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !18, size: 648, flags: DIFlagPublic)
    !18 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 648, flags: DIFlagPublic, elements: !19, identifier: "foo")
    !19 = !{!20}
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !2, file: !2, line: 4, baseType: !21, size: 648, flags: DIFlagPublic)
    !21 = !DICompositeType(tag: DW_TAG_array_type, baseType: !22, size: 648, elements: !23)
    !22 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !23 = !{!24}
    !24 = !DISubrange(count: 81, lowerBound: 0)
    !25 = !DIGlobalVariableExpression(var: !26, expr: !DIExpression())
    !26 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !18, isLocal: false, isDefinition: true)
    !27 = !{i32 2, !"Dwarf Version", i32 5}
    !28 = !{i32 2, !"Debug Info Version", i32 3}
    !29 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !30, splitDebugInlining: false)
    !30 = !{!25, !13, !0, !8}
    !31 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !32, scopeLine: 9, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !34)
    !32 = !DISubroutineType(flags: DIFlagPublic, types: !33)
    !33 = !{null, !18}
    !34 = !{}
    !35 = !DILocalVariable(name: "foo", scope: !31, file: !2, line: 9, type: !18)
    !36 = !DILocation(line: 9, column: 8, scope: !31)
    !37 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !31, file: !2, line: 6, type: !32, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !34)
    !38 = !DILocalVariable(name: "foo", scope: !37, file: !2, line: 7, type: !18)
    !39 = !DILocation(line: 7, column: 12, scope: !37)
    !40 = !DILocation(line: 8, column: 8, scope: !37)
    !41 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 11, type: !42, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !34)
    !42 = !DISubroutineType(flags: DIFlagPublic, types: !43)
    !43 = !{null, !15}
    !44 = !DILocalVariable(name: "bar", scope: !41, file: !2, line: 12, type: !15)
    !45 = !DILocation(line: 12, column: 12, scope: !41)
    !46 = !DILocation(line: 13, column: 8, scope: !41)
    !47 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 15, type: !48, scopeLine: 15, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !34)
    !48 = !DISubroutineType(flags: DIFlagPublic, types: !49)
    !49 = !{null}
    !50 = !DILocalVariable(name: "s", scope: !47, file: !2, line: 17, type: !21)
    !51 = !DILocation(line: 17, column: 12, scope: !47)
    !52 = !DILocalVariable(name: "fb", scope: !47, file: !2, line: 18, type: !15)
    !53 = !DILocation(line: 18, column: 12, scope: !47)
    !54 = !DILocation(line: 0, scope: !47)
    !55 = !DILocation(line: 20, column: 12, scope: !47)
    !56 = !DILocation(line: 21, column: 12, scope: !47)
    !57 = !DILocation(line: 22, column: 8, scope: !47)
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

    %__vtable_grandparent = type {}
    %__vtable_parent = type { %__vtable_grandparent }
    %__vtable_child = type { %__vtable_parent }
    %child = type { %parent, [11 x i16] }
    %parent = type { %grandparent, [11 x i16], i16 }
    %grandparent = type { [6 x i16], i16 }

    @____vtable_grandparent__init = constant %__vtable_grandparent zeroinitializer, !dbg !0
    @____vtable_parent__init = constant %__vtable_parent zeroinitializer, !dbg !5
    @____vtable_child__init = constant %__vtable_child zeroinitializer, !dbg !10
    @__child__init = constant %child zeroinitializer, !dbg !15
    @__parent__init = constant %parent zeroinitializer, !dbg !37
    @__grandparent__init = constant %grandparent zeroinitializer, !dbg !39
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @grandparent(%grandparent* %0) !dbg !45 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !48, metadata !DIExpression()), !dbg !49
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      ret void, !dbg !49
    }

    define void @parent(%parent* %0) !dbg !50 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !53, metadata !DIExpression()), !dbg !54
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void, !dbg !54
    }

    define void @child(%child* %0) !dbg !55 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !58, metadata !DIExpression()), !dbg !59
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      ret void, !dbg !59
    }

    define void @main() !dbg !60 {
    entry:
      %arr = alloca [11 x %child], align 8
      call void @llvm.dbg.declare(metadata [11 x %child]* %arr, metadata !63, metadata !DIExpression()), !dbg !65
      %0 = bitcast [11 x %child]* %arr to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([11 x %child]* getelementptr ([11 x %child], [11 x %child]* null, i32 1) to i64), i1 false)
      %tmpVar = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !66
      %__parent = getelementptr inbounds %child, %child* %tmpVar, i32 0, i32 0, !dbg !66
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !66
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 1, !dbg !66
      store i16 10, i16* %a, align 2, !dbg !66
      %tmpVar1 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !67
      %__parent2 = getelementptr inbounds %child, %child* %tmpVar1, i32 0, i32 0, !dbg !67
      %__grandparent3 = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0, !dbg !67
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent3, i32 0, i32 0, !dbg !67
      %tmpVar4 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 0, !dbg !67
      store i16 20, i16* %tmpVar4, align 2, !dbg !67
      %tmpVar5 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !68
      %__parent6 = getelementptr inbounds %child, %child* %tmpVar5, i32 0, i32 0, !dbg !68
      %b = getelementptr inbounds %parent, %parent* %__parent6, i32 0, i32 2, !dbg !68
      store i16 30, i16* %b, align 2, !dbg !68
      %tmpVar7 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !69
      %__parent8 = getelementptr inbounds %child, %child* %tmpVar7, i32 0, i32 0, !dbg !69
      %x = getelementptr inbounds %parent, %parent* %__parent8, i32 0, i32 1, !dbg !69
      %tmpVar9 = getelementptr inbounds [11 x i16], [11 x i16]* %x, i32 0, i32 1, !dbg !69
      store i16 40, i16* %tmpVar9, align 2, !dbg !69
      %tmpVar10 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 2, !dbg !70
      %z = getelementptr inbounds %child, %child* %tmpVar10, i32 0, i32 1, !dbg !70
      %tmpVar11 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 2, !dbg !70
      store i16 50, i16* %tmpVar11, align 2, !dbg !70
      ret void, !dbg !71
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #1

    define void @__init___vtable_grandparent(%__vtable_grandparent* %0) {
    entry:
      %self = alloca %__vtable_grandparent*, align 8
      store %__vtable_grandparent* %0, %__vtable_grandparent** %self, align 8
      ret void
    }

    define void @__init___vtable_parent(%__vtable_parent* %0) {
    entry:
      %self = alloca %__vtable_parent*, align 8
      store %__vtable_parent* %0, %__vtable_parent** %self, align 8
      %deref = load %__vtable_parent*, %__vtable_parent** %self, align 8
      %__vtable_grandparent = getelementptr inbounds %__vtable_parent, %__vtable_parent* %deref, i32 0, i32 0
      call void @__init___vtable_grandparent(%__vtable_grandparent* %__vtable_grandparent)
      ret void
    }

    define void @__init___vtable_child(%__vtable_child* %0) {
    entry:
      %self = alloca %__vtable_child*, align 8
      store %__vtable_child* %0, %__vtable_child** %self, align 8
      %deref = load %__vtable_child*, %__vtable_child** %self, align 8
      %__vtable_parent = getelementptr inbounds %__vtable_child, %__vtable_child* %deref, i32 0, i32 0
      call void @__init___vtable_parent(%__vtable_parent* %__vtable_parent)
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

    !llvm.module.flags = !{!41, !42}
    !llvm.dbg.cu = !{!43}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "____vtable_grandparent__init", scope: !2, file: !2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_grandparent", scope: !2, file: !2, flags: DIFlagPublic, elements: !4, identifier: "__vtable_grandparent")
    !4 = !{}
    !5 = !DIGlobalVariableExpression(var: !6, expr: !DIExpression())
    !6 = distinct !DIGlobalVariable(name: "____vtable_parent__init", scope: !2, file: !2, type: !7, isLocal: false, isDefinition: true)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_parent", scope: !2, file: !2, flags: DIFlagPublic, elements: !8, identifier: "__vtable_parent")
    !8 = !{!9}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_grandparent", scope: !2, file: !2, baseType: !3, flags: DIFlagPublic)
    !10 = !DIGlobalVariableExpression(var: !11, expr: !DIExpression())
    !11 = distinct !DIGlobalVariable(name: "____vtable_child__init", scope: !2, file: !2, type: !12, isLocal: false, isDefinition: true)
    !12 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_child", scope: !2, file: !2, flags: DIFlagPublic, elements: !13, identifier: "__vtable_child")
    !13 = !{!14}
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_parent", scope: !2, file: !2, baseType: !7, flags: DIFlagPublic)
    !15 = !DIGlobalVariableExpression(var: !16, expr: !DIExpression())
    !16 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !17, isLocal: false, isDefinition: true)
    !17 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 480, flags: DIFlagPublic, elements: !18, identifier: "child")
    !18 = !{!19, !36}
    !19 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !20, size: 304, flags: DIFlagPublic)
    !20 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 304, flags: DIFlagPublic, elements: !21, identifier: "parent")
    !21 = !{!22, !31, !35}
    !22 = !DIDerivedType(tag: DW_TAG_member, name: "__grandparent", scope: !2, file: !2, baseType: !23, size: 112, flags: DIFlagPublic)
    !23 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 112, flags: DIFlagPublic, elements: !24, identifier: "grandparent")
    !24 = !{!25, !30}
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !26, size: 96, flags: DIFlagPublic)
    !26 = !DICompositeType(tag: DW_TAG_array_type, baseType: !27, size: 96, elements: !28)
    !27 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !28 = !{!29}
    !29 = !DISubrange(count: 6, lowerBound: 0)
    !30 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !27, size: 16, offset: 96, flags: DIFlagPublic)
    !31 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !32, size: 176, offset: 112, flags: DIFlagPublic)
    !32 = !DICompositeType(tag: DW_TAG_array_type, baseType: !27, size: 176, elements: !33)
    !33 = !{!34}
    !34 = !DISubrange(count: 11, lowerBound: 0)
    !35 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !27, size: 16, offset: 288, flags: DIFlagPublic)
    !36 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !32, size: 176, offset: 304, flags: DIFlagPublic)
    !37 = !DIGlobalVariableExpression(var: !38, expr: !DIExpression())
    !38 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !20, isLocal: false, isDefinition: true)
    !39 = !DIGlobalVariableExpression(var: !40, expr: !DIExpression())
    !40 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !23, isLocal: false, isDefinition: true)
    !41 = !{i32 2, !"Dwarf Version", i32 5}
    !42 = !{i32 2, !"Debug Info Version", i32 3}
    !43 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !44, splitDebugInlining: false)
    !44 = !{!39, !37, !15, !0, !5, !10}
    !45 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !46, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !43, retainedNodes: !4)
    !46 = !DISubroutineType(flags: DIFlagPublic, types: !47)
    !47 = !{null, !23}
    !48 = !DILocalVariable(name: "grandparent", scope: !45, file: !2, line: 7, type: !23)
    !49 = !DILocation(line: 7, column: 8, scope: !45)
    !50 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !51, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !43, retainedNodes: !4)
    !51 = !DISubroutineType(flags: DIFlagPublic, types: !52)
    !52 = !{null, !20}
    !53 = !DILocalVariable(name: "parent", scope: !50, file: !2, line: 14, type: !20)
    !54 = !DILocation(line: 14, column: 8, scope: !50)
    !55 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !56, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !43, retainedNodes: !4)
    !56 = !DISubroutineType(flags: DIFlagPublic, types: !57)
    !57 = !{null, !17}
    !58 = !DILocalVariable(name: "child", scope: !55, file: !2, line: 20, type: !17)
    !59 = !DILocation(line: 20, column: 8, scope: !55)
    !60 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 22, type: !61, scopeLine: 26, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !43, retainedNodes: !4)
    !61 = !DISubroutineType(flags: DIFlagPublic, types: !62)
    !62 = !{null}
    !63 = !DILocalVariable(name: "arr", scope: !60, file: !2, line: 24, type: !64)
    !64 = !DICompositeType(tag: DW_TAG_array_type, baseType: !17, size: 5280, elements: !33)
    !65 = !DILocation(line: 24, column: 12, scope: !60)
    !66 = !DILocation(line: 26, column: 12, scope: !60)
    !67 = !DILocation(line: 27, column: 12, scope: !60)
    !68 = !DILocation(line: 28, column: 12, scope: !60)
    !69 = !DILocation(line: 29, column: 12, scope: !60)
    !70 = !DILocation(line: 30, column: 12, scope: !60)
    !71 = !DILocation(line: 31, column: 8, scope: !60)
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

    %__vtable_grandparent = type {}
    %__vtable_parent = type { %__vtable_grandparent }
    %__vtable_child = type { %__vtable_parent }
    %parent = type { %grandparent, [11 x i16], i16 }
    %grandparent = type { [6 x i16], i16 }
    %child = type { %parent, [11 x i16] }

    @____vtable_grandparent__init = constant %__vtable_grandparent zeroinitializer, !dbg !0
    @____vtable_parent__init = constant %__vtable_parent zeroinitializer, !dbg !5
    @____vtable_child__init = constant %__vtable_child zeroinitializer, !dbg !10
    @__parent__init = constant %parent zeroinitializer, !dbg !15
    @__grandparent__init = constant %grandparent zeroinitializer, !dbg !33
    @__child__init = constant %child zeroinitializer, !dbg !35
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @grandparent(%grandparent* %0) !dbg !45 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !48, metadata !DIExpression()), !dbg !49
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      ret void, !dbg !49
    }

    define void @parent(%parent* %0) !dbg !50 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !53, metadata !DIExpression()), !dbg !54
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void, !dbg !54
    }

    define void @child(%child* %0) !dbg !55 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !58, metadata !DIExpression()), !dbg !59
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !59
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 0, !dbg !59
      %b = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2, !dbg !59
      %load_b = load i16, i16* %b, align 2, !dbg !59
      %1 = sext i16 %load_b to i32, !dbg !59
      %b1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2, !dbg !59
      %load_b2 = load i16, i16* %b1, align 2, !dbg !59
      %2 = sext i16 %load_b2 to i32, !dbg !59
      %tmpVar = mul i32 %2, 2, !dbg !59
      %tmpVar3 = mul i32 1, %tmpVar, !dbg !59
      %tmpVar4 = add i32 %tmpVar3, 0, !dbg !59
      %tmpVar5 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 %tmpVar4, !dbg !59
      %load_tmpVar = load i16, i16* %tmpVar5, align 2, !dbg !59
      %3 = sext i16 %load_tmpVar to i32, !dbg !59
      %tmpVar6 = add i32 %1, %3, !dbg !59
      %__grandparent7 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !59
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent7, i32 0, i32 1, !dbg !59
      %load_a = load i16, i16* %a, align 2, !dbg !59
      %4 = sext i16 %load_a to i32, !dbg !59
      %tmpVar8 = sub i32 %tmpVar6, %4, !dbg !59
      %tmpVar9 = mul i32 1, %tmpVar8, !dbg !59
      %tmpVar10 = add i32 %tmpVar9, 0, !dbg !59
      %tmpVar11 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 %tmpVar10, !dbg !59
      store i16 20, i16* %tmpVar11, align 2, !dbg !59
      ret void, !dbg !60
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init___vtable_grandparent(%__vtable_grandparent* %0) {
    entry:
      %self = alloca %__vtable_grandparent*, align 8
      store %__vtable_grandparent* %0, %__vtable_grandparent** %self, align 8
      ret void
    }

    define void @__init___vtable_parent(%__vtable_parent* %0) {
    entry:
      %self = alloca %__vtable_parent*, align 8
      store %__vtable_parent* %0, %__vtable_parent** %self, align 8
      %deref = load %__vtable_parent*, %__vtable_parent** %self, align 8
      %__vtable_grandparent = getelementptr inbounds %__vtable_parent, %__vtable_parent* %deref, i32 0, i32 0
      call void @__init___vtable_grandparent(%__vtable_grandparent* %__vtable_grandparent)
      ret void
    }

    define void @__init___vtable_child(%__vtable_child* %0) {
    entry:
      %self = alloca %__vtable_child*, align 8
      store %__vtable_child* %0, %__vtable_child** %self, align 8
      %deref = load %__vtable_child*, %__vtable_child** %self, align 8
      %__vtable_parent = getelementptr inbounds %__vtable_child, %__vtable_child* %deref, i32 0, i32 0
      call void @__init___vtable_parent(%__vtable_parent* %__vtable_parent)
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

    !llvm.module.flags = !{!41, !42}
    !llvm.dbg.cu = !{!43}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "____vtable_grandparent__init", scope: !2, file: !2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_grandparent", scope: !2, file: !2, flags: DIFlagPublic, elements: !4, identifier: "__vtable_grandparent")
    !4 = !{}
    !5 = !DIGlobalVariableExpression(var: !6, expr: !DIExpression())
    !6 = distinct !DIGlobalVariable(name: "____vtable_parent__init", scope: !2, file: !2, type: !7, isLocal: false, isDefinition: true)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_parent", scope: !2, file: !2, flags: DIFlagPublic, elements: !8, identifier: "__vtable_parent")
    !8 = !{!9}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_grandparent", scope: !2, file: !2, baseType: !3, flags: DIFlagPublic)
    !10 = !DIGlobalVariableExpression(var: !11, expr: !DIExpression())
    !11 = distinct !DIGlobalVariable(name: "____vtable_child__init", scope: !2, file: !2, type: !12, isLocal: false, isDefinition: true)
    !12 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_child", scope: !2, file: !2, flags: DIFlagPublic, elements: !13, identifier: "__vtable_child")
    !13 = !{!14}
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_parent", scope: !2, file: !2, baseType: !7, flags: DIFlagPublic)
    !15 = !DIGlobalVariableExpression(var: !16, expr: !DIExpression())
    !16 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !17, isLocal: false, isDefinition: true)
    !17 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 304, flags: DIFlagPublic, elements: !18, identifier: "parent")
    !18 = !{!19, !28, !32}
    !19 = !DIDerivedType(tag: DW_TAG_member, name: "__grandparent", scope: !2, file: !2, baseType: !20, size: 112, flags: DIFlagPublic)
    !20 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 112, flags: DIFlagPublic, elements: !21, identifier: "grandparent")
    !21 = !{!22, !27}
    !22 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !23, size: 96, flags: DIFlagPublic)
    !23 = !DICompositeType(tag: DW_TAG_array_type, baseType: !24, size: 96, elements: !25)
    !24 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !25 = !{!26}
    !26 = !DISubrange(count: 6, lowerBound: 0)
    !27 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !24, size: 16, offset: 96, flags: DIFlagPublic)
    !28 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !29, size: 176, offset: 112, flags: DIFlagPublic)
    !29 = !DICompositeType(tag: DW_TAG_array_type, baseType: !24, size: 176, elements: !30)
    !30 = !{!31}
    !31 = !DISubrange(count: 11, lowerBound: 0)
    !32 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !24, size: 16, offset: 288, flags: DIFlagPublic)
    !33 = !DIGlobalVariableExpression(var: !34, expr: !DIExpression())
    !34 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !20, isLocal: false, isDefinition: true)
    !35 = !DIGlobalVariableExpression(var: !36, expr: !DIExpression())
    !36 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !37, isLocal: false, isDefinition: true)
    !37 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 480, flags: DIFlagPublic, elements: !38, identifier: "child")
    !38 = !{!39, !40}
    !39 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !17, size: 304, flags: DIFlagPublic)
    !40 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !29, size: 176, offset: 304, flags: DIFlagPublic)
    !41 = !{i32 2, !"Dwarf Version", i32 5}
    !42 = !{i32 2, !"Debug Info Version", i32 3}
    !43 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !44, splitDebugInlining: false)
    !44 = !{!33, !15, !35, !0, !5, !10}
    !45 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !46, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !43, retainedNodes: !4)
    !46 = !DISubroutineType(flags: DIFlagPublic, types: !47)
    !47 = !{null, !20}
    !48 = !DILocalVariable(name: "grandparent", scope: !45, file: !2, line: 7, type: !20)
    !49 = !DILocation(line: 7, column: 8, scope: !45)
    !50 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !51, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !43, retainedNodes: !4)
    !51 = !DISubroutineType(flags: DIFlagPublic, types: !52)
    !52 = !{null, !17}
    !53 = !DILocalVariable(name: "parent", scope: !50, file: !2, line: 14, type: !17)
    !54 = !DILocation(line: 14, column: 8, scope: !50)
    !55 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !56, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !43, retainedNodes: !4)
    !56 = !DISubroutineType(flags: DIFlagPublic, types: !57)
    !57 = !{null, !37}
    !58 = !DILocalVariable(name: "child", scope: !55, file: !2, line: 20, type: !37)
    !59 = !DILocation(line: 20, column: 12, scope: !55)
    !60 = !DILocation(line: 21, column: 8, scope: !55)
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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %__vtable_foo = type { i32* }
    %__vtable_bar = type { %__vtable_foo }
    %foo = type {}
    %bar = type { %foo }

    @____vtable_foo__init = constant %__vtable_foo zeroinitializer, !dbg !0
    @____vtable_bar__init = constant %__vtable_bar zeroinitializer, !dbg !8
    @__foo__init = constant %foo zeroinitializer, !dbg !13
    @__bar__init = constant %bar zeroinitializer, !dbg !17
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @foo(%foo* %0) !dbg !26 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !29, metadata !DIExpression()), !dbg !30
      ret void, !dbg !30
    }

    define void @foo_baz(%foo* %0) !dbg !31 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !32, metadata !DIExpression()), !dbg !33
      ret void, !dbg !33
    }

    define void @bar(%bar* %0) !dbg !34 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !37, metadata !DIExpression()), !dbg !38
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void, !dbg !38
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init___vtable_foo(%__vtable_foo* %0) {
    entry:
      %self = alloca %__vtable_foo*, align 8
      store %__vtable_foo* %0, %__vtable_foo** %self, align 8
      ret void
    }

    define void @__init___vtable_bar(%__vtable_bar* %0) {
    entry:
      %self = alloca %__vtable_bar*, align 8
      store %__vtable_bar* %0, %__vtable_bar** %self, align 8
      %deref = load %__vtable_bar*, %__vtable_bar** %self, align 8
      %__vtable_foo = getelementptr inbounds %__vtable_bar, %__vtable_bar* %deref, i32 0, i32 0
      call void @__init___vtable_foo(%__vtable_foo* %__vtable_foo)
      ret void
    }

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

    !llvm.module.flags = !{!22, !23}
    !llvm.dbg.cu = !{!24}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "____vtable_foo__init", scope: !2, file: !2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo", scope: !2, file: !2, size: 64, flags: DIFlagPublic, elements: !4, identifier: "__vtable_foo")
    !4 = !{!5}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "foo.baz", scope: !2, file: !2, baseType: !6, size: 64, flags: DIFlagPublic)
    !6 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "VOID_POINTER", baseType: !7, size: 64, dwarfAddressSpace: 1)
    !7 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !8 = !DIGlobalVariableExpression(var: !9, expr: !DIExpression())
    !9 = distinct !DIGlobalVariable(name: "____vtable_bar__init", scope: !2, file: !2, type: !10, isLocal: false, isDefinition: true)
    !10 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_bar", scope: !2, file: !2, size: 64, flags: DIFlagPublic, elements: !11, identifier: "__vtable_bar")
    !11 = !{!12}
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_foo", scope: !2, file: !2, baseType: !3, size: 64, flags: DIFlagPublic)
    !13 = !DIGlobalVariableExpression(var: !14, expr: !DIExpression())
    !14 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !15, isLocal: false, isDefinition: true)
    !15 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, flags: DIFlagPublic, elements: !16, identifier: "foo")
    !16 = !{}
    !17 = !DIGlobalVariableExpression(var: !18, expr: !DIExpression())
    !18 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 7, type: !19, isLocal: false, isDefinition: true)
    !19 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 7, flags: DIFlagPublic, elements: !20, identifier: "bar")
    !20 = !{!21}
    !21 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !15, flags: DIFlagPublic)
    !22 = !{i32 2, !"Dwarf Version", i32 5}
    !23 = !{i32 2, !"Debug Info Version", i32 3}
    !24 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !25, splitDebugInlining: false)
    !25 = !{!13, !17, !0, !8}
    !26 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !27, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !24, retainedNodes: !16)
    !27 = !DISubroutineType(flags: DIFlagPublic, types: !28)
    !28 = !{null, !15}
    !29 = !DILocalVariable(name: "foo", scope: !26, file: !2, line: 5, type: !15)
    !30 = !DILocation(line: 5, column: 8, scope: !26)
    !31 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !26, file: !2, line: 3, type: !27, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !24, retainedNodes: !16)
    !32 = !DILocalVariable(name: "foo", scope: !31, file: !2, line: 4, type: !15)
    !33 = !DILocation(line: 4, column: 8, scope: !31)
    !34 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 7, type: !35, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !24, retainedNodes: !16)
    !35 = !DISubroutineType(flags: DIFlagPublic, types: !36)
    !36 = !{null, !19}
    !37 = !DILocalVariable(name: "bar", scope: !34, file: !2, line: 8, type: !19)
    !38 = !DILocation(line: 8, column: 8, scope: !34)
    "#);
}

#[test]
fn function_block_parents_alignment() {
    let result = codegen(
        "
FUNCTION_BLOCK parent
    VAR
        a : DINT;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK child EXTENDS parent
    VAR
        b : DINT;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK grandchild EXTENDS child
    VAR
        c : DINT;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    array_of_parent : ARRAY[0..2] OF parent;
    array_of_child : ARRAY[0..2] OF child;
    array_of_grandchild : ARRAY[0..2] OF grandchild;
    parent1 : parent;
    child1 : child;
    grandchild1 : grandchild;
END_VAR

    parent1.a := 1;
    child1.a := 2;
    child1.b := 3;
    grandchild1.a := 4;
    grandchild1.b := 5;
    grandchild1.c := 6;

    array_of_parent[0].a := 7;
    array_of_child[0].a := 8;
    array_of_child[0].b := 9;
    array_of_grandchild[0].a := 10;
    array_of_grandchild[0].b := 11;
    array_of_grandchild[0].c := 12;
    array_of_parent[1].a := 13;
    array_of_child[1].a := 14;
    array_of_child[1].b := 15;
    array_of_grandchild[1].a := 16;
    array_of_grandchild[1].b := 17;
    array_of_grandchild[1].c := 18;
    array_of_parent[2].a := 19;
    array_of_child[2].a := 20;
    array_of_child[2].b := 21;
    array_of_grandchild[2].a := 22;
    array_of_grandchild[2].b := 23;
    array_of_grandchild[2].c := 24;

END_FUNCTION
",
    );

    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %__vtable_parent = type {}
    %__vtable_child = type { %__vtable_parent }
    %__vtable_grandchild = type { %__vtable_child }
    %grandchild = type { %child, i32 }
    %child = type { %parent, i32 }
    %parent = type { i32 }

    @____vtable_parent__init = constant %__vtable_parent zeroinitializer, !dbg !0
    @____vtable_child__init = constant %__vtable_child zeroinitializer, !dbg !5
    @____vtable_grandchild__init = constant %__vtable_grandchild zeroinitializer, !dbg !10
    @__grandchild__init = constant %grandchild zeroinitializer, !dbg !15
    @__child__init = constant %child zeroinitializer, !dbg !29
    @__parent__init = constant %parent zeroinitializer, !dbg !31
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @parent(%parent* %0) !dbg !37 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !40, metadata !DIExpression()), !dbg !41
      %a = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      ret void, !dbg !41
    }

    define void @child(%child* %0) !dbg !42 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !45, metadata !DIExpression()), !dbg !46
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %b = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      ret void, !dbg !46
    }

    define void @grandchild(%grandchild* %0) !dbg !47 {
    entry:
      call void @llvm.dbg.declare(metadata %grandchild* %0, metadata !50, metadata !DIExpression()), !dbg !51
      %__child = getelementptr inbounds %grandchild, %grandchild* %0, i32 0, i32 0
      %c = getelementptr inbounds %grandchild, %grandchild* %0, i32 0, i32 1
      ret void, !dbg !51
    }

    define i32 @main() !dbg !52 {
    entry:
      %main = alloca i32, align 4
      %array_of_parent = alloca [3 x %parent], align 8
      %array_of_child = alloca [3 x %child], align 8
      %array_of_grandchild = alloca [3 x %grandchild], align 8
      %parent1 = alloca %parent, align 8
      %child1 = alloca %child, align 8
      %grandchild1 = alloca %grandchild, align 8
      call void @llvm.dbg.declare(metadata [3 x %parent]* %array_of_parent, metadata !55, metadata !DIExpression()), !dbg !59
      %0 = bitcast [3 x %parent]* %array_of_parent to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([3 x %parent]* getelementptr ([3 x %parent], [3 x %parent]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata [3 x %child]* %array_of_child, metadata !60, metadata !DIExpression()), !dbg !62
      %1 = bitcast [3 x %child]* %array_of_child to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %1, i8 0, i64 ptrtoint ([3 x %child]* getelementptr ([3 x %child], [3 x %child]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata [3 x %grandchild]* %array_of_grandchild, metadata !63, metadata !DIExpression()), !dbg !65
      %2 = bitcast [3 x %grandchild]* %array_of_grandchild to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %2, i8 0, i64 ptrtoint ([3 x %grandchild]* getelementptr ([3 x %grandchild], [3 x %grandchild]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %parent* %parent1, metadata !66, metadata !DIExpression()), !dbg !67
      %3 = bitcast %parent* %parent1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %3, i8* align 1 bitcast (%parent* @__parent__init to i8*), i64 ptrtoint (%parent* getelementptr (%parent, %parent* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %child* %child1, metadata !68, metadata !DIExpression()), !dbg !69
      %4 = bitcast %child* %child1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %4, i8* align 1 bitcast (%child* @__child__init to i8*), i64 ptrtoint (%child* getelementptr (%child, %child* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %grandchild* %grandchild1, metadata !70, metadata !DIExpression()), !dbg !71
      %5 = bitcast %grandchild* %grandchild1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %5, i8* align 1 bitcast (%grandchild* @__grandchild__init to i8*), i64 ptrtoint (%grandchild* getelementptr (%grandchild, %grandchild* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata i32* %main, metadata !72, metadata !DIExpression()), !dbg !73
      store i32 0, i32* %main, align 4
      call void @__init_parent(%parent* %parent1), !dbg !74
      call void @__init_child(%child* %child1), !dbg !74
      call void @__init_grandchild(%grandchild* %grandchild1), !dbg !74
      %a = getelementptr inbounds %parent, %parent* %parent1, i32 0, i32 0, !dbg !75
      store i32 1, i32* %a, align 4, !dbg !75
      %__parent = getelementptr inbounds %child, %child* %child1, i32 0, i32 0, !dbg !76
      %a1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !76
      store i32 2, i32* %a1, align 4, !dbg !76
      %b = getelementptr inbounds %child, %child* %child1, i32 0, i32 1, !dbg !77
      store i32 3, i32* %b, align 4, !dbg !77
      %__child = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 0, !dbg !78
      %__parent2 = getelementptr inbounds %child, %child* %__child, i32 0, i32 0, !dbg !78
      %a3 = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0, !dbg !78
      store i32 4, i32* %a3, align 4, !dbg !78
      %__child4 = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 0, !dbg !79
      %b5 = getelementptr inbounds %child, %child* %__child4, i32 0, i32 1, !dbg !79
      store i32 5, i32* %b5, align 4, !dbg !79
      %c = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 1, !dbg !80
      store i32 6, i32* %c, align 4, !dbg !80
      %tmpVar = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 0, !dbg !81
      %a6 = getelementptr inbounds %parent, %parent* %tmpVar, i32 0, i32 0, !dbg !81
      store i32 7, i32* %a6, align 4, !dbg !81
      %tmpVar7 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 0, !dbg !82
      %__parent8 = getelementptr inbounds %child, %child* %tmpVar7, i32 0, i32 0, !dbg !82
      %a9 = getelementptr inbounds %parent, %parent* %__parent8, i32 0, i32 0, !dbg !82
      store i32 8, i32* %a9, align 4, !dbg !82
      %tmpVar10 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 0, !dbg !83
      %b11 = getelementptr inbounds %child, %child* %tmpVar10, i32 0, i32 1, !dbg !83
      store i32 9, i32* %b11, align 4, !dbg !83
      %tmpVar12 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !84
      %__child13 = getelementptr inbounds %grandchild, %grandchild* %tmpVar12, i32 0, i32 0, !dbg !84
      %__parent14 = getelementptr inbounds %child, %child* %__child13, i32 0, i32 0, !dbg !84
      %a15 = getelementptr inbounds %parent, %parent* %__parent14, i32 0, i32 0, !dbg !84
      store i32 10, i32* %a15, align 4, !dbg !84
      %tmpVar16 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !85
      %__child17 = getelementptr inbounds %grandchild, %grandchild* %tmpVar16, i32 0, i32 0, !dbg !85
      %b18 = getelementptr inbounds %child, %child* %__child17, i32 0, i32 1, !dbg !85
      store i32 11, i32* %b18, align 4, !dbg !85
      %tmpVar19 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !86
      %c20 = getelementptr inbounds %grandchild, %grandchild* %tmpVar19, i32 0, i32 1, !dbg !86
      store i32 12, i32* %c20, align 4, !dbg !86
      %tmpVar21 = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 1, !dbg !87
      %a22 = getelementptr inbounds %parent, %parent* %tmpVar21, i32 0, i32 0, !dbg !87
      store i32 13, i32* %a22, align 4, !dbg !87
      %tmpVar23 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 1, !dbg !88
      %__parent24 = getelementptr inbounds %child, %child* %tmpVar23, i32 0, i32 0, !dbg !88
      %a25 = getelementptr inbounds %parent, %parent* %__parent24, i32 0, i32 0, !dbg !88
      store i32 14, i32* %a25, align 4, !dbg !88
      %tmpVar26 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 1, !dbg !89
      %b27 = getelementptr inbounds %child, %child* %tmpVar26, i32 0, i32 1, !dbg !89
      store i32 15, i32* %b27, align 4, !dbg !89
      %tmpVar28 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !90
      %__child29 = getelementptr inbounds %grandchild, %grandchild* %tmpVar28, i32 0, i32 0, !dbg !90
      %__parent30 = getelementptr inbounds %child, %child* %__child29, i32 0, i32 0, !dbg !90
      %a31 = getelementptr inbounds %parent, %parent* %__parent30, i32 0, i32 0, !dbg !90
      store i32 16, i32* %a31, align 4, !dbg !90
      %tmpVar32 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !91
      %__child33 = getelementptr inbounds %grandchild, %grandchild* %tmpVar32, i32 0, i32 0, !dbg !91
      %b34 = getelementptr inbounds %child, %child* %__child33, i32 0, i32 1, !dbg !91
      store i32 17, i32* %b34, align 4, !dbg !91
      %tmpVar35 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !92
      %c36 = getelementptr inbounds %grandchild, %grandchild* %tmpVar35, i32 0, i32 1, !dbg !92
      store i32 18, i32* %c36, align 4, !dbg !92
      %tmpVar37 = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 2, !dbg !93
      %a38 = getelementptr inbounds %parent, %parent* %tmpVar37, i32 0, i32 0, !dbg !93
      store i32 19, i32* %a38, align 4, !dbg !93
      %tmpVar39 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 2, !dbg !94
      %__parent40 = getelementptr inbounds %child, %child* %tmpVar39, i32 0, i32 0, !dbg !94
      %a41 = getelementptr inbounds %parent, %parent* %__parent40, i32 0, i32 0, !dbg !94
      store i32 20, i32* %a41, align 4, !dbg !94
      %tmpVar42 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 2, !dbg !95
      %b43 = getelementptr inbounds %child, %child* %tmpVar42, i32 0, i32 1, !dbg !95
      store i32 21, i32* %b43, align 4, !dbg !95
      %tmpVar44 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !96
      %__child45 = getelementptr inbounds %grandchild, %grandchild* %tmpVar44, i32 0, i32 0, !dbg !96
      %__parent46 = getelementptr inbounds %child, %child* %__child45, i32 0, i32 0, !dbg !96
      %a47 = getelementptr inbounds %parent, %parent* %__parent46, i32 0, i32 0, !dbg !96
      store i32 22, i32* %a47, align 4, !dbg !96
      %tmpVar48 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !97
      %__child49 = getelementptr inbounds %grandchild, %grandchild* %tmpVar48, i32 0, i32 0, !dbg !97
      %b50 = getelementptr inbounds %child, %child* %__child49, i32 0, i32 1, !dbg !97
      store i32 23, i32* %b50, align 4, !dbg !97
      %tmpVar51 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !98
      %c52 = getelementptr inbounds %grandchild, %grandchild* %tmpVar51, i32 0, i32 1, !dbg !98
      store i32 24, i32* %c52, align 4, !dbg !98
      %main_ret = load i32, i32* %main, align 4, !dbg !99
      ret i32 %main_ret, !dbg !99
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #1

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #2

    define void @__init___vtable_parent(%__vtable_parent* %0) {
    entry:
      %self = alloca %__vtable_parent*, align 8
      store %__vtable_parent* %0, %__vtable_parent** %self, align 8
      ret void
    }

    define void @__init___vtable_child(%__vtable_child* %0) {
    entry:
      %self = alloca %__vtable_child*, align 8
      store %__vtable_child* %0, %__vtable_child** %self, align 8
      %deref = load %__vtable_child*, %__vtable_child** %self, align 8
      %__vtable_parent = getelementptr inbounds %__vtable_child, %__vtable_child* %deref, i32 0, i32 0
      call void @__init___vtable_parent(%__vtable_parent* %__vtable_parent)
      ret void
    }

    define void @__init___vtable_grandchild(%__vtable_grandchild* %0) {
    entry:
      %self = alloca %__vtable_grandchild*, align 8
      store %__vtable_grandchild* %0, %__vtable_grandchild** %self, align 8
      %deref = load %__vtable_grandchild*, %__vtable_grandchild** %self, align 8
      %__vtable_child = getelementptr inbounds %__vtable_grandchild, %__vtable_grandchild* %deref, i32 0, i32 0
      call void @__init___vtable_child(%__vtable_child* %__vtable_child)
      ret void
    }

    define void @__init_grandchild(%grandchild* %0) {
    entry:
      %self = alloca %grandchild*, align 8
      store %grandchild* %0, %grandchild** %self, align 8
      %deref = load %grandchild*, %grandchild** %self, align 8
      %__child = getelementptr inbounds %grandchild, %grandchild* %deref, i32 0, i32 0
      call void @__init_child(%child* %__child)
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

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn writeonly }
    attributes #2 = { argmemonly nofree nounwind willreturn }

    !llvm.module.flags = !{!33, !34}
    !llvm.dbg.cu = !{!35}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "____vtable_parent__init", scope: !2, file: !2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_parent", scope: !2, file: !2, flags: DIFlagPublic, elements: !4, identifier: "__vtable_parent")
    !4 = !{}
    !5 = !DIGlobalVariableExpression(var: !6, expr: !DIExpression())
    !6 = distinct !DIGlobalVariable(name: "____vtable_child__init", scope: !2, file: !2, type: !7, isLocal: false, isDefinition: true)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_child", scope: !2, file: !2, flags: DIFlagPublic, elements: !8, identifier: "__vtable_child")
    !8 = !{!9}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_parent", scope: !2, file: !2, baseType: !3, flags: DIFlagPublic)
    !10 = !DIGlobalVariableExpression(var: !11, expr: !DIExpression())
    !11 = distinct !DIGlobalVariable(name: "____vtable_grandchild__init", scope: !2, file: !2, type: !12, isLocal: false, isDefinition: true)
    !12 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_grandchild", scope: !2, file: !2, flags: DIFlagPublic, elements: !13, identifier: "__vtable_grandchild")
    !13 = !{!14}
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_child", scope: !2, file: !2, baseType: !7, flags: DIFlagPublic)
    !15 = !DIGlobalVariableExpression(var: !16, expr: !DIExpression())
    !16 = distinct !DIGlobalVariable(name: "__grandchild__init", scope: !2, file: !2, line: 14, type: !17, isLocal: false, isDefinition: true)
    !17 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandchild", scope: !2, file: !2, line: 14, size: 96, flags: DIFlagPublic, elements: !18, identifier: "grandchild")
    !18 = !{!19, !28}
    !19 = !DIDerivedType(tag: DW_TAG_member, name: "__child", scope: !2, file: !2, baseType: !20, size: 64, flags: DIFlagPublic)
    !20 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 8, size: 64, flags: DIFlagPublic, elements: !21, identifier: "child")
    !21 = !{!22, !27}
    !22 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !23, size: 32, flags: DIFlagPublic)
    !23 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 2, size: 32, flags: DIFlagPublic, elements: !24, identifier: "parent")
    !24 = !{!25}
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 4, baseType: !26, size: 32, flags: DIFlagPublic)
    !26 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !27 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 10, baseType: !26, size: 32, offset: 32, flags: DIFlagPublic)
    !28 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 16, baseType: !26, size: 32, offset: 64, flags: DIFlagPublic)
    !29 = !DIGlobalVariableExpression(var: !30, expr: !DIExpression())
    !30 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 8, type: !20, isLocal: false, isDefinition: true)
    !31 = !DIGlobalVariableExpression(var: !32, expr: !DIExpression())
    !32 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 2, type: !23, isLocal: false, isDefinition: true)
    !33 = !{i32 2, !"Dwarf Version", i32 5}
    !34 = !{i32 2, !"Debug Info Version", i32 3}
    !35 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !36, splitDebugInlining: false)
    !36 = !{!31, !29, !15, !0, !5, !10}
    !37 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 2, type: !38, scopeLine: 6, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !4)
    !38 = !DISubroutineType(flags: DIFlagPublic, types: !39)
    !39 = !{null, !23}
    !40 = !DILocalVariable(name: "parent", scope: !37, file: !2, line: 6, type: !23)
    !41 = !DILocation(line: 6, scope: !37)
    !42 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 8, type: !43, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !4)
    !43 = !DISubroutineType(flags: DIFlagPublic, types: !44)
    !44 = !{null, !20}
    !45 = !DILocalVariable(name: "child", scope: !42, file: !2, line: 12, type: !20)
    !46 = !DILocation(line: 12, scope: !42)
    !47 = distinct !DISubprogram(name: "grandchild", linkageName: "grandchild", scope: !2, file: !2, line: 14, type: !48, scopeLine: 18, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !4)
    !48 = !DISubroutineType(flags: DIFlagPublic, types: !49)
    !49 = !{null, !17}
    !50 = !DILocalVariable(name: "grandchild", scope: !47, file: !2, line: 18, type: !17)
    !51 = !DILocation(line: 18, scope: !47)
    !52 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 20, type: !53, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !4)
    !53 = !DISubroutineType(flags: DIFlagPublic, types: !54)
    !54 = !{null}
    !55 = !DILocalVariable(name: "array_of_parent", scope: !52, file: !2, line: 22, type: !56)
    !56 = !DICompositeType(tag: DW_TAG_array_type, baseType: !23, size: 96, elements: !57)
    !57 = !{!58}
    !58 = !DISubrange(count: 3, lowerBound: 0)
    !59 = !DILocation(line: 22, column: 4, scope: !52)
    !60 = !DILocalVariable(name: "array_of_child", scope: !52, file: !2, line: 23, type: !61)
    !61 = !DICompositeType(tag: DW_TAG_array_type, baseType: !20, size: 192, elements: !57)
    !62 = !DILocation(line: 23, column: 4, scope: !52)
    !63 = !DILocalVariable(name: "array_of_grandchild", scope: !52, file: !2, line: 24, type: !64)
    !64 = !DICompositeType(tag: DW_TAG_array_type, baseType: !17, size: 288, elements: !57)
    !65 = !DILocation(line: 24, column: 4, scope: !52)
    !66 = !DILocalVariable(name: "parent1", scope: !52, file: !2, line: 25, type: !23)
    !67 = !DILocation(line: 25, column: 4, scope: !52)
    !68 = !DILocalVariable(name: "child1", scope: !52, file: !2, line: 26, type: !20)
    !69 = !DILocation(line: 26, column: 4, scope: !52)
    !70 = !DILocalVariable(name: "grandchild1", scope: !52, file: !2, line: 27, type: !17)
    !71 = !DILocation(line: 27, column: 4, scope: !52)
    !72 = !DILocalVariable(name: "main", scope: !52, file: !2, line: 20, type: !26)
    !73 = !DILocation(line: 20, column: 9, scope: !52)
    !74 = !DILocation(line: 0, scope: !52)
    !75 = !DILocation(line: 30, column: 4, scope: !52)
    !76 = !DILocation(line: 31, column: 4, scope: !52)
    !77 = !DILocation(line: 32, column: 4, scope: !52)
    !78 = !DILocation(line: 33, column: 4, scope: !52)
    !79 = !DILocation(line: 34, column: 4, scope: !52)
    !80 = !DILocation(line: 35, column: 4, scope: !52)
    !81 = !DILocation(line: 37, column: 4, scope: !52)
    !82 = !DILocation(line: 38, column: 4, scope: !52)
    !83 = !DILocation(line: 39, column: 4, scope: !52)
    !84 = !DILocation(line: 40, column: 4, scope: !52)
    !85 = !DILocation(line: 41, column: 4, scope: !52)
    !86 = !DILocation(line: 42, column: 4, scope: !52)
    !87 = !DILocation(line: 43, column: 4, scope: !52)
    !88 = !DILocation(line: 44, column: 4, scope: !52)
    !89 = !DILocation(line: 45, column: 4, scope: !52)
    !90 = !DILocation(line: 46, column: 4, scope: !52)
    !91 = !DILocation(line: 47, column: 4, scope: !52)
    !92 = !DILocation(line: 48, column: 4, scope: !52)
    !93 = !DILocation(line: 49, column: 4, scope: !52)
    !94 = !DILocation(line: 50, column: 4, scope: !52)
    !95 = !DILocation(line: 51, column: 4, scope: !52)
    !96 = !DILocation(line: 52, column: 4, scope: !52)
    !97 = !DILocation(line: 53, column: 4, scope: !52)
    !98 = !DILocation(line: 54, column: 4, scope: !52)
    !99 = !DILocation(line: 56, scope: !52)
    "#);
}
