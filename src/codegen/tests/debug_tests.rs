use insta::assert_snapshot;

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

    assert_snapshot!(codegen)
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

    assert_snapshot!(codegen)
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

    assert_snapshot!(codegen)
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

    assert_snapshot!(codegen)
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
    assert_snapshot!(codegen)
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
    assert_snapshot!(codegen)
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
    assert_snapshot!(codegen)
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
    assert_snapshot!(codegen)
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
    assert_snapshot!(codegen)
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

    assert_snapshot!(codegen)
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

    assert_snapshot!(codegen)
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

    assert_snapshot!(codegen, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define i32 @main() !dbg !4 {
    entry:
      %main = alloca i32, align 4
      %x1 = alloca i16, align 2
      %x2 = alloca i16, align 2
      %x3 = alloca i16, align 2
      call void @llvm.dbg.declare(metadata i16* %x1, metadata !8, metadata !DIExpression()), !dbg !10
      store i16 0, i16* %x1, align 2
      call void @llvm.dbg.declare(metadata i16* %x2, metadata !11, metadata !DIExpression()), !dbg !12
      store i16 0, i16* %x2, align 2
      call void @llvm.dbg.declare(metadata i16* %x3, metadata !13, metadata !DIExpression()), !dbg !14
      store i16 0, i16* %x3, align 2
      call void @llvm.dbg.declare(metadata i32* %main, metadata !15, metadata !DIExpression()), !dbg !17
      store i32 0, i32* %main, align 4
      br label %condition_check, !dbg !18

    condition_check:                                  ; preds = %continue2, %entry
      br i1 true, label %while_body, label %continue, !dbg !19

    while_body:                                       ; preds = %condition_check
      br i1 false, label %condition_body, label %continue1, !dbg !19

    continue:                                         ; preds = %condition_body, %condition_check
      %main_ret = load i32, i32* %main, align 4, !dbg !20
      ret i32 %main_ret, !dbg !20

    condition_body:                                   ; preds = %while_body
      br label %continue, !dbg !19

    buffer_block:                                     ; No predecessors!
      br label %continue1, !dbg !21

    continue1:                                        ; preds = %buffer_block, %while_body
      %load_x1 = load i16, i16* %x1, align 2, !dbg !22
      %0 = sext i16 %load_x1 to i32, !dbg !22
      %tmpVar = add i32 %0, 1, !dbg !22
      %1 = trunc i32 %tmpVar to i16, !dbg !22
      store i16 %1, i16* %x1, align 2, !dbg !22
      %load_x13 = load i16, i16* %x1, align 2, !dbg !22
      switch i16 %load_x13, label %else [
        i16 1, label %case
        i16 2, label %case4
        i16 3, label %case5
      ], !dbg !23

    case:                                             ; preds = %continue1
      store i16 1, i16* %x2, align 2, !dbg !24
      br label %continue2, !dbg !25

    case4:                                            ; preds = %continue1
      store i16 2, i16* %x2, align 2, !dbg !26
      br label %continue2, !dbg !25

    case5:                                            ; preds = %continue1
      store i16 3, i16* %x2, align 2, !dbg !27
      br label %continue2, !dbg !25

    else:                                             ; preds = %continue1
      store i16 0, i16* %x1, align 2, !dbg !28
      store i16 1, i16* %x2, align 2, !dbg !29
      store i16 2, i16* %x3, align 2, !dbg !30
      br label %continue2, !dbg !25

    continue2:                                        ; preds = %else, %case5, %case4, %case
      br label %condition_check, !dbg !18
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

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
    !8 = !DILocalVariable(name: "x1", scope: !4, file: !3, line: 4, type: !9, align: 16)
    !9 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !10 = !DILocation(line: 4, column: 16, scope: !4)
    !11 = !DILocalVariable(name: "x2", scope: !4, file: !3, line: 5, type: !9, align: 16)
    !12 = !DILocation(line: 5, column: 16, scope: !4)
    !13 = !DILocalVariable(name: "x3", scope: !4, file: !3, line: 6, type: !9, align: 16)
    !14 = !DILocation(line: 6, column: 16, scope: !4)
    !15 = !DILocalVariable(name: "main", scope: !4, file: !3, line: 2, type: !16, align: 32)
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

    assert_snapshot!(codegen, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %fb = type {}

    @__fb__init = constant %fb zeroinitializer, !dbg !0
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @fb(%fb* %0) !dbg !9 {
    entry:
      call void @llvm.dbg.declare(metadata %fb* %0, metadata !12, metadata !DIExpression()), !dbg !13
      ret void, !dbg !13
    }

    define void @fb_foo(%fb* %0) !dbg !14 {
    entry:
      call void @llvm.dbg.declare(metadata %fb* %0, metadata !15, metadata !DIExpression()), !dbg !16
      ret void, !dbg !16
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init_fb(%fb* %0) {
    entry:
      %self = alloca %fb*, align 8
      store %fb* %0, %fb** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!5, !6}
    !llvm.dbg.cu = !{!7}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__fb__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb", scope: !2, file: !2, line: 2, align: 64, flags: DIFlagPublic, elements: !4, identifier: "fb")
    !4 = !{}
    !5 = !{i32 2, !"Dwarf Version", i32 5}
    !6 = !{i32 2, !"Debug Info Version", i32 3}
    !7 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !8, splitDebugInlining: false)
    !8 = !{!0}
    !9 = distinct !DISubprogram(name: "fb", linkageName: "fb", scope: !2, file: !2, line: 2, type: !10, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !4)
    !10 = !DISubroutineType(flags: DIFlagPublic, types: !11)
    !11 = !{null, !3}
    !12 = !DILocalVariable(name: "fb", scope: !9, file: !2, line: 5, type: !3)
    !13 = !DILocation(line: 5, column: 8, scope: !9)
    !14 = distinct !DISubprogram(name: "fb.foo", linkageName: "fb.foo", scope: !9, file: !2, line: 3, type: !10, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !4)
    !15 = !DILocalVariable(name: "fb", scope: !14, file: !2, line: 4, type: !3)
    !16 = !DILocation(line: 4, column: 8, scope: !14)
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

    assert_snapshot!(codegen, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %PLC_PRG = type {}

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @PLC_PRG_instance = global %PLC_PRG zeroinitializer, !dbg !0

    define i32 @main() !dbg !9 {
    entry:
      %main = alloca i32, align 4
      call void @llvm.dbg.declare(metadata i32* %main, metadata !12, metadata !DIExpression()), !dbg !14
      store i32 0, i32* %main, align 4
      call void @PLC_PRG(%PLC_PRG* @PLC_PRG_instance), !dbg !15
      call void @PLC_PRG_act(%PLC_PRG* @PLC_PRG_instance), !dbg !16
      %main_ret = load i32, i32* %main, align 4, !dbg !17
      ret i32 %main_ret, !dbg !17
    }

    define void @PLC_PRG(%PLC_PRG* %0) !dbg !18 {
    entry:
      call void @llvm.dbg.declare(metadata %PLC_PRG* %0, metadata !21, metadata !DIExpression()), !dbg !22
      %x = alloca i32, align 4
      call void @llvm.dbg.declare(metadata i32* %x, metadata !23, metadata !DIExpression()), !dbg !24
      store i32 0, i32* %x, align 4
      store i32 0, i32* %x, align 4, !dbg !22
      ret void, !dbg !25
    }

    define void @PLC_PRG_act(%PLC_PRG* %0) !dbg !26 {
    entry:
      call void @llvm.dbg.declare(metadata %PLC_PRG* %0, metadata !27, metadata !DIExpression()), !dbg !28
      %x = alloca i32, align 4
      call void @llvm.dbg.declare(metadata i32* %x, metadata !29, metadata !DIExpression()), !dbg !30
      store i32 0, i32* %x, align 4
      %load_x = load i32, i32* %x, align 4, !dbg !28
      %tmpVar = add i32 %load_x, 1, !dbg !28
      store i32 %tmpVar, i32* %x, align 4, !dbg !28
      ret void, !dbg !31
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init_plc_prg(%PLC_PRG* %0) {
    entry:
      %self = alloca %PLC_PRG*, align 8
      store %PLC_PRG* %0, %PLC_PRG** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_plc_prg(%PLC_PRG* @PLC_PRG_instance)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!5, !6}
    !llvm.dbg.cu = !{!7}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "PLC_PRG", scope: !2, file: !2, line: 7, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "PLC_PRG", scope: !2, file: !2, line: 7, align: 64, flags: DIFlagPublic, elements: !4, identifier: "PLC_PRG")
    !4 = !{}
    !5 = !{i32 2, !"Dwarf Version", i32 5}
    !6 = !{i32 2, !"Debug Info Version", i32 3}
    !7 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !8, splitDebugInlining: false)
    !8 = !{!0}
    !9 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 2, type: !10, scopeLine: 3, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !4)
    !10 = !DISubroutineType(flags: DIFlagPublic, types: !11)
    !11 = !{null}
    !12 = !DILocalVariable(name: "main", scope: !9, file: !2, line: 2, type: !13, align: 32)
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
    !23 = !DILocalVariable(name: "x", scope: !18, file: !2, line: 9, type: !13, align: 32)
    !24 = !DILocation(line: 9, column: 12, scope: !18)
    !25 = !DILocation(line: 13, column: 8, scope: !18)
    !26 = distinct !DISubprogram(name: "PLC_PRG.act", linkageName: "PLC_PRG.act", scope: !2, file: !2, line: 16, type: !19, scopeLine: 17, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !4)
    !27 = !DILocalVariable(name: "PLC_PRG", scope: !26, file: !2, line: 17, type: !3)
    !28 = !DILocation(line: 17, column: 16, scope: !26)
    !29 = !DILocalVariable(name: "x", scope: !26, file: !2, line: 9, type: !13, align: 32)
    !30 = !DILocation(line: 9, column: 12, scope: !26)
    !31 = !DILocation(line: 18, column: 12, scope: !26)
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

    assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %struct_ = type { %inner, [3 x %inner], [81 x i8], i8, float, [3 x [81 x i8]], i16 }
    %inner = type { [81 x i8], i8, float, [3 x [81 x i8]], i16 }

    @__struct___init = constant %struct_ { %inner { [81 x i8] c"Hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", i8 1, float 0x400921CAC0000000, [3 x [81 x i8]] [[81 x i8] c"aaaa\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"bbbb\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"cccc\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"], i16 42 }, [3 x %inner] zeroinitializer, [81 x i8] c"Hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", i8 1, float 0x400921CAC0000000, [3 x [81 x i8]] [[81 x i8] c"aa\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"bb\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"cc\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"], i16 42 }, !dbg !0
    @__inner__init = constant %inner { [81 x i8] c"Hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", i8 1, float 0x400921CAC0000000, [3 x [81 x i8]] [[81 x i8] c"aaaa\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"bbbb\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"cccc\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"], i16 42 }, !dbg !30
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @main() !dbg !36 {
    entry:
      %st = alloca %struct_, align 8
      %s = alloca [81 x i8], align 1
      %b = alloca i8, align 1
      %arr = alloca [3 x [81 x i8]], align 1
      %i = alloca i16, align 2
      call void @llvm.dbg.declare(metadata %struct_* %st, metadata !40, metadata !DIExpression()), !dbg !41
      %0 = bitcast %struct_* %st to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 getelementptr inbounds (%struct_, %struct_* @__struct___init, i32 0, i32 0, i32 0, i32 0), i64 ptrtoint (%struct_* getelementptr (%struct_, %struct_* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata [81 x i8]* %s, metadata !42, metadata !DIExpression()), !dbg !43
      %1 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %1, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata i8* %b, metadata !44, metadata !DIExpression()), !dbg !45
      store i8 0, i8* %b, align 1
      call void @llvm.dbg.declare(metadata [3 x [81 x i8]]* %arr, metadata !46, metadata !DIExpression()), !dbg !47
      %2 = bitcast [3 x [81 x i8]]* %arr to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %2, i8 0, i64 ptrtoint ([3 x [81 x i8]]* getelementptr ([3 x [81 x i8]], [3 x [81 x i8]]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata i16* %i, metadata !48, metadata !DIExpression()), !dbg !49
      store i16 0, i16* %i, align 2
      call void @__init_struct_(%struct_* %st), !dbg !50
      %s1 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 2, !dbg !51
      %3 = bitcast [81 x i8]* %s to i8*, !dbg !51
      %4 = bitcast [81 x i8]* %s1 to i8*, !dbg !51
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %3, i8* align 1 %4, i32 80, i1 false), !dbg !51
      %inner = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 0, !dbg !52
      %s2 = getelementptr inbounds %inner, %inner* %inner, i32 0, i32 0, !dbg !52
      %5 = bitcast [81 x i8]* %s to i8*, !dbg !52
      %6 = bitcast [81 x i8]* %s2 to i8*, !dbg !52
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %5, i8* align 1 %6, i32 80, i1 false), !dbg !52
      %b3 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 3, !dbg !53
      %load_b = load i8, i8* %b3, align 1, !dbg !53
      store i8 %load_b, i8* %b, align 1, !dbg !53
      %inner4 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 0, !dbg !54
      %b5 = getelementptr inbounds %inner, %inner* %inner4, i32 0, i32 1, !dbg !54
      %load_b6 = load i8, i8* %b5, align 1, !dbg !54
      store i8 %load_b6, i8* %b, align 1, !dbg !54
      %arr7 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 5, !dbg !55
      %7 = bitcast [3 x [81 x i8]]* %arr to i8*, !dbg !55
      %8 = bitcast [3 x [81 x i8]]* %arr7 to i8*, !dbg !55
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %7, i8* align 1 %8, i64 ptrtoint ([3 x [81 x i8]]* getelementptr ([3 x [81 x i8]], [3 x [81 x i8]]* null, i32 1) to i64), i1 false), !dbg !55
      %inner8 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 0, !dbg !56
      %arr9 = getelementptr inbounds %inner, %inner* %inner8, i32 0, i32 3, !dbg !56
      %9 = bitcast [3 x [81 x i8]]* %arr to i8*, !dbg !56
      %10 = bitcast [3 x [81 x i8]]* %arr9 to i8*, !dbg !56
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %9, i8* align 1 %10, i64 ptrtoint ([3 x [81 x i8]]* getelementptr ([3 x [81 x i8]], [3 x [81 x i8]]* null, i32 1) to i64), i1 false), !dbg !56
      %i10 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 6, !dbg !57
      %load_i = load i16, i16* %i10, align 2, !dbg !57
      store i16 %load_i, i16* %i, align 2, !dbg !57
      %inner11 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 0, !dbg !58
      %i12 = getelementptr inbounds %inner, %inner* %inner11, i32 0, i32 4, !dbg !58
      %load_i13 = load i16, i16* %i12, align 2, !dbg !58
      store i16 %load_i13, i16* %i, align 2, !dbg !58
      %tmpVar = getelementptr inbounds [3 x [81 x i8]], [3 x [81 x i8]]* %arr, i32 0, i32 0, !dbg !59
      %arr14 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 5, !dbg !59
      %tmpVar15 = getelementptr inbounds [3 x [81 x i8]], [3 x [81 x i8]]* %arr14, i32 0, i32 0, !dbg !59
      %11 = bitcast [81 x i8]* %tmpVar to i8*, !dbg !59
      %12 = bitcast [81 x i8]* %tmpVar15 to i8*, !dbg !59
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %11, i8* align 1 %12, i32 80, i1 false), !dbg !59
      %tmpVar16 = getelementptr inbounds [3 x [81 x i8]], [3 x [81 x i8]]* %arr, i32 0, i32 1, !dbg !60
      %inner17 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 0, !dbg !60
      %arr18 = getelementptr inbounds %inner, %inner* %inner17, i32 0, i32 3, !dbg !60
      %tmpVar19 = getelementptr inbounds [3 x [81 x i8]], [3 x [81 x i8]]* %arr18, i32 0, i32 1, !dbg !60
      %13 = bitcast [81 x i8]* %tmpVar16 to i8*, !dbg !60
      %14 = bitcast [81 x i8]* %tmpVar19 to i8*, !dbg !60
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %13, i8* align 1 %14, i32 80, i1 false), !dbg !60
      %tmpVar20 = getelementptr inbounds [3 x [81 x i8]], [3 x [81 x i8]]* %arr, i32 0, i32 2, !dbg !61
      %inner21 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 0, !dbg !61
      %arr22 = getelementptr inbounds %inner, %inner* %inner21, i32 0, i32 3, !dbg !61
      %tmpVar23 = getelementptr inbounds [3 x [81 x i8]], [3 x [81 x i8]]* %arr22, i32 0, i32 2, !dbg !61
      %15 = bitcast [81 x i8]* %tmpVar20 to i8*, !dbg !61
      %16 = bitcast [81 x i8]* %tmpVar23 to i8*, !dbg !61
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %15, i8* align 1 %16, i32 80, i1 false), !dbg !61
      ret void, !dbg !62
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #2

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #1

    define void @__init_struct_(%struct_* %0) {
    entry:
      %self = alloca %struct_*, align 8
      store %struct_* %0, %struct_** %self, align 8
      %deref = load %struct_*, %struct_** %self, align 8
      %inner = getelementptr inbounds %struct_, %struct_* %deref, i32 0, i32 0
      call void @__init_inner(%inner* %inner)
      ret void
    }

    define void @__init_inner(%inner* %0) {
    entry:
      %self = alloca %inner*, align 8
      store %inner* %0, %inner** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn }
    attributes #2 = { argmemonly nofree nounwind willreturn writeonly }

    !llvm.module.flags = !{!32, !33}
    !llvm.dbg.cu = !{!34}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__struct___init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "struct_", scope: !2, file: !2, line: 2, size: 13440, align: 64, flags: DIFlagPublic, elements: !4, identifier: "struct_")
    !4 = !{!5, !23, !25, !26, !27, !28, !29}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "inner", scope: !2, file: !2, line: 3, baseType: !6, size: 2688, align: 64, flags: DIFlagPublic)
    !6 = !DICompositeType(tag: DW_TAG_structure_type, name: "inner", scope: !2, file: !2, line: 13, size: 2688, align: 64, flags: DIFlagPublic, elements: !7, identifier: "inner")
    !7 = !{!8, !13, !15, !17, !21}
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !2, file: !2, line: 14, baseType: !9, size: 648, align: 8, flags: DIFlagPublic)
    !9 = !DICompositeType(tag: DW_TAG_array_type, baseType: !10, size: 648, align: 8, elements: !11)
    !10 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !11 = !{!12}
    !12 = !DISubrange(count: 81, lowerBound: 0)
    !13 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 15, baseType: !14, size: 8, align: 8, offset: 648, flags: DIFlagPublic)
    !14 = !DIBasicType(name: "BOOL", size: 8, encoding: DW_ATE_boolean, flags: DIFlagPublic)
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "r", scope: !2, file: !2, line: 16, baseType: !16, size: 32, align: 32, offset: 672, flags: DIFlagPublic)
    !16 = !DIBasicType(name: "REAL", size: 32, encoding: DW_ATE_float, flags: DIFlagPublic)
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "arr", scope: !2, file: !2, line: 17, baseType: !18, size: 1944, align: 8, offset: 704, flags: DIFlagPublic)
    !18 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 1944, align: 8, elements: !19)
    !19 = !{!20}
    !20 = !DISubrange(count: 3, lowerBound: 0)
    !21 = !DIDerivedType(tag: DW_TAG_member, name: "i", scope: !2, file: !2, line: 18, baseType: !22, size: 16, align: 16, offset: 2656, flags: DIFlagPublic)
    !22 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !23 = !DIDerivedType(tag: DW_TAG_member, name: "inner_arr", scope: !2, file: !2, line: 4, baseType: !24, size: 8064, align: 64, offset: 2688, flags: DIFlagPublic)
    !24 = !DICompositeType(tag: DW_TAG_array_type, baseType: !6, size: 8064, align: 64, elements: !19)
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !2, file: !2, line: 5, baseType: !9, size: 648, align: 8, offset: 10752, flags: DIFlagPublic)
    !26 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 6, baseType: !14, size: 8, align: 8, offset: 11400, flags: DIFlagPublic)
    !27 = !DIDerivedType(tag: DW_TAG_member, name: "r", scope: !2, file: !2, line: 7, baseType: !16, size: 32, align: 32, offset: 11424, flags: DIFlagPublic)
    !28 = !DIDerivedType(tag: DW_TAG_member, name: "arr", scope: !2, file: !2, line: 8, baseType: !18, size: 1944, align: 8, offset: 11456, flags: DIFlagPublic)
    !29 = !DIDerivedType(tag: DW_TAG_member, name: "i", scope: !2, file: !2, line: 9, baseType: !22, size: 16, align: 16, offset: 13408, flags: DIFlagPublic)
    !30 = !DIGlobalVariableExpression(var: !31, expr: !DIExpression())
    !31 = distinct !DIGlobalVariable(name: "__inner__init", scope: !2, file: !2, line: 13, type: !6, isLocal: false, isDefinition: true)
    !32 = !{i32 2, !"Dwarf Version", i32 5}
    !33 = !{i32 2, !"Debug Info Version", i32 3}
    !34 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !35, splitDebugInlining: false)
    !35 = !{!0, !30}
    !36 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 22, type: !37, scopeLine: 22, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !34, retainedNodes: !39)
    !37 = !DISubroutineType(flags: DIFlagPublic, types: !38)
    !38 = !{null}
    !39 = !{}
    !40 = !DILocalVariable(name: "st", scope: !36, file: !2, line: 24, type: !3, align: 64)
    !41 = !DILocation(line: 24, column: 4, scope: !36)
    !42 = !DILocalVariable(name: "s", scope: !36, file: !2, line: 25, type: !9, align: 8)
    !43 = !DILocation(line: 25, column: 4, scope: !36)
    !44 = !DILocalVariable(name: "b", scope: !36, file: !2, line: 26, type: !14, align: 8)
    !45 = !DILocation(line: 26, column: 4, scope: !36)
    !46 = !DILocalVariable(name: "arr", scope: !36, file: !2, line: 27, type: !18, align: 8)
    !47 = !DILocation(line: 27, column: 4, scope: !36)
    !48 = !DILocalVariable(name: "i", scope: !36, file: !2, line: 28, type: !22, align: 16)
    !49 = !DILocation(line: 28, column: 4, scope: !36)
    !50 = !DILocation(line: 0, scope: !36)
    !51 = !DILocation(line: 32, column: 4, scope: !36)
    !52 = !DILocation(line: 33, column: 4, scope: !36)
    !53 = !DILocation(line: 34, column: 4, scope: !36)
    !54 = !DILocation(line: 35, column: 4, scope: !36)
    !55 = !DILocation(line: 36, column: 4, scope: !36)
    !56 = !DILocation(line: 37, column: 4, scope: !36)
    !57 = !DILocation(line: 38, column: 4, scope: !36)
    !58 = !DILocation(line: 39, column: 4, scope: !36)
    !59 = !DILocation(line: 41, column: 4, scope: !36)
    !60 = !DILocation(line: 42, column: 4, scope: !36)
    !61 = !DILocation(line: 43, column: 4, scope: !36)
    !62 = !DILocation(line: 45, scope: !36)
    "#);
}
