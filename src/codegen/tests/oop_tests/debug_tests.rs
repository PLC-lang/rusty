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
    %__vtable_foo_type = type { i32* }
    %__vtable_bar_type = type { i32*, %__vtable_foo_type }

    @__foo__init = constant %foo zeroinitializer, !dbg !0
    @__bar__init = constant %bar zeroinitializer, !dbg !16
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer, !dbg !21
    @__vtable_foo = global %__vtable_foo_type zeroinitializer, !dbg !28
    @____vtable_bar_type__init = constant %__vtable_bar_type zeroinitializer, !dbg !30
    @__vtable_bar = global %__vtable_bar_type zeroinitializer, !dbg !35

    define void @foo(%foo* %0) !dbg !41 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !45, metadata !DIExpression()), !dbg !46
      %a = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %b = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %c = getelementptr inbounds %foo, %foo* %0, i32 0, i32 2
      ret void, !dbg !46
    }

    define void @bar(%bar* %0) !dbg !47 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !50, metadata !DIExpression()), !dbg !51
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void, !dbg !51
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
      %__vtable_foo_type = getelementptr inbounds %__vtable_bar_type, %__vtable_bar_type* %deref, i32 0, i32 1
      call void @__init___vtable_foo_type(%__vtable_foo_type* %__vtable_foo_type)
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

    !llvm.module.flags = !{!37, !38}
    !llvm.dbg.cu = !{!39}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 7808, align: 64, flags: DIFlagPublic, elements: !4, identifier: "foo")
    !4 = !{!5, !7, !12}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 4, baseType: !6, size: 16, align: 16, flags: DIFlagPublic)
    !6 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 5, baseType: !8, size: 648, align: 8, offset: 16, flags: DIFlagPublic)
    !8 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 648, align: 8, elements: !10)
    !9 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !10 = !{!11}
    !11 = !DISubrange(count: 81, lowerBound: 0)
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 6, baseType: !13, size: 7128, align: 8, offset: 664, flags: DIFlagPublic)
    !13 = !DICompositeType(tag: DW_TAG_array_type, baseType: !8, size: 7128, align: 8, elements: !14)
    !14 = !{!15}
    !15 = !DISubrange(count: 11, lowerBound: 0)
    !16 = !DIGlobalVariableExpression(var: !17, expr: !DIExpression())
    !17 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 10, type: !18, isLocal: false, isDefinition: true)
    !18 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 10, size: 7808, align: 64, flags: DIFlagPublic, elements: !19, identifier: "bar")
    !19 = !{!20}
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !3, size: 7792, align: 64, flags: DIFlagPublic)
    !21 = !DIGlobalVariableExpression(var: !22, expr: !DIExpression())
    !22 = distinct !DIGlobalVariable(name: "____vtable_foo_type__init", scope: !2, file: !2, type: !23, isLocal: false, isDefinition: true)
    !23 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !24, identifier: "__vtable_foo_type")
    !24 = !{!25}
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !26, size: 64, align: 64, flags: DIFlagPublic)
    !26 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !27, size: 64, align: 64, dwarfAddressSpace: 1)
    !27 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !28 = !DIGlobalVariableExpression(var: !29, expr: !DIExpression())
    !29 = distinct !DIGlobalVariable(name: "__vtable_foo", scope: !2, file: !2, type: !23, isLocal: false, isDefinition: true)
    !30 = !DIGlobalVariableExpression(var: !31, expr: !DIExpression())
    !31 = distinct !DIGlobalVariable(name: "____vtable_bar_type__init", scope: !2, file: !2, type: !32, isLocal: false, isDefinition: true)
    !32 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_bar_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !33, identifier: "__vtable_bar_type")
    !33 = !{!25, !34}
    !34 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_foo_type", scope: !2, file: !2, baseType: !23, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !35 = !DIGlobalVariableExpression(var: !36, expr: !DIExpression())
    !36 = distinct !DIGlobalVariable(name: "__vtable_bar", scope: !2, file: !2, type: !32, isLocal: false, isDefinition: true)
    !37 = !{i32 2, !"Dwarf Version", i32 5}
    !38 = !{i32 2, !"Debug Info Version", i32 3}
    !39 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !40, splitDebugInlining: false)
    !40 = !{!28, !21, !35, !30, !0, !16}
    !41 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !42, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !39, retainedNodes: !44)
    !42 = !DISubroutineType(flags: DIFlagPublic, types: !43)
    !43 = !{null, !3}
    !44 = !{}
    !45 = !DILocalVariable(name: "foo", scope: !41, file: !2, line: 8, type: !3)
    !46 = !DILocation(line: 8, column: 8, scope: !41)
    !47 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 10, type: !48, scopeLine: 11, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !39, retainedNodes: !44)
    !48 = !DISubroutineType(flags: DIFlagPublic, types: !49)
    !49 = !{null, !18}
    !50 = !DILocalVariable(name: "bar", scope: !47, file: !2, line: 11, type: !18)
    !51 = !DILocation(line: 11, column: 8, scope: !47)
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
    %__vtable_fb_type = type { i32* }
    %__vtable_fb2_type = type { i32*, %__vtable_fb_type }
    %__vtable_foo_type = type { i32* }

    @__fb2__init = constant %fb2 zeroinitializer, !dbg !0
    @__fb__init = constant %fb zeroinitializer, !dbg !11
    @__foo__init = constant %foo zeroinitializer, !dbg !13
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_fb_type__init = constant %__vtable_fb_type zeroinitializer, !dbg !18
    @__vtable_fb = global %__vtable_fb_type zeroinitializer, !dbg !25
    @____vtable_fb2_type__init = constant %__vtable_fb2_type zeroinitializer, !dbg !27
    @__vtable_fb2 = global %__vtable_fb2_type zeroinitializer, !dbg !32
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer, !dbg !34
    @__vtable_foo = global %__vtable_foo_type zeroinitializer, !dbg !37

    define void @fb(%fb* %0) !dbg !43 {
    entry:
      call void @llvm.dbg.declare(metadata %fb* %0, metadata !47, metadata !DIExpression()), !dbg !48
      %x = getelementptr inbounds %fb, %fb* %0, i32 0, i32 0
      %y = getelementptr inbounds %fb, %fb* %0, i32 0, i32 1
      ret void, !dbg !48
    }

    define void @fb2(%fb2* %0) !dbg !49 {
    entry:
      call void @llvm.dbg.declare(metadata %fb2* %0, metadata !52, metadata !DIExpression()), !dbg !53
      %__fb = getelementptr inbounds %fb2, %fb2* %0, i32 0, i32 0
      ret void, !dbg !53
    }

    define void @foo(%foo* %0) !dbg !54 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !57, metadata !DIExpression()), !dbg !58
      %myFb = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %__fb = getelementptr inbounds %fb2, %fb2* %myFb, i32 0, i32 0, !dbg !58
      %x = getelementptr inbounds %fb, %fb* %__fb, i32 0, i32 0, !dbg !58
      store i16 1, i16* %x, align 2, !dbg !58
      ret void, !dbg !59
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
      %__vtable_fb_type = getelementptr inbounds %__vtable_fb2_type, %__vtable_fb2_type* %deref, i32 0, i32 1
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
      call void @__init___vtable_fb_type(%__vtable_fb_type* @__vtable_fb)
      call void @__init___vtable_fb2_type(%__vtable_fb2_type* @__vtable_fb2)
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!39, !40}
    !llvm.dbg.cu = !{!41}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__fb2__init", scope: !2, file: !2, line: 9, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb2", scope: !2, file: !2, line: 9, size: 64, align: 64, flags: DIFlagPublic, elements: !4, identifier: "fb2")
    !4 = !{!5}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__fb", scope: !2, file: !2, baseType: !6, size: 32, align: 64, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb", scope: !2, file: !2, line: 2, size: 64, align: 64, flags: DIFlagPublic, elements: !7, identifier: "fb")
    !7 = !{!8, !10}
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 4, baseType: !9, size: 16, align: 16, flags: DIFlagPublic)
    !9 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 5, baseType: !9, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
    !11 = !DIGlobalVariableExpression(var: !12, expr: !DIExpression())
    !12 = distinct !DIGlobalVariable(name: "__fb__init", scope: !2, file: !2, line: 2, type: !6, isLocal: false, isDefinition: true)
    !13 = !DIGlobalVariableExpression(var: !14, expr: !DIExpression())
    !14 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 12, type: !15, isLocal: false, isDefinition: true)
    !15 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 12, size: 64, align: 64, flags: DIFlagPublic, elements: !16, identifier: "foo")
    !16 = !{!17}
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "myFb", scope: !2, file: !2, line: 14, baseType: !3, size: 32, align: 64, flags: DIFlagPublic)
    !18 = !DIGlobalVariableExpression(var: !19, expr: !DIExpression())
    !19 = distinct !DIGlobalVariable(name: "____vtable_fb_type__init", scope: !2, file: !2, type: !20, isLocal: false, isDefinition: true)
    !20 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_fb_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !21, identifier: "__vtable_fb_type")
    !21 = !{!22}
    !22 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !23, size: 64, align: 64, flags: DIFlagPublic)
    !23 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !24, size: 64, align: 64, dwarfAddressSpace: 1)
    !24 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !25 = !DIGlobalVariableExpression(var: !26, expr: !DIExpression())
    !26 = distinct !DIGlobalVariable(name: "__vtable_fb", scope: !2, file: !2, type: !20, isLocal: false, isDefinition: true)
    !27 = !DIGlobalVariableExpression(var: !28, expr: !DIExpression())
    !28 = distinct !DIGlobalVariable(name: "____vtable_fb2_type__init", scope: !2, file: !2, type: !29, isLocal: false, isDefinition: true)
    !29 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_fb2_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !30, identifier: "__vtable_fb2_type")
    !30 = !{!22, !31}
    !31 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_fb_type", scope: !2, file: !2, baseType: !20, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !32 = !DIGlobalVariableExpression(var: !33, expr: !DIExpression())
    !33 = distinct !DIGlobalVariable(name: "__vtable_fb2", scope: !2, file: !2, type: !29, isLocal: false, isDefinition: true)
    !34 = !DIGlobalVariableExpression(var: !35, expr: !DIExpression())
    !35 = distinct !DIGlobalVariable(name: "____vtable_foo_type__init", scope: !2, file: !2, type: !36, isLocal: false, isDefinition: true)
    !36 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !21, identifier: "__vtable_foo_type")
    !37 = !DIGlobalVariableExpression(var: !38, expr: !DIExpression())
    !38 = distinct !DIGlobalVariable(name: "__vtable_foo", scope: !2, file: !2, type: !36, isLocal: false, isDefinition: true)
    !39 = !{i32 2, !"Dwarf Version", i32 5}
    !40 = !{i32 2, !"Debug Info Version", i32 3}
    !41 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !42, splitDebugInlining: false)
    !42 = !{!25, !18, !32, !27, !37, !34, !11, !0, !13}
    !43 = distinct !DISubprogram(name: "fb", linkageName: "fb", scope: !2, file: !2, line: 2, type: !44, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !41, retainedNodes: !46)
    !44 = !DISubroutineType(flags: DIFlagPublic, types: !45)
    !45 = !{null, !6}
    !46 = !{}
    !47 = !DILocalVariable(name: "fb", scope: !43, file: !2, line: 7, type: !6)
    !48 = !DILocation(line: 7, column: 8, scope: !43)
    !49 = distinct !DISubprogram(name: "fb2", linkageName: "fb2", scope: !2, file: !2, line: 9, type: !50, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !41, retainedNodes: !46)
    !50 = !DISubroutineType(flags: DIFlagPublic, types: !51)
    !51 = !{null, !3}
    !52 = !DILocalVariable(name: "fb2", scope: !49, file: !2, line: 10, type: !3)
    !53 = !DILocation(line: 10, column: 8, scope: !49)
    !54 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 12, type: !55, scopeLine: 16, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !41, retainedNodes: !46)
    !55 = !DISubroutineType(flags: DIFlagPublic, types: !56)
    !56 = !{null, !15}
    !57 = !DILocalVariable(name: "foo", scope: !54, file: !2, line: 16, type: !15)
    !58 = !DILocation(line: 16, column: 12, scope: !54)
    !59 = !DILocation(line: 17, column: 8, scope: !54)
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
    %__vtable_foo_type = type { i32*, i32* }
    %__vtable_bar_type = type { i32*, %__vtable_foo_type }

    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"world\00"
    @__bar__init = constant %bar zeroinitializer, !dbg !0
    @__foo__init = constant %foo zeroinitializer, !dbg !13
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer, !dbg !15
    @__vtable_foo = global %__vtable_foo_type zeroinitializer, !dbg !23
    @____vtable_bar_type__init = constant %__vtable_bar_type zeroinitializer, !dbg !25
    @__vtable_bar = global %__vtable_bar_type zeroinitializer, !dbg !30

    define void @foo(%foo* %0) !dbg !36 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !40, metadata !DIExpression()), !dbg !41
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void, !dbg !41
    }

    define void @foo_baz(%foo* %0) !dbg !42 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !43, metadata !DIExpression()), !dbg !44
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %1 = bitcast [81 x i8]* %s to i8*, !dbg !44
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_0, i32 0, i32 0), i32 6, i1 false), !dbg !44
      ret void, !dbg !45
    }

    define void @bar(%bar* %0) !dbg !46 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !49, metadata !DIExpression()), !dbg !50
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %__foo, i32 0, i32 0, !dbg !50
      %1 = bitcast [81 x i8]* %s to i8*, !dbg !50
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_1, i32 0, i32 0), i32 6, i1 false), !dbg !50
      ret void, !dbg !51
    }

    define void @main() !dbg !52 {
    entry:
      %s = alloca [81 x i8], align 1
      %fb = alloca %bar, align 8
      call void @llvm.dbg.declare(metadata [81 x i8]* %s, metadata !55, metadata !DIExpression()), !dbg !56
      %0 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %bar* %fb, metadata !57, metadata !DIExpression()), !dbg !58
      %1 = bitcast %bar* %fb to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 getelementptr inbounds (%bar, %bar* @__bar__init, i32 0, i32 0, i32 0, i32 0), i64 ptrtoint (%bar* getelementptr (%bar, %bar* null, i32 1) to i64), i1 false)
      call void @__init_bar(%bar* %fb), !dbg !59
      call void @__user_init_bar(%bar* %fb), !dbg !59
      %__foo = getelementptr inbounds %bar, %bar* %fb, i32 0, i32 0, !dbg !59
      call void @foo_baz(%foo* %__foo), !dbg !60
      call void @bar(%bar* %fb), !dbg !61
      ret void, !dbg !62
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
      %__vtable_foo_type = getelementptr inbounds %__vtable_bar_type, %__vtable_bar_type* %deref, i32 0, i32 1
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
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      call void @__init___vtable_bar_type(%__vtable_bar_type* @__vtable_bar)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn }
    attributes #2 = { argmemonly nofree nounwind willreturn writeonly }

    !llvm.module.flags = !{!32, !33}
    !llvm.dbg.cu = !{!34}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 11, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 11, size: 704, align: 64, flags: DIFlagPublic, elements: !4, identifier: "bar")
    !4 = !{!5}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !6, size: 648, align: 64, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 704, align: 64, flags: DIFlagPublic, elements: !7, identifier: "foo")
    !7 = !{!8}
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !2, file: !2, line: 4, baseType: !9, size: 648, align: 8, flags: DIFlagPublic)
    !9 = !DICompositeType(tag: DW_TAG_array_type, baseType: !10, size: 648, align: 8, elements: !11)
    !10 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !11 = !{!12}
    !12 = !DISubrange(count: 81, lowerBound: 0)
    !13 = !DIGlobalVariableExpression(var: !14, expr: !DIExpression())
    !14 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !6, isLocal: false, isDefinition: true)
    !15 = !DIGlobalVariableExpression(var: !16, expr: !DIExpression())
    !16 = distinct !DIGlobalVariable(name: "____vtable_foo_type__init", scope: !2, file: !2, type: !17, isLocal: false, isDefinition: true)
    !17 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !18, identifier: "__vtable_foo_type")
    !18 = !{!19, !22}
    !19 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !20, size: 64, align: 64, flags: DIFlagPublic)
    !20 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !21, size: 64, align: 64, dwarfAddressSpace: 1)
    !21 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !22 = !DIDerivedType(tag: DW_TAG_member, name: "foo.baz", scope: !2, file: !2, baseType: !20, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !23 = !DIGlobalVariableExpression(var: !24, expr: !DIExpression())
    !24 = distinct !DIGlobalVariable(name: "__vtable_foo", scope: !2, file: !2, type: !17, isLocal: false, isDefinition: true)
    !25 = !DIGlobalVariableExpression(var: !26, expr: !DIExpression())
    !26 = distinct !DIGlobalVariable(name: "____vtable_bar_type__init", scope: !2, file: !2, type: !27, isLocal: false, isDefinition: true)
    !27 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_bar_type", scope: !2, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !28, identifier: "__vtable_bar_type")
    !28 = !{!19, !29}
    !29 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_foo_type", scope: !2, file: !2, baseType: !17, size: 128, align: 64, offset: 64, flags: DIFlagPublic)
    !30 = !DIGlobalVariableExpression(var: !31, expr: !DIExpression())
    !31 = distinct !DIGlobalVariable(name: "__vtable_bar", scope: !2, file: !2, type: !27, isLocal: false, isDefinition: true)
    !32 = !{i32 2, !"Dwarf Version", i32 5}
    !33 = !{i32 2, !"Debug Info Version", i32 3}
    !34 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !35, splitDebugInlining: false)
    !35 = !{!23, !15, !30, !25, !13, !0}
    !36 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !37, scopeLine: 9, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !34, retainedNodes: !39)
    !37 = !DISubroutineType(flags: DIFlagPublic, types: !38)
    !38 = !{null, !6}
    !39 = !{}
    !40 = !DILocalVariable(name: "foo", scope: !36, file: !2, line: 9, type: !6)
    !41 = !DILocation(line: 9, column: 8, scope: !36)
    !42 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !36, file: !2, line: 6, type: !37, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !34, retainedNodes: !39)
    !43 = !DILocalVariable(name: "foo", scope: !42, file: !2, line: 7, type: !6)
    !44 = !DILocation(line: 7, column: 12, scope: !42)
    !45 = !DILocation(line: 8, column: 8, scope: !42)
    !46 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 11, type: !47, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !34, retainedNodes: !39)
    !47 = !DISubroutineType(flags: DIFlagPublic, types: !48)
    !48 = !{null, !3}
    !49 = !DILocalVariable(name: "bar", scope: !46, file: !2, line: 12, type: !3)
    !50 = !DILocation(line: 12, column: 12, scope: !46)
    !51 = !DILocation(line: 13, column: 8, scope: !46)
    !52 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 15, type: !53, scopeLine: 15, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !34, retainedNodes: !39)
    !53 = !DISubroutineType(flags: DIFlagPublic, types: !54)
    !54 = !{null}
    !55 = !DILocalVariable(name: "s", scope: !52, file: !2, line: 17, type: !9, align: 8)
    !56 = !DILocation(line: 17, column: 12, scope: !52)
    !57 = !DILocalVariable(name: "fb", scope: !52, file: !2, line: 18, type: !3, align: 64)
    !58 = !DILocation(line: 18, column: 12, scope: !52)
    !59 = !DILocation(line: 0, scope: !52)
    !60 = !DILocation(line: 20, column: 12, scope: !52)
    !61 = !DILocation(line: 21, column: 12, scope: !52)
    !62 = !DILocation(line: 22, column: 8, scope: !52)
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
    %__vtable_grandparent_type = type { i32* }
    %__vtable_parent_type = type { i32*, %__vtable_grandparent_type }
    %__vtable_child_type = type { i32*, %__vtable_parent_type }

    @__child__init = constant %child zeroinitializer, !dbg !0
    @__parent__init = constant %parent zeroinitializer, !dbg !23
    @__grandparent__init = constant %grandparent zeroinitializer, !dbg !25
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_grandparent_type__init = constant %__vtable_grandparent_type zeroinitializer, !dbg !27
    @__vtable_grandparent = global %__vtable_grandparent_type zeroinitializer, !dbg !34
    @____vtable_parent_type__init = constant %__vtable_parent_type zeroinitializer, !dbg !36
    @__vtable_parent = global %__vtable_parent_type zeroinitializer, !dbg !41
    @____vtable_child_type__init = constant %__vtable_child_type zeroinitializer, !dbg !43
    @__vtable_child = global %__vtable_child_type zeroinitializer, !dbg !48

    define void @grandparent(%grandparent* %0) !dbg !54 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !58, metadata !DIExpression()), !dbg !59
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      ret void, !dbg !59
    }

    define void @parent(%parent* %0) !dbg !60 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !63, metadata !DIExpression()), !dbg !64
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void, !dbg !64
    }

    define void @child(%child* %0) !dbg !65 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !68, metadata !DIExpression()), !dbg !69
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      ret void, !dbg !69
    }

    define void @main() !dbg !70 {
    entry:
      %arr = alloca [11 x %child], align 8
      call void @llvm.dbg.declare(metadata [11 x %child]* %arr, metadata !73, metadata !DIExpression()), !dbg !75
      %0 = bitcast [11 x %child]* %arr to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([11 x %child]* getelementptr ([11 x %child], [11 x %child]* null, i32 1) to i64), i1 false)
      %tmpVar = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !76
      %__parent = getelementptr inbounds %child, %child* %tmpVar, i32 0, i32 0, !dbg !76
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !76
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 1, !dbg !76
      store i16 10, i16* %a, align 2, !dbg !76
      %tmpVar1 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !77
      %__parent2 = getelementptr inbounds %child, %child* %tmpVar1, i32 0, i32 0, !dbg !77
      %__grandparent3 = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0, !dbg !77
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent3, i32 0, i32 0, !dbg !77
      %tmpVar4 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 0, !dbg !77
      store i16 20, i16* %tmpVar4, align 2, !dbg !77
      %tmpVar5 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !78
      %__parent6 = getelementptr inbounds %child, %child* %tmpVar5, i32 0, i32 0, !dbg !78
      %b = getelementptr inbounds %parent, %parent* %__parent6, i32 0, i32 2, !dbg !78
      store i16 30, i16* %b, align 2, !dbg !78
      %tmpVar7 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !79
      %__parent8 = getelementptr inbounds %child, %child* %tmpVar7, i32 0, i32 0, !dbg !79
      %x = getelementptr inbounds %parent, %parent* %__parent8, i32 0, i32 1, !dbg !79
      %tmpVar9 = getelementptr inbounds [11 x i16], [11 x i16]* %x, i32 0, i32 1, !dbg !79
      store i16 40, i16* %tmpVar9, align 2, !dbg !79
      %tmpVar10 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 2, !dbg !80
      %z = getelementptr inbounds %child, %child* %tmpVar10, i32 0, i32 1, !dbg !80
      %tmpVar11 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 2, !dbg !80
      store i16 50, i16* %tmpVar11, align 2, !dbg !80
      ret void, !dbg !81
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
      %__vtable_grandparent_type = getelementptr inbounds %__vtable_parent_type, %__vtable_parent_type* %deref, i32 0, i32 1
      call void @__init___vtable_grandparent_type(%__vtable_grandparent_type* %__vtable_grandparent_type)
      ret void
    }

    define void @__init___vtable_child_type(%__vtable_child_type* %0) {
    entry:
      %self = alloca %__vtable_child_type*, align 8
      store %__vtable_child_type* %0, %__vtable_child_type** %self, align 8
      %deref = load %__vtable_child_type*, %__vtable_child_type** %self, align 8
      %__vtable_parent_type = getelementptr inbounds %__vtable_child_type, %__vtable_child_type* %deref, i32 0, i32 1
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
      call void @__init___vtable_grandparent_type(%__vtable_grandparent_type* @__vtable_grandparent)
      call void @__init___vtable_parent_type(%__vtable_parent_type* @__vtable_parent)
      call void @__init___vtable_child_type(%__vtable_child_type* @__vtable_child)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn writeonly }

    !llvm.module.flags = !{!50, !51}
    !llvm.dbg.cu = !{!52}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 512, align: 64, flags: DIFlagPublic, elements: !4, identifier: "child")
    !4 = !{!5, !22}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !6, size: 304, align: 64, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 320, align: 64, flags: DIFlagPublic, elements: !7, identifier: "parent")
    !7 = !{!8, !17, !21}
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "__grandparent", scope: !2, file: !2, baseType: !9, size: 112, align: 64, flags: DIFlagPublic)
    !9 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 128, align: 64, flags: DIFlagPublic, elements: !10, identifier: "grandparent")
    !10 = !{!11, !16}
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !12, size: 96, align: 16, flags: DIFlagPublic)
    !12 = !DICompositeType(tag: DW_TAG_array_type, baseType: !13, size: 96, align: 16, elements: !14)
    !13 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !14 = !{!15}
    !15 = !DISubrange(count: 6, lowerBound: 0)
    !16 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !13, size: 16, align: 16, offset: 96, flags: DIFlagPublic)
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !18, size: 176, align: 16, offset: 112, flags: DIFlagPublic)
    !18 = !DICompositeType(tag: DW_TAG_array_type, baseType: !13, size: 176, align: 16, elements: !19)
    !19 = !{!20}
    !20 = !DISubrange(count: 11, lowerBound: 0)
    !21 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !13, size: 16, align: 16, offset: 288, flags: DIFlagPublic)
    !22 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !18, size: 176, align: 16, offset: 304, flags: DIFlagPublic)
    !23 = !DIGlobalVariableExpression(var: !24, expr: !DIExpression())
    !24 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !6, isLocal: false, isDefinition: true)
    !25 = !DIGlobalVariableExpression(var: !26, expr: !DIExpression())
    !26 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !9, isLocal: false, isDefinition: true)
    !27 = !DIGlobalVariableExpression(var: !28, expr: !DIExpression())
    !28 = distinct !DIGlobalVariable(name: "____vtable_grandparent_type__init", scope: !2, file: !2, type: !29, isLocal: false, isDefinition: true)
    !29 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_grandparent_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !30, identifier: "__vtable_grandparent_type")
    !30 = !{!31}
    !31 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !32, size: 64, align: 64, flags: DIFlagPublic)
    !32 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !33, size: 64, align: 64, dwarfAddressSpace: 1)
    !33 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !34 = !DIGlobalVariableExpression(var: !35, expr: !DIExpression())
    !35 = distinct !DIGlobalVariable(name: "__vtable_grandparent", scope: !2, file: !2, type: !29, isLocal: false, isDefinition: true)
    !36 = !DIGlobalVariableExpression(var: !37, expr: !DIExpression())
    !37 = distinct !DIGlobalVariable(name: "____vtable_parent_type__init", scope: !2, file: !2, type: !38, isLocal: false, isDefinition: true)
    !38 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_parent_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !39, identifier: "__vtable_parent_type")
    !39 = !{!31, !40}
    !40 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_grandparent_type", scope: !2, file: !2, baseType: !29, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !41 = !DIGlobalVariableExpression(var: !42, expr: !DIExpression())
    !42 = distinct !DIGlobalVariable(name: "__vtable_parent", scope: !2, file: !2, type: !38, isLocal: false, isDefinition: true)
    !43 = !DIGlobalVariableExpression(var: !44, expr: !DIExpression())
    !44 = distinct !DIGlobalVariable(name: "____vtable_child_type__init", scope: !2, file: !2, type: !45, isLocal: false, isDefinition: true)
    !45 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_child_type", scope: !2, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !46, identifier: "__vtable_child_type")
    !46 = !{!31, !47}
    !47 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_parent_type", scope: !2, file: !2, baseType: !38, size: 128, align: 64, offset: 64, flags: DIFlagPublic)
    !48 = !DIGlobalVariableExpression(var: !49, expr: !DIExpression())
    !49 = distinct !DIGlobalVariable(name: "__vtable_child", scope: !2, file: !2, type: !45, isLocal: false, isDefinition: true)
    !50 = !{i32 2, !"Dwarf Version", i32 5}
    !51 = !{i32 2, !"Debug Info Version", i32 3}
    !52 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !53, splitDebugInlining: false)
    !53 = !{!34, !27, !41, !36, !48, !43, !25, !23, !0}
    !54 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !55, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !52, retainedNodes: !57)
    !55 = !DISubroutineType(flags: DIFlagPublic, types: !56)
    !56 = !{null, !9}
    !57 = !{}
    !58 = !DILocalVariable(name: "grandparent", scope: !54, file: !2, line: 7, type: !9)
    !59 = !DILocation(line: 7, column: 8, scope: !54)
    !60 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !61, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !52, retainedNodes: !57)
    !61 = !DISubroutineType(flags: DIFlagPublic, types: !62)
    !62 = !{null, !6}
    !63 = !DILocalVariable(name: "parent", scope: !60, file: !2, line: 14, type: !6)
    !64 = !DILocation(line: 14, column: 8, scope: !60)
    !65 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !66, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !52, retainedNodes: !57)
    !66 = !DISubroutineType(flags: DIFlagPublic, types: !67)
    !67 = !{null, !3}
    !68 = !DILocalVariable(name: "child", scope: !65, file: !2, line: 20, type: !3)
    !69 = !DILocation(line: 20, column: 8, scope: !65)
    !70 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 22, type: !71, scopeLine: 26, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !52, retainedNodes: !57)
    !71 = !DISubroutineType(flags: DIFlagPublic, types: !72)
    !72 = !{null}
    !73 = !DILocalVariable(name: "arr", scope: !70, file: !2, line: 24, type: !74, align: 64)
    !74 = !DICompositeType(tag: DW_TAG_array_type, baseType: !3, size: 5280, align: 64, elements: !19)
    !75 = !DILocation(line: 24, column: 12, scope: !70)
    !76 = !DILocation(line: 26, column: 12, scope: !70)
    !77 = !DILocation(line: 27, column: 12, scope: !70)
    !78 = !DILocation(line: 28, column: 12, scope: !70)
    !79 = !DILocation(line: 29, column: 12, scope: !70)
    !80 = !DILocation(line: 30, column: 12, scope: !70)
    !81 = !DILocation(line: 31, column: 8, scope: !70)
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
    %__vtable_grandparent_type = type { i32* }
    %__vtable_parent_type = type { i32*, %__vtable_grandparent_type }
    %__vtable_child_type = type { i32*, %__vtable_parent_type }

    @__parent__init = constant %parent zeroinitializer, !dbg !0
    @__grandparent__init = constant %grandparent zeroinitializer, !dbg !19
    @__child__init = constant %child zeroinitializer, !dbg !21
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_grandparent_type__init = constant %__vtable_grandparent_type zeroinitializer, !dbg !27
    @__vtable_grandparent = global %__vtable_grandparent_type zeroinitializer, !dbg !34
    @____vtable_parent_type__init = constant %__vtable_parent_type zeroinitializer, !dbg !36
    @__vtable_parent = global %__vtable_parent_type zeroinitializer, !dbg !41
    @____vtable_child_type__init = constant %__vtable_child_type zeroinitializer, !dbg !43
    @__vtable_child = global %__vtable_child_type zeroinitializer, !dbg !48

    define void @grandparent(%grandparent* %0) !dbg !54 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !58, metadata !DIExpression()), !dbg !59
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      ret void, !dbg !59
    }

    define void @parent(%parent* %0) !dbg !60 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !63, metadata !DIExpression()), !dbg !64
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void, !dbg !64
    }

    define void @child(%child* %0) !dbg !65 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !68, metadata !DIExpression()), !dbg !69
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !69
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 0, !dbg !69
      %b = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2, !dbg !69
      %load_b = load i16, i16* %b, align 2, !dbg !69
      %1 = sext i16 %load_b to i32, !dbg !69
      %b1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2, !dbg !69
      %load_b2 = load i16, i16* %b1, align 2, !dbg !69
      %2 = sext i16 %load_b2 to i32, !dbg !69
      %tmpVar = mul i32 %2, 2, !dbg !69
      %tmpVar3 = mul i32 1, %tmpVar, !dbg !69
      %tmpVar4 = add i32 %tmpVar3, 0, !dbg !69
      %tmpVar5 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 %tmpVar4, !dbg !69
      %load_tmpVar = load i16, i16* %tmpVar5, align 2, !dbg !69
      %3 = sext i16 %load_tmpVar to i32, !dbg !69
      %tmpVar6 = add i32 %1, %3, !dbg !69
      %__grandparent7 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !69
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent7, i32 0, i32 1, !dbg !69
      %load_a = load i16, i16* %a, align 2, !dbg !69
      %4 = sext i16 %load_a to i32, !dbg !69
      %tmpVar8 = sub i32 %tmpVar6, %4, !dbg !69
      %tmpVar9 = mul i32 1, %tmpVar8, !dbg !69
      %tmpVar10 = add i32 %tmpVar9, 0, !dbg !69
      %tmpVar11 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 %tmpVar10, !dbg !69
      store i16 20, i16* %tmpVar11, align 2, !dbg !69
      ret void, !dbg !70
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
      %__vtable_grandparent_type = getelementptr inbounds %__vtable_parent_type, %__vtable_parent_type* %deref, i32 0, i32 1
      call void @__init___vtable_grandparent_type(%__vtable_grandparent_type* %__vtable_grandparent_type)
      ret void
    }

    define void @__init___vtable_child_type(%__vtable_child_type* %0) {
    entry:
      %self = alloca %__vtable_child_type*, align 8
      store %__vtable_child_type* %0, %__vtable_child_type** %self, align 8
      %deref = load %__vtable_child_type*, %__vtable_child_type** %self, align 8
      %__vtable_parent_type = getelementptr inbounds %__vtable_child_type, %__vtable_child_type* %deref, i32 0, i32 1
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
      call void @__init___vtable_grandparent_type(%__vtable_grandparent_type* @__vtable_grandparent)
      call void @__init___vtable_parent_type(%__vtable_parent_type* @__vtable_parent)
      call void @__init___vtable_child_type(%__vtable_child_type* @__vtable_child)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!50, !51}
    !llvm.dbg.cu = !{!52}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 320, align: 64, flags: DIFlagPublic, elements: !4, identifier: "parent")
    !4 = !{!5, !14, !18}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__grandparent", scope: !2, file: !2, baseType: !6, size: 112, align: 64, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 128, align: 64, flags: DIFlagPublic, elements: !7, identifier: "grandparent")
    !7 = !{!8, !13}
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !9, size: 96, align: 16, flags: DIFlagPublic)
    !9 = !DICompositeType(tag: DW_TAG_array_type, baseType: !10, size: 96, align: 16, elements: !11)
    !10 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !11 = !{!12}
    !12 = !DISubrange(count: 6, lowerBound: 0)
    !13 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !10, size: 16, align: 16, offset: 96, flags: DIFlagPublic)
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !15, size: 176, align: 16, offset: 112, flags: DIFlagPublic)
    !15 = !DICompositeType(tag: DW_TAG_array_type, baseType: !10, size: 176, align: 16, elements: !16)
    !16 = !{!17}
    !17 = !DISubrange(count: 11, lowerBound: 0)
    !18 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !10, size: 16, align: 16, offset: 288, flags: DIFlagPublic)
    !19 = !DIGlobalVariableExpression(var: !20, expr: !DIExpression())
    !20 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !6, isLocal: false, isDefinition: true)
    !21 = !DIGlobalVariableExpression(var: !22, expr: !DIExpression())
    !22 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !23, isLocal: false, isDefinition: true)
    !23 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 512, align: 64, flags: DIFlagPublic, elements: !24, identifier: "child")
    !24 = !{!25, !26}
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !3, size: 304, align: 64, flags: DIFlagPublic)
    !26 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !15, size: 176, align: 16, offset: 304, flags: DIFlagPublic)
    !27 = !DIGlobalVariableExpression(var: !28, expr: !DIExpression())
    !28 = distinct !DIGlobalVariable(name: "____vtable_grandparent_type__init", scope: !2, file: !2, type: !29, isLocal: false, isDefinition: true)
    !29 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_grandparent_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !30, identifier: "__vtable_grandparent_type")
    !30 = !{!31}
    !31 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !32, size: 64, align: 64, flags: DIFlagPublic)
    !32 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !33, size: 64, align: 64, dwarfAddressSpace: 1)
    !33 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !34 = !DIGlobalVariableExpression(var: !35, expr: !DIExpression())
    !35 = distinct !DIGlobalVariable(name: "__vtable_grandparent", scope: !2, file: !2, type: !29, isLocal: false, isDefinition: true)
    !36 = !DIGlobalVariableExpression(var: !37, expr: !DIExpression())
    !37 = distinct !DIGlobalVariable(name: "____vtable_parent_type__init", scope: !2, file: !2, type: !38, isLocal: false, isDefinition: true)
    !38 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_parent_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !39, identifier: "__vtable_parent_type")
    !39 = !{!31, !40}
    !40 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_grandparent_type", scope: !2, file: !2, baseType: !29, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !41 = !DIGlobalVariableExpression(var: !42, expr: !DIExpression())
    !42 = distinct !DIGlobalVariable(name: "__vtable_parent", scope: !2, file: !2, type: !38, isLocal: false, isDefinition: true)
    !43 = !DIGlobalVariableExpression(var: !44, expr: !DIExpression())
    !44 = distinct !DIGlobalVariable(name: "____vtable_child_type__init", scope: !2, file: !2, type: !45, isLocal: false, isDefinition: true)
    !45 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_child_type", scope: !2, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !46, identifier: "__vtable_child_type")
    !46 = !{!31, !47}
    !47 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_parent_type", scope: !2, file: !2, baseType: !38, size: 128, align: 64, offset: 64, flags: DIFlagPublic)
    !48 = !DIGlobalVariableExpression(var: !49, expr: !DIExpression())
    !49 = distinct !DIGlobalVariable(name: "__vtable_child", scope: !2, file: !2, type: !45, isLocal: false, isDefinition: true)
    !50 = !{i32 2, !"Dwarf Version", i32 5}
    !51 = !{i32 2, !"Debug Info Version", i32 3}
    !52 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !53, splitDebugInlining: false)
    !53 = !{!34, !27, !41, !36, !48, !43, !19, !0, !21}
    !54 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !55, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !52, retainedNodes: !57)
    !55 = !DISubroutineType(flags: DIFlagPublic, types: !56)
    !56 = !{null, !6}
    !57 = !{}
    !58 = !DILocalVariable(name: "grandparent", scope: !54, file: !2, line: 7, type: !6)
    !59 = !DILocation(line: 7, column: 8, scope: !54)
    !60 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !61, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !52, retainedNodes: !57)
    !61 = !DISubroutineType(flags: DIFlagPublic, types: !62)
    !62 = !{null, !3}
    !63 = !DILocalVariable(name: "parent", scope: !60, file: !2, line: 14, type: !3)
    !64 = !DILocation(line: 14, column: 8, scope: !60)
    !65 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !66, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !52, retainedNodes: !57)
    !66 = !DISubroutineType(flags: DIFlagPublic, types: !67)
    !67 = !{null, !23}
    !68 = !DILocalVariable(name: "child", scope: !65, file: !2, line: 20, type: !23)
    !69 = !DILocation(line: 20, column: 12, scope: !65)
    !70 = !DILocation(line: 21, column: 8, scope: !65)
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

    %foo = type {}
    %bar = type { %foo }
    %__vtable_foo_type = type { i32*, i32* }
    %__vtable_bar_type = type { i32*, %__vtable_foo_type }

    @__foo__init = constant %foo zeroinitializer, !dbg !0
    @__bar__init = constant %bar zeroinitializer, !dbg !5
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer, !dbg !10
    @__vtable_foo = global %__vtable_foo_type zeroinitializer, !dbg !18
    @____vtable_bar_type__init = constant %__vtable_bar_type zeroinitializer, !dbg !20
    @__vtable_bar = global %__vtable_bar_type zeroinitializer, !dbg !25

    define void @foo(%foo* %0) !dbg !31 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !34, metadata !DIExpression()), !dbg !35
      ret void, !dbg !35
    }

    define void @foo_baz(%foo* %0) !dbg !36 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !37, metadata !DIExpression()), !dbg !38
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
      %__vtable_foo_type = getelementptr inbounds %__vtable_bar_type, %__vtable_bar_type* %deref, i32 0, i32 1
      call void @__init___vtable_foo_type(%__vtable_foo_type* %__vtable_foo_type)
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

    !llvm.module.flags = !{!27, !28}
    !llvm.dbg.cu = !{!29}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, align: 64, flags: DIFlagPublic, elements: !4, identifier: "foo")
    !4 = !{}
    !5 = !DIGlobalVariableExpression(var: !6, expr: !DIExpression())
    !6 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 7, type: !7, isLocal: false, isDefinition: true)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 7, align: 64, flags: DIFlagPublic, elements: !8, identifier: "bar")
    !8 = !{!9}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !3, align: 64, flags: DIFlagPublic)
    !10 = !DIGlobalVariableExpression(var: !11, expr: !DIExpression())
    !11 = distinct !DIGlobalVariable(name: "____vtable_foo_type__init", scope: !2, file: !2, type: !12, isLocal: false, isDefinition: true)
    !12 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !13, identifier: "__vtable_foo_type")
    !13 = !{!14, !17}
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !15, size: 64, align: 64, flags: DIFlagPublic)
    !15 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !16, size: 64, align: 64, dwarfAddressSpace: 1)
    !16 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "foo.baz", scope: !2, file: !2, baseType: !15, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !18 = !DIGlobalVariableExpression(var: !19, expr: !DIExpression())
    !19 = distinct !DIGlobalVariable(name: "__vtable_foo", scope: !2, file: !2, type: !12, isLocal: false, isDefinition: true)
    !20 = !DIGlobalVariableExpression(var: !21, expr: !DIExpression())
    !21 = distinct !DIGlobalVariable(name: "____vtable_bar_type__init", scope: !2, file: !2, type: !22, isLocal: false, isDefinition: true)
    !22 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_bar_type", scope: !2, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !23, identifier: "__vtable_bar_type")
    !23 = !{!14, !24}
    !24 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_foo_type", scope: !2, file: !2, baseType: !12, size: 128, align: 64, offset: 64, flags: DIFlagPublic)
    !25 = !DIGlobalVariableExpression(var: !26, expr: !DIExpression())
    !26 = distinct !DIGlobalVariable(name: "__vtable_bar", scope: !2, file: !2, type: !22, isLocal: false, isDefinition: true)
    !27 = !{i32 2, !"Dwarf Version", i32 5}
    !28 = !{i32 2, !"Debug Info Version", i32 3}
    !29 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !30, splitDebugInlining: false)
    !30 = !{!18, !10, !25, !20, !0, !5}
    !31 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !32, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !4)
    !32 = !DISubroutineType(flags: DIFlagPublic, types: !33)
    !33 = !{null, !3}
    !34 = !DILocalVariable(name: "foo", scope: !31, file: !2, line: 5, type: !3)
    !35 = !DILocation(line: 5, column: 8, scope: !31)
    !36 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !31, file: !2, line: 3, type: !32, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !4)
    !37 = !DILocalVariable(name: "foo", scope: !36, file: !2, line: 4, type: !3)
    !38 = !DILocation(line: 4, column: 8, scope: !36)
    !39 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 7, type: !40, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !4)
    !40 = !DISubroutineType(flags: DIFlagPublic, types: !41)
    !41 = !{null, !7}
    !42 = !DILocalVariable(name: "bar", scope: !39, file: !2, line: 8, type: !7)
    !43 = !DILocation(line: 8, column: 8, scope: !39)
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

    %grandchild = type { %child, i32 }
    %child = type { %parent, i32 }
    %parent = type { i32 }
    %__vtable_parent_type = type { i32* }
    %__vtable_child_type = type { i32*, %__vtable_parent_type }
    %__vtable_grandchild_type = type { i32*, %__vtable_child_type }

    @__grandchild__init = constant %grandchild zeroinitializer, !dbg !0
    @__child__init = constant %child zeroinitializer, !dbg !15
    @__parent__init = constant %parent zeroinitializer, !dbg !17
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_parent_type__init = constant %__vtable_parent_type zeroinitializer, !dbg !19
    @__vtable_parent = global %__vtable_parent_type zeroinitializer, !dbg !26
    @____vtable_child_type__init = constant %__vtable_child_type zeroinitializer, !dbg !28
    @__vtable_child = global %__vtable_child_type zeroinitializer, !dbg !33
    @____vtable_grandchild_type__init = constant %__vtable_grandchild_type zeroinitializer, !dbg !35
    @__vtable_grandchild = global %__vtable_grandchild_type zeroinitializer, !dbg !40

    define void @parent(%parent* %0) !dbg !46 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !50, metadata !DIExpression()), !dbg !51
      %a = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      ret void, !dbg !51
    }

    define void @child(%child* %0) !dbg !52 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !55, metadata !DIExpression()), !dbg !56
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %b = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      ret void, !dbg !56
    }

    define void @grandchild(%grandchild* %0) !dbg !57 {
    entry:
      call void @llvm.dbg.declare(metadata %grandchild* %0, metadata !60, metadata !DIExpression()), !dbg !61
      %__child = getelementptr inbounds %grandchild, %grandchild* %0, i32 0, i32 0
      %c = getelementptr inbounds %grandchild, %grandchild* %0, i32 0, i32 1
      ret void, !dbg !61
    }

    define i32 @main() !dbg !62 {
    entry:
      %main = alloca i32, align 4
      %array_of_parent = alloca [3 x %parent], align 8
      %array_of_child = alloca [3 x %child], align 8
      %array_of_grandchild = alloca [3 x %grandchild], align 8
      %parent1 = alloca %parent, align 8
      %child1 = alloca %child, align 8
      %grandchild1 = alloca %grandchild, align 8
      call void @llvm.dbg.declare(metadata [3 x %parent]* %array_of_parent, metadata !65, metadata !DIExpression()), !dbg !69
      %0 = bitcast [3 x %parent]* %array_of_parent to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([3 x %parent]* getelementptr ([3 x %parent], [3 x %parent]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata [3 x %child]* %array_of_child, metadata !70, metadata !DIExpression()), !dbg !72
      %1 = bitcast [3 x %child]* %array_of_child to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %1, i8 0, i64 ptrtoint ([3 x %child]* getelementptr ([3 x %child], [3 x %child]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata [3 x %grandchild]* %array_of_grandchild, metadata !73, metadata !DIExpression()), !dbg !75
      %2 = bitcast [3 x %grandchild]* %array_of_grandchild to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %2, i8 0, i64 ptrtoint ([3 x %grandchild]* getelementptr ([3 x %grandchild], [3 x %grandchild]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %parent* %parent1, metadata !76, metadata !DIExpression()), !dbg !77
      %3 = bitcast %parent* %parent1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %3, i8* align 1 bitcast (%parent* @__parent__init to i8*), i64 ptrtoint (%parent* getelementptr (%parent, %parent* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %child* %child1, metadata !78, metadata !DIExpression()), !dbg !79
      %4 = bitcast %child* %child1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %4, i8* align 1 bitcast (%child* @__child__init to i8*), i64 ptrtoint (%child* getelementptr (%child, %child* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %grandchild* %grandchild1, metadata !80, metadata !DIExpression()), !dbg !81
      %5 = bitcast %grandchild* %grandchild1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %5, i8* align 1 bitcast (%grandchild* @__grandchild__init to i8*), i64 ptrtoint (%grandchild* getelementptr (%grandchild, %grandchild* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata i32* %main, metadata !82, metadata !DIExpression()), !dbg !83
      store i32 0, i32* %main, align 4
      call void @__init_parent(%parent* %parent1), !dbg !84
      call void @__init_child(%child* %child1), !dbg !84
      call void @__init_grandchild(%grandchild* %grandchild1), !dbg !84
      call void @__user_init_parent(%parent* %parent1), !dbg !84
      call void @__user_init_child(%child* %child1), !dbg !84
      call void @__user_init_grandchild(%grandchild* %grandchild1), !dbg !84
      %a = getelementptr inbounds %parent, %parent* %parent1, i32 0, i32 0, !dbg !85
      store i32 1, i32* %a, align 4, !dbg !85
      %__parent = getelementptr inbounds %child, %child* %child1, i32 0, i32 0, !dbg !86
      %a1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !86
      store i32 2, i32* %a1, align 4, !dbg !86
      %b = getelementptr inbounds %child, %child* %child1, i32 0, i32 1, !dbg !87
      store i32 3, i32* %b, align 4, !dbg !87
      %__child = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 0, !dbg !88
      %__parent2 = getelementptr inbounds %child, %child* %__child, i32 0, i32 0, !dbg !88
      %a3 = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0, !dbg !88
      store i32 4, i32* %a3, align 4, !dbg !88
      %__child4 = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 0, !dbg !89
      %b5 = getelementptr inbounds %child, %child* %__child4, i32 0, i32 1, !dbg !89
      store i32 5, i32* %b5, align 4, !dbg !89
      %c = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 1, !dbg !90
      store i32 6, i32* %c, align 4, !dbg !90
      %tmpVar = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 0, !dbg !91
      %a6 = getelementptr inbounds %parent, %parent* %tmpVar, i32 0, i32 0, !dbg !91
      store i32 7, i32* %a6, align 4, !dbg !91
      %tmpVar7 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 0, !dbg !92
      %__parent8 = getelementptr inbounds %child, %child* %tmpVar7, i32 0, i32 0, !dbg !92
      %a9 = getelementptr inbounds %parent, %parent* %__parent8, i32 0, i32 0, !dbg !92
      store i32 8, i32* %a9, align 4, !dbg !92
      %tmpVar10 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 0, !dbg !93
      %b11 = getelementptr inbounds %child, %child* %tmpVar10, i32 0, i32 1, !dbg !93
      store i32 9, i32* %b11, align 4, !dbg !93
      %tmpVar12 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !94
      %__child13 = getelementptr inbounds %grandchild, %grandchild* %tmpVar12, i32 0, i32 0, !dbg !94
      %__parent14 = getelementptr inbounds %child, %child* %__child13, i32 0, i32 0, !dbg !94
      %a15 = getelementptr inbounds %parent, %parent* %__parent14, i32 0, i32 0, !dbg !94
      store i32 10, i32* %a15, align 4, !dbg !94
      %tmpVar16 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !95
      %__child17 = getelementptr inbounds %grandchild, %grandchild* %tmpVar16, i32 0, i32 0, !dbg !95
      %b18 = getelementptr inbounds %child, %child* %__child17, i32 0, i32 1, !dbg !95
      store i32 11, i32* %b18, align 4, !dbg !95
      %tmpVar19 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !96
      %c20 = getelementptr inbounds %grandchild, %grandchild* %tmpVar19, i32 0, i32 1, !dbg !96
      store i32 12, i32* %c20, align 4, !dbg !96
      %tmpVar21 = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 1, !dbg !97
      %a22 = getelementptr inbounds %parent, %parent* %tmpVar21, i32 0, i32 0, !dbg !97
      store i32 13, i32* %a22, align 4, !dbg !97
      %tmpVar23 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 1, !dbg !98
      %__parent24 = getelementptr inbounds %child, %child* %tmpVar23, i32 0, i32 0, !dbg !98
      %a25 = getelementptr inbounds %parent, %parent* %__parent24, i32 0, i32 0, !dbg !98
      store i32 14, i32* %a25, align 4, !dbg !98
      %tmpVar26 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 1, !dbg !99
      %b27 = getelementptr inbounds %child, %child* %tmpVar26, i32 0, i32 1, !dbg !99
      store i32 15, i32* %b27, align 4, !dbg !99
      %tmpVar28 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !100
      %__child29 = getelementptr inbounds %grandchild, %grandchild* %tmpVar28, i32 0, i32 0, !dbg !100
      %__parent30 = getelementptr inbounds %child, %child* %__child29, i32 0, i32 0, !dbg !100
      %a31 = getelementptr inbounds %parent, %parent* %__parent30, i32 0, i32 0, !dbg !100
      store i32 16, i32* %a31, align 4, !dbg !100
      %tmpVar32 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !101
      %__child33 = getelementptr inbounds %grandchild, %grandchild* %tmpVar32, i32 0, i32 0, !dbg !101
      %b34 = getelementptr inbounds %child, %child* %__child33, i32 0, i32 1, !dbg !101
      store i32 17, i32* %b34, align 4, !dbg !101
      %tmpVar35 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !102
      %c36 = getelementptr inbounds %grandchild, %grandchild* %tmpVar35, i32 0, i32 1, !dbg !102
      store i32 18, i32* %c36, align 4, !dbg !102
      %tmpVar37 = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 2, !dbg !103
      %a38 = getelementptr inbounds %parent, %parent* %tmpVar37, i32 0, i32 0, !dbg !103
      store i32 19, i32* %a38, align 4, !dbg !103
      %tmpVar39 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 2, !dbg !104
      %__parent40 = getelementptr inbounds %child, %child* %tmpVar39, i32 0, i32 0, !dbg !104
      %a41 = getelementptr inbounds %parent, %parent* %__parent40, i32 0, i32 0, !dbg !104
      store i32 20, i32* %a41, align 4, !dbg !104
      %tmpVar42 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 2, !dbg !105
      %b43 = getelementptr inbounds %child, %child* %tmpVar42, i32 0, i32 1, !dbg !105
      store i32 21, i32* %b43, align 4, !dbg !105
      %tmpVar44 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !106
      %__child45 = getelementptr inbounds %grandchild, %grandchild* %tmpVar44, i32 0, i32 0, !dbg !106
      %__parent46 = getelementptr inbounds %child, %child* %__child45, i32 0, i32 0, !dbg !106
      %a47 = getelementptr inbounds %parent, %parent* %__parent46, i32 0, i32 0, !dbg !106
      store i32 22, i32* %a47, align 4, !dbg !106
      %tmpVar48 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !107
      %__child49 = getelementptr inbounds %grandchild, %grandchild* %tmpVar48, i32 0, i32 0, !dbg !107
      %b50 = getelementptr inbounds %child, %child* %__child49, i32 0, i32 1, !dbg !107
      store i32 23, i32* %b50, align 4, !dbg !107
      %tmpVar51 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !108
      %c52 = getelementptr inbounds %grandchild, %grandchild* %tmpVar51, i32 0, i32 1, !dbg !108
      store i32 24, i32* %c52, align 4, !dbg !108
      %main_ret = load i32, i32* %main, align 4, !dbg !109
      ret i32 %main_ret, !dbg !109
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
      %__vtable_parent_type = getelementptr inbounds %__vtable_child_type, %__vtable_child_type* %deref, i32 0, i32 1
      call void @__init___vtable_parent_type(%__vtable_parent_type* %__vtable_parent_type)
      ret void
    }

    define void @__init___vtable_grandchild_type(%__vtable_grandchild_type* %0) {
    entry:
      %self = alloca %__vtable_grandchild_type*, align 8
      store %__vtable_grandchild_type* %0, %__vtable_grandchild_type** %self, align 8
      %deref = load %__vtable_grandchild_type*, %__vtable_grandchild_type** %self, align 8
      %__vtable_child_type = getelementptr inbounds %__vtable_grandchild_type, %__vtable_grandchild_type* %deref, i32 0, i32 1
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
      call void @__init___vtable_parent_type(%__vtable_parent_type* @__vtable_parent)
      call void @__init___vtable_child_type(%__vtable_child_type* @__vtable_child)
      call void @__init___vtable_grandchild_type(%__vtable_grandchild_type* @__vtable_grandchild)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn writeonly }
    attributes #2 = { argmemonly nofree nounwind willreturn }

    !llvm.module.flags = !{!42, !43}
    !llvm.dbg.cu = !{!44}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__grandchild__init", scope: !2, file: !2, line: 14, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandchild", scope: !2, file: !2, line: 14, size: 128, align: 64, flags: DIFlagPublic, elements: !4, identifier: "grandchild")
    !4 = !{!5, !14}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__child", scope: !2, file: !2, baseType: !6, size: 64, align: 64, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 8, size: 64, align: 64, flags: DIFlagPublic, elements: !7, identifier: "child")
    !7 = !{!8, !13}
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !9, size: 32, align: 64, flags: DIFlagPublic)
    !9 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 2, size: 64, align: 64, flags: DIFlagPublic, elements: !10, identifier: "parent")
    !10 = !{!11}
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 4, baseType: !12, size: 32, align: 32, flags: DIFlagPublic)
    !12 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !13 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 10, baseType: !12, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 16, baseType: !12, size: 32, align: 32, offset: 64, flags: DIFlagPublic)
    !15 = !DIGlobalVariableExpression(var: !16, expr: !DIExpression())
    !16 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 8, type: !6, isLocal: false, isDefinition: true)
    !17 = !DIGlobalVariableExpression(var: !18, expr: !DIExpression())
    !18 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 2, type: !9, isLocal: false, isDefinition: true)
    !19 = !DIGlobalVariableExpression(var: !20, expr: !DIExpression())
    !20 = distinct !DIGlobalVariable(name: "____vtable_parent_type__init", scope: !2, file: !2, type: !21, isLocal: false, isDefinition: true)
    !21 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_parent_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !22, identifier: "__vtable_parent_type")
    !22 = !{!23}
    !23 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !24, size: 64, align: 64, flags: DIFlagPublic)
    !24 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !25, size: 64, align: 64, dwarfAddressSpace: 1)
    !25 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !26 = !DIGlobalVariableExpression(var: !27, expr: !DIExpression())
    !27 = distinct !DIGlobalVariable(name: "__vtable_parent", scope: !2, file: !2, type: !21, isLocal: false, isDefinition: true)
    !28 = !DIGlobalVariableExpression(var: !29, expr: !DIExpression())
    !29 = distinct !DIGlobalVariable(name: "____vtable_child_type__init", scope: !2, file: !2, type: !30, isLocal: false, isDefinition: true)
    !30 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_child_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !31, identifier: "__vtable_child_type")
    !31 = !{!23, !32}
    !32 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_parent_type", scope: !2, file: !2, baseType: !21, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !33 = !DIGlobalVariableExpression(var: !34, expr: !DIExpression())
    !34 = distinct !DIGlobalVariable(name: "__vtable_child", scope: !2, file: !2, type: !30, isLocal: false, isDefinition: true)
    !35 = !DIGlobalVariableExpression(var: !36, expr: !DIExpression())
    !36 = distinct !DIGlobalVariable(name: "____vtable_grandchild_type__init", scope: !2, file: !2, type: !37, isLocal: false, isDefinition: true)
    !37 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_grandchild_type", scope: !2, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !38, identifier: "__vtable_grandchild_type")
    !38 = !{!23, !39}
    !39 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_child_type", scope: !2, file: !2, baseType: !30, size: 128, align: 64, offset: 64, flags: DIFlagPublic)
    !40 = !DIGlobalVariableExpression(var: !41, expr: !DIExpression())
    !41 = distinct !DIGlobalVariable(name: "__vtable_grandchild", scope: !2, file: !2, type: !37, isLocal: false, isDefinition: true)
    !42 = !{i32 2, !"Dwarf Version", i32 5}
    !43 = !{i32 2, !"Debug Info Version", i32 3}
    !44 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !45, splitDebugInlining: false)
    !45 = !{!26, !19, !33, !28, !40, !35, !17, !15, !0}
    !46 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 2, type: !47, scopeLine: 6, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !44, retainedNodes: !49)
    !47 = !DISubroutineType(flags: DIFlagPublic, types: !48)
    !48 = !{null, !9}
    !49 = !{}
    !50 = !DILocalVariable(name: "parent", scope: !46, file: !2, line: 6, type: !9)
    !51 = !DILocation(line: 6, scope: !46)
    !52 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 8, type: !53, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !44, retainedNodes: !49)
    !53 = !DISubroutineType(flags: DIFlagPublic, types: !54)
    !54 = !{null, !6}
    !55 = !DILocalVariable(name: "child", scope: !52, file: !2, line: 12, type: !6)
    !56 = !DILocation(line: 12, scope: !52)
    !57 = distinct !DISubprogram(name: "grandchild", linkageName: "grandchild", scope: !2, file: !2, line: 14, type: !58, scopeLine: 18, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !44, retainedNodes: !49)
    !58 = !DISubroutineType(flags: DIFlagPublic, types: !59)
    !59 = !{null, !3}
    !60 = !DILocalVariable(name: "grandchild", scope: !57, file: !2, line: 18, type: !3)
    !61 = !DILocation(line: 18, scope: !57)
    !62 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 20, type: !63, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !44, retainedNodes: !49)
    !63 = !DISubroutineType(flags: DIFlagPublic, types: !64)
    !64 = !{null}
    !65 = !DILocalVariable(name: "array_of_parent", scope: !62, file: !2, line: 22, type: !66, align: 64)
    !66 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 96, align: 64, elements: !67)
    !67 = !{!68}
    !68 = !DISubrange(count: 3, lowerBound: 0)
    !69 = !DILocation(line: 22, column: 4, scope: !62)
    !70 = !DILocalVariable(name: "array_of_child", scope: !62, file: !2, line: 23, type: !71, align: 64)
    !71 = !DICompositeType(tag: DW_TAG_array_type, baseType: !6, size: 192, align: 64, elements: !67)
    !72 = !DILocation(line: 23, column: 4, scope: !62)
    !73 = !DILocalVariable(name: "array_of_grandchild", scope: !62, file: !2, line: 24, type: !74, align: 64)
    !74 = !DICompositeType(tag: DW_TAG_array_type, baseType: !3, size: 288, align: 64, elements: !67)
    !75 = !DILocation(line: 24, column: 4, scope: !62)
    !76 = !DILocalVariable(name: "parent1", scope: !62, file: !2, line: 25, type: !9, align: 64)
    !77 = !DILocation(line: 25, column: 4, scope: !62)
    !78 = !DILocalVariable(name: "child1", scope: !62, file: !2, line: 26, type: !6, align: 64)
    !79 = !DILocation(line: 26, column: 4, scope: !62)
    !80 = !DILocalVariable(name: "grandchild1", scope: !62, file: !2, line: 27, type: !3, align: 64)
    !81 = !DILocation(line: 27, column: 4, scope: !62)
    !82 = !DILocalVariable(name: "main", scope: !62, file: !2, line: 20, type: !12, align: 32)
    !83 = !DILocation(line: 20, column: 9, scope: !62)
    !84 = !DILocation(line: 0, scope: !62)
    !85 = !DILocation(line: 30, column: 4, scope: !62)
    !86 = !DILocation(line: 31, column: 4, scope: !62)
    !87 = !DILocation(line: 32, column: 4, scope: !62)
    !88 = !DILocation(line: 33, column: 4, scope: !62)
    !89 = !DILocation(line: 34, column: 4, scope: !62)
    !90 = !DILocation(line: 35, column: 4, scope: !62)
    !91 = !DILocation(line: 37, column: 4, scope: !62)
    !92 = !DILocation(line: 38, column: 4, scope: !62)
    !93 = !DILocation(line: 39, column: 4, scope: !62)
    !94 = !DILocation(line: 40, column: 4, scope: !62)
    !95 = !DILocation(line: 41, column: 4, scope: !62)
    !96 = !DILocation(line: 42, column: 4, scope: !62)
    !97 = !DILocation(line: 43, column: 4, scope: !62)
    !98 = !DILocation(line: 44, column: 4, scope: !62)
    !99 = !DILocation(line: 45, column: 4, scope: !62)
    !100 = !DILocation(line: 46, column: 4, scope: !62)
    !101 = !DILocation(line: 47, column: 4, scope: !62)
    !102 = !DILocation(line: 48, column: 4, scope: !62)
    !103 = !DILocation(line: 49, column: 4, scope: !62)
    !104 = !DILocation(line: 50, column: 4, scope: !62)
    !105 = !DILocation(line: 51, column: 4, scope: !62)
    !106 = !DILocation(line: 52, column: 4, scope: !62)
    !107 = !DILocation(line: 53, column: 4, scope: !62)
    !108 = !DILocation(line: 54, column: 4, scope: !62)
    !109 = !DILocation(line: 56, scope: !62)
    "#);
}
