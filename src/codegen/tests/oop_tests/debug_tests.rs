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

    %foo = type { i32*, i16, [81 x i8], [11 x [81 x i8]] }
    %bar = type { %foo }
    %__vtable_foo_type = type { i32* }
    %__vtable_bar_type = type { %__vtable_foo_type, i32* }

    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !0
    @__bar__init = unnamed_addr constant %bar zeroinitializer, !dbg !20
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = unnamed_addr constant %__vtable_foo_type zeroinitializer, !dbg !26
    @__vtable_foo = global %__vtable_foo_type zeroinitializer, !dbg !32
    @____vtable_bar_type__init = unnamed_addr constant %__vtable_bar_type zeroinitializer, !dbg !34
    @__vtable_bar = global %__vtable_bar_type zeroinitializer, !dbg !41

    define void @foo(%foo* %0) !dbg !47 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !51, metadata !DIExpression()), !dbg !52
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %a = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %b = getelementptr inbounds %foo, %foo* %0, i32 0, i32 2
      %c = getelementptr inbounds %foo, %foo* %0, i32 0, i32 3
      ret void, !dbg !52
    }

    define void @bar(%bar* %0) !dbg !53 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !56, metadata !DIExpression()), !dbg !57
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void, !dbg !57
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init___vtable_bar_type(%__vtable_bar_type* %0) {
    entry:
      %self = alloca %__vtable_bar_type*, align 8
      store %__vtable_bar_type* %0, %__vtable_bar_type** %self, align 8
      %deref = load %__vtable_bar_type*, %__vtable_bar_type** %self, align 8
      %__vtable_foo_type = getelementptr inbounds %__vtable_bar_type, %__vtable_bar_type* %deref, i32 0, i32 0
      call void @__init___vtable_foo_type(%__vtable_foo_type* %__vtable_foo_type)
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
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
      store i32* bitcast (%__vtable_bar_type* @__vtable_bar to i32*), i32** %__vtable, align 8
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
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      call void @__init___vtable_bar_type(%__vtable_bar_type* @__vtable_bar)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!43, !44}
    !llvm.dbg.cu = !{!45}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 7872, align: 64, flags: DIFlagPublic, elements: !5, identifier: "foo")
    !5 = !{!6, !9, !11, !16}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !8, size: 64, align: 64, dwarfAddressSpace: 1)
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
    !26 = !DIGlobalVariableExpression(var: !27, expr: !DIExpression())
    !27 = distinct !DIGlobalVariable(name: "____vtable_foo_type__init", scope: !2, file: !2, type: !28, isLocal: false, isDefinition: true)
    !28 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !29)
    !29 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !30, identifier: "__vtable_foo_type")
    !30 = !{!31}
    !31 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !32 = !DIGlobalVariableExpression(var: !33, expr: !DIExpression())
    !33 = distinct !DIGlobalVariable(name: "__vtable_foo", scope: !2, file: !2, type: !29, isLocal: false, isDefinition: true)
    !34 = !DIGlobalVariableExpression(var: !35, expr: !DIExpression())
    !35 = distinct !DIGlobalVariable(name: "____vtable_bar_type__init", scope: !2, file: !2, type: !36, isLocal: false, isDefinition: true)
    !36 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !37)
    !37 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_bar_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !38, identifier: "__vtable_bar_type")
    !38 = !{!39, !40}
    !39 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_foo_type", scope: !2, file: !2, baseType: !29, size: 64, align: 64, flags: DIFlagPublic)
    !40 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !7, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !41 = !DIGlobalVariableExpression(var: !42, expr: !DIExpression())
    !42 = distinct !DIGlobalVariable(name: "__vtable_bar", scope: !2, file: !2, type: !37, isLocal: false, isDefinition: true)
    !43 = !{i32 2, !"Dwarf Version", i32 5}
    !44 = !{i32 2, !"Debug Info Version", i32 3}
    !45 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !46, splitDebugInlining: false)
    !46 = !{!32, !26, !41, !34, !0, !20}
    !47 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !48, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !45, retainedNodes: !50)
    !48 = !DISubroutineType(flags: DIFlagPublic, types: !49)
    !49 = !{null, !4}
    !50 = !{}
    !51 = !DILocalVariable(name: "foo", scope: !47, file: !2, line: 8, type: !4)
    !52 = !DILocation(line: 8, column: 8, scope: !47)
    !53 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 10, type: !54, scopeLine: 11, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !45, retainedNodes: !50)
    !54 = !DISubroutineType(flags: DIFlagPublic, types: !55)
    !55 = !{null, !23}
    !56 = !DILocalVariable(name: "bar", scope: !53, file: !2, line: 11, type: !23)
    !57 = !DILocation(line: 11, column: 8, scope: !53)
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

    %fb2 = type { %fb }
    %fb = type { i32*, i16, i16 }
    %foo = type { i32*, %fb2 }
    %__vtable_fb_type = type { i32* }
    %__vtable_fb2_type = type { %__vtable_fb_type, i32* }
    %__vtable_foo_type = type { i32* }

    @__fb2__init = unnamed_addr constant %fb2 zeroinitializer, !dbg !0
    @__fb__init = unnamed_addr constant %fb zeroinitializer, !dbg !15
    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !18
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_fb_type__init = unnamed_addr constant %__vtable_fb_type zeroinitializer, !dbg !24
    @__vtable_fb = global %__vtable_fb_type zeroinitializer, !dbg !30
    @____vtable_fb2_type__init = unnamed_addr constant %__vtable_fb2_type zeroinitializer, !dbg !32
    @__vtable_fb2 = global %__vtable_fb2_type zeroinitializer, !dbg !39
    @____vtable_foo_type__init = unnamed_addr constant %__vtable_foo_type zeroinitializer, !dbg !41
    @__vtable_foo = global %__vtable_foo_type zeroinitializer, !dbg !45

    define void @fb(%fb* %0) !dbg !51 {
    entry:
      call void @llvm.dbg.declare(metadata %fb* %0, metadata !55, metadata !DIExpression()), !dbg !56
      %this = alloca %fb*, align 8
      store %fb* %0, %fb** %this, align 8
      %__vtable = getelementptr inbounds %fb, %fb* %0, i32 0, i32 0
      %x = getelementptr inbounds %fb, %fb* %0, i32 0, i32 1
      %y = getelementptr inbounds %fb, %fb* %0, i32 0, i32 2
      ret void, !dbg !56
    }

    define void @fb2(%fb2* %0) !dbg !57 {
    entry:
      call void @llvm.dbg.declare(metadata %fb2* %0, metadata !60, metadata !DIExpression()), !dbg !61
      %this = alloca %fb2*, align 8
      store %fb2* %0, %fb2** %this, align 8
      %__fb = getelementptr inbounds %fb2, %fb2* %0, i32 0, i32 0
      ret void, !dbg !61
    }

    define void @foo(%foo* %0) !dbg !62 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !65, metadata !DIExpression()), !dbg !66
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %myFb = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %__fb = getelementptr inbounds %fb2, %fb2* %myFb, i32 0, i32 0, !dbg !66
      %x = getelementptr inbounds %fb, %fb* %__fb, i32 0, i32 1, !dbg !66
      store i16 1, i16* %x, align 2, !dbg !66
      ret void, !dbg !67
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init___vtable_fb_type(%__vtable_fb_type* %0) {
    entry:
      %self = alloca %__vtable_fb_type*, align 8
      store %__vtable_fb_type* %0, %__vtable_fb_type** %self, align 8
      ret void
    }

    define void @__init___vtable_fb2_type(%__vtable_fb2_type* %0) {
    entry:
      %self = alloca %__vtable_fb2_type*, align 8
      store %__vtable_fb2_type* %0, %__vtable_fb2_type** %self, align 8
      %deref = load %__vtable_fb2_type*, %__vtable_fb2_type** %self, align 8
      %__vtable_fb_type = getelementptr inbounds %__vtable_fb2_type, %__vtable_fb2_type* %deref, i32 0, i32 0
      call void @__init___vtable_fb_type(%__vtable_fb_type* %__vtable_fb_type)
      ret void
    }

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
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
      store i32* bitcast (%__vtable_fb2_type* @__vtable_fb2 to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_fb(%fb* %0) {
    entry:
      %self = alloca %fb*, align 8
      store %fb* %0, %fb** %self, align 8
      %deref = load %fb*, %fb** %self, align 8
      %__vtable = getelementptr inbounds %fb, %fb* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_fb_type* @__vtable_fb to i32*), i32** %__vtable, align 8
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
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
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
      %myFb = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 1
      call void @__user_init_fb2(%fb2* %myFb)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_fb_type(%__vtable_fb_type* @__vtable_fb)
      call void @__init___vtable_fb2_type(%__vtable_fb2_type* @__vtable_fb2)
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!47, !48}
    !llvm.dbg.cu = !{!49}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__fb2__init", scope: !2, file: !2, line: 9, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb2", scope: !2, file: !2, line: 9, size: 128, align: 64, flags: DIFlagPublic, elements: !5, identifier: "fb2")
    !5 = !{!6}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__fb", scope: !2, file: !2, baseType: !7, size: 128, align: 64, flags: DIFlagPublic)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb", scope: !2, file: !2, line: 2, size: 128, align: 64, flags: DIFlagPublic, elements: !8, identifier: "fb")
    !8 = !{!9, !12, !14}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !10, size: 64, align: 64, flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !11, size: 64, align: 64, dwarfAddressSpace: 1)
    !11 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 4, baseType: !13, size: 16, align: 16, offset: 64, flags: DIFlagPublic)
    !13 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 5, baseType: !13, size: 16, align: 16, offset: 80, flags: DIFlagPublic)
    !15 = !DIGlobalVariableExpression(var: !16, expr: !DIExpression())
    !16 = distinct !DIGlobalVariable(name: "__fb__init", scope: !2, file: !2, line: 2, type: !17, isLocal: false, isDefinition: true)
    !17 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !7)
    !18 = !DIGlobalVariableExpression(var: !19, expr: !DIExpression())
    !19 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 12, type: !20, isLocal: false, isDefinition: true)
    !20 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !21)
    !21 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 12, size: 192, align: 64, flags: DIFlagPublic, elements: !22, identifier: "foo")
    !22 = !{!9, !23}
    !23 = !DIDerivedType(tag: DW_TAG_member, name: "myFb", scope: !2, file: !2, line: 14, baseType: !4, size: 128, align: 64, offset: 64, flags: DIFlagPublic)
    !24 = !DIGlobalVariableExpression(var: !25, expr: !DIExpression())
    !25 = distinct !DIGlobalVariable(name: "____vtable_fb_type__init", scope: !2, file: !2, type: !26, isLocal: false, isDefinition: true)
    !26 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !27)
    !27 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_fb_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !28, identifier: "__vtable_fb_type")
    !28 = !{!29}
    !29 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !10, size: 64, align: 64, flags: DIFlagPublic)
    !30 = !DIGlobalVariableExpression(var: !31, expr: !DIExpression())
    !31 = distinct !DIGlobalVariable(name: "__vtable_fb", scope: !2, file: !2, type: !27, isLocal: false, isDefinition: true)
    !32 = !DIGlobalVariableExpression(var: !33, expr: !DIExpression())
    !33 = distinct !DIGlobalVariable(name: "____vtable_fb2_type__init", scope: !2, file: !2, type: !34, isLocal: false, isDefinition: true)
    !34 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !35)
    !35 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_fb2_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !36, identifier: "__vtable_fb2_type")
    !36 = !{!37, !38}
    !37 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_fb_type", scope: !2, file: !2, baseType: !27, size: 64, align: 64, flags: DIFlagPublic)
    !38 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !10, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !39 = !DIGlobalVariableExpression(var: !40, expr: !DIExpression())
    !40 = distinct !DIGlobalVariable(name: "__vtable_fb2", scope: !2, file: !2, type: !35, isLocal: false, isDefinition: true)
    !41 = !DIGlobalVariableExpression(var: !42, expr: !DIExpression())
    !42 = distinct !DIGlobalVariable(name: "____vtable_foo_type__init", scope: !2, file: !2, type: !43, isLocal: false, isDefinition: true)
    !43 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !44)
    !44 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !28, identifier: "__vtable_foo_type")
    !45 = !DIGlobalVariableExpression(var: !46, expr: !DIExpression())
    !46 = distinct !DIGlobalVariable(name: "__vtable_foo", scope: !2, file: !2, type: !44, isLocal: false, isDefinition: true)
    !47 = !{i32 2, !"Dwarf Version", i32 5}
    !48 = !{i32 2, !"Debug Info Version", i32 3}
    !49 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !50, splitDebugInlining: false)
    !50 = !{!30, !24, !39, !32, !45, !41, !15, !0, !18}
    !51 = distinct !DISubprogram(name: "fb", linkageName: "fb", scope: !2, file: !2, line: 2, type: !52, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !49, retainedNodes: !54)
    !52 = !DISubroutineType(flags: DIFlagPublic, types: !53)
    !53 = !{null, !7}
    !54 = !{}
    !55 = !DILocalVariable(name: "fb", scope: !51, file: !2, line: 7, type: !7)
    !56 = !DILocation(line: 7, column: 8, scope: !51)
    !57 = distinct !DISubprogram(name: "fb2", linkageName: "fb2", scope: !2, file: !2, line: 9, type: !58, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !49, retainedNodes: !54)
    !58 = !DISubroutineType(flags: DIFlagPublic, types: !59)
    !59 = !{null, !4}
    !60 = !DILocalVariable(name: "fb2", scope: !57, file: !2, line: 10, type: !4)
    !61 = !DILocation(line: 10, column: 8, scope: !57)
    !62 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 12, type: !63, scopeLine: 16, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !49, retainedNodes: !54)
    !63 = !DISubroutineType(flags: DIFlagPublic, types: !64)
    !64 = !{null, !21}
    !65 = !DILocalVariable(name: "foo", scope: !62, file: !2, line: 16, type: !21)
    !66 = !DILocation(line: 16, column: 12, scope: !62)
    !67 = !DILocation(line: 17, column: 8, scope: !62)
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

    %bar = type { %foo }
    %foo = type { i32*, [81 x i8] }
    %__vtable_foo_type = type { i32*, i32* }
    %__vtable_bar_type = type { %__vtable_foo_type, i32* }

    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"world\00"
    @__bar__init = unnamed_addr constant %bar zeroinitializer, !dbg !0
    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !17
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = unnamed_addr constant %__vtable_foo_type zeroinitializer, !dbg !20
    @__vtable_foo = global %__vtable_foo_type zeroinitializer, !dbg !27
    @____vtable_bar_type__init = unnamed_addr constant %__vtable_bar_type zeroinitializer, !dbg !29
    @__vtable_bar = global %__vtable_bar_type zeroinitializer, !dbg !36

    define void @foo(%foo* %0) !dbg !42 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !46, metadata !DIExpression()), !dbg !47
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      ret void, !dbg !47
    }

    define void @foo__baz(%foo* %0) !dbg !48 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !49, metadata !DIExpression()), !dbg !50
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %1 = bitcast [81 x i8]* %s to i8*, !dbg !50
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_0, i32 0, i32 0), i32 6, i1 false), !dbg !50
      ret void, !dbg !51
    }

    define void @bar(%bar* %0) !dbg !52 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !55, metadata !DIExpression()), !dbg !56
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %__foo, i32 0, i32 1, !dbg !56
      %1 = bitcast [81 x i8]* %s to i8*, !dbg !56
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_1, i32 0, i32 0), i32 6, i1 false), !dbg !56
      ret void, !dbg !57
    }

    define void @main() !dbg !58 {
    entry:
      %s = alloca [81 x i8], align 1
      %fb = alloca %bar, align 8
      call void @llvm.dbg.declare(metadata [81 x i8]* %s, metadata !61, metadata !DIExpression()), !dbg !62
      %0 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %bar* %fb, metadata !63, metadata !DIExpression()), !dbg !64
      %1 = bitcast %bar* %fb to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 bitcast (%bar* @__bar__init to i8*), i64 ptrtoint (%bar* getelementptr (%bar, %bar* null, i32 1) to i64), i1 false)
      call void @__init_bar(%bar* %fb), !dbg !65
      call void @__user_init_bar(%bar* %fb), !dbg !65
      %__foo = getelementptr inbounds %bar, %bar* %fb, i32 0, i32 0, !dbg !65
      call void @foo__baz(%foo* %__foo), !dbg !66
      call void @bar(%bar* %fb), !dbg !67
      ret void, !dbg !68
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #1

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #2

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init___vtable_bar_type(%__vtable_bar_type* %0) {
    entry:
      %self = alloca %__vtable_bar_type*, align 8
      store %__vtable_bar_type* %0, %__vtable_bar_type** %self, align 8
      %deref = load %__vtable_bar_type*, %__vtable_bar_type** %self, align 8
      %__vtable_foo_type = getelementptr inbounds %__vtable_bar_type, %__vtable_bar_type* %deref, i32 0, i32 0
      call void @__init___vtable_foo_type(%__vtable_foo_type* %__vtable_foo_type)
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
      store i32* bitcast (%__vtable_bar_type* @__vtable_bar to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
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
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      call void @__init___vtable_bar_type(%__vtable_bar_type* @__vtable_bar)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn }
    attributes #2 = { argmemonly nofree nounwind willreturn writeonly }

    !llvm.module.flags = !{!38, !39}
    !llvm.dbg.cu = !{!40}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 11, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 11, size: 768, align: 64, flags: DIFlagPublic, elements: !5, identifier: "bar")
    !5 = !{!6}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !7, size: 768, align: 64, flags: DIFlagPublic)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 768, align: 64, flags: DIFlagPublic, elements: !8, identifier: "foo")
    !8 = !{!9, !12}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !10, size: 64, align: 64, flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !11, size: 64, align: 64, dwarfAddressSpace: 1)
    !11 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !2, file: !2, line: 4, baseType: !13, size: 648, align: 8, offset: 64, flags: DIFlagPublic)
    !13 = !DICompositeType(tag: DW_TAG_array_type, baseType: !14, size: 648, align: 8, elements: !15)
    !14 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !15 = !{!16}
    !16 = !DISubrange(count: 81, lowerBound: 0)
    !17 = !DIGlobalVariableExpression(var: !18, expr: !DIExpression())
    !18 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !19, isLocal: false, isDefinition: true)
    !19 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !7)
    !20 = !DIGlobalVariableExpression(var: !21, expr: !DIExpression())
    !21 = distinct !DIGlobalVariable(name: "____vtable_foo_type__init", scope: !2, file: !2, type: !22, isLocal: false, isDefinition: true)
    !22 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !23)
    !23 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !24, identifier: "__vtable_foo_type")
    !24 = !{!25, !26}
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !10, size: 64, align: 64, flags: DIFlagPublic)
    !26 = !DIDerivedType(tag: DW_TAG_member, name: "foo.baz", scope: !2, file: !2, baseType: !10, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !27 = !DIGlobalVariableExpression(var: !28, expr: !DIExpression())
    !28 = distinct !DIGlobalVariable(name: "__vtable_foo", scope: !2, file: !2, type: !23, isLocal: false, isDefinition: true)
    !29 = !DIGlobalVariableExpression(var: !30, expr: !DIExpression())
    !30 = distinct !DIGlobalVariable(name: "____vtable_bar_type__init", scope: !2, file: !2, type: !31, isLocal: false, isDefinition: true)
    !31 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !32)
    !32 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_bar_type", scope: !2, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !33, identifier: "__vtable_bar_type")
    !33 = !{!34, !35}
    !34 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_foo_type", scope: !2, file: !2, baseType: !23, size: 128, align: 64, flags: DIFlagPublic)
    !35 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !10, size: 64, align: 64, offset: 128, flags: DIFlagPublic)
    !36 = !DIGlobalVariableExpression(var: !37, expr: !DIExpression())
    !37 = distinct !DIGlobalVariable(name: "__vtable_bar", scope: !2, file: !2, type: !32, isLocal: false, isDefinition: true)
    !38 = !{i32 2, !"Dwarf Version", i32 5}
    !39 = !{i32 2, !"Debug Info Version", i32 3}
    !40 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !41, splitDebugInlining: false)
    !41 = !{!27, !20, !36, !29, !17, !0}
    !42 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !43, scopeLine: 9, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !40, retainedNodes: !45)
    !43 = !DISubroutineType(flags: DIFlagPublic, types: !44)
    !44 = !{null, !7}
    !45 = !{}
    !46 = !DILocalVariable(name: "foo", scope: !42, file: !2, line: 9, type: !7)
    !47 = !DILocation(line: 9, column: 8, scope: !42)
    !48 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !42, file: !2, line: 6, type: !43, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !40, retainedNodes: !45)
    !49 = !DILocalVariable(name: "foo", scope: !48, file: !2, line: 7, type: !7)
    !50 = !DILocation(line: 7, column: 12, scope: !48)
    !51 = !DILocation(line: 8, column: 8, scope: !48)
    !52 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 11, type: !53, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !40, retainedNodes: !45)
    !53 = !DISubroutineType(flags: DIFlagPublic, types: !54)
    !54 = !{null, !4}
    !55 = !DILocalVariable(name: "bar", scope: !52, file: !2, line: 12, type: !4)
    !56 = !DILocation(line: 12, column: 12, scope: !52)
    !57 = !DILocation(line: 13, column: 8, scope: !52)
    !58 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 15, type: !59, scopeLine: 15, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !40, retainedNodes: !45)
    !59 = !DISubroutineType(flags: DIFlagPublic, types: !60)
    !60 = !{null}
    !61 = !DILocalVariable(name: "s", scope: !58, file: !2, line: 17, type: !13, align: 8)
    !62 = !DILocation(line: 17, column: 12, scope: !58)
    !63 = !DILocalVariable(name: "fb", scope: !58, file: !2, line: 18, type: !4, align: 64)
    !64 = !DILocation(line: 18, column: 12, scope: !58)
    !65 = !DILocation(line: 0, scope: !58)
    !66 = !DILocation(line: 20, column: 12, scope: !58)
    !67 = !DILocation(line: 21, column: 12, scope: !58)
    !68 = !DILocation(line: 22, column: 8, scope: !58)
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

    %child = type { %parent, [11 x i16] }
    %parent = type { %grandparent, [11 x i16], i16 }
    %grandparent = type { i32*, [6 x i16], i16 }
    %__vtable_grandparent_type = type { i32* }
    %__vtable_parent_type = type { %__vtable_grandparent_type, i32* }
    %__vtable_child_type = type { %__vtable_parent_type, i32* }

    @__child__init = unnamed_addr constant %child zeroinitializer, !dbg !0
    @__parent__init = unnamed_addr constant %parent zeroinitializer, !dbg !27
    @__grandparent__init = unnamed_addr constant %grandparent zeroinitializer, !dbg !30
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_grandparent_type__init = unnamed_addr constant %__vtable_grandparent_type zeroinitializer, !dbg !33
    @__vtable_grandparent = global %__vtable_grandparent_type zeroinitializer, !dbg !39
    @____vtable_parent_type__init = unnamed_addr constant %__vtable_parent_type zeroinitializer, !dbg !41
    @__vtable_parent = global %__vtable_parent_type zeroinitializer, !dbg !48
    @____vtable_child_type__init = unnamed_addr constant %__vtable_child_type zeroinitializer, !dbg !50
    @__vtable_child = global %__vtable_child_type zeroinitializer, !dbg !57

    define void @grandparent(%grandparent* %0) !dbg !63 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !67, metadata !DIExpression()), !dbg !68
      %this = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %this, align 8
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 2
      ret void, !dbg !68
    }

    define void @parent(%parent* %0) !dbg !69 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !72, metadata !DIExpression()), !dbg !73
      %this = alloca %parent*, align 8
      store %parent* %0, %parent** %this, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void, !dbg !73
    }

    define void @child(%child* %0) !dbg !74 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !77, metadata !DIExpression()), !dbg !78
      %this = alloca %child*, align 8
      store %child* %0, %child** %this, align 8
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      ret void, !dbg !78
    }

    define void @main() !dbg !79 {
    entry:
      %arr = alloca [11 x %child], align 8
      call void @llvm.dbg.declare(metadata [11 x %child]* %arr, metadata !82, metadata !DIExpression()), !dbg !84
      %0 = bitcast [11 x %child]* %arr to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([11 x %child]* getelementptr ([11 x %child], [11 x %child]* null, i32 1) to i64), i1 false)
      %tmpVar = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !85
      %__parent = getelementptr inbounds %child, %child* %tmpVar, i32 0, i32 0, !dbg !85
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !85
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 2, !dbg !85
      store i16 10, i16* %a, align 2, !dbg !85
      %tmpVar1 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !86
      %__parent2 = getelementptr inbounds %child, %child* %tmpVar1, i32 0, i32 0, !dbg !86
      %__grandparent3 = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0, !dbg !86
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent3, i32 0, i32 1, !dbg !86
      %tmpVar4 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 0, !dbg !86
      store i16 20, i16* %tmpVar4, align 2, !dbg !86
      %tmpVar5 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !87
      %__parent6 = getelementptr inbounds %child, %child* %tmpVar5, i32 0, i32 0, !dbg !87
      %b = getelementptr inbounds %parent, %parent* %__parent6, i32 0, i32 2, !dbg !87
      store i16 30, i16* %b, align 2, !dbg !87
      %tmpVar7 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !88
      %__parent8 = getelementptr inbounds %child, %child* %tmpVar7, i32 0, i32 0, !dbg !88
      %x = getelementptr inbounds %parent, %parent* %__parent8, i32 0, i32 1, !dbg !88
      %tmpVar9 = getelementptr inbounds [11 x i16], [11 x i16]* %x, i32 0, i32 1, !dbg !88
      store i16 40, i16* %tmpVar9, align 2, !dbg !88
      %tmpVar10 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 2, !dbg !89
      %z = getelementptr inbounds %child, %child* %tmpVar10, i32 0, i32 1, !dbg !89
      %tmpVar11 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 2, !dbg !89
      store i16 50, i16* %tmpVar11, align 2, !dbg !89
      ret void, !dbg !90
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #1

    define void @__init___vtable_grandparent_type(%__vtable_grandparent_type* %0) {
    entry:
      %self = alloca %__vtable_grandparent_type*, align 8
      store %__vtable_grandparent_type* %0, %__vtable_grandparent_type** %self, align 8
      ret void
    }

    define void @__init___vtable_parent_type(%__vtable_parent_type* %0) {
    entry:
      %self = alloca %__vtable_parent_type*, align 8
      store %__vtable_parent_type* %0, %__vtable_parent_type** %self, align 8
      %deref = load %__vtable_parent_type*, %__vtable_parent_type** %self, align 8
      %__vtable_grandparent_type = getelementptr inbounds %__vtable_parent_type, %__vtable_parent_type* %deref, i32 0, i32 0
      call void @__init___vtable_grandparent_type(%__vtable_grandparent_type* %__vtable_grandparent_type)
      ret void
    }

    define void @__init___vtable_child_type(%__vtable_child_type* %0) {
    entry:
      %self = alloca %__vtable_child_type*, align 8
      store %__vtable_child_type* %0, %__vtable_child_type** %self, align 8
      %deref = load %__vtable_child_type*, %__vtable_child_type** %self, align 8
      %__vtable_parent_type = getelementptr inbounds %__vtable_child_type, %__vtable_child_type* %deref, i32 0, i32 0
      call void @__init___vtable_parent_type(%__vtable_parent_type* %__vtable_parent_type)
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
      store i32* bitcast (%__vtable_child_type* @__vtable_child to i32*), i32** %__vtable, align 8
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
      store i32* bitcast (%__vtable_parent_type* @__vtable_parent to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      %deref = load %grandparent*, %grandparent** %self, align 8
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_grandparent_type* @__vtable_grandparent to i32*), i32** %__vtable, align 8
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
      call void @__init___vtable_grandparent_type(%__vtable_grandparent_type* @__vtable_grandparent)
      call void @__init___vtable_parent_type(%__vtable_parent_type* @__vtable_parent)
      call void @__init___vtable_child_type(%__vtable_child_type* @__vtable_child)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn writeonly }

    !llvm.module.flags = !{!59, !60}
    !llvm.dbg.cu = !{!61}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 576, align: 64, flags: DIFlagPublic, elements: !5, identifier: "child")
    !5 = !{!6, !26}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !7, size: 384, align: 64, flags: DIFlagPublic)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 384, align: 64, flags: DIFlagPublic, elements: !8, identifier: "parent")
    !8 = !{!9, !21, !25}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__grandparent", scope: !2, file: !2, baseType: !10, size: 192, align: 64, flags: DIFlagPublic)
    !10 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 192, align: 64, flags: DIFlagPublic, elements: !11, identifier: "grandparent")
    !11 = !{!12, !15, !20}
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !13, size: 64, align: 64, flags: DIFlagPublic)
    !13 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 1)
    !14 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !16, size: 96, align: 16, offset: 64, flags: DIFlagPublic)
    !16 = !DICompositeType(tag: DW_TAG_array_type, baseType: !17, size: 96, align: 16, elements: !18)
    !17 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !18 = !{!19}
    !19 = !DISubrange(count: 6, lowerBound: 0)
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !17, size: 16, align: 16, offset: 160, flags: DIFlagPublic)
    !21 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !22, size: 176, align: 16, offset: 192, flags: DIFlagPublic)
    !22 = !DICompositeType(tag: DW_TAG_array_type, baseType: !17, size: 176, align: 16, elements: !23)
    !23 = !{!24}
    !24 = !DISubrange(count: 11, lowerBound: 0)
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !17, size: 16, align: 16, offset: 368, flags: DIFlagPublic)
    !26 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !22, size: 176, align: 16, offset: 384, flags: DIFlagPublic)
    !27 = !DIGlobalVariableExpression(var: !28, expr: !DIExpression())
    !28 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !29, isLocal: false, isDefinition: true)
    !29 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !7)
    !30 = !DIGlobalVariableExpression(var: !31, expr: !DIExpression())
    !31 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !32, isLocal: false, isDefinition: true)
    !32 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !10)
    !33 = !DIGlobalVariableExpression(var: !34, expr: !DIExpression())
    !34 = distinct !DIGlobalVariable(name: "____vtable_grandparent_type__init", scope: !2, file: !2, type: !35, isLocal: false, isDefinition: true)
    !35 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !36)
    !36 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_grandparent_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !37, identifier: "__vtable_grandparent_type")
    !37 = !{!38}
    !38 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !13, size: 64, align: 64, flags: DIFlagPublic)
    !39 = !DIGlobalVariableExpression(var: !40, expr: !DIExpression())
    !40 = distinct !DIGlobalVariable(name: "__vtable_grandparent", scope: !2, file: !2, type: !36, isLocal: false, isDefinition: true)
    !41 = !DIGlobalVariableExpression(var: !42, expr: !DIExpression())
    !42 = distinct !DIGlobalVariable(name: "____vtable_parent_type__init", scope: !2, file: !2, type: !43, isLocal: false, isDefinition: true)
    !43 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !44)
    !44 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_parent_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !45, identifier: "__vtable_parent_type")
    !45 = !{!46, !47}
    !46 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_grandparent_type", scope: !2, file: !2, baseType: !36, size: 64, align: 64, flags: DIFlagPublic)
    !47 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !13, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !48 = !DIGlobalVariableExpression(var: !49, expr: !DIExpression())
    !49 = distinct !DIGlobalVariable(name: "__vtable_parent", scope: !2, file: !2, type: !44, isLocal: false, isDefinition: true)
    !50 = !DIGlobalVariableExpression(var: !51, expr: !DIExpression())
    !51 = distinct !DIGlobalVariable(name: "____vtable_child_type__init", scope: !2, file: !2, type: !52, isLocal: false, isDefinition: true)
    !52 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !53)
    !53 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_child_type", scope: !2, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !54, identifier: "__vtable_child_type")
    !54 = !{!55, !56}
    !55 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_parent_type", scope: !2, file: !2, baseType: !44, size: 128, align: 64, flags: DIFlagPublic)
    !56 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !13, size: 64, align: 64, offset: 128, flags: DIFlagPublic)
    !57 = !DIGlobalVariableExpression(var: !58, expr: !DIExpression())
    !58 = distinct !DIGlobalVariable(name: "__vtable_child", scope: !2, file: !2, type: !53, isLocal: false, isDefinition: true)
    !59 = !{i32 2, !"Dwarf Version", i32 5}
    !60 = !{i32 2, !"Debug Info Version", i32 3}
    !61 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !62, splitDebugInlining: false)
    !62 = !{!39, !33, !48, !41, !57, !50, !30, !27, !0}
    !63 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !64, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !61, retainedNodes: !66)
    !64 = !DISubroutineType(flags: DIFlagPublic, types: !65)
    !65 = !{null, !10}
    !66 = !{}
    !67 = !DILocalVariable(name: "grandparent", scope: !63, file: !2, line: 7, type: !10)
    !68 = !DILocation(line: 7, column: 8, scope: !63)
    !69 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !70, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !61, retainedNodes: !66)
    !70 = !DISubroutineType(flags: DIFlagPublic, types: !71)
    !71 = !{null, !7}
    !72 = !DILocalVariable(name: "parent", scope: !69, file: !2, line: 14, type: !7)
    !73 = !DILocation(line: 14, column: 8, scope: !69)
    !74 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !75, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !61, retainedNodes: !66)
    !75 = !DISubroutineType(flags: DIFlagPublic, types: !76)
    !76 = !{null, !4}
    !77 = !DILocalVariable(name: "child", scope: !74, file: !2, line: 20, type: !4)
    !78 = !DILocation(line: 20, column: 8, scope: !74)
    !79 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 22, type: !80, scopeLine: 26, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !61, retainedNodes: !66)
    !80 = !DISubroutineType(flags: DIFlagPublic, types: !81)
    !81 = !{null}
    !82 = !DILocalVariable(name: "arr", scope: !79, file: !2, line: 24, type: !83, align: 64)
    !83 = !DICompositeType(tag: DW_TAG_array_type, baseType: !4, size: 6336, align: 64, elements: !23)
    !84 = !DILocation(line: 24, column: 12, scope: !79)
    !85 = !DILocation(line: 26, column: 12, scope: !79)
    !86 = !DILocation(line: 27, column: 12, scope: !79)
    !87 = !DILocation(line: 28, column: 12, scope: !79)
    !88 = !DILocation(line: 29, column: 12, scope: !79)
    !89 = !DILocation(line: 30, column: 12, scope: !79)
    !90 = !DILocation(line: 31, column: 8, scope: !79)
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

    %parent = type { %grandparent, [11 x i16], i16 }
    %grandparent = type { i32*, [6 x i16], i16 }
    %child = type { %parent, [11 x i16] }
    %__vtable_grandparent_type = type { i32* }
    %__vtable_parent_type = type { %__vtable_grandparent_type, i32* }
    %__vtable_child_type = type { %__vtable_parent_type, i32* }

    @__parent__init = unnamed_addr constant %parent zeroinitializer, !dbg !0
    @__grandparent__init = unnamed_addr constant %grandparent zeroinitializer, !dbg !23
    @__child__init = unnamed_addr constant %child zeroinitializer, !dbg !26
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_grandparent_type__init = unnamed_addr constant %__vtable_grandparent_type zeroinitializer, !dbg !33
    @__vtable_grandparent = global %__vtable_grandparent_type zeroinitializer, !dbg !39
    @____vtable_parent_type__init = unnamed_addr constant %__vtable_parent_type zeroinitializer, !dbg !41
    @__vtable_parent = global %__vtable_parent_type zeroinitializer, !dbg !48
    @____vtable_child_type__init = unnamed_addr constant %__vtable_child_type zeroinitializer, !dbg !50
    @__vtable_child = global %__vtable_child_type zeroinitializer, !dbg !57

    define void @grandparent(%grandparent* %0) !dbg !63 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !67, metadata !DIExpression()), !dbg !68
      %this = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %this, align 8
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 2
      ret void, !dbg !68
    }

    define void @parent(%parent* %0) !dbg !69 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !72, metadata !DIExpression()), !dbg !73
      %this = alloca %parent*, align 8
      store %parent* %0, %parent** %this, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void, !dbg !73
    }

    define void @child(%child* %0) !dbg !74 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !77, metadata !DIExpression()), !dbg !78
      %this = alloca %child*, align 8
      store %child* %0, %child** %this, align 8
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !78
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 1, !dbg !78
      %b = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2, !dbg !78
      %load_b = load i16, i16* %b, align 2, !dbg !78
      %1 = sext i16 %load_b to i32, !dbg !78
      %b1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2, !dbg !78
      %load_b2 = load i16, i16* %b1, align 2, !dbg !78
      %2 = sext i16 %load_b2 to i32, !dbg !78
      %tmpVar = mul i32 %2, 2, !dbg !78
      %tmpVar3 = mul i32 1, %tmpVar, !dbg !78
      %tmpVar4 = add i32 %tmpVar3, 0, !dbg !78
      %tmpVar5 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 %tmpVar4, !dbg !78
      %load_tmpVar = load i16, i16* %tmpVar5, align 2, !dbg !78
      %3 = sext i16 %load_tmpVar to i32, !dbg !78
      %tmpVar6 = add i32 %1, %3, !dbg !78
      %__grandparent7 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !78
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent7, i32 0, i32 2, !dbg !78
      %load_a = load i16, i16* %a, align 2, !dbg !78
      %4 = sext i16 %load_a to i32, !dbg !78
      %tmpVar8 = sub i32 %tmpVar6, %4, !dbg !78
      %tmpVar9 = mul i32 1, %tmpVar8, !dbg !78
      %tmpVar10 = add i32 %tmpVar9, 0, !dbg !78
      %tmpVar11 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 %tmpVar10, !dbg !78
      store i16 20, i16* %tmpVar11, align 2, !dbg !78
      ret void, !dbg !79
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init___vtable_grandparent_type(%__vtable_grandparent_type* %0) {
    entry:
      %self = alloca %__vtable_grandparent_type*, align 8
      store %__vtable_grandparent_type* %0, %__vtable_grandparent_type** %self, align 8
      ret void
    }

    define void @__init___vtable_parent_type(%__vtable_parent_type* %0) {
    entry:
      %self = alloca %__vtable_parent_type*, align 8
      store %__vtable_parent_type* %0, %__vtable_parent_type** %self, align 8
      %deref = load %__vtable_parent_type*, %__vtable_parent_type** %self, align 8
      %__vtable_grandparent_type = getelementptr inbounds %__vtable_parent_type, %__vtable_parent_type* %deref, i32 0, i32 0
      call void @__init___vtable_grandparent_type(%__vtable_grandparent_type* %__vtable_grandparent_type)
      ret void
    }

    define void @__init___vtable_child_type(%__vtable_child_type* %0) {
    entry:
      %self = alloca %__vtable_child_type*, align 8
      store %__vtable_child_type* %0, %__vtable_child_type** %self, align 8
      %deref = load %__vtable_child_type*, %__vtable_child_type** %self, align 8
      %__vtable_parent_type = getelementptr inbounds %__vtable_child_type, %__vtable_child_type* %deref, i32 0, i32 0
      call void @__init___vtable_parent_type(%__vtable_parent_type* %__vtable_parent_type)
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
      store i32* bitcast (%__vtable_parent_type* @__vtable_parent to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      %deref = load %grandparent*, %grandparent** %self, align 8
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_grandparent_type* @__vtable_grandparent to i32*), i32** %__vtable, align 8
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
      store i32* bitcast (%__vtable_child_type* @__vtable_child to i32*), i32** %__vtable, align 8
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
      call void @__init___vtable_grandparent_type(%__vtable_grandparent_type* @__vtable_grandparent)
      call void @__init___vtable_parent_type(%__vtable_parent_type* @__vtable_parent)
      call void @__init___vtable_child_type(%__vtable_child_type* @__vtable_child)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!59, !60}
    !llvm.dbg.cu = !{!61}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 384, align: 64, flags: DIFlagPublic, elements: !5, identifier: "parent")
    !5 = !{!6, !18, !22}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__grandparent", scope: !2, file: !2, baseType: !7, size: 192, align: 64, flags: DIFlagPublic)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 192, align: 64, flags: DIFlagPublic, elements: !8, identifier: "grandparent")
    !8 = !{!9, !12, !17}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !10, size: 64, align: 64, flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !11, size: 64, align: 64, dwarfAddressSpace: 1)
    !11 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !13, size: 96, align: 16, offset: 64, flags: DIFlagPublic)
    !13 = !DICompositeType(tag: DW_TAG_array_type, baseType: !14, size: 96, align: 16, elements: !15)
    !14 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !15 = !{!16}
    !16 = !DISubrange(count: 6, lowerBound: 0)
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !14, size: 16, align: 16, offset: 160, flags: DIFlagPublic)
    !18 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !19, size: 176, align: 16, offset: 192, flags: DIFlagPublic)
    !19 = !DICompositeType(tag: DW_TAG_array_type, baseType: !14, size: 176, align: 16, elements: !20)
    !20 = !{!21}
    !21 = !DISubrange(count: 11, lowerBound: 0)
    !22 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !14, size: 16, align: 16, offset: 368, flags: DIFlagPublic)
    !23 = !DIGlobalVariableExpression(var: !24, expr: !DIExpression())
    !24 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !25, isLocal: false, isDefinition: true)
    !25 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !7)
    !26 = !DIGlobalVariableExpression(var: !27, expr: !DIExpression())
    !27 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !28, isLocal: false, isDefinition: true)
    !28 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !29)
    !29 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 576, align: 64, flags: DIFlagPublic, elements: !30, identifier: "child")
    !30 = !{!31, !32}
    !31 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !4, size: 384, align: 64, flags: DIFlagPublic)
    !32 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !19, size: 176, align: 16, offset: 384, flags: DIFlagPublic)
    !33 = !DIGlobalVariableExpression(var: !34, expr: !DIExpression())
    !34 = distinct !DIGlobalVariable(name: "____vtable_grandparent_type__init", scope: !2, file: !2, type: !35, isLocal: false, isDefinition: true)
    !35 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !36)
    !36 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_grandparent_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !37, identifier: "__vtable_grandparent_type")
    !37 = !{!38}
    !38 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !10, size: 64, align: 64, flags: DIFlagPublic)
    !39 = !DIGlobalVariableExpression(var: !40, expr: !DIExpression())
    !40 = distinct !DIGlobalVariable(name: "__vtable_grandparent", scope: !2, file: !2, type: !36, isLocal: false, isDefinition: true)
    !41 = !DIGlobalVariableExpression(var: !42, expr: !DIExpression())
    !42 = distinct !DIGlobalVariable(name: "____vtable_parent_type__init", scope: !2, file: !2, type: !43, isLocal: false, isDefinition: true)
    !43 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !44)
    !44 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_parent_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !45, identifier: "__vtable_parent_type")
    !45 = !{!46, !47}
    !46 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_grandparent_type", scope: !2, file: !2, baseType: !36, size: 64, align: 64, flags: DIFlagPublic)
    !47 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !10, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !48 = !DIGlobalVariableExpression(var: !49, expr: !DIExpression())
    !49 = distinct !DIGlobalVariable(name: "__vtable_parent", scope: !2, file: !2, type: !44, isLocal: false, isDefinition: true)
    !50 = !DIGlobalVariableExpression(var: !51, expr: !DIExpression())
    !51 = distinct !DIGlobalVariable(name: "____vtable_child_type__init", scope: !2, file: !2, type: !52, isLocal: false, isDefinition: true)
    !52 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !53)
    !53 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_child_type", scope: !2, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !54, identifier: "__vtable_child_type")
    !54 = !{!55, !56}
    !55 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_parent_type", scope: !2, file: !2, baseType: !44, size: 128, align: 64, flags: DIFlagPublic)
    !56 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !10, size: 64, align: 64, offset: 128, flags: DIFlagPublic)
    !57 = !DIGlobalVariableExpression(var: !58, expr: !DIExpression())
    !58 = distinct !DIGlobalVariable(name: "__vtable_child", scope: !2, file: !2, type: !53, isLocal: false, isDefinition: true)
    !59 = !{i32 2, !"Dwarf Version", i32 5}
    !60 = !{i32 2, !"Debug Info Version", i32 3}
    !61 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !62, splitDebugInlining: false)
    !62 = !{!39, !33, !48, !41, !57, !50, !23, !0, !26}
    !63 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !64, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !61, retainedNodes: !66)
    !64 = !DISubroutineType(flags: DIFlagPublic, types: !65)
    !65 = !{null, !7}
    !66 = !{}
    !67 = !DILocalVariable(name: "grandparent", scope: !63, file: !2, line: 7, type: !7)
    !68 = !DILocation(line: 7, column: 8, scope: !63)
    !69 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !70, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !61, retainedNodes: !66)
    !70 = !DISubroutineType(flags: DIFlagPublic, types: !71)
    !71 = !{null, !4}
    !72 = !DILocalVariable(name: "parent", scope: !69, file: !2, line: 14, type: !4)
    !73 = !DILocation(line: 14, column: 8, scope: !69)
    !74 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !75, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !61, retainedNodes: !66)
    !75 = !DISubroutineType(flags: DIFlagPublic, types: !76)
    !76 = !{null, !29}
    !77 = !DILocalVariable(name: "child", scope: !74, file: !2, line: 20, type: !29)
    !78 = !DILocation(line: 20, column: 12, scope: !74)
    !79 = !DILocation(line: 21, column: 8, scope: !74)
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

    %foo = type { i32* }
    %bar = type { %foo }
    %__vtable_foo_type = type { i32*, i32* }
    %__vtable_bar_type = type { %__vtable_foo_type, i32* }

    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !0
    @__bar__init = unnamed_addr constant %bar zeroinitializer, !dbg !9
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = unnamed_addr constant %__vtable_foo_type zeroinitializer, !dbg !15
    @__vtable_foo = global %__vtable_foo_type zeroinitializer, !dbg !22
    @____vtable_bar_type__init = unnamed_addr constant %__vtable_bar_type zeroinitializer, !dbg !24
    @__vtable_bar = global %__vtable_bar_type zeroinitializer, !dbg !31

    define void @foo(%foo* %0) !dbg !37 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !41, metadata !DIExpression()), !dbg !42
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void, !dbg !42
    }

    define void @foo__baz(%foo* %0) !dbg !43 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !44, metadata !DIExpression()), !dbg !45
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void, !dbg !45
    }

    define void @bar(%bar* %0) !dbg !46 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !49, metadata !DIExpression()), !dbg !50
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void, !dbg !50
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init___vtable_bar_type(%__vtable_bar_type* %0) {
    entry:
      %self = alloca %__vtable_bar_type*, align 8
      store %__vtable_bar_type* %0, %__vtable_bar_type** %self, align 8
      %deref = load %__vtable_bar_type*, %__vtable_bar_type** %self, align 8
      %__vtable_foo_type = getelementptr inbounds %__vtable_bar_type, %__vtable_bar_type* %deref, i32 0, i32 0
      call void @__init___vtable_foo_type(%__vtable_foo_type* %__vtable_foo_type)
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
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
      store i32* bitcast (%__vtable_bar_type* @__vtable_bar to i32*), i32** %__vtable, align 8
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
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      call void @__init___vtable_bar_type(%__vtable_bar_type* @__vtable_bar)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!33, !34}
    !llvm.dbg.cu = !{!35}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 64, align: 64, flags: DIFlagPublic, elements: !5, identifier: "foo")
    !5 = !{!6}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !8, size: 64, align: 64, dwarfAddressSpace: 1)
    !8 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !9 = !DIGlobalVariableExpression(var: !10, expr: !DIExpression())
    !10 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 7, type: !11, isLocal: false, isDefinition: true)
    !11 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !12)
    !12 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 7, size: 64, align: 64, flags: DIFlagPublic, elements: !13, identifier: "bar")
    !13 = !{!14}
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !4, size: 64, align: 64, flags: DIFlagPublic)
    !15 = !DIGlobalVariableExpression(var: !16, expr: !DIExpression())
    !16 = distinct !DIGlobalVariable(name: "____vtable_foo_type__init", scope: !2, file: !2, type: !17, isLocal: false, isDefinition: true)
    !17 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !18)
    !18 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !19, identifier: "__vtable_foo_type")
    !19 = !{!20, !21}
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !21 = !DIDerivedType(tag: DW_TAG_member, name: "foo.baz", scope: !2, file: !2, baseType: !7, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !22 = !DIGlobalVariableExpression(var: !23, expr: !DIExpression())
    !23 = distinct !DIGlobalVariable(name: "__vtable_foo", scope: !2, file: !2, type: !18, isLocal: false, isDefinition: true)
    !24 = !DIGlobalVariableExpression(var: !25, expr: !DIExpression())
    !25 = distinct !DIGlobalVariable(name: "____vtable_bar_type__init", scope: !2, file: !2, type: !26, isLocal: false, isDefinition: true)
    !26 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !27)
    !27 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_bar_type", scope: !2, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !28, identifier: "__vtable_bar_type")
    !28 = !{!29, !30}
    !29 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_foo_type", scope: !2, file: !2, baseType: !18, size: 128, align: 64, flags: DIFlagPublic)
    !30 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !7, size: 64, align: 64, offset: 128, flags: DIFlagPublic)
    !31 = !DIGlobalVariableExpression(var: !32, expr: !DIExpression())
    !32 = distinct !DIGlobalVariable(name: "__vtable_bar", scope: !2, file: !2, type: !27, isLocal: false, isDefinition: true)
    !33 = !{i32 2, !"Dwarf Version", i32 5}
    !34 = !{i32 2, !"Debug Info Version", i32 3}
    !35 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !36, splitDebugInlining: false)
    !36 = !{!22, !15, !31, !24, !0, !9}
    !37 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !38, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !40)
    !38 = !DISubroutineType(flags: DIFlagPublic, types: !39)
    !39 = !{null, !4}
    !40 = !{}
    !41 = !DILocalVariable(name: "foo", scope: !37, file: !2, line: 5, type: !4)
    !42 = !DILocation(line: 5, column: 8, scope: !37)
    !43 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !37, file: !2, line: 3, type: !38, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !40)
    !44 = !DILocalVariable(name: "foo", scope: !43, file: !2, line: 4, type: !4)
    !45 = !DILocation(line: 4, column: 8, scope: !43)
    !46 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 7, type: !47, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !40)
    !47 = !DISubroutineType(flags: DIFlagPublic, types: !48)
    !48 = !{null, !12}
    !49 = !DILocalVariable(name: "bar", scope: !46, file: !2, line: 8, type: !12)
    !50 = !DILocation(line: 8, column: 8, scope: !46)
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

    %grandchild = type { %child, i32 }
    %child = type { %parent, i32 }
    %parent = type { i32*, i32 }
    %__vtable_parent_type = type { i32* }
    %__vtable_child_type = type { %__vtable_parent_type, i32* }
    %__vtable_grandchild_type = type { %__vtable_child_type, i32* }

    @__grandchild__init = unnamed_addr constant %grandchild zeroinitializer, !dbg !0
    @__child__init = unnamed_addr constant %child zeroinitializer, !dbg !19
    @__parent__init = unnamed_addr constant %parent zeroinitializer, !dbg !22
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_parent_type__init = unnamed_addr constant %__vtable_parent_type zeroinitializer, !dbg !25
    @__vtable_parent = global %__vtable_parent_type zeroinitializer, !dbg !31
    @____vtable_child_type__init = unnamed_addr constant %__vtable_child_type zeroinitializer, !dbg !33
    @__vtable_child = global %__vtable_child_type zeroinitializer, !dbg !40
    @____vtable_grandchild_type__init = unnamed_addr constant %__vtable_grandchild_type zeroinitializer, !dbg !42
    @__vtable_grandchild = global %__vtable_grandchild_type zeroinitializer, !dbg !49

    define void @parent(%parent* %0) !dbg !55 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !59, metadata !DIExpression()), !dbg !60
      %this = alloca %parent*, align 8
      store %parent* %0, %parent** %this, align 8
      %__vtable = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %a = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      ret void, !dbg !60
    }

    define void @child(%child* %0) !dbg !61 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !64, metadata !DIExpression()), !dbg !65
      %this = alloca %child*, align 8
      store %child* %0, %child** %this, align 8
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %b = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      ret void, !dbg !65
    }

    define void @grandchild(%grandchild* %0) !dbg !66 {
    entry:
      call void @llvm.dbg.declare(metadata %grandchild* %0, metadata !69, metadata !DIExpression()), !dbg !70
      %this = alloca %grandchild*, align 8
      store %grandchild* %0, %grandchild** %this, align 8
      %__child = getelementptr inbounds %grandchild, %grandchild* %0, i32 0, i32 0
      %c = getelementptr inbounds %grandchild, %grandchild* %0, i32 0, i32 1
      ret void, !dbg !70
    }

    define i32 @main() !dbg !71 {
    entry:
      %main = alloca i32, align 4
      %array_of_parent = alloca [3 x %parent], align 8
      %array_of_child = alloca [3 x %child], align 8
      %array_of_grandchild = alloca [3 x %grandchild], align 8
      %parent1 = alloca %parent, align 8
      %child1 = alloca %child, align 8
      %grandchild1 = alloca %grandchild, align 8
      call void @llvm.dbg.declare(metadata [3 x %parent]* %array_of_parent, metadata !74, metadata !DIExpression()), !dbg !78
      %0 = bitcast [3 x %parent]* %array_of_parent to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([3 x %parent]* getelementptr ([3 x %parent], [3 x %parent]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata [3 x %child]* %array_of_child, metadata !79, metadata !DIExpression()), !dbg !81
      %1 = bitcast [3 x %child]* %array_of_child to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %1, i8 0, i64 ptrtoint ([3 x %child]* getelementptr ([3 x %child], [3 x %child]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata [3 x %grandchild]* %array_of_grandchild, metadata !82, metadata !DIExpression()), !dbg !84
      %2 = bitcast [3 x %grandchild]* %array_of_grandchild to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %2, i8 0, i64 ptrtoint ([3 x %grandchild]* getelementptr ([3 x %grandchild], [3 x %grandchild]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %parent* %parent1, metadata !85, metadata !DIExpression()), !dbg !86
      %3 = bitcast %parent* %parent1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %3, i8* align 1 bitcast (%parent* @__parent__init to i8*), i64 ptrtoint (%parent* getelementptr (%parent, %parent* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %child* %child1, metadata !87, metadata !DIExpression()), !dbg !88
      %4 = bitcast %child* %child1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %4, i8* align 1 bitcast (%child* @__child__init to i8*), i64 ptrtoint (%child* getelementptr (%child, %child* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %grandchild* %grandchild1, metadata !89, metadata !DIExpression()), !dbg !90
      %5 = bitcast %grandchild* %grandchild1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %5, i8* align 1 bitcast (%grandchild* @__grandchild__init to i8*), i64 ptrtoint (%grandchild* getelementptr (%grandchild, %grandchild* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata i32* %main, metadata !91, metadata !DIExpression()), !dbg !92
      store i32 0, i32* %main, align 4
      call void @__init_parent(%parent* %parent1), !dbg !93
      call void @__init_child(%child* %child1), !dbg !93
      call void @__init_grandchild(%grandchild* %grandchild1), !dbg !93
      call void @__user_init_parent(%parent* %parent1), !dbg !93
      call void @__user_init_child(%child* %child1), !dbg !93
      call void @__user_init_grandchild(%grandchild* %grandchild1), !dbg !93
      %a = getelementptr inbounds %parent, %parent* %parent1, i32 0, i32 1, !dbg !94
      store i32 1, i32* %a, align 4, !dbg !94
      %__parent = getelementptr inbounds %child, %child* %child1, i32 0, i32 0, !dbg !95
      %a1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 1, !dbg !95
      store i32 2, i32* %a1, align 4, !dbg !95
      %b = getelementptr inbounds %child, %child* %child1, i32 0, i32 1, !dbg !96
      store i32 3, i32* %b, align 4, !dbg !96
      %__child = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 0, !dbg !97
      %__parent2 = getelementptr inbounds %child, %child* %__child, i32 0, i32 0, !dbg !97
      %a3 = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 1, !dbg !97
      store i32 4, i32* %a3, align 4, !dbg !97
      %__child4 = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 0, !dbg !98
      %b5 = getelementptr inbounds %child, %child* %__child4, i32 0, i32 1, !dbg !98
      store i32 5, i32* %b5, align 4, !dbg !98
      %c = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 1, !dbg !99
      store i32 6, i32* %c, align 4, !dbg !99
      %tmpVar = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 0, !dbg !100
      %a6 = getelementptr inbounds %parent, %parent* %tmpVar, i32 0, i32 1, !dbg !100
      store i32 7, i32* %a6, align 4, !dbg !100
      %tmpVar7 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 0, !dbg !101
      %__parent8 = getelementptr inbounds %child, %child* %tmpVar7, i32 0, i32 0, !dbg !101
      %a9 = getelementptr inbounds %parent, %parent* %__parent8, i32 0, i32 1, !dbg !101
      store i32 8, i32* %a9, align 4, !dbg !101
      %tmpVar10 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 0, !dbg !102
      %b11 = getelementptr inbounds %child, %child* %tmpVar10, i32 0, i32 1, !dbg !102
      store i32 9, i32* %b11, align 4, !dbg !102
      %tmpVar12 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !103
      %__child13 = getelementptr inbounds %grandchild, %grandchild* %tmpVar12, i32 0, i32 0, !dbg !103
      %__parent14 = getelementptr inbounds %child, %child* %__child13, i32 0, i32 0, !dbg !103
      %a15 = getelementptr inbounds %parent, %parent* %__parent14, i32 0, i32 1, !dbg !103
      store i32 10, i32* %a15, align 4, !dbg !103
      %tmpVar16 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !104
      %__child17 = getelementptr inbounds %grandchild, %grandchild* %tmpVar16, i32 0, i32 0, !dbg !104
      %b18 = getelementptr inbounds %child, %child* %__child17, i32 0, i32 1, !dbg !104
      store i32 11, i32* %b18, align 4, !dbg !104
      %tmpVar19 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !105
      %c20 = getelementptr inbounds %grandchild, %grandchild* %tmpVar19, i32 0, i32 1, !dbg !105
      store i32 12, i32* %c20, align 4, !dbg !105
      %tmpVar21 = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 1, !dbg !106
      %a22 = getelementptr inbounds %parent, %parent* %tmpVar21, i32 0, i32 1, !dbg !106
      store i32 13, i32* %a22, align 4, !dbg !106
      %tmpVar23 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 1, !dbg !107
      %__parent24 = getelementptr inbounds %child, %child* %tmpVar23, i32 0, i32 0, !dbg !107
      %a25 = getelementptr inbounds %parent, %parent* %__parent24, i32 0, i32 1, !dbg !107
      store i32 14, i32* %a25, align 4, !dbg !107
      %tmpVar26 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 1, !dbg !108
      %b27 = getelementptr inbounds %child, %child* %tmpVar26, i32 0, i32 1, !dbg !108
      store i32 15, i32* %b27, align 4, !dbg !108
      %tmpVar28 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !109
      %__child29 = getelementptr inbounds %grandchild, %grandchild* %tmpVar28, i32 0, i32 0, !dbg !109
      %__parent30 = getelementptr inbounds %child, %child* %__child29, i32 0, i32 0, !dbg !109
      %a31 = getelementptr inbounds %parent, %parent* %__parent30, i32 0, i32 1, !dbg !109
      store i32 16, i32* %a31, align 4, !dbg !109
      %tmpVar32 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !110
      %__child33 = getelementptr inbounds %grandchild, %grandchild* %tmpVar32, i32 0, i32 0, !dbg !110
      %b34 = getelementptr inbounds %child, %child* %__child33, i32 0, i32 1, !dbg !110
      store i32 17, i32* %b34, align 4, !dbg !110
      %tmpVar35 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !111
      %c36 = getelementptr inbounds %grandchild, %grandchild* %tmpVar35, i32 0, i32 1, !dbg !111
      store i32 18, i32* %c36, align 4, !dbg !111
      %tmpVar37 = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 2, !dbg !112
      %a38 = getelementptr inbounds %parent, %parent* %tmpVar37, i32 0, i32 1, !dbg !112
      store i32 19, i32* %a38, align 4, !dbg !112
      %tmpVar39 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 2, !dbg !113
      %__parent40 = getelementptr inbounds %child, %child* %tmpVar39, i32 0, i32 0, !dbg !113
      %a41 = getelementptr inbounds %parent, %parent* %__parent40, i32 0, i32 1, !dbg !113
      store i32 20, i32* %a41, align 4, !dbg !113
      %tmpVar42 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 2, !dbg !114
      %b43 = getelementptr inbounds %child, %child* %tmpVar42, i32 0, i32 1, !dbg !114
      store i32 21, i32* %b43, align 4, !dbg !114
      %tmpVar44 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !115
      %__child45 = getelementptr inbounds %grandchild, %grandchild* %tmpVar44, i32 0, i32 0, !dbg !115
      %__parent46 = getelementptr inbounds %child, %child* %__child45, i32 0, i32 0, !dbg !115
      %a47 = getelementptr inbounds %parent, %parent* %__parent46, i32 0, i32 1, !dbg !115
      store i32 22, i32* %a47, align 4, !dbg !115
      %tmpVar48 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !116
      %__child49 = getelementptr inbounds %grandchild, %grandchild* %tmpVar48, i32 0, i32 0, !dbg !116
      %b50 = getelementptr inbounds %child, %child* %__child49, i32 0, i32 1, !dbg !116
      store i32 23, i32* %b50, align 4, !dbg !116
      %tmpVar51 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !117
      %c52 = getelementptr inbounds %grandchild, %grandchild* %tmpVar51, i32 0, i32 1, !dbg !117
      store i32 24, i32* %c52, align 4, !dbg !117
      %main_ret = load i32, i32* %main, align 4, !dbg !118
      ret i32 %main_ret, !dbg !118
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #1

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #2

    define void @__init___vtable_parent_type(%__vtable_parent_type* %0) {
    entry:
      %self = alloca %__vtable_parent_type*, align 8
      store %__vtable_parent_type* %0, %__vtable_parent_type** %self, align 8
      ret void
    }

    define void @__init___vtable_child_type(%__vtable_child_type* %0) {
    entry:
      %self = alloca %__vtable_child_type*, align 8
      store %__vtable_child_type* %0, %__vtable_child_type** %self, align 8
      %deref = load %__vtable_child_type*, %__vtable_child_type** %self, align 8
      %__vtable_parent_type = getelementptr inbounds %__vtable_child_type, %__vtable_child_type* %deref, i32 0, i32 0
      call void @__init___vtable_parent_type(%__vtable_parent_type* %__vtable_parent_type)
      ret void
    }

    define void @__init___vtable_grandchild_type(%__vtable_grandchild_type* %0) {
    entry:
      %self = alloca %__vtable_grandchild_type*, align 8
      store %__vtable_grandchild_type* %0, %__vtable_grandchild_type** %self, align 8
      %deref = load %__vtable_grandchild_type*, %__vtable_grandchild_type** %self, align 8
      %__vtable_child_type = getelementptr inbounds %__vtable_grandchild_type, %__vtable_grandchild_type* %deref, i32 0, i32 0
      call void @__init___vtable_child_type(%__vtable_child_type* %__vtable_child_type)
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
      store i32* bitcast (%__vtable_grandchild_type* @__vtable_grandchild to i32*), i32** %__vtable, align 8
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
      store i32* bitcast (%__vtable_child_type* @__vtable_child to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__vtable = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_parent_type* @__vtable_parent to i32*), i32** %__vtable, align 8
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
      call void @__init___vtable_parent_type(%__vtable_parent_type* @__vtable_parent)
      call void @__init___vtable_child_type(%__vtable_child_type* @__vtable_child)
      call void @__init___vtable_grandchild_type(%__vtable_grandchild_type* @__vtable_grandchild)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn writeonly }
    attributes #2 = { argmemonly nofree nounwind willreturn }

    !llvm.module.flags = !{!51, !52}
    !llvm.dbg.cu = !{!53}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__grandchild__init", scope: !2, file: !2, line: 14, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandchild", scope: !2, file: !2, line: 14, size: 256, align: 64, flags: DIFlagPublic, elements: !5, identifier: "grandchild")
    !5 = !{!6, !18}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__child", scope: !2, file: !2, baseType: !7, size: 192, align: 64, flags: DIFlagPublic)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 8, size: 192, align: 64, flags: DIFlagPublic, elements: !8, identifier: "child")
    !8 = !{!9, !17}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !10, size: 128, align: 64, flags: DIFlagPublic)
    !10 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 2, size: 128, align: 64, flags: DIFlagPublic, elements: !11, identifier: "parent")
    !11 = !{!12, !15}
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !13, size: 64, align: 64, flags: DIFlagPublic)
    !13 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 1)
    !14 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 4, baseType: !16, size: 32, align: 32, offset: 64, flags: DIFlagPublic)
    !16 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 10, baseType: !16, size: 32, align: 32, offset: 128, flags: DIFlagPublic)
    !18 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 16, baseType: !16, size: 32, align: 32, offset: 192, flags: DIFlagPublic)
    !19 = !DIGlobalVariableExpression(var: !20, expr: !DIExpression())
    !20 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 8, type: !21, isLocal: false, isDefinition: true)
    !21 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !7)
    !22 = !DIGlobalVariableExpression(var: !23, expr: !DIExpression())
    !23 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 2, type: !24, isLocal: false, isDefinition: true)
    !24 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !10)
    !25 = !DIGlobalVariableExpression(var: !26, expr: !DIExpression())
    !26 = distinct !DIGlobalVariable(name: "____vtable_parent_type__init", scope: !2, file: !2, type: !27, isLocal: false, isDefinition: true)
    !27 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !28)
    !28 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_parent_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !29, identifier: "__vtable_parent_type")
    !29 = !{!30}
    !30 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !13, size: 64, align: 64, flags: DIFlagPublic)
    !31 = !DIGlobalVariableExpression(var: !32, expr: !DIExpression())
    !32 = distinct !DIGlobalVariable(name: "__vtable_parent", scope: !2, file: !2, type: !28, isLocal: false, isDefinition: true)
    !33 = !DIGlobalVariableExpression(var: !34, expr: !DIExpression())
    !34 = distinct !DIGlobalVariable(name: "____vtable_child_type__init", scope: !2, file: !2, type: !35, isLocal: false, isDefinition: true)
    !35 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !36)
    !36 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_child_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !37, identifier: "__vtable_child_type")
    !37 = !{!38, !39}
    !38 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_parent_type", scope: !2, file: !2, baseType: !28, size: 64, align: 64, flags: DIFlagPublic)
    !39 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !13, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !40 = !DIGlobalVariableExpression(var: !41, expr: !DIExpression())
    !41 = distinct !DIGlobalVariable(name: "__vtable_child", scope: !2, file: !2, type: !36, isLocal: false, isDefinition: true)
    !42 = !DIGlobalVariableExpression(var: !43, expr: !DIExpression())
    !43 = distinct !DIGlobalVariable(name: "____vtable_grandchild_type__init", scope: !2, file: !2, type: !44, isLocal: false, isDefinition: true)
    !44 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !45)
    !45 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_grandchild_type", scope: !2, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !46, identifier: "__vtable_grandchild_type")
    !46 = !{!47, !48}
    !47 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_child_type", scope: !2, file: !2, baseType: !36, size: 128, align: 64, flags: DIFlagPublic)
    !48 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !13, size: 64, align: 64, offset: 128, flags: DIFlagPublic)
    !49 = !DIGlobalVariableExpression(var: !50, expr: !DIExpression())
    !50 = distinct !DIGlobalVariable(name: "__vtable_grandchild", scope: !2, file: !2, type: !45, isLocal: false, isDefinition: true)
    !51 = !{i32 2, !"Dwarf Version", i32 5}
    !52 = !{i32 2, !"Debug Info Version", i32 3}
    !53 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !54, splitDebugInlining: false)
    !54 = !{!31, !25, !40, !33, !49, !42, !22, !19, !0}
    !55 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 2, type: !56, scopeLine: 6, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !53, retainedNodes: !58)
    !56 = !DISubroutineType(flags: DIFlagPublic, types: !57)
    !57 = !{null, !10}
    !58 = !{}
    !59 = !DILocalVariable(name: "parent", scope: !55, file: !2, line: 6, type: !10)
    !60 = !DILocation(line: 6, scope: !55)
    !61 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 8, type: !62, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !53, retainedNodes: !58)
    !62 = !DISubroutineType(flags: DIFlagPublic, types: !63)
    !63 = !{null, !7}
    !64 = !DILocalVariable(name: "child", scope: !61, file: !2, line: 12, type: !7)
    !65 = !DILocation(line: 12, scope: !61)
    !66 = distinct !DISubprogram(name: "grandchild", linkageName: "grandchild", scope: !2, file: !2, line: 14, type: !67, scopeLine: 18, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !53, retainedNodes: !58)
    !67 = !DISubroutineType(flags: DIFlagPublic, types: !68)
    !68 = !{null, !4}
    !69 = !DILocalVariable(name: "grandchild", scope: !66, file: !2, line: 18, type: !4)
    !70 = !DILocation(line: 18, scope: !66)
    !71 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 20, type: !72, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !53, retainedNodes: !58)
    !72 = !DISubroutineType(flags: DIFlagPublic, types: !73)
    !73 = !{null}
    !74 = !DILocalVariable(name: "array_of_parent", scope: !71, file: !2, line: 22, type: !75, align: 64)
    !75 = !DICompositeType(tag: DW_TAG_array_type, baseType: !10, size: 384, align: 64, elements: !76)
    !76 = !{!77}
    !77 = !DISubrange(count: 3, lowerBound: 0)
    !78 = !DILocation(line: 22, column: 4, scope: !71)
    !79 = !DILocalVariable(name: "array_of_child", scope: !71, file: !2, line: 23, type: !80, align: 64)
    !80 = !DICompositeType(tag: DW_TAG_array_type, baseType: !7, size: 576, align: 64, elements: !76)
    !81 = !DILocation(line: 23, column: 4, scope: !71)
    !82 = !DILocalVariable(name: "array_of_grandchild", scope: !71, file: !2, line: 24, type: !83, align: 64)
    !83 = !DICompositeType(tag: DW_TAG_array_type, baseType: !4, size: 768, align: 64, elements: !76)
    !84 = !DILocation(line: 24, column: 4, scope: !71)
    !85 = !DILocalVariable(name: "parent1", scope: !71, file: !2, line: 25, type: !10, align: 64)
    !86 = !DILocation(line: 25, column: 4, scope: !71)
    !87 = !DILocalVariable(name: "child1", scope: !71, file: !2, line: 26, type: !7, align: 64)
    !88 = !DILocation(line: 26, column: 4, scope: !71)
    !89 = !DILocalVariable(name: "grandchild1", scope: !71, file: !2, line: 27, type: !4, align: 64)
    !90 = !DILocation(line: 27, column: 4, scope: !71)
    !91 = !DILocalVariable(name: "main", scope: !71, file: !2, line: 20, type: !16, align: 32)
    !92 = !DILocation(line: 20, column: 9, scope: !71)
    !93 = !DILocation(line: 0, scope: !71)
    !94 = !DILocation(line: 30, column: 4, scope: !71)
    !95 = !DILocation(line: 31, column: 4, scope: !71)
    !96 = !DILocation(line: 32, column: 4, scope: !71)
    !97 = !DILocation(line: 33, column: 4, scope: !71)
    !98 = !DILocation(line: 34, column: 4, scope: !71)
    !99 = !DILocation(line: 35, column: 4, scope: !71)
    !100 = !DILocation(line: 37, column: 4, scope: !71)
    !101 = !DILocation(line: 38, column: 4, scope: !71)
    !102 = !DILocation(line: 39, column: 4, scope: !71)
    !103 = !DILocation(line: 40, column: 4, scope: !71)
    !104 = !DILocation(line: 41, column: 4, scope: !71)
    !105 = !DILocation(line: 42, column: 4, scope: !71)
    !106 = !DILocation(line: 43, column: 4, scope: !71)
    !107 = !DILocation(line: 44, column: 4, scope: !71)
    !108 = !DILocation(line: 45, column: 4, scope: !71)
    !109 = !DILocation(line: 46, column: 4, scope: !71)
    !110 = !DILocation(line: 47, column: 4, scope: !71)
    !111 = !DILocation(line: 48, column: 4, scope: !71)
    !112 = !DILocation(line: 49, column: 4, scope: !71)
    !113 = !DILocation(line: 50, column: 4, scope: !71)
    !114 = !DILocation(line: 51, column: 4, scope: !71)
    !115 = !DILocation(line: 52, column: 4, scope: !71)
    !116 = !DILocation(line: 53, column: 4, scope: !71)
    !117 = !DILocation(line: 54, column: 4, scope: !71)
    !118 = !DILocation(line: 56, scope: !71)
    "#);
}
