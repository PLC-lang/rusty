use plc_util::filtered_assert_snapshot;

mod expression_debugging;

use test_utils::codegen_with_debug as codegen;
use test_utils::codegen_with_debug_version;

#[test]
fn test_global_var_int_added_to_debug_info() {
    let codegen = codegen(
        r#"
    VAR_GLOBAL
        a : SINT; //8bit
        b : USINT; //8bit
        c : INT; //16bit
        d : UINT; //16bit
        e : DINT; //32bit
        f : UDINT; //32bit
        g : LINT; //64bit
        h : ULINT; //64bit
    END_VAR
    "#,
    );

    filtered_assert_snapshot!(codegen)
}

#[test]
fn test_global_var_byteseq_added_to_debug_info() {
    let codegen = codegen(
        r#"
    VAR_GLOBAL
        a : BOOL; //8bit DW_ATE_boolean
        b : BYTE; //8bit
        c : WORD; //16bit
        d : DWORD; //32bit
        e : LWORD; //64bit
    END_VAR
    "#,
    );

    filtered_assert_snapshot!(codegen)
}

#[test]
fn test_global_var_enum_added_to_debug_info() {
    //Multiple types
    let codegen = codegen(
        r#"
    TYPE en1 : (a,b,c); END_TYPE
    TYPE en2 : BYTE (d,e,f); END_TYPE
    VAR_GLOBAL
        en3 : LINT (a,b,c);
    END_VAR
    "#,
    );

    filtered_assert_snapshot!(codegen)
}

#[test]
fn test_global_var_float_added_to_debug_info() {
    let codegen = codegen(
        r#"
    VAR_GLOBAL
        a : REAL;
        b : LREAL;
    END_VAR
    "#,
    );

    filtered_assert_snapshot!(codegen)
}

#[test]
fn test_global_var_array_added_to_debug_info() {
    let codegen = codegen(
        r#"
    VAR_GLOBAL
        a : ARRAY[0..10] OF DINT;
        b : ARRAY[0..10, 11..20] OF DINT;
        c : ARRAY[0..10] OF ARRAY[11..20] OF DINT;
    END_VAR
    "#,
    );
    filtered_assert_snapshot!(codegen)
}

#[test]
fn test_global_var_pointer_added_to_debug_info() {
    let codegen = codegen(
        r#"
    VAR_GLOBAL
        a : REF_TO DINT;
        b : REF_TO ARRAY[0..10] OF DINT;
    END_VAR
    "#,
    );
    filtered_assert_snapshot!(codegen)
}

#[test]
fn test_global_var_string_added_to_debug_info() {
    let codegen = codegen(
        r#"
    VAR_GLOBAL
        a : STRING;
        b : WSTRING;
    END_VAR
    "#,
    );
    filtered_assert_snapshot!(codegen)
}

#[test]
fn test_global_var_struct_added_to_debug_info() {
    let codegen = codegen(
        r#"
    TYPE myStruct : STRUCT
        a : DINT;
        b : LREAL;
        c : ARRAY[0..10] OF DINT;
        // d : STRING;
    END_STRUCT
    END_TYPE

    VAR_GLOBAL
        gStruct : myStruct;
        b : ARRAY[0..10] OF myStruct;
    END_VAR
    "#,
    );
    filtered_assert_snapshot!(codegen)
}

#[test]
fn test_global_var_nested_struct_added_to_debug_info() {
    let codegen = codegen(
        r#"
    TYPE myStruct : STRUCT
        a : DINT;
        b : myStruct2;
    END_STRUCT
    END_TYPE

    TYPE myStruct2 : STRUCT
        a : DINT;
        b : LREAL;
    END_STRUCT
    END_TYPE

    VAR_GLOBAL
        gStruct : myStruct;
    END_VAR
    "#,
    );
    filtered_assert_snapshot!(codegen)
}

#[test]
fn test_self_referential_struct_debug_info() {
    // This test verifies that self-referential types (structs containing pointers to themselves)
    // don't cause infinite recursion / stack overflow when generating debug info.
    // If this test completes without stack overflow, it passes.
    let _ = codegen(
        r#"
    TYPE Node : STRUCT
        data : DINT;
        next : REF_TO Node;
    END_STRUCT END_TYPE

    VAR_GLOBAL
        node : Node;
    END_VAR
    "#,
    );
}

#[test]
fn test_global_alias_type() {
    let codegen = codegen(
        r#"
    TYPE myInt : DINT; END_TYPE

    VAR_GLOBAL
        gInt : myInt;
    END_VAR
    "#,
    );

    filtered_assert_snapshot!(codegen)
}

#[test]
fn test_dwarf_version_override() {
    let codegen = codegen_with_debug_version(
        r#"
    TYPE myInt : DINT; END_TYPE

    VAR_GLOBAL
        gInt : myInt;
    END_VAR
    "#,
        4,
    );

    filtered_assert_snapshot!(codegen)
}

#[test]
fn switch_case_debug_info() {
    let codegen = codegen(
        r#"
        FUNCTION main : DINT
            VAR
                x1 : INT;
                x2 : INT;
                x3 : INT;
            END_VAR

            WHILE TRUE DO
            x1 := x1 + 1;

            CASE x1 OF
                1: x2 := 1;
                2: x2 := 2;
                3: x2 := 3;
                ELSE
                    x1 := 0;
                    x2 := 1;
                    x3 := 2;
            END_CASE

            END_WHILE

        END_FUNCTION
        "#,
    );

    filtered_assert_snapshot!(codegen, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define i32 @main() !dbg !4 {
    entry:
      %main = alloca i32, align [filtered]
      %x1 = alloca i16, align [filtered]
      %x2 = alloca i16, align [filtered]
      %x3 = alloca i16, align [filtered]
        #dbg_declare(ptr %x1, !8, !DIExpression(), !10)
      store i16 0, ptr %x1, align [filtered]
        #dbg_declare(ptr %x2, !11, !DIExpression(), !12)
      store i16 0, ptr %x2, align [filtered]
        #dbg_declare(ptr %x3, !13, !DIExpression(), !14)
      store i16 0, ptr %x3, align [filtered]
        #dbg_declare(ptr %main, !15, !DIExpression(), !17)
      store i32 0, ptr %main, align [filtered]
      br label %condition_check, !dbg !18

    condition_check:                                  ; preds = %continue2, %entry
      br i1 true, label %while_body, label %continue, !dbg !19

    while_body:                                       ; preds = %condition_check
      br i1 false, label %condition_body, label %continue1, !dbg !19

    continue:                                         ; preds = %condition_body, %condition_check
      %main_ret = load i32, ptr %main, align [filtered], !dbg !20
      ret i32 %main_ret, !dbg !20

    condition_body:                                   ; preds = %while_body
      br label %continue, !dbg !19

    buffer_block:                                     ; No predecessors!
      br label %continue1, !dbg !21

    continue1:                                        ; preds = %buffer_block, %while_body
      %load_x1 = load i16, ptr %x1, align [filtered], !dbg !22
      %0 = sext i16 %load_x1 to i32, !dbg !22
      %tmpVar = add i32 %0, 1, !dbg !22
      %1 = trunc i32 %tmpVar to i16, !dbg !22
      store i16 %1, ptr %x1, align [filtered], !dbg !22
      %load_x13 = load i16, ptr %x1, align [filtered], !dbg !22
      switch i16 %load_x13, label %else [
        i16 1, label %case
        i16 2, label %case4
        i16 3, label %case5
      ], !dbg !23

    case:                                             ; preds = %continue1
      store i16 1, ptr %x2, align [filtered], !dbg !24
      br label %continue2, !dbg !25

    case4:                                            ; preds = %continue1
      store i16 2, ptr %x2, align [filtered], !dbg !26
      br label %continue2, !dbg !25

    case5:                                            ; preds = %continue1
      store i16 3, ptr %x2, align [filtered], !dbg !27
      br label %continue2, !dbg !25

    else:                                             ; preds = %continue1
      store i16 0, ptr %x1, align [filtered], !dbg !28
      store i16 1, ptr %x2, align [filtered], !dbg !29
      store i16 2, ptr %x3, align [filtered], !dbg !30
      br label %continue2, !dbg !25

    continue2:                                        ; preds = %else, %case5, %case4, %case
      br label %condition_check, !dbg !18
    }

    !llvm.module.flags = !{!0, !1}
    !llvm.dbg.cu = !{!2}

    !0 = !{i32 2, !"Dwarf Version", i32 5}
    !1 = !{i32 2, !"Debug Info Version", i32 3}
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "<internal>", directory: "")
    !4 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !3, file: !3, line: 2, type: !5, scopeLine: 9, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !7)
    !5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
    !6 = !{null}
    !7 = !{}
    !8 = !DILocalVariable(name: "x1", scope: !4, file: !3, line: 4, type: !9, align [filtered])
    !9 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !10 = !DILocation(line: 4, column: 16, scope: !4)
    !11 = !DILocalVariable(name: "x2", scope: !4, file: !3, line: 5, type: !9, align [filtered])
    !12 = !DILocation(line: 5, column: 16, scope: !4)
    !13 = !DILocalVariable(name: "x3", scope: !4, file: !3, line: 6, type: !9, align [filtered])
    !14 = !DILocation(line: 6, column: 16, scope: !4)
    !15 = !DILocalVariable(name: "main", scope: !4, file: !3, line: 2, type: !16, align [filtered])
    !16 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !17 = !DILocation(line: 2, column: 17, scope: !4)
    !18 = !DILocation(line: 22, column: 12, scope: !4)
    !19 = !DILocation(line: 9, column: 18, scope: !4)
    !20 = !DILocation(line: 24, column: 8, scope: !4)
    !21 = !DILocation(line: 0, scope: !4)
    !22 = !DILocation(line: 10, column: 12, scope: !4)
    !23 = !DILocation(line: 12, column: 17, scope: !4)
    !24 = !DILocation(line: 13, column: 19, scope: !4)
    !25 = !DILocation(line: 20, column: 12, scope: !4)
    !26 = !DILocation(line: 14, column: 19, scope: !4)
    !27 = !DILocation(line: 15, column: 19, scope: !4)
    !28 = !DILocation(line: 17, column: 20, scope: !4)
    !29 = !DILocation(line: 18, column: 20, scope: !4)
    !30 = !DILocation(line: 19, column: 20, scope: !4)
    "#);
}

#[test]
fn dbg_declare_has_valid_metadata_references_for_methods() {
    let codegen = codegen(
        r"
        FUNCTION_BLOCK fb
        METHOD foo
        END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    filtered_assert_snapshot!(codegen, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_fb = type { ptr, ptr }
    %fb = type { ptr }

    @__vtable_fb_instance = global %__vtable_fb zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @fb(ptr %0) !dbg !4 {
    entry:
        #dbg_declare(ptr %0, !14, !DIExpression(), !15)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 0
      ret void, !dbg !15
    }

    define void @fb__foo(ptr %0) !dbg !16 {
    entry:
        #dbg_declare(ptr %0, !17, !DIExpression(), !18)
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 0
      ret void, !dbg !18
    }

    define void @fb__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !15
      store ptr %0, ptr %self, align [filtered], !dbg !15
      %deref = load ptr, ptr %self, align [filtered], !dbg !15
      %__vtable = getelementptr inbounds nuw %fb, ptr %deref, i32 0, i32 0, !dbg !15
      call void @__fb___vtable__ctor(ptr %__vtable), !dbg !15
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !15
      %__vtable2 = getelementptr inbounds nuw %fb, ptr %deref1, i32 0, i32 0, !dbg !15
      store ptr @__vtable_fb_instance, ptr %__vtable2, align [filtered], !dbg !15
      ret void, !dbg !15
    }

    define void @__vtable_fb__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !15
      store ptr %0, ptr %self, align [filtered], !dbg !15
      %deref = load ptr, ptr %self, align [filtered], !dbg !15
      %__body = getelementptr inbounds nuw %__vtable_fb, ptr %deref, i32 0, i32 0, !dbg !15
      call void @____vtable_fb___body__ctor(ptr %__body), !dbg !15
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !15
      %__body2 = getelementptr inbounds nuw %__vtable_fb, ptr %deref1, i32 0, i32 0, !dbg !15
      store ptr @fb, ptr %__body2, align [filtered], !dbg !15
      %deref3 = load ptr, ptr %self, align [filtered], !dbg !15
      %foo = getelementptr inbounds nuw %__vtable_fb, ptr %deref3, i32 0, i32 1, !dbg !15
      call void @____vtable_fb_foo__ctor(ptr %foo), !dbg !15
      %deref4 = load ptr, ptr %self, align [filtered], !dbg !15
      %foo5 = getelementptr inbounds nuw %__vtable_fb, ptr %deref4, i32 0, i32 1, !dbg !15
      store ptr @fb__foo, ptr %foo5, align [filtered], !dbg !15
      ret void, !dbg !15
    }

    define void @__fb___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !15
      store ptr %0, ptr %self, align [filtered], !dbg !15
      ret void, !dbg !15
    }

    define void @____vtable_fb___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !15
      store ptr %0, ptr %self, align [filtered], !dbg !15
      ret void, !dbg !15
    }

    define void @____vtable_fb_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !15
      store ptr %0, ptr %self, align [filtered], !dbg !15
      ret void, !dbg !15
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_fb__ctor(ptr @__vtable_fb_instance), !dbg !15
      ret void, !dbg !15
    }

    !llvm.module.flags = !{!0, !1}
    !llvm.dbg.cu = !{!2}

    !0 = !{i32 2, !"Dwarf Version", i32 5}
    !1 = !{i32 2, !"Debug Info Version", i32 3}
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "<internal>", directory: "")
    !4 = distinct !DISubprogram(name: "fb", linkageName: "fb", scope: !3, file: !3, line: 2, type: !5, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !13)
    !5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
    !6 = !{null, !7}
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb", scope: !3, file: !3, line: 2, size: 64, align [filtered], flags: DIFlagPublic, elements: !8, identifier: "fb")
    !8 = !{!9}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !3, file: !3, baseType: !10, size: 64, align [filtered], flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____fb___vtable", scope: !3, file: !3, baseType: !11, align [filtered])
    !11 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__fb___vtable", baseType: !12, size: 64, align [filtered], dwarfAddressSpace: 1)
    !12 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !13 = !{}
    !14 = !DILocalVariable(name: "fb", scope: !4, file: !3, line: 5, type: !7)
    !15 = !DILocation(line: 5, column: 8, scope: !4)
    !16 = distinct !DISubprogram(name: "fb.foo", linkageName: "fb.foo", scope: !4, file: !3, line: 3, type: !5, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !13)
    !17 = !DILocalVariable(name: "fb", scope: !16, file: !3, line: 4, type: !7)
    !18 = !DILocation(line: 4, column: 8, scope: !16)
    "#);
}

#[test]
fn action_with_var_temp() {
    let codegen = codegen(
        r"
        FUNCTION main : DINT
            PLC_PRG();
            PLC_PRG.act();
        END_FUNCTION

        PROGRAM PLC_PRG
        VAR_TEMP
            x : DINT;
        END_VAR

            x := 0;
        END_PROGRAM

        ACTIONS
            ACTION act
                x := x + 1;
            END_ACTION
        END_ACTIONS
        ",
    );

    filtered_assert_snapshot!(codegen, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %PLC_PRG = type {}

    @PLC_PRG_instance = global %PLC_PRG zeroinitializer, !dbg !0
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define i32 @main() !dbg !9 {
    entry:
      %main = alloca i32, align [filtered]
        #dbg_declare(ptr %main, !12, !DIExpression(), !14)
      store i32 0, ptr %main, align [filtered]
      call void @PLC_PRG(ptr @PLC_PRG_instance), !dbg !15
      call void @PLC_PRG__act(ptr @PLC_PRG_instance), !dbg !16
      %main_ret = load i32, ptr %main, align [filtered], !dbg !17
      ret i32 %main_ret, !dbg !17
    }

    define void @PLC_PRG(ptr %0) !dbg !18 {
    entry:
        #dbg_declare(ptr %0, !21, !DIExpression(), !22)
      %x = alloca i32, align [filtered]
        #dbg_declare(ptr %x, !23, !DIExpression(), !24)
      store i32 0, ptr %x, align [filtered]
      store i32 0, ptr %x, align [filtered], !dbg !22
      ret void, !dbg !25
    }

    define void @PLC_PRG__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !26
      store ptr %0, ptr %self, align [filtered], !dbg !26
      ret void, !dbg !26
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @PLC_PRG__ctor(ptr @PLC_PRG_instance), !dbg !26
      ret void, !dbg !26
    }

    define void @PLC_PRG__act(ptr %0) !dbg !27 {
    entry:
        #dbg_declare(ptr %0, !28, !DIExpression(), !29)
      %x = alloca i32, align [filtered]
        #dbg_declare(ptr %x, !30, !DIExpression(), !31)
      store i32 0, ptr %x, align [filtered]
      %load_x = load i32, ptr %x, align [filtered], !dbg !29
      %tmpVar = add i32 %load_x, 1, !dbg !29
      store i32 %tmpVar, ptr %x, align [filtered], !dbg !29
      ret void, !dbg !26
    }

    !llvm.module.flags = !{!5, !6}
    !llvm.dbg.cu = !{!7}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "PLC_PRG", scope: !2, file: !2, line: 7, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "PLC_PRG", scope: !2, file: !2, line: 7, align [filtered], flags: DIFlagPublic, elements: !4, identifier: "PLC_PRG")
    !4 = !{}
    !5 = !{i32 2, !"Dwarf Version", i32 5}
    !6 = !{i32 2, !"Debug Info Version", i32 3}
    !7 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !8, splitDebugInlining: false)
    !8 = !{!0}
    !9 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 2, type: !10, scopeLine: 3, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !4)
    !10 = !DISubroutineType(flags: DIFlagPublic, types: !11)
    !11 = !{null}
    !12 = !DILocalVariable(name: "main", scope: !9, file: !2, line: 2, type: !13, align [filtered])
    !13 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !14 = !DILocation(line: 2, column: 17, scope: !9)
    !15 = !DILocation(line: 3, column: 12, scope: !9)
    !16 = !DILocation(line: 4, column: 12, scope: !9)
    !17 = !DILocation(line: 5, column: 8, scope: !9)
    !18 = distinct !DISubprogram(name: "PLC_PRG", linkageName: "PLC_PRG", scope: !2, file: !2, line: 7, type: !19, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !4)
    !19 = !DISubroutineType(flags: DIFlagPublic, types: !20)
    !20 = !{null, !3}
    !21 = !DILocalVariable(name: "PLC_PRG", scope: !18, file: !2, line: 12, type: !3)
    !22 = !DILocation(line: 12, column: 12, scope: !18)
    !23 = !DILocalVariable(name: "x", scope: !18, file: !2, line: 9, type: !13, align [filtered])
    !24 = !DILocation(line: 9, column: 12, scope: !18)
    !25 = !DILocation(line: 13, column: 8, scope: !18)
    !26 = !DILocation(line: 18, column: 12, scope: !27)
    !27 = distinct !DISubprogram(name: "PLC_PRG.act", linkageName: "PLC_PRG.act", scope: !2, file: !2, line: 16, type: !19, scopeLine: 17, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !4)
    !28 = !DILocalVariable(name: "PLC_PRG", scope: !27, file: !2, line: 17, type: !3)
    !29 = !DILocation(line: 17, column: 16, scope: !27)
    !30 = !DILocalVariable(name: "x", scope: !27, file: !2, line: 9, type: !13, align [filtered])
    !31 = !DILocation(line: 9, column: 12, scope: !27)
    "#);
}

#[test]
fn nested_array_struct_sizes() {
    let result = codegen(
        "
TYPE struct_ : STRUCT
        inner: inner;
        inner_arr: ARRAY[0..2] OF inner;
        s : STRING := 'Hello';
        b : BOOL := TRUE;
        r : REAL := 3.1415;
        arr: ARRAY[0..2] OF STRING := ['aa', 'bb', 'cc'];
        i : INT := 42;
    END_STRUCT
END_TYPE

TYPE inner : STRUCT
        s : STRING := 'Hello';
        b : BOOL := TRUE;
        r : REAL := 3.1415;
        arr: ARRAY[0..2] OF STRING := ['aaaa', 'bbbb', 'cccc'];
        i : INT := 42;
    END_STRUCT
END_TYPE

FUNCTION main
VAR
    st: struct_;
    s : STRING;
    b : BOOL;
    arr: ARRAY[0..2] OF STRING;
    i : INT;
END_VAR


    s := st.s;
    s := st.inner.s;
    b := st.b;
    b := st.inner.b;
    arr := st.arr;
    arr := st.inner.arr;
    i := st.i;
    i := st.inner.i;
    // arr := ['', '', ''];
    arr[0] := st.arr[0];
    arr[1] := st.inner.arr[1];
    arr[2] := st.inner.arr[2];

END_FUNCTION
    ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %struct_ = type { %inner, [3 x %inner], [81 x i8], i8, float, [3 x [81 x i8]], i16 }
    %inner = type { [81 x i8], i8, float, [3 x [81 x i8]], i16 }

    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"Hello\00"
    @utf08_literal_1 = private unnamed_addr constant [3 x i8] c"aa\00"
    @utf08_literal_2 = private unnamed_addr constant [5 x i8] c"aaaa\00"
    @utf08_literal_3 = private unnamed_addr constant [3 x i8] c"bb\00"
    @utf08_literal_4 = private unnamed_addr constant [5 x i8] c"bbbb\00"
    @utf08_literal_5 = private unnamed_addr constant [3 x i8] c"cc\00"
    @utf08_literal_6 = private unnamed_addr constant [5 x i8] c"cccc\00"

    define void @main() !dbg !4 {
    entry:
      %st = alloca %struct_, align [filtered]
      %s = alloca [81 x i8], align [filtered]
      %b = alloca i8, align [filtered]
      %arr = alloca [3 x [81 x i8]], align [filtered]
      %i = alloca i16, align [filtered]
        #dbg_declare(ptr %st, !8, !DIExpression(), !37)
      call void @llvm.memset.p0.i64(ptr align [filtered] %st, i8 0, i64 ptrtoint (ptr getelementptr (%struct_, ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %s, !38, !DIExpression(), !39)
      call void @llvm.memset.p0.i64(ptr align [filtered] %s, i8 0, i64 ptrtoint (ptr getelementptr ([81 x i8], ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %b, !40, !DIExpression(), !41)
      store i8 0, ptr %b, align [filtered]
        #dbg_declare(ptr %arr, !42, !DIExpression(), !43)
      call void @llvm.memset.p0.i64(ptr align [filtered] %arr, i8 0, i64 ptrtoint (ptr getelementptr ([3 x [81 x i8]], ptr null, i32 1) to i64), i1 false)
        #dbg_declare(ptr %i, !44, !DIExpression(), !45)
      store i16 0, ptr %i, align [filtered]
      call void @struct___ctor(ptr %st), !dbg !46
      call void @__main_arr__ctor(ptr %arr), !dbg !46
      %s1 = getelementptr inbounds nuw %struct_, ptr %st, i32 0, i32 2, !dbg !47
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %s, ptr align [filtered] %s1, i32 80, i1 false), !dbg !47
      %inner = getelementptr inbounds nuw %struct_, ptr %st, i32 0, i32 0, !dbg !48
      %s2 = getelementptr inbounds nuw %inner, ptr %inner, i32 0, i32 0, !dbg !48
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %s, ptr align [filtered] %s2, i32 80, i1 false), !dbg !48
      %b3 = getelementptr inbounds nuw %struct_, ptr %st, i32 0, i32 3, !dbg !49
      %load_b = load i8, ptr %b3, align [filtered], !dbg !49
      store i8 %load_b, ptr %b, align [filtered], !dbg !49
      %inner4 = getelementptr inbounds nuw %struct_, ptr %st, i32 0, i32 0, !dbg !50
      %b5 = getelementptr inbounds nuw %inner, ptr %inner4, i32 0, i32 1, !dbg !50
      %load_b6 = load i8, ptr %b5, align [filtered], !dbg !50
      store i8 %load_b6, ptr %b, align [filtered], !dbg !50
      %arr7 = getelementptr inbounds nuw %struct_, ptr %st, i32 0, i32 5, !dbg !51
      call void @llvm.memcpy.p0.p0.i64(ptr align [filtered] %arr, ptr align [filtered] %arr7, i64 ptrtoint (ptr getelementptr ([3 x [81 x i8]], ptr null, i32 1) to i64), i1 false), !dbg !51
      %inner8 = getelementptr inbounds nuw %struct_, ptr %st, i32 0, i32 0, !dbg !52
      %arr9 = getelementptr inbounds nuw %inner, ptr %inner8, i32 0, i32 3, !dbg !52
      call void @llvm.memcpy.p0.p0.i64(ptr align [filtered] %arr, ptr align [filtered] %arr9, i64 ptrtoint (ptr getelementptr ([3 x [81 x i8]], ptr null, i32 1) to i64), i1 false), !dbg !52
      %i10 = getelementptr inbounds nuw %struct_, ptr %st, i32 0, i32 6, !dbg !53
      %load_i = load i16, ptr %i10, align [filtered], !dbg !53
      store i16 %load_i, ptr %i, align [filtered], !dbg !53
      %inner11 = getelementptr inbounds nuw %struct_, ptr %st, i32 0, i32 0, !dbg !54
      %i12 = getelementptr inbounds nuw %inner, ptr %inner11, i32 0, i32 4, !dbg !54
      %load_i13 = load i16, ptr %i12, align [filtered], !dbg !54
      store i16 %load_i13, ptr %i, align [filtered], !dbg !54
      %tmpVar = getelementptr inbounds [3 x [81 x i8]], ptr %arr, i32 0, i32 0, !dbg !55
      %arr14 = getelementptr inbounds nuw %struct_, ptr %st, i32 0, i32 5, !dbg !55
      %tmpVar15 = getelementptr inbounds [3 x [81 x i8]], ptr %arr14, i32 0, i32 0, !dbg !55
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %tmpVar, ptr align [filtered] %tmpVar15, i32 80, i1 false), !dbg !55
      %tmpVar16 = getelementptr inbounds [3 x [81 x i8]], ptr %arr, i32 0, i32 1, !dbg !56
      %inner17 = getelementptr inbounds nuw %struct_, ptr %st, i32 0, i32 0, !dbg !56
      %arr18 = getelementptr inbounds nuw %inner, ptr %inner17, i32 0, i32 3, !dbg !56
      %tmpVar19 = getelementptr inbounds [3 x [81 x i8]], ptr %arr18, i32 0, i32 1, !dbg !56
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %tmpVar16, ptr align [filtered] %tmpVar19, i32 80, i1 false), !dbg !56
      %tmpVar20 = getelementptr inbounds [3 x [81 x i8]], ptr %arr, i32 0, i32 2, !dbg !57
      %inner21 = getelementptr inbounds nuw %struct_, ptr %st, i32 0, i32 0, !dbg !57
      %arr22 = getelementptr inbounds nuw %inner, ptr %inner21, i32 0, i32 3, !dbg !57
      %tmpVar23 = getelementptr inbounds [3 x [81 x i8]], ptr %arr22, i32 0, i32 2, !dbg !57
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %tmpVar20, ptr align [filtered] %tmpVar23, i32 80, i1 false), !dbg !57
      ret void, !dbg !58
    }

    define void @struct___ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !58
      store ptr %0, ptr %self, align [filtered], !dbg !58
      %deref = load ptr, ptr %self, align [filtered], !dbg !58
      %inner = getelementptr inbounds nuw %struct_, ptr %deref, i32 0, i32 0, !dbg !58
      call void @inner__ctor(ptr %inner), !dbg !58
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !58
      %inner_arr = getelementptr inbounds nuw %struct_, ptr %deref1, i32 0, i32 1, !dbg !58
      call void @__struct__inner_arr__ctor(ptr %inner_arr), !dbg !58
      %deref2 = load ptr, ptr %self, align [filtered], !dbg !58
      %s = getelementptr inbounds nuw %struct_, ptr %deref2, i32 0, i32 2, !dbg !58
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %s, ptr align [filtered] @utf08_literal_0, i32 6, i1 false), !dbg !58
      %deref3 = load ptr, ptr %self, align [filtered], !dbg !58
      %b = getelementptr inbounds nuw %struct_, ptr %deref3, i32 0, i32 3, !dbg !58
      store i8 1, ptr %b, align [filtered], !dbg !58
      %deref4 = load ptr, ptr %self, align [filtered], !dbg !58
      %r = getelementptr inbounds nuw %struct_, ptr %deref4, i32 0, i32 4, !dbg !58
      store float 0x400921CAC0000000, ptr %r, align [filtered], !dbg !58
      %deref5 = load ptr, ptr %self, align [filtered], !dbg !58
      %arr = getelementptr inbounds nuw %struct_, ptr %deref5, i32 0, i32 5, !dbg !58
      call void @__struct__arr__ctor(ptr %arr), !dbg !58
      %deref6 = load ptr, ptr %self, align [filtered], !dbg !58
      %arr7 = getelementptr inbounds nuw %struct_, ptr %deref6, i32 0, i32 5, !dbg !58
      store [3 x [81 x i8]] [[81 x i8] c"aa\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"bb\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"cc\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"], ptr %arr7, align [filtered], !dbg !58
      %deref8 = load ptr, ptr %self, align [filtered], !dbg !58
      %i = getelementptr inbounds nuw %struct_, ptr %deref8, i32 0, i32 6, !dbg !58
      store i16 42, ptr %i, align [filtered], !dbg !58
      ret void, !dbg !58
    }

    define void @inner__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !58
      store ptr %0, ptr %self, align [filtered], !dbg !58
      %deref = load ptr, ptr %self, align [filtered], !dbg !58
      %s = getelementptr inbounds nuw %inner, ptr %deref, i32 0, i32 0, !dbg !58
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %s, ptr align [filtered] @utf08_literal_0, i32 6, i1 false), !dbg !58
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !58
      %b = getelementptr inbounds nuw %inner, ptr %deref1, i32 0, i32 1, !dbg !58
      store i8 1, ptr %b, align [filtered], !dbg !58
      %deref2 = load ptr, ptr %self, align [filtered], !dbg !58
      %r = getelementptr inbounds nuw %inner, ptr %deref2, i32 0, i32 2, !dbg !58
      store float 0x400921CAC0000000, ptr %r, align [filtered], !dbg !58
      %deref3 = load ptr, ptr %self, align [filtered], !dbg !58
      %arr = getelementptr inbounds nuw %inner, ptr %deref3, i32 0, i32 3, !dbg !58
      call void @__inner_arr__ctor(ptr %arr), !dbg !58
      %deref4 = load ptr, ptr %self, align [filtered], !dbg !58
      %arr5 = getelementptr inbounds nuw %inner, ptr %deref4, i32 0, i32 3, !dbg !58
      store [3 x [81 x i8]] [[81 x i8] c"aaaa\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"bbbb\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"cccc\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"], ptr %arr5, align [filtered], !dbg !58
      %deref6 = load ptr, ptr %self, align [filtered], !dbg !58
      %i = getelementptr inbounds nuw %inner, ptr %deref6, i32 0, i32 4, !dbg !58
      store i16 42, ptr %i, align [filtered], !dbg !58
      ret void, !dbg !58
    }

    define void @__main_arr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !58
      store ptr %0, ptr %self, align [filtered], !dbg !58
      ret void, !dbg !58
    }

    define void @__struct__inner_arr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !58
      store ptr %0, ptr %self, align [filtered], !dbg !58
      ret void, !dbg !58
    }

    define void @__struct__arr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !58
      store ptr %0, ptr %self, align [filtered], !dbg !58
      ret void, !dbg !58
    }

    define void @__inner_arr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !58
      store ptr %0, ptr %self, align [filtered], !dbg !58
      ret void, !dbg !58
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #1

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #1

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }

    !llvm.module.flags = !{!0, !1}
    !llvm.dbg.cu = !{!2}

    !0 = !{i32 2, !"Dwarf Version", i32 5}
    !1 = !{i32 2, !"Debug Info Version", i32 3}
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "<internal>", directory: "")
    !4 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !3, file: !3, line: 22, type: !5, scopeLine: 1, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !7)
    !5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
    !6 = !{null}
    !7 = !{}
    !8 = !DILocalVariable(name: "st", scope: !4, file: !3, line: 24, type: !9, align [filtered])
    !9 = !DICompositeType(tag: DW_TAG_structure_type, name: "struct_", scope: !3, file: !3, line: 2, size: 13440, align [filtered], flags: DIFlagPublic, elements: !10, identifier: "struct_")
    !10 = !{!11, !30, !32, !33, !34, !35, !36}
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "inner", scope: !3, file: !3, line: 3, baseType: !12, size: 2688, align [filtered], flags: DIFlagPublic)
    !12 = !DICompositeType(tag: DW_TAG_structure_type, name: "inner", scope: !3, file: !3, line: 13, size: 2688, align [filtered], flags: DIFlagPublic, elements: !13, identifier: "inner")
    !13 = !{!14, !20, !22, !24, !28}
    !14 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !3, file: !3, line: 14, baseType: !15, size: 648, align [filtered], flags: DIFlagPublic)
    !15 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !3, file: !3, baseType: !16, align [filtered])
    !16 = !DICompositeType(tag: DW_TAG_array_type, baseType: !17, size: 648, align [filtered], elements: !18)
    !17 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !18 = !{!19}
    !19 = !DISubrange(count: 81, lowerBound: 0)
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !3, file: !3, line: 15, baseType: !21, size: 8, align [filtered], offset: 648, flags: DIFlagPublic)
    !21 = !DIBasicType(name: "BOOL", size: 8, encoding: DW_ATE_boolean, flags: DIFlagPublic)
    !22 = !DIDerivedType(tag: DW_TAG_member, name: "r", scope: !3, file: !3, line: 16, baseType: !23, size: 32, align [filtered], offset: 672, flags: DIFlagPublic)
    !23 = !DIBasicType(name: "REAL", size: 32, encoding: DW_ATE_float, flags: DIFlagPublic)
    !24 = !DIDerivedType(tag: DW_TAG_member, name: "arr", scope: !3, file: !3, line: 17, baseType: !25, size: 1944, align [filtered], offset: 704, flags: DIFlagPublic)
    !25 = !DICompositeType(tag: DW_TAG_array_type, baseType: !15, size: 1944, align [filtered], elements: !26)
    !26 = !{!27}
    !27 = !DISubrange(count: 3, lowerBound: 0)
    !28 = !DIDerivedType(tag: DW_TAG_member, name: "i", scope: !3, file: !3, line: 18, baseType: !29, size: 16, align [filtered], offset: 2656, flags: DIFlagPublic)
    !29 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !30 = !DIDerivedType(tag: DW_TAG_member, name: "inner_arr", scope: !3, file: !3, line: 4, baseType: !31, size: 8064, align [filtered], offset: 2688, flags: DIFlagPublic)
    !31 = !DICompositeType(tag: DW_TAG_array_type, baseType: !12, size: 8064, align [filtered], elements: !26)
    !32 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !3, file: !3, line: 5, baseType: !15, size: 648, align [filtered], offset: 10752, flags: DIFlagPublic)
    !33 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !3, file: !3, line: 6, baseType: !21, size: 8, align [filtered], offset: 11400, flags: DIFlagPublic)
    !34 = !DIDerivedType(tag: DW_TAG_member, name: "r", scope: !3, file: !3, line: 7, baseType: !23, size: 32, align [filtered], offset: 11424, flags: DIFlagPublic)
    !35 = !DIDerivedType(tag: DW_TAG_member, name: "arr", scope: !3, file: !3, line: 8, baseType: !25, size: 1944, align [filtered], offset: 11456, flags: DIFlagPublic)
    !36 = !DIDerivedType(tag: DW_TAG_member, name: "i", scope: !3, file: !3, line: 9, baseType: !29, size: 16, align [filtered], offset: 13408, flags: DIFlagPublic)
    !37 = !DILocation(line: 24, column: 4, scope: !4)
    !38 = !DILocalVariable(name: "s", scope: !4, file: !3, line: 25, type: !15, align [filtered])
    !39 = !DILocation(line: 25, column: 4, scope: !4)
    !40 = !DILocalVariable(name: "b", scope: !4, file: !3, line: 26, type: !21, align [filtered])
    !41 = !DILocation(line: 26, column: 4, scope: !4)
    !42 = !DILocalVariable(name: "arr", scope: !4, file: !3, line: 27, type: !25, align [filtered])
    !43 = !DILocation(line: 27, column: 4, scope: !4)
    !44 = !DILocalVariable(name: "i", scope: !4, file: !3, line: 28, type: !29, align [filtered])
    !45 = !DILocation(line: 28, column: 4, scope: !4)
    !46 = !DILocation(line: 0, scope: !4)
    !47 = !DILocation(line: 32, column: 4, scope: !4)
    !48 = !DILocation(line: 33, column: 4, scope: !4)
    !49 = !DILocation(line: 34, column: 4, scope: !4)
    !50 = !DILocation(line: 35, column: 4, scope: !4)
    !51 = !DILocation(line: 36, column: 4, scope: !4)
    !52 = !DILocation(line: 37, column: 4, scope: !4)
    !53 = !DILocation(line: 38, column: 4, scope: !4)
    !54 = !DILocation(line: 39, column: 4, scope: !4)
    !55 = !DILocation(line: 41, column: 4, scope: !4)
    !56 = !DILocation(line: 42, column: 4, scope: !4)
    !57 = !DILocation(line: 43, column: 4, scope: !4)
    !58 = !DILocation(line: 45, scope: !4)
    "#);
}

#[test]
fn constants_are_tagged_as_such() {
    let result = codegen(
        "
        VAR_GLOBAL CONSTANT
            x: DINT;
            s: STRING;
            f: foo;
        END_VAR

        PROGRAM prog
        VAR CONSTANT
            a, b, c: DINT;
        END_VAR
        END_PROGRAM

        TYPE foo : STRUCT
            z: DINT;
        END_STRUCT
        END_TYPE

        FUNCTION bar : DINT
        VAR CONSTANT
            d: DINT := 42;
        END_VAR
        END_FUNCTION
    ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { i32 }
    %prog = type { i32, i32, i32 }

    @x = unnamed_addr constant i32 0, !dbg !0
    @s = unnamed_addr constant [81 x i8] zeroinitializer, !dbg !5
    @f = unnamed_addr constant %foo zeroinitializer, !dbg !13
    @prog_instance = global %prog zeroinitializer, !dbg !19
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @prog(ptr %0) !dbg !30 {
    entry:
        #dbg_declare(ptr %0, !34, !DIExpression(), !35)
      %a = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 1
      %c = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 2
      ret void, !dbg !35
    }

    define i32 @bar() !dbg !36 {
    entry:
      %bar = alloca i32, align [filtered]
      %d = alloca i32, align [filtered]
        #dbg_declare(ptr %d, !39, !DIExpression(), !40)
      store i32 42, ptr %d, align [filtered]
        #dbg_declare(ptr %bar, !41, !DIExpression(), !42)
      store i32 0, ptr %bar, align [filtered]
      %bar_ret = load i32, ptr %bar, align [filtered], !dbg !43
      ret i32 %bar_ret, !dbg !43
    }

    define void @prog__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !43
      store ptr %0, ptr %self, align [filtered], !dbg !43
      ret void, !dbg !43
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !43
      store ptr %0, ptr %self, align [filtered], !dbg !43
      ret void, !dbg !43
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @prog__ctor(ptr @prog_instance), !dbg !43
      ret void, !dbg !43
    }

    !llvm.module.flags = !{!26, !27}
    !llvm.dbg.cu = !{!28}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "x", scope: !2, file: !2, line: 3, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !5 = !DIGlobalVariableExpression(var: !6, expr: !DIExpression())
    !6 = distinct !DIGlobalVariable(name: "s", scope: !2, file: !2, line: 4, type: !7, isLocal: false, isDefinition: true)
    !7 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !8)
    !8 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !2, file: !2, baseType: !9, align [filtered])
    !9 = !DICompositeType(tag: DW_TAG_array_type, baseType: !10, size: 648, align [filtered], elements: !11)
    !10 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !11 = !{!12}
    !12 = !DISubrange(count: 81, lowerBound: 0)
    !13 = !DIGlobalVariableExpression(var: !14, expr: !DIExpression())
    !14 = distinct !DIGlobalVariable(name: "f", scope: !2, file: !2, line: 5, type: !15, isLocal: false, isDefinition: true)
    !15 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !16)
    !16 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 14, size: 32, align [filtered], flags: DIFlagPublic, elements: !17, identifier: "foo")
    !17 = !{!18}
    !18 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 15, baseType: !4, size: 32, align [filtered], flags: DIFlagPublic)
    !19 = !DIGlobalVariableExpression(var: !20, expr: !DIExpression())
    !20 = distinct !DIGlobalVariable(name: "prog", scope: !2, file: !2, line: 8, type: !21, isLocal: false, isDefinition: true)
    !21 = !DICompositeType(tag: DW_TAG_structure_type, name: "prog", scope: !2, file: !2, line: 8, size: 96, align [filtered], flags: DIFlagPublic, elements: !22, identifier: "prog")
    !22 = !{!23, !24, !25}
    !23 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 10, baseType: !3, size: 32, align [filtered], flags: DIFlagPublic)
    !24 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 10, baseType: !3, size: 32, align [filtered], offset: 32, flags: DIFlagPublic)
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 10, baseType: !3, size: 32, align [filtered], offset: 64, flags: DIFlagPublic)
    !26 = !{i32 2, !"Dwarf Version", i32 5}
    !27 = !{i32 2, !"Debug Info Version", i32 3}
    !28 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !29, splitDebugInlining: false)
    !29 = !{!0, !5, !13, !19}
    !30 = distinct !DISubprogram(name: "prog", linkageName: "prog", scope: !2, file: !2, line: 8, type: !31, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !28, retainedNodes: !33)
    !31 = !DISubroutineType(flags: DIFlagPublic, types: !32)
    !32 = !{null, !21}
    !33 = !{}
    !34 = !DILocalVariable(name: "prog", scope: !30, file: !2, line: 12, type: !21)
    !35 = !DILocation(line: 12, column: 8, scope: !30)
    !36 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 19, type: !37, scopeLine: 23, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !28, retainedNodes: !33)
    !37 = !DISubroutineType(flags: DIFlagPublic, types: !38)
    !38 = !{null}
    !39 = !DILocalVariable(name: "d", scope: !36, file: !2, line: 21, type: !3, align [filtered])
    !40 = !DILocation(line: 21, column: 12, scope: !36)
    !41 = !DILocalVariable(name: "bar", scope: !36, file: !2, line: 19, type: !4, align [filtered])
    !42 = !DILocation(line: 19, column: 17, scope: !36)
    !43 = !DILocation(line: 23, column: 8, scope: !36)
    "#);
}

#[test]
fn test_debug_info_regular_pointer_types() {
    let codegen = codegen(
        r#"
    VAR_GLOBAL
        basic_ptr : POINTER TO DINT;
        array_ptr : POINTER TO ARRAY[0..10] OF DINT;
        struct_ptr : REF_TO myStruct;
        string_ptr : REF_TO STRING;
    END_VAR

    TYPE myStruct : STRUCT
        x : DINT;
        y : BOOL;
    END_STRUCT END_TYPE
    "#,
    );
    filtered_assert_snapshot!(codegen, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @basic_ptr = global ptr null, !dbg !0
    @array_ptr = global ptr null, !dbg !6
    @struct_ptr = global ptr null, !dbg !13
    @string_ptr = global ptr null, !dbg !22
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @myStruct__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__global_basic_ptr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__global_array_ptr___ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__global_array_ptr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__global_struct_ptr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__global_string_ptr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__global_basic_ptr__ctor(ptr @basic_ptr)
      call void @__global_array_ptr__ctor(ptr @array_ptr)
      call void @__global_struct_ptr__ctor(ptr @struct_ptr)
      call void @__global_string_ptr__ctor(ptr @string_ptr)
      ret void
    }

    !llvm.module.flags = !{!31, !32}
    !llvm.dbg.cu = !{!33}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "basic_ptr", scope: !2, file: !2, line: 3, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____global_basic_ptr", scope: !2, file: !2, baseType: !4, align [filtered])
    !4 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_basic_ptr", baseType: !5, size: 64, align [filtered], dwarfAddressSpace: 1)
    !5 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !6 = !DIGlobalVariableExpression(var: !7, expr: !DIExpression())
    !7 = distinct !DIGlobalVariable(name: "array_ptr", scope: !2, file: !2, line: 4, type: !8, isLocal: false, isDefinition: true)
    !8 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____global_array_ptr", scope: !2, file: !2, baseType: !9, align [filtered])
    !9 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_array_ptr", baseType: !10, size: 64, align [filtered], dwarfAddressSpace: 1)
    !10 = !DICompositeType(tag: DW_TAG_array_type, baseType: !5, size: 352, align [filtered], elements: !11)
    !11 = !{!12}
    !12 = !DISubrange(count: 11, lowerBound: 0)
    !13 = !DIGlobalVariableExpression(var: !14, expr: !DIExpression())
    !14 = distinct !DIGlobalVariable(name: "struct_ptr", scope: !2, file: !2, line: 5, type: !15, isLocal: false, isDefinition: true)
    !15 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REF_TO____global_struct_ptr", scope: !2, file: !2, baseType: !16, align [filtered])
    !16 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_struct_ptr", baseType: !17, size: 64, align [filtered], dwarfAddressSpace: 1)
    !17 = !DICompositeType(tag: DW_TAG_structure_type, name: "myStruct", scope: !2, file: !2, line: 9, size: 64, align [filtered], flags: DIFlagPublic, elements: !18, identifier: "myStruct")
    !18 = !{!19, !20}
    !19 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 10, baseType: !5, size: 32, align [filtered], flags: DIFlagPublic)
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 11, baseType: !21, size: 8, align [filtered], offset: 32, flags: DIFlagPublic)
    !21 = !DIBasicType(name: "BOOL", size: 8, encoding: DW_ATE_boolean, flags: DIFlagPublic)
    !22 = !DIGlobalVariableExpression(var: !23, expr: !DIExpression())
    !23 = distinct !DIGlobalVariable(name: "string_ptr", scope: !2, file: !2, line: 6, type: !24, isLocal: false, isDefinition: true)
    !24 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REF_TO____global_string_ptr", scope: !2, file: !2, baseType: !25, align [filtered])
    !25 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_string_ptr", baseType: !26, size: 64, align [filtered], dwarfAddressSpace: 1)
    !26 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !2, file: !2, baseType: !27, align [filtered])
    !27 = !DICompositeType(tag: DW_TAG_array_type, baseType: !28, size: 648, align [filtered], elements: !29)
    !28 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !29 = !{!30}
    !30 = !DISubrange(count: 81, lowerBound: 0)
    !31 = !{i32 2, !"Dwarf Version", i32 5}
    !32 = !{i32 2, !"Debug Info Version", i32 3}
    !33 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !34, splitDebugInlining: false)
    !34 = !{!0, !6, !13, !22}
    "#)
}

#[test]
fn test_debug_info_auto_deref_parameters() {
    let codegen = codegen(
        r#"
    PROGRAM test_with_ref_params
    VAR_INPUT {ref}
        input_ref : STRING;
        array_ref : ARRAY[0..5] OF DINT;
    END_VAR
    VAR_IN_OUT
        inout_value : DINT;
        inout_struct : myStruct;
    END_VAR
    VAR
        local_ref : REF_TO DINT;
    END_VAR
    END_PROGRAM

    TYPE myStruct : STRUCT
        x : DINT;
        y : BOOL;
    END_STRUCT END_TYPE
    "#,
    );
    filtered_assert_snapshot!(codegen, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %test_with_ref_params = type { ptr, ptr, ptr, ptr, ptr }

    @test_with_ref_params_instance = global %test_with_ref_params zeroinitializer, !dbg !0
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @test_with_ref_params(ptr %0) !dbg !38 {
    entry:
        #dbg_declare(ptr %0, !42, !DIExpression(), !43)
      %input_ref = getelementptr inbounds nuw %test_with_ref_params, ptr %0, i32 0, i32 0
      %array_ref = getelementptr inbounds nuw %test_with_ref_params, ptr %0, i32 0, i32 1
      %inout_value = getelementptr inbounds nuw %test_with_ref_params, ptr %0, i32 0, i32 2
      %inout_struct = getelementptr inbounds nuw %test_with_ref_params, ptr %0, i32 0, i32 3
      %local_ref = getelementptr inbounds nuw %test_with_ref_params, ptr %0, i32 0, i32 4
      ret void, !dbg !43
    }

    define void @test_with_ref_params__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !43
      store ptr %0, ptr %self, align [filtered], !dbg !43
      %deref = load ptr, ptr %self, align [filtered], !dbg !43
      %array_ref = getelementptr inbounds nuw %test_with_ref_params, ptr %deref, i32 0, i32 1, !dbg !43
      %deref1 = load ptr, ptr %array_ref, align [filtered], !dbg !43
      call void @__test_with_ref_params_array_ref__ctor(ptr %deref1), !dbg !43
      %deref2 = load ptr, ptr %self, align [filtered], !dbg !43
      %local_ref = getelementptr inbounds nuw %test_with_ref_params, ptr %deref2, i32 0, i32 4, !dbg !43
      call void @__test_with_ref_params_local_ref__ctor(ptr %local_ref), !dbg !43
      ret void, !dbg !43
    }

    define void @myStruct__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !43
      store ptr %0, ptr %self, align [filtered], !dbg !43
      ret void, !dbg !43
    }

    define void @__test_with_ref_params_array_ref__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !43
      store ptr %0, ptr %self, align [filtered], !dbg !43
      ret void, !dbg !43
    }

    define void @__test_with_ref_params_local_ref__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !43
      store ptr %0, ptr %self, align [filtered], !dbg !43
      ret void, !dbg !43
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @test_with_ref_params__ctor(ptr @test_with_ref_params_instance), !dbg !43
      ret void, !dbg !43
    }

    !llvm.module.flags = !{!34, !35}
    !llvm.dbg.cu = !{!36}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "test_with_ref_params", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "test_with_ref_params", scope: !2, file: !2, line: 2, size: 320, align [filtered], flags: DIFlagPublic, elements: !4, identifier: "test_with_ref_params")
    !4 = !{!5, !13, !20, !23, !31}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "input_ref", scope: !2, file: !2, line: 4, baseType: !6, size: 64, align [filtered], flags: DIFlagPublic)
    !6 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____auto_pointer_to_STRING", scope: !2, file: !2, baseType: !7, align [filtered])
    !7 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_STRING", baseType: !8, size: 64, align [filtered], dwarfAddressSpace: 1)
    !8 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !2, file: !2, baseType: !9, align [filtered])
    !9 = !DICompositeType(tag: DW_TAG_array_type, baseType: !10, size: 648, align [filtered], elements: !11)
    !10 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !11 = !{!12}
    !12 = !DISubrange(count: 81, lowerBound: 0)
    !13 = !DIDerivedType(tag: DW_TAG_member, name: "array_ref", scope: !2, file: !2, line: 5, baseType: !14, size: 64, align [filtered], offset: 64, flags: DIFlagPublic)
    !14 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____auto_pointer_to___test_with_ref_params_array_ref", scope: !2, file: !2, baseType: !15, align [filtered])
    !15 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to___test_with_ref_params_array_ref", baseType: !16, size: 64, align [filtered], dwarfAddressSpace: 1)
    !16 = !DICompositeType(tag: DW_TAG_array_type, baseType: !17, size: 192, align [filtered], elements: !18)
    !17 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !18 = !{!19}
    !19 = !DISubrange(count: 6, lowerBound: 0)
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "inout_value", scope: !2, file: !2, line: 8, baseType: !21, size: 64, align [filtered], offset: 128, flags: DIFlagPublic)
    !21 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____auto_pointer_to_DINT", scope: !2, file: !2, baseType: !22, align [filtered])
    !22 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_DINT", baseType: !17, size: 64, align [filtered], dwarfAddressSpace: 1)
    !23 = !DIDerivedType(tag: DW_TAG_member, name: "inout_struct", scope: !2, file: !2, line: 9, baseType: !24, size: 64, align [filtered], offset: 192, flags: DIFlagPublic)
    !24 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____auto_pointer_to_myStruct", scope: !2, file: !2, baseType: !25, align [filtered])
    !25 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_myStruct", baseType: !26, size: 64, align [filtered], dwarfAddressSpace: 1)
    !26 = !DICompositeType(tag: DW_TAG_structure_type, name: "myStruct", scope: !2, file: !2, line: 16, size: 64, align [filtered], flags: DIFlagPublic, elements: !27, identifier: "myStruct")
    !27 = !{!28, !29}
    !28 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 17, baseType: !17, size: 32, align [filtered], flags: DIFlagPublic)
    !29 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 18, baseType: !30, size: 8, align [filtered], offset: 32, flags: DIFlagPublic)
    !30 = !DIBasicType(name: "BOOL", size: 8, encoding: DW_ATE_boolean, flags: DIFlagPublic)
    !31 = !DIDerivedType(tag: DW_TAG_member, name: "local_ref", scope: !2, file: !2, line: 12, baseType: !32, size: 64, align [filtered], offset: 256, flags: DIFlagPublic)
    !32 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REF_TO____test_with_ref_params_local_ref", scope: !2, file: !2, baseType: !33, align [filtered])
    !33 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__test_with_ref_params_local_ref", baseType: !17, size: 64, align [filtered], dwarfAddressSpace: 1)
    !34 = !{i32 2, !"Dwarf Version", i32 5}
    !35 = !{i32 2, !"Debug Info Version", i32 3}
    !36 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !37, splitDebugInlining: false)
    !37 = !{!0}
    !38 = distinct !DISubprogram(name: "test_with_ref_params", linkageName: "test_with_ref_params", scope: !2, file: !2, line: 2, type: !39, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !36, retainedNodes: !41)
    !39 = !DISubroutineType(flags: DIFlagPublic, types: !40)
    !40 = !{null, !3, !6, !14, !21, !24}
    !41 = !{}
    !42 = !DILocalVariable(name: "test_with_ref_params", scope: !38, file: !2, line: 14, type: !3)
    !43 = !DILocation(line: 14, column: 4, scope: !38)
    "#)
}

#[test]
fn test_debug_info_auto_deref_alias_pointers() {
    let codegen = codegen(
        r#"
    VAR_GLOBAL
        global_var : DINT := 42;
        alias_int AT global_var : DINT;

        global_struct : myStruct;
        alias_struct AT global_struct : myStruct;
    END_VAR

    TYPE myStruct : STRUCT
        x : DINT;
        y : BOOL;
    END_STRUCT END_TYPE
    "#,
    );
    filtered_assert_snapshot!(codegen, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %myStruct = type { i32, i8 }

    @global_var = global i32 42, !dbg !0
    @alias_int = global ptr null, !dbg !4
    @global_struct = global %myStruct zeroinitializer, !dbg !8
    @alias_struct = global ptr null, !dbg !15
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @myStruct__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__global_alias_int__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__global_alias_struct__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      store i32 42, ptr @global_var, align [filtered]
      %deref = load ptr, ptr @alias_int, align [filtered]
      call void @__global_alias_int__ctor(ptr %deref)
      store ptr @global_var, ptr @alias_int, align [filtered]
      call void @myStruct__ctor(ptr @global_struct)
      %deref1 = load ptr, ptr @alias_struct, align [filtered]
      call void @__global_alias_struct__ctor(ptr %deref1)
      store ptr @global_struct, ptr @alias_struct, align [filtered]
      ret void
    }

    !llvm.module.flags = !{!19, !20}
    !llvm.dbg.cu = !{!21}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "global_var", scope: !2, file: !2, line: 3, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !4 = !DIGlobalVariableExpression(var: !5, expr: !DIExpression())
    !5 = distinct !DIGlobalVariable(name: "alias_int", scope: !2, file: !2, line: 4, type: !6, isLocal: false, isDefinition: true)
    !6 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____global_alias_int", scope: !2, file: !2, baseType: !7, align [filtered])
    !7 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_alias_int", baseType: !3, size: 64, align [filtered], dwarfAddressSpace: 1)
    !8 = !DIGlobalVariableExpression(var: !9, expr: !DIExpression())
    !9 = distinct !DIGlobalVariable(name: "global_struct", scope: !2, file: !2, line: 6, type: !10, isLocal: false, isDefinition: true)
    !10 = !DICompositeType(tag: DW_TAG_structure_type, name: "myStruct", scope: !2, file: !2, line: 10, size: 64, align [filtered], flags: DIFlagPublic, elements: !11, identifier: "myStruct")
    !11 = !{!12, !13}
    !12 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !3, size: 32, align [filtered], flags: DIFlagPublic)
    !13 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 12, baseType: !14, size: 8, align [filtered], offset: 32, flags: DIFlagPublic)
    !14 = !DIBasicType(name: "BOOL", size: 8, encoding: DW_ATE_boolean, flags: DIFlagPublic)
    !15 = !DIGlobalVariableExpression(var: !16, expr: !DIExpression())
    !16 = distinct !DIGlobalVariable(name: "alias_struct", scope: !2, file: !2, line: 7, type: !17, isLocal: false, isDefinition: true)
    !17 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____global_alias_struct", scope: !2, file: !2, baseType: !18, align [filtered])
    !18 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_alias_struct", baseType: !10, size: 64, align [filtered], dwarfAddressSpace: 1)
    !19 = !{i32 2, !"Dwarf Version", i32 5}
    !20 = !{i32 2, !"Debug Info Version", i32 3}
    !21 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !22, splitDebugInlining: false)
    !22 = !{!0, !4, !8, !15}
    "#)
}

#[test]
fn test_debug_info_mixed_pointer_types() {
    let codegen = codegen(
        r#"
    VAR_GLOBAL
        regular_ptr : POINTER TO DINT;
        alias_var AT regular_ptr : POINTER TO DINT;
    END_VAR

    PROGRAM mixed_ptr
    VAR_INPUT {ref}
        ref_param : STRING;
    END_VAR
    VAR_IN_OUT
        inout_param : DINT;
    END_VAR
    VAR
        local_ptr : POINTER TO BOOL;
        local_ref : REF_TO BOOL;
    END_VAR
    END_PROGRAM
    "#,
    );
    filtered_assert_snapshot!(codegen, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %mixed_ptr = type { ptr, ptr, ptr, ptr }

    @regular_ptr = global ptr null, !dbg !0
    @alias_var = global ptr null, !dbg !6
    @mixed_ptr_instance = global %mixed_ptr zeroinitializer, !dbg !12
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @mixed_ptr(ptr %0) !dbg !38 {
    entry:
        #dbg_declare(ptr %0, !42, !DIExpression(), !43)
      %ref_param = getelementptr inbounds nuw %mixed_ptr, ptr %0, i32 0, i32 0
      %inout_param = getelementptr inbounds nuw %mixed_ptr, ptr %0, i32 0, i32 1
      %local_ptr = getelementptr inbounds nuw %mixed_ptr, ptr %0, i32 0, i32 2
      %local_ref = getelementptr inbounds nuw %mixed_ptr, ptr %0, i32 0, i32 3
      ret void, !dbg !43
    }

    define void @mixed_ptr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !43
      store ptr %0, ptr %self, align [filtered], !dbg !43
      %deref = load ptr, ptr %self, align [filtered], !dbg !43
      %local_ptr = getelementptr inbounds nuw %mixed_ptr, ptr %deref, i32 0, i32 2, !dbg !43
      call void @__mixed_ptr_local_ptr__ctor(ptr %local_ptr), !dbg !43
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !43
      %local_ref = getelementptr inbounds nuw %mixed_ptr, ptr %deref1, i32 0, i32 3, !dbg !43
      call void @__mixed_ptr_local_ref__ctor(ptr %local_ref), !dbg !43
      ret void, !dbg !43
    }

    define void @__mixed_ptr_local_ptr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !43
      store ptr %0, ptr %self, align [filtered], !dbg !43
      ret void, !dbg !43
    }

    define void @__mixed_ptr_local_ref__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !43
      store ptr %0, ptr %self, align [filtered], !dbg !43
      ret void, !dbg !43
    }

    define void @__global_regular_ptr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !43
      store ptr %0, ptr %self, align [filtered], !dbg !43
      ret void, !dbg !43
    }

    define void @__global_alias_var___ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !43
      store ptr %0, ptr %self, align [filtered], !dbg !43
      ret void, !dbg !43
    }

    define void @__global_alias_var__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !43
      store ptr %0, ptr %self, align [filtered], !dbg !43
      ret void, !dbg !43
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__global_regular_ptr__ctor(ptr @regular_ptr), !dbg !43
      %deref = load ptr, ptr @alias_var, align [filtered], !dbg !43
      call void @__global_alias_var__ctor(ptr %deref), !dbg !43
      store ptr @regular_ptr, ptr @alias_var, align [filtered], !dbg !43
      call void @mixed_ptr__ctor(ptr @mixed_ptr_instance), !dbg !43
      ret void, !dbg !43
    }

    !llvm.module.flags = !{!34, !35}
    !llvm.dbg.cu = !{!36}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "regular_ptr", scope: !2, file: !2, line: 3, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____global_regular_ptr", scope: !2, file: !2, baseType: !4, align [filtered])
    !4 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_regular_ptr", baseType: !5, size: 64, align [filtered], dwarfAddressSpace: 1)
    !5 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !6 = !DIGlobalVariableExpression(var: !7, expr: !DIExpression())
    !7 = distinct !DIGlobalVariable(name: "alias_var", scope: !2, file: !2, line: 4, type: !8, isLocal: false, isDefinition: true)
    !8 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____global_alias_var", scope: !2, file: !2, baseType: !9, align [filtered])
    !9 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_alias_var", baseType: !10, size: 64, align [filtered], dwarfAddressSpace: 1)
    !10 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____global_alias_var_", scope: !2, file: !2, baseType: !11, align [filtered])
    !11 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_alias_var_", baseType: !5, size: 64, align [filtered], dwarfAddressSpace: 1)
    !12 = !DIGlobalVariableExpression(var: !13, expr: !DIExpression())
    !13 = distinct !DIGlobalVariable(name: "mixed_ptr", scope: !2, file: !2, line: 7, type: !14, isLocal: false, isDefinition: true)
    !14 = !DICompositeType(tag: DW_TAG_structure_type, name: "mixed_ptr", scope: !2, file: !2, line: 7, size: 256, align [filtered], flags: DIFlagPublic, elements: !15, identifier: "mixed_ptr")
    !15 = !{!16, !24, !27, !31}
    !16 = !DIDerivedType(tag: DW_TAG_member, name: "ref_param", scope: !2, file: !2, line: 9, baseType: !17, size: 64, align [filtered], flags: DIFlagPublic)
    !17 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____auto_pointer_to_STRING", scope: !2, file: !2, baseType: !18, align [filtered])
    !18 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_STRING", baseType: !19, size: 64, align [filtered], dwarfAddressSpace: 1)
    !19 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !2, file: !2, baseType: !20, align [filtered])
    !20 = !DICompositeType(tag: DW_TAG_array_type, baseType: !21, size: 648, align [filtered], elements: !22)
    !21 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !22 = !{!23}
    !23 = !DISubrange(count: 81, lowerBound: 0)
    !24 = !DIDerivedType(tag: DW_TAG_member, name: "inout_param", scope: !2, file: !2, line: 12, baseType: !25, size: 64, align [filtered], offset: 64, flags: DIFlagPublic)
    !25 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____auto_pointer_to_DINT", scope: !2, file: !2, baseType: !26, align [filtered])
    !26 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_DINT", baseType: !5, size: 64, align [filtered], dwarfAddressSpace: 1)
    !27 = !DIDerivedType(tag: DW_TAG_member, name: "local_ptr", scope: !2, file: !2, line: 15, baseType: !28, size: 64, align [filtered], offset: 128, flags: DIFlagPublic)
    !28 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____mixed_ptr_local_ptr", scope: !2, file: !2, baseType: !29, align [filtered])
    !29 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__mixed_ptr_local_ptr", baseType: !30, size: 64, align [filtered], dwarfAddressSpace: 1)
    !30 = !DIBasicType(name: "BOOL", size: 8, encoding: DW_ATE_boolean, flags: DIFlagPublic)
    !31 = !DIDerivedType(tag: DW_TAG_member, name: "local_ref", scope: !2, file: !2, line: 16, baseType: !32, size: 64, align [filtered], offset: 192, flags: DIFlagPublic)
    !32 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REF_TO____mixed_ptr_local_ref", scope: !2, file: !2, baseType: !33, align [filtered])
    !33 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__mixed_ptr_local_ref", baseType: !30, size: 64, align [filtered], dwarfAddressSpace: 1)
    !34 = !{i32 2, !"Dwarf Version", i32 5}
    !35 = !{i32 2, !"Debug Info Version", i32 3}
    !36 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !37, splitDebugInlining: false)
    !37 = !{!0, !6, !12}
    !38 = distinct !DISubprogram(name: "mixed_ptr", linkageName: "mixed_ptr", scope: !2, file: !2, line: 7, type: !39, scopeLine: 18, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !36, retainedNodes: !41)
    !39 = !DISubroutineType(flags: DIFlagPublic, types: !40)
    !40 = !{null, !14, !17, !25}
    !41 = !{}
    !42 = !DILocalVariable(name: "mixed_ptr", scope: !38, file: !2, line: 18, type: !14)
    !43 = !DILocation(line: 18, column: 4, scope: !38)
    "#)
}

#[test]
fn test_debug_info_auto_deref_reference_to_pointers() {
    let codegen = codegen(
        r#"
    VAR_GLOBAL
        basic_reference : REFERENCE TO DINT;
        array_reference : REFERENCE TO ARRAY[0..10] OF DINT;
        struct_reference : REFERENCE TO myStruct;
        string_reference : REFERENCE TO STRING;
    END_VAR

    PROGRAM test_with_reference_params
    VAR_INPUT
        ref_param : REFERENCE TO DINT;
        array_ref_param : REFERENCE TO ARRAY[0..5] OF BOOL;
    END_VAR
    VAR
        local_reference : REFERENCE TO myStruct;
    END_VAR
    END_PROGRAM

    TYPE myStruct : STRUCT
        x : DINT;
        y : BOOL;
    END_STRUCT END_TYPE
    "#,
    );
    filtered_assert_snapshot!(codegen, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %test_with_reference_params = type { ptr, ptr, ptr }

    @basic_reference = global ptr null, !dbg !0
    @array_reference = global ptr null, !dbg !6
    @struct_reference = global ptr null, !dbg !13
    @string_reference = global ptr null, !dbg !22
    @test_with_reference_params_instance = global %test_with_reference_params zeroinitializer, !dbg !31
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @test_with_reference_params(ptr %0) !dbg !51 {
    entry:
        #dbg_declare(ptr %0, !55, !DIExpression(), !56)
      %ref_param = getelementptr inbounds nuw %test_with_reference_params, ptr %0, i32 0, i32 0
      %array_ref_param = getelementptr inbounds nuw %test_with_reference_params, ptr %0, i32 0, i32 1
      %local_reference = getelementptr inbounds nuw %test_with_reference_params, ptr %0, i32 0, i32 2
      ret void, !dbg !56
    }

    define void @test_with_reference_params__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !56
      store ptr %0, ptr %self, align [filtered], !dbg !56
      %deref = load ptr, ptr %self, align [filtered], !dbg !56
      %ref_param = getelementptr inbounds nuw %test_with_reference_params, ptr %deref, i32 0, i32 0, !dbg !56
      %deref1 = load ptr, ptr %ref_param, align [filtered], !dbg !56
      call void @__test_with_reference_params_ref_param__ctor(ptr %deref1), !dbg !56
      %deref2 = load ptr, ptr %self, align [filtered], !dbg !56
      %array_ref_param = getelementptr inbounds nuw %test_with_reference_params, ptr %deref2, i32 0, i32 1, !dbg !56
      %deref3 = load ptr, ptr %array_ref_param, align [filtered], !dbg !56
      call void @__test_with_reference_params_array_ref_param__ctor(ptr %deref3), !dbg !56
      %deref4 = load ptr, ptr %self, align [filtered], !dbg !56
      %local_reference = getelementptr inbounds nuw %test_with_reference_params, ptr %deref4, i32 0, i32 2, !dbg !56
      %deref5 = load ptr, ptr %local_reference, align [filtered], !dbg !56
      call void @__test_with_reference_params_local_reference__ctor(ptr %deref5), !dbg !56
      ret void, !dbg !56
    }

    define void @myStruct__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !56
      store ptr %0, ptr %self, align [filtered], !dbg !56
      ret void, !dbg !56
    }

    define void @__test_with_reference_params_ref_param__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !56
      store ptr %0, ptr %self, align [filtered], !dbg !56
      ret void, !dbg !56
    }

    define void @__test_with_reference_params_array_ref_param___ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !56
      store ptr %0, ptr %self, align [filtered], !dbg !56
      ret void, !dbg !56
    }

    define void @__test_with_reference_params_array_ref_param__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !56
      store ptr %0, ptr %self, align [filtered], !dbg !56
      ret void, !dbg !56
    }

    define void @__test_with_reference_params_local_reference__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !56
      store ptr %0, ptr %self, align [filtered], !dbg !56
      ret void, !dbg !56
    }

    define void @__global_basic_reference__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !56
      store ptr %0, ptr %self, align [filtered], !dbg !56
      ret void, !dbg !56
    }

    define void @__global_array_reference___ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !56
      store ptr %0, ptr %self, align [filtered], !dbg !56
      ret void, !dbg !56
    }

    define void @__global_array_reference__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !56
      store ptr %0, ptr %self, align [filtered], !dbg !56
      ret void, !dbg !56
    }

    define void @__global_struct_reference__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !56
      store ptr %0, ptr %self, align [filtered], !dbg !56
      ret void, !dbg !56
    }

    define void @__global_string_reference__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !56
      store ptr %0, ptr %self, align [filtered], !dbg !56
      ret void, !dbg !56
    }

    define void @__unit___internal____ctor() {
    entry:
      %deref = load ptr, ptr @basic_reference, align [filtered], !dbg !56
      call void @__global_basic_reference__ctor(ptr %deref), !dbg !56
      %deref1 = load ptr, ptr @array_reference, align [filtered], !dbg !56
      call void @__global_array_reference__ctor(ptr %deref1), !dbg !56
      %deref2 = load ptr, ptr @struct_reference, align [filtered], !dbg !56
      call void @__global_struct_reference__ctor(ptr %deref2), !dbg !56
      %deref3 = load ptr, ptr @string_reference, align [filtered], !dbg !56
      call void @__global_string_reference__ctor(ptr %deref3), !dbg !56
      call void @test_with_reference_params__ctor(ptr @test_with_reference_params_instance), !dbg !56
      ret void, !dbg !56
    }

    !llvm.module.flags = !{!47, !48}
    !llvm.dbg.cu = !{!49}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "basic_reference", scope: !2, file: !2, line: 3, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REFERENCE_TO____global_basic_reference", scope: !2, file: !2, baseType: !4, align [filtered])
    !4 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_basic_reference", baseType: !5, size: 64, align [filtered], dwarfAddressSpace: 1)
    !5 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !6 = !DIGlobalVariableExpression(var: !7, expr: !DIExpression())
    !7 = distinct !DIGlobalVariable(name: "array_reference", scope: !2, file: !2, line: 4, type: !8, isLocal: false, isDefinition: true)
    !8 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REFERENCE_TO____global_array_reference", scope: !2, file: !2, baseType: !9, align [filtered])
    !9 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_array_reference", baseType: !10, size: 64, align [filtered], dwarfAddressSpace: 1)
    !10 = !DICompositeType(tag: DW_TAG_array_type, baseType: !5, size: 352, align [filtered], elements: !11)
    !11 = !{!12}
    !12 = !DISubrange(count: 11, lowerBound: 0)
    !13 = !DIGlobalVariableExpression(var: !14, expr: !DIExpression())
    !14 = distinct !DIGlobalVariable(name: "struct_reference", scope: !2, file: !2, line: 5, type: !15, isLocal: false, isDefinition: true)
    !15 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REFERENCE_TO____global_struct_reference", scope: !2, file: !2, baseType: !16, align [filtered])
    !16 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_struct_reference", baseType: !17, size: 64, align [filtered], dwarfAddressSpace: 1)
    !17 = !DICompositeType(tag: DW_TAG_structure_type, name: "myStruct", scope: !2, file: !2, line: 19, size: 64, align [filtered], flags: DIFlagPublic, elements: !18, identifier: "myStruct")
    !18 = !{!19, !20}
    !19 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 20, baseType: !5, size: 32, align [filtered], flags: DIFlagPublic)
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 21, baseType: !21, size: 8, align [filtered], offset: 32, flags: DIFlagPublic)
    !21 = !DIBasicType(name: "BOOL", size: 8, encoding: DW_ATE_boolean, flags: DIFlagPublic)
    !22 = !DIGlobalVariableExpression(var: !23, expr: !DIExpression())
    !23 = distinct !DIGlobalVariable(name: "string_reference", scope: !2, file: !2, line: 6, type: !24, isLocal: false, isDefinition: true)
    !24 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REFERENCE_TO____global_string_reference", scope: !2, file: !2, baseType: !25, align [filtered])
    !25 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_string_reference", baseType: !26, size: 64, align [filtered], dwarfAddressSpace: 1)
    !26 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !2, file: !2, baseType: !27, align [filtered])
    !27 = !DICompositeType(tag: DW_TAG_array_type, baseType: !28, size: 648, align [filtered], elements: !29)
    !28 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !29 = !{!30}
    !30 = !DISubrange(count: 81, lowerBound: 0)
    !31 = !DIGlobalVariableExpression(var: !32, expr: !DIExpression())
    !32 = distinct !DIGlobalVariable(name: "test_with_reference_params", scope: !2, file: !2, line: 9, type: !33, isLocal: false, isDefinition: true)
    !33 = !DICompositeType(tag: DW_TAG_structure_type, name: "test_with_reference_params", scope: !2, file: !2, line: 9, size: 192, align [filtered], flags: DIFlagPublic, elements: !34, identifier: "test_with_reference_params")
    !34 = !{!35, !38, !44}
    !35 = !DIDerivedType(tag: DW_TAG_member, name: "ref_param", scope: !2, file: !2, line: 11, baseType: !36, size: 64, align [filtered], flags: DIFlagPublic)
    !36 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REFERENCE_TO____test_with_reference_params_ref_param", scope: !2, file: !2, baseType: !37, align [filtered])
    !37 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__test_with_reference_params_ref_param", baseType: !5, size: 64, align [filtered], dwarfAddressSpace: 1)
    !38 = !DIDerivedType(tag: DW_TAG_member, name: "array_ref_param", scope: !2, file: !2, line: 12, baseType: !39, size: 64, align [filtered], offset: 64, flags: DIFlagPublic)
    !39 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REFERENCE_TO____test_with_reference_params_array_ref_param", scope: !2, file: !2, baseType: !40, align [filtered])
    !40 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__test_with_reference_params_array_ref_param", baseType: !41, size: 64, align [filtered], dwarfAddressSpace: 1)
    !41 = !DICompositeType(tag: DW_TAG_array_type, baseType: !21, size: 48, align [filtered], elements: !42)
    !42 = !{!43}
    !43 = !DISubrange(count: 6, lowerBound: 0)
    !44 = !DIDerivedType(tag: DW_TAG_member, name: "local_reference", scope: !2, file: !2, line: 15, baseType: !45, size: 64, align [filtered], offset: 128, flags: DIFlagPublic)
    !45 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REFERENCE_TO____test_with_reference_params_local_reference", scope: !2, file: !2, baseType: !46, align [filtered])
    !46 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__test_with_reference_params_local_reference", baseType: !17, size: 64, align [filtered], dwarfAddressSpace: 1)
    !47 = !{i32 2, !"Dwarf Version", i32 5}
    !48 = !{i32 2, !"Debug Info Version", i32 3}
    !49 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !50, splitDebugInlining: false)
    !50 = !{!0, !6, !13, !22, !31}
    !51 = distinct !DISubprogram(name: "test_with_reference_params", linkageName: "test_with_reference_params", scope: !2, file: !2, line: 9, type: !52, scopeLine: 17, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !49, retainedNodes: !54)
    !52 = !DISubroutineType(flags: DIFlagPublic, types: !53)
    !53 = !{null, !33, !36, !39}
    !54 = !{}
    !55 = !DILocalVariable(name: "test_with_reference_params", scope: !51, file: !2, line: 17, type: !33)
    !56 = !DILocation(line: 17, column: 4, scope: !51)
    "#)
}

#[test]
fn range_datatype_debug() {
    let codegen = codegen(
        r#"
        TYPE RangeType :
            DINT(0..100) := 0;
        END_TYPE

        Function main : DINT
        VAR
            r : RangeType;
        END_VAR
            r := 50;
            main := r;
        END_FUNCTION
    "#,
    );
    filtered_assert_snapshot!(codegen, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define i32 @main() !dbg !4 {
    entry:
      %main = alloca i32, align [filtered]
      %r = alloca i32, align [filtered]
        #dbg_declare(ptr %r, !8, !DIExpression(), !11)
      store i32 0, ptr %r, align [filtered]
        #dbg_declare(ptr %main, !12, !DIExpression(), !13)
      store i32 0, ptr %main, align [filtered]
      call void @RangeType__ctor(ptr %r), !dbg !14
      store i32 50, ptr %r, align [filtered], !dbg !15
      %load_r = load i32, ptr %r, align [filtered], !dbg !16
      store i32 %load_r, ptr %main, align [filtered], !dbg !16
      %main_ret = load i32, ptr %main, align [filtered], !dbg !17
      ret i32 %main_ret, !dbg !17
    }

    define void @RangeType__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !17
      store ptr %0, ptr %self, align [filtered], !dbg !17
      %deref = load ptr, ptr %self, align [filtered], !dbg !17
      store i32 0, ptr %deref, align [filtered], !dbg !17
      ret void, !dbg !17
    }

    !llvm.module.flags = !{!0, !1}
    !llvm.dbg.cu = !{!2}

    !0 = !{i32 2, !"Dwarf Version", i32 5}
    !1 = !{i32 2, !"Debug Info Version", i32 3}
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "<internal>", directory: "")
    !4 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !3, file: !3, line: 6, type: !5, scopeLine: 1, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !7)
    !5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
    !6 = !{null}
    !7 = !{}
    !8 = !DILocalVariable(name: "r", scope: !4, file: !3, line: 8, type: !9, align [filtered])
    !9 = !DIDerivedType(tag: DW_TAG_typedef, name: "__SUBRANGE_0_100__DINT", scope: !3, file: !3, line: 2, baseType: !10, align [filtered])
    !10 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !11 = !DILocation(line: 8, column: 12, scope: !4)
    !12 = !DILocalVariable(name: "main", scope: !4, file: !3, line: 6, type: !10, align [filtered])
    !13 = !DILocation(line: 6, column: 17, scope: !4)
    !14 = !DILocation(line: 0, scope: !4)
    !15 = !DILocation(line: 10, column: 12, scope: !4)
    !16 = !DILocation(line: 11, column: 12, scope: !4)
    !17 = !DILocation(line: 12, column: 8, scope: !4)
    "#)
}

#[test]
fn range_datatype_reference_expr_bounds_debug() {
    let codegen = codegen(
        r#"
        VAR_GLOBAL CONSTANT
            ZERO : DINT := 0;
        END_VAR

        TYPE RangeType :
            DINT(ZERO..(100+3)) := 0;
        END_TYPE

        Function main : DINT
        VAR
            r : RangeType;
        END_VAR
            r := 50;
            main := r;
        END_FUNCTION
    "#,
    );
    filtered_assert_snapshot!(codegen, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @ZERO = unnamed_addr constant i32 0, !dbg !0

    define i32 @main() !dbg !9 {
    entry:
      %main = alloca i32, align [filtered]
      %r = alloca i32, align [filtered]
        #dbg_declare(ptr %r, !13, !DIExpression(), !15)
      store i32 0, ptr %r, align [filtered]
        #dbg_declare(ptr %main, !16, !DIExpression(), !17)
      store i32 0, ptr %main, align [filtered]
      call void @RangeType__ctor(ptr %r), !dbg !18
      store i32 50, ptr %r, align [filtered], !dbg !19
      %load_r = load i32, ptr %r, align [filtered], !dbg !20
      store i32 %load_r, ptr %main, align [filtered], !dbg !20
      %main_ret = load i32, ptr %main, align [filtered], !dbg !21
      ret i32 %main_ret, !dbg !21
    }

    define void @RangeType__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !21
      store ptr %0, ptr %self, align [filtered], !dbg !21
      %deref = load ptr, ptr %self, align [filtered], !dbg !21
      store i32 0, ptr %deref, align [filtered], !dbg !21
      ret void, !dbg !21
    }

    !llvm.module.flags = !{!5, !6}
    !llvm.dbg.cu = !{!7}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "ZERO", scope: !2, file: !2, line: 3, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !5 = !{i32 2, !"Dwarf Version", i32 5}
    !6 = !{i32 2, !"Debug Info Version", i32 3}
    !7 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !8, splitDebugInlining: false)
    !8 = !{!0}
    !9 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 10, type: !10, scopeLine: 1, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !12)
    !10 = !DISubroutineType(flags: DIFlagPublic, types: !11)
    !11 = !{null}
    !12 = !{}
    !13 = !DILocalVariable(name: "r", scope: !9, file: !2, line: 12, type: !14, align [filtered])
    !14 = !DIDerivedType(tag: DW_TAG_typedef, name: "__SUBRANGE_0_103__DINT", scope: !2, file: !2, line: 6, baseType: !4, align [filtered])
    !15 = !DILocation(line: 12, column: 12, scope: !9)
    !16 = !DILocalVariable(name: "main", scope: !9, file: !2, line: 10, type: !4, align [filtered])
    !17 = !DILocation(line: 10, column: 17, scope: !9)
    !18 = !DILocation(line: 0, scope: !9)
    !19 = !DILocation(line: 14, column: 12, scope: !9)
    !20 = !DILocation(line: 15, column: 12, scope: !9)
    !21 = !DILocation(line: 16, column: 8, scope: !9)
    "#);
}

#[test]
fn range_datatype_fqn_reference_bounds_debug() {
    let codegen = codegen(
        r#"
        TYPE RangeType :
            DINT(prog.TEN..(100+3)) := 0;
        END_TYPE

        PROGRAM prog
        VAR CONSTANT
            TEN : DINT := 10;
        END_VAR
        VAR
            r : RangeType;
        END_VAR
        END_PROGRAM
    "#,
    );
    filtered_assert_snapshot!(codegen, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prog = type { i32, i32 }

    @prog_instance = global %prog { i32 10, i32 0 }, !dbg !0
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @prog(ptr %0) !dbg !14 {
    entry:
        #dbg_declare(ptr %0, !18, !DIExpression(), !19)
      %TEN = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 0
      %r = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 1
      ret void, !dbg !19
    }

    define void @prog__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !19
      store ptr %0, ptr %self, align [filtered], !dbg !19
      %deref = load ptr, ptr %self, align [filtered], !dbg !19
      %r = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 1, !dbg !19
      call void @RangeType__ctor(ptr %r), !dbg !19
      ret void, !dbg !19
    }

    define void @RangeType__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !19
      store ptr %0, ptr %self, align [filtered], !dbg !19
      %deref = load ptr, ptr %self, align [filtered], !dbg !19
      store i32 0, ptr %deref, align [filtered], !dbg !19
      ret void, !dbg !19
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @prog__ctor(ptr @prog_instance), !dbg !19
      ret void, !dbg !19
    }

    !llvm.module.flags = !{!10, !11}
    !llvm.dbg.cu = !{!12}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "prog", scope: !2, file: !2, line: 6, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "prog", scope: !2, file: !2, line: 6, size: 64, align [filtered], flags: DIFlagPublic, elements: !4, identifier: "prog")
    !4 = !{!5, !8}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "TEN", scope: !2, file: !2, line: 8, baseType: !6, size: 32, align [filtered], flags: DIFlagPublic)
    !6 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !7)
    !7 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "r", scope: !2, file: !2, line: 11, baseType: !9, size: 32, align [filtered], offset: 32, flags: DIFlagPublic)
    !9 = !DIDerivedType(tag: DW_TAG_typedef, name: "__SUBRANGE_10_103__DINT", scope: !2, file: !2, line: 2, baseType: !7, align [filtered])
    !10 = !{i32 2, !"Dwarf Version", i32 5}
    !11 = !{i32 2, !"Debug Info Version", i32 3}
    !12 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !13, splitDebugInlining: false)
    !13 = !{!0}
    !14 = distinct !DISubprogram(name: "prog", linkageName: "prog", scope: !2, file: !2, line: 6, type: !15, scopeLine: 13, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !12, retainedNodes: !17)
    !15 = !DISubroutineType(flags: DIFlagPublic, types: !16)
    !16 = !{null, !3}
    !17 = !{}
    !18 = !DILocalVariable(name: "prog", scope: !14, file: !2, line: 13, type: !3)
    !19 = !DILocation(line: 13, column: 8, scope: !14)
    "#);
}

#[test]
fn range_datatype_debug_alias_reused() {
    let codegen = codegen(
        r#"
        TYPE RangeType :
            DINT(prog.ZERO..(100+3)) := 0;
        END_TYPE

        PROGRAM prog
        VAR CONSTANT
            ZERO : DINT := 10;
        END_VAR
        VAR
            u : RangeType; // first use of RangeType
            v : RangeType; // second use of RangeType, should reuse debug info
            w : DINT(10..103); // direct use of same range, should reuse debug info
        END_VAR
        END_PROGRAM
    "#,
    );
    filtered_assert_snapshot!(codegen, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prog = type { i32, i32, i32, i32 }

    @prog_instance = global %prog { i32 10, i32 0, i32 0, i32 0 }, !dbg !0
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @prog(ptr %0) !dbg !16 {
    entry:
        #dbg_declare(ptr %0, !20, !DIExpression(), !21)
      %ZERO = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 0
      %u = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 1
      %v = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 2
      %w = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 3
      ret void, !dbg !21
    }

    define void @prog__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !21
      store ptr %0, ptr %self, align [filtered], !dbg !21
      %deref = load ptr, ptr %self, align [filtered], !dbg !21
      %u = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 1, !dbg !21
      call void @RangeType__ctor(ptr %u), !dbg !21
      %deref1 = load ptr, ptr %self, align [filtered], !dbg !21
      %v = getelementptr inbounds nuw %prog, ptr %deref1, i32 0, i32 2, !dbg !21
      call void @RangeType__ctor(ptr %v), !dbg !21
      %deref2 = load ptr, ptr %self, align [filtered], !dbg !21
      %w = getelementptr inbounds nuw %prog, ptr %deref2, i32 0, i32 3, !dbg !21
      call void @__prog_w__ctor(ptr %w), !dbg !21
      ret void, !dbg !21
    }

    define void @RangeType__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !21
      store ptr %0, ptr %self, align [filtered], !dbg !21
      %deref = load ptr, ptr %self, align [filtered], !dbg !21
      store i32 0, ptr %deref, align [filtered], !dbg !21
      ret void, !dbg !21
    }

    define void @__prog_w__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered], !dbg !21
      store ptr %0, ptr %self, align [filtered], !dbg !21
      ret void, !dbg !21
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @prog__ctor(ptr @prog_instance), !dbg !21
      ret void, !dbg !21
    }

    !llvm.module.flags = !{!12, !13}
    !llvm.dbg.cu = !{!14}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "prog", scope: !2, file: !2, line: 6, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "prog", scope: !2, file: !2, line: 6, size: 128, align [filtered], flags: DIFlagPublic, elements: !4, identifier: "prog")
    !4 = !{!5, !8, !10, !11}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "ZERO", scope: !2, file: !2, line: 8, baseType: !6, size: 32, align [filtered], flags: DIFlagPublic)
    !6 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !7)
    !7 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "u", scope: !2, file: !2, line: 11, baseType: !9, size: 32, align [filtered], offset: 32, flags: DIFlagPublic)
    !9 = !DIDerivedType(tag: DW_TAG_typedef, name: "__SUBRANGE_10_103__DINT", scope: !2, file: !2, line: 2, baseType: !7, align [filtered])
    !10 = !DIDerivedType(tag: DW_TAG_member, name: "v", scope: !2, file: !2, line: 12, baseType: !9, size: 32, align [filtered], offset: 64, flags: DIFlagPublic)
    !11 = !DIDerivedType(tag: DW_TAG_member, name: "w", scope: !2, file: !2, line: 13, baseType: !9, size: 32, align [filtered], offset: 96, flags: DIFlagPublic)
    !12 = !{i32 2, !"Dwarf Version", i32 5}
    !13 = !{i32 2, !"Debug Info Version", i32 3}
    !14 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !15, splitDebugInlining: false)
    !15 = !{!0}
    !16 = distinct !DISubprogram(name: "prog", linkageName: "prog", scope: !2, file: !2, line: 6, type: !17, scopeLine: 15, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !14, retainedNodes: !19)
    !17 = !DISubroutineType(flags: DIFlagPublic, types: !18)
    !18 = !{null, !3}
    !19 = !{}
    !20 = !DILocalVariable(name: "prog", scope: !16, file: !2, line: 15, type: !3)
    !21 = !DILocation(line: 15, column: 8, scope: !16)
    "#);
}
