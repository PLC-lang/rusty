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

    %__vtable_foo = type { ptr }
    %foo = type { ptr, i16, [81 x i8], [11 x [81 x i8]] }
    %__vtable_bar = type { ptr }
    %bar = type { %foo }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer, !dbg !0
    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !6
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer, !dbg !27
    @____vtable_bar__init = unnamed_addr constant %__vtable_bar zeroinitializer, !dbg !29
    @__bar__init = unnamed_addr constant %bar zeroinitializer, !dbg !33
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer, !dbg !39

    define void @foo(ptr %0) !dbg !45 {
    entry:
        #dbg_declare(ptr %0, !48, !DIExpression(), !49)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %a = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %b = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      %c = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 3
      ret void, !dbg !49
    }

    define void @bar(ptr %0) !dbg !50 {
    entry:
        #dbg_declare(ptr %0, !53, !DIExpression(), !54)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__foo = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      ret void, !dbg !54
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @bar, ptr %__body, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__foo = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0
      call void @__init_foo(ptr %__foo)
      %deref1 = load ptr, ptr %self, align 8
      %__foo2 = getelementptr inbounds nuw %bar, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %foo, ptr %__foo2, i32 0, i32 0
      store ptr @__vtable_bar_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__foo = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0
      call void @__user_init_foo(ptr %__foo)
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__init___vtable_bar(ptr @__vtable_bar_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_bar(ptr @__vtable_bar_instance)
      ret void
    }

    !llvm.module.flags = !{!41, !42}
    !llvm.dbg.cu = !{!43}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "____vtable_foo__init", scope: !2, file: !2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_foo")
    !5 = !{}
    !6 = !DIGlobalVariableExpression(var: !7, expr: !DIExpression())
    !7 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !8, isLocal: false, isDefinition: true)
    !8 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !9)
    !9 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 7872, align: 64, flags: DIFlagPublic, elements: !10, identifier: "foo")
    !10 = !{!11, !15, !17, !23}
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !12, size: 64, align: 64, flags: DIFlagPublic)
    !12 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____foo___vtable", scope: !2, file: !2, baseType: !13, align: 64)
    !13 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__foo___vtable", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 1)
    !14 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 4, baseType: !16, size: 16, align: 16, offset: 64, flags: DIFlagPublic)
    !16 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 5, baseType: !18, size: 648, align: 8, offset: 80, flags: DIFlagPublic)
    !18 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !2, file: !2, baseType: !19, align: 8)
    !19 = !DICompositeType(tag: DW_TAG_array_type, baseType: !20, size: 648, align: 8, elements: !21)
    !20 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !21 = !{!22}
    !22 = !DISubrange(count: 81, lowerBound: 0)
    !23 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 6, baseType: !24, size: 7128, align: 8, offset: 728, flags: DIFlagPublic)
    !24 = !DICompositeType(tag: DW_TAG_array_type, baseType: !18, size: 7128, align: 8, elements: !25)
    !25 = !{!26}
    !26 = !DISubrange(count: 11, lowerBound: 0)
    !27 = !DIGlobalVariableExpression(var: !28, expr: !DIExpression())
    !28 = distinct !DIGlobalVariable(name: "__vtable_foo_instance", scope: !2, file: !2, type: !4, isLocal: false, isDefinition: true)
    !29 = !DIGlobalVariableExpression(var: !30, expr: !DIExpression())
    !30 = distinct !DIGlobalVariable(name: "____vtable_bar__init", scope: !2, file: !2, type: !31, isLocal: false, isDefinition: true)
    !31 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !32)
    !32 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_bar", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_bar")
    !33 = !DIGlobalVariableExpression(var: !34, expr: !DIExpression())
    !34 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 10, type: !35, isLocal: false, isDefinition: true)
    !35 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !36)
    !36 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 10, size: 7872, align: 64, flags: DIFlagPublic, elements: !37, identifier: "bar")
    !37 = !{!38}
    !38 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !2, file: !2, baseType: !9, size: 7872, align: 64, flags: DIFlagPublic)
    !39 = !DIGlobalVariableExpression(var: !40, expr: !DIExpression())
    !40 = distinct !DIGlobalVariable(name: "__vtable_bar_instance", scope: !2, file: !2, type: !32, isLocal: false, isDefinition: true)
    !41 = !{i32 2, !"Dwarf Version", i32 5}
    !42 = !{i32 2, !"Debug Info Version", i32 3}
    !43 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !44, splitDebugInlining: false)
    !44 = !{!27, !0, !6, !39, !29, !33}
    !45 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !46, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !43, retainedNodes: !5)
    !46 = !DISubroutineType(flags: DIFlagPublic, types: !47)
    !47 = !{null, !9}
    !48 = !DILocalVariable(name: "foo", scope: !45, file: !2, line: 8, type: !9)
    !49 = !DILocation(line: 8, column: 8, scope: !45)
    !50 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 10, type: !51, scopeLine: 11, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !43, retainedNodes: !5)
    !51 = !DISubroutineType(flags: DIFlagPublic, types: !52)
    !52 = !{null, !36}
    !53 = !DILocalVariable(name: "bar", scope: !50, file: !2, line: 11, type: !36)
    !54 = !DILocation(line: 11, column: 8, scope: !50)
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

    %__vtable_fb = type { ptr }
    %fb = type { ptr, i16, i16 }
    %__vtable_fb2 = type { ptr }
    %fb2 = type { %fb }
    %__vtable_foo = type { ptr }
    %foo = type { ptr, %fb2 }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_fb__init = unnamed_addr constant %__vtable_fb zeroinitializer, !dbg !0
    @__fb__init = unnamed_addr constant %fb zeroinitializer, !dbg !6
    @__vtable_fb_instance = global %__vtable_fb zeroinitializer, !dbg !18
    @____vtable_fb2__init = unnamed_addr constant %__vtable_fb2 zeroinitializer, !dbg !20
    @__fb2__init = unnamed_addr constant %fb2 zeroinitializer, !dbg !24
    @__vtable_fb2_instance = global %__vtable_fb2 zeroinitializer, !dbg !30
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer, !dbg !32
    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !36
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer, !dbg !45

    define void @fb(ptr %0) !dbg !51 {
    entry:
        #dbg_declare(ptr %0, !54, !DIExpression(), !55)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 1
      %y = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 2
      ret void, !dbg !55
    }

    define void @fb2(ptr %0) !dbg !56 {
    entry:
        #dbg_declare(ptr %0, !59, !DIExpression(), !60)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__fb = getelementptr inbounds nuw %fb2, ptr %0, i32 0, i32 0
      ret void, !dbg !60
    }

    define void @foo(ptr %0) !dbg !61 {
    entry:
        #dbg_declare(ptr %0, !64, !DIExpression(), !65)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %myFb = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %__fb = getelementptr inbounds nuw %fb2, ptr %myFb, i32 0, i32 0, !dbg !65
      %x = getelementptr inbounds nuw %fb, ptr %__fb, i32 0, i32 1, !dbg !65
      store i16 1, ptr %x, align 2, !dbg !65
      ret void, !dbg !66
    }

    define void @__init___vtable_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_fb, ptr %deref, i32 0, i32 0
      store ptr @fb, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_fb2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_fb, ptr %deref, i32 0, i32 0
      store ptr @fb2, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_fb, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      ret void
    }

    define void @__init_fb2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__fb = getelementptr inbounds nuw %fb2, ptr %deref, i32 0, i32 0
      call void @__init_fb(ptr %__fb)
      %deref1 = load ptr, ptr %self, align 8
      %__fb2 = getelementptr inbounds nuw %fb2, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %fb, ptr %__fb2, i32 0, i32 0
      store ptr @__vtable_fb2_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %fb, ptr %deref, i32 0, i32 0
      store ptr @__vtable_fb_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %myFb = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 1
      call void @__init_fb2(ptr %myFb)
      %deref1 = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_fb2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__fb = getelementptr inbounds nuw %fb2, ptr %deref, i32 0, i32 0
      call void @__user_init_fb(ptr %__fb)
      ret void
    }

    define void @__user_init___vtable_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_fb2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %myFb = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 1
      call void @__user_init_fb2(ptr %myFb)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_fb(ptr @__vtable_fb_instance)
      call void @__init___vtable_fb2(ptr @__vtable_fb2_instance)
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_fb(ptr @__vtable_fb_instance)
      call void @__user_init___vtable_fb2(ptr @__vtable_fb2_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }

    !llvm.module.flags = !{!47, !48}
    !llvm.dbg.cu = !{!49}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "____vtable_fb__init", scope: !2, file: !2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_fb", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_fb")
    !5 = !{}
    !6 = !DIGlobalVariableExpression(var: !7, expr: !DIExpression())
    !7 = distinct !DIGlobalVariable(name: "__fb__init", scope: !2, file: !2, line: 2, type: !8, isLocal: false, isDefinition: true)
    !8 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !9)
    !9 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb", scope: !2, file: !2, line: 2, size: 128, align: 64, flags: DIFlagPublic, elements: !10, identifier: "fb")
    !10 = !{!11, !15, !17}
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !12, size: 64, align: 64, flags: DIFlagPublic)
    !12 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____fb___vtable", scope: !2, file: !2, baseType: !13, align: 64)
    !13 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__fb___vtable", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 1)
    !14 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 4, baseType: !16, size: 16, align: 16, offset: 64, flags: DIFlagPublic)
    !16 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 5, baseType: !16, size: 16, align: 16, offset: 80, flags: DIFlagPublic)
    !18 = !DIGlobalVariableExpression(var: !19, expr: !DIExpression())
    !19 = distinct !DIGlobalVariable(name: "__vtable_fb_instance", scope: !2, file: !2, type: !4, isLocal: false, isDefinition: true)
    !20 = !DIGlobalVariableExpression(var: !21, expr: !DIExpression())
    !21 = distinct !DIGlobalVariable(name: "____vtable_fb2__init", scope: !2, file: !2, type: !22, isLocal: false, isDefinition: true)
    !22 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !23)
    !23 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_fb2", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_fb2")
    !24 = !DIGlobalVariableExpression(var: !25, expr: !DIExpression())
    !25 = distinct !DIGlobalVariable(name: "__fb2__init", scope: !2, file: !2, line: 9, type: !26, isLocal: false, isDefinition: true)
    !26 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !27)
    !27 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb2", scope: !2, file: !2, line: 9, size: 128, align: 64, flags: DIFlagPublic, elements: !28, identifier: "fb2")
    !28 = !{!29}
    !29 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !2, file: !2, baseType: !9, size: 128, align: 64, flags: DIFlagPublic)
    !30 = !DIGlobalVariableExpression(var: !31, expr: !DIExpression())
    !31 = distinct !DIGlobalVariable(name: "__vtable_fb2_instance", scope: !2, file: !2, type: !23, isLocal: false, isDefinition: true)
    !32 = !DIGlobalVariableExpression(var: !33, expr: !DIExpression())
    !33 = distinct !DIGlobalVariable(name: "____vtable_foo__init", scope: !2, file: !2, type: !34, isLocal: false, isDefinition: true)
    !34 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !35)
    !35 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_foo")
    !36 = !DIGlobalVariableExpression(var: !37, expr: !DIExpression())
    !37 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 12, type: !38, isLocal: false, isDefinition: true)
    !38 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !39)
    !39 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 12, size: 192, align: 64, flags: DIFlagPublic, elements: !40, identifier: "foo")
    !40 = !{!41, !44}
    !41 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !42, size: 64, align: 64, flags: DIFlagPublic)
    !42 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____foo___vtable", scope: !2, file: !2, baseType: !43, align: 64)
    !43 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__foo___vtable", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 1)
    !44 = !DIDerivedType(tag: DW_TAG_member, name: "myFb", scope: !2, file: !2, line: 14, baseType: !27, size: 128, align: 64, offset: 64, flags: DIFlagPublic)
    !45 = !DIGlobalVariableExpression(var: !46, expr: !DIExpression())
    !46 = distinct !DIGlobalVariable(name: "__vtable_foo_instance", scope: !2, file: !2, type: !35, isLocal: false, isDefinition: true)
    !47 = !{i32 2, !"Dwarf Version", i32 5}
    !48 = !{i32 2, !"Debug Info Version", i32 3}
    !49 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !50, splitDebugInlining: false)
    !50 = !{!18, !0, !6, !30, !20, !24, !45, !32, !36}
    !51 = distinct !DISubprogram(name: "fb", linkageName: "fb", scope: !2, file: !2, line: 2, type: !52, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !49, retainedNodes: !5)
    !52 = !DISubroutineType(flags: DIFlagPublic, types: !53)
    !53 = !{null, !9}
    !54 = !DILocalVariable(name: "fb", scope: !51, file: !2, line: 7, type: !9)
    !55 = !DILocation(line: 7, column: 8, scope: !51)
    !56 = distinct !DISubprogram(name: "fb2", linkageName: "fb2", scope: !2, file: !2, line: 9, type: !57, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !49, retainedNodes: !5)
    !57 = !DISubroutineType(flags: DIFlagPublic, types: !58)
    !58 = !{null, !27}
    !59 = !DILocalVariable(name: "fb2", scope: !56, file: !2, line: 10, type: !27)
    !60 = !DILocation(line: 10, column: 8, scope: !56)
    !61 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 12, type: !62, scopeLine: 16, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !49, retainedNodes: !5)
    !62 = !DISubroutineType(flags: DIFlagPublic, types: !63)
    !63 = !{null, !39}
    !64 = !DILocalVariable(name: "foo", scope: !61, file: !2, line: 16, type: !39)
    !65 = !DILocation(line: 16, column: 12, scope: !61)
    !66 = !DILocation(line: 17, column: 8, scope: !61)
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

    %__vtable_foo = type { ptr, ptr }
    %foo = type { ptr, [81 x i8] }
    %__vtable_bar = type { ptr, ptr }
    %bar = type { %foo }

    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"world\00"
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer, !dbg !0
    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !6
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer, !dbg !21
    @____vtable_bar__init = unnamed_addr constant %__vtable_bar zeroinitializer, !dbg !23
    @__bar__init = unnamed_addr constant %bar zeroinitializer, !dbg !27
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer, !dbg !33

    define void @foo(ptr %0) !dbg !39 {
    entry:
        #dbg_declare(ptr %0, !42, !DIExpression(), !43)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      ret void, !dbg !43
    }

    define void @foo__baz(ptr %0) !dbg !44 {
    entry:
        #dbg_declare(ptr %0, !45, !DIExpression(), !46)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %s, ptr align 1 @utf08_literal_0, i32 6, i1 false), !dbg !46
      ret void, !dbg !47
    }

    define void @bar(ptr %0) !dbg !48 {
    entry:
        #dbg_declare(ptr %0, !51, !DIExpression(), !52)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__foo = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %__foo, i32 0, i32 1, !dbg !52
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %s, ptr align 1 @utf08_literal_1, i32 6, i1 false), !dbg !52
      ret void, !dbg !53
    }

    define void @main() !dbg !54 {
    entry:
      %s = alloca [81 x i8], align 1
      %fb = alloca %bar, align 8
        #dbg_declare(ptr %s, !57, !DIExpression(), !58)
      call void @llvm.memset.p0.i64(ptr align 1 %s, i8 0, i64 ptrtoint (ptr getelementptr ([81 x i8], ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %fb, !59, !DIExpression(), !60)
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %fb, ptr align 1 @__bar__init, i64 ptrtoint (ptr getelementptr (%bar, ptr null, i32 1) to i64), i1 false)
      call void @__init_bar(ptr %fb), !dbg !61
      call void @__user_init_bar(ptr %fb), !dbg !61
      %__foo = getelementptr inbounds nuw %bar, ptr %fb, i32 0, i32 0, !dbg !61
      call void @foo__baz(ptr %__foo), !dbg !62
      call void @bar(ptr %fb), !dbg !63
      ret void, !dbg !64
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #1

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %baz = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__baz, ptr %baz, align 8
      ret void
    }

    define void @__init___vtable_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @bar, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %baz = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__baz, ptr %baz, align 8
      ret void
    }

    define void @__init_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__foo = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0
      call void @__init_foo(ptr %__foo)
      %deref1 = load ptr, ptr %self, align 8
      %__foo2 = getelementptr inbounds nuw %bar, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %foo, ptr %__foo2, i32 0, i32 0
      store ptr @__vtable_bar_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__foo = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0
      call void @__user_init_foo(ptr %__foo)
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__init___vtable_bar(ptr @__vtable_bar_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_bar(ptr @__vtable_bar_instance)
      ret void
    }

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: write) }

    !llvm.module.flags = !{!35, !36}
    !llvm.dbg.cu = !{!37}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "____vtable_foo__init", scope: !2, file: !2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_foo")
    !5 = !{}
    !6 = !DIGlobalVariableExpression(var: !7, expr: !DIExpression())
    !7 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !8, isLocal: false, isDefinition: true)
    !8 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !9)
    !9 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 768, align: 64, flags: DIFlagPublic, elements: !10, identifier: "foo")
    !10 = !{!11, !15}
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !12, size: 64, align: 64, flags: DIFlagPublic)
    !12 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____foo___vtable", scope: !2, file: !2, baseType: !13, align: 64)
    !13 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__foo___vtable", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 1)
    !14 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !2, file: !2, line: 4, baseType: !16, size: 648, align: 8, offset: 64, flags: DIFlagPublic)
    !16 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !2, file: !2, baseType: !17, align: 8)
    !17 = !DICompositeType(tag: DW_TAG_array_type, baseType: !18, size: 648, align: 8, elements: !19)
    !18 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !19 = !{!20}
    !20 = !DISubrange(count: 81, lowerBound: 0)
    !21 = !DIGlobalVariableExpression(var: !22, expr: !DIExpression())
    !22 = distinct !DIGlobalVariable(name: "__vtable_foo_instance", scope: !2, file: !2, type: !4, isLocal: false, isDefinition: true)
    !23 = !DIGlobalVariableExpression(var: !24, expr: !DIExpression())
    !24 = distinct !DIGlobalVariable(name: "____vtable_bar__init", scope: !2, file: !2, type: !25, isLocal: false, isDefinition: true)
    !25 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !26)
    !26 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_bar", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_bar")
    !27 = !DIGlobalVariableExpression(var: !28, expr: !DIExpression())
    !28 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 11, type: !29, isLocal: false, isDefinition: true)
    !29 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !30)
    !30 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 11, size: 768, align: 64, flags: DIFlagPublic, elements: !31, identifier: "bar")
    !31 = !{!32}
    !32 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !2, file: !2, baseType: !9, size: 768, align: 64, flags: DIFlagPublic)
    !33 = !DIGlobalVariableExpression(var: !34, expr: !DIExpression())
    !34 = distinct !DIGlobalVariable(name: "__vtable_bar_instance", scope: !2, file: !2, type: !26, isLocal: false, isDefinition: true)
    !35 = !{i32 2, !"Dwarf Version", i32 5}
    !36 = !{i32 2, !"Debug Info Version", i32 3}
    !37 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !38, splitDebugInlining: false)
    !38 = !{!21, !0, !6, !33, !23, !27}
    !39 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !40, scopeLine: 9, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !37, retainedNodes: !5)
    !40 = !DISubroutineType(flags: DIFlagPublic, types: !41)
    !41 = !{null, !9}
    !42 = !DILocalVariable(name: "foo", scope: !39, file: !2, line: 9, type: !9)
    !43 = !DILocation(line: 9, column: 8, scope: !39)
    !44 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !39, file: !2, line: 6, type: !40, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !37, retainedNodes: !5)
    !45 = !DILocalVariable(name: "foo", scope: !44, file: !2, line: 7, type: !9)
    !46 = !DILocation(line: 7, column: 12, scope: !44)
    !47 = !DILocation(line: 8, column: 8, scope: !44)
    !48 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 11, type: !49, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !37, retainedNodes: !5)
    !49 = !DISubroutineType(flags: DIFlagPublic, types: !50)
    !50 = !{null, !30}
    !51 = !DILocalVariable(name: "bar", scope: !48, file: !2, line: 12, type: !30)
    !52 = !DILocation(line: 12, column: 12, scope: !48)
    !53 = !DILocation(line: 13, column: 8, scope: !48)
    !54 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 15, type: !55, scopeLine: 15, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !37, retainedNodes: !5)
    !55 = !DISubroutineType(flags: DIFlagPublic, types: !56)
    !56 = !{null}
    !57 = !DILocalVariable(name: "s", scope: !54, file: !2, line: 17, type: !16, align: 8)
    !58 = !DILocation(line: 17, column: 12, scope: !54)
    !59 = !DILocalVariable(name: "fb", scope: !54, file: !2, line: 18, type: !30, align: 64)
    !60 = !DILocation(line: 18, column: 12, scope: !54)
    !61 = !DILocation(line: 0, scope: !54)
    !62 = !DILocation(line: 20, column: 12, scope: !54)
    !63 = !DILocation(line: 21, column: 12, scope: !54)
    !64 = !DILocation(line: 22, column: 8, scope: !54)
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

    %__vtable_grandparent = type { ptr }
    %grandparent = type { ptr, [6 x i16], i16 }
    %__vtable_parent = type { ptr }
    %parent = type { %grandparent, [11 x i16], i16 }
    %__vtable_child = type { ptr }
    %child = type { %parent, [11 x i16] }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_grandparent__init = unnamed_addr constant %__vtable_grandparent zeroinitializer, !dbg !0
    @__grandparent__init = unnamed_addr constant %grandparent zeroinitializer, !dbg !6
    @__vtable_grandparent_instance = global %__vtable_grandparent zeroinitializer, !dbg !21
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer, !dbg !23
    @__parent__init = unnamed_addr constant %parent zeroinitializer, !dbg !27
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer, !dbg !38
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer, !dbg !40
    @__child__init = unnamed_addr constant %child zeroinitializer, !dbg !44
    @__vtable_child_instance = global %__vtable_child zeroinitializer, !dbg !51

    define void @grandparent(ptr %0) !dbg !57 {
    entry:
        #dbg_declare(ptr %0, !60, !DIExpression(), !61)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 0
      %y = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 1
      %a = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 2
      ret void, !dbg !61
    }

    define void @parent(ptr %0) !dbg !62 {
    entry:
        #dbg_declare(ptr %0, !65, !DIExpression(), !66)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %b = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void, !dbg !66
    }

    define void @child(ptr %0) !dbg !67 {
    entry:
        #dbg_declare(ptr %0, !70, !DIExpression(), !71)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %z = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      ret void, !dbg !71
    }

    define void @main() !dbg !72 {
    entry:
      %arr = alloca [11 x %child], align 8
        #dbg_declare(ptr %arr, !75, !DIExpression(), !77)
      call void @llvm.memset.p0.i64(ptr align 1 %arr, i8 0, i64 ptrtoint (ptr getelementptr ([11 x %child], ptr null, i32 1) to i64), i1 false)
      %tmpVar = getelementptr inbounds [11 x %child], ptr %arr, i32 0, i32 0, !dbg !78
      %__parent = getelementptr inbounds nuw %child, ptr %tmpVar, i32 0, i32 0, !dbg !78
      %__grandparent = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 0, !dbg !78
      %a = getelementptr inbounds nuw %grandparent, ptr %__grandparent, i32 0, i32 2, !dbg !78
      store i16 10, ptr %a, align 2, !dbg !78
      %tmpVar1 = getelementptr inbounds [11 x %child], ptr %arr, i32 0, i32 0, !dbg !79
      %__parent2 = getelementptr inbounds nuw %child, ptr %tmpVar1, i32 0, i32 0, !dbg !79
      %__grandparent3 = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0, !dbg !79
      %y = getelementptr inbounds nuw %grandparent, ptr %__grandparent3, i32 0, i32 1, !dbg !79
      %tmpVar4 = getelementptr inbounds [6 x i16], ptr %y, i32 0, i32 0, !dbg !79
      store i16 20, ptr %tmpVar4, align 2, !dbg !79
      %tmpVar5 = getelementptr inbounds [11 x %child], ptr %arr, i32 0, i32 1, !dbg !80
      %__parent6 = getelementptr inbounds nuw %child, ptr %tmpVar5, i32 0, i32 0, !dbg !80
      %b = getelementptr inbounds nuw %parent, ptr %__parent6, i32 0, i32 2, !dbg !80
      store i16 30, ptr %b, align 2, !dbg !80
      %tmpVar7 = getelementptr inbounds [11 x %child], ptr %arr, i32 0, i32 1, !dbg !81
      %__parent8 = getelementptr inbounds nuw %child, ptr %tmpVar7, i32 0, i32 0, !dbg !81
      %x = getelementptr inbounds nuw %parent, ptr %__parent8, i32 0, i32 1, !dbg !81
      %tmpVar9 = getelementptr inbounds [11 x i16], ptr %x, i32 0, i32 1, !dbg !81
      store i16 40, ptr %tmpVar9, align 2, !dbg !81
      %tmpVar10 = getelementptr inbounds [11 x %child], ptr %arr, i32 0, i32 2, !dbg !82
      %z = getelementptr inbounds nuw %child, ptr %tmpVar10, i32 0, i32 1, !dbg !82
      %tmpVar11 = getelementptr inbounds [11 x i16], ptr %z, i32 0, i32 2, !dbg !82
      store i16 50, ptr %tmpVar11, align 2, !dbg !82
      ret void, !dbg !83
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    define void @__init___vtable_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      store ptr @grandparent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__grandparent = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__init_grandparent(ptr %__grandparent)
      %deref1 = load ptr, ptr %self, align 8
      %__grandparent2 = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent2, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_grandparent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__user_init_grandparent(ptr %__grandparent)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_grandparent(ptr @__vtable_grandparent_instance)
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_grandparent(ptr @__vtable_grandparent_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }

    !llvm.module.flags = !{!53, !54}
    !llvm.dbg.cu = !{!55}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "____vtable_grandparent__init", scope: !2, file: !2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_grandparent", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_grandparent")
    !5 = !{}
    !6 = !DIGlobalVariableExpression(var: !7, expr: !DIExpression())
    !7 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !8, isLocal: false, isDefinition: true)
    !8 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !9)
    !9 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 192, align: 64, flags: DIFlagPublic, elements: !10, identifier: "grandparent")
    !10 = !{!11, !15, !20}
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !12, size: 64, align: 64, flags: DIFlagPublic)
    !12 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____grandparent___vtable", scope: !2, file: !2, baseType: !13, align: 64)
    !13 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__grandparent___vtable", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 1)
    !14 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !16, size: 96, align: 16, offset: 64, flags: DIFlagPublic)
    !16 = !DICompositeType(tag: DW_TAG_array_type, baseType: !17, size: 96, align: 16, elements: !18)
    !17 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !18 = !{!19}
    !19 = !DISubrange(count: 6, lowerBound: 0)
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !17, size: 16, align: 16, offset: 160, flags: DIFlagPublic)
    !21 = !DIGlobalVariableExpression(var: !22, expr: !DIExpression())
    !22 = distinct !DIGlobalVariable(name: "__vtable_grandparent_instance", scope: !2, file: !2, type: !4, isLocal: false, isDefinition: true)
    !23 = !DIGlobalVariableExpression(var: !24, expr: !DIExpression())
    !24 = distinct !DIGlobalVariable(name: "____vtable_parent__init", scope: !2, file: !2, type: !25, isLocal: false, isDefinition: true)
    !25 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !26)
    !26 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_parent", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_parent")
    !27 = !DIGlobalVariableExpression(var: !28, expr: !DIExpression())
    !28 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !29, isLocal: false, isDefinition: true)
    !29 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !30)
    !30 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 384, align: 64, flags: DIFlagPublic, elements: !31, identifier: "parent")
    !31 = !{!32, !33, !37}
    !32 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !2, file: !2, baseType: !9, size: 192, align: 64, flags: DIFlagPublic)
    !33 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !34, size: 176, align: 16, offset: 192, flags: DIFlagPublic)
    !34 = !DICompositeType(tag: DW_TAG_array_type, baseType: !17, size: 176, align: 16, elements: !35)
    !35 = !{!36}
    !36 = !DISubrange(count: 11, lowerBound: 0)
    !37 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !17, size: 16, align: 16, offset: 368, flags: DIFlagPublic)
    !38 = !DIGlobalVariableExpression(var: !39, expr: !DIExpression())
    !39 = distinct !DIGlobalVariable(name: "__vtable_parent_instance", scope: !2, file: !2, type: !26, isLocal: false, isDefinition: true)
    !40 = !DIGlobalVariableExpression(var: !41, expr: !DIExpression())
    !41 = distinct !DIGlobalVariable(name: "____vtable_child__init", scope: !2, file: !2, type: !42, isLocal: false, isDefinition: true)
    !42 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !43)
    !43 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_child", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_child")
    !44 = !DIGlobalVariableExpression(var: !45, expr: !DIExpression())
    !45 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !46, isLocal: false, isDefinition: true)
    !46 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !47)
    !47 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 576, align: 64, flags: DIFlagPublic, elements: !48, identifier: "child")
    !48 = !{!49, !50}
    !49 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !2, file: !2, baseType: !30, size: 384, align: 64, flags: DIFlagPublic)
    !50 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !34, size: 176, align: 16, offset: 384, flags: DIFlagPublic)
    !51 = !DIGlobalVariableExpression(var: !52, expr: !DIExpression())
    !52 = distinct !DIGlobalVariable(name: "__vtable_child_instance", scope: !2, file: !2, type: !43, isLocal: false, isDefinition: true)
    !53 = !{i32 2, !"Dwarf Version", i32 5}
    !54 = !{i32 2, !"Debug Info Version", i32 3}
    !55 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !56, splitDebugInlining: false)
    !56 = !{!21, !0, !6, !38, !23, !27, !51, !40, !44}
    !57 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !58, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !55, retainedNodes: !5)
    !58 = !DISubroutineType(flags: DIFlagPublic, types: !59)
    !59 = !{null, !9}
    !60 = !DILocalVariable(name: "grandparent", scope: !57, file: !2, line: 7, type: !9)
    !61 = !DILocation(line: 7, column: 8, scope: !57)
    !62 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !63, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !55, retainedNodes: !5)
    !63 = !DISubroutineType(flags: DIFlagPublic, types: !64)
    !64 = !{null, !30}
    !65 = !DILocalVariable(name: "parent", scope: !62, file: !2, line: 14, type: !30)
    !66 = !DILocation(line: 14, column: 8, scope: !62)
    !67 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !68, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !55, retainedNodes: !5)
    !68 = !DISubroutineType(flags: DIFlagPublic, types: !69)
    !69 = !{null, !47}
    !70 = !DILocalVariable(name: "child", scope: !67, file: !2, line: 20, type: !47)
    !71 = !DILocation(line: 20, column: 8, scope: !67)
    !72 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 22, type: !73, scopeLine: 26, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !55, retainedNodes: !5)
    !73 = !DISubroutineType(flags: DIFlagPublic, types: !74)
    !74 = !{null}
    !75 = !DILocalVariable(name: "arr", scope: !72, file: !2, line: 24, type: !76, align: 64)
    !76 = !DICompositeType(tag: DW_TAG_array_type, baseType: !47, size: 6336, align: 64, elements: !35)
    !77 = !DILocation(line: 24, column: 12, scope: !72)
    !78 = !DILocation(line: 26, column: 12, scope: !72)
    !79 = !DILocation(line: 27, column: 12, scope: !72)
    !80 = !DILocation(line: 28, column: 12, scope: !72)
    !81 = !DILocation(line: 29, column: 12, scope: !72)
    !82 = !DILocation(line: 30, column: 12, scope: !72)
    !83 = !DILocation(line: 31, column: 8, scope: !72)
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

    %__vtable_grandparent = type { ptr }
    %grandparent = type { ptr, [6 x i16], i16 }
    %__vtable_parent = type { ptr }
    %parent = type { %grandparent, [11 x i16], i16 }
    %__vtable_child = type { ptr }
    %child = type { %parent, [11 x i16] }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_grandparent__init = unnamed_addr constant %__vtable_grandparent zeroinitializer, !dbg !0
    @__grandparent__init = unnamed_addr constant %grandparent zeroinitializer, !dbg !6
    @__vtable_grandparent_instance = global %__vtable_grandparent zeroinitializer, !dbg !21
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer, !dbg !23
    @__parent__init = unnamed_addr constant %parent zeroinitializer, !dbg !27
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer, !dbg !38
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer, !dbg !40
    @__child__init = unnamed_addr constant %child zeroinitializer, !dbg !44
    @__vtable_child_instance = global %__vtable_child zeroinitializer, !dbg !51

    define void @grandparent(ptr %0) !dbg !57 {
    entry:
        #dbg_declare(ptr %0, !60, !DIExpression(), !61)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 0
      %y = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 1
      %a = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 2
      ret void, !dbg !61
    }

    define void @parent(ptr %0) !dbg !62 {
    entry:
        #dbg_declare(ptr %0, !65, !DIExpression(), !66)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %b = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void, !dbg !66
    }

    define void @child(ptr %0) !dbg !67 {
    entry:
        #dbg_declare(ptr %0, !70, !DIExpression(), !71)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %z = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      %__grandparent = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 0, !dbg !71
      %y = getelementptr inbounds nuw %grandparent, ptr %__grandparent, i32 0, i32 1, !dbg !71
      %b = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2, !dbg !71
      %load_b = load i16, ptr %b, align 2, !dbg !71
      %1 = sext i16 %load_b to i32, !dbg !71
      %b1 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2, !dbg !71
      %load_b2 = load i16, ptr %b1, align 2, !dbg !71
      %2 = sext i16 %load_b2 to i32, !dbg !71
      %tmpVar = mul i32 %2, 2, !dbg !71
      %tmpVar3 = mul i32 1, %tmpVar, !dbg !71
      %tmpVar4 = add i32 %tmpVar3, 0, !dbg !71
      %tmpVar5 = getelementptr inbounds [11 x i16], ptr %z, i32 0, i32 %tmpVar4, !dbg !71
      %load_tmpVar = load i16, ptr %tmpVar5, align 2, !dbg !71
      %3 = sext i16 %load_tmpVar to i32, !dbg !71
      %tmpVar6 = add i32 %1, %3, !dbg !71
      %__grandparent7 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 0, !dbg !71
      %a = getelementptr inbounds nuw %grandparent, ptr %__grandparent7, i32 0, i32 2, !dbg !71
      %load_a = load i16, ptr %a, align 2, !dbg !71
      %4 = sext i16 %load_a to i32, !dbg !71
      %tmpVar8 = sub i32 %tmpVar6, %4, !dbg !71
      %tmpVar9 = mul i32 1, %tmpVar8, !dbg !71
      %tmpVar10 = add i32 %tmpVar9, 0, !dbg !71
      %tmpVar11 = getelementptr inbounds [6 x i16], ptr %y, i32 0, i32 %tmpVar10, !dbg !71
      store i16 20, ptr %tmpVar11, align 2, !dbg !71
      ret void, !dbg !72
    }

    define void @__init___vtable_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      store ptr @grandparent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__init_grandparent(ptr %__grandparent)
      %deref1 = load ptr, ptr %self, align 8
      %__grandparent2 = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent2, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_grandparent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__grandparent = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__user_init_grandparent(ptr %__grandparent)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_grandparent(ptr @__vtable_grandparent_instance)
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_grandparent(ptr @__vtable_grandparent_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }

    !llvm.module.flags = !{!53, !54}
    !llvm.dbg.cu = !{!55}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "____vtable_grandparent__init", scope: !2, file: !2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_grandparent", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_grandparent")
    !5 = !{}
    !6 = !DIGlobalVariableExpression(var: !7, expr: !DIExpression())
    !7 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !8, isLocal: false, isDefinition: true)
    !8 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !9)
    !9 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 192, align: 64, flags: DIFlagPublic, elements: !10, identifier: "grandparent")
    !10 = !{!11, !15, !20}
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !12, size: 64, align: 64, flags: DIFlagPublic)
    !12 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____grandparent___vtable", scope: !2, file: !2, baseType: !13, align: 64)
    !13 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__grandparent___vtable", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 1)
    !14 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 4, baseType: !16, size: 96, align: 16, offset: 64, flags: DIFlagPublic)
    !16 = !DICompositeType(tag: DW_TAG_array_type, baseType: !17, size: 96, align: 16, elements: !18)
    !17 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !18 = !{!19}
    !19 = !DISubrange(count: 6, lowerBound: 0)
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 5, baseType: !17, size: 16, align: 16, offset: 160, flags: DIFlagPublic)
    !21 = !DIGlobalVariableExpression(var: !22, expr: !DIExpression())
    !22 = distinct !DIGlobalVariable(name: "__vtable_grandparent_instance", scope: !2, file: !2, type: !4, isLocal: false, isDefinition: true)
    !23 = !DIGlobalVariableExpression(var: !24, expr: !DIExpression())
    !24 = distinct !DIGlobalVariable(name: "____vtable_parent__init", scope: !2, file: !2, type: !25, isLocal: false, isDefinition: true)
    !25 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !26)
    !26 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_parent", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_parent")
    !27 = !DIGlobalVariableExpression(var: !28, expr: !DIExpression())
    !28 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !29, isLocal: false, isDefinition: true)
    !29 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !30)
    !30 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 384, align: 64, flags: DIFlagPublic, elements: !31, identifier: "parent")
    !31 = !{!32, !33, !37}
    !32 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !2, file: !2, baseType: !9, size: 192, align: 64, flags: DIFlagPublic)
    !33 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !34, size: 176, align: 16, offset: 192, flags: DIFlagPublic)
    !34 = !DICompositeType(tag: DW_TAG_array_type, baseType: !17, size: 176, align: 16, elements: !35)
    !35 = !{!36}
    !36 = !DISubrange(count: 11, lowerBound: 0)
    !37 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 12, baseType: !17, size: 16, align: 16, offset: 368, flags: DIFlagPublic)
    !38 = !DIGlobalVariableExpression(var: !39, expr: !DIExpression())
    !39 = distinct !DIGlobalVariable(name: "__vtable_parent_instance", scope: !2, file: !2, type: !26, isLocal: false, isDefinition: true)
    !40 = !DIGlobalVariableExpression(var: !41, expr: !DIExpression())
    !41 = distinct !DIGlobalVariable(name: "____vtable_child__init", scope: !2, file: !2, type: !42, isLocal: false, isDefinition: true)
    !42 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !43)
    !43 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_child", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_child")
    !44 = !DIGlobalVariableExpression(var: !45, expr: !DIExpression())
    !45 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !46, isLocal: false, isDefinition: true)
    !46 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !47)
    !47 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 576, align: 64, flags: DIFlagPublic, elements: !48, identifier: "child")
    !48 = !{!49, !50}
    !49 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !2, file: !2, baseType: !30, size: 384, align: 64, flags: DIFlagPublic)
    !50 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !34, size: 176, align: 16, offset: 384, flags: DIFlagPublic)
    !51 = !DIGlobalVariableExpression(var: !52, expr: !DIExpression())
    !52 = distinct !DIGlobalVariable(name: "__vtable_child_instance", scope: !2, file: !2, type: !43, isLocal: false, isDefinition: true)
    !53 = !{i32 2, !"Dwarf Version", i32 5}
    !54 = !{i32 2, !"Debug Info Version", i32 3}
    !55 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !56, splitDebugInlining: false)
    !56 = !{!21, !0, !6, !38, !23, !27, !51, !40, !44}
    !57 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !58, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !55, retainedNodes: !5)
    !58 = !DISubroutineType(flags: DIFlagPublic, types: !59)
    !59 = !{null, !9}
    !60 = !DILocalVariable(name: "grandparent", scope: !57, file: !2, line: 7, type: !9)
    !61 = !DILocation(line: 7, column: 8, scope: !57)
    !62 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !63, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !55, retainedNodes: !5)
    !63 = !DISubroutineType(flags: DIFlagPublic, types: !64)
    !64 = !{null, !30}
    !65 = !DILocalVariable(name: "parent", scope: !62, file: !2, line: 14, type: !30)
    !66 = !DILocation(line: 14, column: 8, scope: !62)
    !67 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !68, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !55, retainedNodes: !5)
    !68 = !DISubroutineType(flags: DIFlagPublic, types: !69)
    !69 = !{null, !47}
    !70 = !DILocalVariable(name: "child", scope: !67, file: !2, line: 20, type: !47)
    !71 = !DILocation(line: 20, column: 12, scope: !67)
    !72 = !DILocation(line: 21, column: 8, scope: !67)
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

    %__vtable_foo = type { ptr, ptr }
    %foo = type { ptr }
    %__vtable_bar = type { ptr, ptr }
    %bar = type { %foo }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer, !dbg !0
    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !6
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer, !dbg !15
    @____vtable_bar__init = unnamed_addr constant %__vtable_bar zeroinitializer, !dbg !17
    @__bar__init = unnamed_addr constant %bar zeroinitializer, !dbg !21
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer, !dbg !27

    define void @foo(ptr %0) !dbg !33 {
    entry:
        #dbg_declare(ptr %0, !36, !DIExpression(), !37)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void, !dbg !37
    }

    define void @foo__baz(ptr %0) !dbg !38 {
    entry:
        #dbg_declare(ptr %0, !39, !DIExpression(), !40)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void, !dbg !40
    }

    define void @bar(ptr %0) !dbg !41 {
    entry:
        #dbg_declare(ptr %0, !44, !DIExpression(), !45)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__foo = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      ret void, !dbg !45
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %baz = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__baz, ptr %baz, align 8
      ret void
    }

    define void @__init___vtable_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @bar, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %baz = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__baz, ptr %baz, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__foo = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0
      call void @__init_foo(ptr %__foo)
      %deref1 = load ptr, ptr %self, align 8
      %__foo2 = getelementptr inbounds nuw %bar, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %foo, ptr %__foo2, i32 0, i32 0
      store ptr @__vtable_bar_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__foo = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0
      call void @__user_init_foo(ptr %__foo)
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__init___vtable_bar(ptr @__vtable_bar_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_bar(ptr @__vtable_bar_instance)
      ret void
    }

    !llvm.module.flags = !{!29, !30}
    !llvm.dbg.cu = !{!31}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "____vtable_foo__init", scope: !2, file: !2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_foo", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_foo")
    !5 = !{}
    !6 = !DIGlobalVariableExpression(var: !7, expr: !DIExpression())
    !7 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !8, isLocal: false, isDefinition: true)
    !8 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !9)
    !9 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 64, align: 64, flags: DIFlagPublic, elements: !10, identifier: "foo")
    !10 = !{!11}
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !12, size: 64, align: 64, flags: DIFlagPublic)
    !12 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____foo___vtable", scope: !2, file: !2, baseType: !13, align: 64)
    !13 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__foo___vtable", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 1)
    !14 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !15 = !DIGlobalVariableExpression(var: !16, expr: !DIExpression())
    !16 = distinct !DIGlobalVariable(name: "__vtable_foo_instance", scope: !2, file: !2, type: !4, isLocal: false, isDefinition: true)
    !17 = !DIGlobalVariableExpression(var: !18, expr: !DIExpression())
    !18 = distinct !DIGlobalVariable(name: "____vtable_bar__init", scope: !2, file: !2, type: !19, isLocal: false, isDefinition: true)
    !19 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !20)
    !20 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_bar", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_bar")
    !21 = !DIGlobalVariableExpression(var: !22, expr: !DIExpression())
    !22 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 7, type: !23, isLocal: false, isDefinition: true)
    !23 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !24)
    !24 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 7, size: 64, align: 64, flags: DIFlagPublic, elements: !25, identifier: "bar")
    !25 = !{!26}
    !26 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !2, file: !2, baseType: !9, size: 64, align: 64, flags: DIFlagPublic)
    !27 = !DIGlobalVariableExpression(var: !28, expr: !DIExpression())
    !28 = distinct !DIGlobalVariable(name: "__vtable_bar_instance", scope: !2, file: !2, type: !20, isLocal: false, isDefinition: true)
    !29 = !{i32 2, !"Dwarf Version", i32 5}
    !30 = !{i32 2, !"Debug Info Version", i32 3}
    !31 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !32, splitDebugInlining: false)
    !32 = !{!15, !0, !6, !27, !17, !21}
    !33 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !34, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !31, retainedNodes: !5)
    !34 = !DISubroutineType(flags: DIFlagPublic, types: !35)
    !35 = !{null, !9}
    !36 = !DILocalVariable(name: "foo", scope: !33, file: !2, line: 5, type: !9)
    !37 = !DILocation(line: 5, column: 8, scope: !33)
    !38 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !33, file: !2, line: 3, type: !34, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !31, retainedNodes: !5)
    !39 = !DILocalVariable(name: "foo", scope: !38, file: !2, line: 4, type: !9)
    !40 = !DILocation(line: 4, column: 8, scope: !38)
    !41 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 7, type: !42, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !31, retainedNodes: !5)
    !42 = !DISubroutineType(flags: DIFlagPublic, types: !43)
    !43 = !{null, !24}
    !44 = !DILocalVariable(name: "bar", scope: !41, file: !2, line: 8, type: !24)
    !45 = !DILocation(line: 8, column: 8, scope: !41)
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

    %__vtable_parent = type { ptr }
    %parent = type { ptr, i32 }
    %__vtable_child = type { ptr }
    %child = type { %parent, i32 }
    %__vtable_grandchild = type { ptr }
    %grandchild = type { %child, i32 }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer, !dbg !0
    @__parent__init = unnamed_addr constant %parent zeroinitializer, !dbg !6
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer, !dbg !17
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer, !dbg !19
    @__child__init = unnamed_addr constant %child zeroinitializer, !dbg !23
    @__vtable_child_instance = global %__vtable_child zeroinitializer, !dbg !30
    @____vtable_grandchild__init = unnamed_addr constant %__vtable_grandchild zeroinitializer, !dbg !32
    @__grandchild__init = unnamed_addr constant %grandchild zeroinitializer, !dbg !36
    @__vtable_grandchild_instance = global %__vtable_grandchild zeroinitializer, !dbg !43

    define void @parent(ptr %0) !dbg !49 {
    entry:
        #dbg_declare(ptr %0, !52, !DIExpression(), !53)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %a = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      ret void, !dbg !53
    }

    define void @child(ptr %0) !dbg !54 {
    entry:
        #dbg_declare(ptr %0, !57, !DIExpression(), !58)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      ret void, !dbg !58
    }

    define void @grandchild(ptr %0) !dbg !59 {
    entry:
        #dbg_declare(ptr %0, !62, !DIExpression(), !63)
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__child = getelementptr inbounds nuw %grandchild, ptr %0, i32 0, i32 0
      %c = getelementptr inbounds nuw %grandchild, ptr %0, i32 0, i32 1
      ret void, !dbg !63
    }

    define i32 @main() !dbg !64 {
    entry:
      %main = alloca i32, align 4
      %array_of_parent = alloca [3 x %parent], align 8
      %array_of_child = alloca [3 x %child], align 8
      %array_of_grandchild = alloca [3 x %grandchild], align 8
      %parent1 = alloca %parent, align 8
      %child1 = alloca %child, align 8
      %grandchild1 = alloca %grandchild, align 8
        #dbg_declare(ptr %array_of_parent, !67, !DIExpression(), !71)
      call void @llvm.memset.p0.i64(ptr align 1 %array_of_parent, i8 0, i64 ptrtoint (ptr getelementptr ([3 x %parent], ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %array_of_child, !72, !DIExpression(), !74)
      call void @llvm.memset.p0.i64(ptr align 1 %array_of_child, i8 0, i64 ptrtoint (ptr getelementptr ([3 x %child], ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %array_of_grandchild, !75, !DIExpression(), !77)
      call void @llvm.memset.p0.i64(ptr align 1 %array_of_grandchild, i8 0, i64 ptrtoint (ptr getelementptr ([3 x %grandchild], ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %parent1, !78, !DIExpression(), !79)
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %parent1, ptr align 1 @__parent__init, i64 ptrtoint (ptr getelementptr (%parent, ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %child1, !80, !DIExpression(), !81)
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %child1, ptr align 1 @__child__init, i64 ptrtoint (ptr getelementptr (%child, ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %grandchild1, !82, !DIExpression(), !83)
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %grandchild1, ptr align 1 @__grandchild__init, i64 ptrtoint (ptr getelementptr (%grandchild, ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %main, !84, !DIExpression(), !85)
      store i32 0, ptr %main, align 4
      call void @__init_parent(ptr %parent1), !dbg !86
      call void @__init_child(ptr %child1), !dbg !86
      call void @__init_grandchild(ptr %grandchild1), !dbg !86
      call void @__user_init_parent(ptr %parent1), !dbg !86
      call void @__user_init_child(ptr %child1), !dbg !86
      call void @__user_init_grandchild(ptr %grandchild1), !dbg !86
      %a = getelementptr inbounds nuw %parent, ptr %parent1, i32 0, i32 1, !dbg !87
      store i32 1, ptr %a, align 4, !dbg !87
      %__parent = getelementptr inbounds nuw %child, ptr %child1, i32 0, i32 0, !dbg !88
      %a1 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1, !dbg !88
      store i32 2, ptr %a1, align 4, !dbg !88
      %b = getelementptr inbounds nuw %child, ptr %child1, i32 0, i32 1, !dbg !89
      store i32 3, ptr %b, align 4, !dbg !89
      %__child = getelementptr inbounds nuw %grandchild, ptr %grandchild1, i32 0, i32 0, !dbg !90
      %__parent2 = getelementptr inbounds nuw %child, ptr %__child, i32 0, i32 0, !dbg !90
      %a3 = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 1, !dbg !90
      store i32 4, ptr %a3, align 4, !dbg !90
      %__child4 = getelementptr inbounds nuw %grandchild, ptr %grandchild1, i32 0, i32 0, !dbg !91
      %b5 = getelementptr inbounds nuw %child, ptr %__child4, i32 0, i32 1, !dbg !91
      store i32 5, ptr %b5, align 4, !dbg !91
      %c = getelementptr inbounds nuw %grandchild, ptr %grandchild1, i32 0, i32 1, !dbg !92
      store i32 6, ptr %c, align 4, !dbg !92
      %tmpVar = getelementptr inbounds [3 x %parent], ptr %array_of_parent, i32 0, i32 0, !dbg !93
      %a6 = getelementptr inbounds nuw %parent, ptr %tmpVar, i32 0, i32 1, !dbg !93
      store i32 7, ptr %a6, align 4, !dbg !93
      %tmpVar7 = getelementptr inbounds [3 x %child], ptr %array_of_child, i32 0, i32 0, !dbg !94
      %__parent8 = getelementptr inbounds nuw %child, ptr %tmpVar7, i32 0, i32 0, !dbg !94
      %a9 = getelementptr inbounds nuw %parent, ptr %__parent8, i32 0, i32 1, !dbg !94
      store i32 8, ptr %a9, align 4, !dbg !94
      %tmpVar10 = getelementptr inbounds [3 x %child], ptr %array_of_child, i32 0, i32 0, !dbg !95
      %b11 = getelementptr inbounds nuw %child, ptr %tmpVar10, i32 0, i32 1, !dbg !95
      store i32 9, ptr %b11, align 4, !dbg !95
      %tmpVar12 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 0, !dbg !96
      %__child13 = getelementptr inbounds nuw %grandchild, ptr %tmpVar12, i32 0, i32 0, !dbg !96
      %__parent14 = getelementptr inbounds nuw %child, ptr %__child13, i32 0, i32 0, !dbg !96
      %a15 = getelementptr inbounds nuw %parent, ptr %__parent14, i32 0, i32 1, !dbg !96
      store i32 10, ptr %a15, align 4, !dbg !96
      %tmpVar16 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 0, !dbg !97
      %__child17 = getelementptr inbounds nuw %grandchild, ptr %tmpVar16, i32 0, i32 0, !dbg !97
      %b18 = getelementptr inbounds nuw %child, ptr %__child17, i32 0, i32 1, !dbg !97
      store i32 11, ptr %b18, align 4, !dbg !97
      %tmpVar19 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 0, !dbg !98
      %c20 = getelementptr inbounds nuw %grandchild, ptr %tmpVar19, i32 0, i32 1, !dbg !98
      store i32 12, ptr %c20, align 4, !dbg !98
      %tmpVar21 = getelementptr inbounds [3 x %parent], ptr %array_of_parent, i32 0, i32 1, !dbg !99
      %a22 = getelementptr inbounds nuw %parent, ptr %tmpVar21, i32 0, i32 1, !dbg !99
      store i32 13, ptr %a22, align 4, !dbg !99
      %tmpVar23 = getelementptr inbounds [3 x %child], ptr %array_of_child, i32 0, i32 1, !dbg !100
      %__parent24 = getelementptr inbounds nuw %child, ptr %tmpVar23, i32 0, i32 0, !dbg !100
      %a25 = getelementptr inbounds nuw %parent, ptr %__parent24, i32 0, i32 1, !dbg !100
      store i32 14, ptr %a25, align 4, !dbg !100
      %tmpVar26 = getelementptr inbounds [3 x %child], ptr %array_of_child, i32 0, i32 1, !dbg !101
      %b27 = getelementptr inbounds nuw %child, ptr %tmpVar26, i32 0, i32 1, !dbg !101
      store i32 15, ptr %b27, align 4, !dbg !101
      %tmpVar28 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 1, !dbg !102
      %__child29 = getelementptr inbounds nuw %grandchild, ptr %tmpVar28, i32 0, i32 0, !dbg !102
      %__parent30 = getelementptr inbounds nuw %child, ptr %__child29, i32 0, i32 0, !dbg !102
      %a31 = getelementptr inbounds nuw %parent, ptr %__parent30, i32 0, i32 1, !dbg !102
      store i32 16, ptr %a31, align 4, !dbg !102
      %tmpVar32 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 1, !dbg !103
      %__child33 = getelementptr inbounds nuw %grandchild, ptr %tmpVar32, i32 0, i32 0, !dbg !103
      %b34 = getelementptr inbounds nuw %child, ptr %__child33, i32 0, i32 1, !dbg !103
      store i32 17, ptr %b34, align 4, !dbg !103
      %tmpVar35 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 1, !dbg !104
      %c36 = getelementptr inbounds nuw %grandchild, ptr %tmpVar35, i32 0, i32 1, !dbg !104
      store i32 18, ptr %c36, align 4, !dbg !104
      %tmpVar37 = getelementptr inbounds [3 x %parent], ptr %array_of_parent, i32 0, i32 2, !dbg !105
      %a38 = getelementptr inbounds nuw %parent, ptr %tmpVar37, i32 0, i32 1, !dbg !105
      store i32 19, ptr %a38, align 4, !dbg !105
      %tmpVar39 = getelementptr inbounds [3 x %child], ptr %array_of_child, i32 0, i32 2, !dbg !106
      %__parent40 = getelementptr inbounds nuw %child, ptr %tmpVar39, i32 0, i32 0, !dbg !106
      %a41 = getelementptr inbounds nuw %parent, ptr %__parent40, i32 0, i32 1, !dbg !106
      store i32 20, ptr %a41, align 4, !dbg !106
      %tmpVar42 = getelementptr inbounds [3 x %child], ptr %array_of_child, i32 0, i32 2, !dbg !107
      %b43 = getelementptr inbounds nuw %child, ptr %tmpVar42, i32 0, i32 1, !dbg !107
      store i32 21, ptr %b43, align 4, !dbg !107
      %tmpVar44 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 2, !dbg !108
      %__child45 = getelementptr inbounds nuw %grandchild, ptr %tmpVar44, i32 0, i32 0, !dbg !108
      %__parent46 = getelementptr inbounds nuw %child, ptr %__child45, i32 0, i32 0, !dbg !108
      %a47 = getelementptr inbounds nuw %parent, ptr %__parent46, i32 0, i32 1, !dbg !108
      store i32 22, ptr %a47, align 4, !dbg !108
      %tmpVar48 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 2, !dbg !109
      %__child49 = getelementptr inbounds nuw %grandchild, ptr %tmpVar48, i32 0, i32 0, !dbg !109
      %b50 = getelementptr inbounds nuw %child, ptr %__child49, i32 0, i32 1, !dbg !109
      store i32 23, ptr %b50, align 4, !dbg !109
      %tmpVar51 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 2, !dbg !110
      %c52 = getelementptr inbounds nuw %grandchild, ptr %tmpVar51, i32 0, i32 1, !dbg !110
      store i32 24, ptr %c52, align 4, !dbg !110
      %main_ret = load i32, ptr %main, align 4, !dbg !111
      ret i32 %main_ret, !dbg !111
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #1

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_grandchild(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0
      store ptr @grandchild, ptr %__body, align 8
      ret void
    }

    define void @__init_grandchild(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__child = getelementptr inbounds nuw %grandchild, ptr %deref, i32 0, i32 0
      call void @__init_child(ptr %__child)
      %deref1 = load ptr, ptr %self, align 8
      %__child2 = getelementptr inbounds nuw %grandchild, ptr %deref1, i32 0, i32 0
      %__parent = getelementptr inbounds nuw %child, ptr %__child2, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 0
      store ptr @__vtable_grandchild_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_grandchild(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init_grandchild(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__child = getelementptr inbounds nuw %grandchild, ptr %deref, i32 0, i32 0
      call void @__user_init_child(ptr %__child)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__init___vtable_grandchild(ptr @__vtable_grandchild_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_grandchild(ptr @__vtable_grandchild_instance)
      ret void
    }

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }

    !llvm.module.flags = !{!45, !46}
    !llvm.dbg.cu = !{!47}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "____vtable_parent__init", scope: !2, file: !2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_parent", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_parent")
    !5 = !{}
    !6 = !DIGlobalVariableExpression(var: !7, expr: !DIExpression())
    !7 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 2, type: !8, isLocal: false, isDefinition: true)
    !8 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !9)
    !9 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 2, size: 128, align: 64, flags: DIFlagPublic, elements: !10, identifier: "parent")
    !10 = !{!11, !15}
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !12, size: 64, align: 64, flags: DIFlagPublic)
    !12 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____parent___vtable", scope: !2, file: !2, baseType: !13, align: 64)
    !13 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__parent___vtable", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 1)
    !14 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 4, baseType: !16, size: 32, align: 32, offset: 64, flags: DIFlagPublic)
    !16 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !17 = !DIGlobalVariableExpression(var: !18, expr: !DIExpression())
    !18 = distinct !DIGlobalVariable(name: "__vtable_parent_instance", scope: !2, file: !2, type: !4, isLocal: false, isDefinition: true)
    !19 = !DIGlobalVariableExpression(var: !20, expr: !DIExpression())
    !20 = distinct !DIGlobalVariable(name: "____vtable_child__init", scope: !2, file: !2, type: !21, isLocal: false, isDefinition: true)
    !21 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !22)
    !22 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_child", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_child")
    !23 = !DIGlobalVariableExpression(var: !24, expr: !DIExpression())
    !24 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 8, type: !25, isLocal: false, isDefinition: true)
    !25 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !26)
    !26 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 8, size: 192, align: 64, flags: DIFlagPublic, elements: !27, identifier: "child")
    !27 = !{!28, !29}
    !28 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !2, file: !2, baseType: !9, size: 128, align: 64, flags: DIFlagPublic)
    !29 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 10, baseType: !16, size: 32, align: 32, offset: 128, flags: DIFlagPublic)
    !30 = !DIGlobalVariableExpression(var: !31, expr: !DIExpression())
    !31 = distinct !DIGlobalVariable(name: "__vtable_child_instance", scope: !2, file: !2, type: !22, isLocal: false, isDefinition: true)
    !32 = !DIGlobalVariableExpression(var: !33, expr: !DIExpression())
    !33 = distinct !DIGlobalVariable(name: "____vtable_grandchild__init", scope: !2, file: !2, type: !34, isLocal: false, isDefinition: true)
    !34 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !35)
    !35 = !DICompositeType(tag: DW_TAG_structure_type, name: "__vtable_grandchild", scope: !2, file: !2, flags: DIFlagFwdDecl, elements: !5, identifier: "__vtable_grandchild")
    !36 = !DIGlobalVariableExpression(var: !37, expr: !DIExpression())
    !37 = distinct !DIGlobalVariable(name: "__grandchild__init", scope: !2, file: !2, line: 14, type: !38, isLocal: false, isDefinition: true)
    !38 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !39)
    !39 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandchild", scope: !2, file: !2, line: 14, size: 256, align: 64, flags: DIFlagPublic, elements: !40, identifier: "grandchild")
    !40 = !{!41, !42}
    !41 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !2, file: !2, baseType: !26, size: 192, align: 64, flags: DIFlagPublic)
    !42 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 16, baseType: !16, size: 32, align: 32, offset: 192, flags: DIFlagPublic)
    !43 = !DIGlobalVariableExpression(var: !44, expr: !DIExpression())
    !44 = distinct !DIGlobalVariable(name: "__vtable_grandchild_instance", scope: !2, file: !2, type: !35, isLocal: false, isDefinition: true)
    !45 = !{i32 2, !"Dwarf Version", i32 5}
    !46 = !{i32 2, !"Debug Info Version", i32 3}
    !47 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !48, splitDebugInlining: false)
    !48 = !{!17, !0, !6, !30, !19, !23, !43, !32, !36}
    !49 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 2, type: !50, scopeLine: 6, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !47, retainedNodes: !5)
    !50 = !DISubroutineType(flags: DIFlagPublic, types: !51)
    !51 = !{null, !9}
    !52 = !DILocalVariable(name: "parent", scope: !49, file: !2, line: 6, type: !9)
    !53 = !DILocation(line: 6, scope: !49)
    !54 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 8, type: !55, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !47, retainedNodes: !5)
    !55 = !DISubroutineType(flags: DIFlagPublic, types: !56)
    !56 = !{null, !26}
    !57 = !DILocalVariable(name: "child", scope: !54, file: !2, line: 12, type: !26)
    !58 = !DILocation(line: 12, scope: !54)
    !59 = distinct !DISubprogram(name: "grandchild", linkageName: "grandchild", scope: !2, file: !2, line: 14, type: !60, scopeLine: 18, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !47, retainedNodes: !5)
    !60 = !DISubroutineType(flags: DIFlagPublic, types: !61)
    !61 = !{null, !39}
    !62 = !DILocalVariable(name: "grandchild", scope: !59, file: !2, line: 18, type: !39)
    !63 = !DILocation(line: 18, scope: !59)
    !64 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 20, type: !65, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !47, retainedNodes: !5)
    !65 = !DISubroutineType(flags: DIFlagPublic, types: !66)
    !66 = !{null}
    !67 = !DILocalVariable(name: "array_of_parent", scope: !64, file: !2, line: 22, type: !68, align: 64)
    !68 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 384, align: 64, elements: !69)
    !69 = !{!70}
    !70 = !DISubrange(count: 3, lowerBound: 0)
    !71 = !DILocation(line: 22, column: 4, scope: !64)
    !72 = !DILocalVariable(name: "array_of_child", scope: !64, file: !2, line: 23, type: !73, align: 64)
    !73 = !DICompositeType(tag: DW_TAG_array_type, baseType: !26, size: 576, align: 64, elements: !69)
    !74 = !DILocation(line: 23, column: 4, scope: !64)
    !75 = !DILocalVariable(name: "array_of_grandchild", scope: !64, file: !2, line: 24, type: !76, align: 64)
    !76 = !DICompositeType(tag: DW_TAG_array_type, baseType: !39, size: 768, align: 64, elements: !69)
    !77 = !DILocation(line: 24, column: 4, scope: !64)
    !78 = !DILocalVariable(name: "parent1", scope: !64, file: !2, line: 25, type: !9, align: 64)
    !79 = !DILocation(line: 25, column: 4, scope: !64)
    !80 = !DILocalVariable(name: "child1", scope: !64, file: !2, line: 26, type: !26, align: 64)
    !81 = !DILocation(line: 26, column: 4, scope: !64)
    !82 = !DILocalVariable(name: "grandchild1", scope: !64, file: !2, line: 27, type: !39, align: 64)
    !83 = !DILocation(line: 27, column: 4, scope: !64)
    !84 = !DILocalVariable(name: "main", scope: !64, file: !2, line: 20, type: !16, align: 32)
    !85 = !DILocation(line: 20, column: 9, scope: !64)
    !86 = !DILocation(line: 0, scope: !64)
    !87 = !DILocation(line: 30, column: 4, scope: !64)
    !88 = !DILocation(line: 31, column: 4, scope: !64)
    !89 = !DILocation(line: 32, column: 4, scope: !64)
    !90 = !DILocation(line: 33, column: 4, scope: !64)
    !91 = !DILocation(line: 34, column: 4, scope: !64)
    !92 = !DILocation(line: 35, column: 4, scope: !64)
    !93 = !DILocation(line: 37, column: 4, scope: !64)
    !94 = !DILocation(line: 38, column: 4, scope: !64)
    !95 = !DILocation(line: 39, column: 4, scope: !64)
    !96 = !DILocation(line: 40, column: 4, scope: !64)
    !97 = !DILocation(line: 41, column: 4, scope: !64)
    !98 = !DILocation(line: 42, column: 4, scope: !64)
    !99 = !DILocation(line: 43, column: 4, scope: !64)
    !100 = !DILocation(line: 44, column: 4, scope: !64)
    !101 = !DILocation(line: 45, column: 4, scope: !64)
    !102 = !DILocation(line: 46, column: 4, scope: !64)
    !103 = !DILocation(line: 47, column: 4, scope: !64)
    !104 = !DILocation(line: 48, column: 4, scope: !64)
    !105 = !DILocation(line: 49, column: 4, scope: !64)
    !106 = !DILocation(line: 50, column: 4, scope: !64)
    !107 = !DILocation(line: 51, column: 4, scope: !64)
    !108 = !DILocation(line: 52, column: 4, scope: !64)
    !109 = !DILocation(line: 53, column: 4, scope: !64)
    !110 = !DILocation(line: 54, column: 4, scope: !64)
    !111 = !DILocation(line: 56, scope: !64)
    "#);
}
