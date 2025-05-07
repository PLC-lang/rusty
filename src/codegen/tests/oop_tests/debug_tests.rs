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
    insta::assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { i32*, i16, [81 x i8], [11 x [81 x i8]] }
    %bar = type { %foo }
    %__vtable_foo_type = type { i32* }
    %__vtable_bar_type = type { i32*, %__vtable_foo_type }

    @__foo__init = constant %foo zeroinitializer, !dbg !0
    @__bar__init = constant %bar zeroinitializer, !dbg !19
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer, !dbg !24
    @__vtable_foo = global %__vtable_foo_type zeroinitializer, !dbg !29
    @____vtable_bar_type__init = constant %__vtable_bar_type zeroinitializer, !dbg !31
    @__vtable_bar = global %__vtable_bar_type zeroinitializer, !dbg !36

    define void @foo(%foo* %0) !dbg !42 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !46, metadata !DIExpression()), !dbg !47
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %a = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %b = getelementptr inbounds %foo, %foo* %0, i32 0, i32 2
      %c = getelementptr inbounds %foo, %foo* %0, i32 0, i32 3
      ret void, !dbg !47
    }

    define void @bar(%bar* %0) !dbg !48 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !51, metadata !DIExpression()), !dbg !52
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void, !dbg !52
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

    !llvm.module.flags = !{!38, !39}
    !llvm.dbg.cu = !{!40}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 7872, align: 64, flags: DIFlagPublic, elements: !4, identifier: "foo")
    !4 = !{!5, !8, !10, !15}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !6, size: 64, align: 64, flags: DIFlagPublic)
    !6 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !7, size: 64, align: 64, dwarfAddressSpace: 1)
    !7 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 4, baseType: !9, size: 16, align: 16, offset: 64, flags: DIFlagPublic)
    !9 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 5, baseType: !11, size: 648, align: 8, offset: 80, flags: DIFlagPublic)
    !11 = !DICompositeType(tag: DW_TAG_array_type, baseType: !12, size: 648, align: 8, elements: !13)
    !12 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !13 = !{!14}
    !14 = !DISubrange(count: 81, lowerBound: 0)
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 6, baseType: !16, size: 7128, align: 8, offset: 728, flags: DIFlagPublic)
    !16 = !DICompositeType(tag: DW_TAG_array_type, baseType: !11, size: 7128, align: 8, elements: !17)
    !17 = !{!18}
    !18 = !DISubrange(count: 11, lowerBound: 0)
    !19 = !DIGlobalVariableExpression(var: !20, expr: !DIExpression())
    !20 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 10, type: !21, isLocal: false, isDefinition: true)
    !21 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 10, size: 7872, align: 64, flags: DIFlagPublic, elements: !22, identifier: "bar")
    !22 = !{!23}
    !23 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !3, size: 7872, align: 64, flags: DIFlagPublic)
    !24 = !DIGlobalVariableExpression(var: !25, expr: !DIExpression())
    !25 = distinct !DIGlobalVariable(name: "____vtable_foo_type__init", scope: !2, file: !2, type: !26, isLocal: false, isDefinition: true)
    !26 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !27, identifier: "__vtable_foo_type")
    !27 = !{!28}
    !28 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !6, size: 64, align: 64, flags: DIFlagPublic)
    !29 = !DIGlobalVariableExpression(var: !30, expr: !DIExpression())
    !30 = distinct !DIGlobalVariable(name: "__vtable_foo", scope: !2, file: !2, type: !26, isLocal: false, isDefinition: true)
    !31 = !DIGlobalVariableExpression(var: !32, expr: !DIExpression())
    !32 = distinct !DIGlobalVariable(name: "____vtable_bar_type__init", scope: !2, file: !2, type: !33, isLocal: false, isDefinition: true)
    !33 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_bar_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !34, identifier: "__vtable_bar_type")
    !34 = !{!28, !35}
    !35 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_foo_type", scope: !2, file: !2, baseType: !26, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !36 = !DIGlobalVariableExpression(var: !37, expr: !DIExpression())
    !37 = distinct !DIGlobalVariable(name: "__vtable_bar", scope: !2, file: !2, type: !33, isLocal: false, isDefinition: true)
    !38 = !{i32 2, !"Dwarf Version", i32 5}
    !39 = !{i32 2, !"Debug Info Version", i32 3}
    !40 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !41, splitDebugInlining: false)
    !41 = !{!29, !24, !36, !31, !0, !19}
    !42 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !43, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !40, retainedNodes: !45)
    !43 = !DISubroutineType(flags: DIFlagPublic, types: !44)
    !44 = !{null, !3}
    !45 = !{}
    !46 = !DILocalVariable(name: "foo", scope: !42, file: !2, line: 8, type: !3)
    !47 = !DILocation(line: 8, column: 8, scope: !42)
    !48 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 10, type: !49, scopeLine: 11, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !40, retainedNodes: !45)
    !49 = !DISubroutineType(flags: DIFlagPublic, types: !50)
    !50 = !{null, !21}
    !51 = !DILocalVariable(name: "bar", scope: !48, file: !2, line: 11, type: !21)
    !52 = !DILocation(line: 11, column: 8, scope: !48)
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

    insta::assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %fb2 = type { %fb }
    %fb = type { i32*, i16, i16 }
    %foo = type { i32*, %fb2 }
    %__vtable_fb_type = type { i32* }
    %__vtable_fb2_type = type { i32*, %__vtable_fb_type }
    %__vtable_foo_type = type { i32* }

    @__fb2__init = constant %fb2 zeroinitializer, !dbg !0
    @__fb__init = constant %fb zeroinitializer, !dbg !14
    @__foo__init = constant %foo zeroinitializer, !dbg !16
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_fb_type__init = constant %__vtable_fb_type zeroinitializer, !dbg !21
    @__vtable_fb = global %__vtable_fb_type zeroinitializer, !dbg !26
    @____vtable_fb2_type__init = constant %__vtable_fb2_type zeroinitializer, !dbg !28
    @__vtable_fb2 = global %__vtable_fb2_type zeroinitializer, !dbg !33
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer, !dbg !35
    @__vtable_foo = global %__vtable_foo_type zeroinitializer, !dbg !38

    define void @fb(%fb* %0) !dbg !44 {
    entry:
      call void @llvm.dbg.declare(metadata %fb* %0, metadata !48, metadata !DIExpression()), !dbg !49
      %__vtable = getelementptr inbounds %fb, %fb* %0, i32 0, i32 0
      %x = getelementptr inbounds %fb, %fb* %0, i32 0, i32 1
      %y = getelementptr inbounds %fb, %fb* %0, i32 0, i32 2
      ret void, !dbg !49
    }

    define void @fb2(%fb2* %0) !dbg !50 {
    entry:
      call void @llvm.dbg.declare(metadata %fb2* %0, metadata !53, metadata !DIExpression()), !dbg !54
      %__fb = getelementptr inbounds %fb2, %fb2* %0, i32 0, i32 0
      ret void, !dbg !54
    }

    define void @foo(%foo* %0) !dbg !55 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !58, metadata !DIExpression()), !dbg !59
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %myFb = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %__fb = getelementptr inbounds %fb2, %fb2* %myFb, i32 0, i32 0, !dbg !59
      %x = getelementptr inbounds %fb, %fb* %__fb, i32 0, i32 1, !dbg !59
      store i16 1, i16* %x, align 2, !dbg !59
      ret void, !dbg !60
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

    !llvm.module.flags = !{!40, !41}
    !llvm.dbg.cu = !{!42}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__fb2__init", scope: !2, file: !2, line: 9, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb2", scope: !2, file: !2, line: 9, size: 128, align: 64, flags: DIFlagPublic, elements: !4, identifier: "fb2")
    !4 = !{!5}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__fb", scope: !2, file: !2, baseType: !6, size: 128, align: 64, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb", scope: !2, file: !2, line: 2, size: 128, align: 64, flags: DIFlagPublic, elements: !7, identifier: "fb")
    !7 = !{!8, !11, !13}
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !9, size: 64, align: 64, flags: DIFlagPublic)
    !9 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !10, size: 64, align: 64, dwarfAddressSpace: 1)
    !10 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 4, baseType: !12, size: 16, align: 16, offset: 64, flags: DIFlagPublic)
    !12 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !13 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 5, baseType: !12, size: 16, align: 16, offset: 80, flags: DIFlagPublic)
    !14 = !DIGlobalVariableExpression(var: !15, expr: !DIExpression())
    !15 = distinct !DIGlobalVariable(name: "__fb__init", scope: !2, file: !2, line: 2, type: !6, isLocal: false, isDefinition: true)
    !16 = !DIGlobalVariableExpression(var: !17, expr: !DIExpression())
    !17 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 12, type: !18, isLocal: false, isDefinition: true)
    !18 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 12, size: 192, align: 64, flags: DIFlagPublic, elements: !19, identifier: "foo")
    !19 = !{!8, !20}
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "myFb", scope: !2, file: !2, line: 14, baseType: !3, size: 128, align: 64, offset: 64, flags: DIFlagPublic)
    !21 = !DIGlobalVariableExpression(var: !22, expr: !DIExpression())
    !22 = distinct !DIGlobalVariable(name: "____vtable_fb_type__init", scope: !2, file: !2, type: !23, isLocal: false, isDefinition: true)
    !23 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_fb_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !24, identifier: "__vtable_fb_type")
    !24 = !{!25}
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !9, size: 64, align: 64, flags: DIFlagPublic)
    !26 = !DIGlobalVariableExpression(var: !27, expr: !DIExpression())
    !27 = distinct !DIGlobalVariable(name: "__vtable_fb", scope: !2, file: !2, type: !23, isLocal: false, isDefinition: true)
    !28 = !DIGlobalVariableExpression(var: !29, expr: !DIExpression())
    !29 = distinct !DIGlobalVariable(name: "____vtable_fb2_type__init", scope: !2, file: !2, type: !30, isLocal: false, isDefinition: true)
    !30 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_fb2_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !31, identifier: "__vtable_fb2_type")
    !31 = !{!25, !32}
    !32 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_fb_type", scope: !2, file: !2, baseType: !23, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !33 = !DIGlobalVariableExpression(var: !34, expr: !DIExpression())
    !34 = distinct !DIGlobalVariable(name: "__vtable_fb2", scope: !2, file: !2, type: !30, isLocal: false, isDefinition: true)
    !35 = !DIGlobalVariableExpression(var: !36, expr: !DIExpression())
    !36 = distinct !DIGlobalVariable(name: "____vtable_foo_type__init", scope: !2, file: !2, type: !37, isLocal: false, isDefinition: true)
    !37 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !24, identifier: "__vtable_foo_type")
    !38 = !DIGlobalVariableExpression(var: !39, expr: !DIExpression())
    !39 = distinct !DIGlobalVariable(name: "__vtable_foo", scope: !2, file: !2, type: !37, isLocal: false, isDefinition: true)
    !40 = !{i32 2, !"Dwarf Version", i32 5}
    !41 = !{i32 2, !"Debug Info Version", i32 3}
    !42 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !43, splitDebugInlining: false)
    !43 = !{!26, !21, !33, !28, !38, !35, !14, !0, !16}
    !44 = distinct !DISubprogram(name: "fb", linkageName: "fb", scope: !2, file: !2, line: 2, type: !45, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !42, retainedNodes: !47)
    !45 = !DISubroutineType(flags: DIFlagPublic, types: !46)
    !46 = !{null, !6}
    !47 = !{}
    !48 = !DILocalVariable(name: "fb", scope: !44, file: !2, line: 7, type: !6)
    !49 = !DILocation(line: 7, column: 8, scope: !44)
    !50 = distinct !DISubprogram(name: "fb2", linkageName: "fb2", scope: !2, file: !2, line: 9, type: !51, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !42, retainedNodes: !47)
    !51 = !DISubroutineType(flags: DIFlagPublic, types: !52)
    !52 = !{null, !3}
    !53 = !DILocalVariable(name: "fb2", scope: !50, file: !2, line: 10, type: !3)
    !54 = !DILocation(line: 10, column: 8, scope: !50)
    !55 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 12, type: !56, scopeLine: 16, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !42, retainedNodes: !47)
    !56 = !DISubroutineType(flags: DIFlagPublic, types: !57)
    !57 = !{null, !18}
    !58 = !DILocalVariable(name: "foo", scope: !55, file: !2, line: 16, type: !18)
    !59 = !DILocation(line: 16, column: 12, scope: !55)
    !60 = !DILocation(line: 17, column: 8, scope: !55)
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
    insta::assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %bar = type { %foo }
    %foo = type { i32*, [81 x i8] }
    %__vtable_foo_type = type { i32*, i32* }
    %__vtable_bar_type = type { i32*, %__vtable_foo_type }

    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"world\00"
    @__bar__init = constant %bar zeroinitializer, !dbg !0
    @__foo__init = constant %foo zeroinitializer, !dbg !16
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer, !dbg !18
    @__vtable_foo = global %__vtable_foo_type zeroinitializer, !dbg !24
    @____vtable_bar_type__init = constant %__vtable_bar_type zeroinitializer, !dbg !26
    @__vtable_bar = global %__vtable_bar_type zeroinitializer, !dbg !31

    define void @foo(%foo* %0) !dbg !37 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !41, metadata !DIExpression()), !dbg !42
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      ret void, !dbg !42
    }

    define void @foo_baz(%foo* %0) !dbg !43 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !44, metadata !DIExpression()), !dbg !45
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %1 = bitcast [81 x i8]* %s to i8*, !dbg !45
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_0, i32 0, i32 0), i32 6, i1 false), !dbg !45
      ret void, !dbg !46
    }

    define void @bar(%bar* %0) !dbg !47 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !50, metadata !DIExpression()), !dbg !51
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %__foo, i32 0, i32 1, !dbg !51
      %1 = bitcast [81 x i8]* %s to i8*, !dbg !51
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_1, i32 0, i32 0), i32 6, i1 false), !dbg !51
      ret void, !dbg !52
    }

    define void @main() !dbg !53 {
    entry:
      %s = alloca [81 x i8], align 1
      %fb = alloca %bar, align 8
      call void @llvm.dbg.declare(metadata [81 x i8]* %s, metadata !56, metadata !DIExpression()), !dbg !57
      %0 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %bar* %fb, metadata !58, metadata !DIExpression()), !dbg !59
      %1 = bitcast %bar* %fb to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 bitcast (%bar* @__bar__init to i8*), i64 ptrtoint (%bar* getelementptr (%bar, %bar* null, i32 1) to i64), i1 false)
      call void @__init_bar(%bar* %fb), !dbg !60
      call void @__user_init_bar(%bar* %fb), !dbg !60
      %__foo = getelementptr inbounds %bar, %bar* %fb, i32 0, i32 0, !dbg !60
      call void @foo_baz(%foo* %__foo), !dbg !61
      call void @bar(%bar* %fb), !dbg !62
      ret void, !dbg !63
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

    !llvm.module.flags = !{!33, !34}
    !llvm.dbg.cu = !{!35}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 11, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 11, size: 768, align: 64, flags: DIFlagPublic, elements: !4, identifier: "bar")
    !4 = !{!5}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !6, size: 768, align: 64, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 768, align: 64, flags: DIFlagPublic, elements: !7, identifier: "foo")
    !7 = !{!8, !11}
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !9, size: 64, align: 64, flags: DIFlagPublic)
    !9 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !10, size: 64, align: 64, dwarfAddressSpace: 1)
    !10 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !2, file: !2, line: 4, baseType: !12, size: 648, align: 8, offset: 64, flags: DIFlagPublic)
    !12 = !DICompositeType(tag: DW_TAG_array_type, baseType: !13, size: 648, align: 8, elements: !14)
    !13 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !14 = !{!15}
    !15 = !DISubrange(count: 81, lowerBound: 0)
    !16 = !DIGlobalVariableExpression(var: !17, expr: !DIExpression())
    !17 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !6, isLocal: false, isDefinition: true)
    !18 = !DIGlobalVariableExpression(var: !19, expr: !DIExpression())
    !19 = distinct !DIGlobalVariable(name: "____vtable_foo_type__init", scope: !2, file: !2, type: !20, isLocal: false, isDefinition: true)
    !20 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !21, identifier: "__vtable_foo_type")
    !21 = !{!22, !23}
    !22 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !9, size: 64, align: 64, flags: DIFlagPublic)
    !23 = !DIDerivedType(tag: DW_TAG_member, name: "foo.baz", scope: !2, file: !2, baseType: !9, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !24 = !DIGlobalVariableExpression(var: !25, expr: !DIExpression())
    !25 = distinct !DIGlobalVariable(name: "__vtable_foo", scope: !2, file: !2, type: !20, isLocal: false, isDefinition: true)
    !26 = !DIGlobalVariableExpression(var: !27, expr: !DIExpression())
    !27 = distinct !DIGlobalVariable(name: "____vtable_bar_type__init", scope: !2, file: !2, type: !28, isLocal: false, isDefinition: true)
    !28 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_bar_type", scope: !2, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !29, identifier: "__vtable_bar_type")
    !29 = !{!22, !30}
    !30 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_foo_type", scope: !2, file: !2, baseType: !20, size: 128, align: 64, offset: 64, flags: DIFlagPublic)
    !31 = !DIGlobalVariableExpression(var: !32, expr: !DIExpression())
    !32 = distinct !DIGlobalVariable(name: "__vtable_bar", scope: !2, file: !2, type: !28, isLocal: false, isDefinition: true)
    !33 = !{i32 2, !"Dwarf Version", i32 5}
    !34 = !{i32 2, !"Debug Info Version", i32 3}
    !35 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !36, splitDebugInlining: false)
    !36 = !{!24, !18, !31, !26, !16, !0}
    !37 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !38, scopeLine: 9, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !40)
    !38 = !DISubroutineType(flags: DIFlagPublic, types: !39)
    !39 = !{null, !6}
    !40 = !{}
    !41 = !DILocalVariable(name: "foo", scope: !37, file: !2, line: 9, type: !6)
    !42 = !DILocation(line: 9, column: 8, scope: !37)
    !43 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !37, file: !2, line: 6, type: !38, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !40)
    !44 = !DILocalVariable(name: "foo", scope: !43, file: !2, line: 7, type: !6)
    !45 = !DILocation(line: 7, column: 12, scope: !43)
    !46 = !DILocation(line: 8, column: 8, scope: !43)
    !47 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 11, type: !48, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !40)
    !48 = !DISubroutineType(flags: DIFlagPublic, types: !49)
    !49 = !{null, !3}
    !50 = !DILocalVariable(name: "bar", scope: !47, file: !2, line: 12, type: !3)
    !51 = !DILocation(line: 12, column: 12, scope: !47)
    !52 = !DILocation(line: 13, column: 8, scope: !47)
    !53 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 15, type: !54, scopeLine: 15, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !35, retainedNodes: !40)
    !54 = !DISubroutineType(flags: DIFlagPublic, types: !55)
    !55 = !{null}
    !56 = !DILocalVariable(name: "s", scope: !53, file: !2, line: 17, type: !12, align: 8)
    !57 = !DILocation(line: 17, column: 12, scope: !53)
    !58 = !DILocalVariable(name: "fb", scope: !53, file: !2, line: 18, type: !3, align: 64)
    !59 = !DILocation(line: 18, column: 12, scope: !53)
    !60 = !DILocation(line: 0, scope: !53)
    !61 = !DILocation(line: 20, column: 12, scope: !53)
    !62 = !DILocation(line: 21, column: 12, scope: !53)
    !63 = !DILocation(line: 22, column: 8, scope: !53)
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
    insta::assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %child = type { %parent, [11 x i16] }
    %parent = type { %grandparent, [11 x i16], i16 }
    %grandparent = type { i32*, [6 x i16], i16 }
    %__vtable_grandparent_type = type { i32* }
    %__vtable_parent_type = type { i32*, %__vtable_grandparent_type }
    %__vtable_child_type = type { i32*, %__vtable_parent_type }

    @__child__init = constant %child zeroinitializer, !dbg !0
    @__parent__init = constant %parent zeroinitializer, !dbg !26
    @__grandparent__init = constant %grandparent zeroinitializer, !dbg !28
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_grandparent_type__init = constant %__vtable_grandparent_type zeroinitializer, !dbg !30
    @__vtable_grandparent = global %__vtable_grandparent_type zeroinitializer, !dbg !35
    @____vtable_parent_type__init = constant %__vtable_parent_type zeroinitializer, !dbg !37
    @__vtable_parent = global %__vtable_parent_type zeroinitializer, !dbg !42
    @____vtable_child_type__init = constant %__vtable_child_type zeroinitializer, !dbg !44
    @__vtable_child = global %__vtable_child_type zeroinitializer, !dbg !49

    define void @grandparent(%grandparent* %0) !dbg !55 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !59, metadata !DIExpression()), !dbg !60
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 2
      ret void, !dbg !60
    }

    define void @parent(%parent* %0) !dbg !61 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !64, metadata !DIExpression()), !dbg !65
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void, !dbg !65
    }

    define void @child(%child* %0) !dbg !66 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !69, metadata !DIExpression()), !dbg !70
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      ret void, !dbg !70
    }

    define void @main() !dbg !71 {
    entry:
      %arr = alloca [11 x %child], align 8
      call void @llvm.dbg.declare(metadata [11 x %child]* %arr, metadata !74, metadata !DIExpression()), !dbg !76
      %0 = bitcast [11 x %child]* %arr to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([11 x %child]* getelementptr ([11 x %child], [11 x %child]* null, i32 1) to i64), i1 false)
      %tmpVar = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !77
      %__parent = getelementptr inbounds %child, %child* %tmpVar, i32 0, i32 0, !dbg !77
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !77
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 2, !dbg !77
      store i16 10, i16* %a, align 2, !dbg !77
      %tmpVar1 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !78
      %__parent2 = getelementptr inbounds %child, %child* %tmpVar1, i32 0, i32 0, !dbg !78
      %__grandparent3 = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0, !dbg !78
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent3, i32 0, i32 1, !dbg !78
      %tmpVar4 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 0, !dbg !78
      store i16 20, i16* %tmpVar4, align 2, !dbg !78
      %tmpVar5 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !79
      %__parent6 = getelementptr inbounds %child, %child* %tmpVar5, i32 0, i32 0, !dbg !79
      %b = getelementptr inbounds %parent, %parent* %__parent6, i32 0, i32 2, !dbg !79
      store i16 30, i16* %b, align 2, !dbg !79
      %tmpVar7 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !80
      %__parent8 = getelementptr inbounds %child, %child* %tmpVar7, i32 0, i32 0, !dbg !80
      %x = getelementptr inbounds %parent, %parent* %__parent8, i32 0, i32 1, !dbg !80
      %tmpVar9 = getelementptr inbounds [11 x i16], [11 x i16]* %x, i32 0, i32 1, !dbg !80
      store i16 40, i16* %tmpVar9, align 2, !dbg !80
      %tmpVar10 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 2, !dbg !81
      %z = getelementptr inbounds %child, %child* %tmpVar10, i32 0, i32 1, !dbg !81
      %tmpVar11 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 2, !dbg !81
      store i16 50, i16* %tmpVar11, align 2, !dbg !81
      ret void, !dbg !82
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

    !llvm.module.flags = !{!51, !52}
    !llvm.dbg.cu = !{!53}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 576, align: 64, flags: DIFlagPublic, elements: !4, identifier: "child")
    !4 = !{!5, !25}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !6, size: 384, align: 64, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 384, align: 64, flags: DIFlagPublic, elements: !7, identifier: "parent")
    !7 = !{!8, !20, !24}
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "__grandparent", scope: !2, file: !2, baseType: !9, size: 192, align: 64, flags: DIFlagPublic)
    !9 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 192, align: 64, flags: DIFlagPublic, elements: !10, identifier: "grandparent")
    !10 = !{!11, !14, !19}
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !12, size: 64, align: 64, flags: DIFlagPublic)
    !12 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !13, size: 64, align: 64, dwarfAddressSpace: 1)
    !13 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !15, size: 96, align: 16, offset: 64, flags: DIFlagPublic)
    !15 = !DICompositeType(tag: DW_TAG_array_type, baseType: !16, size: 96, align: 16, elements: !17)
    !16 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !17 = !{!18}
    !18 = !DISubrange(count: 6, lowerBound: 0)
    !19 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !16, size: 16, align: 16, offset: 160, flags: DIFlagPublic)
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !21, size: 176, align: 16, offset: 192, flags: DIFlagPublic)
    !21 = !DICompositeType(tag: DW_TAG_array_type, baseType: !16, size: 176, align: 16, elements: !22)
    !22 = !{!23}
    !23 = !DISubrange(count: 11, lowerBound: 0)
    !24 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !16, size: 16, align: 16, offset: 368, flags: DIFlagPublic)
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !21, size: 176, align: 16, offset: 384, flags: DIFlagPublic)
    !26 = !DIGlobalVariableExpression(var: !27, expr: !DIExpression())
    !27 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !6, isLocal: false, isDefinition: true)
    !28 = !DIGlobalVariableExpression(var: !29, expr: !DIExpression())
    !29 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !9, isLocal: false, isDefinition: true)
    !30 = !DIGlobalVariableExpression(var: !31, expr: !DIExpression())
    !31 = distinct !DIGlobalVariable(name: "____vtable_grandparent_type__init", scope: !2, file: !2, type: !32, isLocal: false, isDefinition: true)
    !32 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_grandparent_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !33, identifier: "__vtable_grandparent_type")
    !33 = !{!34}
    !34 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !12, size: 64, align: 64, flags: DIFlagPublic)
    !35 = !DIGlobalVariableExpression(var: !36, expr: !DIExpression())
    !36 = distinct !DIGlobalVariable(name: "__vtable_grandparent", scope: !2, file: !2, type: !32, isLocal: false, isDefinition: true)
    !37 = !DIGlobalVariableExpression(var: !38, expr: !DIExpression())
    !38 = distinct !DIGlobalVariable(name: "____vtable_parent_type__init", scope: !2, file: !2, type: !39, isLocal: false, isDefinition: true)
    !39 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_parent_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !40, identifier: "__vtable_parent_type")
    !40 = !{!34, !41}
    !41 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_grandparent_type", scope: !2, file: !2, baseType: !32, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !42 = !DIGlobalVariableExpression(var: !43, expr: !DIExpression())
    !43 = distinct !DIGlobalVariable(name: "__vtable_parent", scope: !2, file: !2, type: !39, isLocal: false, isDefinition: true)
    !44 = !DIGlobalVariableExpression(var: !45, expr: !DIExpression())
    !45 = distinct !DIGlobalVariable(name: "____vtable_child_type__init", scope: !2, file: !2, type: !46, isLocal: false, isDefinition: true)
    !46 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_child_type", scope: !2, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !47, identifier: "__vtable_child_type")
    !47 = !{!34, !48}
    !48 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_parent_type", scope: !2, file: !2, baseType: !39, size: 128, align: 64, offset: 64, flags: DIFlagPublic)
    !49 = !DIGlobalVariableExpression(var: !50, expr: !DIExpression())
    !50 = distinct !DIGlobalVariable(name: "__vtable_child", scope: !2, file: !2, type: !46, isLocal: false, isDefinition: true)
    !51 = !{i32 2, !"Dwarf Version", i32 5}
    !52 = !{i32 2, !"Debug Info Version", i32 3}
    !53 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !54, splitDebugInlining: false)
    !54 = !{!35, !30, !42, !37, !49, !44, !28, !26, !0}
    !55 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !56, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !53, retainedNodes: !58)
    !56 = !DISubroutineType(flags: DIFlagPublic, types: !57)
    !57 = !{null, !9}
    !58 = !{}
    !59 = !DILocalVariable(name: "grandparent", scope: !55, file: !2, line: 7, type: !9)
    !60 = !DILocation(line: 7, column: 8, scope: !55)
    !61 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !62, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !53, retainedNodes: !58)
    !62 = !DISubroutineType(flags: DIFlagPublic, types: !63)
    !63 = !{null, !6}
    !64 = !DILocalVariable(name: "parent", scope: !61, file: !2, line: 14, type: !6)
    !65 = !DILocation(line: 14, column: 8, scope: !61)
    !66 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !67, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !53, retainedNodes: !58)
    !67 = !DISubroutineType(flags: DIFlagPublic, types: !68)
    !68 = !{null, !3}
    !69 = !DILocalVariable(name: "child", scope: !66, file: !2, line: 20, type: !3)
    !70 = !DILocation(line: 20, column: 8, scope: !66)
    !71 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 22, type: !72, scopeLine: 26, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !53, retainedNodes: !58)
    !72 = !DISubroutineType(flags: DIFlagPublic, types: !73)
    !73 = !{null}
    !74 = !DILocalVariable(name: "arr", scope: !71, file: !2, line: 24, type: !75, align: 64)
    !75 = !DICompositeType(tag: DW_TAG_array_type, baseType: !3, size: 6336, align: 64, elements: !22)
    !76 = !DILocation(line: 24, column: 12, scope: !71)
    !77 = !DILocation(line: 26, column: 12, scope: !71)
    !78 = !DILocation(line: 27, column: 12, scope: !71)
    !79 = !DILocation(line: 28, column: 12, scope: !71)
    !80 = !DILocation(line: 29, column: 12, scope: !71)
    !81 = !DILocation(line: 30, column: 12, scope: !71)
    !82 = !DILocation(line: 31, column: 8, scope: !71)
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

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %parent = type { %grandparent, [11 x i16], i16 }
    %grandparent = type { i32*, [6 x i16], i16 }
    %child = type { %parent, [11 x i16] }
    %__vtable_grandparent_type = type { i32* }
    %__vtable_parent_type = type { i32*, %__vtable_grandparent_type }
    %__vtable_child_type = type { i32*, %__vtable_parent_type }

    @__parent__init = constant %parent zeroinitializer, !dbg !0
    @__grandparent__init = constant %grandparent zeroinitializer, !dbg !22
    @__child__init = constant %child zeroinitializer, !dbg !24
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_grandparent_type__init = constant %__vtable_grandparent_type zeroinitializer, !dbg !30
    @__vtable_grandparent = global %__vtable_grandparent_type zeroinitializer, !dbg !35
    @____vtable_parent_type__init = constant %__vtable_parent_type zeroinitializer, !dbg !37
    @__vtable_parent = global %__vtable_parent_type zeroinitializer, !dbg !42
    @____vtable_child_type__init = constant %__vtable_child_type zeroinitializer, !dbg !44
    @__vtable_child = global %__vtable_child_type zeroinitializer, !dbg !49

    define void @grandparent(%grandparent* %0) !dbg !55 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !59, metadata !DIExpression()), !dbg !60
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 2
      ret void, !dbg !60
    }

    define void @parent(%parent* %0) !dbg !61 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !64, metadata !DIExpression()), !dbg !65
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void, !dbg !65
    }

    define void @child(%child* %0) !dbg !66 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !69, metadata !DIExpression()), !dbg !70
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !70
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 1, !dbg !70
      %b = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2, !dbg !70
      %load_b = load i16, i16* %b, align 2, !dbg !70
      %1 = sext i16 %load_b to i32, !dbg !70
      %b1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2, !dbg !70
      %load_b2 = load i16, i16* %b1, align 2, !dbg !70
      %2 = sext i16 %load_b2 to i32, !dbg !70
      %tmpVar = mul i32 %2, 2, !dbg !70
      %tmpVar3 = mul i32 1, %tmpVar, !dbg !70
      %tmpVar4 = add i32 %tmpVar3, 0, !dbg !70
      %tmpVar5 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 %tmpVar4, !dbg !70
      %load_tmpVar = load i16, i16* %tmpVar5, align 2, !dbg !70
      %3 = sext i16 %load_tmpVar to i32, !dbg !70
      %tmpVar6 = add i32 %1, %3, !dbg !70
      %__grandparent7 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0, !dbg !70
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent7, i32 0, i32 2, !dbg !70
      %load_a = load i16, i16* %a, align 2, !dbg !70
      %4 = sext i16 %load_a to i32, !dbg !70
      %tmpVar8 = sub i32 %tmpVar6, %4, !dbg !70
      %tmpVar9 = mul i32 1, %tmpVar8, !dbg !70
      %tmpVar10 = add i32 %tmpVar9, 0, !dbg !70
      %tmpVar11 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 %tmpVar10, !dbg !70
      store i16 20, i16* %tmpVar11, align 2, !dbg !70
      ret void, !dbg !71
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

    !llvm.module.flags = !{!51, !52}
    !llvm.dbg.cu = !{!53}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 384, align: 64, flags: DIFlagPublic, elements: !4, identifier: "parent")
    !4 = !{!5, !17, !21}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__grandparent", scope: !2, file: !2, baseType: !6, size: 192, align: 64, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 192, align: 64, flags: DIFlagPublic, elements: !7, identifier: "grandparent")
    !7 = !{!8, !11, !16}
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !9, size: 64, align: 64, flags: DIFlagPublic)
    !9 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !10, size: 64, align: 64, dwarfAddressSpace: 1)
    !10 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !12, size: 96, align: 16, offset: 64, flags: DIFlagPublic)
    !12 = !DICompositeType(tag: DW_TAG_array_type, baseType: !13, size: 96, align: 16, elements: !14)
    !13 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !14 = !{!15}
    !15 = !DISubrange(count: 6, lowerBound: 0)
    !16 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !13, size: 16, align: 16, offset: 160, flags: DIFlagPublic)
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !18, size: 176, align: 16, offset: 192, flags: DIFlagPublic)
    !18 = !DICompositeType(tag: DW_TAG_array_type, baseType: !13, size: 176, align: 16, elements: !19)
    !19 = !{!20}
    !20 = !DISubrange(count: 11, lowerBound: 0)
    !21 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !13, size: 16, align: 16, offset: 368, flags: DIFlagPublic)
    !22 = !DIGlobalVariableExpression(var: !23, expr: !DIExpression())
    !23 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !6, isLocal: false, isDefinition: true)
    !24 = !DIGlobalVariableExpression(var: !25, expr: !DIExpression())
    !25 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !26, isLocal: false, isDefinition: true)
    !26 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 576, align: 64, flags: DIFlagPublic, elements: !27, identifier: "child")
    !27 = !{!28, !29}
    !28 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !3, size: 384, align: 64, flags: DIFlagPublic)
    !29 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !18, size: 176, align: 16, offset: 384, flags: DIFlagPublic)
    !30 = !DIGlobalVariableExpression(var: !31, expr: !DIExpression())
    !31 = distinct !DIGlobalVariable(name: "____vtable_grandparent_type__init", scope: !2, file: !2, type: !32, isLocal: false, isDefinition: true)
    !32 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_grandparent_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !33, identifier: "__vtable_grandparent_type")
    !33 = !{!34}
    !34 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !9, size: 64, align: 64, flags: DIFlagPublic)
    !35 = !DIGlobalVariableExpression(var: !36, expr: !DIExpression())
    !36 = distinct !DIGlobalVariable(name: "__vtable_grandparent", scope: !2, file: !2, type: !32, isLocal: false, isDefinition: true)
    !37 = !DIGlobalVariableExpression(var: !38, expr: !DIExpression())
    !38 = distinct !DIGlobalVariable(name: "____vtable_parent_type__init", scope: !2, file: !2, type: !39, isLocal: false, isDefinition: true)
    !39 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_parent_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !40, identifier: "__vtable_parent_type")
    !40 = !{!34, !41}
    !41 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_grandparent_type", scope: !2, file: !2, baseType: !32, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !42 = !DIGlobalVariableExpression(var: !43, expr: !DIExpression())
    !43 = distinct !DIGlobalVariable(name: "__vtable_parent", scope: !2, file: !2, type: !39, isLocal: false, isDefinition: true)
    !44 = !DIGlobalVariableExpression(var: !45, expr: !DIExpression())
    !45 = distinct !DIGlobalVariable(name: "____vtable_child_type__init", scope: !2, file: !2, type: !46, isLocal: false, isDefinition: true)
    !46 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_child_type", scope: !2, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !47, identifier: "__vtable_child_type")
    !47 = !{!34, !48}
    !48 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_parent_type", scope: !2, file: !2, baseType: !39, size: 128, align: 64, offset: 64, flags: DIFlagPublic)
    !49 = !DIGlobalVariableExpression(var: !50, expr: !DIExpression())
    !50 = distinct !DIGlobalVariable(name: "__vtable_child", scope: !2, file: !2, type: !46, isLocal: false, isDefinition: true)
    !51 = !{i32 2, !"Dwarf Version", i32 5}
    !52 = !{i32 2, !"Debug Info Version", i32 3}
    !53 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !54, splitDebugInlining: false)
    !54 = !{!35, !30, !42, !37, !49, !44, !22, !0, !24}
    !55 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !56, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !53, retainedNodes: !58)
    !56 = !DISubroutineType(flags: DIFlagPublic, types: !57)
    !57 = !{null, !6}
    !58 = !{}
    !59 = !DILocalVariable(name: "grandparent", scope: !55, file: !2, line: 7, type: !6)
    !60 = !DILocation(line: 7, column: 8, scope: !55)
    !61 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !62, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !53, retainedNodes: !58)
    !62 = !DISubroutineType(flags: DIFlagPublic, types: !63)
    !63 = !{null, !3}
    !64 = !DILocalVariable(name: "parent", scope: !61, file: !2, line: 14, type: !3)
    !65 = !DILocation(line: 14, column: 8, scope: !61)
    !66 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !67, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !53, retainedNodes: !58)
    !67 = !DISubroutineType(flags: DIFlagPublic, types: !68)
    !68 = !{null, !26}
    !69 = !DILocalVariable(name: "child", scope: !66, file: !2, line: 20, type: !26)
    !70 = !DILocation(line: 20, column: 12, scope: !66)
    !71 = !DILocation(line: 21, column: 8, scope: !66)
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
    insta::assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { i32* }
    %bar = type { %foo }
    %__vtable_foo_type = type { i32*, i32* }
    %__vtable_bar_type = type { i32*, %__vtable_foo_type }

    @__foo__init = constant %foo zeroinitializer, !dbg !0
    @__bar__init = constant %bar zeroinitializer, !dbg !8
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer, !dbg !13
    @__vtable_foo = global %__vtable_foo_type zeroinitializer, !dbg !19
    @____vtable_bar_type__init = constant %__vtable_bar_type zeroinitializer, !dbg !21
    @__vtable_bar = global %__vtable_bar_type zeroinitializer, !dbg !26

    define void @foo(%foo* %0) !dbg !32 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !36, metadata !DIExpression()), !dbg !37
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void, !dbg !37
    }

    define void @foo_baz(%foo* %0) !dbg !38 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !39, metadata !DIExpression()), !dbg !40
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void, !dbg !40
    }

    define void @bar(%bar* %0) !dbg !41 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !44, metadata !DIExpression()), !dbg !45
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void, !dbg !45
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

    !llvm.module.flags = !{!28, !29}
    !llvm.dbg.cu = !{!30}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 64, align: 64, flags: DIFlagPublic, elements: !4, identifier: "foo")
    !4 = !{!5}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !6, size: 64, align: 64, flags: DIFlagPublic)
    !6 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !7, size: 64, align: 64, dwarfAddressSpace: 1)
    !7 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !8 = !DIGlobalVariableExpression(var: !9, expr: !DIExpression())
    !9 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 7, type: !10, isLocal: false, isDefinition: true)
    !10 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 7, size: 64, align: 64, flags: DIFlagPublic, elements: !11, identifier: "bar")
    !11 = !{!12}
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "__foo", scope: !2, file: !2, baseType: !3, size: 64, align: 64, flags: DIFlagPublic)
    !13 = !DIGlobalVariableExpression(var: !14, expr: !DIExpression())
    !14 = distinct !DIGlobalVariable(name: "____vtable_foo_type__init", scope: !2, file: !2, type: !15, isLocal: false, isDefinition: true)
    !15 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !16, identifier: "__vtable_foo_type")
    !16 = !{!17, !18}
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !6, size: 64, align: 64, flags: DIFlagPublic)
    !18 = !DIDerivedType(tag: DW_TAG_member, name: "foo.baz", scope: !2, file: !2, baseType: !6, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !19 = !DIGlobalVariableExpression(var: !20, expr: !DIExpression())
    !20 = distinct !DIGlobalVariable(name: "__vtable_foo", scope: !2, file: !2, type: !15, isLocal: false, isDefinition: true)
    !21 = !DIGlobalVariableExpression(var: !22, expr: !DIExpression())
    !22 = distinct !DIGlobalVariable(name: "____vtable_bar_type__init", scope: !2, file: !2, type: !23, isLocal: false, isDefinition: true)
    !23 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_bar_type", scope: !2, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !24, identifier: "__vtable_bar_type")
    !24 = !{!17, !25}
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_foo_type", scope: !2, file: !2, baseType: !15, size: 128, align: 64, offset: 64, flags: DIFlagPublic)
    !26 = !DIGlobalVariableExpression(var: !27, expr: !DIExpression())
    !27 = distinct !DIGlobalVariable(name: "__vtable_bar", scope: !2, file: !2, type: !23, isLocal: false, isDefinition: true)
    !28 = !{i32 2, !"Dwarf Version", i32 5}
    !29 = !{i32 2, !"Debug Info Version", i32 3}
    !30 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !31, splitDebugInlining: false)
    !31 = !{!19, !13, !26, !21, !0, !8}
    !32 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !33, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !30, retainedNodes: !35)
    !33 = !DISubroutineType(flags: DIFlagPublic, types: !34)
    !34 = !{null, !3}
    !35 = !{}
    !36 = !DILocalVariable(name: "foo", scope: !32, file: !2, line: 5, type: !3)
    !37 = !DILocation(line: 5, column: 8, scope: !32)
    !38 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !32, file: !2, line: 3, type: !33, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !30, retainedNodes: !35)
    !39 = !DILocalVariable(name: "foo", scope: !38, file: !2, line: 4, type: !3)
    !40 = !DILocation(line: 4, column: 8, scope: !38)
    !41 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 7, type: !42, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !30, retainedNodes: !35)
    !42 = !DISubroutineType(flags: DIFlagPublic, types: !43)
    !43 = !{null, !10}
    !44 = !DILocalVariable(name: "bar", scope: !41, file: !2, line: 8, type: !10)
    !45 = !DILocation(line: 8, column: 8, scope: !41)
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

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %grandchild = type { %child, i32 }
    %child = type { %parent, i32 }
    %parent = type { i32*, i32 }
    %__vtable_parent_type = type { i32* }
    %__vtable_child_type = type { i32*, %__vtable_parent_type }
    %__vtable_grandchild_type = type { i32*, %__vtable_child_type }

    @__grandchild__init = constant %grandchild zeroinitializer, !dbg !0
    @__child__init = constant %child zeroinitializer, !dbg !18
    @__parent__init = constant %parent zeroinitializer, !dbg !20
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_parent_type__init = constant %__vtable_parent_type zeroinitializer, !dbg !22
    @__vtable_parent = global %__vtable_parent_type zeroinitializer, !dbg !27
    @____vtable_child_type__init = constant %__vtable_child_type zeroinitializer, !dbg !29
    @__vtable_child = global %__vtable_child_type zeroinitializer, !dbg !34
    @____vtable_grandchild_type__init = constant %__vtable_grandchild_type zeroinitializer, !dbg !36
    @__vtable_grandchild = global %__vtable_grandchild_type zeroinitializer, !dbg !41

    define void @parent(%parent* %0) !dbg !47 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !51, metadata !DIExpression()), !dbg !52
      %__vtable = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %a = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      ret void, !dbg !52
    }

    define void @child(%child* %0) !dbg !53 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !56, metadata !DIExpression()), !dbg !57
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %b = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      ret void, !dbg !57
    }

    define void @grandchild(%grandchild* %0) !dbg !58 {
    entry:
      call void @llvm.dbg.declare(metadata %grandchild* %0, metadata !61, metadata !DIExpression()), !dbg !62
      %__child = getelementptr inbounds %grandchild, %grandchild* %0, i32 0, i32 0
      %c = getelementptr inbounds %grandchild, %grandchild* %0, i32 0, i32 1
      ret void, !dbg !62
    }

    define i32 @main() !dbg !63 {
    entry:
      %main = alloca i32, align 4
      %array_of_parent = alloca [3 x %parent], align 8
      %array_of_child = alloca [3 x %child], align 8
      %array_of_grandchild = alloca [3 x %grandchild], align 8
      %parent1 = alloca %parent, align 8
      %child1 = alloca %child, align 8
      %grandchild1 = alloca %grandchild, align 8
      call void @llvm.dbg.declare(metadata [3 x %parent]* %array_of_parent, metadata !66, metadata !DIExpression()), !dbg !70
      %0 = bitcast [3 x %parent]* %array_of_parent to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([3 x %parent]* getelementptr ([3 x %parent], [3 x %parent]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata [3 x %child]* %array_of_child, metadata !71, metadata !DIExpression()), !dbg !73
      %1 = bitcast [3 x %child]* %array_of_child to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %1, i8 0, i64 ptrtoint ([3 x %child]* getelementptr ([3 x %child], [3 x %child]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata [3 x %grandchild]* %array_of_grandchild, metadata !74, metadata !DIExpression()), !dbg !76
      %2 = bitcast [3 x %grandchild]* %array_of_grandchild to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %2, i8 0, i64 ptrtoint ([3 x %grandchild]* getelementptr ([3 x %grandchild], [3 x %grandchild]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %parent* %parent1, metadata !77, metadata !DIExpression()), !dbg !78
      %3 = bitcast %parent* %parent1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %3, i8* align 1 bitcast (%parent* @__parent__init to i8*), i64 ptrtoint (%parent* getelementptr (%parent, %parent* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %child* %child1, metadata !79, metadata !DIExpression()), !dbg !80
      %4 = bitcast %child* %child1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %4, i8* align 1 bitcast (%child* @__child__init to i8*), i64 ptrtoint (%child* getelementptr (%child, %child* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata %grandchild* %grandchild1, metadata !81, metadata !DIExpression()), !dbg !82
      %5 = bitcast %grandchild* %grandchild1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %5, i8* align 1 bitcast (%grandchild* @__grandchild__init to i8*), i64 ptrtoint (%grandchild* getelementptr (%grandchild, %grandchild* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata i32* %main, metadata !83, metadata !DIExpression()), !dbg !84
      store i32 0, i32* %main, align 4
      call void @__init_parent(%parent* %parent1), !dbg !85
      call void @__init_child(%child* %child1), !dbg !85
      call void @__init_grandchild(%grandchild* %grandchild1), !dbg !85
      call void @__user_init_parent(%parent* %parent1), !dbg !85
      call void @__user_init_child(%child* %child1), !dbg !85
      call void @__user_init_grandchild(%grandchild* %grandchild1), !dbg !85
      %a = getelementptr inbounds %parent, %parent* %parent1, i32 0, i32 1, !dbg !86
      store i32 1, i32* %a, align 4, !dbg !86
      %__parent = getelementptr inbounds %child, %child* %child1, i32 0, i32 0, !dbg !87
      %a1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 1, !dbg !87
      store i32 2, i32* %a1, align 4, !dbg !87
      %b = getelementptr inbounds %child, %child* %child1, i32 0, i32 1, !dbg !88
      store i32 3, i32* %b, align 4, !dbg !88
      %__child = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 0, !dbg !89
      %__parent2 = getelementptr inbounds %child, %child* %__child, i32 0, i32 0, !dbg !89
      %a3 = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 1, !dbg !89
      store i32 4, i32* %a3, align 4, !dbg !89
      %__child4 = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 0, !dbg !90
      %b5 = getelementptr inbounds %child, %child* %__child4, i32 0, i32 1, !dbg !90
      store i32 5, i32* %b5, align 4, !dbg !90
      %c = getelementptr inbounds %grandchild, %grandchild* %grandchild1, i32 0, i32 1, !dbg !91
      store i32 6, i32* %c, align 4, !dbg !91
      %tmpVar = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 0, !dbg !92
      %a6 = getelementptr inbounds %parent, %parent* %tmpVar, i32 0, i32 1, !dbg !92
      store i32 7, i32* %a6, align 4, !dbg !92
      %tmpVar7 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 0, !dbg !93
      %__parent8 = getelementptr inbounds %child, %child* %tmpVar7, i32 0, i32 0, !dbg !93
      %a9 = getelementptr inbounds %parent, %parent* %__parent8, i32 0, i32 1, !dbg !93
      store i32 8, i32* %a9, align 4, !dbg !93
      %tmpVar10 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 0, !dbg !94
      %b11 = getelementptr inbounds %child, %child* %tmpVar10, i32 0, i32 1, !dbg !94
      store i32 9, i32* %b11, align 4, !dbg !94
      %tmpVar12 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !95
      %__child13 = getelementptr inbounds %grandchild, %grandchild* %tmpVar12, i32 0, i32 0, !dbg !95
      %__parent14 = getelementptr inbounds %child, %child* %__child13, i32 0, i32 0, !dbg !95
      %a15 = getelementptr inbounds %parent, %parent* %__parent14, i32 0, i32 1, !dbg !95
      store i32 10, i32* %a15, align 4, !dbg !95
      %tmpVar16 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !96
      %__child17 = getelementptr inbounds %grandchild, %grandchild* %tmpVar16, i32 0, i32 0, !dbg !96
      %b18 = getelementptr inbounds %child, %child* %__child17, i32 0, i32 1, !dbg !96
      store i32 11, i32* %b18, align 4, !dbg !96
      %tmpVar19 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 0, !dbg !97
      %c20 = getelementptr inbounds %grandchild, %grandchild* %tmpVar19, i32 0, i32 1, !dbg !97
      store i32 12, i32* %c20, align 4, !dbg !97
      %tmpVar21 = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 1, !dbg !98
      %a22 = getelementptr inbounds %parent, %parent* %tmpVar21, i32 0, i32 1, !dbg !98
      store i32 13, i32* %a22, align 4, !dbg !98
      %tmpVar23 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 1, !dbg !99
      %__parent24 = getelementptr inbounds %child, %child* %tmpVar23, i32 0, i32 0, !dbg !99
      %a25 = getelementptr inbounds %parent, %parent* %__parent24, i32 0, i32 1, !dbg !99
      store i32 14, i32* %a25, align 4, !dbg !99
      %tmpVar26 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 1, !dbg !100
      %b27 = getelementptr inbounds %child, %child* %tmpVar26, i32 0, i32 1, !dbg !100
      store i32 15, i32* %b27, align 4, !dbg !100
      %tmpVar28 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !101
      %__child29 = getelementptr inbounds %grandchild, %grandchild* %tmpVar28, i32 0, i32 0, !dbg !101
      %__parent30 = getelementptr inbounds %child, %child* %__child29, i32 0, i32 0, !dbg !101
      %a31 = getelementptr inbounds %parent, %parent* %__parent30, i32 0, i32 1, !dbg !101
      store i32 16, i32* %a31, align 4, !dbg !101
      %tmpVar32 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !102
      %__child33 = getelementptr inbounds %grandchild, %grandchild* %tmpVar32, i32 0, i32 0, !dbg !102
      %b34 = getelementptr inbounds %child, %child* %__child33, i32 0, i32 1, !dbg !102
      store i32 17, i32* %b34, align 4, !dbg !102
      %tmpVar35 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 1, !dbg !103
      %c36 = getelementptr inbounds %grandchild, %grandchild* %tmpVar35, i32 0, i32 1, !dbg !103
      store i32 18, i32* %c36, align 4, !dbg !103
      %tmpVar37 = getelementptr inbounds [3 x %parent], [3 x %parent]* %array_of_parent, i32 0, i32 2, !dbg !104
      %a38 = getelementptr inbounds %parent, %parent* %tmpVar37, i32 0, i32 1, !dbg !104
      store i32 19, i32* %a38, align 4, !dbg !104
      %tmpVar39 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 2, !dbg !105
      %__parent40 = getelementptr inbounds %child, %child* %tmpVar39, i32 0, i32 0, !dbg !105
      %a41 = getelementptr inbounds %parent, %parent* %__parent40, i32 0, i32 1, !dbg !105
      store i32 20, i32* %a41, align 4, !dbg !105
      %tmpVar42 = getelementptr inbounds [3 x %child], [3 x %child]* %array_of_child, i32 0, i32 2, !dbg !106
      %b43 = getelementptr inbounds %child, %child* %tmpVar42, i32 0, i32 1, !dbg !106
      store i32 21, i32* %b43, align 4, !dbg !106
      %tmpVar44 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !107
      %__child45 = getelementptr inbounds %grandchild, %grandchild* %tmpVar44, i32 0, i32 0, !dbg !107
      %__parent46 = getelementptr inbounds %child, %child* %__child45, i32 0, i32 0, !dbg !107
      %a47 = getelementptr inbounds %parent, %parent* %__parent46, i32 0, i32 1, !dbg !107
      store i32 22, i32* %a47, align 4, !dbg !107
      %tmpVar48 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !108
      %__child49 = getelementptr inbounds %grandchild, %grandchild* %tmpVar48, i32 0, i32 0, !dbg !108
      %b50 = getelementptr inbounds %child, %child* %__child49, i32 0, i32 1, !dbg !108
      store i32 23, i32* %b50, align 4, !dbg !108
      %tmpVar51 = getelementptr inbounds [3 x %grandchild], [3 x %grandchild]* %array_of_grandchild, i32 0, i32 2, !dbg !109
      %c52 = getelementptr inbounds %grandchild, %grandchild* %tmpVar51, i32 0, i32 1, !dbg !109
      store i32 24, i32* %c52, align 4, !dbg !109
      %main_ret = load i32, i32* %main, align 4, !dbg !110
      ret i32 %main_ret, !dbg !110
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

    !llvm.module.flags = !{!43, !44}
    !llvm.dbg.cu = !{!45}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__grandchild__init", scope: !2, file: !2, line: 14, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandchild", scope: !2, file: !2, line: 14, size: 256, align: 64, flags: DIFlagPublic, elements: !4, identifier: "grandchild")
    !4 = !{!5, !17}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__child", scope: !2, file: !2, baseType: !6, size: 192, align: 64, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 8, size: 192, align: 64, flags: DIFlagPublic, elements: !7, identifier: "child")
    !7 = !{!8, !16}
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "__parent", scope: !2, file: !2, baseType: !9, size: 128, align: 64, flags: DIFlagPublic)
    !9 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 2, size: 128, align: 64, flags: DIFlagPublic, elements: !10, identifier: "parent")
    !10 = !{!11, !14}
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !12, size: 64, align: 64, flags: DIFlagPublic)
    !12 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__VOID_POINTER", baseType: !13, size: 64, align: 64, dwarfAddressSpace: 1)
    !13 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 4, baseType: !15, size: 32, align: 32, offset: 64, flags: DIFlagPublic)
    !15 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !16 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 10, baseType: !15, size: 32, align: 32, offset: 128, flags: DIFlagPublic)
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 16, baseType: !15, size: 32, align: 32, offset: 192, flags: DIFlagPublic)
    !18 = !DIGlobalVariableExpression(var: !19, expr: !DIExpression())
    !19 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 8, type: !6, isLocal: false, isDefinition: true)
    !20 = !DIGlobalVariableExpression(var: !21, expr: !DIExpression())
    !21 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 2, type: !9, isLocal: false, isDefinition: true)
    !22 = !DIGlobalVariableExpression(var: !23, expr: !DIExpression())
    !23 = distinct !DIGlobalVariable(name: "____vtable_parent_type__init", scope: !2, file: !2, type: !24, isLocal: false, isDefinition: true)
    !24 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_parent_type", scope: !2, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !25, identifier: "__vtable_parent_type")
    !25 = !{!26}
    !26 = !DIDerivedType(tag: DW_TAG_member, name: "__body", scope: !2, file: !2, baseType: !12, size: 64, align: 64, flags: DIFlagPublic)
    !27 = !DIGlobalVariableExpression(var: !28, expr: !DIExpression())
    !28 = distinct !DIGlobalVariable(name: "__vtable_parent", scope: !2, file: !2, type: !24, isLocal: false, isDefinition: true)
    !29 = !DIGlobalVariableExpression(var: !30, expr: !DIExpression())
    !30 = distinct !DIGlobalVariable(name: "____vtable_child_type__init", scope: !2, file: !2, type: !31, isLocal: false, isDefinition: true)
    !31 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_child_type", scope: !2, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !32, identifier: "__vtable_child_type")
    !32 = !{!26, !33}
    !33 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_parent_type", scope: !2, file: !2, baseType: !24, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !34 = !DIGlobalVariableExpression(var: !35, expr: !DIExpression())
    !35 = distinct !DIGlobalVariable(name: "__vtable_child", scope: !2, file: !2, type: !31, isLocal: false, isDefinition: true)
    !36 = !DIGlobalVariableExpression(var: !37, expr: !DIExpression())
    !37 = distinct !DIGlobalVariable(name: "____vtable_grandchild_type__init", scope: !2, file: !2, type: !38, isLocal: false, isDefinition: true)
    !38 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_grandchild_type", scope: !2, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !39, identifier: "__vtable_grandchild_type")
    !39 = !{!26, !40}
    !40 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable_child_type", scope: !2, file: !2, baseType: !31, size: 128, align: 64, offset: 64, flags: DIFlagPublic)
    !41 = !DIGlobalVariableExpression(var: !42, expr: !DIExpression())
    !42 = distinct !DIGlobalVariable(name: "__vtable_grandchild", scope: !2, file: !2, type: !38, isLocal: false, isDefinition: true)
    !43 = !{i32 2, !"Dwarf Version", i32 5}
    !44 = !{i32 2, !"Debug Info Version", i32 3}
    !45 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !46, splitDebugInlining: false)
    !46 = !{!27, !22, !34, !29, !41, !36, !20, !18, !0}
    !47 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 2, type: !48, scopeLine: 6, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !45, retainedNodes: !50)
    !48 = !DISubroutineType(flags: DIFlagPublic, types: !49)
    !49 = !{null, !9}
    !50 = !{}
    !51 = !DILocalVariable(name: "parent", scope: !47, file: !2, line: 6, type: !9)
    !52 = !DILocation(line: 6, scope: !47)
    !53 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 8, type: !54, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !45, retainedNodes: !50)
    !54 = !DISubroutineType(flags: DIFlagPublic, types: !55)
    !55 = !{null, !6}
    !56 = !DILocalVariable(name: "child", scope: !53, file: !2, line: 12, type: !6)
    !57 = !DILocation(line: 12, scope: !53)
    !58 = distinct !DISubprogram(name: "grandchild", linkageName: "grandchild", scope: !2, file: !2, line: 14, type: !59, scopeLine: 18, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !45, retainedNodes: !50)
    !59 = !DISubroutineType(flags: DIFlagPublic, types: !60)
    !60 = !{null, !3}
    !61 = !DILocalVariable(name: "grandchild", scope: !58, file: !2, line: 18, type: !3)
    !62 = !DILocation(line: 18, scope: !58)
    !63 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 20, type: !64, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !45, retainedNodes: !50)
    !64 = !DISubroutineType(flags: DIFlagPublic, types: !65)
    !65 = !{null}
    !66 = !DILocalVariable(name: "array_of_parent", scope: !63, file: !2, line: 22, type: !67, align: 64)
    !67 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 384, align: 64, elements: !68)
    !68 = !{!69}
    !69 = !DISubrange(count: 3, lowerBound: 0)
    !70 = !DILocation(line: 22, column: 4, scope: !63)
    !71 = !DILocalVariable(name: "array_of_child", scope: !63, file: !2, line: 23, type: !72, align: 64)
    !72 = !DICompositeType(tag: DW_TAG_array_type, baseType: !6, size: 576, align: 64, elements: !68)
    !73 = !DILocation(line: 23, column: 4, scope: !63)
    !74 = !DILocalVariable(name: "array_of_grandchild", scope: !63, file: !2, line: 24, type: !75, align: 64)
    !75 = !DICompositeType(tag: DW_TAG_array_type, baseType: !3, size: 768, align: 64, elements: !68)
    !76 = !DILocation(line: 24, column: 4, scope: !63)
    !77 = !DILocalVariable(name: "parent1", scope: !63, file: !2, line: 25, type: !9, align: 64)
    !78 = !DILocation(line: 25, column: 4, scope: !63)
    !79 = !DILocalVariable(name: "child1", scope: !63, file: !2, line: 26, type: !6, align: 64)
    !80 = !DILocation(line: 26, column: 4, scope: !63)
    !81 = !DILocalVariable(name: "grandchild1", scope: !63, file: !2, line: 27, type: !3, align: 64)
    !82 = !DILocation(line: 27, column: 4, scope: !63)
    !83 = !DILocalVariable(name: "main", scope: !63, file: !2, line: 20, type: !15, align: 32)
    !84 = !DILocation(line: 20, column: 9, scope: !63)
    !85 = !DILocation(line: 0, scope: !63)
    !86 = !DILocation(line: 30, column: 4, scope: !63)
    !87 = !DILocation(line: 31, column: 4, scope: !63)
    !88 = !DILocation(line: 32, column: 4, scope: !63)
    !89 = !DILocation(line: 33, column: 4, scope: !63)
    !90 = !DILocation(line: 34, column: 4, scope: !63)
    !91 = !DILocation(line: 35, column: 4, scope: !63)
    !92 = !DILocation(line: 37, column: 4, scope: !63)
    !93 = !DILocation(line: 38, column: 4, scope: !63)
    !94 = !DILocation(line: 39, column: 4, scope: !63)
    !95 = !DILocation(line: 40, column: 4, scope: !63)
    !96 = !DILocation(line: 41, column: 4, scope: !63)
    !97 = !DILocation(line: 42, column: 4, scope: !63)
    !98 = !DILocation(line: 43, column: 4, scope: !63)
    !99 = !DILocation(line: 44, column: 4, scope: !63)
    !100 = !DILocation(line: 45, column: 4, scope: !63)
    !101 = !DILocation(line: 46, column: 4, scope: !63)
    !102 = !DILocation(line: 47, column: 4, scope: !63)
    !103 = !DILocation(line: 48, column: 4, scope: !63)
    !104 = !DILocation(line: 49, column: 4, scope: !63)
    !105 = !DILocation(line: 50, column: 4, scope: !63)
    !106 = !DILocation(line: 51, column: 4, scope: !63)
    !107 = !DILocation(line: 52, column: 4, scope: !63)
    !108 = !DILocation(line: 53, column: 4, scope: !63)
    !109 = !DILocation(line: 54, column: 4, scope: !63)
    !110 = !DILocation(line: 56, scope: !63)
    "###);
}
