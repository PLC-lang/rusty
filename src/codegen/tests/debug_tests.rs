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
        b : REF_TO ARRAY[0..10] DINT;
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
      %main = alloca i32, align 4, !dbg !8
      %x1 = alloca i16, align 2, !dbg !8
      %x2 = alloca i16, align 2, !dbg !8
      %x3 = alloca i16, align 2, !dbg !8
      call void @llvm.dbg.declare(metadata i16* %x1, metadata !9, metadata !DIExpression()), !dbg !11
      store i16 0, i16* %x1, align 2, !dbg !8
      call void @llvm.dbg.declare(metadata i16* %x2, metadata !12, metadata !DIExpression()), !dbg !13
      store i16 0, i16* %x2, align 2, !dbg !8
      call void @llvm.dbg.declare(metadata i16* %x3, metadata !14, metadata !DIExpression()), !dbg !15
      store i16 0, i16* %x3, align 2, !dbg !8
      call void @llvm.dbg.declare(metadata i32* %main, metadata !16, metadata !DIExpression()), !dbg !18
      store i32 0, i32* %main, align 4, !dbg !8
      br label %condition_check, !dbg !19

    condition_check:                                  ; preds = %continue2, %entry
      br i1 true, label %while_body, label %continue, !dbg !20

    while_body:                                       ; preds = %condition_check
      br i1 false, label %condition_body, label %continue1, !dbg !20

    continue:                                         ; preds = %condition_body, %condition_check
      %main_ret = load i32, i32* %main, align 4, !dbg !19
      ret i32 %main_ret, !dbg !19

    condition_body:                                   ; preds = %while_body
      br label %continue, !dbg !20

    buffer_block:                                     ; No predecessors!
      br label %continue1, !dbg !20

    continue1:                                        ; preds = %buffer_block, %while_body
      %load_x1 = load i16, i16* %x1, align 2, !dbg !21
      %0 = sext i16 %load_x1 to i32, !dbg !21
      %tmpVar = add i32 %0, 1, !dbg !21
      %1 = trunc i32 %tmpVar to i16, !dbg !21
      store i16 %1, i16* %x1, align 2, !dbg !21
      %load_x13 = load i16, i16* %x1, align 2, !dbg !21
      switch i16 %load_x13, label %else [
        i16 1, label %case
        i16 2, label %case4
        i16 3, label %case5
      ], !dbg !19

    case:                                             ; preds = %continue1
      store i16 1, i16* %x2, align 2, !dbg !22
      br label %continue2, !dbg !22

    case4:                                            ; preds = %continue1
      store i16 2, i16* %x2, align 2, !dbg !23
      br label %continue2, !dbg !23

    case5:                                            ; preds = %continue1
      store i16 3, i16* %x2, align 2, !dbg !24
      br label %continue2, !dbg !24

    else:                                             ; preds = %continue1
      store i16 0, i16* %x1, align 2, !dbg !25
      store i16 1, i16* %x2, align 2, !dbg !26
      store i16 2, i16* %x3, align 2, !dbg !27
      br label %continue2, !dbg !27

    continue2:                                        ; preds = %else, %case5, %case4, %case
      br label %condition_check, !dbg !19
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
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "<test>", directory: "")
    !4 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !3, file: !3, line: 2, type: !5, scopeLine: 9, flags: DIFlagPublic, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !2, retainedNodes: !7)
    !5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
    !6 = !{null}
    !7 = !{}
    !8 = !DILocation(line: 9, column: 12, scope: !4)
    !9 = !DILocalVariable(name: "x1", scope: !4, file: !3, line: 4, type: !10, align: 16)
    !10 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !11 = !DILocation(line: 4, column: 16, scope: !4)
    !12 = !DILocalVariable(name: "x2", scope: !4, file: !3, line: 5, type: !10, align: 16)
    !13 = !DILocation(line: 5, column: 16, scope: !4)
    !14 = !DILocalVariable(name: "x3", scope: !4, file: !3, line: 6, type: !10, align: 16)
    !15 = !DILocation(line: 6, column: 16, scope: !4)
    !16 = !DILocalVariable(name: "main", scope: !4, file: !3, line: 2, type: !17, align: 32)
    !17 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !18 = !DILocation(line: 2, column: 17, scope: !4)
    !19 = !DILocation(line: 12, column: 17, scope: !4)
    !20 = !DILocation(line: 9, column: 18, scope: !4)
    !21 = !DILocation(line: 10, column: 12, scope: !4)
    !22 = !DILocation(line: 13, column: 19, scope: !4)
    !23 = !DILocation(line: 14, column: 19, scope: !4)
    !24 = !DILocation(line: 15, column: 19, scope: !4)
    !25 = !DILocation(line: 17, column: 20, scope: !4)
    !26 = !DILocation(line: 18, column: 20, scope: !4)
    !27 = !DILocation(line: 19, column: 20, scope: !4)
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

    // We want to make sure the `dbg.declare` for the method `foo` references a non-empty metadata field, i.e.
    // `!<number>` should not be `!<number> = {}`. Concretely, `!17` should be non-empty
    assert!(codegen.contains(
        r#"call void @llvm.dbg.declare(metadata %fb* %0, metadata !13, metadata !DIExpression()), !dbg !16"#
    ));

    assert_snapshot!(codegen, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %fb = type {}

    @__fb__init = unnamed_addr constant %fb zeroinitializer, !dbg !0

    define void @fb(%fb* %0) !dbg !10 {
    entry:
      call void @llvm.dbg.declare(metadata %fb* %0, metadata !13, metadata !DIExpression()), !dbg !14
      ret void, !dbg !14
    }

    define void @fb.foo(%fb* %0) !dbg !15 {
    entry:
      call void @llvm.dbg.declare(metadata %fb* %0, metadata !13, metadata !DIExpression()), !dbg !16
      ret void, !dbg !16
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

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
    !7 = distinct !DICompileUnit(language: DW_LANG_C, file: !8, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !9, splitDebugInlining: false)
    !8 = !DIFile(filename: "<internal>", directory: "src")
    !9 = !{!0}
    !10 = distinct !DISubprogram(name: "fb", linkageName: "fb", scope: !2, file: !2, line: 2, type: !11, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !4)
    !11 = !DISubroutineType(flags: DIFlagPublic, types: !12)
    !12 = !{null}
    !13 = !DILocalVariable(name: "fb", scope: !10, file: !2, line: 2, type: !3)
    !14 = !DILocation(line: 5, column: 8, scope: !10)
    !15 = distinct !DISubprogram(name: "fb.foo", linkageName: "fb.foo", scope: !2, file: !2, line: 3, type: !11, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !4)
    !16 = !DILocation(line: 4, column: 8, scope: !15)
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %fb = type {}

    @__fb__init = external global %fb, !dbg !0

    define void @__init_fb(%fb* %0) !dbg !10 {
    entry:
      %self = alloca %fb*, align 8, !dbg !14
      call void @llvm.dbg.declare(metadata %fb** %self, metadata !15, metadata !DIExpression()), !dbg !14
      store %fb* %0, %fb** %self, align 8, !dbg !14
      ret void, !dbg !14
    }

    declare !dbg !16 void @fb(%fb*)

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

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
    !7 = distinct !DICompileUnit(language: DW_LANG_C, file: !8, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !9, splitDebugInlining: false)
    !8 = !DIFile(filename: "__initializers", directory: "src")
    !9 = !{!0}
    !10 = distinct !DISubprogram(name: "__init_fb", linkageName: "__init_fb", scope: !2, file: !2, line: 2, type: !11, scopeLine: 2, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !4)
    !11 = !DISubroutineType(flags: DIFlagPublic, types: !12)
    !12 = !{null, !13}
    !13 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_fb", baseType: !3, size: 64, align: 64, dwarfAddressSpace: 1)
    !14 = !DILocation(line: 2, column: 23, scope: !10)
    !15 = !DILocalVariable(name: "self", scope: !10, file: !2, line: 2, type: !13)
    !16 = distinct !DISubprogram(name: "fb", linkageName: "fb", scope: !2, file: !2, line: 2, type: !17, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !7, retainedNodes: !4)
    !17 = !DISubroutineType(flags: DIFlagPublic, types: !18)
    !18 = !{null}
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() !dbg !4 {
    entry:
      ret void, !dbg !9
    }

    !llvm.module.flags = !{!0, !1}
    !llvm.dbg.cu = !{!2}

    !0 = !{i32 2, !"Dwarf Version", i32 5}
    !1 = !{i32 2, !"Debug Info Version", i32 3}
    !2 = distinct !DICompileUnit(language: DW_LANG_C, file: !3, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
    !3 = !DIFile(filename: "__init___testproject", directory: "src")
    !4 = distinct !DISubprogram(name: "__init___testproject", linkageName: "__init___testproject", scope: !5, file: !5, type: !6, scopeLine: 1, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !8)
    !5 = !DIFile(filename: "<internal>", directory: "")
    !6 = !DISubroutineType(flags: DIFlagPublic, types: !7)
    !7 = !{null}
    !8 = !{}
    !9 = !DILocation(line: 0, scope: !4)
    "#);
}
