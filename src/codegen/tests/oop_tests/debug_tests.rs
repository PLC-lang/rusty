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

    define void @foo(%foo* %0) !dbg !34 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !38, metadata !DIExpression()), !dbg !39
      %a = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0, !dbg !39
      %b = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1, !dbg !39
      %c = getelementptr inbounds %foo, %foo* %0, i32 0, i32 2, !dbg !39
      ret void, !dbg !39
    }

    define void @bar(%bar* %0) !dbg !40 {
    entry:
      call void @llvm.dbg.declare(metadata %bar* %0, metadata !41, metadata !DIExpression()), !dbg !42
      %__SUPER = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0, !dbg !42
      ret void, !dbg !42
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init_foo(%foo* %0) !dbg !43 {
    entry:
      %self = alloca %foo*, align 8, !dbg !47
      call void @llvm.dbg.declare(metadata %foo** %self, metadata !48, metadata !DIExpression()), !dbg !47
      store %foo* %0, %foo** %self, align 8, !dbg !47
      ret void, !dbg !47
    }

    define void @__init_bar(%bar* %0) !dbg !49 {
    entry:
      %self = alloca %bar*, align 8, !dbg !53
      call void @llvm.dbg.declare(metadata %bar** %self, metadata !54, metadata !DIExpression()), !dbg !53
      store %bar* %0, %bar** %self, align 8, !dbg !53
      %deref = load %bar*, %bar** %self, align 8, !dbg !53
      %__SUPER = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0, !dbg !53
      call void @__init_foo(%foo* %__SUPER), !dbg !55
      ret void, !dbg !55
    }

    define void @__init___Test() !dbg !56 {
    entry:
      ret void, !dbg !57
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!21, !22}
    !llvm.dbg.cu = !{!23, !25, !32}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 2, size: 7792, align: 64, flags: DIFlagPublic, elements: !4, identifier: "foo")
    !4 = !{!5, !7, !12}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 4, baseType: !6, size: 16, align: 16, flags: DIFlagPublic)
    !6 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 5, baseType: !8, size: 648, align: 8, offset: 16, flags: DIFlagPublic)
    !8 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 648, align: 8, elements: !10)
    !9 = !DIBasicType(name: "char", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !10 = !{!11}
    !11 = !DISubrange(count: 81, lowerBound: 0)
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 6, baseType: !13, size: 7128, align: 8, offset: 664, flags: DIFlagPublic)
    !13 = !DICompositeType(tag: DW_TAG_array_type, baseType: !8, size: 7128, align: 8, elements: !14)
    !14 = !{!15}
    !15 = !DISubrange(count: 11, lowerBound: 0)
    !16 = !DIGlobalVariableExpression(var: !17, expr: !DIExpression())
    !17 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 10, type: !18, isLocal: false, isDefinition: true)
    !18 = !DICompositeType(tag: DW_TAG_structure_type, name: "bar", scope: !2, file: !2, line: 10, size: 7792, align: 64, flags: DIFlagPublic, elements: !19, identifier: "bar")
    !19 = !{!20}
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "__SUPER", scope: !2, file: !2, baseType: !3, size: 7792, align: 64, flags: DIFlagPublic)
    !21 = !{i32 2, !"Dwarf Version", i32 5}
    !22 = !{i32 2, !"Debug Info Version", i32 3}
    !23 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !24, splitDebugInlining: false)
    !24 = !{!0, !16}
    !25 = distinct !DICompileUnit(language: DW_LANG_C, file: !26, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !27, splitDebugInlining: false)
    !26 = !DIFile(filename: "__initializers", directory: "")
    !27 = !{!28, !30}
    !28 = !DIGlobalVariableExpression(var: !29, expr: !DIExpression())
    !29 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !30 = !DIGlobalVariableExpression(var: !31, expr: !DIExpression())
    !31 = distinct !DIGlobalVariable(name: "__bar__init", scope: !2, file: !2, line: 10, type: !18, isLocal: false, isDefinition: true)
    !32 = distinct !DICompileUnit(language: DW_LANG_C, file: !33, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !33 = !DIFile(filename: "__init___Test", directory: "")
    !34 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 2, type: !35, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !23, retainedNodes: !37)
    !35 = !DISubroutineType(flags: DIFlagPublic, types: !36)
    !36 = !{null}
    !37 = !{}
    !38 = !DILocalVariable(name: "foo", scope: !34, file: !2, line: 2, type: !3)
    !39 = !DILocation(line: 8, column: 8, scope: !34)
    !40 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 10, type: !35, scopeLine: 11, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !23, retainedNodes: !37)
    !41 = !DILocalVariable(name: "bar", scope: !40, file: !2, line: 10, type: !18)
    !42 = !DILocation(line: 11, column: 8, scope: !40)
    !43 = distinct !DISubprogram(name: "__init_foo", linkageName: "__init_foo", scope: !2, file: !2, line: 2, type: !44, scopeLine: 2, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !25, retainedNodes: !37)
    !44 = !DISubroutineType(flags: DIFlagPublic, types: !45)
    !45 = !{null, !46}
    !46 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_foo", baseType: !3, size: 64, align: 64, dwarfAddressSpace: 1)
    !47 = !DILocation(line: 2, column: 23, scope: !43)
    !48 = !DILocalVariable(name: "self", scope: !43, file: !2, line: 2, type: !46)
    !49 = distinct !DISubprogram(name: "__init_bar", linkageName: "__init_bar", scope: !2, file: !2, line: 10, type: !50, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !25, retainedNodes: !37)
    !50 = !DISubroutineType(flags: DIFlagPublic, types: !51)
    !51 = !{null, !52}
    !52 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_bar", baseType: !18, size: 64, align: 64, dwarfAddressSpace: 1)
    !53 = !DILocation(line: 10, column: 23, scope: !49)
    !54 = !DILocalVariable(name: "self", scope: !49, file: !2, line: 10, type: !52)
    !55 = !DILocation(line: 0, scope: !49)
    !56 = distinct !DISubprogram(name: "__init___Test", linkageName: "__init___Test", scope: !2, file: !2, type: !35, scopeLine: 1, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !32, retainedNodes: !37)
    !57 = !DILocation(line: 0, scope: !56)
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

    define void @fb(%fb* %0) !dbg !33 {
    entry:
      call void @llvm.dbg.declare(metadata %fb* %0, metadata !37, metadata !DIExpression()), !dbg !38
      %x = getelementptr inbounds %fb, %fb* %0, i32 0, i32 0, !dbg !38
      %y = getelementptr inbounds %fb, %fb* %0, i32 0, i32 1, !dbg !38
      ret void, !dbg !38
    }

    define void @fb2(%fb2* %0) !dbg !39 {
    entry:
      call void @llvm.dbg.declare(metadata %fb2* %0, metadata !40, metadata !DIExpression()), !dbg !41
      %__SUPER = getelementptr inbounds %fb2, %fb2* %0, i32 0, i32 0, !dbg !41
      ret void, !dbg !41
    }

    define void @foo(%foo* %0) !dbg !42 {
    entry:
      call void @llvm.dbg.declare(metadata %foo* %0, metadata !43, metadata !DIExpression()), !dbg !44
      %myFb = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0, !dbg !44
      %__SUPER = getelementptr inbounds %fb2, %fb2* %myFb, i32 0, i32 0, !dbg !44
      %x = getelementptr inbounds %fb, %fb* %__SUPER, i32 0, i32 0, !dbg !44
      store i16 1, i16* %x, align 2, !dbg !44
      ret void, !dbg !44
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init_fb2(%fb2* %0) !dbg !45 {
    entry:
      %self = alloca %fb2*, align 8, !dbg !49
      call void @llvm.dbg.declare(metadata %fb2** %self, metadata !50, metadata !DIExpression()), !dbg !49
      store %fb2* %0, %fb2** %self, align 8, !dbg !49
      %deref = load %fb2*, %fb2** %self, align 8, !dbg !49
      %__SUPER = getelementptr inbounds %fb2, %fb2* %deref, i32 0, i32 0, !dbg !49
      call void @__init_fb(%fb* %__SUPER), !dbg !51
      ret void, !dbg !51
    }

    define void @__init_fb(%fb* %0) !dbg !52 {
    entry:
      %self = alloca %fb*, align 8, !dbg !56
      call void @llvm.dbg.declare(metadata %fb** %self, metadata !57, metadata !DIExpression()), !dbg !56
      store %fb* %0, %fb** %self, align 8, !dbg !56
      ret void, !dbg !56
    }

    define void @__init_foo(%foo* %0) !dbg !58 {
    entry:
      %self = alloca %foo*, align 8, !dbg !62
      call void @llvm.dbg.declare(metadata %foo** %self, metadata !63, metadata !DIExpression()), !dbg !62
      store %foo* %0, %foo** %self, align 8, !dbg !62
      %deref = load %foo*, %foo** %self, align 8, !dbg !62
      %myFb = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0, !dbg !62
      call void @__init_fb2(%fb2* %myFb), !dbg !64
      ret void, !dbg !64
    }

    define void @__init___Test() !dbg !65 {
    entry:
      ret void, !dbg !66
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!18, !19}
    !llvm.dbg.cu = !{!20, !22, !31}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__fb2__init", scope: !2, file: !2, line: 9, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb2", scope: !2, file: !2, line: 9, size: 32, align: 64, flags: DIFlagPublic, elements: !4, identifier: "fb2")
    !4 = !{!5}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__SUPER", scope: !2, file: !2, baseType: !6, size: 32, align: 64, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb", scope: !2, file: !2, line: 2, size: 32, align: 64, flags: DIFlagPublic, elements: !7, identifier: "fb")
    !7 = !{!8, !10}
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 4, baseType: !9, size: 16, align: 16, flags: DIFlagPublic)
    !9 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 5, baseType: !9, size: 16, align: 16, offset: 16, flags: DIFlagPublic)
    !11 = !DIGlobalVariableExpression(var: !12, expr: !DIExpression())
    !12 = distinct !DIGlobalVariable(name: "__fb__init", scope: !2, file: !2, line: 2, type: !6, isLocal: false, isDefinition: true)
    !13 = !DIGlobalVariableExpression(var: !14, expr: !DIExpression())
    !14 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 12, type: !15, isLocal: false, isDefinition: true)
    !15 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 12, size: 32, align: 64, flags: DIFlagPublic, elements: !16, identifier: "foo")
    !16 = !{!17}
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "myFb", scope: !2, file: !2, line: 14, baseType: !3, size: 32, align: 64, flags: DIFlagPublic)
    !18 = !{i32 2, !"Dwarf Version", i32 5}
    !19 = !{i32 2, !"Debug Info Version", i32 3}
    !20 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !21, splitDebugInlining: false)
    !21 = !{!11, !0, !13}
    !22 = distinct !DICompileUnit(language: DW_LANG_C, file: !23, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !24, splitDebugInlining: false)
    !23 = !DIFile(filename: "__initializers", directory: "")
    !24 = !{!25, !27, !29}
    !25 = !DIGlobalVariableExpression(var: !26, expr: !DIExpression())
    !26 = distinct !DIGlobalVariable(name: "__fb2__init", scope: !2, file: !2, line: 9, type: !3, isLocal: false, isDefinition: true)
    !27 = !DIGlobalVariableExpression(var: !28, expr: !DIExpression())
    !28 = distinct !DIGlobalVariable(name: "__fb__init", scope: !2, file: !2, line: 2, type: !6, isLocal: false, isDefinition: true)
    !29 = !DIGlobalVariableExpression(var: !30, expr: !DIExpression())
    !30 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 12, type: !15, isLocal: false, isDefinition: true)
    !31 = distinct !DICompileUnit(language: DW_LANG_C, file: !32, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !32 = !DIFile(filename: "__init___Test", directory: "")
    !33 = distinct !DISubprogram(name: "fb", linkageName: "fb", scope: !2, file: !2, line: 2, type: !34, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !20, retainedNodes: !36)
    !34 = !DISubroutineType(flags: DIFlagPublic, types: !35)
    !35 = !{null}
    !36 = !{}
    !37 = !DILocalVariable(name: "fb", scope: !33, file: !2, line: 2, type: !6)
    !38 = !DILocation(line: 7, column: 8, scope: !33)
    !39 = distinct !DISubprogram(name: "fb2", linkageName: "fb2", scope: !2, file: !2, line: 9, type: !34, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !20, retainedNodes: !36)
    !40 = !DILocalVariable(name: "fb2", scope: !39, file: !2, line: 9, type: !3)
    !41 = !DILocation(line: 10, column: 8, scope: !39)
    !42 = distinct !DISubprogram(name: "foo", linkageName: "foo", scope: !2, file: !2, line: 12, type: !34, scopeLine: 16, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !20, retainedNodes: !36)
    !43 = !DILocalVariable(name: "foo", scope: !42, file: !2, line: 12, type: !15)
    !44 = !DILocation(line: 16, column: 12, scope: !42)
    !45 = distinct !DISubprogram(name: "__init_fb2", linkageName: "__init_fb2", scope: !2, file: !2, line: 9, type: !46, scopeLine: 9, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !22, retainedNodes: !36)
    !46 = !DISubroutineType(flags: DIFlagPublic, types: !47)
    !47 = !{null, !48}
    !48 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_fb2", baseType: !3, size: 64, align: 64, dwarfAddressSpace: 1)
    !49 = !DILocation(line: 9, column: 23, scope: !45)
    !50 = !DILocalVariable(name: "self", scope: !45, file: !2, line: 9, type: !48)
    !51 = !DILocation(line: 0, scope: !45)
    !52 = distinct !DISubprogram(name: "__init_fb", linkageName: "__init_fb", scope: !2, file: !2, line: 2, type: !53, scopeLine: 2, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !22, retainedNodes: !36)
    !53 = !DISubroutineType(flags: DIFlagPublic, types: !54)
    !54 = !{null, !55}
    !55 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_fb", baseType: !6, size: 64, align: 64, dwarfAddressSpace: 1)
    !56 = !DILocation(line: 2, column: 23, scope: !52)
    !57 = !DILocalVariable(name: "self", scope: !52, file: !2, line: 2, type: !55)
    !58 = distinct !DISubprogram(name: "__init_foo", linkageName: "__init_foo", scope: !2, file: !2, line: 12, type: !59, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !22, retainedNodes: !36)
    !59 = !DISubroutineType(flags: DIFlagPublic, types: !60)
    !60 = !{null, !61}
    !61 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_foo", baseType: !15, size: 64, align: 64, dwarfAddressSpace: 1)
    !62 = !DILocation(line: 12, column: 23, scope: !58)
    !63 = !DILocalVariable(name: "self", scope: !58, file: !2, line: 12, type: !61)
    !64 = !DILocation(line: 0, scope: !58)
    !65 = distinct !DISubprogram(name: "__init___Test", linkageName: "__init___Test", scope: !2, file: !2, type: !34, scopeLine: 1, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !31, retainedNodes: !36)
    !66 = !DILocation(line: 0, scope: !65)
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
    @__bar__init = constant %bar zeroinitializer
    @__foo__init = constant %foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @foo(%foo* %0) {
    entry:
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }

    define void @foo.baz(%foo* %0) {
    entry:
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %1 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_0, i32 0, i32 0), i32 6, i1 false)
      ret void
    }

    define void @bar(%bar* %0) {
    entry:
      %__SUPER = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %__SUPER, i32 0, i32 0
      %1 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_1, i32 0, i32 0), i32 6, i1 false)
      ret void
    }

    define void @main() {
    entry:
      %s = alloca [81 x i8], align 1
      %fb = alloca %bar, align 8
      %0 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      %1 = bitcast %bar* %fb to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 getelementptr inbounds (%bar, %bar* @__bar__init, i32 0, i32 0, i32 0, i32 0), i64 ptrtoint (%bar* getelementptr (%bar, %bar* null, i32 1) to i64), i1 false)
      call void @__init_bar(%bar* %fb)
      %__SUPER = getelementptr inbounds %bar, %bar* %fb, i32 0, i32 0
      call void @foo.baz(%foo* %__SUPER)
      call void @bar(%bar* %fb)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #1

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    define void @__init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %__SUPER = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %__SUPER)
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

    attributes #0 = { argmemonly nofree nounwind willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn writeonly }
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

    define void @grandparent(%grandparent* %0) !dbg !42 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !46, metadata !DIExpression()), !dbg !47
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0, !dbg !47
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1, !dbg !47
      ret void, !dbg !47
    }

    define void @parent(%parent* %0) !dbg !48 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !49, metadata !DIExpression()), !dbg !50
      %__SUPER = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0, !dbg !50
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1, !dbg !50
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2, !dbg !50
      ret void, !dbg !50
    }

    define void @child(%child* %0) !dbg !51 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !52, metadata !DIExpression()), !dbg !53
      %__SUPER = getelementptr inbounds %child, %child* %0, i32 0, i32 0, !dbg !53
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1, !dbg !53
      ret void, !dbg !53
    }

    define void @main() !dbg !54 {
    entry:
      %arr = alloca [11 x %child], align 8, !dbg !55
      call void @llvm.dbg.declare(metadata [11 x %child]* %arr, metadata !56, metadata !DIExpression()), !dbg !58
      %0 = bitcast [11 x %child]* %arr to i8*, !dbg !55
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([11 x %child]* getelementptr ([11 x %child], [11 x %child]* null, i32 1) to i64), i1 false), !dbg !55
      %tmpVar = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !55
      %__SUPER = getelementptr inbounds %child, %child* %tmpVar, i32 0, i32 0, !dbg !55
      %__SUPER1 = getelementptr inbounds %parent, %parent* %__SUPER, i32 0, i32 0, !dbg !55
      %a = getelementptr inbounds %grandparent, %grandparent* %__SUPER1, i32 0, i32 1, !dbg !55
      store i16 10, i16* %a, align 2, !dbg !55
      %tmpVar2 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0, !dbg !59
      %__SUPER3 = getelementptr inbounds %child, %child* %tmpVar2, i32 0, i32 0, !dbg !59
      %__SUPER4 = getelementptr inbounds %parent, %parent* %__SUPER3, i32 0, i32 0, !dbg !59
      %y = getelementptr inbounds %grandparent, %grandparent* %__SUPER4, i32 0, i32 0, !dbg !59
      %tmpVar5 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 0, !dbg !59
      store i16 20, i16* %tmpVar5, align 2, !dbg !59
      %tmpVar6 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !60
      %__SUPER7 = getelementptr inbounds %child, %child* %tmpVar6, i32 0, i32 0, !dbg !60
      %b = getelementptr inbounds %parent, %parent* %__SUPER7, i32 0, i32 2, !dbg !60
      store i16 30, i16* %b, align 2, !dbg !60
      %tmpVar8 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1, !dbg !61
      %__SUPER9 = getelementptr inbounds %child, %child* %tmpVar8, i32 0, i32 0, !dbg !61
      %x = getelementptr inbounds %parent, %parent* %__SUPER9, i32 0, i32 1, !dbg !61
      %tmpVar10 = getelementptr inbounds [11 x i16], [11 x i16]* %x, i32 0, i32 1, !dbg !61
      store i16 40, i16* %tmpVar10, align 2, !dbg !61
      %tmpVar11 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 2, !dbg !62
      %z = getelementptr inbounds %child, %child* %tmpVar11, i32 0, i32 1, !dbg !62
      %tmpVar12 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 2, !dbg !62
      store i16 50, i16* %tmpVar12, align 2, !dbg !62
      ret void, !dbg !62
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #1

    define void @__init_child(%child* %0) !dbg !63 {
    entry:
      %self = alloca %child*, align 8, !dbg !67
      call void @llvm.dbg.declare(metadata %child** %self, metadata !68, metadata !DIExpression()), !dbg !67
      store %child* %0, %child** %self, align 8, !dbg !67
      %deref = load %child*, %child** %self, align 8, !dbg !67
      %__SUPER = getelementptr inbounds %child, %child* %deref, i32 0, i32 0, !dbg !67
      call void @__init_parent(%parent* %__SUPER), !dbg !69
      ret void, !dbg !69
    }

    define void @__init_parent(%parent* %0) !dbg !70 {
    entry:
      %self = alloca %parent*, align 8, !dbg !74
      call void @llvm.dbg.declare(metadata %parent** %self, metadata !75, metadata !DIExpression()), !dbg !74
      store %parent* %0, %parent** %self, align 8, !dbg !74
      %deref = load %parent*, %parent** %self, align 8, !dbg !74
      %__SUPER = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0, !dbg !74
      call void @__init_grandparent(%grandparent* %__SUPER), !dbg !76
      ret void, !dbg !76
    }

    define void @__init_grandparent(%grandparent* %0) !dbg !77 {
    entry:
      %self = alloca %grandparent*, align 8, !dbg !81
      call void @llvm.dbg.declare(metadata %grandparent** %self, metadata !82, metadata !DIExpression()), !dbg !81
      store %grandparent* %0, %grandparent** %self, align 8, !dbg !81
      ret void, !dbg !81
    }

    define void @__init___Test() !dbg !83 {
    entry:
      ret void, !dbg !84
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn writeonly }

    !llvm.module.flags = !{!27, !28}
    !llvm.dbg.cu = !{!29, !31, !40}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 480, align: 64, flags: DIFlagPublic, elements: !4, identifier: "child")
    !4 = !{!5, !22}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__SUPER", scope: !2, file: !2, baseType: !6, size: 304, align: 64, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 304, align: 64, flags: DIFlagPublic, elements: !7, identifier: "parent")
    !7 = !{!8, !17, !21}
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "__SUPER", scope: !2, file: !2, baseType: !9, size: 112, align: 64, flags: DIFlagPublic)
    !9 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 112, align: 64, flags: DIFlagPublic, elements: !10, identifier: "grandparent")
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
    !27 = !{i32 2, !"Dwarf Version", i32 5}
    !28 = !{i32 2, !"Debug Info Version", i32 3}
    !29 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !30, splitDebugInlining: false)
    !30 = !{!25, !23, !0}
    !31 = distinct !DICompileUnit(language: DW_LANG_C, file: !32, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !33, splitDebugInlining: false)
    !32 = !DIFile(filename: "__initializers", directory: "")
    !33 = !{!34, !36, !38}
    !34 = !DIGlobalVariableExpression(var: !35, expr: !DIExpression())
    !35 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !3, isLocal: false, isDefinition: true)
    !36 = !DIGlobalVariableExpression(var: !37, expr: !DIExpression())
    !37 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !6, isLocal: false, isDefinition: true)
    !38 = !DIGlobalVariableExpression(var: !39, expr: !DIExpression())
    !39 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !9, isLocal: false, isDefinition: true)
    !40 = distinct !DICompileUnit(language: DW_LANG_C, file: !41, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !41 = !DIFile(filename: "__init___Test", directory: "")
    !42 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !43, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !45)
    !43 = !DISubroutineType(flags: DIFlagPublic, types: !44)
    !44 = !{null}
    !45 = !{}
    !46 = !DILocalVariable(name: "grandparent", scope: !42, file: !2, line: 2, type: !9)
    !47 = !DILocation(line: 7, column: 8, scope: !42)
    !48 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !43, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !45)
    !49 = !DILocalVariable(name: "parent", scope: !48, file: !2, line: 9, type: !6)
    !50 = !DILocation(line: 14, column: 8, scope: !48)
    !51 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !43, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !45)
    !52 = !DILocalVariable(name: "child", scope: !51, file: !2, line: 16, type: !3)
    !53 = !DILocation(line: 20, column: 8, scope: !51)
    !54 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 22, type: !43, scopeLine: 26, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !45)
    !55 = !DILocation(line: 26, column: 12, scope: !54)
    !56 = !DILocalVariable(name: "arr", scope: !54, file: !2, line: 24, type: !57, align: 64)
    !57 = !DICompositeType(tag: DW_TAG_array_type, baseType: !3, size: 5280, align: 64, elements: !19)
    !58 = !DILocation(line: 24, column: 12, scope: !54)
    !59 = !DILocation(line: 27, column: 12, scope: !54)
    !60 = !DILocation(line: 28, column: 12, scope: !54)
    !61 = !DILocation(line: 29, column: 12, scope: !54)
    !62 = !DILocation(line: 30, column: 12, scope: !54)
    !63 = distinct !DISubprogram(name: "__init_child", linkageName: "__init_child", scope: !2, file: !2, line: 16, type: !64, scopeLine: 16, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !31, retainedNodes: !45)
    !64 = !DISubroutineType(flags: DIFlagPublic, types: !65)
    !65 = !{null, !66}
    !66 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_child", baseType: !3, size: 64, align: 64, dwarfAddressSpace: 1)
    !67 = !DILocation(line: 16, column: 23, scope: !63)
    !68 = !DILocalVariable(name: "self", scope: !63, file: !2, line: 16, type: !66)
    !69 = !DILocation(line: 0, scope: !63)
    !70 = distinct !DISubprogram(name: "__init_parent", linkageName: "__init_parent", scope: !2, file: !2, line: 9, type: !71, scopeLine: 9, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !31, retainedNodes: !45)
    !71 = !DISubroutineType(flags: DIFlagPublic, types: !72)
    !72 = !{null, !73}
    !73 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_parent", baseType: !6, size: 64, align: 64, dwarfAddressSpace: 1)
    !74 = !DILocation(line: 9, column: 23, scope: !70)
    !75 = !DILocalVariable(name: "self", scope: !70, file: !2, line: 9, type: !73)
    !76 = !DILocation(line: 0, scope: !70)
    !77 = distinct !DISubprogram(name: "__init_grandparent", linkageName: "__init_grandparent", scope: !2, file: !2, line: 2, type: !78, scopeLine: 2, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !31, retainedNodes: !45)
    !78 = !DISubroutineType(flags: DIFlagPublic, types: !79)
    !79 = !{null, !80}
    !80 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_grandparent", baseType: !9, size: 64, align: 64, dwarfAddressSpace: 1)
    !81 = !DILocation(line: 2, column: 23, scope: !77)
    !82 = !DILocalVariable(name: "self", scope: !77, file: !2, line: 2, type: !80)
    !83 = distinct !DISubprogram(name: "__init___Test", linkageName: "__init___Test", scope: !2, file: !2, type: !43, scopeLine: 1, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !40, retainedNodes: !45)
    !84 = !DILocation(line: 0, scope: !83)
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

    define void @grandparent(%grandparent* %0) !dbg !42 {
    entry:
      call void @llvm.dbg.declare(metadata %grandparent* %0, metadata !46, metadata !DIExpression()), !dbg !47
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0, !dbg !47
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1, !dbg !47
      ret void, !dbg !47
    }

    define void @parent(%parent* %0) !dbg !48 {
    entry:
      call void @llvm.dbg.declare(metadata %parent* %0, metadata !49, metadata !DIExpression()), !dbg !50
      %__SUPER = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0, !dbg !50
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1, !dbg !50
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2, !dbg !50
      ret void, !dbg !50
    }

    define void @child(%child* %0) !dbg !51 {
    entry:
      call void @llvm.dbg.declare(metadata %child* %0, metadata !52, metadata !DIExpression()), !dbg !53
      %__SUPER = getelementptr inbounds %child, %child* %0, i32 0, i32 0, !dbg !53
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1, !dbg !53
      %__SUPER1 = getelementptr inbounds %parent, %parent* %__SUPER, i32 0, i32 0, !dbg !53
      %y = getelementptr inbounds %grandparent, %grandparent* %__SUPER1, i32 0, i32 0, !dbg !53
      %b = getelementptr inbounds %parent, %parent* %__SUPER, i32 0, i32 2, !dbg !53
      %load_b = load i16, i16* %b, align 2, !dbg !53
      %1 = sext i16 %load_b to i32, !dbg !53
      %b2 = getelementptr inbounds %parent, %parent* %__SUPER, i32 0, i32 2, !dbg !53
      %load_b3 = load i16, i16* %b2, align 2, !dbg !53
      %2 = sext i16 %load_b3 to i32, !dbg !53
      %tmpVar = mul i32 %2, 2, !dbg !53
      %tmpVar4 = mul i32 1, %tmpVar, !dbg !53
      %tmpVar5 = add i32 %tmpVar4, 0, !dbg !53
      %tmpVar6 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 %tmpVar5, !dbg !53
      %load_tmpVar = load i16, i16* %tmpVar6, align 2, !dbg !53
      %3 = sext i16 %load_tmpVar to i32, !dbg !53
      %tmpVar7 = add i32 %1, %3, !dbg !53
      %__SUPER8 = getelementptr inbounds %parent, %parent* %__SUPER, i32 0, i32 0, !dbg !53
      %a = getelementptr inbounds %grandparent, %grandparent* %__SUPER8, i32 0, i32 1, !dbg !53
      %load_a = load i16, i16* %a, align 2, !dbg !53
      %4 = sext i16 %load_a to i32, !dbg !53
      %tmpVar9 = sub i32 %tmpVar7, %4, !dbg !53
      %tmpVar10 = mul i32 1, %tmpVar9, !dbg !53
      %tmpVar11 = add i32 %tmpVar10, 0, !dbg !53
      %tmpVar12 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 %tmpVar11, !dbg !53
      store i16 20, i16* %tmpVar12, align 2, !dbg !53
      ret void, !dbg !53
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init_parent(%parent* %0) !dbg !54 {
    entry:
      %self = alloca %parent*, align 8, !dbg !58
      call void @llvm.dbg.declare(metadata %parent** %self, metadata !59, metadata !DIExpression()), !dbg !58
      store %parent* %0, %parent** %self, align 8, !dbg !58
      %deref = load %parent*, %parent** %self, align 8, !dbg !58
      %__SUPER = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0, !dbg !58
      call void @__init_grandparent(%grandparent* %__SUPER), !dbg !60
      ret void, !dbg !60
    }

    define void @__init_grandparent(%grandparent* %0) !dbg !61 {
    entry:
      %self = alloca %grandparent*, align 8, !dbg !65
      call void @llvm.dbg.declare(metadata %grandparent** %self, metadata !66, metadata !DIExpression()), !dbg !65
      store %grandparent* %0, %grandparent** %self, align 8, !dbg !65
      ret void, !dbg !65
    }

    define void @__init_child(%child* %0) !dbg !67 {
    entry:
      %self = alloca %child*, align 8, !dbg !71
      call void @llvm.dbg.declare(metadata %child** %self, metadata !72, metadata !DIExpression()), !dbg !71
      store %child* %0, %child** %self, align 8, !dbg !71
      %deref = load %child*, %child** %self, align 8, !dbg !71
      %__SUPER = getelementptr inbounds %child, %child* %deref, i32 0, i32 0, !dbg !71
      call void @__init_parent(%parent* %__SUPER), !dbg !73
      ret void, !dbg !73
    }

    define void @__init___Test() !dbg !74 {
    entry:
      ret void, !dbg !75
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!27, !28}
    !llvm.dbg.cu = !{!29, !31, !40}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "parent", scope: !2, file: !2, line: 9, size: 304, align: 64, flags: DIFlagPublic, elements: !4, identifier: "parent")
    !4 = !{!5, !14, !18}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "__SUPER", scope: !2, file: !2, baseType: !6, size: 112, align: 64, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "grandparent", scope: !2, file: !2, line: 2, size: 112, align: 64, flags: DIFlagPublic, elements: !7, identifier: "grandparent")
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
    !23 = !DICompositeType(tag: DW_TAG_structure_type, name: "child", scope: !2, file: !2, line: 16, size: 480, align: 64, flags: DIFlagPublic, elements: !24, identifier: "child")
    !24 = !{!25, !26}
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "__SUPER", scope: !2, file: !2, baseType: !3, size: 304, align: 64, flags: DIFlagPublic)
    !26 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 18, baseType: !15, size: 176, align: 16, offset: 304, flags: DIFlagPublic)
    !27 = !{i32 2, !"Dwarf Version", i32 5}
    !28 = !{i32 2, !"Debug Info Version", i32 3}
    !29 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !30, splitDebugInlining: false)
    !30 = !{!19, !0, !21}
    !31 = distinct !DICompileUnit(language: DW_LANG_C, file: !32, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !33, splitDebugInlining: false)
    !32 = !DIFile(filename: "__initializers", directory: "")
    !33 = !{!34, !36, !38}
    !34 = !DIGlobalVariableExpression(var: !35, expr: !DIExpression())
    !35 = distinct !DIGlobalVariable(name: "__parent__init", scope: !2, file: !2, line: 9, type: !3, isLocal: false, isDefinition: true)
    !36 = !DIGlobalVariableExpression(var: !37, expr: !DIExpression())
    !37 = distinct !DIGlobalVariable(name: "__grandparent__init", scope: !2, file: !2, line: 2, type: !6, isLocal: false, isDefinition: true)
    !38 = !DIGlobalVariableExpression(var: !39, expr: !DIExpression())
    !39 = distinct !DIGlobalVariable(name: "__child__init", scope: !2, file: !2, line: 16, type: !23, isLocal: false, isDefinition: true)
    !40 = distinct !DICompileUnit(language: DW_LANG_C, file: !41, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !41 = !DIFile(filename: "__init___Test", directory: "")
    !42 = distinct !DISubprogram(name: "grandparent", linkageName: "grandparent", scope: !2, file: !2, line: 2, type: !43, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !45)
    !43 = !DISubroutineType(flags: DIFlagPublic, types: !44)
    !44 = !{null}
    !45 = !{}
    !46 = !DILocalVariable(name: "grandparent", scope: !42, file: !2, line: 2, type: !6)
    !47 = !DILocation(line: 7, column: 8, scope: !42)
    !48 = distinct !DISubprogram(name: "parent", linkageName: "parent", scope: !2, file: !2, line: 9, type: !43, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !45)
    !49 = !DILocalVariable(name: "parent", scope: !48, file: !2, line: 9, type: !3)
    !50 = !DILocation(line: 14, column: 8, scope: !48)
    !51 = distinct !DISubprogram(name: "child", linkageName: "child", scope: !2, file: !2, line: 16, type: !43, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !29, retainedNodes: !45)
    !52 = !DILocalVariable(name: "child", scope: !51, file: !2, line: 16, type: !23)
    !53 = !DILocation(line: 20, column: 12, scope: !51)
    !54 = distinct !DISubprogram(name: "__init_parent", linkageName: "__init_parent", scope: !2, file: !2, line: 9, type: !55, scopeLine: 9, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !31, retainedNodes: !45)
    !55 = !DISubroutineType(flags: DIFlagPublic, types: !56)
    !56 = !{null, !57}
    !57 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_parent", baseType: !3, size: 64, align: 64, dwarfAddressSpace: 1)
    !58 = !DILocation(line: 9, column: 23, scope: !54)
    !59 = !DILocalVariable(name: "self", scope: !54, file: !2, line: 9, type: !57)
    !60 = !DILocation(line: 0, scope: !54)
    !61 = distinct !DISubprogram(name: "__init_grandparent", linkageName: "__init_grandparent", scope: !2, file: !2, line: 2, type: !62, scopeLine: 2, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !31, retainedNodes: !45)
    !62 = !DISubroutineType(flags: DIFlagPublic, types: !63)
    !63 = !{null, !64}
    !64 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_grandparent", baseType: !6, size: 64, align: 64, dwarfAddressSpace: 1)
    !65 = !DILocation(line: 2, column: 23, scope: !61)
    !66 = !DILocalVariable(name: "self", scope: !61, file: !2, line: 2, type: !64)
    !67 = distinct !DISubprogram(name: "__init_child", linkageName: "__init_child", scope: !2, file: !2, line: 16, type: !68, scopeLine: 16, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !31, retainedNodes: !45)
    !68 = !DISubroutineType(flags: DIFlagPublic, types: !69)
    !69 = !{null, !70}
    !70 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_child", baseType: !23, size: 64, align: 64, dwarfAddressSpace: 1)
    !71 = !DILocation(line: 16, column: 23, scope: !67)
    !72 = !DILocalVariable(name: "self", scope: !67, file: !2, line: 16, type: !70)
    !73 = !DILocation(line: 0, scope: !67)
    !74 = distinct !DISubprogram(name: "__init___Test", linkageName: "__init___Test", scope: !2, file: !2, type: !43, scopeLine: 1, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !40, retainedNodes: !45)
    !75 = !DILocation(line: 0, scope: !74)
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
    insta::assert_snapshot!(result, @r#""#);
}
