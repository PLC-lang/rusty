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

    assert_snapshot!(codegen, @r###"
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
    "###);
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

    assert_snapshot!(codegen, @r###"
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

    define void @fb.foo(%fb* %0) !dbg !14 {
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
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb", scope: !2, file: !2, line: 2, flags: DIFlagPublic, elements: !4, identifier: "fb")
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
    "###);
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

    assert_snapshot!(codegen, @r###"
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
      call void @PLC_PRG.act(%PLC_PRG* @PLC_PRG_instance), !dbg !16
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

    define void @PLC_PRG.act(%PLC_PRG* %0) !dbg !26 {
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
    "###);
}
