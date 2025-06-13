use plc_util::filtered_assert_snapshot;
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
    filtered_assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { i16, [81 x i8], [11 x [81 x i8]] }
    %bar = type { %foo }

    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !0
    @__bar__init = unnamed_addr constant %bar zeroinitializer, !dbg !17
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @foo(%foo* %0) !dbg !27 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !31, metadata !DIExpression()), !dbg !32
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %a = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %b = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %c = getelementptr inbounds %foo, %foo* %0, i32 0, i32 2
      ret void, !dbg !32
    }

    define void @bar(%bar* %0) !dbg !33 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !36, metadata !DIExpression()), !dbg !37
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void, !dbg !37
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

    define void @__user_init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %__foo = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__user_init_foo(%foo* %__foo)
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
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

    !llvm.module.flags = !{!23, !24}
    !llvm.dbg.cu = !{!25}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 7792, align: 64, flags: DIFlagPublic, elements: !5, identifier: "foo")
    !5 = !{!6, !8, !13}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 4, baseType: !7, size: 16, align: 16, flags: DIFlagPublic)
    !7 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 5, baseType: !9, size: 648, align: 8, offset: 16, flags: DIFlagPublic)
    !9 = !DICompositeType(tag: DW_TAG_array_type, baseType: !10, size: 648, align: 8, elements: !11)
    !10 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !11 = !{!12}
    !12 = !DISubrange(count: 81, lowerBound: 0)
    !13 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 6, baseType: !14, size: 7128, align: 8, offset: 664, flags: DIFlagPublic)
    !14 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 7128, align: 8, elements: !15)
    !15 = !{!16}
    !16 = !DISubrange(count: 11, lowerBound: 0)
    !17 = !DIGlobalVariableExpression(var: !18, expr: !DIExpression())
    !18 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 10, type: !19, isLocal: false, isDefinition: true)
    !19 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !20)
    !20 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 10, size: 7792, align: 64, flags: DIFlagPublic, elements: !21, identifier: "bar")
    !21 = !{!22}
    !22 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !4, size: 7792, align: 64, flags: DIFlagPublic)
    !23 = !{i32 2, !"Dwarf Version", i32 5}
    !24 = !{i32 2, !"Debug Info Version", i32 3}
    !25 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !26, splitDebugInlining: false)
    !26 = !{!0, !17}
    !27 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !28, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !25, retainedNodes: !30)
    !28 = !DISubroutineType(flags: DIFlagPublic, types: !29)
    !29 = !{null, !4}
    !30 = !{}
    !31 = !DILocalVariable(name: "foo", scope: !27, file: !2, line: 8, type: !4)
    !32 = !DILocation(line: 8, column: 8, scope: !27)
    !33 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 10, type: !34, scopeLine: 11, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !25, retainedNodes: !30)
    !34 = !DISubroutineType(flags: DIFlagPublic, types: !35)
    !35 = !{null, !20}
    !36 = !DILocalVariable(name: "bar", scope: !33, file: !2, line: 11, type: !20)
    !37 = !DILocation(line: 11, column: 8, scope: !33)
    "###);
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

    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %fb2 = type { %fb }
    %fb = type { i16, i16 }
    %foo = type { %fb2 }

    @__fb2__init = unnamed_addr constant %fb2 zeroinitializer, !dbg !0
    @__fb__init = unnamed_addr constant %fb zeroinitializer, !dbg !12
    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !15
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @fb(%fb* %0) !dbg !25 {
    entry:
      call void @llvm.dbg.declare(metadata %fb* %0, metadata !29, metadata !DIExpression()), !dbg !30
      %this = alloca %fb*, align 8
      store %fb* %0, %fb** %this, align 8
      %x = getelementptr inbounds %fb, %fb* %0, i32 0, i32 0
      %y = getelementptr inbounds %fb, %fb* %0, i32 0, i32 1
      ret void, !dbg !30
    }

    define void @fb2(%fb2* %0) !dbg !31 {
    entry:
      call void @llvm.dbg.declare(metadata %fb2* %0, metadata !34, metadata !DIExpression()), !dbg !35
      %this = alloca %fb2*, align 8
      store %fb2* %0, %fb2** %this, align 8
      %__fb = getelementptr inbounds %fb2, %fb2* %0, i32 0, i32 0
      ret void, !dbg !35
    }

    define void @foo(%foo* %0) !dbg !36 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !39, metadata !DIExpression()), !dbg !40
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %myFb = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %__fb = getelementptr inbounds %fb2, %fb2* %myFb, i32 0, i32 0, !dbg !40
      %x = getelementptr inbounds %fb, %fb* %__fb, i32 0, i32 0, !dbg !40
      store i16 1, i16* %x, align 2, !dbg !40
      ret void, !dbg !41
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

    define void @__user_init_fb(%fb* %0) {
    entry:
      %self = alloca %fb*, align 8
      store %fb* %0, %fb** %self, align 8
      ret void
    }

    define void @__user_init_fb2(%fb2* %0) {
    entry:
      %self = alloca %fb2*, align 8
      store %fb2* %0, %fb2** %self, align 8
      %deref = load %fb2*, %fb2** %self, align 8
      %__fb = getelementptr inbounds %fb2, %fb2* %deref, i32 0, i32 0
      call void @__user_init_fb(%fb* %__fb)
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %myFb = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      call void @__user_init_fb2(%fb2* %myFb)
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
    !1 = distinct !DIGlobalVariable(name: "__fb2__init", scope: !2, file: !2, line: 9, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb2", scope: !2, file: !2, line: 9, size: 32, align: 64, flags: DIFlagPublic, elements: !5, identifier: "fb2")
    !5 = !{!6}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__fb", scope: !2, file: !2, baseType: !7, size: 32, align: 64, flags: DIFlagPublic)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb", scope: !2, file: !2, line: 2, size: 32, align: 64, flags: DIFlagPublic, elements: !8, identifier: "fb")
    !8 = !{!9, !11}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 4, baseType: !10, size: 16, align: 16, flags: DIFlagPublic)
    !10 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 5, baseType: !10, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
    !12 = !DIGlobalVariableExpression(var: !13, expr: !DIExpression())
    !13 = distinct !DIGlobalVariable(name: "__fb__init", scope: !2, file: !2, line: 2, type: !14, isLocal: false, isDefinition: true)
    !14 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !7)
    !15 = !DIGlobalVariableExpression(var: !16, expr: !DIExpression())
    !16 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 12, type: !17, isLocal: false, isDefinition: true)
    !17 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !18)
    !18 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 12, size: 32, align: 64, flags: DIFlagPublic, elements: !19, identifier: "foo")
    !19 = !{!20}
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "myFb", scope: !2, file: !2, line: 14, baseType: !4, size: 32, align: 64, flags: DIFlagPublic)
    !21 = !{i32 2, !"Dwarf Version", i32 5}
    !22 = !{i32 2, !"Debug Info Version", i32 3}
    !23 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !24, splitDebugInlining: false)
    !24 = !{!12, !0, !15}
    !25 = distinct !DISubprogram(name: "fb", linkageName: "fb", scope: !2, file: !2, line: 2, type: !26, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !23, retainedNodes: !28)
    !26 = !DISubroutineType(flags: DIFlagPublic, types: !27)
    !27 = !{null, !7}
    !28 = !{}
    !29 = !DILocalVariable(name: "fb", scope: !25, file: !2, line: 7, type: !7)
    !30 = !DILocation(line: 7, column: 8, scope: !25)
    !31 = distinct !DISubprogram(name: "fb2", linkageName: "fb2", scope: !2, file: !2, line: 9, type: !32, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !23, retainedNodes: !28)
    !32 = !DISubroutineType(flags: DIFlagPublic, types: !33)
    !33 = !{null, !4}
    !34 = !DILocalVariable(name: "fb2", scope: !31, file: !2, line: 10, type: !4)
    !35 = !DILocation(line: 10, column: 8, scope: !31)
    !36 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 12, type: !37, scopeLine: 16, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !23, retainedNodes: !28)
    !37 = !DISubroutineType(flags: DIFlagPublic, types: !38)
    !38 = !{null, !18}
    !39 = !DILocalVariable(name: "foo", scope: !36, file: !2, line: 16, type: !18)
    !40 = !DILocation(line: 16, column: 12, scope: !36)
    !41 = !DILocation(line: 17, column: 8, scope: !36)
    "###);
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
    filtered_assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %bar = type { %foo }
    %foo = type { [81 x i8] }

    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"world\00"
    @__bar__init = unnamed_addr constant %bar zeroinitializer, !dbg !0
    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !14
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @foo(%foo* %0) !dbg !21 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !25, metadata !DIExpression()), !dbg !26
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void, !dbg !26
    }

    define void @foo__baz(%foo* %0) !dbg !27 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !28, metadata !DIExpression()), !dbg !29
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %1 = bitcast [81 x i8]* %s to i8*, !dbg !29
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_0, i32 0, i32 0), i32 6, i1 false), !dbg !29
      ret void, !dbg !30
    }

    define void @bar(%bar* %0) !dbg !31 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !34, metadata !DIExpression()), !dbg !35
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %__foo, i32 0, i32 0, !dbg !35
      %1 = bitcast [81 x i8]* %s to i8*, !dbg !35
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_1, i32 0, i32 0), i32 6, i1 false), !dbg !35
      ret void, !dbg !36
    }

    define void @main() !dbg !37 {
    entry:
      %s = alloca [81 x i8], align 1
      %fb = alloca %bar, align 8
      call void @llvm.dbg.declare(metadata [81 x i8]* %s, metadata !40, metadata !DIExpression()), !dbg !41
      %0 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %bar* %fb, metadata !42, metadata !DIExpression()), !dbg !43
      %1 = bitcast %bar* %fb to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 getelementptr inbounds (%bar, %bar* @__bar__init, i32 0, i32 0, i32 0, i32 0), i64 ptrtoint (%bar* getelementptr (%bar, %bar* null, i32 1) to i64), i1 false)
      call void @__init_bar(%bar* %fb), !dbg !44
      call void @__user_init_bar(%bar* %fb), !dbg !44
      %__foo = getelementptr inbounds %bar, %bar* %fb, i32 0, i32 0, !dbg !44
      call void @foo__baz(%foo* %__foo), !dbg !45
      call void @bar(%bar* %fb), !dbg !46
      ret void, !dbg !47
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

    define void @__user_init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %__foo = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__user_init_foo(%foo* %__foo)
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
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

    !llvm.module.flags = !{!17, !18}
    !llvm.dbg.cu = !{!19}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 11, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 11, size: 648, align: 64, flags: DIFlagPublic, elements: !5, identifier: "bar")
    !5 = !{!6}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !7, size: 648, align: 64, flags: DIFlagPublic)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 648, align: 64, flags: DIFlagPublic, elements: !8, identifier: "foo")
    !8 = !{!9}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !2, file: !2, line: 4, baseType: !10, size: 648, align: 8, flags: DIFlagPublic)
    !10 = !DICompositeType(tag: DW_TAG_array_type, baseType: !11, size: 648, align: 8, elements: !12)
    !11 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !12 = !{!13}
    !13 = !DISubrange(count: 81, lowerBound: 0)
    !14 = !DIGlobalVariableExpression(var: !15, expr: !DIExpression())
    !15 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !16, isLocal: false, isDefinition: true)
    !16 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !7)
    !17 = !{i32 2, !"Dwarf Version", i32 5}
    !18 = !{i32 2, !"Debug Info Version", i32 3}
    !19 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !20, splitDebugInlining: false)
    !20 = !{!14, !0}
    !21 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !22, scopeLine: 9, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !19, retainedNodes: !24)
    !22 = !DISubroutineType(flags: DIFlagPublic, types: !23)
    !23 = !{null, !7}
    !24 = !{}
    !25 = !DILocalVariable(name: "foo", scope: !21, file: !2, line: 9, type: !7)
    !26 = !DILocation(line: 9, column: 8, scope: !21)
    !27 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !21, file: !2, line: 6, type: !22, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !19, retainedNodes: !24)
    !28 = !DILocalVariable(name: "foo", scope: !27, file: !2, line: 7, type: !7)
    !29 = !DILocation(line: 7, column: 12, scope: !27)
    !30 = !DILocation(line: 8, column: 8, scope: !27)
    !31 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 11, type: !32, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !19, retainedNodes: !24)
    !32 = !DISubroutineType(flags: DIFlagPublic, types: !33)
    !33 = !{null, !4}
    !34 = !DILocalVariable(name: "bar", scope: !31, file: !2, line: 12, type: !4)
    !35 = !DILocation(line: 12, column: 12, scope: !31)
    !36 = !DILocation(line: 13, column: 8, scope: !31)
    !37 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 15, type: !38, scopeLine: 15, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !19, retainedNodes: !24)
    !38 = !DISubroutineType(flags: DIFlagPublic, types: !39)
    !39 = !{null}
    !40 = !DILocalVariable(name: "s", scope: !37, file: !2, line: 17, type: !10, align: 8)
    !41 = !DILocation(line: 17, column: 12, scope: !37)
    !42 = !DILocalVariable(name: "fb", scope: !37, file: !2, line: 18, type: !4, align: 64)
    !43 = !DILocation(line: 18, column: 12, scope: !37)
    !44 = !DILocation(line: 0, scope: !37)
    !45 = !DILocation(line: 20, column: 12, scope: !37)
    !46 = !DILocation(line: 21, column: 12, scope: !37)
    !47 = !DILocation(line: 22, column: 8, scope: !37)
    "###);
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
    filtered_assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %child = type { %parent, [11 x i16] }
    %parent = type { %grandparent, [11 x i16], i16 }
    %grandparent = type { [6 x i16], i16 }

    @__child__init = unnamed_addr constant %child zeroinitializer, !dbg !0
    @__parent__init = unnamed_addr constant %parent zeroinitializer, !dbg !24
    @__grandparent__init = unnamed_addr constant %grandparent zeroinitializer, !dbg !27
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @grandparent(%grandparent* %0) !dbg !34 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !38, metadata !DIExpression()), !dbg !39
      %this = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %this, align 8
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      ret void, !dbg !39
    }

    define void @parent(%parent* %0) !dbg !40 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !43, metadata !DIExpression()), !dbg !44
      %this = alloca %parent*, align 8
      store %parent* %0, %parent** %this, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void, !dbg !44
    }

    define void @child(%child* %0) !dbg !45 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !48, metadata !DIExpression()), !dbg !49
      %this = alloca %child*, align 8
      store %child* %0, %child** %this, align 8
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      ret void, !dbg !49
    }

    define void @main() !dbg !50 {
    entry:
      %arr = alloca [11 x %child], align 8
      call void @llvm.dbg.declare(metadata [11 x %child]* %arr, metadata !53, metadata !DIExpression()), !dbg !55
      %0 = bitcast [11 x %child]* %arr to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([11 x %child]* getelementptr ([11 x %child], [11 x %child]* null, i32 1) to i64), i1 false)
      %tmpVar = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !56
      %__parent = getelementptr inbounds %child, %child* %tmpVar, i32 0, i32 0, !dbg !56
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !56
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 1, !dbg !56
      store i16 10, i16* %a, align 2, !dbg !56
      %tmpVar1 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !57
      %__parent2 = getelementptr inbounds %child, %child* %tmpVar1, i32 0, i32 0, !dbg !57
      %__grandparent3 = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0, !dbg !57
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent3, i32 0, i32 0, !dbg !57
      %tmpVar4 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 0, !dbg !57
      store i16 20, i16* %tmpVar4, align 2, !dbg !57
      %tmpVar5 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !58
      %__parent6 = getelementptr inbounds %child, %child* %tmpVar5, i32 0, i32 0, !dbg !58
      %b = getelementptr inbounds %parent, %parent* %__parent6, i32 0, i32 2, !dbg !58
      store i16 30, i16* %b, align 2, !dbg !58
      %tmpVar7 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !59
      %__parent8 = getelementptr inbounds %child, %child* %tmpVar7, i32 0, i32 0, !dbg !59
      %x = getelementptr inbounds %parent, %parent* %__parent8, i32 0, i32 1, !dbg !59
      %tmpVar9 = getelementptr inbounds [11 x i16], [11 x i16]* %x, i32 0, i32 1, !dbg !59
      store i16 40, i16* %tmpVar9, align 2, !dbg !59
      %tmpVar10 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 2, !dbg !60
      %z = getelementptr inbounds %child, %child* %tmpVar10, i32 0, i32 1, !dbg !60
      %tmpVar11 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 2, !dbg !60
      store i16 50, i16* %tmpVar11, align 2, !dbg !60
      ret void, !dbg !61
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

    define void @__user_init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__user_init_grandparent(%grandparent* %__grandparent)
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn writeonly }

    !llvm.module.flags = !{!30, !31}
    !llvm.dbg.cu = !{!32}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 480, align: 64, flags: DIFlagPublic, elements: !5, identifier: "child")
    !5 = !{!6, !23}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !7, size: 304, align: 64, flags: DIFlagPublic)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 304, align: 64, flags: DIFlagPublic, elements: !8, identifier: "parent")
    !8 = !{!9, !18, !22}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__grandparent", scope: !2, file: !2, baseType: !10, size: 112, align: 64, flags: DIFlagPublic)
    !10 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 112, align: 64, flags: DIFlagPublic, elements: !11, identifier: "grandparent")
    !11 = !{!12, !17}
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !13, size: 96, align: 16, flags: DIFlagPublic)
    !13 = !DICompositeType(tag: DW_TAG_array_type, baseType: !14, size: 96, align: 16, elements: !15)
    !14 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !15 = !{!16}
    !16 = !DISubrange(count: 6, lowerBound: 0)
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !14, size: 16, align: 16, offset: 96, flags: DIFlagPublic)
    !18 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !19, size: 176, align: 16, offset: 112, flags: DIFlagPublic)
    !19 = !DICompositeType(tag: DW_TAG_array_type, baseType: !14, size: 176, align: 16, elements: !20)
    !20 = !{!21}
    !21 = !DISubrange(count: 11, lowerBound: 0)
    !22 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !14, size: 16, align: 16, offset: 288, flags: DIFlagPublic)
    !23 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !19, size: 176, align: 16, offset: 304, flags: DIFlagPublic)
    !24 = !DIGlobalVariableExpression(var: !25, expr: !DIExpression())
    !25 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !26, isLocal: false, isDefinition: true)
    !26 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !7)
    !27 = !DIGlobalVariableExpression(var: !28, expr: !DIExpression())
    !28 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !29, isLocal: false, isDefinition: true)
    !29 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !10)
    !30 = !{i32 2, !"Dwarf Version", i32 5}
    !31 = !{i32 2, !"Debug Info Version", i32 3}
    !32 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !33, splitDebugInlining: false)
    !33 = !{!27, !24, !0}
    !34 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !35, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !32, retainedNodes: !37)
    !35 = !DISubroutineType(flags: DIFlagPublic, types: !36)
    !36 = !{null, !10}
    !37 = !{}
    !38 = !DILocalVariable(name: "grandparent", scope: !34, file: !2, line: 7, type: !10)
    !39 = !DILocation(line: 7, column: 8, scope: !34)
    !40 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !41, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !32, retainedNodes: !37)
    !41 = !DISubroutineType(flags: DIFlagPublic, types: !42)
    !42 = !{null, !7}
    !43 = !DILocalVariable(name: "parent", scope: !40, file: !2, line: 14, type: !7)
    !44 = !DILocation(line: 14, column: 8, scope: !40)
    !45 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !46, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !32, retainedNodes: !37)
    !46 = !DISubroutineType(flags: DIFlagPublic, types: !47)
    !47 = !{null, !4}
    !48 = !DILocalVariable(name: "child", scope: !45, file: !2, line: 20, type: !4)
    !49 = !DILocation(line: 20, column: 8, scope: !45)
    !50 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 22, type: !51, scopeLine: 26, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !32, retainedNodes: !37)
    !51 = !DISubroutineType(flags: DIFlagPublic, types: !52)
    !52 = !{null}
    !53 = !DILocalVariable(name: "arr", scope: !50, file: !2, line: 24, type: !54, align: 64)
    !54 = !DICompositeType(tag: DW_TAG_array_type, baseType: !4, size: 5280, align: 64, elements: !20)
    !55 = !DILocation(line: 24, column: 12, scope: !50)
    !56 = !DILocation(line: 26, column: 12, scope: !50)
    !57 = !DILocation(line: 27, column: 12, scope: !50)
    !58 = !DILocation(line: 28, column: 12, scope: !50)
    !59 = !DILocation(line: 29, column: 12, scope: !50)
    !60 = !DILocation(line: 30, column: 12, scope: !50)
    !61 = !DILocation(line: 31, column: 8, scope: !50)
    "###);
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

    filtered_assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %parent = type { %grandparent, [11 x i16], i16 }
    %grandparent = type { [6 x i16], i16 }
    %child = type { %parent, [11 x i16] }

    @__parent__init = unnamed_addr constant %parent zeroinitializer, !dbg !0
    @__grandparent__init = unnamed_addr constant %grandparent zeroinitializer, !dbg !20
    @__child__init = unnamed_addr constant %child zeroinitializer, !dbg !23
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @grandparent(%grandparent* %0) !dbg !34 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !38, metadata !DIExpression()), !dbg !39
      %this = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %this, align 8
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      ret void, !dbg !39
    }

    define void @parent(%parent* %0) !dbg !40 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !43, metadata !DIExpression()), !dbg !44
      %this = alloca %parent*, align 8
      store %parent* %0, %parent** %this, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void, !dbg !44
    }

    define void @child(%child* %0) !dbg !45 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !48, metadata !DIExpression()), !dbg !49
      %this = alloca %child*, align 8
      store %child* %0, %child** %this, align 8
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !49
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 0, !dbg !49
      %b = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2, !dbg !49
      %load_b = load i16, i16* %b, align 2, !dbg !49
      %1 = sext i16 %load_b to i32, !dbg !49
      %b1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2, !dbg !49
      %load_b2 = load i16, i16* %b1, align 2, !dbg !49
      %2 = sext i16 %load_b2 to i32, !dbg !49
      %tmpVar = mul i32 %2, 2, !dbg !49
      %tmpVar3 = mul i32 1, %tmpVar, !dbg !49
      %tmpVar4 = add i32 %tmpVar3, 0, !dbg !49
      %tmpVar5 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 %tmpVar4, !dbg !49
      %load_tmpVar = load i16, i16* %tmpVar5, align 2, !dbg !49
      %3 = sext i16 %load_tmpVar to i32, !dbg !49
      %tmpVar6 = add i32 %1, %3, !dbg !49
      %__grandparent7 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !49
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent7, i32 0, i32 1, !dbg !49
      %load_a = load i16, i16* %a, align 2, !dbg !49
      %4 = sext i16 %load_a to i32, !dbg !49
      %tmpVar8 = sub i32 %tmpVar6, %4, !dbg !49
      %tmpVar9 = mul i32 1, %tmpVar8, !dbg !49
      %tmpVar10 = add i32 %tmpVar9, 0, !dbg !49
      %tmpVar11 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 %tmpVar10, !dbg !49
      store i16 20, i16* %tmpVar11, align 2, !dbg !49
      ret void, !dbg !50
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

    define void @__user_init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__user_init_grandparent(%grandparent* %__grandparent)
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
    !1 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 304, align: 64, flags: DIFlagPublic, elements: !5, identifier: "parent")
    !5 = !{!6, !15, !19}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__grandparent", scope: !2, file: !2, baseType: !7, size: 112, align: 64, flags: DIFlagPublic)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 112, align: 64, flags: DIFlagPublic, elements: !8, identifier: "grandparent")
    !8 = !{!9, !14}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !10, size: 96, align: 16, flags: DIFlagPublic)
    !10 = !DICompositeType(tag: DW_TAG_array_type, baseType: !11, size: 96, align: 16, elements: !12)
    !11 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !12 = !{!13}
    !13 = !DISubrange(count: 6, lowerBound: 0)
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !11, size: 16, align: 16, offset: 96, flags: DIFlagPublic)
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !16, size: 176, align: 16, offset: 112, flags: DIFlagPublic)
    !16 = !DICompositeType(tag: DW_TAG_array_type, baseType: !11, size: 176, align: 16, elements: !17)
    !17 = !{!18}
    !18 = !DISubrange(count: 11, lowerBound: 0)
    !19 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !11, size: 16, align: 16, offset: 288, flags: DIFlagPublic)
    !20 = !DIGlobalVariableExpression(var: !21, expr: !DIExpression())
    !21 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !22, isLocal: false, isDefinition: true)
    !22 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !7)
    !23 = !DIGlobalVariableExpression(var: !24, expr: !DIExpression())
    !24 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !25, isLocal: false, isDefinition: true)
    !25 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !26)
    !26 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 480, align: 64, flags: DIFlagPublic, elements: !27, identifier: "child")
    !27 = !{!28, !29}
    !28 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !4, size: 304, align: 64, flags: DIFlagPublic)
    !29 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !16, size: 176, align: 16, offset: 304, flags: DIFlagPublic)
    !30 = !{i32 2, !"Dwarf Version", i32 5}
    !31 = !{i32 2, !"Debug Info Version", i32 3}
    !32 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !33, splitDebugInlining: false)
    !33 = !{!20, !0, !23}
    !34 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !35, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !32, retainedNodes: !37)
    !35 = !DISubroutineType(flags: DIFlagPublic, types: !36)
    !36 = !{null, !7}
    !37 = !{}
    !38 = !DILocalVariable(name: "grandparent", scope: !34, file: !2, line: 7, type: !7)
    !39 = !DILocation(line: 7, column: 8, scope: !34)
    !40 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !41, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !32, retainedNodes: !37)
    !41 = !DISubroutineType(flags: DIFlagPublic, types: !42)
    !42 = !{null, !4}
    !43 = !DILocalVariable(name: "parent", scope: !40, file: !2, line: 14, type: !4)
    !44 = !DILocation(line: 14, column: 8, scope: !40)
    !45 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !46, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !32, retainedNodes: !37)
    !46 = !DISubroutineType(flags: DIFlagPublic, types: !47)
    !47 = !{null, !26}
    !48 = !DILocalVariable(name: "child", scope: !45, file: !2, line: 20, type: !26)
    !49 = !DILocation(line: 20, column: 12, scope: !45)
    !50 = !DILocation(line: 21, column: 8, scope: !45)
    "###);
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
    filtered_assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type {}
    %bar = type { %foo }

    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !0
    @__bar__init = unnamed_addr constant %bar zeroinitializer, !dbg !6
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @foo(%foo* %0) !dbg !16 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !19, metadata !DIExpression()), !dbg !20
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      ret void, !dbg !20
    }

    define void @foo__baz(%foo* %0) !dbg !21 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !22, metadata !DIExpression()), !dbg !23
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      ret void, !dbg !23
    }

    define void @bar(%bar* %0) !dbg !24 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !27, metadata !DIExpression()), !dbg !28
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void, !dbg !28
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

    define void @__user_init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %__foo = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__user_init_foo(%foo* %__foo)
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
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

    !llvm.module.flags = !{!12, !13}
    !llvm.dbg.cu = !{!14}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, align: 64, flags: DIFlagPublic, elements: !5, identifier: "foo")
    !5 = !{}
    !6 = !DIGlobalVariableExpression(var: !7, expr: !DIExpression())
    !7 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 7, type: !8, isLocal: false, isDefinition: true)
    !8 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !9)
    !9 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 7, align: 64, flags: DIFlagPublic, elements: !10, identifier: "bar")
    !10 = !{!11}
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !4, align: 64, flags: DIFlagPublic)
    !12 = !{i32 2, !"Dwarf Version", i32 5}
    !13 = !{i32 2, !"Debug Info Version", i32 3}
    !14 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !15, splitDebugInlining: false)
    !15 = !{!0, !6}
    !16 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !17, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !14, retainedNodes: !5)
    !17 = !DISubroutineType(flags: DIFlagPublic, types: !18)
    !18 = !{null, !4}
    !19 = !DILocalVariable(name: "foo", scope: !16, file: !2, line: 5, type: !4)
    !20 = !DILocation(line: 5, column: 8, scope: !16)
    !21 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !16, file: !2, line: 3, type: !17, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !14, retainedNodes: !5)
    !22 = !DILocalVariable(name: "foo", scope: !21, file: !2, line: 4, type: !4)
    !23 = !DILocation(line: 4, column: 8, scope: !21)
    !24 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 7, type: !25, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !14, retainedNodes: !5)
    !25 = !DISubroutineType(flags: DIFlagPublic, types: !26)
    !26 = !{null, !9}
    !27 = !DILocalVariable(name: "bar", scope: !24, file: !2, line: 8, type: !9)
    !28 = !DILocation(line: 8, column: 8, scope: !24)
    "###);
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

    filtered_assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %grandchild = type { %child, i32 }
    %child = type { %parent, i32 }
    %parent = type { i32 }

    @__grandchild__init = unnamed_addr constant %grandchild zeroinitializer, !dbg !0
    @__child__init = unnamed_addr constant %child zeroinitializer, !dbg !16
    @__parent__init = unnamed_addr constant %parent zeroinitializer, !dbg !19
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @parent(%parent* %0) !dbg !26 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !30, metadata !DIExpression()), !dbg !31
      %this = alloca %parent*, align 8
      store %parent* %0, %parent** %this, align 8
      %a = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      ret void, !dbg !31
    }

    define void @child(%child* %0) !dbg !32 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !35, metadata !DIExpression()), !dbg !36
      %this = alloca %child*, align 8
      store %child* %0, %child** %this, align 8
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %b = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      ret void, !dbg !36
    }

    define void @grandchild(%grandchild* %0) !dbg !37 {
    entry:
      call void @llvm.dbg.declare(metadata %grandchild* %0, metadata !40, metadata !DIExpression()), !dbg !41
      %this = alloca %grandchild*, align 8
      store %grandchild* %0, %grandchild** %this, align 8
      %__child = getelementptr inbounds %grandchild, %grandchild* %0, i32 0, i32 0
      %c = getelementptr inbounds %grandchild, %grandchild* %0, i32 0, i32 1
      ret void, !dbg !41
    }

    define i32 @main() !dbg !42 {
    entry:
      %main = alloca i32, align 4
      %array_of_parent = alloca [3 x %parent], align 8
      %array_of_child = alloca [3 x %child], align 8
      %array_of_grandchild = alloca [3 x %grandchild], align 8
      %parent1 = alloca %parent, align 8
      %child1 = alloca %child, align 8
      %grandchild1 = alloca %grandchild, align 8
      call void @llvm.dbg.declare(metadata [3 x %parent]* %array_of_parent, metadata !45, metadata !DIExpression()), !dbg !49
      %0 = bitcast [3 x %parent]* %array_of_parent to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([3 x %parent]* getelementptr ([3 x %parent], [3 x %parent]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata [3 x %child]* %array_of_child, metadata !50, metadata !DIExpression()), !dbg !52
      %1 = bitcast [3 x %child]* %array_of_child to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %1, i8 0, i64 ptrtoint ([3 x %child]* getelementptr ([3 x %child], [3 x %child]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata [3 x %grandchild]* %array_of_grandchild, metadata !53, metadata !DIExpression()), !dbg !55
      %2 = bitcast [3 x %grandchild]* %array_of_grandchild to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %2, i8 0, i64 ptrtoint ([3 x %grandchild]* getelementptr ([3 x %grandchild], [3 x %grandchild]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %parent* %parent1, metadata !56, metadata !DIExpression()), !dbg !57
      %3 = bitcast %parent* %parent1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %3, i8* align 1 bitcast (%parent* @__parent__init to i8*), i64 ptrtoint (%parent* getelementptr (%parent, %parent* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %child* %child1, metadata !58, metadata !DIExpression()), !dbg !59
      %4 = bitcast %child* %child1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %4, i8* align 1 bitcast (%child* @__child__init to i8*), i64 ptrtoint (%child* getelementptr (%child, %child* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %grandchild* %grandchild1, metadata !60, metadata !DIExpression()), !dbg !61
      %5 = bitcast %grandchild* %grandchild1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %5, i8* align 1 bitcast (%grandchild* @__grandchild__init to i8*), i64 ptrtoint (%grandchild* getelementptr (%grandchild, %grandchild* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata i32* %main, metadata !62, metadata !DIExpression()), !dbg !63
      store i32 0, i32* %main, align 4
      call void @__init_parent(%parent* %parent1), !dbg !64
      call void @__init_child(%child* %child1), !dbg !64
      call void @__init_grandchild(%grandchild* %grandchild1), !dbg !64
      call void @__user_init_parent(%parent* %parent1), !dbg !64
      call void @__user_init_child(%child* %child1), !dbg !64
      call void @__user_init_grandchild(%grandchild* %grandchild1), !dbg !64
      %a = getelementptr inbounds %parent, %parent* %parent1, i32 0, i32 0, !dbg !65
      store i32 1, i32* %a, align 4, !dbg !65
      %__parent = getelementptr inbounds %child, %child* %child1, i32 0, i32 0, !dbg !66
      %a1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !66
      store i32 2, i32* %a1, align 4, !dbg !66
      %b = getelementptr inbounds %child, %child* %child1, i32 0, i32 1, !dbg !67
      store i32 3, i32* %b, align 4, !dbg !67
      %__child = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 0, !dbg !68
      %__parent2 = getelementptr inbounds %child, %child* %__child, i32 0, i32 0, !dbg !68
      %a3 = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0, !dbg !68
      store i32 4, i32* %a3, align 4, !dbg !68
      %__child4 = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 0, !dbg !69
      %b5 = getelementptr inbounds %child, %child* %__child4, i32 0, i32 1, !dbg !69
      store i32 5, i32* %b5, align 4, !dbg !69
      %c = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 1, !dbg !70
      store i32 6, i32* %c, align 4, !dbg !70
      %tmpVar = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 0, !dbg !71
      %a6 = getelementptr inbounds %parent, %parent* %tmpVar, i32 0, i32 0, !dbg !71
      store i32 7, i32* %a6, align 4, !dbg !71
      %tmpVar7 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 0, !dbg !72
      %__parent8 = getelementptr inbounds %child, %child* %tmpVar7, i32 0, i32 0, !dbg !72
      %a9 = getelementptr inbounds %parent, %parent* %__parent8, i32 0, i32 0, !dbg !72
      store i32 8, i32* %a9, align 4, !dbg !72
      %tmpVar10 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 0, !dbg !73
      %b11 = getelementptr inbounds %child, %child* %tmpVar10, i32 0, i32 1, !dbg !73
      store i32 9, i32* %b11, align 4, !dbg !73
      %tmpVar12 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !74
      %__child13 = getelementptr inbounds %grandchild, %grandchild* %tmpVar12, i32 0, i32 0, !dbg !74
      %__parent14 = getelementptr inbounds %child, %child* %__child13, i32 0, i32 0, !dbg !74
      %a15 = getelementptr inbounds %parent, %parent* %__parent14, i32 0, i32 0, !dbg !74
      store i32 10, i32* %a15, align 4, !dbg !74
      %tmpVar16 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !75
      %__child17 = getelementptr inbounds %grandchild, %grandchild* %tmpVar16, i32 0, i32 0, !dbg !75
      %b18 = getelementptr inbounds %child, %child* %__child17, i32 0, i32 1, !dbg !75
      store i32 11, i32* %b18, align 4, !dbg !75
      %tmpVar19 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !76
      %c20 = getelementptr inbounds %grandchild, %grandchild* %tmpVar19, i32 0, i32 1, !dbg !76
      store i32 12, i32* %c20, align 4, !dbg !76
      %tmpVar21 = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 1, !dbg !77
      %a22 = getelementptr inbounds %parent, %parent* %tmpVar21, i32 0, i32 0, !dbg !77
      store i32 13, i32* %a22, align 4, !dbg !77
      %tmpVar23 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 1, !dbg !78
      %__parent24 = getelementptr inbounds %child, %child* %tmpVar23, i32 0, i32 0, !dbg !78
      %a25 = getelementptr inbounds %parent, %parent* %__parent24, i32 0, i32 0, !dbg !78
      store i32 14, i32* %a25, align 4, !dbg !78
      %tmpVar26 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 1, !dbg !79
      %b27 = getelementptr inbounds %child, %child* %tmpVar26, i32 0, i32 1, !dbg !79
      store i32 15, i32* %b27, align 4, !dbg !79
      %tmpVar28 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !80
      %__child29 = getelementptr inbounds %grandchild, %grandchild* %tmpVar28, i32 0, i32 0, !dbg !80
      %__parent30 = getelementptr inbounds %child, %child* %__child29, i32 0, i32 0, !dbg !80
      %a31 = getelementptr inbounds %parent, %parent* %__parent30, i32 0, i32 0, !dbg !80
      store i32 16, i32* %a31, align 4, !dbg !80
      %tmpVar32 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !81
      %__child33 = getelementptr inbounds %grandchild, %grandchild* %tmpVar32, i32 0, i32 0, !dbg !81
      %b34 = getelementptr inbounds %child, %child* %__child33, i32 0, i32 1, !dbg !81
      store i32 17, i32* %b34, align 4, !dbg !81
      %tmpVar35 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !82
      %c36 = getelementptr inbounds %grandchild, %grandchild* %tmpVar35, i32 0, i32 1, !dbg !82
      store i32 18, i32* %c36, align 4, !dbg !82
      %tmpVar37 = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 2, !dbg !83
      %a38 = getelementptr inbounds %parent, %parent* %tmpVar37, i32 0, i32 0, !dbg !83
      store i32 19, i32* %a38, align 4, !dbg !83
      %tmpVar39 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 2, !dbg !84
      %__parent40 = getelementptr inbounds %child, %child* %tmpVar39, i32 0, i32 0, !dbg !84
      %a41 = getelementptr inbounds %parent, %parent* %__parent40, i32 0, i32 0, !dbg !84
      store i32 20, i32* %a41, align 4, !dbg !84
      %tmpVar42 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 2, !dbg !85
      %b43 = getelementptr inbounds %child, %child* %tmpVar42, i32 0, i32 1, !dbg !85
      store i32 21, i32* %b43, align 4, !dbg !85
      %tmpVar44 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !86
      %__child45 = getelementptr inbounds %grandchild, %grandchild* %tmpVar44, i32 0, i32 0, !dbg !86
      %__parent46 = getelementptr inbounds %child, %child* %__child45, i32 0, i32 0, !dbg !86
      %a47 = getelementptr inbounds %parent, %parent* %__parent46, i32 0, i32 0, !dbg !86
      store i32 22, i32* %a47, align 4, !dbg !86
      %tmpVar48 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !87
      %__child49 = getelementptr inbounds %grandchild, %grandchild* %tmpVar48, i32 0, i32 0, !dbg !87
      %b50 = getelementptr inbounds %child, %child* %__child49, i32 0, i32 1, !dbg !87
      store i32 23, i32* %b50, align 4, !dbg !87
      %tmpVar51 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !88
      %c52 = getelementptr inbounds %grandchild, %grandchild* %tmpVar51, i32 0, i32 1, !dbg !88
      store i32 24, i32* %c52, align 4, !dbg !88
      %main_ret = load i32, i32* %main, align 4, !dbg !89
      ret i32 %main_ret, !dbg !89
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #1

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #2

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

    define void @__user_init_grandchild(%grandchild* %0) {
    entry:
      %self = alloca %grandchild*, align 8
      store %grandchild* %0, %grandchild** %self, align 8
      %deref = load %grandchild*, %grandchild** %self, align 8
      %__child = getelementptr inbounds %grandchild, %grandchild* %deref, i32 0, i32 0
      call void @__user_init_child(%child* %__child)
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
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

    !llvm.module.flags = !{!22, !23}
    !llvm.dbg.cu = !{!24}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__grandchild__init", scope: !2, file: !2, line: 14, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandchild", scope: !2, file: !2, line: 14, size: 96, align: 64, flags: DIFlagPublic, elements: !5, identifier: "grandchild")
    !5 = !{!6, !15}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__child", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 8, size: 64, align: 64, flags: DIFlagPublic, elements: !8, identifier: "child")
    !8 = !{!9, !14}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !10, size: 32, align: 64, flags: DIFlagPublic)
    !10 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 2, size: 32, align: 64, flags: DIFlagPublic, elements: !11, identifier: "parent")
    !11 = !{!12}
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 4, baseType: !13, size: 32, align: 32, flags: DIFlagPublic)
    !13 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 10, baseType: !13, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 16, baseType: !13, size: 32, align: 32, offset: 64, flags: DIFlagPublic)
    !16 = !DIGlobalVariableExpression(var: !17, expr: !DIExpression())
    !17 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 8, type: !18, isLocal: false, isDefinition: true)
    !18 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !7)
    !19 = !DIGlobalVariableExpression(var: !20, expr: !DIExpression())
    !20 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 2, type: !21, isLocal: false, isDefinition: true)
    !21 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !10)
    !22 = !{i32 2, !"Dwarf Version", i32 5}
    !23 = !{i32 2, !"Debug Info Version", i32 3}
    !24 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !25, splitDebugInlining: false)
    !25 = !{!19, !16, !0}
    !26 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 2, type: !27, scopeLine: 6, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !24, retainedNodes: !29)
    !27 = !DISubroutineType(flags: DIFlagPublic, types: !28)
    !28 = !{null, !10}
    !29 = !{}
    !30 = !DILocalVariable(name: "parent", scope: !26, file: !2, line: 6, type: !10)
    !31 = !DILocation(line: 6, scope: !26)
    !32 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 8, type: !33, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !24, retainedNodes: !29)
    !33 = !DISubroutineType(flags: DIFlagPublic, types: !34)
    !34 = !{null, !7}
    !35 = !DILocalVariable(name: "child", scope: !32, file: !2, line: 12, type: !7)
    !36 = !DILocation(line: 12, scope: !32)
    !37 = distinct !DISubprogram(name: "grandchild", linkageName: "grandchild", scope: !2, file: !2, line: 14, type: !38, scopeLine: 18, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !24, retainedNodes: !29)
    !38 = !DISubroutineType(flags: DIFlagPublic, types: !39)
    !39 = !{null, !4}
    !40 = !DILocalVariable(name: "grandchild", scope: !37, file: !2, line: 18, type: !4)
    !41 = !DILocation(line: 18, scope: !37)
    !42 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 20, type: !43, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !24, retainedNodes: !29)
    !43 = !DISubroutineType(flags: DIFlagPublic, types: !44)
    !44 = !{null}
    !45 = !DILocalVariable(name: "array_of_parent", scope: !42, file: !2, line: 22, type: !46, align: 64)
    !46 = !DICompositeType(tag: DW_TAG_array_type, baseType: !10, size: 96, align: 64, elements: !47)
    !47 = !{!48}
    !48 = !DISubrange(count: 3, lowerBound: 0)
    !49 = !DILocation(line: 22, column: 4, scope: !42)
    !50 = !DILocalVariable(name: "array_of_child", scope: !42, file: !2, line: 23, type: !51, align: 64)
    !51 = !DICompositeType(tag: DW_TAG_array_type, baseType: !7, size: 192, align: 64, elements: !47)
    !52 = !DILocation(line: 23, column: 4, scope: !42)
    !53 = !DILocalVariable(name: "array_of_grandchild", scope: !42, file: !2, line: 24, type: !54, align: 64)
    !54 = !DICompositeType(tag: DW_TAG_array_type, baseType: !4, size: 288, align: 64, elements: !47)
    !55 = !DILocation(line: 24, column: 4, scope: !42)
    !56 = !DILocalVariable(name: "parent1", scope: !42, file: !2, line: 25, type: !10, align: 64)
    !57 = !DILocation(line: 25, column: 4, scope: !42)
    !58 = !DILocalVariable(name: "child1", scope: !42, file: !2, line: 26, type: !7, align: 64)
    !59 = !DILocation(line: 26, column: 4, scope: !42)
    !60 = !DILocalVariable(name: "grandchild1", scope: !42, file: !2, line: 27, type: !4, align: 64)
    !61 = !DILocation(line: 27, column: 4, scope: !42)
    !62 = !DILocalVariable(name: "main", scope: !42, file: !2, line: 20, type: !13, align: 32)
    !63 = !DILocation(line: 20, column: 9, scope: !42)
    !64 = !DILocation(line: 0, scope: !42)
    !65 = !DILocation(line: 30, column: 4, scope: !42)
    !66 = !DILocation(line: 31, column: 4, scope: !42)
    !67 = !DILocation(line: 32, column: 4, scope: !42)
    !68 = !DILocation(line: 33, column: 4, scope: !42)
    !69 = !DILocation(line: 34, column: 4, scope: !42)
    !70 = !DILocation(line: 35, column: 4, scope: !42)
    !71 = !DILocation(line: 37, column: 4, scope: !42)
    !72 = !DILocation(line: 38, column: 4, scope: !42)
    !73 = !DILocation(line: 39, column: 4, scope: !42)
    !74 = !DILocation(line: 40, column: 4, scope: !42)
    !75 = !DILocation(line: 41, column: 4, scope: !42)
    !76 = !DILocation(line: 42, column: 4, scope: !42)
    !77 = !DILocation(line: 43, column: 4, scope: !42)
    !78 = !DILocation(line: 44, column: 4, scope: !42)
    !79 = !DILocation(line: 45, column: 4, scope: !42)
    !80 = !DILocation(line: 46, column: 4, scope: !42)
    !81 = !DILocation(line: 47, column: 4, scope: !42)
    !82 = !DILocation(line: 48, column: 4, scope: !42)
    !83 = !DILocation(line: 49, column: 4, scope: !42)
    !84 = !DILocation(line: 50, column: 4, scope: !42)
    !85 = !DILocation(line: 51, column: 4, scope: !42)
    !86 = !DILocation(line: 52, column: 4, scope: !42)
    !87 = !DILocation(line: 53, column: 4, scope: !42)
    !88 = !DILocation(line: 54, column: 4, scope: !42)
    !89 = !DILocation(line: 56, scope: !42)
    "###);
}
