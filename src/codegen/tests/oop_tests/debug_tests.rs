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

    %__vtable_foo = type { void (%foo*)* }
    %foo = type { i32*, i16, [81 x i8], [11 x [81 x i8]] }
    %__vtable_bar = type { void (%bar*)* }
    %bar = type { %foo }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !0
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @____vtable_bar__init = unnamed_addr constant %__vtable_bar zeroinitializer
    @__bar__init = unnamed_addr constant %bar zeroinitializer, !dbg !22
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer

    define void @foo(%foo* %0) !dbg !32 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !36, metadata !DIExpression()), !dbg !37
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %a = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %b = getelementptr inbounds %foo, %foo* %0, i32 0, i32 2
      %c = getelementptr inbounds %foo, %foo* %0, i32 0, i32 3
      ret void, !dbg !37
    }

    define void @bar(%bar* %0) !dbg !38 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !41, metadata !DIExpression()), !dbg !42
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void, !dbg !42
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init___vtable_foo(%__vtable_foo* %0) {
    entry:
      %self = alloca %__vtable_foo*, align 8
      store %__vtable_foo* %0, %__vtable_foo** %self, align 8
      %deref = load %__vtable_foo*, %__vtable_foo** %self, align 8
      %__body = getelementptr inbounds %__vtable_foo, %__vtable_foo* %deref, i32 0, i32 0
      store void (%foo*)* @foo, void (%foo*)** %__body, align 8
      ret void
    }

    define void @__init___vtable_bar(%__vtable_bar* %0) {
    entry:
      %self = alloca %__vtable_bar*, align 8
      store %__vtable_bar* %0, %__vtable_bar** %self, align 8
      %deref = load %__vtable_bar*, %__vtable_bar** %self, align 8
      %__body = getelementptr inbounds %__vtable_bar, %__vtable_bar* %deref, i32 0, i32 0
      store void (%bar*)* @bar, void (%bar*)** %__body, align 8
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

    !llvm.module.flags = !{!28, !29}
    !llvm.dbg.cu = !{!30}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 7872, align: 64, flags: DIFlagPublic, elements: !5, identifier: "foo")
    !5 = !{!6, !10, !12, !18}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____foo___vtable", scope: !2, file: !2, baseType: !8, align: 64)
    !8 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__foo___vtable", baseType: !9, size: 64, align: 64, dwarfAddressSpace: 1)
    !9 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 4, baseType: !11, size: 16, align: 16, offset: 64, flags: DIFlagPublic)
    !11 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 5, baseType: !13, size: 648, align: 8, offset: 80, flags: DIFlagPublic)
    !13 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !2, file: !2, baseType: !14, align: 8)
    !14 = !DICompositeType(tag: DW_TAG_array_type, baseType: !15, size: 648, align: 8, elements: !16)
    !15 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !16 = !{!17}
    !17 = !DISubrange(count: 81, lowerBound: 0)
    !18 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 6, baseType: !19, size: 7128, align: 8, offset: 728, flags: DIFlagPublic)
    !19 = !DICompositeType(tag: DW_TAG_array_type, baseType: !13, size: 7128, align: 8, elements: !20)
    !20 = !{!21}
    !21 = !DISubrange(count: 11, lowerBound: 0)
    !22 = !DIGlobalVariableExpression(var: !23, expr: !DIExpression())
    !23 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 10, type: !24, isLocal: false, isDefinition: true)
    !24 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !25)
    !25 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 10, size: 7872, align: 64, flags: DIFlagPublic, elements: !26, identifier: "bar")
    !26 = !{!27}
    !27 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !4, size: 7872, align: 64, flags: DIFlagPublic)
    !28 = !{i32 2, !"Dwarf Version", i32 5}
    !29 = !{i32 2, !"Debug Info Version", i32 3}
    !30 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !31, splitDebugInlining: false)
    !31 = !{!0, !22}
    !32 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !33, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !30, retainedNodes: !35)
    !33 = !DISubroutineType(flags: DIFlagPublic, types: !34)
    !34 = !{null, !4}
    !35 = !{}
    !36 = !DILocalVariable(name: "foo", scope: !32, file: !2, line: 8, type: !4)
    !37 = !DILocation(line: 8, column: 8, scope: !32)
    !38 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 10, type: !39, scopeLine: 11, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !30, retainedNodes: !35)
    !39 = !DISubroutineType(flags: DIFlagPublic, types: !40)
    !40 = !{null, !25}
    !41 = !DILocalVariable(name: "bar", scope: !38, file: !2, line: 11, type: !25)
    !42 = !DILocation(line: 11, column: 8, scope: !38)
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

    %__vtable_fb = type { void (%fb*)* }
    %fb = type { i32*, i16, i16 }
    %__vtable_fb2 = type { void (%fb2*)* }
    %fb2 = type { %fb }
    %__vtable_foo = type { void (%foo*)* }
    %foo = type { i32*, %fb2 }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_fb__init = unnamed_addr constant %__vtable_fb zeroinitializer
    @__fb__init = unnamed_addr constant %fb zeroinitializer, !dbg !0
    @__vtable_fb_instance = global %__vtable_fb zeroinitializer
    @____vtable_fb2__init = unnamed_addr constant %__vtable_fb2 zeroinitializer
    @__fb2__init = unnamed_addr constant %fb2 zeroinitializer, !dbg !13
    @__vtable_fb2_instance = global %__vtable_fb2 zeroinitializer
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !19
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @fb(%fb* %0) !dbg !32 {
    entry:
      call void @llvm.dbg.declare(metadata %fb* %0, metadata !36, metadata !DIExpression()), !dbg !37
      %this = alloca %fb*, align 8
      store %fb* %0, %fb** %this, align 8
      %__vtable = getelementptr inbounds %fb, %fb* %0, i32 0, i32 0
      %x = getelementptr inbounds %fb, %fb* %0, i32 0, i32 1
      %y = getelementptr inbounds %fb, %fb* %0, i32 0, i32 2
      ret void, !dbg !37
    }

    define void @fb2(%fb2* %0) !dbg !38 {
    entry:
      call void @llvm.dbg.declare(metadata %fb2* %0, metadata !41, metadata !DIExpression()), !dbg !42
      %this = alloca %fb2*, align 8
      store %fb2* %0, %fb2** %this, align 8
      %__fb = getelementptr inbounds %fb2, %fb2* %0, i32 0, i32 0
      ret void, !dbg !42
    }

    define void @foo(%foo* %0) !dbg !43 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !46, metadata !DIExpression()), !dbg !47
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %myFb = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %__fb = getelementptr inbounds %fb2, %fb2* %myFb, i32 0, i32 0, !dbg !47
      %x = getelementptr inbounds %fb, %fb* %__fb, i32 0, i32 1, !dbg !47
      store i16 1, i16* %x, align 2, !dbg !47
      ret void, !dbg !48
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init___vtable_fb(%__vtable_fb* %0) {
    entry:
      %self = alloca %__vtable_fb*, align 8
      store %__vtable_fb* %0, %__vtable_fb** %self, align 8
      %deref = load %__vtable_fb*, %__vtable_fb** %self, align 8
      %__body = getelementptr inbounds %__vtable_fb, %__vtable_fb* %deref, i32 0, i32 0
      store void (%fb*)* @fb, void (%fb*)** %__body, align 8
      ret void
    }

    define void @__init___vtable_fb2(%__vtable_fb2* %0) {
    entry:
      %self = alloca %__vtable_fb2*, align 8
      store %__vtable_fb2* %0, %__vtable_fb2** %self, align 8
      %deref = load %__vtable_fb2*, %__vtable_fb2** %self, align 8
      %__body = getelementptr inbounds %__vtable_fb2, %__vtable_fb2* %deref, i32 0, i32 0
      store void (%fb2*)* @fb2, void (%fb2*)** %__body, align 8
      ret void
    }

    define void @__init___vtable_foo(%__vtable_foo* %0) {
    entry:
      %self = alloca %__vtable_foo*, align 8
      store %__vtable_foo* %0, %__vtable_foo** %self, align 8
      %deref = load %__vtable_foo*, %__vtable_foo** %self, align 8
      %__body = getelementptr inbounds %__vtable_foo, %__vtable_foo* %deref, i32 0, i32 0
      store void (%foo*)* @foo, void (%foo*)** %__body, align 8
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

    !llvm.module.flags = !{!28, !29}
    !llvm.dbg.cu = !{!30}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__fb__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb", scope: !2, file: !2, line: 2, size: 128, align: 64, flags: DIFlagPublic, elements: !5, identifier: "fb")
    !5 = !{!6, !10, !12}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____fb___vtable", scope: !2, file: !2, baseType: !8, align: 64)
    !8 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__fb___vtable", baseType: !9, size: 64, align: 64, dwarfAddressSpace: 1)
    !9 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 4, baseType: !11, size: 16, align: 16, offset: 64, flags: DIFlagPublic)
    !11 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 5, baseType: !11, size: 16, align: 16, offset: 80, flags: DIFlagPublic)
    !13 = !DIGlobalVariableExpression(var: !14, expr: !DIExpression())
    !14 = distinct !DIGlobalVariable(name: "__fb2__init", scope: !2, file: !2, line: 9, type: !15, isLocal: false, isDefinition: true)
    !15 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !16)
    !16 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb2", scope: !2, file: !2, line: 9, size: 128, align: 64, flags: DIFlagPublic, elements: !17, identifier: "fb2")
    !17 = !{!18}
    !18 = !DIDerivedType(tag: DW_TAG_member, name: "__fb", scope: !2, file: !2, baseType: !4, size: 128, align: 64, flags: DIFlagPublic)
    !19 = !DIGlobalVariableExpression(var: !20, expr: !DIExpression())
    !20 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 12, type: !21, isLocal: false, isDefinition: true)
    !21 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !22)
    !22 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 12, size: 192, align: 64, flags: DIFlagPublic, elements: !23, identifier: "foo")
    !23 = !{!24, !27}
    !24 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !25, size: 64, align: 64, flags: DIFlagPublic)
    !25 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____foo___vtable", scope: !2, file: !2, baseType: !26, align: 64)
    !26 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__foo___vtable", baseType: !9, size: 64, align: 64, dwarfAddressSpace: 1)
    !27 = !DIDerivedType(tag: DW_TAG_member, name: "myFb", scope: !2, file: !2, line: 14, baseType: !16, size: 128, align: 64, offset: 64, flags: DIFlagPublic)
    !28 = !{i32 2, !"Dwarf Version", i32 5}
    !29 = !{i32 2, !"Debug Info Version", i32 3}
    !30 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !31, splitDebugInlining: false)
    !31 = !{!0, !13, !19}
    !32 = distinct !DISubprogram(name: "fb", linkageName: "fb", scope: !2, file: !2, line: 2, type: !33, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !30, retainedNodes: !35)
    !33 = !DISubroutineType(flags: DIFlagPublic, types: !34)
    !34 = !{null, !4}
    !35 = !{}
    !36 = !DILocalVariable(name: "fb", scope: !32, file: !2, line: 7, type: !4)
    !37 = !DILocation(line: 7, column: 8, scope: !32)
    !38 = distinct !DISubprogram(name: "fb2", linkageName: "fb2", scope: !2, file: !2, line: 9, type: !39, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !30, retainedNodes: !35)
    !39 = !DISubroutineType(flags: DIFlagPublic, types: !40)
    !40 = !{null, !16}
    !41 = !DILocalVariable(name: "fb2", scope: !38, file: !2, line: 10, type: !16)
    !42 = !DILocation(line: 10, column: 8, scope: !38)
    !43 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 12, type: !44, scopeLine: 16, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !30, retainedNodes: !35)
    !44 = !DISubroutineType(flags: DIFlagPublic, types: !45)
    !45 = !{null, !22}
    !46 = !DILocalVariable(name: "foo", scope: !43, file: !2, line: 16, type: !22)
    !47 = !DILocation(line: 16, column: 12, scope: !43)
    !48 = !DILocation(line: 17, column: 8, scope: !43)
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

    %__vtable_foo = type { void (%foo*)*, void (%foo*)* }
    %foo = type { i32*, [81 x i8] }
    %__vtable_bar = type { void (%bar*)*, void (%foo*)* }
    %bar = type { %foo }

    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"world\00"
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !0
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @____vtable_bar__init = unnamed_addr constant %__vtable_bar zeroinitializer
    @__bar__init = unnamed_addr constant %bar zeroinitializer, !dbg !16
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer

    define void @foo(%foo* %0) !dbg !26 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !30, metadata !DIExpression()), !dbg !31
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      ret void, !dbg !31
    }

    define void @foo__baz(%foo* %0) !dbg !32 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !33, metadata !DIExpression()), !dbg !34
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %1 = bitcast [81 x i8]* %s to i8*, !dbg !34
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_0, i32 0, i32 0), i32 6, i1 false), !dbg !34
      ret void, !dbg !35
    }

    define void @bar(%bar* %0) !dbg !36 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !39, metadata !DIExpression()), !dbg !40
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %__foo, i32 0, i32 1, !dbg !40
      %1 = bitcast [81 x i8]* %s to i8*, !dbg !40
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_1, i32 0, i32 0), i32 6, i1 false), !dbg !40
      ret void, !dbg !41
    }

    define void @main() !dbg !42 {
    entry:
      %s = alloca [81 x i8], align 1
      %fb = alloca %bar, align 8
      call void @llvm.dbg.declare(metadata [81 x i8]* %s, metadata !45, metadata !DIExpression()), !dbg !46
      %0 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %bar* %fb, metadata !47, metadata !DIExpression()), !dbg !48
      %1 = bitcast %bar* %fb to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 bitcast (%bar* @__bar__init to i8*), i64 ptrtoint (%bar* getelementptr (%bar, %bar* null, i32 1) to i64), i1 false)
      call void @__init_bar(%bar* %fb), !dbg !49
      call void @__user_init_bar(%bar* %fb), !dbg !49
      %__foo = getelementptr inbounds %bar, %bar* %fb, i32 0, i32 0, !dbg !49
      call void @foo__baz(%foo* %__foo), !dbg !50
      call void @bar(%bar* %fb), !dbg !51
      ret void, !dbg !52
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
      %__body = getelementptr inbounds %__vtable_foo, %__vtable_foo* %deref, i32 0, i32 0
      store void (%foo*)* @foo, void (%foo*)** %__body, align 8
      %deref1 = load %__vtable_foo*, %__vtable_foo** %self, align 8
      %baz = getelementptr inbounds %__vtable_foo, %__vtable_foo* %deref1, i32 0, i32 1
      store void (%foo*)* @foo__baz, void (%foo*)** %baz, align 8
      ret void
    }

    define void @__init___vtable_bar(%__vtable_bar* %0) {
    entry:
      %self = alloca %__vtable_bar*, align 8
      store %__vtable_bar* %0, %__vtable_bar** %self, align 8
      %deref = load %__vtable_bar*, %__vtable_bar** %self, align 8
      %__body = getelementptr inbounds %__vtable_bar, %__vtable_bar* %deref, i32 0, i32 0
      store void (%bar*)* @bar, void (%bar*)** %__body, align 8
      %deref1 = load %__vtable_bar*, %__vtable_bar** %self, align 8
      %baz = getelementptr inbounds %__vtable_bar, %__vtable_bar* %deref1, i32 0, i32 1
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

    !llvm.module.flags = !{!22, !23}
    !llvm.dbg.cu = !{!24}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 768, align: 64, flags: DIFlagPublic, elements: !5, identifier: "foo")
    !5 = !{!6, !10}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____foo___vtable", scope: !2, file: !2, baseType: !8, align: 64)
    !8 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__foo___vtable", baseType: !9, size: 64, align: 64, dwarfAddressSpace: 1)
    !9 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !2, file: !2, line: 4, baseType: !11, size: 648, align: 8, offset: 64, flags: DIFlagPublic)
    !11 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !2, file: !2, baseType: !12, align: 8)
    !12 = !DICompositeType(tag: DW_TAG_array_type, baseType: !13, size: 648, align: 8, elements: !14)
    !13 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !14 = !{!15}
    !15 = !DISubrange(count: 81, lowerBound: 0)
    !16 = !DIGlobalVariableExpression(var: !17, expr: !DIExpression())
    !17 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 11, type: !18, isLocal: false, isDefinition: true)
    !18 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !19)
    !19 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 11, size: 768, align: 64, flags: DIFlagPublic, elements: !20, identifier: "bar")
    !20 = !{!21}
    !21 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !4, size: 768, align: 64, flags: DIFlagPublic)
    !22 = !{i32 2, !"Dwarf Version", i32 5}
    !23 = !{i32 2, !"Debug Info Version", i32 3}
    !24 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !25, splitDebugInlining: false)
    !25 = !{!0, !16}
    !26 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !27, scopeLine: 9, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !24, retainedNodes: !29)
    !27 = !DISubroutineType(flags: DIFlagPublic, types: !28)
    !28 = !{null, !4}
    !29 = !{}
    !30 = !DILocalVariable(name: "foo", scope: !26, file: !2, line: 9, type: !4)
    !31 = !DILocation(line: 9, column: 8, scope: !26)
    !32 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !26, file: !2, line: 6, type: !27, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !24, retainedNodes: !29)
    !33 = !DILocalVariable(name: "foo", scope: !32, file: !2, line: 7, type: !4)
    !34 = !DILocation(line: 7, column: 12, scope: !32)
    !35 = !DILocation(line: 8, column: 8, scope: !32)
    !36 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 11, type: !37, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !24, retainedNodes: !29)
    !37 = !DISubroutineType(flags: DIFlagPublic, types: !38)
    !38 = !{null, !19}
    !39 = !DILocalVariable(name: "bar", scope: !36, file: !2, line: 12, type: !19)
    !40 = !DILocation(line: 12, column: 12, scope: !36)
    !41 = !DILocation(line: 13, column: 8, scope: !36)
    !42 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 15, type: !43, scopeLine: 15, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !24, retainedNodes: !29)
    !43 = !DISubroutineType(flags: DIFlagPublic, types: !44)
    !44 = !{null}
    !45 = !DILocalVariable(name: "s", scope: !42, file: !2, line: 17, type: !11, align: 8)
    !46 = !DILocation(line: 17, column: 12, scope: !42)
    !47 = !DILocalVariable(name: "fb", scope: !42, file: !2, line: 18, type: !19, align: 64)
    !48 = !DILocation(line: 18, column: 12, scope: !42)
    !49 = !DILocation(line: 0, scope: !42)
    !50 = !DILocation(line: 20, column: 12, scope: !42)
    !51 = !DILocation(line: 21, column: 12, scope: !42)
    !52 = !DILocation(line: 22, column: 8, scope: !42)
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

    %__vtable_grandparent = type { void (%grandparent*)* }
    %grandparent = type { i32*, [6 x i16], i16 }
    %__vtable_parent = type { void (%parent*)* }
    %parent = type { %grandparent, [11 x i16], i16 }
    %__vtable_child = type { void (%child*)* }
    %child = type { %parent, [11 x i16] }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_grandparent__init = unnamed_addr constant %__vtable_grandparent zeroinitializer
    @__grandparent__init = unnamed_addr constant %grandparent zeroinitializer, !dbg !0
    @__vtable_grandparent_instance = global %__vtable_grandparent zeroinitializer
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent zeroinitializer, !dbg !16
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child zeroinitializer, !dbg !27
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @grandparent(%grandparent* %0) !dbg !38 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !42, metadata !DIExpression()), !dbg !43
      %this = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %this, align 8
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 2
      ret void, !dbg !43
    }

    define void @parent(%parent* %0) !dbg !44 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !47, metadata !DIExpression()), !dbg !48
      %this = alloca %parent*, align 8
      store %parent* %0, %parent** %this, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void, !dbg !48
    }

    define void @child(%child* %0) !dbg !49 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !52, metadata !DIExpression()), !dbg !53
      %this = alloca %child*, align 8
      store %child* %0, %child** %this, align 8
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      ret void, !dbg !53
    }

    define void @main() !dbg !54 {
    entry:
      %arr = alloca [11 x %child], align 8
      call void @llvm.dbg.declare(metadata [11 x %child]* %arr, metadata !57, metadata !DIExpression()), !dbg !59
      %0 = bitcast [11 x %child]* %arr to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([11 x %child]* getelementptr ([11 x %child], [11 x %child]* null, i32 1) to i64), i1 false)
      %tmpVar = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !60
      %__parent = getelementptr inbounds %child, %child* %tmpVar, i32 0, i32 0, !dbg !60
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !60
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 2, !dbg !60
      store i16 10, i16* %a, align 2, !dbg !60
      %tmpVar1 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !61
      %__parent2 = getelementptr inbounds %child, %child* %tmpVar1, i32 0, i32 0, !dbg !61
      %__grandparent3 = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0, !dbg !61
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent3, i32 0, i32 1, !dbg !61
      %tmpVar4 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 0, !dbg !61
      store i16 20, i16* %tmpVar4, align 2, !dbg !61
      %tmpVar5 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !62
      %__parent6 = getelementptr inbounds %child, %child* %tmpVar5, i32 0, i32 0, !dbg !62
      %b = getelementptr inbounds %parent, %parent* %__parent6, i32 0, i32 2, !dbg !62
      store i16 30, i16* %b, align 2, !dbg !62
      %tmpVar7 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !63
      %__parent8 = getelementptr inbounds %child, %child* %tmpVar7, i32 0, i32 0, !dbg !63
      %x = getelementptr inbounds %parent, %parent* %__parent8, i32 0, i32 1, !dbg !63
      %tmpVar9 = getelementptr inbounds [11 x i16], [11 x i16]* %x, i32 0, i32 1, !dbg !63
      store i16 40, i16* %tmpVar9, align 2, !dbg !63
      %tmpVar10 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 2, !dbg !64
      %z = getelementptr inbounds %child, %child* %tmpVar10, i32 0, i32 1, !dbg !64
      %tmpVar11 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 2, !dbg !64
      store i16 50, i16* %tmpVar11, align 2, !dbg !64
      ret void, !dbg !65
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #1

    define void @__init___vtable_grandparent(%__vtable_grandparent* %0) {
    entry:
      %self = alloca %__vtable_grandparent*, align 8
      store %__vtable_grandparent* %0, %__vtable_grandparent** %self, align 8
      %deref = load %__vtable_grandparent*, %__vtable_grandparent** %self, align 8
      %__body = getelementptr inbounds %__vtable_grandparent, %__vtable_grandparent* %deref, i32 0, i32 0
      store void (%grandparent*)* @grandparent, void (%grandparent*)** %__body, align 8
      ret void
    }

    define void @__init___vtable_parent(%__vtable_parent* %0) {
    entry:
      %self = alloca %__vtable_parent*, align 8
      store %__vtable_parent* %0, %__vtable_parent** %self, align 8
      %deref = load %__vtable_parent*, %__vtable_parent** %self, align 8
      %__body = getelementptr inbounds %__vtable_parent, %__vtable_parent* %deref, i32 0, i32 0
      store void (%parent*)* @parent, void (%parent*)** %__body, align 8
      ret void
    }

    define void @__init___vtable_child(%__vtable_child* %0) {
    entry:
      %self = alloca %__vtable_child*, align 8
      store %__vtable_child* %0, %__vtable_child** %self, align 8
      %deref = load %__vtable_child*, %__vtable_child** %self, align 8
      %__body = getelementptr inbounds %__vtable_child, %__vtable_child* %deref, i32 0, i32 0
      store void (%child*)* @child, void (%child*)** %__body, align 8
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

    !llvm.module.flags = !{!34, !35}
    !llvm.dbg.cu = !{!36}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 192, align: 64, flags: DIFlagPublic, elements: !5, identifier: "grandparent")
    !5 = !{!6, !10, !15}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____grandparent___vtable", scope: !2, file: !2, baseType: !8, align: 64)
    !8 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__grandparent___vtable", baseType: !9, size: 64, align: 64, dwarfAddressSpace: 1)
    !9 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !11, size: 96, align: 16, offset: 64, flags: DIFlagPublic)
    !11 = !DICompositeType(tag: DW_TAG_array_type, baseType: !12, size: 96, align: 16, elements: !13)
    !12 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !13 = !{!14}
    !14 = !DISubrange(count: 6, lowerBound: 0)
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !12, size: 16, align: 16, offset: 160, flags: DIFlagPublic)
    !16 = !DIGlobalVariableExpression(var: !17, expr: !DIExpression())
    !17 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !18, isLocal: false, isDefinition: true)
    !18 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !19)
    !19 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 384, align: 64, flags: DIFlagPublic, elements: !20, identifier: "parent")
    !20 = !{!21, !22, !26}
    !21 = !DIDerivedType(tag: DW_TAG_member, name: "__grandparent", scope: !2, file: !2, baseType: !4, size: 192, align: 64, flags: DIFlagPublic)
    !22 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !23, size: 176, align: 16, offset: 192, flags: DIFlagPublic)
    !23 = !DICompositeType(tag: DW_TAG_array_type, baseType: !12, size: 176, align: 16, elements: !24)
    !24 = !{!25}
    !25 = !DISubrange(count: 11, lowerBound: 0)
    !26 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !12, size: 16, align: 16, offset: 368, flags: DIFlagPublic)
    !27 = !DIGlobalVariableExpression(var: !28, expr: !DIExpression())
    !28 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !29, isLocal: false, isDefinition: true)
    !29 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !30)
    !30 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 576, align: 64, flags: DIFlagPublic, elements: !31, identifier: "child")
    !31 = !{!32, !33}
    !32 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !19, size: 384, align: 64, flags: DIFlagPublic)
    !33 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !23, size: 176, align: 16, offset: 384, flags: DIFlagPublic)
    !34 = !{i32 2, !"Dwarf Version", i32 5}
    !35 = !{i32 2, !"Debug Info Version", i32 3}
    !36 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !37, splitDebugInlining: false)
    !37 = !{!0, !16, !27}
    !38 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !39, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !36, retainedNodes: !41)
    !39 = !DISubroutineType(flags: DIFlagPublic, types: !40)
    !40 = !{null, !4}
    !41 = !{}
    !42 = !DILocalVariable(name: "grandparent", scope: !38, file: !2, line: 7, type: !4)
    !43 = !DILocation(line: 7, column: 8, scope: !38)
    !44 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !45, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !36, retainedNodes: !41)
    !45 = !DISubroutineType(flags: DIFlagPublic, types: !46)
    !46 = !{null, !19}
    !47 = !DILocalVariable(name: "parent", scope: !44, file: !2, line: 14, type: !19)
    !48 = !DILocation(line: 14, column: 8, scope: !44)
    !49 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !50, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !36, retainedNodes: !41)
    !50 = !DISubroutineType(flags: DIFlagPublic, types: !51)
    !51 = !{null, !30}
    !52 = !DILocalVariable(name: "child", scope: !49, file: !2, line: 20, type: !30)
    !53 = !DILocation(line: 20, column: 8, scope: !49)
    !54 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 22, type: !55, scopeLine: 26, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !36, retainedNodes: !41)
    !55 = !DISubroutineType(flags: DIFlagPublic, types: !56)
    !56 = !{null}
    !57 = !DILocalVariable(name: "arr", scope: !54, file: !2, line: 24, type: !58, align: 64)
    !58 = !DICompositeType(tag: DW_TAG_array_type, baseType: !30, size: 6336, align: 64, elements: !24)
    !59 = !DILocation(line: 24, column: 12, scope: !54)
    !60 = !DILocation(line: 26, column: 12, scope: !54)
    !61 = !DILocation(line: 27, column: 12, scope: !54)
    !62 = !DILocation(line: 28, column: 12, scope: !54)
    !63 = !DILocation(line: 29, column: 12, scope: !54)
    !64 = !DILocation(line: 30, column: 12, scope: !54)
    !65 = !DILocation(line: 31, column: 8, scope: !54)
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

    %__vtable_grandparent = type { void (%grandparent*)* }
    %grandparent = type { i32*, [6 x i16], i16 }
    %__vtable_parent = type { void (%parent*)* }
    %parent = type { %grandparent, [11 x i16], i16 }
    %__vtable_child = type { void (%child*)* }
    %child = type { %parent, [11 x i16] }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_grandparent__init = unnamed_addr constant %__vtable_grandparent zeroinitializer
    @__grandparent__init = unnamed_addr constant %grandparent zeroinitializer, !dbg !0
    @__vtable_grandparent_instance = global %__vtable_grandparent zeroinitializer
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent zeroinitializer, !dbg !16
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child zeroinitializer, !dbg !27
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @grandparent(%grandparent* %0) !dbg !38 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !42, metadata !DIExpression()), !dbg !43
      %this = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %this, align 8
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 2
      ret void, !dbg !43
    }

    define void @parent(%parent* %0) !dbg !44 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !47, metadata !DIExpression()), !dbg !48
      %this = alloca %parent*, align 8
      store %parent* %0, %parent** %this, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void, !dbg !48
    }

    define void @child(%child* %0) !dbg !49 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !52, metadata !DIExpression()), !dbg !53
      %this = alloca %child*, align 8
      store %child* %0, %child** %this, align 8
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !53
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 1, !dbg !53
      %b = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2, !dbg !53
      %load_b = load i16, i16* %b, align 2, !dbg !53
      %1 = sext i16 %load_b to i32, !dbg !53
      %b1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2, !dbg !53
      %load_b2 = load i16, i16* %b1, align 2, !dbg !53
      %2 = sext i16 %load_b2 to i32, !dbg !53
      %tmpVar = mul i32 %2, 2, !dbg !53
      %tmpVar3 = mul i32 1, %tmpVar, !dbg !53
      %tmpVar4 = add i32 %tmpVar3, 0, !dbg !53
      %tmpVar5 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 %tmpVar4, !dbg !53
      %load_tmpVar = load i16, i16* %tmpVar5, align 2, !dbg !53
      %3 = sext i16 %load_tmpVar to i32, !dbg !53
      %tmpVar6 = add i32 %1, %3, !dbg !53
      %__grandparent7 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !53
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent7, i32 0, i32 2, !dbg !53
      %load_a = load i16, i16* %a, align 2, !dbg !53
      %4 = sext i16 %load_a to i32, !dbg !53
      %tmpVar8 = sub i32 %tmpVar6, %4, !dbg !53
      %tmpVar9 = mul i32 1, %tmpVar8, !dbg !53
      %tmpVar10 = add i32 %tmpVar9, 0, !dbg !53
      %tmpVar11 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 %tmpVar10, !dbg !53
      store i16 20, i16* %tmpVar11, align 2, !dbg !53
      ret void, !dbg !54
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init___vtable_grandparent(%__vtable_grandparent* %0) {
    entry:
      %self = alloca %__vtable_grandparent*, align 8
      store %__vtable_grandparent* %0, %__vtable_grandparent** %self, align 8
      %deref = load %__vtable_grandparent*, %__vtable_grandparent** %self, align 8
      %__body = getelementptr inbounds %__vtable_grandparent, %__vtable_grandparent* %deref, i32 0, i32 0
      store void (%grandparent*)* @grandparent, void (%grandparent*)** %__body, align 8
      ret void
    }

    define void @__init___vtable_parent(%__vtable_parent* %0) {
    entry:
      %self = alloca %__vtable_parent*, align 8
      store %__vtable_parent* %0, %__vtable_parent** %self, align 8
      %deref = load %__vtable_parent*, %__vtable_parent** %self, align 8
      %__body = getelementptr inbounds %__vtable_parent, %__vtable_parent* %deref, i32 0, i32 0
      store void (%parent*)* @parent, void (%parent*)** %__body, align 8
      ret void
    }

    define void @__init___vtable_child(%__vtable_child* %0) {
    entry:
      %self = alloca %__vtable_child*, align 8
      store %__vtable_child* %0, %__vtable_child** %self, align 8
      %deref = load %__vtable_child*, %__vtable_child** %self, align 8
      %__body = getelementptr inbounds %__vtable_child, %__vtable_child* %deref, i32 0, i32 0
      store void (%child*)* @child, void (%child*)** %__body, align 8
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

    !llvm.module.flags = !{!34, !35}
    !llvm.dbg.cu = !{!36}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 192, align: 64, flags: DIFlagPublic, elements: !5, identifier: "grandparent")
    !5 = !{!6, !10, !15}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____grandparent___vtable", scope: !2, file: !2, baseType: !8, align: 64)
    !8 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__grandparent___vtable", baseType: !9, size: 64, align: 64, dwarfAddressSpace: 1)
    !9 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !11, size: 96, align: 16, offset: 64, flags: DIFlagPublic)
    !11 = !DICompositeType(tag: DW_TAG_array_type, baseType: !12, size: 96, align: 16, elements: !13)
    !12 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !13 = !{!14}
    !14 = !DISubrange(count: 6, lowerBound: 0)
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !12, size: 16, align: 16, offset: 160, flags: DIFlagPublic)
    !16 = !DIGlobalVariableExpression(var: !17, expr: !DIExpression())
    !17 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !18, isLocal: false, isDefinition: true)
    !18 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !19)
    !19 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 384, align: 64, flags: DIFlagPublic, elements: !20, identifier: "parent")
    !20 = !{!21, !22, !26}
    !21 = !DIDerivedType(tag: DW_TAG_member, name: "__grandparent", scope: !2, file: !2, baseType: !4, size: 192, align: 64, flags: DIFlagPublic)
    !22 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !23, size: 176, align: 16, offset: 192, flags: DIFlagPublic)
    !23 = !DICompositeType(tag: DW_TAG_array_type, baseType: !12, size: 176, align: 16, elements: !24)
    !24 = !{!25}
    !25 = !DISubrange(count: 11, lowerBound: 0)
    !26 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !12, size: 16, align: 16, offset: 368, flags: DIFlagPublic)
    !27 = !DIGlobalVariableExpression(var: !28, expr: !DIExpression())
    !28 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !29, isLocal: false, isDefinition: true)
    !29 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !30)
    !30 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 576, align: 64, flags: DIFlagPublic, elements: !31, identifier: "child")
    !31 = !{!32, !33}
    !32 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !19, size: 384, align: 64, flags: DIFlagPublic)
    !33 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !23, size: 176, align: 16, offset: 384, flags: DIFlagPublic)
    !34 = !{i32 2, !"Dwarf Version", i32 5}
    !35 = !{i32 2, !"Debug Info Version", i32 3}
    !36 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !37, splitDebugInlining: false)
    !37 = !{!0, !16, !27}
    !38 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !39, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !36, retainedNodes: !41)
    !39 = !DISubroutineType(flags: DIFlagPublic, types: !40)
    !40 = !{null, !4}
    !41 = !{}
    !42 = !DILocalVariable(name: "grandparent", scope: !38, file: !2, line: 7, type: !4)
    !43 = !DILocation(line: 7, column: 8, scope: !38)
    !44 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !45, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !36, retainedNodes: !41)
    !45 = !DISubroutineType(flags: DIFlagPublic, types: !46)
    !46 = !{null, !19}
    !47 = !DILocalVariable(name: "parent", scope: !44, file: !2, line: 14, type: !19)
    !48 = !DILocation(line: 14, column: 8, scope: !44)
    !49 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !50, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !36, retainedNodes: !41)
    !50 = !DISubroutineType(flags: DIFlagPublic, types: !51)
    !51 = !{null, !30}
    !52 = !DILocalVariable(name: "child", scope: !49, file: !2, line: 20, type: !30)
    !53 = !DILocation(line: 20, column: 12, scope: !49)
    !54 = !DILocation(line: 21, column: 8, scope: !49)
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

    %__vtable_foo = type { void (%foo*)*, void (%foo*)* }
    %foo = type { i32* }
    %__vtable_bar = type { void (%bar*)*, void (%foo*)* }
    %bar = type { %foo }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !0
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @____vtable_bar__init = unnamed_addr constant %__vtable_bar zeroinitializer
    @__bar__init = unnamed_addr constant %bar zeroinitializer, !dbg !10
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer

    define void @foo(%foo* %0) !dbg !20 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !24, metadata !DIExpression()), !dbg !25
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void, !dbg !25
    }

    define void @foo__baz(%foo* %0) !dbg !26 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !27, metadata !DIExpression()), !dbg !28
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void, !dbg !28
    }

    define void @bar(%bar* %0) !dbg !29 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !32, metadata !DIExpression()), !dbg !33
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void, !dbg !33
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init___vtable_foo(%__vtable_foo* %0) {
    entry:
      %self = alloca %__vtable_foo*, align 8
      store %__vtable_foo* %0, %__vtable_foo** %self, align 8
      %deref = load %__vtable_foo*, %__vtable_foo** %self, align 8
      %__body = getelementptr inbounds %__vtable_foo, %__vtable_foo* %deref, i32 0, i32 0
      store void (%foo*)* @foo, void (%foo*)** %__body, align 8
      %deref1 = load %__vtable_foo*, %__vtable_foo** %self, align 8
      %baz = getelementptr inbounds %__vtable_foo, %__vtable_foo* %deref1, i32 0, i32 1
      store void (%foo*)* @foo__baz, void (%foo*)** %baz, align 8
      ret void
    }

    define void @__init___vtable_bar(%__vtable_bar* %0) {
    entry:
      %self = alloca %__vtable_bar*, align 8
      store %__vtable_bar* %0, %__vtable_bar** %self, align 8
      %deref = load %__vtable_bar*, %__vtable_bar** %self, align 8
      %__body = getelementptr inbounds %__vtable_bar, %__vtable_bar* %deref, i32 0, i32 0
      store void (%bar*)* @bar, void (%bar*)** %__body, align 8
      %deref1 = load %__vtable_bar*, %__vtable_bar** %self, align 8
      %baz = getelementptr inbounds %__vtable_bar, %__vtable_bar* %deref1, i32 0, i32 1
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

    !llvm.module.flags = !{!16, !17}
    !llvm.dbg.cu = !{!18}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 64, align: 64, flags: DIFlagPublic, elements: !5, identifier: "foo")
    !5 = !{!6}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____foo___vtable", scope: !2, file: !2, baseType: !8, align: 64)
    !8 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__foo___vtable", baseType: !9, size: 64, align: 64, dwarfAddressSpace: 1)
    !9 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !10 = !DIGlobalVariableExpression(var: !11, expr: !DIExpression())
    !11 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 7, type: !12, isLocal: false, isDefinition: true)
    !12 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !13)
    !13 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 7, size: 64, align: 64, flags: DIFlagPublic, elements: !14, identifier: "bar")
    !14 = !{!15}
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !4, size: 64, align: 64, flags: DIFlagPublic)
    !16 = !{i32 2, !"Dwarf Version", i32 5}
    !17 = !{i32 2, !"Debug Info Version", i32 3}
    !18 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !19, splitDebugInlining: false)
    !19 = !{!0, !10}
    !20 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !21, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !18, retainedNodes: !23)
    !21 = !DISubroutineType(flags: DIFlagPublic, types: !22)
    !22 = !{null, !4}
    !23 = !{}
    !24 = !DILocalVariable(name: "foo", scope: !20, file: !2, line: 5, type: !4)
    !25 = !DILocation(line: 5, column: 8, scope: !20)
    !26 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !20, file: !2, line: 3, type: !21, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !18, retainedNodes: !23)
    !27 = !DILocalVariable(name: "foo", scope: !26, file: !2, line: 4, type: !4)
    !28 = !DILocation(line: 4, column: 8, scope: !26)
    !29 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 7, type: !30, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !18, retainedNodes: !23)
    !30 = !DISubroutineType(flags: DIFlagPublic, types: !31)
    !31 = !{null, !13}
    !32 = !DILocalVariable(name: "bar", scope: !29, file: !2, line: 8, type: !13)
    !33 = !DILocation(line: 8, column: 8, scope: !29)
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

    %__vtable_parent = type { void (%parent*)* }
    %parent = type { i32*, i32 }
    %__vtable_child = type { void (%child*)* }
    %child = type { %parent, i32 }
    %__vtable_grandchild = type { void (%grandchild*)* }
    %grandchild = type { %child, i32 }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent zeroinitializer, !dbg !0
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child zeroinitializer, !dbg !12
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @____vtable_grandchild__init = unnamed_addr constant %__vtable_grandchild zeroinitializer
    @__grandchild__init = unnamed_addr constant %grandchild zeroinitializer, !dbg !19
    @__vtable_grandchild_instance = global %__vtable_grandchild zeroinitializer

    define void @parent(%parent* %0) !dbg !30 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !34, metadata !DIExpression()), !dbg !35
      %this = alloca %parent*, align 8
      store %parent* %0, %parent** %this, align 8
      %__vtable = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %a = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      ret void, !dbg !35
    }

    define void @child(%child* %0) !dbg !36 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !39, metadata !DIExpression()), !dbg !40
      %this = alloca %child*, align 8
      store %child* %0, %child** %this, align 8
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %b = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      ret void, !dbg !40
    }

    define void @grandchild(%grandchild* %0) !dbg !41 {
    entry:
      call void @llvm.dbg.declare(metadata %grandchild* %0, metadata !44, metadata !DIExpression()), !dbg !45
      %this = alloca %grandchild*, align 8
      store %grandchild* %0, %grandchild** %this, align 8
      %__child = getelementptr inbounds %grandchild, %grandchild* %0, i32 0, i32 0
      %c = getelementptr inbounds %grandchild, %grandchild* %0, i32 0, i32 1
      ret void, !dbg !45
    }

    define i32 @main() !dbg !46 {
    entry:
      %main = alloca i32, align 4
      %array_of_parent = alloca [3 x %parent], align 8
      %array_of_child = alloca [3 x %child], align 8
      %array_of_grandchild = alloca [3 x %grandchild], align 8
      %parent1 = alloca %parent, align 8
      %child1 = alloca %child, align 8
      %grandchild1 = alloca %grandchild, align 8
      call void @llvm.dbg.declare(metadata [3 x %parent]* %array_of_parent, metadata !49, metadata !DIExpression()), !dbg !53
      %0 = bitcast [3 x %parent]* %array_of_parent to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([3 x %parent]* getelementptr ([3 x %parent], [3 x %parent]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata [3 x %child]* %array_of_child, metadata !54, metadata !DIExpression()), !dbg !56
      %1 = bitcast [3 x %child]* %array_of_child to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %1, i8 0, i64 ptrtoint ([3 x %child]* getelementptr ([3 x %child], [3 x %child]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata [3 x %grandchild]* %array_of_grandchild, metadata !57, metadata !DIExpression()), !dbg !59
      %2 = bitcast [3 x %grandchild]* %array_of_grandchild to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %2, i8 0, i64 ptrtoint ([3 x %grandchild]* getelementptr ([3 x %grandchild], [3 x %grandchild]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %parent* %parent1, metadata !60, metadata !DIExpression()), !dbg !61
      %3 = bitcast %parent* %parent1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %3, i8* align 1 bitcast (%parent* @__parent__init to i8*), i64 ptrtoint (%parent* getelementptr (%parent, %parent* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %child* %child1, metadata !62, metadata !DIExpression()), !dbg !63
      %4 = bitcast %child* %child1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %4, i8* align 1 bitcast (%child* @__child__init to i8*), i64 ptrtoint (%child* getelementptr (%child, %child* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %grandchild* %grandchild1, metadata !64, metadata !DIExpression()), !dbg !65
      %5 = bitcast %grandchild* %grandchild1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %5, i8* align 1 bitcast (%grandchild* @__grandchild__init to i8*), i64 ptrtoint (%grandchild* getelementptr (%grandchild, %grandchild* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata i32* %main, metadata !66, metadata !DIExpression()), !dbg !67
      store i32 0, i32* %main, align 4
      call void @__init_parent(%parent* %parent1), !dbg !68
      call void @__init_child(%child* %child1), !dbg !68
      call void @__init_grandchild(%grandchild* %grandchild1), !dbg !68
      call void @__user_init_parent(%parent* %parent1), !dbg !68
      call void @__user_init_child(%child* %child1), !dbg !68
      call void @__user_init_grandchild(%grandchild* %grandchild1), !dbg !68
      %a = getelementptr inbounds %parent, %parent* %parent1, i32 0, i32 1, !dbg !69
      store i32 1, i32* %a, align 4, !dbg !69
      %__parent = getelementptr inbounds %child, %child* %child1, i32 0, i32 0, !dbg !70
      %a1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 1, !dbg !70
      store i32 2, i32* %a1, align 4, !dbg !70
      %b = getelementptr inbounds %child, %child* %child1, i32 0, i32 1, !dbg !71
      store i32 3, i32* %b, align 4, !dbg !71
      %__child = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 0, !dbg !72
      %__parent2 = getelementptr inbounds %child, %child* %__child, i32 0, i32 0, !dbg !72
      %a3 = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 1, !dbg !72
      store i32 4, i32* %a3, align 4, !dbg !72
      %__child4 = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 0, !dbg !73
      %b5 = getelementptr inbounds %child, %child* %__child4, i32 0, i32 1, !dbg !73
      store i32 5, i32* %b5, align 4, !dbg !73
      %c = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 1, !dbg !74
      store i32 6, i32* %c, align 4, !dbg !74
      %tmpVar = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 0, !dbg !75
      %a6 = getelementptr inbounds %parent, %parent* %tmpVar, i32 0, i32 1, !dbg !75
      store i32 7, i32* %a6, align 4, !dbg !75
      %tmpVar7 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 0, !dbg !76
      %__parent8 = getelementptr inbounds %child, %child* %tmpVar7, i32 0, i32 0, !dbg !76
      %a9 = getelementptr inbounds %parent, %parent* %__parent8, i32 0, i32 1, !dbg !76
      store i32 8, i32* %a9, align 4, !dbg !76
      %tmpVar10 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 0, !dbg !77
      %b11 = getelementptr inbounds %child, %child* %tmpVar10, i32 0, i32 1, !dbg !77
      store i32 9, i32* %b11, align 4, !dbg !77
      %tmpVar12 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !78
      %__child13 = getelementptr inbounds %grandchild, %grandchild* %tmpVar12, i32 0, i32 0, !dbg !78
      %__parent14 = getelementptr inbounds %child, %child* %__child13, i32 0, i32 0, !dbg !78
      %a15 = getelementptr inbounds %parent, %parent* %__parent14, i32 0, i32 1, !dbg !78
      store i32 10, i32* %a15, align 4, !dbg !78
      %tmpVar16 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !79
      %__child17 = getelementptr inbounds %grandchild, %grandchild* %tmpVar16, i32 0, i32 0, !dbg !79
      %b18 = getelementptr inbounds %child, %child* %__child17, i32 0, i32 1, !dbg !79
      store i32 11, i32* %b18, align 4, !dbg !79
      %tmpVar19 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !80
      %c20 = getelementptr inbounds %grandchild, %grandchild* %tmpVar19, i32 0, i32 1, !dbg !80
      store i32 12, i32* %c20, align 4, !dbg !80
      %tmpVar21 = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 1, !dbg !81
      %a22 = getelementptr inbounds %parent, %parent* %tmpVar21, i32 0, i32 1, !dbg !81
      store i32 13, i32* %a22, align 4, !dbg !81
      %tmpVar23 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 1, !dbg !82
      %__parent24 = getelementptr inbounds %child, %child* %tmpVar23, i32 0, i32 0, !dbg !82
      %a25 = getelementptr inbounds %parent, %parent* %__parent24, i32 0, i32 1, !dbg !82
      store i32 14, i32* %a25, align 4, !dbg !82
      %tmpVar26 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 1, !dbg !83
      %b27 = getelementptr inbounds %child, %child* %tmpVar26, i32 0, i32 1, !dbg !83
      store i32 15, i32* %b27, align 4, !dbg !83
      %tmpVar28 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !84
      %__child29 = getelementptr inbounds %grandchild, %grandchild* %tmpVar28, i32 0, i32 0, !dbg !84
      %__parent30 = getelementptr inbounds %child, %child* %__child29, i32 0, i32 0, !dbg !84
      %a31 = getelementptr inbounds %parent, %parent* %__parent30, i32 0, i32 1, !dbg !84
      store i32 16, i32* %a31, align 4, !dbg !84
      %tmpVar32 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !85
      %__child33 = getelementptr inbounds %grandchild, %grandchild* %tmpVar32, i32 0, i32 0, !dbg !85
      %b34 = getelementptr inbounds %child, %child* %__child33, i32 0, i32 1, !dbg !85
      store i32 17, i32* %b34, align 4, !dbg !85
      %tmpVar35 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !86
      %c36 = getelementptr inbounds %grandchild, %grandchild* %tmpVar35, i32 0, i32 1, !dbg !86
      store i32 18, i32* %c36, align 4, !dbg !86
      %tmpVar37 = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 2, !dbg !87
      %a38 = getelementptr inbounds %parent, %parent* %tmpVar37, i32 0, i32 1, !dbg !87
      store i32 19, i32* %a38, align 4, !dbg !87
      %tmpVar39 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 2, !dbg !88
      %__parent40 = getelementptr inbounds %child, %child* %tmpVar39, i32 0, i32 0, !dbg !88
      %a41 = getelementptr inbounds %parent, %parent* %__parent40, i32 0, i32 1, !dbg !88
      store i32 20, i32* %a41, align 4, !dbg !88
      %tmpVar42 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 2, !dbg !89
      %b43 = getelementptr inbounds %child, %child* %tmpVar42, i32 0, i32 1, !dbg !89
      store i32 21, i32* %b43, align 4, !dbg !89
      %tmpVar44 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !90
      %__child45 = getelementptr inbounds %grandchild, %grandchild* %tmpVar44, i32 0, i32 0, !dbg !90
      %__parent46 = getelementptr inbounds %child, %child* %__child45, i32 0, i32 0, !dbg !90
      %a47 = getelementptr inbounds %parent, %parent* %__parent46, i32 0, i32 1, !dbg !90
      store i32 22, i32* %a47, align 4, !dbg !90
      %tmpVar48 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !91
      %__child49 = getelementptr inbounds %grandchild, %grandchild* %tmpVar48, i32 0, i32 0, !dbg !91
      %b50 = getelementptr inbounds %child, %child* %__child49, i32 0, i32 1, !dbg !91
      store i32 23, i32* %b50, align 4, !dbg !91
      %tmpVar51 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !92
      %c52 = getelementptr inbounds %grandchild, %grandchild* %tmpVar51, i32 0, i32 1, !dbg !92
      store i32 24, i32* %c52, align 4, !dbg !92
      %main_ret = load i32, i32* %main, align 4, !dbg !93
      ret i32 %main_ret, !dbg !93
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
      %deref = load %__vtable_parent*, %__vtable_parent** %self, align 8
      %__body = getelementptr inbounds %__vtable_parent, %__vtable_parent* %deref, i32 0, i32 0
      store void (%parent*)* @parent, void (%parent*)** %__body, align 8
      ret void
    }

    define void @__init___vtable_child(%__vtable_child* %0) {
    entry:
      %self = alloca %__vtable_child*, align 8
      store %__vtable_child* %0, %__vtable_child** %self, align 8
      %deref = load %__vtable_child*, %__vtable_child** %self, align 8
      %__body = getelementptr inbounds %__vtable_child, %__vtable_child* %deref, i32 0, i32 0
      store void (%child*)* @child, void (%child*)** %__body, align 8
      ret void
    }

    define void @__init___vtable_grandchild(%__vtable_grandchild* %0) {
    entry:
      %self = alloca %__vtable_grandchild*, align 8
      store %__vtable_grandchild* %0, %__vtable_grandchild** %self, align 8
      %deref = load %__vtable_grandchild*, %__vtable_grandchild** %self, align 8
      %__body = getelementptr inbounds %__vtable_grandchild, %__vtable_grandchild* %deref, i32 0, i32 0
      store void (%grandchild*)* @grandchild, void (%grandchild*)** %__body, align 8
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

    !llvm.module.flags = !{!26, !27}
    !llvm.dbg.cu = !{!28}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 2, size: 128, align: 64, flags: DIFlagPublic, elements: !5, identifier: "parent")
    !5 = !{!6, !10}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____parent___vtable", scope: !2, file: !2, baseType: !8, align: 64)
    !8 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__parent___vtable", baseType: !9, size: 64, align: 64, dwarfAddressSpace: 1)
    !9 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 4, baseType: !11, size: 32, align: 32, offset: 64, flags: DIFlagPublic)
    !11 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !12 = !DIGlobalVariableExpression(var: !13, expr: !DIExpression())
    !13 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 8, type: !14, isLocal: false, isDefinition: true)
    !14 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !15)
    !15 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 8, size: 192, align: 64, flags: DIFlagPublic, elements: !16, identifier: "child")
    !16 = !{!17, !18}
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !4, size: 128, align: 64, flags: DIFlagPublic)
    !18 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 10, baseType: !11, size: 32, align: 32, offset: 128, flags: DIFlagPublic)
    !19 = !DIGlobalVariableExpression(var: !20, expr: !DIExpression())
    !20 = distinct !DIGlobalVariable(name: "__grandchild__init", scope: !2, file: !2, line: 14, type: !21, isLocal: false, isDefinition: true)
    !21 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !22)
    !22 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandchild", scope: !2, file: !2, line: 14, size: 256, align: 64, flags: DIFlagPublic, elements: !23, identifier: "grandchild")
    !23 = !{!24, !25}
    !24 = !DIDerivedType(tag: DW_TAG_member, name: "__child", scope: !2, file: !2, baseType: !15, size: 192, align: 64, flags: DIFlagPublic)
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 16, baseType: !11, size: 32, align: 32, offset: 192, flags: DIFlagPublic)
    !26 = !{i32 2, !"Dwarf Version", i32 5}
    !27 = !{i32 2, !"Debug Info Version", i32 3}
    !28 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !29, splitDebugInlining: false)
    !29 = !{!0, !12, !19}
    !30 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 2, type: !31, scopeLine: 6, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !28, retainedNodes: !33)
    !31 = !DISubroutineType(flags: DIFlagPublic, types: !32)
    !32 = !{null, !4}
    !33 = !{}
    !34 = !DILocalVariable(name: "parent", scope: !30, file: !2, line: 6, type: !4)
    !35 = !DILocation(line: 6, scope: !30)
    !36 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 8, type: !37, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !28, retainedNodes: !33)
    !37 = !DISubroutineType(flags: DIFlagPublic, types: !38)
    !38 = !{null, !15}
    !39 = !DILocalVariable(name: "child", scope: !36, file: !2, line: 12, type: !15)
    !40 = !DILocation(line: 12, scope: !36)
    !41 = distinct !DISubprogram(name: "grandchild", linkageName: "grandchild", scope: !2, file: !2, line: 14, type: !42, scopeLine: 18, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !28, retainedNodes: !33)
    !42 = !DISubroutineType(flags: DIFlagPublic, types: !43)
    !43 = !{null, !22}
    !44 = !DILocalVariable(name: "grandchild", scope: !41, file: !2, line: 18, type: !22)
    !45 = !DILocation(line: 18, scope: !41)
    !46 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 20, type: !47, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !28, retainedNodes: !33)
    !47 = !DISubroutineType(flags: DIFlagPublic, types: !48)
    !48 = !{null}
    !49 = !DILocalVariable(name: "array_of_parent", scope: !46, file: !2, line: 22, type: !50, align: 64)
    !50 = !DICompositeType(tag: DW_TAG_array_type, baseType: !4, size: 384, align: 64, elements: !51)
    !51 = !{!52}
    !52 = !DISubrange(count: 3, lowerBound: 0)
    !53 = !DILocation(line: 22, column: 4, scope: !46)
    !54 = !DILocalVariable(name: "array_of_child", scope: !46, file: !2, line: 23, type: !55, align: 64)
    !55 = !DICompositeType(tag: DW_TAG_array_type, baseType: !15, size: 576, align: 64, elements: !51)
    !56 = !DILocation(line: 23, column: 4, scope: !46)
    !57 = !DILocalVariable(name: "array_of_grandchild", scope: !46, file: !2, line: 24, type: !58, align: 64)
    !58 = !DICompositeType(tag: DW_TAG_array_type, baseType: !22, size: 768, align: 64, elements: !51)
    !59 = !DILocation(line: 24, column: 4, scope: !46)
    !60 = !DILocalVariable(name: "parent1", scope: !46, file: !2, line: 25, type: !4, align: 64)
    !61 = !DILocation(line: 25, column: 4, scope: !46)
    !62 = !DILocalVariable(name: "child1", scope: !46, file: !2, line: 26, type: !15, align: 64)
    !63 = !DILocation(line: 26, column: 4, scope: !46)
    !64 = !DILocalVariable(name: "grandchild1", scope: !46, file: !2, line: 27, type: !22, align: 64)
    !65 = !DILocation(line: 27, column: 4, scope: !46)
    !66 = !DILocalVariable(name: "main", scope: !46, file: !2, line: 20, type: !11, align: 32)
    !67 = !DILocation(line: 20, column: 9, scope: !46)
    !68 = !DILocation(line: 0, scope: !46)
    !69 = !DILocation(line: 30, column: 4, scope: !46)
    !70 = !DILocation(line: 31, column: 4, scope: !46)
    !71 = !DILocation(line: 32, column: 4, scope: !46)
    !72 = !DILocation(line: 33, column: 4, scope: !46)
    !73 = !DILocation(line: 34, column: 4, scope: !46)
    !74 = !DILocation(line: 35, column: 4, scope: !46)
    !75 = !DILocation(line: 37, column: 4, scope: !46)
    !76 = !DILocation(line: 38, column: 4, scope: !46)
    !77 = !DILocation(line: 39, column: 4, scope: !46)
    !78 = !DILocation(line: 40, column: 4, scope: !46)
    !79 = !DILocation(line: 41, column: 4, scope: !46)
    !80 = !DILocation(line: 42, column: 4, scope: !46)
    !81 = !DILocation(line: 43, column: 4, scope: !46)
    !82 = !DILocation(line: 44, column: 4, scope: !46)
    !83 = !DILocation(line: 45, column: 4, scope: !46)
    !84 = !DILocation(line: 46, column: 4, scope: !46)
    !85 = !DILocation(line: 47, column: 4, scope: !46)
    !86 = !DILocation(line: 48, column: 4, scope: !46)
    !87 = !DILocation(line: 49, column: 4, scope: !46)
    !88 = !DILocation(line: 50, column: 4, scope: !46)
    !89 = !DILocation(line: 51, column: 4, scope: !46)
    !90 = !DILocation(line: 52, column: 4, scope: !46)
    !91 = !DILocation(line: 53, column: 4, scope: !46)
    !92 = !DILocation(line: 54, column: 4, scope: !46)
    !93 = !DILocation(line: 56, scope: !46)
    "#);
}
