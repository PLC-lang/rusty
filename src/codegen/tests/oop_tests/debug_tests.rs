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
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { %foo* }
    %foo = type { i32*, i16, [81 x i8], [11 x [81 x i8]] }
    %__vtable_bar = type { %bar* }
    %bar = type { %foo }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !0
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @____vtable_bar__init = unnamed_addr constant %__vtable_bar zeroinitializer
    @__bar__init = unnamed_addr constant %bar zeroinitializer, !dbg !20
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer

    define void @foo(%foo* %0) !dbg !30 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !34, metadata !DIExpression()), !dbg !35
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %a = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %b = getelementptr inbounds %foo, %foo* %0, i32 0, i32 2
      %c = getelementptr inbounds %foo, %foo* %0, i32 0, i32 3
      ret void, !dbg !35
    }

    define void @bar(%bar* %0) !dbg !36 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !39, metadata !DIExpression()), !dbg !40
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void, !dbg !40
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
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_foo* @__vtable_foo_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %__foo = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %__foo)
      %deref1 = load %bar*, %bar** %self, align 8
      %__foo2 = getelementptr inbounds %bar, %bar* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %foo, %foo* %__foo2, i32 0, i32 0
      store i32* bitcast (%__vtable_bar* @__vtable_bar_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_bar(%__vtable_bar* %0) {
    entry:
      %self = alloca %__vtable_bar*, align 8
      store %__vtable_bar* %0, %__vtable_bar** %self, align 8
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

    define void @__user_init___vtable_foo(%__vtable_foo* %0) {
    entry:
      %self = alloca %__vtable_foo*, align 8
      store %__vtable_foo* %0, %__vtable_foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(%__vtable_foo* @__vtable_foo_instance)
      call void @__init___vtable_bar(%__vtable_bar* @__vtable_bar_instance)
      call void @__user_init___vtable_foo(%__vtable_foo* @__vtable_foo_instance)
      call void @__user_init___vtable_bar(%__vtable_bar* @__vtable_bar_instance)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!26, !27}
    !llvm.dbg.cu = !{!28}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 7872, align: 64, flags: DIFlagPublic, elements: !5, identifier: "foo")
    !5 = !{!6, !9, !11, !16}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__foo___vtable", baseType: !8, size: 64, align: 64, dwarfAddressSpace: 1)
    !8 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 4, baseType: !10, size: 16, align: 16, offset: 64, flags: DIFlagPublic)
    !10 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 5, baseType: !12, size: 648, align: 8, offset: 80, flags: DIFlagPublic)
    !12 = !DICompositeType(tag: DW_TAG_array_type, baseType: !13, size: 648, align: 8, elements: !14)
    !13 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !14 = !{!15}
    !15 = !DISubrange(count: 81, lowerBound: 0)
    !16 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 6, baseType: !17, size: 7128, align: 8, offset: 728, flags: DIFlagPublic)
    !17 = !DICompositeType(tag: DW_TAG_array_type, baseType: !12, size: 7128, align: 8, elements: !18)
    !18 = !{!19}
    !19 = !DISubrange(count: 11, lowerBound: 0)
    !20 = !DIGlobalVariableExpression(var: !21, expr: !DIExpression())
    !21 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 10, type: !22, isLocal: false, isDefinition: true)
    !22 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !23)
    !23 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 10, size: 7872, align: 64, flags: DIFlagPublic, elements: !24, identifier: "bar")
    !24 = !{!25}
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !4, size: 7872, align: 64, flags: DIFlagPublic)
    !26 = !{i32 2, !"Dwarf Version", i32 5}
    !27 = !{i32 2, !"Debug Info Version", i32 3}
    !28 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !29, splitDebugInlining: false)
    !29 = !{!0, !20}
    !30 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !31, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !28, retainedNodes: !33)
    !31 = !DISubroutineType(flags: DIFlagPublic, types: !32)
    !32 = !{null, !4}
    !33 = !{}
    !34 = !DILocalVariable(name: "foo", scope: !30, file: !2, line: 8, type: !4)
    !35 = !DILocation(line: 8, column: 8, scope: !30)
    !36 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 10, type: !37, scopeLine: 11, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !28, retainedNodes: !33)
    !37 = !DISubroutineType(flags: DIFlagPublic, types: !38)
    !38 = !{null, !23}
    !39 = !DILocalVariable(name: "bar", scope: !36, file: !2, line: 11, type: !23)
    !40 = !DILocation(line: 11, column: 8, scope: !36)
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

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_fb = type { %fb* }
    %fb = type { i32*, i16, i16 }
    %__vtable_fb2 = type { %fb2* }
    %fb2 = type { %fb }
    %__vtable_foo = type { %foo* }
    %foo = type { i32*, %fb2 }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_fb__init = unnamed_addr constant %__vtable_fb zeroinitializer
    @__fb__init = unnamed_addr constant %fb zeroinitializer, !dbg !0
    @__vtable_fb_instance = global %__vtable_fb zeroinitializer
    @____vtable_fb2__init = unnamed_addr constant %__vtable_fb2 zeroinitializer
    @__fb2__init = unnamed_addr constant %fb2 zeroinitializer, !dbg !12
    @__vtable_fb2_instance = global %__vtable_fb2 zeroinitializer
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !18
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @fb(%fb* %0) !dbg !30 {
    entry:
      call void @llvm.dbg.declare(metadata %fb* %0, metadata !34, metadata !DIExpression()), !dbg !35
      %this = alloca %fb*, align 8
      store %fb* %0, %fb** %this, align 8
      %__vtable = getelementptr inbounds %fb, %fb* %0, i32 0, i32 0
      %x = getelementptr inbounds %fb, %fb* %0, i32 0, i32 1
      %y = getelementptr inbounds %fb, %fb* %0, i32 0, i32 2
      ret void, !dbg !35
    }

    define void @fb2(%fb2* %0) !dbg !36 {
    entry:
      call void @llvm.dbg.declare(metadata %fb2* %0, metadata !39, metadata !DIExpression()), !dbg !40
      %this = alloca %fb2*, align 8
      store %fb2* %0, %fb2** %this, align 8
      %__fb = getelementptr inbounds %fb2, %fb2* %0, i32 0, i32 0
      ret void, !dbg !40
    }

    define void @foo(%foo* %0) !dbg !41 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !44, metadata !DIExpression()), !dbg !45
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %myFb = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %__fb = getelementptr inbounds %fb2, %fb2* %myFb, i32 0, i32 0, !dbg !45
      %x = getelementptr inbounds %fb, %fb* %__fb, i32 0, i32 1, !dbg !45
      store i16 1, i16* %x, align 2, !dbg !45
      ret void, !dbg !46
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
      %deref1 = load %fb2*, %fb2** %self, align 8
      %__fb2 = getelementptr inbounds %fb2, %fb2* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %fb, %fb* %__fb2, i32 0, i32 0
      store i32* bitcast (%__vtable_fb2* @__vtable_fb2_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_fb(%fb* %0) {
    entry:
      %self = alloca %fb*, align 8
      store %fb* %0, %fb** %self, align 8
      %deref = load %fb*, %fb** %self, align 8
      %__vtable = getelementptr inbounds %fb, %fb* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_fb* @__vtable_fb_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %myFb = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 1
      call void @__init_fb2(%fb2* %myFb)
      %deref1 = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref1, i32 0, i32 0
      store i32* bitcast (%__vtable_foo* @__vtable_foo_instance to i32*), i32** %__vtable, align 8
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

    define void @__user_init___vtable_fb(%__vtable_fb* %0) {
    entry:
      %self = alloca %__vtable_fb*, align 8
      store %__vtable_fb* %0, %__vtable_fb** %self, align 8
      ret void
    }

    define void @__user_init___vtable_fb2(%__vtable_fb2* %0) {
    entry:
      %self = alloca %__vtable_fb2*, align 8
      store %__vtable_fb2* %0, %__vtable_fb2** %self, align 8
      ret void
    }

    define void @__user_init___vtable_foo(%__vtable_foo* %0) {
    entry:
      %self = alloca %__vtable_foo*, align 8
      store %__vtable_foo* %0, %__vtable_foo** %self, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %myFb = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 1
      call void @__user_init_fb2(%fb2* %myFb)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_fb(%__vtable_fb* @__vtable_fb_instance)
      call void @__init___vtable_fb2(%__vtable_fb2* @__vtable_fb2_instance)
      call void @__init___vtable_foo(%__vtable_foo* @__vtable_foo_instance)
      call void @__user_init___vtable_fb(%__vtable_fb* @__vtable_fb_instance)
      call void @__user_init___vtable_fb2(%__vtable_fb2* @__vtable_fb2_instance)
      call void @__user_init___vtable_foo(%__vtable_foo* @__vtable_foo_instance)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!26, !27}
    !llvm.dbg.cu = !{!28}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__fb__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb", scope: !2, file: !2, line: 2, size: 128, align: 64, flags: DIFlagPublic, elements: !5, identifier: "fb")
    !5 = !{!6, !9, !11}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__fb___vtable", baseType: !8, size: 64, align: 64, dwarfAddressSpace: 1)
    !8 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 4, baseType: !10, size: 16, align: 16, offset: 64, flags: DIFlagPublic)
    !10 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 5, baseType: !10, size: 16, align: 16, offset: 80, flags: DIFlagPublic)
    !12 = !DIGlobalVariableExpression(var: !13, expr: !DIExpression())
    !13 = distinct !DIGlobalVariable(name: "__fb2__init", scope: !2, file: !2, line: 9, type: !14, isLocal: false, isDefinition: true)
    !14 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !15)
    !15 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb2", scope: !2, file: !2, line: 9, size: 128, align: 64, flags: DIFlagPublic, elements: !16, identifier: "fb2")
    !16 = !{!17}
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "__fb", scope: !2, file: !2, baseType: !4, size: 128, align: 64, flags: DIFlagPublic)
    !18 = !DIGlobalVariableExpression(var: !19, expr: !DIExpression())
    !19 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 12, type: !20, isLocal: false, isDefinition: true)
    !20 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !21)
    !21 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 12, size: 192, align: 64, flags: DIFlagPublic, elements: !22, identifier: "foo")
    !22 = !{!23, !25}
    !23 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !24, size: 64, align: 64, flags: DIFlagPublic)
    !24 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__foo___vtable", baseType: !8, size: 64, align: 64, dwarfAddressSpace: 1)
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "myFb", scope: !2, file: !2, line: 14, baseType: !15, size: 128, align: 64, offset: 64, flags: DIFlagPublic)
    !26 = !{i32 2, !"Dwarf Version", i32 5}
    !27 = !{i32 2, !"Debug Info Version", i32 3}
    !28 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !29, splitDebugInlining: false)
    !29 = !{!0, !12, !18}
    !30 = distinct !DISubprogram(name: "fb", linkageName: "fb", scope: !2, file: !2, line: 2, type: !31, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !28, retainedNodes: !33)
    !31 = !DISubroutineType(flags: DIFlagPublic, types: !32)
    !32 = !{null, !4}
    !33 = !{}
    !34 = !DILocalVariable(name: "fb", scope: !30, file: !2, line: 7, type: !4)
    !35 = !DILocation(line: 7, column: 8, scope: !30)
    !36 = distinct !DISubprogram(name: "fb2", linkageName: "fb2", scope: !2, file: !2, line: 9, type: !37, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !28, retainedNodes: !33)
    !37 = !DISubroutineType(flags: DIFlagPublic, types: !38)
    !38 = !{null, !15}
    !39 = !DILocalVariable(name: "fb2", scope: !36, file: !2, line: 10, type: !15)
    !40 = !DILocation(line: 10, column: 8, scope: !36)
    !41 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 12, type: !42, scopeLine: 16, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !28, retainedNodes: !33)
    !42 = !DISubroutineType(flags: DIFlagPublic, types: !43)
    !43 = !{null, !21}
    !44 = !DILocalVariable(name: "foo", scope: !41, file: !2, line: 16, type: !21)
    !45 = !DILocation(line: 16, column: 12, scope: !41)
    !46 = !DILocation(line: 17, column: 8, scope: !41)
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
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { %foo*, void (%foo*)* }
    %foo = type { i32*, [81 x i8] }
    %__vtable_bar = type { %bar*, void (%foo*)* }
    %bar = type { %foo }

    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"world\00"
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !0
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @____vtable_bar__init = unnamed_addr constant %__vtable_bar zeroinitializer
    @__bar__init = unnamed_addr constant %bar zeroinitializer, !dbg !14
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer

    define void @foo(%foo* %0) !dbg !24 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !28, metadata !DIExpression()), !dbg !29
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      ret void, !dbg !29
    }

    define void @foo__baz(%foo* %0) !dbg !30 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !31, metadata !DIExpression()), !dbg !32
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %1 = bitcast [81 x i8]* %s to i8*, !dbg !32
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_0, i32 0, i32 0), i32 6, i1 false), !dbg !32
      ret void, !dbg !33
    }

    define void @bar(%bar* %0) !dbg !34 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !37, metadata !DIExpression()), !dbg !38
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %__foo, i32 0, i32 1, !dbg !38
      %1 = bitcast [81 x i8]* %s to i8*, !dbg !38
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_1, i32 0, i32 0), i32 6, i1 false), !dbg !38
      ret void, !dbg !39
    }

    define void @main() !dbg !40 {
    entry:
      %s = alloca [81 x i8], align 1
      %fb = alloca %bar, align 8
      call void @llvm.dbg.declare(metadata [81 x i8]* %s, metadata !43, metadata !DIExpression()), !dbg !44
      %0 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %bar* %fb, metadata !45, metadata !DIExpression()), !dbg !46
      %1 = bitcast %bar* %fb to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 bitcast (%bar* @__bar__init to i8*), i64 ptrtoint (%bar* getelementptr (%bar, %bar* null, i32 1) to i64), i1 false)
      call void @__init_bar(%bar* %fb), !dbg !47
      call void @__user_init_bar(%bar* %fb), !dbg !47
      %__foo = getelementptr inbounds %bar, %bar* %fb, i32 0, i32 0, !dbg !47
      call void @foo__baz(%foo* %__foo), !dbg !48
      call void @bar(%bar* %fb), !dbg !49
      ret void, !dbg !50
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
      %deref = load %__vtable_foo*, %__vtable_foo** %self, align 8
      %baz = getelementptr inbounds %__vtable_foo, %__vtable_foo* %deref, i32 0, i32 1
      store void (%foo*)* @foo__baz, void (%foo*)** %baz, align 8
      ret void
    }

    define void @__init___vtable_bar(%__vtable_bar* %0) {
    entry:
      %self = alloca %__vtable_bar*, align 8
      store %__vtable_bar* %0, %__vtable_bar** %self, align 8
      %deref = load %__vtable_bar*, %__vtable_bar** %self, align 8
      %baz = getelementptr inbounds %__vtable_bar, %__vtable_bar* %deref, i32 0, i32 1
      store void (%foo*)* @foo__baz, void (%foo*)** %baz, align 8
      ret void
    }

    define void @__init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %__foo = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %__foo)
      %deref1 = load %bar*, %bar** %self, align 8
      %__foo2 = getelementptr inbounds %bar, %bar* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %foo, %foo* %__foo2, i32 0, i32 0
      store i32* bitcast (%__vtable_bar* @__vtable_bar_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_foo* @__vtable_foo_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_bar(%__vtable_bar* %0) {
    entry:
      %self = alloca %__vtable_bar*, align 8
      store %__vtable_bar* %0, %__vtable_bar** %self, align 8
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

    define void @__user_init___vtable_foo(%__vtable_foo* %0) {
    entry:
      %self = alloca %__vtable_foo*, align 8
      store %__vtable_foo* %0, %__vtable_foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(%__vtable_foo* @__vtable_foo_instance)
      call void @__init___vtable_bar(%__vtable_bar* @__vtable_bar_instance)
      call void @__user_init___vtable_foo(%__vtable_foo* @__vtable_foo_instance)
      call void @__user_init___vtable_bar(%__vtable_bar* @__vtable_bar_instance)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn }
    attributes #2 = { argmemonly nofree nounwind willreturn writeonly }

    !llvm.module.flags = !{!20, !21}
    !llvm.dbg.cu = !{!22}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 768, align: 64, flags: DIFlagPublic, elements: !5, identifier: "foo")
    !5 = !{!6, !9}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__foo___vtable", baseType: !8, size: 64, align: 64, dwarfAddressSpace: 1)
    !8 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !2, file: !2, line: 4, baseType: !10, size: 648, align: 8, offset: 64, flags: DIFlagPublic)
    !10 = !DICompositeType(tag: DW_TAG_array_type, baseType: !11, size: 648, align: 8, elements: !12)
    !11 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !12 = !{!13}
    !13 = !DISubrange(count: 81, lowerBound: 0)
    !14 = !DIGlobalVariableExpression(var: !15, expr: !DIExpression())
    !15 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 11, type: !16, isLocal: false, isDefinition: true)
    !16 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !17)
    !17 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 11, size: 768, align: 64, flags: DIFlagPublic, elements: !18, identifier: "bar")
    !18 = !{!19}
    !19 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !4, size: 768, align: 64, flags: DIFlagPublic)
    !20 = !{i32 2, !"Dwarf Version", i32 5}
    !21 = !{i32 2, !"Debug Info Version", i32 3}
    !22 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !23, splitDebugInlining: false)
    !23 = !{!0, !14}
    !24 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !25, scopeLine: 9, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !22, retainedNodes: !27)
    !25 = !DISubroutineType(flags: DIFlagPublic, types: !26)
    !26 = !{null, !4}
    !27 = !{}
    !28 = !DILocalVariable(name: "foo", scope: !24, file: !2, line: 9, type: !4)
    !29 = !DILocation(line: 9, column: 8, scope: !24)
    !30 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !24, file: !2, line: 6, type: !25, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !22, retainedNodes: !27)
    !31 = !DILocalVariable(name: "foo", scope: !30, file: !2, line: 7, type: !4)
    !32 = !DILocation(line: 7, column: 12, scope: !30)
    !33 = !DILocation(line: 8, column: 8, scope: !30)
    !34 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 11, type: !35, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !22, retainedNodes: !27)
    !35 = !DISubroutineType(flags: DIFlagPublic, types: !36)
    !36 = !{null, !17}
    !37 = !DILocalVariable(name: "bar", scope: !34, file: !2, line: 12, type: !17)
    !38 = !DILocation(line: 12, column: 12, scope: !34)
    !39 = !DILocation(line: 13, column: 8, scope: !34)
    !40 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 15, type: !41, scopeLine: 15, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !22, retainedNodes: !27)
    !41 = !DISubroutineType(flags: DIFlagPublic, types: !42)
    !42 = !{null}
    !43 = !DILocalVariable(name: "s", scope: !40, file: !2, line: 17, type: !10, align: 8)
    !44 = !DILocation(line: 17, column: 12, scope: !40)
    !45 = !DILocalVariable(name: "fb", scope: !40, file: !2, line: 18, type: !17, align: 64)
    !46 = !DILocation(line: 18, column: 12, scope: !40)
    !47 = !DILocation(line: 0, scope: !40)
    !48 = !DILocation(line: 20, column: 12, scope: !40)
    !49 = !DILocation(line: 21, column: 12, scope: !40)
    !50 = !DILocation(line: 22, column: 8, scope: !40)
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
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_grandparent = type { %grandparent* }
    %grandparent = type { i32*, [6 x i16], i16 }
    %__vtable_parent = type { %parent* }
    %parent = type { %grandparent, [11 x i16], i16 }
    %__vtable_child = type { %child* }
    %child = type { %parent, [11 x i16] }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_grandparent__init = unnamed_addr constant %__vtable_grandparent zeroinitializer
    @__grandparent__init = unnamed_addr constant %grandparent zeroinitializer, !dbg !0
    @__vtable_grandparent_instance = global %__vtable_grandparent zeroinitializer
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent zeroinitializer, !dbg !15
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child zeroinitializer, !dbg !26
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @grandparent(%grandparent* %0) !dbg !37 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !41, metadata !DIExpression()), !dbg !42
      %this = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %this, align 8
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 2
      ret void, !dbg !42
    }

    define void @parent(%parent* %0) !dbg !43 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !46, metadata !DIExpression()), !dbg !47
      %this = alloca %parent*, align 8
      store %parent* %0, %parent** %this, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void, !dbg !47
    }

    define void @child(%child* %0) !dbg !48 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !51, metadata !DIExpression()), !dbg !52
      %this = alloca %child*, align 8
      store %child* %0, %child** %this, align 8
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      ret void, !dbg !52
    }

    define void @main() !dbg !53 {
    entry:
      %arr = alloca [11 x %child], align 8
      call void @llvm.dbg.declare(metadata [11 x %child]* %arr, metadata !56, metadata !DIExpression()), !dbg !58
      %0 = bitcast [11 x %child]* %arr to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([11 x %child]* getelementptr ([11 x %child], [11 x %child]* null, i32 1) to i64), i1 false)
      %tmpVar = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !59
      %__parent = getelementptr inbounds %child, %child* %tmpVar, i32 0, i32 0, !dbg !59
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !59
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 2, !dbg !59
      store i16 10, i16* %a, align 2, !dbg !59
      %tmpVar1 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !60
      %__parent2 = getelementptr inbounds %child, %child* %tmpVar1, i32 0, i32 0, !dbg !60
      %__grandparent3 = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0, !dbg !60
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent3, i32 0, i32 1, !dbg !60
      %tmpVar4 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 0, !dbg !60
      store i16 20, i16* %tmpVar4, align 2, !dbg !60
      %tmpVar5 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !61
      %__parent6 = getelementptr inbounds %child, %child* %tmpVar5, i32 0, i32 0, !dbg !61
      %b = getelementptr inbounds %parent, %parent* %__parent6, i32 0, i32 2, !dbg !61
      store i16 30, i16* %b, align 2, !dbg !61
      %tmpVar7 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !62
      %__parent8 = getelementptr inbounds %child, %child* %tmpVar7, i32 0, i32 0, !dbg !62
      %x = getelementptr inbounds %parent, %parent* %__parent8, i32 0, i32 1, !dbg !62
      %tmpVar9 = getelementptr inbounds [11 x i16], [11 x i16]* %x, i32 0, i32 1, !dbg !62
      store i16 40, i16* %tmpVar9, align 2, !dbg !62
      %tmpVar10 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 2, !dbg !63
      %z = getelementptr inbounds %child, %child* %tmpVar10, i32 0, i32 1, !dbg !63
      %tmpVar11 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 2, !dbg !63
      store i16 50, i16* %tmpVar11, align 2, !dbg !63
      ret void, !dbg !64
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
      ret void
    }

    define void @__init___vtable_child(%__vtable_child* %0) {
    entry:
      %self = alloca %__vtable_child*, align 8
      store %__vtable_child* %0, %__vtable_child** %self, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      %deref1 = load %child*, %child** %self, align 8
      %__parent2 = getelementptr inbounds %child, %child* %deref1, i32 0, i32 0
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 0
      store i32* bitcast (%__vtable_child* @__vtable_child_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__init_grandparent(%grandparent* %__grandparent)
      %deref1 = load %parent*, %parent** %self, align 8
      %__grandparent2 = getelementptr inbounds %parent, %parent* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %__grandparent2, i32 0, i32 0
      store i32* bitcast (%__vtable_parent* @__vtable_parent_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      %deref = load %grandparent*, %grandparent** %self, align 8
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_grandparent* @__vtable_grandparent_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_parent(%__vtable_parent* %0) {
    entry:
      %self = alloca %__vtable_parent*, align 8
      store %__vtable_parent* %0, %__vtable_parent** %self, align 8
      ret void
    }

    define void @__user_init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(%__vtable_child* %0) {
    entry:
      %self = alloca %__vtable_child*, align 8
      store %__vtable_child* %0, %__vtable_child** %self, align 8
      ret void
    }

    define void @__user_init___vtable_grandparent(%__vtable_grandparent* %0) {
    entry:
      %self = alloca %__vtable_grandparent*, align 8
      store %__vtable_grandparent* %0, %__vtable_grandparent** %self, align 8
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
      call void @__init___vtable_grandparent(%__vtable_grandparent* @__vtable_grandparent_instance)
      call void @__init___vtable_parent(%__vtable_parent* @__vtable_parent_instance)
      call void @__init___vtable_child(%__vtable_child* @__vtable_child_instance)
      call void @__user_init___vtable_grandparent(%__vtable_grandparent* @__vtable_grandparent_instance)
      call void @__user_init___vtable_parent(%__vtable_parent* @__vtable_parent_instance)
      call void @__user_init___vtable_child(%__vtable_child* @__vtable_child_instance)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn writeonly }

    !llvm.module.flags = !{!33, !34}
    !llvm.dbg.cu = !{!35}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 192, align: 64, flags: DIFlagPublic, elements: !5, identifier: "grandparent")
    !5 = !{!6, !9, !14}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__grandparent___vtable", baseType: !8, size: 64, align: 64, dwarfAddressSpace: 1)
    !8 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !10, size: 96, align: 16, offset: 64, flags: DIFlagPublic)
    !10 = !DICompositeType(tag: DW_TAG_array_type, baseType: !11, size: 96, align: 16, elements: !12)
    !11 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !12 = !{!13}
    !13 = !DISubrange(count: 6, lowerBound: 0)
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !11, size: 16, align: 16, offset: 160, flags: DIFlagPublic)
    !15 = !DIGlobalVariableExpression(var: !16, expr: !DIExpression())
    !16 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !17, isLocal: false, isDefinition: true)
    !17 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !18)
    !18 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 384, align: 64, flags: DIFlagPublic, elements: !19, identifier: "parent")
    !19 = !{!20, !21, !25}
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "__grandparent", scope: !2, file: !2, baseType: !4, size: 192, align: 64, flags: DIFlagPublic)
    !21 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !22, size: 176, align: 16, offset: 192, flags: DIFlagPublic)
    !22 = !DICompositeType(tag: DW_TAG_array_type, baseType: !11, size: 176, align: 16, elements: !23)
    !23 = !{!24}
    !24 = !DISubrange(count: 11, lowerBound: 0)
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !11, size: 16, align: 16, offset: 368, flags: DIFlagPublic)
    !26 = !DIGlobalVariableExpression(var: !27, expr: !DIExpression())
    !27 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !28, isLocal: false, isDefinition: true)
    !28 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !29)
    !29 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 576, align: 64, flags: DIFlagPublic, elements: !30, identifier: "child")
    !30 = !{!31, !32}
    !31 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !18, size: 384, align: 64, flags: DIFlagPublic)
    !32 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !22, size: 176, align: 16, offset: 384, flags: DIFlagPublic)
    !33 = !{i32 2, !"Dwarf Version", i32 5}
    !34 = !{i32 2, !"Debug Info Version", i32 3}
    !35 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !36, splitDebugInlining: false)
    !36 = !{!0, !15, !26}
    !37 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !38, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !40)
    !38 = !DISubroutineType(flags: DIFlagPublic, types: !39)
    !39 = !{null, !4}
    !40 = !{}
    !41 = !DILocalVariable(name: "grandparent", scope: !37, file: !2, line: 7, type: !4)
    !42 = !DILocation(line: 7, column: 8, scope: !37)
    !43 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !44, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !40)
    !44 = !DISubroutineType(flags: DIFlagPublic, types: !45)
    !45 = !{null, !18}
    !46 = !DILocalVariable(name: "parent", scope: !43, file: !2, line: 14, type: !18)
    !47 = !DILocation(line: 14, column: 8, scope: !43)
    !48 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !49, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !40)
    !49 = !DISubroutineType(flags: DIFlagPublic, types: !50)
    !50 = !{null, !29}
    !51 = !DILocalVariable(name: "child", scope: !48, file: !2, line: 20, type: !29)
    !52 = !DILocation(line: 20, column: 8, scope: !48)
    !53 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 22, type: !54, scopeLine: 26, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !40)
    !54 = !DISubroutineType(flags: DIFlagPublic, types: !55)
    !55 = !{null}
    !56 = !DILocalVariable(name: "arr", scope: !53, file: !2, line: 24, type: !57, align: 64)
    !57 = !DICompositeType(tag: DW_TAG_array_type, baseType: !29, size: 6336, align: 64, elements: !23)
    !58 = !DILocation(line: 24, column: 12, scope: !53)
    !59 = !DILocation(line: 26, column: 12, scope: !53)
    !60 = !DILocation(line: 27, column: 12, scope: !53)
    !61 = !DILocation(line: 28, column: 12, scope: !53)
    !62 = !DILocation(line: 29, column: 12, scope: !53)
    !63 = !DILocation(line: 30, column: 12, scope: !53)
    !64 = !DILocation(line: 31, column: 8, scope: !53)
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

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_grandparent = type { %grandparent* }
    %grandparent = type { i32*, [6 x i16], i16 }
    %__vtable_parent = type { %parent* }
    %parent = type { %grandparent, [11 x i16], i16 }
    %__vtable_child = type { %child* }
    %child = type { %parent, [11 x i16] }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_grandparent__init = unnamed_addr constant %__vtable_grandparent zeroinitializer
    @__grandparent__init = unnamed_addr constant %grandparent zeroinitializer, !dbg !0
    @__vtable_grandparent_instance = global %__vtable_grandparent zeroinitializer
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent zeroinitializer, !dbg !15
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child zeroinitializer, !dbg !26
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @grandparent(%grandparent* %0) !dbg !37 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !41, metadata !DIExpression()), !dbg !42
      %this = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %this, align 8
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 2
      ret void, !dbg !42
    }

    define void @parent(%parent* %0) !dbg !43 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !46, metadata !DIExpression()), !dbg !47
      %this = alloca %parent*, align 8
      store %parent* %0, %parent** %this, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void, !dbg !47
    }

    define void @child(%child* %0) !dbg !48 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !51, metadata !DIExpression()), !dbg !52
      %this = alloca %child*, align 8
      store %child* %0, %child** %this, align 8
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !52
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 1, !dbg !52
      %b = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2, !dbg !52
      %load_b = load i16, i16* %b, align 2, !dbg !52
      %1 = sext i16 %load_b to i32, !dbg !52
      %b1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2, !dbg !52
      %load_b2 = load i16, i16* %b1, align 2, !dbg !52
      %2 = sext i16 %load_b2 to i32, !dbg !52
      %tmpVar = mul i32 %2, 2, !dbg !52
      %tmpVar3 = mul i32 1, %tmpVar, !dbg !52
      %tmpVar4 = add i32 %tmpVar3, 0, !dbg !52
      %tmpVar5 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 %tmpVar4, !dbg !52
      %load_tmpVar = load i16, i16* %tmpVar5, align 2, !dbg !52
      %3 = sext i16 %load_tmpVar to i32, !dbg !52
      %tmpVar6 = add i32 %1, %3, !dbg !52
      %__grandparent7 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !52
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent7, i32 0, i32 2, !dbg !52
      %load_a = load i16, i16* %a, align 2, !dbg !52
      %4 = sext i16 %load_a to i32, !dbg !52
      %tmpVar8 = sub i32 %tmpVar6, %4, !dbg !52
      %tmpVar9 = mul i32 1, %tmpVar8, !dbg !52
      %tmpVar10 = add i32 %tmpVar9, 0, !dbg !52
      %tmpVar11 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 %tmpVar10, !dbg !52
      store i16 20, i16* %tmpVar11, align 2, !dbg !52
      ret void, !dbg !53
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
      ret void
    }

    define void @__init___vtable_child(%__vtable_child* %0) {
    entry:
      %self = alloca %__vtable_child*, align 8
      store %__vtable_child* %0, %__vtable_child** %self, align 8
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__init_grandparent(%grandparent* %__grandparent)
      %deref1 = load %parent*, %parent** %self, align 8
      %__grandparent2 = getelementptr inbounds %parent, %parent* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %__grandparent2, i32 0, i32 0
      store i32* bitcast (%__vtable_parent* @__vtable_parent_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      %deref = load %grandparent*, %grandparent** %self, align 8
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_grandparent* @__vtable_grandparent_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      %deref1 = load %child*, %child** %self, align 8
      %__parent2 = getelementptr inbounds %child, %child* %deref1, i32 0, i32 0
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 0
      store i32* bitcast (%__vtable_child* @__vtable_child_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_parent(%__vtable_parent* %0) {
    entry:
      %self = alloca %__vtable_parent*, align 8
      store %__vtable_parent* %0, %__vtable_parent** %self, align 8
      ret void
    }

    define void @__user_init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(%__vtable_child* %0) {
    entry:
      %self = alloca %__vtable_child*, align 8
      store %__vtable_child* %0, %__vtable_child** %self, align 8
      ret void
    }

    define void @__user_init___vtable_grandparent(%__vtable_grandparent* %0) {
    entry:
      %self = alloca %__vtable_grandparent*, align 8
      store %__vtable_grandparent* %0, %__vtable_grandparent** %self, align 8
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
      call void @__init___vtable_grandparent(%__vtable_grandparent* @__vtable_grandparent_instance)
      call void @__init___vtable_parent(%__vtable_parent* @__vtable_parent_instance)
      call void @__init___vtable_child(%__vtable_child* @__vtable_child_instance)
      call void @__user_init___vtable_grandparent(%__vtable_grandparent* @__vtable_grandparent_instance)
      call void @__user_init___vtable_parent(%__vtable_parent* @__vtable_parent_instance)
      call void @__user_init___vtable_child(%__vtable_child* @__vtable_child_instance)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!33, !34}
    !llvm.dbg.cu = !{!35}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 192, align: 64, flags: DIFlagPublic, elements: !5, identifier: "grandparent")
    !5 = !{!6, !9, !14}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__grandparent___vtable", baseType: !8, size: 64, align: 64, dwarfAddressSpace: 1)
    !8 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !10, size: 96, align: 16, offset: 64, flags: DIFlagPublic)
    !10 = !DICompositeType(tag: DW_TAG_array_type, baseType: !11, size: 96, align: 16, elements: !12)
    !11 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !12 = !{!13}
    !13 = !DISubrange(count: 6, lowerBound: 0)
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !11, size: 16, align: 16, offset: 160, flags: DIFlagPublic)
    !15 = !DIGlobalVariableExpression(var: !16, expr: !DIExpression())
    !16 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !17, isLocal: false, isDefinition: true)
    !17 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !18)
    !18 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 384, align: 64, flags: DIFlagPublic, elements: !19, identifier: "parent")
    !19 = !{!20, !21, !25}
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "__grandparent", scope: !2, file: !2, baseType: !4, size: 192, align: 64, flags: DIFlagPublic)
    !21 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !22, size: 176, align: 16, offset: 192, flags: DIFlagPublic)
    !22 = !DICompositeType(tag: DW_TAG_array_type, baseType: !11, size: 176, align: 16, elements: !23)
    !23 = !{!24}
    !24 = !DISubrange(count: 11, lowerBound: 0)
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !11, size: 16, align: 16, offset: 368, flags: DIFlagPublic)
    !26 = !DIGlobalVariableExpression(var: !27, expr: !DIExpression())
    !27 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !28, isLocal: false, isDefinition: true)
    !28 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !29)
    !29 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 576, align: 64, flags: DIFlagPublic, elements: !30, identifier: "child")
    !30 = !{!31, !32}
    !31 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !18, size: 384, align: 64, flags: DIFlagPublic)
    !32 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !22, size: 176, align: 16, offset: 384, flags: DIFlagPublic)
    !33 = !{i32 2, !"Dwarf Version", i32 5}
    !34 = !{i32 2, !"Debug Info Version", i32 3}
    !35 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !36, splitDebugInlining: false)
    !36 = !{!0, !15, !26}
    !37 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !38, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !40)
    !38 = !DISubroutineType(flags: DIFlagPublic, types: !39)
    !39 = !{null, !4}
    !40 = !{}
    !41 = !DILocalVariable(name: "grandparent", scope: !37, file: !2, line: 7, type: !4)
    !42 = !DILocation(line: 7, column: 8, scope: !37)
    !43 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !44, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !40)
    !44 = !DISubroutineType(flags: DIFlagPublic, types: !45)
    !45 = !{null, !18}
    !46 = !DILocalVariable(name: "parent", scope: !43, file: !2, line: 14, type: !18)
    !47 = !DILocation(line: 14, column: 8, scope: !43)
    !48 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !49, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !40)
    !49 = !DISubroutineType(flags: DIFlagPublic, types: !50)
    !50 = !{null, !29}
    !51 = !DILocalVariable(name: "child", scope: !48, file: !2, line: 20, type: !29)
    !52 = !DILocation(line: 20, column: 12, scope: !48)
    !53 = !DILocation(line: 21, column: 8, scope: !48)
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
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { %foo*, void (%foo*)* }
    %foo = type { i32* }
    %__vtable_bar = type { %bar*, void (%foo*)* }
    %bar = type { %foo }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !0
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @____vtable_bar__init = unnamed_addr constant %__vtable_bar zeroinitializer
    @__bar__init = unnamed_addr constant %bar zeroinitializer, !dbg !9
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer

    define void @foo(%foo* %0) !dbg !19 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !23, metadata !DIExpression()), !dbg !24
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void, !dbg !24
    }

    define void @foo__baz(%foo* %0) !dbg !25 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !26, metadata !DIExpression()), !dbg !27
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void, !dbg !27
    }

    define void @bar(%bar* %0) !dbg !28 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !31, metadata !DIExpression()), !dbg !32
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void, !dbg !32
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init___vtable_foo(%__vtable_foo* %0) {
    entry:
      %self = alloca %__vtable_foo*, align 8
      store %__vtable_foo* %0, %__vtable_foo** %self, align 8
      %deref = load %__vtable_foo*, %__vtable_foo** %self, align 8
      %baz = getelementptr inbounds %__vtable_foo, %__vtable_foo* %deref, i32 0, i32 1
      store void (%foo*)* @foo__baz, void (%foo*)** %baz, align 8
      ret void
    }

    define void @__init___vtable_bar(%__vtable_bar* %0) {
    entry:
      %self = alloca %__vtable_bar*, align 8
      store %__vtable_bar* %0, %__vtable_bar** %self, align 8
      %deref = load %__vtable_bar*, %__vtable_bar** %self, align 8
      %baz = getelementptr inbounds %__vtable_bar, %__vtable_bar* %deref, i32 0, i32 1
      store void (%foo*)* @foo__baz, void (%foo*)** %baz, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_foo* @__vtable_foo_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %__foo = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %__foo)
      %deref1 = load %bar*, %bar** %self, align 8
      %__foo2 = getelementptr inbounds %bar, %bar* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %foo, %foo* %__foo2, i32 0, i32 0
      store i32* bitcast (%__vtable_bar* @__vtable_bar_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_bar(%__vtable_bar* %0) {
    entry:
      %self = alloca %__vtable_bar*, align 8
      store %__vtable_bar* %0, %__vtable_bar** %self, align 8
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

    define void @__user_init___vtable_foo(%__vtable_foo* %0) {
    entry:
      %self = alloca %__vtable_foo*, align 8
      store %__vtable_foo* %0, %__vtable_foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(%__vtable_foo* @__vtable_foo_instance)
      call void @__init___vtable_bar(%__vtable_bar* @__vtable_bar_instance)
      call void @__user_init___vtable_foo(%__vtable_foo* @__vtable_foo_instance)
      call void @__user_init___vtable_bar(%__vtable_bar* @__vtable_bar_instance)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!15, !16}
    !llvm.dbg.cu = !{!17}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 64, align: 64, flags: DIFlagPublic, elements: !5, identifier: "foo")
    !5 = !{!6}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__foo___vtable", baseType: !8, size: 64, align: 64, dwarfAddressSpace: 1)
    !8 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !9 = !DIGlobalVariableExpression(var: !10, expr: !DIExpression())
    !10 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 7, type: !11, isLocal: false, isDefinition: true)
    !11 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !12)
    !12 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 7, size: 64, align: 64, flags: DIFlagPublic, elements: !13, identifier: "bar")
    !13 = !{!14}
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !4, size: 64, align: 64, flags: DIFlagPublic)
    !15 = !{i32 2, !"Dwarf Version", i32 5}
    !16 = !{i32 2, !"Debug Info Version", i32 3}
    !17 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !18, splitDebugInlining: false)
    !18 = !{!0, !9}
    !19 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !20, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !17, retainedNodes: !22)
    !20 = !DISubroutineType(flags: DIFlagPublic, types: !21)
    !21 = !{null, !4}
    !22 = !{}
    !23 = !DILocalVariable(name: "foo", scope: !19, file: !2, line: 5, type: !4)
    !24 = !DILocation(line: 5, column: 8, scope: !19)
    !25 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !19, file: !2, line: 3, type: !20, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !17, retainedNodes: !22)
    !26 = !DILocalVariable(name: "foo", scope: !25, file: !2, line: 4, type: !4)
    !27 = !DILocation(line: 4, column: 8, scope: !25)
    !28 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 7, type: !29, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !17, retainedNodes: !22)
    !29 = !DISubroutineType(flags: DIFlagPublic, types: !30)
    !30 = !{null, !12}
    !31 = !DILocalVariable(name: "bar", scope: !28, file: !2, line: 8, type: !12)
    !32 = !DILocation(line: 8, column: 8, scope: !28)
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

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_parent = type { %parent* }
    %parent = type { i32*, i32 }
    %__vtable_child = type { %child* }
    %child = type { %parent, i32 }
    %__vtable_grandchild = type { %grandchild* }
    %grandchild = type { %child, i32 }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent zeroinitializer, !dbg !0
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child zeroinitializer, !dbg !11
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @____vtable_grandchild__init = unnamed_addr constant %__vtable_grandchild zeroinitializer
    @__grandchild__init = unnamed_addr constant %grandchild zeroinitializer, !dbg !18
    @__vtable_grandchild_instance = global %__vtable_grandchild zeroinitializer

    define void @parent(%parent* %0) !dbg !29 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !33, metadata !DIExpression()), !dbg !34
      %this = alloca %parent*, align 8
      store %parent* %0, %parent** %this, align 8
      %__vtable = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %a = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      ret void, !dbg !34
    }

    define void @child(%child* %0) !dbg !35 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !38, metadata !DIExpression()), !dbg !39
      %this = alloca %child*, align 8
      store %child* %0, %child** %this, align 8
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %b = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      ret void, !dbg !39
    }

    define void @grandchild(%grandchild* %0) !dbg !40 {
    entry:
      call void @llvm.dbg.declare(metadata %grandchild* %0, metadata !43, metadata !DIExpression()), !dbg !44
      %this = alloca %grandchild*, align 8
      store %grandchild* %0, %grandchild** %this, align 8
      %__child = getelementptr inbounds %grandchild, %grandchild* %0, i32 0, i32 0
      %c = getelementptr inbounds %grandchild, %grandchild* %0, i32 0, i32 1
      ret void, !dbg !44
    }

    define i32 @main() !dbg !45 {
    entry:
      %main = alloca i32, align 4
      %array_of_parent = alloca [3 x %parent], align 8
      %array_of_child = alloca [3 x %child], align 8
      %array_of_grandchild = alloca [3 x %grandchild], align 8
      %parent1 = alloca %parent, align 8
      %child1 = alloca %child, align 8
      %grandchild1 = alloca %grandchild, align 8
      call void @llvm.dbg.declare(metadata [3 x %parent]* %array_of_parent, metadata !48, metadata !DIExpression()), !dbg !52
      %0 = bitcast [3 x %parent]* %array_of_parent to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([3 x %parent]* getelementptr ([3 x %parent], [3 x %parent]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata [3 x %child]* %array_of_child, metadata !53, metadata !DIExpression()), !dbg !55
      %1 = bitcast [3 x %child]* %array_of_child to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %1, i8 0, i64 ptrtoint ([3 x %child]* getelementptr ([3 x %child], [3 x %child]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata [3 x %grandchild]* %array_of_grandchild, metadata !56, metadata !DIExpression()), !dbg !58
      %2 = bitcast [3 x %grandchild]* %array_of_grandchild to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %2, i8 0, i64 ptrtoint ([3 x %grandchild]* getelementptr ([3 x %grandchild], [3 x %grandchild]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %parent* %parent1, metadata !59, metadata !DIExpression()), !dbg !60
      %3 = bitcast %parent* %parent1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %3, i8* align 1 bitcast (%parent* @__parent__init to i8*), i64 ptrtoint (%parent* getelementptr (%parent, %parent* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %child* %child1, metadata !61, metadata !DIExpression()), !dbg !62
      %4 = bitcast %child* %child1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %4, i8* align 1 bitcast (%child* @__child__init to i8*), i64 ptrtoint (%child* getelementptr (%child, %child* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %grandchild* %grandchild1, metadata !63, metadata !DIExpression()), !dbg !64
      %5 = bitcast %grandchild* %grandchild1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %5, i8* align 1 bitcast (%grandchild* @__grandchild__init to i8*), i64 ptrtoint (%grandchild* getelementptr (%grandchild, %grandchild* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata i32* %main, metadata !65, metadata !DIExpression()), !dbg !66
      store i32 0, i32* %main, align 4
      call void @__init_parent(%parent* %parent1), !dbg !67
      call void @__init_child(%child* %child1), !dbg !67
      call void @__init_grandchild(%grandchild* %grandchild1), !dbg !67
      call void @__user_init_parent(%parent* %parent1), !dbg !67
      call void @__user_init_child(%child* %child1), !dbg !67
      call void @__user_init_grandchild(%grandchild* %grandchild1), !dbg !67
      %a = getelementptr inbounds %parent, %parent* %parent1, i32 0, i32 1, !dbg !68
      store i32 1, i32* %a, align 4, !dbg !68
      %__parent = getelementptr inbounds %child, %child* %child1, i32 0, i32 0, !dbg !69
      %a1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 1, !dbg !69
      store i32 2, i32* %a1, align 4, !dbg !69
      %b = getelementptr inbounds %child, %child* %child1, i32 0, i32 1, !dbg !70
      store i32 3, i32* %b, align 4, !dbg !70
      %__child = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 0, !dbg !71
      %__parent2 = getelementptr inbounds %child, %child* %__child, i32 0, i32 0, !dbg !71
      %a3 = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 1, !dbg !71
      store i32 4, i32* %a3, align 4, !dbg !71
      %__child4 = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 0, !dbg !72
      %b5 = getelementptr inbounds %child, %child* %__child4, i32 0, i32 1, !dbg !72
      store i32 5, i32* %b5, align 4, !dbg !72
      %c = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 1, !dbg !73
      store i32 6, i32* %c, align 4, !dbg !73
      %tmpVar = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 0, !dbg !74
      %a6 = getelementptr inbounds %parent, %parent* %tmpVar, i32 0, i32 1, !dbg !74
      store i32 7, i32* %a6, align 4, !dbg !74
      %tmpVar7 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 0, !dbg !75
      %__parent8 = getelementptr inbounds %child, %child* %tmpVar7, i32 0, i32 0, !dbg !75
      %a9 = getelementptr inbounds %parent, %parent* %__parent8, i32 0, i32 1, !dbg !75
      store i32 8, i32* %a9, align 4, !dbg !75
      %tmpVar10 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 0, !dbg !76
      %b11 = getelementptr inbounds %child, %child* %tmpVar10, i32 0, i32 1, !dbg !76
      store i32 9, i32* %b11, align 4, !dbg !76
      %tmpVar12 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !77
      %__child13 = getelementptr inbounds %grandchild, %grandchild* %tmpVar12, i32 0, i32 0, !dbg !77
      %__parent14 = getelementptr inbounds %child, %child* %__child13, i32 0, i32 0, !dbg !77
      %a15 = getelementptr inbounds %parent, %parent* %__parent14, i32 0, i32 1, !dbg !77
      store i32 10, i32* %a15, align 4, !dbg !77
      %tmpVar16 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !78
      %__child17 = getelementptr inbounds %grandchild, %grandchild* %tmpVar16, i32 0, i32 0, !dbg !78
      %b18 = getelementptr inbounds %child, %child* %__child17, i32 0, i32 1, !dbg !78
      store i32 11, i32* %b18, align 4, !dbg !78
      %tmpVar19 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !79
      %c20 = getelementptr inbounds %grandchild, %grandchild* %tmpVar19, i32 0, i32 1, !dbg !79
      store i32 12, i32* %c20, align 4, !dbg !79
      %tmpVar21 = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 1, !dbg !80
      %a22 = getelementptr inbounds %parent, %parent* %tmpVar21, i32 0, i32 1, !dbg !80
      store i32 13, i32* %a22, align 4, !dbg !80
      %tmpVar23 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 1, !dbg !81
      %__parent24 = getelementptr inbounds %child, %child* %tmpVar23, i32 0, i32 0, !dbg !81
      %a25 = getelementptr inbounds %parent, %parent* %__parent24, i32 0, i32 1, !dbg !81
      store i32 14, i32* %a25, align 4, !dbg !81
      %tmpVar26 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 1, !dbg !82
      %b27 = getelementptr inbounds %child, %child* %tmpVar26, i32 0, i32 1, !dbg !82
      store i32 15, i32* %b27, align 4, !dbg !82
      %tmpVar28 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !83
      %__child29 = getelementptr inbounds %grandchild, %grandchild* %tmpVar28, i32 0, i32 0, !dbg !83
      %__parent30 = getelementptr inbounds %child, %child* %__child29, i32 0, i32 0, !dbg !83
      %a31 = getelementptr inbounds %parent, %parent* %__parent30, i32 0, i32 1, !dbg !83
      store i32 16, i32* %a31, align 4, !dbg !83
      %tmpVar32 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !84
      %__child33 = getelementptr inbounds %grandchild, %grandchild* %tmpVar32, i32 0, i32 0, !dbg !84
      %b34 = getelementptr inbounds %child, %child* %__child33, i32 0, i32 1, !dbg !84
      store i32 17, i32* %b34, align 4, !dbg !84
      %tmpVar35 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !85
      %c36 = getelementptr inbounds %grandchild, %grandchild* %tmpVar35, i32 0, i32 1, !dbg !85
      store i32 18, i32* %c36, align 4, !dbg !85
      %tmpVar37 = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 2, !dbg !86
      %a38 = getelementptr inbounds %parent, %parent* %tmpVar37, i32 0, i32 1, !dbg !86
      store i32 19, i32* %a38, align 4, !dbg !86
      %tmpVar39 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 2, !dbg !87
      %__parent40 = getelementptr inbounds %child, %child* %tmpVar39, i32 0, i32 0, !dbg !87
      %a41 = getelementptr inbounds %parent, %parent* %__parent40, i32 0, i32 1, !dbg !87
      store i32 20, i32* %a41, align 4, !dbg !87
      %tmpVar42 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 2, !dbg !88
      %b43 = getelementptr inbounds %child, %child* %tmpVar42, i32 0, i32 1, !dbg !88
      store i32 21, i32* %b43, align 4, !dbg !88
      %tmpVar44 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !89
      %__child45 = getelementptr inbounds %grandchild, %grandchild* %tmpVar44, i32 0, i32 0, !dbg !89
      %__parent46 = getelementptr inbounds %child, %child* %__child45, i32 0, i32 0, !dbg !89
      %a47 = getelementptr inbounds %parent, %parent* %__parent46, i32 0, i32 1, !dbg !89
      store i32 22, i32* %a47, align 4, !dbg !89
      %tmpVar48 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !90
      %__child49 = getelementptr inbounds %grandchild, %grandchild* %tmpVar48, i32 0, i32 0, !dbg !90
      %b50 = getelementptr inbounds %child, %child* %__child49, i32 0, i32 1, !dbg !90
      store i32 23, i32* %b50, align 4, !dbg !90
      %tmpVar51 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !91
      %c52 = getelementptr inbounds %grandchild, %grandchild* %tmpVar51, i32 0, i32 1, !dbg !91
      store i32 24, i32* %c52, align 4, !dbg !91
      %main_ret = load i32, i32* %main, align 4, !dbg !92
      ret i32 %main_ret, !dbg !92
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
      ret void
    }

    define void @__init___vtable_grandchild(%__vtable_grandchild* %0) {
    entry:
      %self = alloca %__vtable_grandchild*, align 8
      store %__vtable_grandchild* %0, %__vtable_grandchild** %self, align 8
      ret void
    }

    define void @__init_grandchild(%grandchild* %0) {
    entry:
      %self = alloca %grandchild*, align 8
      store %grandchild* %0, %grandchild** %self, align 8
      %deref = load %grandchild*, %grandchild** %self, align 8
      %__child = getelementptr inbounds %grandchild, %grandchild* %deref, i32 0, i32 0
      call void @__init_child(%child* %__child)
      %deref1 = load %grandchild*, %grandchild** %self, align 8
      %__child2 = getelementptr inbounds %grandchild, %grandchild* %deref1, i32 0, i32 0
      %__parent = getelementptr inbounds %child, %child* %__child2, i32 0, i32 0
      %__vtable = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      store i32* bitcast (%__vtable_grandchild* @__vtable_grandchild_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      %deref1 = load %child*, %child** %self, align 8
      %__parent2 = getelementptr inbounds %child, %child* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0
      store i32* bitcast (%__vtable_child* @__vtable_child_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__vtable = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_parent* @__vtable_parent_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      ret void
    }

    define void @__user_init___vtable_parent(%__vtable_parent* %0) {
    entry:
      %self = alloca %__vtable_parent*, align 8
      store %__vtable_parent* %0, %__vtable_parent** %self, align 8
      ret void
    }

    define void @__user_init___vtable_grandchild(%__vtable_grandchild* %0) {
    entry:
      %self = alloca %__vtable_grandchild*, align 8
      store %__vtable_grandchild* %0, %__vtable_grandchild** %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(%__vtable_child* %0) {
    entry:
      %self = alloca %__vtable_child*, align 8
      store %__vtable_child* %0, %__vtable_child** %self, align 8
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

    define void @__user_init_grandchild(%grandchild* %0) {
    entry:
      %self = alloca %grandchild*, align 8
      store %grandchild* %0, %grandchild** %self, align 8
      %deref = load %grandchild*, %grandchild** %self, align 8
      %__child = getelementptr inbounds %grandchild, %grandchild* %deref, i32 0, i32 0
      call void @__user_init_child(%child* %__child)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_parent(%__vtable_parent* @__vtable_parent_instance)
      call void @__init___vtable_child(%__vtable_child* @__vtable_child_instance)
      call void @__init___vtable_grandchild(%__vtable_grandchild* @__vtable_grandchild_instance)
      call void @__user_init___vtable_parent(%__vtable_parent* @__vtable_parent_instance)
      call void @__user_init___vtable_child(%__vtable_child* @__vtable_child_instance)
      call void @__user_init___vtable_grandchild(%__vtable_grandchild* @__vtable_grandchild_instance)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn writeonly }
    attributes #2 = { argmemonly nofree nounwind willreturn }

    !llvm.module.flags = !{!25, !26}
    !llvm.dbg.cu = !{!27}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 2, size: 128, align: 64, flags: DIFlagPublic, elements: !5, identifier: "parent")
    !5 = !{!6, !9}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__parent___vtable", baseType: !8, size: 64, align: 64, dwarfAddressSpace: 1)
    !8 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 4, baseType: !10, size: 32, align: 32, offset: 64, flags: DIFlagPublic)
    !10 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !11 = !DIGlobalVariableExpression(var: !12, expr: !DIExpression())
    !12 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 8, type: !13, isLocal: false, isDefinition: true)
    !13 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !14)
    !14 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 8, size: 192, align: 64, flags: DIFlagPublic, elements: !15, identifier: "child")
    !15 = !{!16, !17}
    !16 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !4, size: 128, align: 64, flags: DIFlagPublic)
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 10, baseType: !10, size: 32, align: 32, offset: 128, flags: DIFlagPublic)
    !18 = !DIGlobalVariableExpression(var: !19, expr: !DIExpression())
    !19 = distinct !DIGlobalVariable(name: "__grandchild__init", scope: !2, file: !2, line: 14, type: !20, isLocal: false, isDefinition: true)
    !20 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !21)
    !21 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandchild", scope: !2, file: !2, line: 14, size: 256, align: 64, flags: DIFlagPublic, elements: !22, identifier: "grandchild")
    !22 = !{!23, !24}
    !23 = !DIDerivedType(tag: DW_TAG_member, name: "__child", scope: !2, file: !2, baseType: !14, size: 192, align: 64, flags: DIFlagPublic)
    !24 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 16, baseType: !10, size: 32, align: 32, offset: 192, flags: DIFlagPublic)
    !25 = !{i32 2, !"Dwarf Version", i32 5}
    !26 = !{i32 2, !"Debug Info Version", i32 3}
    !27 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !28, splitDebugInlining: false)
    !28 = !{!0, !11, !18}
    !29 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 2, type: !30, scopeLine: 6, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !27, retainedNodes: !32)
    !30 = !DISubroutineType(flags: DIFlagPublic, types: !31)
    !31 = !{null, !4}
    !32 = !{}
    !33 = !DILocalVariable(name: "parent", scope: !29, file: !2, line: 6, type: !4)
    !34 = !DILocation(line: 6, scope: !29)
    !35 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 8, type: !36, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !27, retainedNodes: !32)
    !36 = !DISubroutineType(flags: DIFlagPublic, types: !37)
    !37 = !{null, !14}
    !38 = !DILocalVariable(name: "child", scope: !35, file: !2, line: 12, type: !14)
    !39 = !DILocation(line: 12, scope: !35)
    !40 = distinct !DISubprogram(name: "grandchild", linkageName: "grandchild", scope: !2, file: !2, line: 14, type: !41, scopeLine: 18, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !27, retainedNodes: !32)
    !41 = !DISubroutineType(flags: DIFlagPublic, types: !42)
    !42 = !{null, !21}
    !43 = !DILocalVariable(name: "grandchild", scope: !40, file: !2, line: 18, type: !21)
    !44 = !DILocation(line: 18, scope: !40)
    !45 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 20, type: !46, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !27, retainedNodes: !32)
    !46 = !DISubroutineType(flags: DIFlagPublic, types: !47)
    !47 = !{null}
    !48 = !DILocalVariable(name: "array_of_parent", scope: !45, file: !2, line: 22, type: !49, align: 64)
    !49 = !DICompositeType(tag: DW_TAG_array_type, baseType: !4, size: 384, align: 64, elements: !50)
    !50 = !{!51}
    !51 = !DISubrange(count: 3, lowerBound: 0)
    !52 = !DILocation(line: 22, column: 4, scope: !45)
    !53 = !DILocalVariable(name: "array_of_child", scope: !45, file: !2, line: 23, type: !54, align: 64)
    !54 = !DICompositeType(tag: DW_TAG_array_type, baseType: !14, size: 576, align: 64, elements: !50)
    !55 = !DILocation(line: 23, column: 4, scope: !45)
    !56 = !DILocalVariable(name: "array_of_grandchild", scope: !45, file: !2, line: 24, type: !57, align: 64)
    !57 = !DICompositeType(tag: DW_TAG_array_type, baseType: !21, size: 768, align: 64, elements: !50)
    !58 = !DILocation(line: 24, column: 4, scope: !45)
    !59 = !DILocalVariable(name: "parent1", scope: !45, file: !2, line: 25, type: !4, align: 64)
    !60 = !DILocation(line: 25, column: 4, scope: !45)
    !61 = !DILocalVariable(name: "child1", scope: !45, file: !2, line: 26, type: !14, align: 64)
    !62 = !DILocation(line: 26, column: 4, scope: !45)
    !63 = !DILocalVariable(name: "grandchild1", scope: !45, file: !2, line: 27, type: !21, align: 64)
    !64 = !DILocation(line: 27, column: 4, scope: !45)
    !65 = !DILocalVariable(name: "main", scope: !45, file: !2, line: 20, type: !10, align: 32)
    !66 = !DILocation(line: 20, column: 9, scope: !45)
    !67 = !DILocation(line: 0, scope: !45)
    !68 = !DILocation(line: 30, column: 4, scope: !45)
    !69 = !DILocation(line: 31, column: 4, scope: !45)
    !70 = !DILocation(line: 32, column: 4, scope: !45)
    !71 = !DILocation(line: 33, column: 4, scope: !45)
    !72 = !DILocation(line: 34, column: 4, scope: !45)
    !73 = !DILocation(line: 35, column: 4, scope: !45)
    !74 = !DILocation(line: 37, column: 4, scope: !45)
    !75 = !DILocation(line: 38, column: 4, scope: !45)
    !76 = !DILocation(line: 39, column: 4, scope: !45)
    !77 = !DILocation(line: 40, column: 4, scope: !45)
    !78 = !DILocation(line: 41, column: 4, scope: !45)
    !79 = !DILocation(line: 42, column: 4, scope: !45)
    !80 = !DILocation(line: 43, column: 4, scope: !45)
    !81 = !DILocation(line: 44, column: 4, scope: !45)
    !82 = !DILocation(line: 45, column: 4, scope: !45)
    !83 = !DILocation(line: 46, column: 4, scope: !45)
    !84 = !DILocation(line: 47, column: 4, scope: !45)
    !85 = !DILocation(line: 48, column: 4, scope: !45)
    !86 = !DILocation(line: 49, column: 4, scope: !45)
    !87 = !DILocation(line: 50, column: 4, scope: !45)
    !88 = !DILocation(line: 51, column: 4, scope: !45)
    !89 = !DILocation(line: 52, column: 4, scope: !45)
    !90 = !DILocation(line: 53, column: 4, scope: !45)
    !91 = !DILocation(line: 54, column: 4, scope: !45)
    !92 = !DILocation(line: 56, scope: !45)
    "#);
}
