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
    %__vtable_bar = type { ptr }
    %foo = type { ptr, i16, [81 x i8], [11 x [81 x i8]] }
    %bar = type { %foo }

    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal___[ctor-hash]__ctor, ptr null }]

    define void @foo(ptr %0) !dbg !4 {
    entry:
        #dbg_declare(ptr %0, !26, !DIExpression(), !27)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %a = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %b = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      %c = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 3
      ret void, !dbg !28
    }

    define void @bar(ptr %0) !dbg !29 {
    entry:
        #dbg_declare(ptr %0, !35, !DIExpression(), !36)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__foo = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      ret void, !dbg !37
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !36
      store ptr %0, ptr %self, align [filtered], !dbg !36
      %deref = load ptr, ptr %self, align [filtered], !dbg !36
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0, !dbg !36
      call void @__foo___vtable__ctor(ptr %__vtable), !dbg !36
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !36
      %c = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 3, !dbg !36
      call void @__foo_c__ctor(ptr %c), !dbg !36
      %deref2 = load ptr, ptr %self, align [filtered], !dbg !36
      %__vtable3 = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 0, !dbg !36
      store ptr @__vtable_foo_instance, ptr %__vtable3, align [filtered], !dbg !36
      ret void, !dbg !36
    }

    define void @bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !36
      store ptr %0, ptr %self, align [filtered], !dbg !36
      %deref = load ptr, ptr %self, align [filtered], !dbg !36
      %__foo = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0, !dbg !36
      call void @foo__ctor(ptr %__foo), !dbg !36
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !36
      %__foo2 = getelementptr inbounds nuw %bar, ptr %deref1, i32 0, i32 0, !dbg !36
      %__vtable = getelementptr inbounds nuw %foo, ptr %__foo2, i32 0, i32 0, !dbg !36
      store ptr @__vtable_bar_instance, ptr %__vtable, align [filtered], !dbg !36
      ret void, !dbg !36
    }

    define void @__foo_c__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !36
      store ptr %0, ptr %self, align [filtered], !dbg !36
      ret void, !dbg !36
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !36
      store ptr %0, ptr %self, align [filtered], !dbg !36
      %deref = load ptr, ptr %self, align [filtered], !dbg !36
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0, !dbg !36
      call void @____vtable_foo___body__ctor(ptr %__body), !dbg !36
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !36
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0, !dbg !36
      store ptr @foo, ptr %__body2, align [filtered], !dbg !36
      ret void, !dbg !36
    }

    define void @__vtable_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !36
      store ptr %0, ptr %self, align [filtered], !dbg !36
      %deref = load ptr, ptr %self, align [filtered], !dbg !36
      %__body = getelementptr inbounds nuw %__vtable_bar, ptr %deref, i32 0, i32 0, !dbg !36
      call void @____vtable_bar___body__ctor(ptr %__body), !dbg !36
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !36
      %__body2 = getelementptr inbounds nuw %__vtable_bar, ptr %deref1, i32 0, i32 0, !dbg !36
      store ptr @bar, ptr %__body2, align [filtered], !dbg !36
      ret void, !dbg !36
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !36
      store ptr %0, ptr %self, align [filtered], !dbg !36
      ret void, !dbg !36
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !36
      store ptr %0, ptr %self, align [filtered], !dbg !36
      ret void, !dbg !36
    }

    define void @____vtable_bar___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !36
      store ptr %0, ptr %self, align [filtered], !dbg !36
      ret void, !dbg !36
    }

    define void @__unit___internal___[ctor-hash]__ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance), !dbg !36
      call void @__vtable_bar__ctor(ptr @__vtable_bar_instance), !dbg !36
      ret void, !dbg !36
    }

    !llvm.module.flags = !{!0, !1}
    !llvm.dbg.cu = !{!2}

    !0 = !{i32 2, !"Dwarf Version", i32 5}
    !1 = !{i32 2, !"Debug Info Version", i32 3}
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "<internal>", directory: "")
    !4 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !3, file: !3, line: 2, type: !5, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !25, keyInstructions: true)
    !5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
    !6 = !{null, !7}
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !3, file: !3, line: 2, size: 7872, align [filtered], flags: DIFlagPublic, elements: !8, identifier: "foo")
    !8 = !{!9, !13, !15, !21}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !3, file: !3, baseType: !10, size: 64, align [filtered], flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____foo___vtable", scope: !3, file: !3, baseType: !11, align [filtered])
    !11 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__foo___vtable", baseType: !12, size: 64, align [filtered], dwarfAddressSpace: 1)
    !12 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !13 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !3, file: !3, line: 4, baseType: !14, size: 16, align [filtered], offset: 64, flags: DIFlagPublic)
    !14 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !3, file: !3, line: 5, baseType: !16, size: 648, align [filtered], offset: 80, flags: DIFlagPublic)
    !16 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !3, file: !3, baseType: !17, align [filtered])
    !17 = !DICompositeType(tag: DW_TAG_array_type, baseType: !18, size: 648, align [filtered], elements: !19)
    !18 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !19 = !{!20}
    !20 = !DISubrange(count: 81, lowerBound: 0)
    !21 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !3, file: !3, line: 6, baseType: !22, size: 7128, align [filtered], offset: 728, flags: DIFlagPublic)
    !22 = !DICompositeType(tag: DW_TAG_array_type, baseType: !16, size: 7128, align [filtered], elements: !23)
    !23 = !{!24}
    !24 = !DISubrange(count: 11, lowerBound: 0)
    !25 = !{}
    !26 = !DILocalVariable(name: "foo", scope: !4, file: !3, line: 8, type: !7)
    !27 = !DILocation(line: 8, column: 8, scope: !4)
    !28 = !DILocation(line: 8, column: 8, scope: !4, atomGroup: 1, atomRank: 1)
    !29 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !3, file: !3, line: 10, type: !30, scopeLine: 11, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !25, keyInstructions: true)
    !30 = !DISubroutineType(flags: DIFlagPublic, types: !31)
    !31 = !{null, !32}
    !32 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !3, file: !3, line: 10, size: 7872, align [filtered], flags: DIFlagPublic, elements: !33, identifier: "bar")
    !33 = !{!34}
    !34 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !3, file: !3, baseType: !7, size: 7872, align [filtered], flags: DIFlagPublic)
    !35 = !DILocalVariable(name: "bar", scope: !29, file: !3, line: 11, type: !32)
    !36 = !DILocation(line: 11, column: 8, scope: !29)
    !37 = !DILocation(line: 11, column: 8, scope: !29, atomGroup: 1, atomRank: 1)
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
    %__vtable_fb2 = type { ptr }
    %__vtable_foo = type { ptr }
    %fb = type { ptr, i16, i16 }
    %fb2 = type { %fb }
    %foo = type { ptr, %fb2 }

    @__vtable_fb_instance = global %__vtable_fb zeroinitializer
    @__vtable_fb2_instance = global %__vtable_fb2 zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal___[ctor-hash]__ctor, ptr null }]

    define void @fb(ptr %0) !dbg !4 {
    entry:
        #dbg_declare(ptr %0, !17, !DIExpression(), !18)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 1
      %y = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 2
      ret void, !dbg !19
    }

    define void @fb2(ptr %0) !dbg !20 {
    entry:
        #dbg_declare(ptr %0, !26, !DIExpression(), !27)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__fb = getelementptr inbounds nuw %fb2, ptr %0, i32 0, i32 0
      ret void, !dbg !28
    }

    define void @foo(ptr %0) !dbg !29 {
    entry:
        #dbg_declare(ptr %0, !38, !DIExpression(), !39)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %myFb = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %__fb = getelementptr inbounds nuw %fb2, ptr %myFb, i32 0, i32 0, !dbg !39
      %x = getelementptr inbounds nuw %fb, ptr %__fb, i32 0, i32 1, !dbg !39
      store i16 1, ptr %x, align [filtered], !dbg !40
      ret void, !dbg !41
    }

    define void @fb__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !42
      store ptr %0, ptr %self, align [filtered], !dbg !42
      %deref = load ptr, ptr %self, align [filtered], !dbg !42
      %__vtable = getelementptr inbounds nuw %fb, ptr %deref, i32 0, i32 0, !dbg !42
      call void @__fb___vtable__ctor(ptr %__vtable), !dbg !42
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !42
      %__vtable2 = getelementptr inbounds nuw %fb, ptr %deref1, i32 0, i32 0, !dbg !42
      store ptr @__vtable_fb_instance, ptr %__vtable2, align [filtered], !dbg !42
      ret void, !dbg !42
    }

    define void @fb2__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !42
      store ptr %0, ptr %self, align [filtered], !dbg !42
      %deref = load ptr, ptr %self, align [filtered], !dbg !42
      %__fb = getelementptr inbounds nuw %fb2, ptr %deref, i32 0, i32 0, !dbg !42
      call void @fb__ctor(ptr %__fb), !dbg !42
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !42
      %__fb2 = getelementptr inbounds nuw %fb2, ptr %deref1, i32 0, i32 0, !dbg !42
      %__vtable = getelementptr inbounds nuw %fb, ptr %__fb2, i32 0, i32 0, !dbg !42
      store ptr @__vtable_fb2_instance, ptr %__vtable, align [filtered], !dbg !42
      ret void, !dbg !42
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !42
      store ptr %0, ptr %self, align [filtered], !dbg !42
      %deref = load ptr, ptr %self, align [filtered], !dbg !42
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0, !dbg !42
      call void @__foo___vtable__ctor(ptr %__vtable), !dbg !42
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !42
      %myFb = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 1, !dbg !42
      call void @fb2__ctor(ptr %myFb), !dbg !42
      %deref2 = load ptr, ptr %self, align [filtered], !dbg !42
      %__vtable3 = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 0, !dbg !42
      store ptr @__vtable_foo_instance, ptr %__vtable3, align [filtered], !dbg !42
      ret void, !dbg !42
    }

    define void @__vtable_fb__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !42
      store ptr %0, ptr %self, align [filtered], !dbg !42
      %deref = load ptr, ptr %self, align [filtered], !dbg !42
      %__body = getelementptr inbounds nuw %__vtable_fb, ptr %deref, i32 0, i32 0, !dbg !42
      call void @____vtable_fb___body__ctor(ptr %__body), !dbg !42
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !42
      %__body2 = getelementptr inbounds nuw %__vtable_fb, ptr %deref1, i32 0, i32 0, !dbg !42
      store ptr @fb, ptr %__body2, align [filtered], !dbg !42
      ret void, !dbg !42
    }

    define void @__vtable_fb2__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !42
      store ptr %0, ptr %self, align [filtered], !dbg !42
      %deref = load ptr, ptr %self, align [filtered], !dbg !42
      %__body = getelementptr inbounds nuw %__vtable_fb2, ptr %deref, i32 0, i32 0, !dbg !42
      call void @____vtable_fb2___body__ctor(ptr %__body), !dbg !42
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !42
      %__body2 = getelementptr inbounds nuw %__vtable_fb2, ptr %deref1, i32 0, i32 0, !dbg !42
      store ptr @fb2, ptr %__body2, align [filtered], !dbg !42
      ret void, !dbg !42
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !42
      store ptr %0, ptr %self, align [filtered], !dbg !42
      %deref = load ptr, ptr %self, align [filtered], !dbg !42
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0, !dbg !42
      call void @____vtable_foo___body__ctor(ptr %__body), !dbg !42
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !42
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0, !dbg !42
      store ptr @foo, ptr %__body2, align [filtered], !dbg !42
      ret void, !dbg !42
    }

    define void @__fb___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !42
      store ptr %0, ptr %self, align [filtered], !dbg !42
      ret void, !dbg !42
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !42
      store ptr %0, ptr %self, align [filtered], !dbg !42
      ret void, !dbg !42
    }

    define void @____vtable_fb___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !42
      store ptr %0, ptr %self, align [filtered], !dbg !42
      ret void, !dbg !42
    }

    define void @____vtable_fb2___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !42
      store ptr %0, ptr %self, align [filtered], !dbg !42
      ret void, !dbg !42
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !42
      store ptr %0, ptr %self, align [filtered], !dbg !42
      ret void, !dbg !42
    }

    define void @__unit___internal___[ctor-hash]__ctor() {
    entry:
      call void @__vtable_fb__ctor(ptr @__vtable_fb_instance), !dbg !42
      call void @__vtable_fb2__ctor(ptr @__vtable_fb2_instance), !dbg !42
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance), !dbg !42
      ret void, !dbg !42
    }

    !llvm.module.flags = !{!0, !1}
    !llvm.dbg.cu = !{!2}

    !0 = !{i32 2, !"Dwarf Version", i32 5}
    !1 = !{i32 2, !"Debug Info Version", i32 3}
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "<internal>", directory: "")
    !4 = distinct !DISubprogram(name: "fb", linkageName: "fb", scope: !3, file: !3, line: 2, type: !5, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !16, keyInstructions: true)
    !5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
    !6 = !{null, !7}
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb", scope: !3, file: !3, line: 2, size: 128, align [filtered], flags: DIFlagPublic, elements: !8, identifier: "fb")
    !8 = !{!9, !13, !15}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !3, file: !3, baseType: !10, size: 64, align [filtered], flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____fb___vtable", scope: !3, file: !3, baseType: !11, align [filtered])
    !11 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__fb___vtable", baseType: !12, size: 64, align [filtered], dwarfAddressSpace: 1)
    !12 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !13 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !3, file: !3, line: 4, baseType: !14, size: 16, align [filtered], offset: 64, flags: DIFlagPublic)
    !14 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !3, file: !3, line: 5, baseType: !14, size: 16, align [filtered], offset: 80, flags: DIFlagPublic)
    !16 = !{}
    !17 = !DILocalVariable(name: "fb", scope: !4, file: !3, line: 7, type: !7)
    !18 = !DILocation(line: 7, column: 8, scope: !4)
    !19 = !DILocation(line: 7, column: 8, scope: !4, atomGroup: 1, atomRank: 1)
    !20 = distinct !DISubprogram(name: "fb2", linkageName: "fb2", scope: !3, file: !3, line: 9, type: !21, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !16, keyInstructions: true)
    !21 = !DISubroutineType(flags: DIFlagPublic, types: !22)
    !22 = !{null, !23}
    !23 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb2", scope: !3, file: !3, line: 9, size: 128, align [filtered], flags: DIFlagPublic, elements: !24, identifier: "fb2")
    !24 = !{!25}
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !3, file: !3, baseType: !7, size: 128, align [filtered], flags: DIFlagPublic)
    !26 = !DILocalVariable(name: "fb2", scope: !20, file: !3, line: 10, type: !23)
    !27 = !DILocation(line: 10, column: 8, scope: !20)
    !28 = !DILocation(line: 10, column: 8, scope: !20, atomGroup: 1, atomRank: 1)
    !29 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !3, file: !3, line: 12, type: !30, scopeLine: 16, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !16, keyInstructions: true)
    !30 = !DISubroutineType(flags: DIFlagPublic, types: !31)
    !31 = !{null, !32}
    !32 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !3, file: !3, line: 12, size: 192, align [filtered], flags: DIFlagPublic, elements: !33, identifier: "foo")
    !33 = !{!34, !37}
    !34 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !3, file: !3, baseType: !35, size: 64, align [filtered], flags: DIFlagPublic)
    !35 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____foo___vtable", scope: !3, file: !3, baseType: !36, align [filtered])
    !36 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__foo___vtable", baseType: !12, size: 64, align [filtered], dwarfAddressSpace: 1)
    !37 = !DIDerivedType(tag: DW_TAG_member, name: "myFb", scope: !3, file: !3, line: 14, baseType: !23, size: 128, align [filtered], offset: 64, flags: DIFlagPublic)
    !38 = !DILocalVariable(name: "foo", scope: !29, file: !3, line: 16, type: !32)
    !39 = !DILocation(line: 16, column: 12, scope: !29)
    !40 = !DILocation(line: 16, column: 12, scope: !29, atomGroup: 1, atomRank: 1)
    !41 = !DILocation(line: 17, column: 8, scope: !29, atomGroup: 2, atomRank: 1)
    !42 = !DILocation(line: 17, column: 8, scope: !29)
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
    %__vtable_bar = type { ptr, ptr }
    %foo = type { ptr, [81 x i8] }
    %bar = type { %foo }

    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal___[ctor-hash]__ctor, ptr null }]
    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"world\00"

    define void @foo(ptr %0) !dbg !4 {
    entry:
        #dbg_declare(ptr %0, !20, !DIExpression(), !21)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      ret void, !dbg !22
    }

    define void @foo__baz(ptr %0) !dbg !23 {
    entry:
        #dbg_declare(ptr %0, !24, !DIExpression(), !25)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %s, ptr align [filtered] @utf08_literal_0, i32 6, i1 false), !dbg !26
      ret void, !dbg !27
    }

    define void @bar(ptr %0) !dbg !28 {
    entry:
        #dbg_declare(ptr %0, !34, !DIExpression(), !35)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__foo = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %__foo, i32 0, i32 1, !dbg !35
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %s, ptr align [filtered] @utf08_literal_1, i32 6, i1 false), !dbg !36
      ret void, !dbg !37
    }

    define void @main() !dbg !38 {
    entry:
      %s = alloca [81 x i8], align [filtered]
      %fb = alloca %bar, align [filtered]
        #dbg_declare(ptr %s, !41, !DIExpression(), !42)
      call void @llvm.memset.p0.i64(ptr align [filtered] %s, i8 0, i64 ptrtoint (ptr getelementptr ([81 x i8], ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %fb, !43, !DIExpression(), !44)
      call void @llvm.memset.p0.i64(ptr align [filtered] %fb, i8 0, i64 ptrtoint (ptr getelementptr (%bar, ptr null, i32 1) to i64), i1 false)
      call void @bar__ctor(ptr %fb), !dbg !45
      %__foo = getelementptr inbounds nuw %bar, ptr %fb, i32 0, i32 0, !dbg !46
      call void @foo__baz(ptr %__foo), !dbg !47
      call void @bar(ptr %fb), !dbg !48
      ret void, !dbg !49
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !50
      store ptr %0, ptr %self, align [filtered], !dbg !50
      %deref = load ptr, ptr %self, align [filtered], !dbg !50
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0, !dbg !50
      call void @__foo___vtable__ctor(ptr %__vtable), !dbg !50
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !50
      %__vtable2 = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 0, !dbg !50
      store ptr @__vtable_foo_instance, ptr %__vtable2, align [filtered], !dbg !50
      ret void, !dbg !50
    }

    define void @bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !50
      store ptr %0, ptr %self, align [filtered], !dbg !50
      %deref = load ptr, ptr %self, align [filtered], !dbg !50
      %__foo = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0, !dbg !50
      call void @foo__ctor(ptr %__foo), !dbg !50
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !50
      %__foo2 = getelementptr inbounds nuw %bar, ptr %deref1, i32 0, i32 0, !dbg !50
      %__vtable = getelementptr inbounds nuw %foo, ptr %__foo2, i32 0, i32 0, !dbg !50
      store ptr @__vtable_bar_instance, ptr %__vtable, align [filtered], !dbg !50
      ret void, !dbg !50
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !50
      store ptr %0, ptr %self, align [filtered], !dbg !50
      %deref = load ptr, ptr %self, align [filtered], !dbg !50
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0, !dbg !50
      call void @____vtable_foo___body__ctor(ptr %__body), !dbg !50
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !50
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0, !dbg !50
      store ptr @foo, ptr %__body2, align [filtered], !dbg !50
      %deref3 = load ptr, ptr %self, align [filtered], !dbg !50
      %baz = getelementptr inbounds nuw %__vtable_foo, ptr %deref3, i32 0, i32 1, !dbg !50
      call void @____vtable_foo_baz__ctor(ptr %baz), !dbg !50
      %deref4 = load ptr, ptr %self, align [filtered], !dbg !50
      %baz5 = getelementptr inbounds nuw %__vtable_foo, ptr %deref4, i32 0, i32 1, !dbg !50
      store ptr @foo__baz, ptr %baz5, align [filtered], !dbg !50
      ret void, !dbg !50
    }

    define void @__vtable_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !50
      store ptr %0, ptr %self, align [filtered], !dbg !50
      %deref = load ptr, ptr %self, align [filtered], !dbg !50
      %__body = getelementptr inbounds nuw %__vtable_bar, ptr %deref, i32 0, i32 0, !dbg !50
      call void @____vtable_bar___body__ctor(ptr %__body), !dbg !50
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !50
      %__body2 = getelementptr inbounds nuw %__vtable_bar, ptr %deref1, i32 0, i32 0, !dbg !50
      store ptr @bar, ptr %__body2, align [filtered], !dbg !50
      %deref3 = load ptr, ptr %self, align [filtered], !dbg !50
      %baz = getelementptr inbounds nuw %__vtable_bar, ptr %deref3, i32 0, i32 1, !dbg !50
      call void @____vtable_bar_baz__ctor(ptr %baz), !dbg !50
      %deref4 = load ptr, ptr %self, align [filtered], !dbg !50
      %baz5 = getelementptr inbounds nuw %__vtable_bar, ptr %deref4, i32 0, i32 1, !dbg !50
      store ptr @foo__baz, ptr %baz5, align [filtered], !dbg !50
      ret void, !dbg !50
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !50
      store ptr %0, ptr %self, align [filtered], !dbg !50
      ret void, !dbg !50
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !50
      store ptr %0, ptr %self, align [filtered], !dbg !50
      ret void, !dbg !50
    }

    define void @____vtable_foo_baz__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !50
      store ptr %0, ptr %self, align [filtered], !dbg !50
      ret void, !dbg !50
    }

    define void @____vtable_bar___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !50
      store ptr %0, ptr %self, align [filtered], !dbg !50
      ret void, !dbg !50
    }

    define void @____vtable_bar_baz__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !50
      store ptr %0, ptr %self, align [filtered], !dbg !50
      ret void, !dbg !50
    }

    define void @__unit___internal___[ctor-hash]__ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance), !dbg !50
      call void @__vtable_bar__ctor(ptr @__vtable_bar_instance), !dbg !50
      ret void, !dbg !50
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #1

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: write) }

    !llvm.module.flags = !{!0, !1}
    !llvm.dbg.cu = !{!2}

    !0 = !{i32 2, !"Dwarf Version", i32 5}
    !1 = !{i32 2, !"Debug Info Version", i32 3}
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "<internal>", directory: "")
    !4 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !3, file: !3, line: 2, type: !5, scopeLine: 9, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !19, keyInstructions: true)
    !5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
    !6 = !{null, !7}
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !3, file: !3, line: 2, size: 768, align [filtered], flags: DIFlagPublic, elements: !8, identifier: "foo")
    !8 = !{!9, !13}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !3, file: !3, baseType: !10, size: 64, align [filtered], flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____foo___vtable", scope: !3, file: !3, baseType: !11, align [filtered])
    !11 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__foo___vtable", baseType: !12, size: 64, align [filtered], dwarfAddressSpace: 1)
    !12 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !13 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !3, file: !3, line: 4, baseType: !14, size: 648, align [filtered], offset: 64, flags: DIFlagPublic)
    !14 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !3, file: !3, baseType: !15, align [filtered])
    !15 = !DICompositeType(tag: DW_TAG_array_type, baseType: !16, size: 648, align [filtered], elements: !17)
    !16 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !17 = !{!18}
    !18 = !DISubrange(count: 81, lowerBound: 0)
    !19 = !{}
    !20 = !DILocalVariable(name: "foo", scope: !4, file: !3, line: 9, type: !7)
    !21 = !DILocation(line: 9, column: 8, scope: !4)
    !22 = !DILocation(line: 9, column: 8, scope: !4, atomGroup: 1, atomRank: 1)
    !23 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !4, file: !3, line: 6, type: !5, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !19, keyInstructions: true)
    !24 = !DILocalVariable(name: "foo", scope: !23, file: !3, line: 7, type: !7)
    !25 = !DILocation(line: 7, column: 12, scope: !23)
    !26 = !DILocation(line: 7, column: 12, scope: !23, atomGroup: 1, atomRank: 1)
    !27 = !DILocation(line: 8, column: 8, scope: !23, atomGroup: 2, atomRank: 1)
    !28 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !3, file: !3, line: 11, type: !29, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !19, keyInstructions: true)
    !29 = !DISubroutineType(flags: DIFlagPublic, types: !30)
    !30 = !{null, !31}
    !31 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !3, file: !3, line: 11, size: 768, align [filtered], flags: DIFlagPublic, elements: !32, identifier: "bar")
    !32 = !{!33}
    !33 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !3, file: !3, baseType: !7, size: 768, align [filtered], flags: DIFlagPublic)
    !34 = !DILocalVariable(name: "bar", scope: !28, file: !3, line: 12, type: !31)
    !35 = !DILocation(line: 12, column: 12, scope: !28)
    !36 = !DILocation(line: 12, column: 12, scope: !28, atomGroup: 1, atomRank: 1)
    !37 = !DILocation(line: 13, column: 8, scope: !28, atomGroup: 2, atomRank: 1)
    !38 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !3, file: !3, line: 15, type: !39, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !19, keyInstructions: true)
    !39 = !DISubroutineType(flags: DIFlagPublic, types: !40)
    !40 = !{null}
    !41 = !DILocalVariable(name: "s", scope: !38, file: !3, line: 17, type: !14, align [filtered])
    !42 = !DILocation(line: 17, column: 12, scope: !38)
    !43 = !DILocalVariable(name: "fb", scope: !38, file: !3, line: 18, type: !31, align [filtered])
    !44 = !DILocation(line: 18, column: 12, scope: !38)
    !45 = !DILocation(line: 0, scope: !38, atomGroup: 1, atomRank: 1)
    !46 = !DILocation(line: 20, column: 12, scope: !38)
    !47 = !DILocation(line: 20, column: 12, scope: !38, atomGroup: 2, atomRank: 1)
    !48 = !DILocation(line: 21, column: 12, scope: !38, atomGroup: 3, atomRank: 1)
    !49 = !DILocation(line: 22, column: 8, scope: !38, atomGroup: 4, atomRank: 1)
    !50 = !DILocation(line: 22, column: 8, scope: !38)
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
    %__vtable_parent = type { ptr }
    %__vtable_child = type { ptr }
    %grandparent = type { ptr, [6 x i16], i16 }
    %parent = type { %grandparent, [11 x i16], i16 }
    %child = type { %parent, [11 x i16] }

    @__vtable_grandparent_instance = global %__vtable_grandparent zeroinitializer
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal___[ctor-hash]__ctor, ptr null }]

    define void @grandparent(ptr %0) !dbg !4 {
    entry:
        #dbg_declare(ptr %0, !20, !DIExpression(), !21)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 0
      %y = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 1
      %a = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 2
      ret void, !dbg !22
    }

    define void @parent(ptr %0) !dbg !23 {
    entry:
        #dbg_declare(ptr %0, !34, !DIExpression(), !35)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__grandparent = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %b = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void, !dbg !36
    }

    define void @child(ptr %0) !dbg !37 {
    entry:
        #dbg_declare(ptr %0, !44, !DIExpression(), !45)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %z = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      ret void, !dbg !46
    }

    define void @main() !dbg !47 {
    entry:
      %arr = alloca [11 x %child], align [filtered]
        #dbg_declare(ptr %arr, !50, !DIExpression(), !52)
      call void @llvm.memset.p0.i64(ptr align [filtered] %arr, i8 0, i64 ptrtoint (ptr getelementptr ([11 x %child], ptr null, i32 1) to i64), i1 false)
      call void @__main_arr__ctor(ptr %arr), !dbg !53
      %tmpVar = getelementptr inbounds [11 x %child], ptr %arr, i32 0, i32 0, !dbg !54
      %__parent = getelementptr inbounds nuw %child, ptr %tmpVar, i32 0, i32 0, !dbg !54
      %__grandparent = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 0, !dbg !54
      %a = getelementptr inbounds nuw %grandparent, ptr %__grandparent, i32 0, i32 2, !dbg !54
      store i16 10, ptr %a, align [filtered], !dbg !55
      %tmpVar1 = getelementptr inbounds [11 x %child], ptr %arr, i32 0, i32 0, !dbg !56
      %__parent2 = getelementptr inbounds nuw %child, ptr %tmpVar1, i32 0, i32 0, !dbg !56
      %__grandparent3 = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0, !dbg !56
      %y = getelementptr inbounds nuw %grandparent, ptr %__grandparent3, i32 0, i32 1, !dbg !56
      %tmpVar4 = getelementptr inbounds [6 x i16], ptr %y, i32 0, i32 0, !dbg !56
      store i16 20, ptr %tmpVar4, align [filtered], !dbg !57
      %tmpVar5 = getelementptr inbounds [11 x %child], ptr %arr, i32 0, i32 1, !dbg !58
      %__parent6 = getelementptr inbounds nuw %child, ptr %tmpVar5, i32 0, i32 0, !dbg !58
      %b = getelementptr inbounds nuw %parent, ptr %__parent6, i32 0, i32 2, !dbg !58
      store i16 30, ptr %b, align [filtered], !dbg !59
      %tmpVar7 = getelementptr inbounds [11 x %child], ptr %arr, i32 0, i32 1, !dbg !60
      %__parent8 = getelementptr inbounds nuw %child, ptr %tmpVar7, i32 0, i32 0, !dbg !60
      %x = getelementptr inbounds nuw %parent, ptr %__parent8, i32 0, i32 1, !dbg !60
      %tmpVar9 = getelementptr inbounds [11 x i16], ptr %x, i32 0, i32 1, !dbg !60
      store i16 40, ptr %tmpVar9, align [filtered], !dbg !61
      %tmpVar10 = getelementptr inbounds [11 x %child], ptr %arr, i32 0, i32 2, !dbg !62
      %z = getelementptr inbounds nuw %child, ptr %tmpVar10, i32 0, i32 1, !dbg !62
      %tmpVar11 = getelementptr inbounds [11 x i16], ptr %z, i32 0, i32 2, !dbg !62
      store i16 50, ptr %tmpVar11, align [filtered], !dbg !63
      ret void, !dbg !64
    }

    define void @grandparent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !65
      store ptr %0, ptr %self, align [filtered], !dbg !65
      %deref = load ptr, ptr %self, align [filtered], !dbg !65
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %deref, i32 0, i32 0, !dbg !65
      call void @__grandparent___vtable__ctor(ptr %__vtable), !dbg !65
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !65
      %y = getelementptr inbounds nuw %grandparent, ptr %deref1, i32 0, i32 1, !dbg !65
      call void @__grandparent_y__ctor(ptr %y), !dbg !65
      %deref2 = load ptr, ptr %self, align [filtered], !dbg !65
      %__vtable3 = getelementptr inbounds nuw %grandparent, ptr %deref2, i32 0, i32 0, !dbg !65
      store ptr @__vtable_grandparent_instance, ptr %__vtable3, align [filtered], !dbg !65
      ret void, !dbg !65
    }

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !65
      store ptr %0, ptr %self, align [filtered], !dbg !65
      %deref = load ptr, ptr %self, align [filtered], !dbg !65
      %__grandparent = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0, !dbg !65
      call void @grandparent__ctor(ptr %__grandparent), !dbg !65
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !65
      %x = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 1, !dbg !65
      call void @__parent_x__ctor(ptr %x), !dbg !65
      %deref2 = load ptr, ptr %self, align [filtered], !dbg !65
      %__grandparent3 = getelementptr inbounds nuw %parent, ptr %deref2, i32 0, i32 0, !dbg !65
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent3, i32 0, i32 0, !dbg !65
      store ptr @__vtable_parent_instance, ptr %__vtable, align [filtered], !dbg !65
      ret void, !dbg !65
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !65
      store ptr %0, ptr %self, align [filtered], !dbg !65
      %deref = load ptr, ptr %self, align [filtered], !dbg !65
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0, !dbg !65
      call void @parent__ctor(ptr %__parent), !dbg !65
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !65
      %z = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 1, !dbg !65
      call void @__child_z__ctor(ptr %z), !dbg !65
      %deref2 = load ptr, ptr %self, align [filtered], !dbg !65
      %__parent3 = getelementptr inbounds nuw %child, ptr %deref2, i32 0, i32 0, !dbg !65
      %__grandparent = getelementptr inbounds nuw %parent, ptr %__parent3, i32 0, i32 0, !dbg !65
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent, i32 0, i32 0, !dbg !65
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered], !dbg !65
      ret void, !dbg !65
    }

    define void @__grandparent_y__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !65
      store ptr %0, ptr %self, align [filtered], !dbg !65
      ret void, !dbg !65
    }

    define void @__parent_x__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !65
      store ptr %0, ptr %self, align [filtered], !dbg !65
      ret void, !dbg !65
    }

    define void @__child_z__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !65
      store ptr %0, ptr %self, align [filtered], !dbg !65
      ret void, !dbg !65
    }

    define void @__main_arr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !65
      store ptr %0, ptr %self, align [filtered], !dbg !65
      %__main_arr__idx0 = alloca i32, align [filtered]
      store i32 0, ptr %__main_arr__idx0, align [filtered], !dbg !65
      store i32 0, ptr %__main_arr__idx0, align [filtered], !dbg !65
      br label %while_body, !dbg !65

    while_body:                                       ; preds = %continue1, %entry
      %load___main_arr__idx0 = load i32, ptr %__main_arr__idx0, align [filtered], !dbg !65
      %tmpVar = icmp sgt i32 %load___main_arr__idx0, 10, !dbg !65
      %1 = zext i1 %tmpVar to i8, !dbg !65
      %2 = icmp ne i8 %1, 0, !dbg !65
      br i1 %2, label %condition_body, label %continue1, !dbg !65

    continue:                                         ; preds = %condition_body
      ret void, !dbg !65

    condition_body:                                   ; preds = %while_body
      br label %continue, !dbg !65

    buffer_block:                                     ; No predecessors!
      br label %continue1, !dbg !65

    continue1:                                        ; preds = %buffer_block, %while_body
      %deref = load ptr, ptr %self, align [filtered], !dbg !65
      %load___main_arr__idx02 = load i32, ptr %__main_arr__idx0, align [filtered], !dbg !65
      %tmpVar3 = mul i32 1, %load___main_arr__idx02, !dbg !65
      %tmpVar4 = add i32 %tmpVar3, 0, !dbg !65
      %tmpVar5 = getelementptr inbounds [11 x %child], ptr %deref, i32 0, i32 %tmpVar4, !dbg !65
      call void @child__ctor(ptr %tmpVar5), !dbg !65
      %load___main_arr__idx06 = load i32, ptr %__main_arr__idx0, align [filtered], !dbg !65
      %tmpVar7 = add i32 %load___main_arr__idx06, 1, !dbg !65
      store i32 %tmpVar7, ptr %__main_arr__idx0, align [filtered], !dbg !65
      br label %while_body, !dbg !65
    }

    define void @__vtable_grandparent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !65
      store ptr %0, ptr %self, align [filtered], !dbg !65
      %deref = load ptr, ptr %self, align [filtered], !dbg !65
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0, !dbg !65
      call void @____vtable_grandparent___body__ctor(ptr %__body), !dbg !65
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !65
      %__body2 = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref1, i32 0, i32 0, !dbg !65
      store ptr @grandparent, ptr %__body2, align [filtered], !dbg !65
      ret void, !dbg !65
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !65
      store ptr %0, ptr %self, align [filtered], !dbg !65
      %deref = load ptr, ptr %self, align [filtered], !dbg !65
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0, !dbg !65
      call void @____vtable_parent___body__ctor(ptr %__body), !dbg !65
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !65
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0, !dbg !65
      store ptr @parent, ptr %__body2, align [filtered], !dbg !65
      ret void, !dbg !65
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !65
      store ptr %0, ptr %self, align [filtered], !dbg !65
      %deref = load ptr, ptr %self, align [filtered], !dbg !65
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0, !dbg !65
      call void @____vtable_child___body__ctor(ptr %__body), !dbg !65
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !65
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0, !dbg !65
      store ptr @child, ptr %__body2, align [filtered], !dbg !65
      ret void, !dbg !65
    }

    define void @__grandparent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !65
      store ptr %0, ptr %self, align [filtered], !dbg !65
      ret void, !dbg !65
    }

    define void @____vtable_grandparent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !65
      store ptr %0, ptr %self, align [filtered], !dbg !65
      ret void, !dbg !65
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !65
      store ptr %0, ptr %self, align [filtered], !dbg !65
      ret void, !dbg !65
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !65
      store ptr %0, ptr %self, align [filtered], !dbg !65
      ret void, !dbg !65
    }

    define void @__unit___internal___[ctor-hash]__ctor() {
    entry:
      call void @__vtable_grandparent__ctor(ptr @__vtable_grandparent_instance), !dbg !65
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance), !dbg !65
      call void @__vtable_child__ctor(ptr @__vtable_child_instance), !dbg !65
      ret void, !dbg !65
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }

    !llvm.module.flags = !{!0, !1}
    !llvm.dbg.cu = !{!2}

    !0 = !{i32 2, !"Dwarf Version", i32 5}
    !1 = !{i32 2, !"Debug Info Version", i32 3}
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "<internal>", directory: "")
    !4 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !3, file: !3, line: 2, type: !5, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !19, keyInstructions: true)
    !5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
    !6 = !{null, !7}
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !3, file: !3, line: 2, size: 192, align [filtered], flags: DIFlagPublic, elements: !8, identifier: "grandparent")
    !8 = !{!9, !13, !18}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !3, file: !3, baseType: !10, size: 64, align [filtered], flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____grandparent___vtable", scope: !3, file: !3, baseType: !11, align [filtered])
    !11 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__grandparent___vtable", baseType: !12, size: 64, align [filtered], dwarfAddressSpace: 1)
    !12 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !13 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !3, file: !3, line: 4, baseType: !14, size: 96, align [filtered], offset: 64, flags: DIFlagPublic)
    !14 = !DICompositeType(tag: DW_TAG_array_type, baseType: !15, size: 96, align [filtered], elements: !16)
    !15 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !16 = !{!17}
    !17 = !DISubrange(count: 6, lowerBound: 0)
    !18 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !3, file: !3, line: 5, baseType: !15, size: 16, align [filtered], offset: 160, flags: DIFlagPublic)
    !19 = !{}
    !20 = !DILocalVariable(name: "grandparent", scope: !4, file: !3, line: 7, type: !7)
    !21 = !DILocation(line: 7, column: 8, scope: !4)
    !22 = !DILocation(line: 7, column: 8, scope: !4, atomGroup: 1, atomRank: 1)
    !23 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !3, file: !3, line: 9, type: !24, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !19, keyInstructions: true)
    !24 = !DISubroutineType(flags: DIFlagPublic, types: !25)
    !25 = !{null, !26}
    !26 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !3, file: !3, line: 9, size: 384, align [filtered], flags: DIFlagPublic, elements: !27, identifier: "parent")
    !27 = !{!28, !29, !33}
    !28 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !3, file: !3, baseType: !7, size: 192, align [filtered], flags: DIFlagPublic)
    !29 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !3, file: !3, line: 11, baseType: !30, size: 176, align [filtered], offset: 192, flags: DIFlagPublic)
    !30 = !DICompositeType(tag: DW_TAG_array_type, baseType: !15, size: 176, align [filtered], elements: !31)
    !31 = !{!32}
    !32 = !DISubrange(count: 11, lowerBound: 0)
    !33 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !3, file: !3, line: 12, baseType: !15, size: 16, align [filtered], offset: 368, flags: DIFlagPublic)
    !34 = !DILocalVariable(name: "parent", scope: !23, file: !3, line: 14, type: !26)
    !35 = !DILocation(line: 14, column: 8, scope: !23)
    !36 = !DILocation(line: 14, column: 8, scope: !23, atomGroup: 1, atomRank: 1)
    !37 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !3, file: !3, line: 16, type: !38, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !19, keyInstructions: true)
    !38 = !DISubroutineType(flags: DIFlagPublic, types: !39)
    !39 = !{null, !40}
    !40 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !3, file: !3, line: 16, size: 576, align [filtered], flags: DIFlagPublic, elements: !41, identifier: "child")
    !41 = !{!42, !43}
    !42 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !3, file: !3, baseType: !26, size: 384, align [filtered], flags: DIFlagPublic)
    !43 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !3, file: !3, line: 18, baseType: !30, size: 176, align [filtered], offset: 384, flags: DIFlagPublic)
    !44 = !DILocalVariable(name: "child", scope: !37, file: !3, line: 20, type: !40)
    !45 = !DILocation(line: 20, column: 8, scope: !37)
    !46 = !DILocation(line: 20, column: 8, scope: !37, atomGroup: 1, atomRank: 1)
    !47 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !3, file: !3, line: 22, type: !48, scopeLine: 26, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !19, keyInstructions: true)
    !48 = !DISubroutineType(flags: DIFlagPublic, types: !49)
    !49 = !{null}
    !50 = !DILocalVariable(name: "arr", scope: !47, file: !3, line: 24, type: !51, align [filtered])
    !51 = !DICompositeType(tag: DW_TAG_array_type, baseType: !40, size: 6336, align [filtered], elements: !31)
    !52 = !DILocation(line: 24, column: 12, scope: !47)
    !53 = !DILocation(line: 0, scope: !47, atomGroup: 1, atomRank: 1)
    !54 = !DILocation(line: 26, column: 12, scope: !47)
    !55 = !DILocation(line: 26, column: 12, scope: !47, atomGroup: 2, atomRank: 1)
    !56 = !DILocation(line: 27, column: 12, scope: !47)
    !57 = !DILocation(line: 27, column: 12, scope: !47, atomGroup: 3, atomRank: 1)
    !58 = !DILocation(line: 28, column: 12, scope: !47)
    !59 = !DILocation(line: 28, column: 12, scope: !47, atomGroup: 4, atomRank: 1)
    !60 = !DILocation(line: 29, column: 12, scope: !47)
    !61 = !DILocation(line: 29, column: 12, scope: !47, atomGroup: 5, atomRank: 1)
    !62 = !DILocation(line: 30, column: 12, scope: !47)
    !63 = !DILocation(line: 30, column: 12, scope: !47, atomGroup: 6, atomRank: 1)
    !64 = !DILocation(line: 31, column: 8, scope: !47, atomGroup: 7, atomRank: 1)
    !65 = !DILocation(line: 31, column: 8, scope: !47)
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
    %__vtable_parent = type { ptr }
    %__vtable_child = type { ptr }
    %grandparent = type { ptr, [6 x i16], i16 }
    %parent = type { %grandparent, [11 x i16], i16 }
    %child = type { %parent, [11 x i16] }

    @__vtable_grandparent_instance = global %__vtable_grandparent zeroinitializer
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal___[ctor-hash]__ctor, ptr null }]

    define void @grandparent(ptr %0) !dbg !4 {
    entry:
        #dbg_declare(ptr %0, !20, !DIExpression(), !21)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 0
      %y = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 1
      %a = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 2
      ret void, !dbg !22
    }

    define void @parent(ptr %0) !dbg !23 {
    entry:
        #dbg_declare(ptr %0, !34, !DIExpression(), !35)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__grandparent = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %b = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void, !dbg !36
    }

    define void @child(ptr %0) !dbg !37 {
    entry:
        #dbg_declare(ptr %0, !44, !DIExpression(), !45)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %z = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      %__grandparent = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 0, !dbg !45
      %y = getelementptr inbounds nuw %grandparent, ptr %__grandparent, i32 0, i32 1, !dbg !45
      %b = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2, !dbg !45
      %load_b = load i16, ptr %b, align [filtered], !dbg !45
      %1 = sext i16 %load_b to i32, !dbg !45
      %b1 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2, !dbg !45
      %load_b2 = load i16, ptr %b1, align [filtered], !dbg !45
      %2 = sext i16 %load_b2 to i32, !dbg !45
      %tmpVar = mul i32 %2, 2, !dbg !45
      %tmpVar3 = mul i32 1, %tmpVar, !dbg !45
      %tmpVar4 = add i32 %tmpVar3, 0, !dbg !45
      %tmpVar5 = getelementptr inbounds [11 x i16], ptr %z, i32 0, i32 %tmpVar4, !dbg !45
      %load_tmpVar = load i16, ptr %tmpVar5, align [filtered], !dbg !45
      %3 = sext i16 %load_tmpVar to i32, !dbg !45
      %tmpVar6 = add i32 %1, %3, !dbg !45
      %__grandparent7 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 0, !dbg !45
      %a = getelementptr inbounds nuw %grandparent, ptr %__grandparent7, i32 0, i32 2, !dbg !45
      %load_a = load i16, ptr %a, align [filtered], !dbg !45
      %4 = sext i16 %load_a to i32, !dbg !45
      %tmpVar8 = sub i32 %tmpVar6, %4, !dbg !45
      %tmpVar9 = mul i32 1, %tmpVar8, !dbg !45
      %tmpVar10 = add i32 %tmpVar9, 0, !dbg !45
      %tmpVar11 = getelementptr inbounds [6 x i16], ptr %y, i32 0, i32 %tmpVar10, !dbg !45
      store i16 20, ptr %tmpVar11, align [filtered], !dbg !46
      ret void, !dbg !47
    }

    define void @grandparent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !48
      store ptr %0, ptr %self, align [filtered], !dbg !48
      %deref = load ptr, ptr %self, align [filtered], !dbg !48
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %deref, i32 0, i32 0, !dbg !48
      call void @__grandparent___vtable__ctor(ptr %__vtable), !dbg !48
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !48
      %y = getelementptr inbounds nuw %grandparent, ptr %deref1, i32 0, i32 1, !dbg !48
      call void @__grandparent_y__ctor(ptr %y), !dbg !48
      %deref2 = load ptr, ptr %self, align [filtered], !dbg !48
      %__vtable3 = getelementptr inbounds nuw %grandparent, ptr %deref2, i32 0, i32 0, !dbg !48
      store ptr @__vtable_grandparent_instance, ptr %__vtable3, align [filtered], !dbg !48
      ret void, !dbg !48
    }

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !48
      store ptr %0, ptr %self, align [filtered], !dbg !48
      %deref = load ptr, ptr %self, align [filtered], !dbg !48
      %__grandparent = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0, !dbg !48
      call void @grandparent__ctor(ptr %__grandparent), !dbg !48
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !48
      %x = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 1, !dbg !48
      call void @__parent_x__ctor(ptr %x), !dbg !48
      %deref2 = load ptr, ptr %self, align [filtered], !dbg !48
      %__grandparent3 = getelementptr inbounds nuw %parent, ptr %deref2, i32 0, i32 0, !dbg !48
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent3, i32 0, i32 0, !dbg !48
      store ptr @__vtable_parent_instance, ptr %__vtable, align [filtered], !dbg !48
      ret void, !dbg !48
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !48
      store ptr %0, ptr %self, align [filtered], !dbg !48
      %deref = load ptr, ptr %self, align [filtered], !dbg !48
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0, !dbg !48
      call void @parent__ctor(ptr %__parent), !dbg !48
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !48
      %z = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 1, !dbg !48
      call void @__child_z__ctor(ptr %z), !dbg !48
      %deref2 = load ptr, ptr %self, align [filtered], !dbg !48
      %__parent3 = getelementptr inbounds nuw %child, ptr %deref2, i32 0, i32 0, !dbg !48
      %__grandparent = getelementptr inbounds nuw %parent, ptr %__parent3, i32 0, i32 0, !dbg !48
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent, i32 0, i32 0, !dbg !48
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered], !dbg !48
      ret void, !dbg !48
    }

    define void @__grandparent_y__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !48
      store ptr %0, ptr %self, align [filtered], !dbg !48
      ret void, !dbg !48
    }

    define void @__parent_x__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !48
      store ptr %0, ptr %self, align [filtered], !dbg !48
      ret void, !dbg !48
    }

    define void @__child_z__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !48
      store ptr %0, ptr %self, align [filtered], !dbg !48
      ret void, !dbg !48
    }

    define void @__vtable_grandparent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !48
      store ptr %0, ptr %self, align [filtered], !dbg !48
      %deref = load ptr, ptr %self, align [filtered], !dbg !48
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0, !dbg !48
      call void @____vtable_grandparent___body__ctor(ptr %__body), !dbg !48
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !48
      %__body2 = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref1, i32 0, i32 0, !dbg !48
      store ptr @grandparent, ptr %__body2, align [filtered], !dbg !48
      ret void, !dbg !48
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !48
      store ptr %0, ptr %self, align [filtered], !dbg !48
      %deref = load ptr, ptr %self, align [filtered], !dbg !48
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0, !dbg !48
      call void @____vtable_parent___body__ctor(ptr %__body), !dbg !48
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !48
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0, !dbg !48
      store ptr @parent, ptr %__body2, align [filtered], !dbg !48
      ret void, !dbg !48
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !48
      store ptr %0, ptr %self, align [filtered], !dbg !48
      %deref = load ptr, ptr %self, align [filtered], !dbg !48
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0, !dbg !48
      call void @____vtable_child___body__ctor(ptr %__body), !dbg !48
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !48
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0, !dbg !48
      store ptr @child, ptr %__body2, align [filtered], !dbg !48
      ret void, !dbg !48
    }

    define void @__grandparent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !48
      store ptr %0, ptr %self, align [filtered], !dbg !48
      ret void, !dbg !48
    }

    define void @____vtable_grandparent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !48
      store ptr %0, ptr %self, align [filtered], !dbg !48
      ret void, !dbg !48
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !48
      store ptr %0, ptr %self, align [filtered], !dbg !48
      ret void, !dbg !48
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !48
      store ptr %0, ptr %self, align [filtered], !dbg !48
      ret void, !dbg !48
    }

    define void @__unit___internal___[ctor-hash]__ctor() {
    entry:
      call void @__vtable_grandparent__ctor(ptr @__vtable_grandparent_instance), !dbg !48
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance), !dbg !48
      call void @__vtable_child__ctor(ptr @__vtable_child_instance), !dbg !48
      ret void, !dbg !48
    }

    !llvm.module.flags = !{!0, !1}
    !llvm.dbg.cu = !{!2}

    !0 = !{i32 2, !"Dwarf Version", i32 5}
    !1 = !{i32 2, !"Debug Info Version", i32 3}
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "<internal>", directory: "")
    !4 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !3, file: !3, line: 2, type: !5, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !19, keyInstructions: true)
    !5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
    !6 = !{null, !7}
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !3, file: !3, line: 2, size: 192, align [filtered], flags: DIFlagPublic, elements: !8, identifier: "grandparent")
    !8 = !{!9, !13, !18}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !3, file: !3, baseType: !10, size: 64, align [filtered], flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____grandparent___vtable", scope: !3, file: !3, baseType: !11, align [filtered])
    !11 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__grandparent___vtable", baseType: !12, size: 64, align [filtered], dwarfAddressSpace: 1)
    !12 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !13 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !3, file: !3, line: 4, baseType: !14, size: 96, align [filtered], offset: 64, flags: DIFlagPublic)
    !14 = !DICompositeType(tag: DW_TAG_array_type, baseType: !15, size: 96, align [filtered], elements: !16)
    !15 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !16 = !{!17}
    !17 = !DISubrange(count: 6, lowerBound: 0)
    !18 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !3, file: !3, line: 5, baseType: !15, size: 16, align [filtered], offset: 160, flags: DIFlagPublic)
    !19 = !{}
    !20 = !DILocalVariable(name: "grandparent", scope: !4, file: !3, line: 7, type: !7)
    !21 = !DILocation(line: 7, column: 8, scope: !4)
    !22 = !DILocation(line: 7, column: 8, scope: !4, atomGroup: 1, atomRank: 1)
    !23 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !3, file: !3, line: 9, type: !24, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !19, keyInstructions: true)
    !24 = !DISubroutineType(flags: DIFlagPublic, types: !25)
    !25 = !{null, !26}
    !26 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !3, file: !3, line: 9, size: 384, align [filtered], flags: DIFlagPublic, elements: !27, identifier: "parent")
    !27 = !{!28, !29, !33}
    !28 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !3, file: !3, baseType: !7, size: 192, align [filtered], flags: DIFlagPublic)
    !29 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !3, file: !3, line: 11, baseType: !30, size: 176, align [filtered], offset: 192, flags: DIFlagPublic)
    !30 = !DICompositeType(tag: DW_TAG_array_type, baseType: !15, size: 176, align [filtered], elements: !31)
    !31 = !{!32}
    !32 = !DISubrange(count: 11, lowerBound: 0)
    !33 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !3, file: !3, line: 12, baseType: !15, size: 16, align [filtered], offset: 368, flags: DIFlagPublic)
    !34 = !DILocalVariable(name: "parent", scope: !23, file: !3, line: 14, type: !26)
    !35 = !DILocation(line: 14, column: 8, scope: !23)
    !36 = !DILocation(line: 14, column: 8, scope: !23, atomGroup: 1, atomRank: 1)
    !37 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !3, file: !3, line: 16, type: !38, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !19, keyInstructions: true)
    !38 = !DISubroutineType(flags: DIFlagPublic, types: !39)
    !39 = !{null, !40}
    !40 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !3, file: !3, line: 16, size: 576, align [filtered], flags: DIFlagPublic, elements: !41, identifier: "child")
    !41 = !{!42, !43}
    !42 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !3, file: !3, baseType: !26, size: 384, align [filtered], flags: DIFlagPublic)
    !43 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !3, file: !3, line: 18, baseType: !30, size: 176, align [filtered], offset: 384, flags: DIFlagPublic)
    !44 = !DILocalVariable(name: "child", scope: !37, file: !3, line: 20, type: !40)
    !45 = !DILocation(line: 20, column: 12, scope: !37)
    !46 = !DILocation(line: 20, column: 12, scope: !37, atomGroup: 1, atomRank: 1)
    !47 = !DILocation(line: 21, column: 8, scope: !37, atomGroup: 2, atomRank: 1)
    !48 = !DILocation(line: 21, column: 8, scope: !37)
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
    %__vtable_bar = type { ptr, ptr }
    %foo = type { ptr }
    %bar = type { %foo }

    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal___[ctor-hash]__ctor, ptr null }]

    define void @foo(ptr %0) !dbg !4 {
    entry:
        #dbg_declare(ptr %0, !14, !DIExpression(), !15)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void, !dbg !16
    }

    define void @foo__baz(ptr %0) !dbg !17 {
    entry:
        #dbg_declare(ptr %0, !18, !DIExpression(), !19)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void, !dbg !20
    }

    define void @bar(ptr %0) !dbg !21 {
    entry:
        #dbg_declare(ptr %0, !27, !DIExpression(), !28)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__foo = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      ret void, !dbg !29
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !28
      store ptr %0, ptr %self, align [filtered], !dbg !28
      %deref = load ptr, ptr %self, align [filtered], !dbg !28
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0, !dbg !28
      call void @__foo___vtable__ctor(ptr %__vtable), !dbg !28
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !28
      %__vtable2 = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 0, !dbg !28
      store ptr @__vtable_foo_instance, ptr %__vtable2, align [filtered], !dbg !28
      ret void, !dbg !28
    }

    define void @bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !28
      store ptr %0, ptr %self, align [filtered], !dbg !28
      %deref = load ptr, ptr %self, align [filtered], !dbg !28
      %__foo = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0, !dbg !28
      call void @foo__ctor(ptr %__foo), !dbg !28
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !28
      %__foo2 = getelementptr inbounds nuw %bar, ptr %deref1, i32 0, i32 0, !dbg !28
      %__vtable = getelementptr inbounds nuw %foo, ptr %__foo2, i32 0, i32 0, !dbg !28
      store ptr @__vtable_bar_instance, ptr %__vtable, align [filtered], !dbg !28
      ret void, !dbg !28
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !28
      store ptr %0, ptr %self, align [filtered], !dbg !28
      %deref = load ptr, ptr %self, align [filtered], !dbg !28
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0, !dbg !28
      call void @____vtable_foo___body__ctor(ptr %__body), !dbg !28
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !28
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0, !dbg !28
      store ptr @foo, ptr %__body2, align [filtered], !dbg !28
      %deref3 = load ptr, ptr %self, align [filtered], !dbg !28
      %baz = getelementptr inbounds nuw %__vtable_foo, ptr %deref3, i32 0, i32 1, !dbg !28
      call void @____vtable_foo_baz__ctor(ptr %baz), !dbg !28
      %deref4 = load ptr, ptr %self, align [filtered], !dbg !28
      %baz5 = getelementptr inbounds nuw %__vtable_foo, ptr %deref4, i32 0, i32 1, !dbg !28
      store ptr @foo__baz, ptr %baz5, align [filtered], !dbg !28
      ret void, !dbg !28
    }

    define void @__vtable_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !28
      store ptr %0, ptr %self, align [filtered], !dbg !28
      %deref = load ptr, ptr %self, align [filtered], !dbg !28
      %__body = getelementptr inbounds nuw %__vtable_bar, ptr %deref, i32 0, i32 0, !dbg !28
      call void @____vtable_bar___body__ctor(ptr %__body), !dbg !28
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !28
      %__body2 = getelementptr inbounds nuw %__vtable_bar, ptr %deref1, i32 0, i32 0, !dbg !28
      store ptr @bar, ptr %__body2, align [filtered], !dbg !28
      %deref3 = load ptr, ptr %self, align [filtered], !dbg !28
      %baz = getelementptr inbounds nuw %__vtable_bar, ptr %deref3, i32 0, i32 1, !dbg !28
      call void @____vtable_bar_baz__ctor(ptr %baz), !dbg !28
      %deref4 = load ptr, ptr %self, align [filtered], !dbg !28
      %baz5 = getelementptr inbounds nuw %__vtable_bar, ptr %deref4, i32 0, i32 1, !dbg !28
      store ptr @foo__baz, ptr %baz5, align [filtered], !dbg !28
      ret void, !dbg !28
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !28
      store ptr %0, ptr %self, align [filtered], !dbg !28
      ret void, !dbg !28
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !28
      store ptr %0, ptr %self, align [filtered], !dbg !28
      ret void, !dbg !28
    }

    define void @____vtable_foo_baz__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !28
      store ptr %0, ptr %self, align [filtered], !dbg !28
      ret void, !dbg !28
    }

    define void @____vtable_bar___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !28
      store ptr %0, ptr %self, align [filtered], !dbg !28
      ret void, !dbg !28
    }

    define void @____vtable_bar_baz__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !28
      store ptr %0, ptr %self, align [filtered], !dbg !28
      ret void, !dbg !28
    }

    define void @__unit___internal___[ctor-hash]__ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance), !dbg !28
      call void @__vtable_bar__ctor(ptr @__vtable_bar_instance), !dbg !28
      ret void, !dbg !28
    }

    !llvm.module.flags = !{!0, !1}
    !llvm.dbg.cu = !{!2}

    !0 = !{i32 2, !"Dwarf Version", i32 5}
    !1 = !{i32 2, !"Debug Info Version", i32 3}
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "<internal>", directory: "")
    !4 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !3, file: !3, line: 2, type: !5, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !13, keyInstructions: true)
    !5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
    !6 = !{null, !7}
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !3, file: !3, line: 2, size: 64, align [filtered], flags: DIFlagPublic, elements: !8, identifier: "foo")
    !8 = !{!9}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !3, file: !3, baseType: !10, size: 64, align [filtered], flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____foo___vtable", scope: !3, file: !3, baseType: !11, align [filtered])
    !11 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__foo___vtable", baseType: !12, size: 64, align [filtered], dwarfAddressSpace: 1)
    !12 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !13 = !{}
    !14 = !DILocalVariable(name: "foo", scope: !4, file: !3, line: 5, type: !7)
    !15 = !DILocation(line: 5, column: 8, scope: !4)
    !16 = !DILocation(line: 5, column: 8, scope: !4, atomGroup: 1, atomRank: 1)
    !17 = distinct !DISubprogram(name: "foo.baz", linkageName: "foo.baz", scope: !4, file: !3, line: 3, type: !5, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !13, keyInstructions: true)
    !18 = !DILocalVariable(name: "foo", scope: !17, file: !3, line: 4, type: !7)
    !19 = !DILocation(line: 4, column: 8, scope: !17)
    !20 = !DILocation(line: 4, column: 8, scope: !17, atomGroup: 1, atomRank: 1)
    !21 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !3, file: !3, line: 7, type: !22, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !13, keyInstructions: true)
    !22 = !DISubroutineType(flags: DIFlagPublic, types: !23)
    !23 = !{null, !24}
    !24 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !3, file: !3, line: 7, size: 64, align [filtered], flags: DIFlagPublic, elements: !25, identifier: "bar")
    !25 = !{!26}
    !26 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !3, file: !3, baseType: !7, size: 64, align [filtered], flags: DIFlagPublic)
    !27 = !DILocalVariable(name: "bar", scope: !21, file: !3, line: 8, type: !24)
    !28 = !DILocation(line: 8, column: 8, scope: !21)
    !29 = !DILocation(line: 8, column: 8, scope: !21, atomGroup: 1, atomRank: 1)
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
    %__vtable_child = type { ptr }
    %__vtable_grandchild = type { ptr }
    %parent = type { ptr, i32 }
    %child = type { %parent, i32 }
    %grandchild = type { %child, i32 }

    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer
    @__vtable_grandchild_instance = global %__vtable_grandchild zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal___[ctor-hash]__ctor, ptr null }]

    define void @parent(ptr %0) !dbg !4 {
    entry:
        #dbg_declare(ptr %0, !16, !DIExpression(), !17)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %a = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      ret void, !dbg !18
    }

    define void @child(ptr %0) !dbg !19 {
    entry:
        #dbg_declare(ptr %0, !26, !DIExpression(), !27)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      ret void, !dbg !28
    }

    define void @grandchild(ptr %0) !dbg !29 {
    entry:
        #dbg_declare(ptr %0, !36, !DIExpression(), !37)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__child = getelementptr inbounds nuw %grandchild, ptr %0, i32 0, i32 0
      %c = getelementptr inbounds nuw %grandchild, ptr %0, i32 0, i32 1
      ret void, !dbg !38
    }

    define i32 @main() !dbg !39 {
    entry:
      %main = alloca i32, align [filtered]
      %array_of_parent = alloca [3 x %parent], align [filtered]
      %array_of_child = alloca [3 x %child], align [filtered]
      %array_of_grandchild = alloca [3 x %grandchild], align [filtered]
      %parent1 = alloca %parent, align [filtered]
      %child1 = alloca %child, align [filtered]
      %grandchild1 = alloca %grandchild, align [filtered]
        #dbg_declare(ptr %array_of_parent, !42, !DIExpression(), !46)
      call void @llvm.memset.p0.i64(ptr align [filtered] %array_of_parent, i8 0, i64 ptrtoint (ptr getelementptr ([3 x %parent], ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %array_of_child, !47, !DIExpression(), !49)
      call void @llvm.memset.p0.i64(ptr align [filtered] %array_of_child, i8 0, i64 ptrtoint (ptr getelementptr ([3 x %child], ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %array_of_grandchild, !50, !DIExpression(), !52)
      call void @llvm.memset.p0.i64(ptr align [filtered] %array_of_grandchild, i8 0, i64 ptrtoint (ptr getelementptr ([3 x %grandchild], ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %parent1, !53, !DIExpression(), !54)
      call void @llvm.memset.p0.i64(ptr align [filtered] %parent1, i8 0, i64 ptrtoint (ptr getelementptr (%parent, ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %child1, !55, !DIExpression(), !56)
      call void @llvm.memset.p0.i64(ptr align [filtered] %child1, i8 0, i64 ptrtoint (ptr getelementptr (%child, ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %grandchild1, !57, !DIExpression(), !58)
      call void @llvm.memset.p0.i64(ptr align [filtered] %grandchild1, i8 0, i64 ptrtoint (ptr getelementptr (%grandchild, ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %main, !59, !DIExpression(), !60)
      store i32 0, ptr %main, align [filtered]
      call void @__main_array_of_parent__ctor(ptr %array_of_parent), !dbg !61
      call void @__main_array_of_child__ctor(ptr %array_of_child), !dbg !61
      call void @__main_array_of_grandchild__ctor(ptr %array_of_grandchild), !dbg !61
      call void @parent__ctor(ptr %parent1), !dbg !61
      call void @child__ctor(ptr %child1), !dbg !61
      call void @grandchild__ctor(ptr %grandchild1), !dbg !62
      %a = getelementptr inbounds nuw %parent, ptr %parent1, i32 0, i32 1, !dbg !63
      store i32 1, ptr %a, align [filtered], !dbg !64
      %__parent = getelementptr inbounds nuw %child, ptr %child1, i32 0, i32 0, !dbg !65
      %a1 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 1, !dbg !65
      store i32 2, ptr %a1, align [filtered], !dbg !66
      %b = getelementptr inbounds nuw %child, ptr %child1, i32 0, i32 1, !dbg !67
      store i32 3, ptr %b, align [filtered], !dbg !68
      %__child = getelementptr inbounds nuw %grandchild, ptr %grandchild1, i32 0, i32 0, !dbg !69
      %__parent2 = getelementptr inbounds nuw %child, ptr %__child, i32 0, i32 0, !dbg !69
      %a3 = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 1, !dbg !69
      store i32 4, ptr %a3, align [filtered], !dbg !70
      %__child4 = getelementptr inbounds nuw %grandchild, ptr %grandchild1, i32 0, i32 0, !dbg !71
      %b5 = getelementptr inbounds nuw %child, ptr %__child4, i32 0, i32 1, !dbg !71
      store i32 5, ptr %b5, align [filtered], !dbg !72
      %c = getelementptr inbounds nuw %grandchild, ptr %grandchild1, i32 0, i32 1, !dbg !73
      store i32 6, ptr %c, align [filtered], !dbg !74
      %tmpVar = getelementptr inbounds [3 x %parent], ptr %array_of_parent, i32 0, i32 0, !dbg !75
      %a6 = getelementptr inbounds nuw %parent, ptr %tmpVar, i32 0, i32 1, !dbg !75
      store i32 7, ptr %a6, align [filtered], !dbg !76
      %tmpVar7 = getelementptr inbounds [3 x %child], ptr %array_of_child, i32 0, i32 0, !dbg !77
      %__parent8 = getelementptr inbounds nuw %child, ptr %tmpVar7, i32 0, i32 0, !dbg !77
      %a9 = getelementptr inbounds nuw %parent, ptr %__parent8, i32 0, i32 1, !dbg !77
      store i32 8, ptr %a9, align [filtered], !dbg !78
      %tmpVar10 = getelementptr inbounds [3 x %child], ptr %array_of_child, i32 0, i32 0, !dbg !79
      %b11 = getelementptr inbounds nuw %child, ptr %tmpVar10, i32 0, i32 1, !dbg !79
      store i32 9, ptr %b11, align [filtered], !dbg !80
      %tmpVar12 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 0, !dbg !81
      %__child13 = getelementptr inbounds nuw %grandchild, ptr %tmpVar12, i32 0, i32 0, !dbg !81
      %__parent14 = getelementptr inbounds nuw %child, ptr %__child13, i32 0, i32 0, !dbg !81
      %a15 = getelementptr inbounds nuw %parent, ptr %__parent14, i32 0, i32 1, !dbg !81
      store i32 10, ptr %a15, align [filtered], !dbg !82
      %tmpVar16 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 0, !dbg !83
      %__child17 = getelementptr inbounds nuw %grandchild, ptr %tmpVar16, i32 0, i32 0, !dbg !83
      %b18 = getelementptr inbounds nuw %child, ptr %__child17, i32 0, i32 1, !dbg !83
      store i32 11, ptr %b18, align [filtered], !dbg !84
      %tmpVar19 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 0, !dbg !85
      %c20 = getelementptr inbounds nuw %grandchild, ptr %tmpVar19, i32 0, i32 1, !dbg !85
      store i32 12, ptr %c20, align [filtered], !dbg !86
      %tmpVar21 = getelementptr inbounds [3 x %parent], ptr %array_of_parent, i32 0, i32 1, !dbg !87
      %a22 = getelementptr inbounds nuw %parent, ptr %tmpVar21, i32 0, i32 1, !dbg !87
      store i32 13, ptr %a22, align [filtered], !dbg !88
      %tmpVar23 = getelementptr inbounds [3 x %child], ptr %array_of_child, i32 0, i32 1, !dbg !89
      %__parent24 = getelementptr inbounds nuw %child, ptr %tmpVar23, i32 0, i32 0, !dbg !89
      %a25 = getelementptr inbounds nuw %parent, ptr %__parent24, i32 0, i32 1, !dbg !89
      store i32 14, ptr %a25, align [filtered], !dbg !90
      %tmpVar26 = getelementptr inbounds [3 x %child], ptr %array_of_child, i32 0, i32 1, !dbg !91
      %b27 = getelementptr inbounds nuw %child, ptr %tmpVar26, i32 0, i32 1, !dbg !91
      store i32 15, ptr %b27, align [filtered], !dbg !92
      %tmpVar28 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 1, !dbg !93
      %__child29 = getelementptr inbounds nuw %grandchild, ptr %tmpVar28, i32 0, i32 0, !dbg !93
      %__parent30 = getelementptr inbounds nuw %child, ptr %__child29, i32 0, i32 0, !dbg !93
      %a31 = getelementptr inbounds nuw %parent, ptr %__parent30, i32 0, i32 1, !dbg !93
      store i32 16, ptr %a31, align [filtered], !dbg !94
      %tmpVar32 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 1, !dbg !95
      %__child33 = getelementptr inbounds nuw %grandchild, ptr %tmpVar32, i32 0, i32 0, !dbg !95
      %b34 = getelementptr inbounds nuw %child, ptr %__child33, i32 0, i32 1, !dbg !95
      store i32 17, ptr %b34, align [filtered], !dbg !96
      %tmpVar35 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 1, !dbg !97
      %c36 = getelementptr inbounds nuw %grandchild, ptr %tmpVar35, i32 0, i32 1, !dbg !97
      store i32 18, ptr %c36, align [filtered], !dbg !98
      %tmpVar37 = getelementptr inbounds [3 x %parent], ptr %array_of_parent, i32 0, i32 2, !dbg !99
      %a38 = getelementptr inbounds nuw %parent, ptr %tmpVar37, i32 0, i32 1, !dbg !99
      store i32 19, ptr %a38, align [filtered], !dbg !100
      %tmpVar39 = getelementptr inbounds [3 x %child], ptr %array_of_child, i32 0, i32 2, !dbg !101
      %__parent40 = getelementptr inbounds nuw %child, ptr %tmpVar39, i32 0, i32 0, !dbg !101
      %a41 = getelementptr inbounds nuw %parent, ptr %__parent40, i32 0, i32 1, !dbg !101
      store i32 20, ptr %a41, align [filtered], !dbg !102
      %tmpVar42 = getelementptr inbounds [3 x %child], ptr %array_of_child, i32 0, i32 2, !dbg !103
      %b43 = getelementptr inbounds nuw %child, ptr %tmpVar42, i32 0, i32 1, !dbg !103
      store i32 21, ptr %b43, align [filtered], !dbg !104
      %tmpVar44 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 2, !dbg !105
      %__child45 = getelementptr inbounds nuw %grandchild, ptr %tmpVar44, i32 0, i32 0, !dbg !105
      %__parent46 = getelementptr inbounds nuw %child, ptr %__child45, i32 0, i32 0, !dbg !105
      %a47 = getelementptr inbounds nuw %parent, ptr %__parent46, i32 0, i32 1, !dbg !105
      store i32 22, ptr %a47, align [filtered], !dbg !106
      %tmpVar48 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 2, !dbg !107
      %__child49 = getelementptr inbounds nuw %grandchild, ptr %tmpVar48, i32 0, i32 0, !dbg !107
      %b50 = getelementptr inbounds nuw %child, ptr %__child49, i32 0, i32 1, !dbg !107
      store i32 23, ptr %b50, align [filtered], !dbg !108
      %tmpVar51 = getelementptr inbounds [3 x %grandchild], ptr %array_of_grandchild, i32 0, i32 2, !dbg !109
      %c52 = getelementptr inbounds nuw %grandchild, ptr %tmpVar51, i32 0, i32 1, !dbg !109
      store i32 24, ptr %c52, align [filtered], !dbg !110
      %main_ret = load i32, ptr %main, align [filtered], !dbg !111
      ret i32 %main_ret, !dbg !112
    }

    define void @parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !111
      store ptr %0, ptr %self, align [filtered], !dbg !111
      %deref = load ptr, ptr %self, align [filtered], !dbg !111
      %__vtable = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0, !dbg !111
      call void @__parent___vtable__ctor(ptr %__vtable), !dbg !111
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !111
      %__vtable2 = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 0, !dbg !111
      store ptr @__vtable_parent_instance, ptr %__vtable2, align [filtered], !dbg !111
      ret void, !dbg !111
    }

    define void @child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !111
      store ptr %0, ptr %self, align [filtered], !dbg !111
      %deref = load ptr, ptr %self, align [filtered], !dbg !111
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0, !dbg !111
      call void @parent__ctor(ptr %__parent), !dbg !111
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !111
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0, !dbg !111
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0, !dbg !111
      store ptr @__vtable_child_instance, ptr %__vtable, align [filtered], !dbg !111
      ret void, !dbg !111
    }

    define void @grandchild__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !111
      store ptr %0, ptr %self, align [filtered], !dbg !111
      %deref = load ptr, ptr %self, align [filtered], !dbg !111
      %__child = getelementptr inbounds nuw %grandchild, ptr %deref, i32 0, i32 0, !dbg !111
      call void @child__ctor(ptr %__child), !dbg !111
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !111
      %__child2 = getelementptr inbounds nuw %grandchild, ptr %deref1, i32 0, i32 0, !dbg !111
      %__parent = getelementptr inbounds nuw %child, ptr %__child2, i32 0, i32 0, !dbg !111
      %__vtable = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 0, !dbg !111
      store ptr @__vtable_grandchild_instance, ptr %__vtable, align [filtered], !dbg !111
      ret void, !dbg !111
    }

    define void @__main_array_of_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !111
      store ptr %0, ptr %self, align [filtered], !dbg !111
      %__main_array_of_parent__idx0 = alloca i32, align [filtered]
      store i32 0, ptr %__main_array_of_parent__idx0, align [filtered], !dbg !111
      store i32 0, ptr %__main_array_of_parent__idx0, align [filtered], !dbg !111
      br label %while_body, !dbg !111

    while_body:                                       ; preds = %continue1, %entry
      %load___main_array_of_parent__idx0 = load i32, ptr %__main_array_of_parent__idx0, align [filtered], !dbg !111
      %tmpVar = icmp sgt i32 %load___main_array_of_parent__idx0, 2, !dbg !111
      %1 = zext i1 %tmpVar to i8, !dbg !111
      %2 = icmp ne i8 %1, 0, !dbg !111
      br i1 %2, label %condition_body, label %continue1, !dbg !111

    continue:                                         ; preds = %condition_body
      ret void, !dbg !111

    condition_body:                                   ; preds = %while_body
      br label %continue, !dbg !111

    buffer_block:                                     ; No predecessors!
      br label %continue1, !dbg !111

    continue1:                                        ; preds = %buffer_block, %while_body
      %deref = load ptr, ptr %self, align [filtered], !dbg !111
      %load___main_array_of_parent__idx02 = load i32, ptr %__main_array_of_parent__idx0, align [filtered], !dbg !111
      %tmpVar3 = mul i32 1, %load___main_array_of_parent__idx02, !dbg !111
      %tmpVar4 = add i32 %tmpVar3, 0, !dbg !111
      %tmpVar5 = getelementptr inbounds [3 x %parent], ptr %deref, i32 0, i32 %tmpVar4, !dbg !111
      call void @parent__ctor(ptr %tmpVar5), !dbg !111
      %load___main_array_of_parent__idx06 = load i32, ptr %__main_array_of_parent__idx0, align [filtered], !dbg !111
      %tmpVar7 = add i32 %load___main_array_of_parent__idx06, 1, !dbg !111
      store i32 %tmpVar7, ptr %__main_array_of_parent__idx0, align [filtered], !dbg !111
      br label %while_body, !dbg !111
    }

    define void @__main_array_of_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !111
      store ptr %0, ptr %self, align [filtered], !dbg !111
      %__main_array_of_child__idx0 = alloca i32, align [filtered]
      store i32 0, ptr %__main_array_of_child__idx0, align [filtered], !dbg !111
      store i32 0, ptr %__main_array_of_child__idx0, align [filtered], !dbg !111
      br label %while_body, !dbg !111

    while_body:                                       ; preds = %continue1, %entry
      %load___main_array_of_child__idx0 = load i32, ptr %__main_array_of_child__idx0, align [filtered], !dbg !111
      %tmpVar = icmp sgt i32 %load___main_array_of_child__idx0, 2, !dbg !111
      %1 = zext i1 %tmpVar to i8, !dbg !111
      %2 = icmp ne i8 %1, 0, !dbg !111
      br i1 %2, label %condition_body, label %continue1, !dbg !111

    continue:                                         ; preds = %condition_body
      ret void, !dbg !111

    condition_body:                                   ; preds = %while_body
      br label %continue, !dbg !111

    buffer_block:                                     ; No predecessors!
      br label %continue1, !dbg !111

    continue1:                                        ; preds = %buffer_block, %while_body
      %deref = load ptr, ptr %self, align [filtered], !dbg !111
      %load___main_array_of_child__idx02 = load i32, ptr %__main_array_of_child__idx0, align [filtered], !dbg !111
      %tmpVar3 = mul i32 1, %load___main_array_of_child__idx02, !dbg !111
      %tmpVar4 = add i32 %tmpVar3, 0, !dbg !111
      %tmpVar5 = getelementptr inbounds [3 x %child], ptr %deref, i32 0, i32 %tmpVar4, !dbg !111
      call void @child__ctor(ptr %tmpVar5), !dbg !111
      %load___main_array_of_child__idx06 = load i32, ptr %__main_array_of_child__idx0, align [filtered], !dbg !111
      %tmpVar7 = add i32 %load___main_array_of_child__idx06, 1, !dbg !111
      store i32 %tmpVar7, ptr %__main_array_of_child__idx0, align [filtered], !dbg !111
      br label %while_body, !dbg !111
    }

    define void @__main_array_of_grandchild__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !111
      store ptr %0, ptr %self, align [filtered], !dbg !111
      %__main_array_of_grandchild__idx0 = alloca i32, align [filtered]
      store i32 0, ptr %__main_array_of_grandchild__idx0, align [filtered], !dbg !111
      store i32 0, ptr %__main_array_of_grandchild__idx0, align [filtered], !dbg !111
      br label %while_body, !dbg !111

    while_body:                                       ; preds = %continue1, %entry
      %load___main_array_of_grandchild__idx0 = load i32, ptr %__main_array_of_grandchild__idx0, align [filtered], !dbg !111
      %tmpVar = icmp sgt i32 %load___main_array_of_grandchild__idx0, 2, !dbg !111
      %1 = zext i1 %tmpVar to i8, !dbg !111
      %2 = icmp ne i8 %1, 0, !dbg !111
      br i1 %2, label %condition_body, label %continue1, !dbg !111

    continue:                                         ; preds = %condition_body
      ret void, !dbg !111

    condition_body:                                   ; preds = %while_body
      br label %continue, !dbg !111

    buffer_block:                                     ; No predecessors!
      br label %continue1, !dbg !111

    continue1:                                        ; preds = %buffer_block, %while_body
      %deref = load ptr, ptr %self, align [filtered], !dbg !111
      %load___main_array_of_grandchild__idx02 = load i32, ptr %__main_array_of_grandchild__idx0, align [filtered], !dbg !111
      %tmpVar3 = mul i32 1, %load___main_array_of_grandchild__idx02, !dbg !111
      %tmpVar4 = add i32 %tmpVar3, 0, !dbg !111
      %tmpVar5 = getelementptr inbounds [3 x %grandchild], ptr %deref, i32 0, i32 %tmpVar4, !dbg !111
      call void @grandchild__ctor(ptr %tmpVar5), !dbg !111
      %load___main_array_of_grandchild__idx06 = load i32, ptr %__main_array_of_grandchild__idx0, align [filtered], !dbg !111
      %tmpVar7 = add i32 %load___main_array_of_grandchild__idx06, 1, !dbg !111
      store i32 %tmpVar7, ptr %__main_array_of_grandchild__idx0, align [filtered], !dbg !111
      br label %while_body, !dbg !111
    }

    define void @__vtable_parent__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !111
      store ptr %0, ptr %self, align [filtered], !dbg !111
      %deref = load ptr, ptr %self, align [filtered], !dbg !111
      %__body = getelementptr inbounds nuw %__vtable_parent, ptr %deref, i32 0, i32 0, !dbg !111
      call void @____vtable_parent___body__ctor(ptr %__body), !dbg !111
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !111
      %__body2 = getelementptr inbounds nuw %__vtable_parent, ptr %deref1, i32 0, i32 0, !dbg !111
      store ptr @parent, ptr %__body2, align [filtered], !dbg !111
      ret void, !dbg !111
    }

    define void @__vtable_child__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !111
      store ptr %0, ptr %self, align [filtered], !dbg !111
      %deref = load ptr, ptr %self, align [filtered], !dbg !111
      %__body = getelementptr inbounds nuw %__vtable_child, ptr %deref, i32 0, i32 0, !dbg !111
      call void @____vtable_child___body__ctor(ptr %__body), !dbg !111
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !111
      %__body2 = getelementptr inbounds nuw %__vtable_child, ptr %deref1, i32 0, i32 0, !dbg !111
      store ptr @child, ptr %__body2, align [filtered], !dbg !111
      ret void, !dbg !111
    }

    define void @__vtable_grandchild__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !111
      store ptr %0, ptr %self, align [filtered], !dbg !111
      %deref = load ptr, ptr %self, align [filtered], !dbg !111
      %__body = getelementptr inbounds nuw %__vtable_grandchild, ptr %deref, i32 0, i32 0, !dbg !111
      call void @____vtable_grandchild___body__ctor(ptr %__body), !dbg !111
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !111
      %__body2 = getelementptr inbounds nuw %__vtable_grandchild, ptr %deref1, i32 0, i32 0, !dbg !111
      store ptr @grandchild, ptr %__body2, align [filtered], !dbg !111
      ret void, !dbg !111
    }

    define void @__parent___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !111
      store ptr %0, ptr %self, align [filtered], !dbg !111
      ret void, !dbg !111
    }

    define void @____vtable_parent___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !111
      store ptr %0, ptr %self, align [filtered], !dbg !111
      ret void, !dbg !111
    }

    define void @____vtable_child___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !111
      store ptr %0, ptr %self, align [filtered], !dbg !111
      ret void, !dbg !111
    }

    define void @____vtable_grandchild___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !111
      store ptr %0, ptr %self, align [filtered], !dbg !111
      ret void, !dbg !111
    }

    define void @__unit___internal___[ctor-hash]__ctor() {
    entry:
      call void @__vtable_parent__ctor(ptr @__vtable_parent_instance), !dbg !111
      call void @__vtable_child__ctor(ptr @__vtable_child_instance), !dbg !111
      call void @__vtable_grandchild__ctor(ptr @__vtable_grandchild_instance), !dbg !111
      ret void, !dbg !111
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }

    !llvm.module.flags = !{!0, !1}
    !llvm.dbg.cu = !{!2}

    !0 = !{i32 2, !"Dwarf Version", i32 5}
    !1 = !{i32 2, !"Debug Info Version", i32 3}
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "<internal>", directory: "")
    !4 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !3, file: !3, line: 2, type: !5, scopeLine: 6, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !15, keyInstructions: true)
    !5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
    !6 = !{null, !7}
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !3, file: !3, line: 2, size: 128, align [filtered], flags: DIFlagPublic, elements: !8, identifier: "parent")
    !8 = !{!9, !13}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !3, file: !3, baseType: !10, size: 64, align [filtered], flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____parent___vtable", scope: !3, file: !3, baseType: !11, align [filtered])
    !11 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__parent___vtable", baseType: !12, size: 64, align [filtered], dwarfAddressSpace: 1)
    !12 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !13 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !3, file: !3, line: 4, baseType: !14, size: 32, align [filtered], offset: 64, flags: DIFlagPublic)
    !14 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !15 = !{}
    !16 = !DILocalVariable(name: "parent", scope: !4, file: !3, line: 6, type: !7)
    !17 = !DILocation(line: 6, scope: !4)
    !18 = !DILocation(line: 6, scope: !4, atomGroup: 1, atomRank: 1)
    !19 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !3, file: !3, line: 8, type: !20, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !15, keyInstructions: true)
    !20 = !DISubroutineType(flags: DIFlagPublic, types: !21)
    !21 = !{null, !22}
    !22 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !3, file: !3, line: 8, size: 192, align [filtered], flags: DIFlagPublic, elements: !23, identifier: "child")
    !23 = !{!24, !25}
    !24 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !3, file: !3, baseType: !7, size: 128, align [filtered], flags: DIFlagPublic)
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !3, file: !3, line: 10, baseType: !14, size: 32, align [filtered], offset: 128, flags: DIFlagPublic)
    !26 = !DILocalVariable(name: "child", scope: !19, file: !3, line: 12, type: !22)
    !27 = !DILocation(line: 12, scope: !19)
    !28 = !DILocation(line: 12, scope: !19, atomGroup: 1, atomRank: 1)
    !29 = distinct !DISubprogram(name: "grandchild", linkageName: "grandchild", scope: !3, file: !3, line: 14, type: !30, scopeLine: 18, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !15, keyInstructions: true)
    !30 = !DISubroutineType(flags: DIFlagPublic, types: !31)
    !31 = !{null, !32}
    !32 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandchild", scope: !3, file: !3, line: 14, size: 256, align [filtered], flags: DIFlagPublic, elements: !33, identifier: "grandchild")
    !33 = !{!34, !35}
    !34 = !DIDerivedType(tag: DW_TAG_member, name: "SUPER", scope: !3, file: !3, baseType: !22, size: 192, align [filtered], flags: DIFlagPublic)
    !35 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !3, file: !3, line: 16, baseType: !14, size: 32, align [filtered], offset: 192, flags: DIFlagPublic)
    !36 = !DILocalVariable(name: "grandchild", scope: !29, file: !3, line: 18, type: !32)
    !37 = !DILocation(line: 18, scope: !29)
    !38 = !DILocation(line: 18, scope: !29, atomGroup: 1, atomRank: 1)
    !39 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !3, file: !3, line: 20, type: !40, scopeLine: 30, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !15, keyInstructions: true)
    !40 = !DISubroutineType(flags: DIFlagPublic, types: !41)
    !41 = !{null}
    !42 = !DILocalVariable(name: "array_of_parent", scope: !39, file: !3, line: 22, type: !43, align [filtered])
    !43 = !DICompositeType(tag: DW_TAG_array_type, baseType: !7, size: 384, align [filtered], elements: !44)
    !44 = !{!45}
    !45 = !DISubrange(count: 3, lowerBound: 0)
    !46 = !DILocation(line: 22, column: 4, scope: !39)
    !47 = !DILocalVariable(name: "array_of_child", scope: !39, file: !3, line: 23, type: !48, align [filtered])
    !48 = !DICompositeType(tag: DW_TAG_array_type, baseType: !22, size: 576, align [filtered], elements: !44)
    !49 = !DILocation(line: 23, column: 4, scope: !39)
    !50 = !DILocalVariable(name: "array_of_grandchild", scope: !39, file: !3, line: 24, type: !51, align [filtered])
    !51 = !DICompositeType(tag: DW_TAG_array_type, baseType: !32, size: 768, align [filtered], elements: !44)
    !52 = !DILocation(line: 24, column: 4, scope: !39)
    !53 = !DILocalVariable(name: "parent1", scope: !39, file: !3, line: 25, type: !7, align [filtered])
    !54 = !DILocation(line: 25, column: 4, scope: !39)
    !55 = !DILocalVariable(name: "child1", scope: !39, file: !3, line: 26, type: !22, align [filtered])
    !56 = !DILocation(line: 26, column: 4, scope: !39)
    !57 = !DILocalVariable(name: "grandchild1", scope: !39, file: !3, line: 27, type: !32, align [filtered])
    !58 = !DILocation(line: 27, column: 4, scope: !39)
    !59 = !DILocalVariable(name: "main", scope: !39, file: !3, line: 20, type: !14, align [filtered])
    !60 = !DILocation(line: 20, column: 9, scope: !39)
    !61 = !DILocation(line: 0, scope: !39)
    !62 = !DILocation(line: 0, scope: !39, atomGroup: 1, atomRank: 1)
    !63 = !DILocation(line: 30, column: 4, scope: !39)
    !64 = !DILocation(line: 30, column: 4, scope: !39, atomGroup: 2, atomRank: 1)
    !65 = !DILocation(line: 31, column: 4, scope: !39)
    !66 = !DILocation(line: 31, column: 4, scope: !39, atomGroup: 3, atomRank: 1)
    !67 = !DILocation(line: 32, column: 4, scope: !39)
    !68 = !DILocation(line: 32, column: 4, scope: !39, atomGroup: 4, atomRank: 1)
    !69 = !DILocation(line: 33, column: 4, scope: !39)
    !70 = !DILocation(line: 33, column: 4, scope: !39, atomGroup: 5, atomRank: 1)
    !71 = !DILocation(line: 34, column: 4, scope: !39)
    !72 = !DILocation(line: 34, column: 4, scope: !39, atomGroup: 6, atomRank: 1)
    !73 = !DILocation(line: 35, column: 4, scope: !39)
    !74 = !DILocation(line: 35, column: 4, scope: !39, atomGroup: 7, atomRank: 1)
    !75 = !DILocation(line: 37, column: 4, scope: !39)
    !76 = !DILocation(line: 37, column: 4, scope: !39, atomGroup: 8, atomRank: 1)
    !77 = !DILocation(line: 38, column: 4, scope: !39)
    !78 = !DILocation(line: 38, column: 4, scope: !39, atomGroup: 9, atomRank: 1)
    !79 = !DILocation(line: 39, column: 4, scope: !39)
    !80 = !DILocation(line: 39, column: 4, scope: !39, atomGroup: 10, atomRank: 1)
    !81 = !DILocation(line: 40, column: 4, scope: !39)
    !82 = !DILocation(line: 40, column: 4, scope: !39, atomGroup: 11, atomRank: 1)
    !83 = !DILocation(line: 41, column: 4, scope: !39)
    !84 = !DILocation(line: 41, column: 4, scope: !39, atomGroup: 12, atomRank: 1)
    !85 = !DILocation(line: 42, column: 4, scope: !39)
    !86 = !DILocation(line: 42, column: 4, scope: !39, atomGroup: 13, atomRank: 1)
    !87 = !DILocation(line: 43, column: 4, scope: !39)
    !88 = !DILocation(line: 43, column: 4, scope: !39, atomGroup: 14, atomRank: 1)
    !89 = !DILocation(line: 44, column: 4, scope: !39)
    !90 = !DILocation(line: 44, column: 4, scope: !39, atomGroup: 15, atomRank: 1)
    !91 = !DILocation(line: 45, column: 4, scope: !39)
    !92 = !DILocation(line: 45, column: 4, scope: !39, atomGroup: 16, atomRank: 1)
    !93 = !DILocation(line: 46, column: 4, scope: !39)
    !94 = !DILocation(line: 46, column: 4, scope: !39, atomGroup: 17, atomRank: 1)
    !95 = !DILocation(line: 47, column: 4, scope: !39)
    !96 = !DILocation(line: 47, column: 4, scope: !39, atomGroup: 18, atomRank: 1)
    !97 = !DILocation(line: 48, column: 4, scope: !39)
    !98 = !DILocation(line: 48, column: 4, scope: !39, atomGroup: 19, atomRank: 1)
    !99 = !DILocation(line: 49, column: 4, scope: !39)
    !100 = !DILocation(line: 49, column: 4, scope: !39, atomGroup: 20, atomRank: 1)
    !101 = !DILocation(line: 50, column: 4, scope: !39)
    !102 = !DILocation(line: 50, column: 4, scope: !39, atomGroup: 21, atomRank: 1)
    !103 = !DILocation(line: 51, column: 4, scope: !39)
    !104 = !DILocation(line: 51, column: 4, scope: !39, atomGroup: 22, atomRank: 1)
    !105 = !DILocation(line: 52, column: 4, scope: !39)
    !106 = !DILocation(line: 52, column: 4, scope: !39, atomGroup: 23, atomRank: 1)
    !107 = !DILocation(line: 53, column: 4, scope: !39)
    !108 = !DILocation(line: 53, column: 4, scope: !39, atomGroup: 24, atomRank: 1)
    !109 = !DILocation(line: 54, column: 4, scope: !39)
    !110 = !DILocation(line: 54, column: 4, scope: !39, atomGroup: 25, atomRank: 1)
    !111 = !DILocation(line: 56, scope: !39)
    !112 = !DILocation(line: 56, scope: !39, atomGroup: 26, atomRank: 1)
    "#);
}
