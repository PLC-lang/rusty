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

    filtered_assert_snapshot!(codegen, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_fb = type { void (%fb*)*, void (%fb*)* }
    %fb = type { i32* }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_fb__init = unnamed_addr constant %__vtable_fb zeroinitializer
    @__fb__init = unnamed_addr constant %fb zeroinitializer, !dbg !0
    @__vtable_fb_instance = global %__vtable_fb zeroinitializer

    define void @fb(%fb* %0) !dbg !14 {
    entry:
      call void @llvm.dbg.declare(metadata %fb* %0, metadata !18, metadata !DIExpression()), !dbg !19
      %this = alloca %fb*, align 8
      store %fb* %0, %fb** %this, align 8
      %__vtable = getelementptr inbounds %fb, %fb* %0, i32 0, i32 0
      ret void, !dbg !19
    }

    define void @fb__foo(%fb* %0) !dbg !20 {
    entry:
      call void @llvm.dbg.declare(metadata %fb* %0, metadata !21, metadata !DIExpression()), !dbg !22
      %this = alloca %fb*, align 8
      store %fb* %0, %fb** %this, align 8
      %__vtable = getelementptr inbounds %fb, %fb* %0, i32 0, i32 0
      ret void, !dbg !22
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init___vtable_fb(%__vtable_fb* %0) {
    entry:
      %self = alloca %__vtable_fb*, align 8
      store %__vtable_fb* %0, %__vtable_fb** %self, align 8
      %deref = load %__vtable_fb*, %__vtable_fb** %self, align 8
      %__body = getelementptr inbounds %__vtable_fb, %__vtable_fb* %deref, i32 0, i32 0
      store void (%fb*)* @fb, void (%fb*)** %__body, align 8
      %deref1 = load %__vtable_fb*, %__vtable_fb** %self, align 8
      %foo = getelementptr inbounds %__vtable_fb, %__vtable_fb* %deref1, i32 0, i32 1
      store void (%fb*)* @fb__foo, void (%fb*)** %foo, align 8
      ret void
    }

    define void @__init_fb(%fb* %0) {
    entry:
      %self = alloca %fb*, align 8
      store %fb* %0, %fb** %self, align 8
      %deref = load %fb*, %fb** %self, align 8
      %__vtable = getelementptr inbounds %fb, %fb* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_fb* @__vtable_fb_instance to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_fb(%fb* %0) {
    entry:
      %self = alloca %fb*, align 8
      store %fb* %0, %fb** %self, align 8
      ret void
    }

    define void @__user_init___vtable_fb(%__vtable_fb* %0) {
    entry:
      %self = alloca %__vtable_fb*, align 8
      store %__vtable_fb* %0, %__vtable_fb** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_fb(%__vtable_fb* @__vtable_fb_instance)
      call void @__user_init___vtable_fb(%__vtable_fb* @__vtable_fb_instance)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!10, !11}
    !llvm.dbg.cu = !{!12}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__fb__init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "fb", scope: !2, file: !2, line: 2, size: 64, align: 64, flags: DIFlagPublic, elements: !5, identifier: "fb")
    !5 = !{!6}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "__vtable", scope: !2, file: !2, baseType: !7, size: 64, align: 64, flags: DIFlagPublic)
    !7 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____fb___vtable", scope: !2, file: !2, baseType: !8, align: 64)
    !8 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__fb___vtable", baseType: !9, size: 64, align: 64, dwarfAddressSpace: 1)
    !9 = !DIBasicType(name: "__VOID", encoding: DW_ATE_unsigned, flags: DIFlagPublic)
    !10 = !{i32 2, !"Dwarf Version", i32 5}
    !11 = !{i32 2, !"Debug Info Version", i32 3}
    !12 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !13, splitDebugInlining: false)
    !13 = !{!0}
    !14 = distinct !DISubprogram(name: "fb", linkageName: "fb", scope: !2, file: !2, line: 2, type: !15, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !12, retainedNodes: !17)
    !15 = !DISubroutineType(flags: DIFlagPublic, types: !16)
    !16 = !{null, !4}
    !17 = !{}
    !18 = !DILocalVariable(name: "fb", scope: !14, file: !2, line: 5, type: !4)
    !19 = !DILocation(line: 5, column: 8, scope: !14)
    !20 = distinct !DISubprogram(name: "fb.foo", linkageName: "fb.foo", scope: !14, file: !2, line: 3, type: !15, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !12, retainedNodes: !17)
    !21 = !DILocalVariable(name: "fb", scope: !20, file: !2, line: 4, type: !4)
    !22 = !DILocation(line: 4, column: 8, scope: !20)
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

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @PLC_PRG_instance = global %PLC_PRG zeroinitializer, !dbg !0

    define i32 @main() !dbg !9 {
    entry:
      %main = alloca i32, align 4
      call void @llvm.dbg.declare(metadata i32* %main, metadata !12, metadata !DIExpression()), !dbg !14
      store i32 0, i32* %main, align 4
      call void @PLC_PRG(%PLC_PRG* @PLC_PRG_instance), !dbg !15
      call void @PLC_PRG__act(%PLC_PRG* @PLC_PRG_instance), !dbg !16
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

    define void @PLC_PRG__act(%PLC_PRG* %0) !dbg !26 {
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

    define void @__user_init_PLC_PRG(%PLC_PRG* %0) {
    entry:
      %self = alloca %PLC_PRG*, align 8
      store %PLC_PRG* %0, %PLC_PRG** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_plc_prg(%PLC_PRG* @PLC_PRG_instance)
      call void @__user_init_PLC_PRG(%PLC_PRG* @PLC_PRG_instance)
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

    filtered_assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %struct_ = type { %inner, [3 x %inner], [81 x i8], i8, float, [3 x [81 x i8]], i16 }
    %inner = type { [81 x i8], i8, float, [3 x [81 x i8]], i16 }

    @__struct___init = unnamed_addr constant %struct_ { %inner { [81 x i8] c"Hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", i8 1, float 0x400921CAC0000000, [3 x [81 x i8]] [[81 x i8] c"aaaa\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"bbbb\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"cccc\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"], i16 42 }, [3 x %inner] zeroinitializer, [81 x i8] c"Hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", i8 1, float 0x400921CAC0000000, [3 x [81 x i8]] [[81 x i8] c"aa\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"bb\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"cc\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"], i16 42 }, !dbg !0
    @__inner__init = unnamed_addr constant %inner { [81 x i8] c"Hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", i8 1, float 0x400921CAC0000000, [3 x [81 x i8]] [[81 x i8] c"aaaa\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"bbbb\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"cccc\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"], i16 42 }, !dbg !32
    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"Hello\00"
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @main() !dbg !39 {
    entry:
      %st = alloca %struct_, align 8
      %s = alloca [81 x i8], align 1
      %b = alloca i8, align 1
      %arr = alloca [3 x [81 x i8]], align 1
      %i = alloca i16, align 2
      call void @llvm.dbg.declare(metadata %struct_* %st, metadata !43, metadata !DIExpression()), !dbg !44
      %0 = bitcast %struct_* %st to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 getelementptr inbounds (%struct_, %struct_* @__struct___init, i32 0, i32 0, i32 0, i32 0), i64 ptrtoint (%struct_* getelementptr (%struct_, %struct_* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata [81 x i8]* %s, metadata !45, metadata !DIExpression()), !dbg !46
      %1 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %1, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata i8* %b, metadata !47, metadata !DIExpression()), !dbg !48
      store i8 0, i8* %b, align 1
      call void @llvm.dbg.declare(metadata [3 x [81 x i8]]* %arr, metadata !49, metadata !DIExpression()), !dbg !50
      %2 = bitcast [3 x [81 x i8]]* %arr to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %2, i8 0, i64 ptrtoint ([3 x [81 x i8]]* getelementptr ([3 x [81 x i8]], [3 x [81 x i8]]* null, i32 1) to i64), i1 false)
      call void @llvm.dbg.declare(metadata i16* %i, metadata !51, metadata !DIExpression()), !dbg !52
      store i16 0, i16* %i, align 2
      call void @__init_struct_(%struct_* %st), !dbg !53
      call void @__user_init_struct_(%struct_* %st), !dbg !53
      %s1 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 2, !dbg !54
      %3 = bitcast [81 x i8]* %s to i8*, !dbg !54
      %4 = bitcast [81 x i8]* %s1 to i8*, !dbg !54
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %3, i8* align 1 %4, i32 80, i1 false), !dbg !54
      %inner = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 0, !dbg !55
      %s2 = getelementptr inbounds %inner, %inner* %inner, i32 0, i32 0, !dbg !55
      %5 = bitcast [81 x i8]* %s to i8*, !dbg !55
      %6 = bitcast [81 x i8]* %s2 to i8*, !dbg !55
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %5, i8* align 1 %6, i32 80, i1 false), !dbg !55
      %b3 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 3, !dbg !56
      %load_b = load i8, i8* %b3, align 1, !dbg !56
      store i8 %load_b, i8* %b, align 1, !dbg !56
      %inner4 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 0, !dbg !57
      %b5 = getelementptr inbounds %inner, %inner* %inner4, i32 0, i32 1, !dbg !57
      %load_b6 = load i8, i8* %b5, align 1, !dbg !57
      store i8 %load_b6, i8* %b, align 1, !dbg !57
      %arr7 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 5, !dbg !58
      %7 = bitcast [3 x [81 x i8]]* %arr to i8*, !dbg !58
      %8 = bitcast [3 x [81 x i8]]* %arr7 to i8*, !dbg !58
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %7, i8* align 1 %8, i64 ptrtoint ([3 x [81 x i8]]* getelementptr ([3 x [81 x i8]], [3 x [81 x i8]]* null, i32 1) to i64), i1 false), !dbg !58
      %inner8 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 0, !dbg !59
      %arr9 = getelementptr inbounds %inner, %inner* %inner8, i32 0, i32 3, !dbg !59
      %9 = bitcast [3 x [81 x i8]]* %arr to i8*, !dbg !59
      %10 = bitcast [3 x [81 x i8]]* %arr9 to i8*, !dbg !59
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %9, i8* align 1 %10, i64 ptrtoint ([3 x [81 x i8]]* getelementptr ([3 x [81 x i8]], [3 x [81 x i8]]* null, i32 1) to i64), i1 false), !dbg !59
      %i10 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 6, !dbg !60
      %load_i = load i16, i16* %i10, align 2, !dbg !60
      store i16 %load_i, i16* %i, align 2, !dbg !60
      %inner11 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 0, !dbg !61
      %i12 = getelementptr inbounds %inner, %inner* %inner11, i32 0, i32 4, !dbg !61
      %load_i13 = load i16, i16* %i12, align 2, !dbg !61
      store i16 %load_i13, i16* %i, align 2, !dbg !61
      %tmpVar = getelementptr inbounds [3 x [81 x i8]], [3 x [81 x i8]]* %arr, i32 0, i32 0, !dbg !62
      %arr14 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 5, !dbg !62
      %tmpVar15 = getelementptr inbounds [3 x [81 x i8]], [3 x [81 x i8]]* %arr14, i32 0, i32 0, !dbg !62
      %11 = bitcast [81 x i8]* %tmpVar to i8*, !dbg !62
      %12 = bitcast [81 x i8]* %tmpVar15 to i8*, !dbg !62
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %11, i8* align 1 %12, i32 80, i1 false), !dbg !62
      %tmpVar16 = getelementptr inbounds [3 x [81 x i8]], [3 x [81 x i8]]* %arr, i32 0, i32 1, !dbg !63
      %inner17 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 0, !dbg !63
      %arr18 = getelementptr inbounds %inner, %inner* %inner17, i32 0, i32 3, !dbg !63
      %tmpVar19 = getelementptr inbounds [3 x [81 x i8]], [3 x [81 x i8]]* %arr18, i32 0, i32 1, !dbg !63
      %13 = bitcast [81 x i8]* %tmpVar16 to i8*, !dbg !63
      %14 = bitcast [81 x i8]* %tmpVar19 to i8*, !dbg !63
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %13, i8* align 1 %14, i32 80, i1 false), !dbg !63
      %tmpVar20 = getelementptr inbounds [3 x [81 x i8]], [3 x [81 x i8]]* %arr, i32 0, i32 2, !dbg !64
      %inner21 = getelementptr inbounds %struct_, %struct_* %st, i32 0, i32 0, !dbg !64
      %arr22 = getelementptr inbounds %inner, %inner* %inner21, i32 0, i32 3, !dbg !64
      %tmpVar23 = getelementptr inbounds [3 x [81 x i8]], [3 x [81 x i8]]* %arr22, i32 0, i32 2, !dbg !64
      %15 = bitcast [81 x i8]* %tmpVar20 to i8*, !dbg !64
      %16 = bitcast [81 x i8]* %tmpVar23 to i8*, !dbg !64
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %15, i8* align 1 %16, i32 80, i1 false), !dbg !64
      ret void, !dbg !65
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
      %deref1 = load %struct_*, %struct_** %self, align 8
      %s = getelementptr inbounds %struct_, %struct_* %deref1, i32 0, i32 2
      %1 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_0, i32 0, i32 0), i32 6, i1 false)
      %deref2 = load %struct_*, %struct_** %self, align 8
      %b = getelementptr inbounds %struct_, %struct_* %deref2, i32 0, i32 3
      store i8 1, i8* %b, align 1
      %deref3 = load %struct_*, %struct_** %self, align 8
      %r = getelementptr inbounds %struct_, %struct_* %deref3, i32 0, i32 4
      store float 0x400921CAC0000000, float* %r, align 4
      %deref4 = load %struct_*, %struct_** %self, align 8
      %i = getelementptr inbounds %struct_, %struct_* %deref4, i32 0, i32 6
      store i16 42, i16* %i, align 2
      ret void
    }

    define void @__init_inner(%inner* %0) {
    entry:
      %self = alloca %inner*, align 8
      store %inner* %0, %inner** %self, align 8
      %deref = load %inner*, %inner** %self, align 8
      %s = getelementptr inbounds %inner, %inner* %deref, i32 0, i32 0
      %1 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_0, i32 0, i32 0), i32 6, i1 false)
      %deref1 = load %inner*, %inner** %self, align 8
      %b = getelementptr inbounds %inner, %inner* %deref1, i32 0, i32 1
      store i8 1, i8* %b, align 1
      %deref2 = load %inner*, %inner** %self, align 8
      %r = getelementptr inbounds %inner, %inner* %deref2, i32 0, i32 2
      store float 0x400921CAC0000000, float* %r, align 4
      %deref3 = load %inner*, %inner** %self, align 8
      %i = getelementptr inbounds %inner, %inner* %deref3, i32 0, i32 4
      store i16 42, i16* %i, align 2
      ret void
    }

    define void @__user_init_inner(%inner* %0) {
    entry:
      %self = alloca %inner*, align 8
      store %inner* %0, %inner** %self, align 8
      ret void
    }

    define void @__user_init_struct_(%struct_* %0) {
    entry:
      %self = alloca %struct_*, align 8
      store %struct_* %0, %struct_** %self, align 8
      %deref = load %struct_*, %struct_** %self, align 8
      %inner = getelementptr inbounds %struct_, %struct_* %deref, i32 0, i32 0
      call void @__user_init_inner(%inner* %inner)
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn }
    attributes #2 = { argmemonly nofree nounwind willreturn writeonly }

    !llvm.module.flags = !{!35, !36}
    !llvm.dbg.cu = !{!37}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__struct___init", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "struct_", scope: !2, file: !2, line: 2, size: 13440, align: 64, flags: DIFlagPublic, elements: !5, identifier: "struct_")
    !5 = !{!6, !25, !27, !28, !29, !30, !31}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "inner", scope: !2, file: !2, line: 3, baseType: !7, size: 2688, align: 64, flags: DIFlagPublic)
    !7 = !DICompositeType(tag: DW_TAG_structure_type, name: "inner", scope: !2, file: !2, line: 13, size: 2688, align: 64, flags: DIFlagPublic, elements: !8, identifier: "inner")
    !8 = !{!9, !15, !17, !19, !23}
    !9 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !2, file: !2, line: 14, baseType: !10, size: 648, align: 8, flags: DIFlagPublic)
    !10 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !2, file: !2, baseType: !11, align: 8)
    !11 = !DICompositeType(tag: DW_TAG_array_type, baseType: !12, size: 648, align: 8, elements: !13)
    !12 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !13 = !{!14}
    !14 = !DISubrange(count: 81, lowerBound: 0)
    !15 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 15, baseType: !16, size: 8, align: 8, offset: 648, flags: DIFlagPublic)
    !16 = !DIBasicType(name: "BOOL", size: 8, encoding: DW_ATE_boolean, flags: DIFlagPublic)
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "r", scope: !2, file: !2, line: 16, baseType: !18, size: 32, align: 32, offset: 672, flags: DIFlagPublic)
    !18 = !DIBasicType(name: "REAL", size: 32, encoding: DW_ATE_float, flags: DIFlagPublic)
    !19 = !DIDerivedType(tag: DW_TAG_member, name: "arr", scope: !2, file: !2, line: 17, baseType: !20, size: 1944, align: 8, offset: 704, flags: DIFlagPublic)
    !20 = !DICompositeType(tag: DW_TAG_array_type, baseType: !10, size: 1944, align: 8, elements: !21)
    !21 = !{!22}
    !22 = !DISubrange(count: 3, lowerBound: 0)
    !23 = !DIDerivedType(tag: DW_TAG_member, name: "i", scope: !2, file: !2, line: 18, baseType: !24, size: 16, align: 16, offset: 2656, flags: DIFlagPublic)
    !24 = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "inner_arr", scope: !2, file: !2, line: 4, baseType: !26, size: 8064, align: 64, offset: 2688, flags: DIFlagPublic)
    !26 = !DICompositeType(tag: DW_TAG_array_type, baseType: !7, size: 8064, align: 64, elements: !21)
    !27 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !2, file: !2, line: 5, baseType: !10, size: 648, align: 8, offset: 10752, flags: DIFlagPublic)
    !28 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 6, baseType: !16, size: 8, align: 8, offset: 11400, flags: DIFlagPublic)
    !29 = !DIDerivedType(tag: DW_TAG_member, name: "r", scope: !2, file: !2, line: 7, baseType: !18, size: 32, align: 32, offset: 11424, flags: DIFlagPublic)
    !30 = !DIDerivedType(tag: DW_TAG_member, name: "arr", scope: !2, file: !2, line: 8, baseType: !20, size: 1944, align: 8, offset: 11456, flags: DIFlagPublic)
    !31 = !DIDerivedType(tag: DW_TAG_member, name: "i", scope: !2, file: !2, line: 9, baseType: !24, size: 16, align: 16, offset: 13408, flags: DIFlagPublic)
    !32 = !DIGlobalVariableExpression(var: !33, expr: !DIExpression())
    !33 = distinct !DIGlobalVariable(name: "__inner__init", scope: !2, file: !2, line: 13, type: !34, isLocal: false, isDefinition: true)
    !34 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !7)
    !35 = !{i32 2, !"Dwarf Version", i32 5}
    !36 = !{i32 2, !"Debug Info Version", i32 3}
    !37 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !38, splitDebugInlining: false)
    !38 = !{!0, !32}
    !39 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !2, file: !2, line: 22, type: !40, scopeLine: 22, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !37, retainedNodes: !42)
    !40 = !DISubroutineType(flags: DIFlagPublic, types: !41)
    !41 = !{null}
    !42 = !{}
    !43 = !DILocalVariable(name: "st", scope: !39, file: !2, line: 24, type: !4, align: 64)
    !44 = !DILocation(line: 24, column: 4, scope: !39)
    !45 = !DILocalVariable(name: "s", scope: !39, file: !2, line: 25, type: !10, align: 8)
    !46 = !DILocation(line: 25, column: 4, scope: !39)
    !47 = !DILocalVariable(name: "b", scope: !39, file: !2, line: 26, type: !16, align: 8)
    !48 = !DILocation(line: 26, column: 4, scope: !39)
    !49 = !DILocalVariable(name: "arr", scope: !39, file: !2, line: 27, type: !20, align: 8)
    !50 = !DILocation(line: 27, column: 4, scope: !39)
    !51 = !DILocalVariable(name: "i", scope: !39, file: !2, line: 28, type: !24, align: 16)
    !52 = !DILocation(line: 28, column: 4, scope: !39)
    !53 = !DILocation(line: 0, scope: !39)
    !54 = !DILocation(line: 32, column: 4, scope: !39)
    !55 = !DILocation(line: 33, column: 4, scope: !39)
    !56 = !DILocation(line: 34, column: 4, scope: !39)
    !57 = !DILocation(line: 35, column: 4, scope: !39)
    !58 = !DILocation(line: 36, column: 4, scope: !39)
    !59 = !DILocation(line: 37, column: 4, scope: !39)
    !60 = !DILocation(line: 38, column: 4, scope: !39)
    !61 = !DILocation(line: 39, column: 4, scope: !39)
    !62 = !DILocation(line: 41, column: 4, scope: !39)
    !63 = !DILocation(line: 42, column: 4, scope: !39)
    !64 = !DILocation(line: 43, column: 4, scope: !39)
    !65 = !DILocation(line: 45, scope: !39)
    "###);
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

    filtered_assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prog = type { i32, i32, i32 }
    %foo = type { i32 }

    @x = unnamed_addr constant i32 0, !dbg !0
    @s = unnamed_addr constant [81 x i8] zeroinitializer, !dbg !5
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @prog_instance = global %prog zeroinitializer, !dbg !13
    @__foo__init = unnamed_addr constant %foo zeroinitializer, !dbg !20
    @f = unnamed_addr constant %foo zeroinitializer, !dbg !26

    define void @prog(%prog* %0) !dbg !32 {
    entry:
      call void @llvm.dbg.declare(metadata %prog* %0, metadata !36, metadata !DIExpression()), !dbg !37
      %a = getelementptr inbounds %prog, %prog* %0, i32 0, i32 0
      %b = getelementptr inbounds %prog, %prog* %0, i32 0, i32 1
      %c = getelementptr inbounds %prog, %prog* %0, i32 0, i32 2
      ret void, !dbg !37
    }

    define i32 @bar() !dbg !38 {
    entry:
      %bar = alloca i32, align 4
      %d = alloca i32, align 4
      call void @llvm.dbg.declare(metadata i32* %d, metadata !41, metadata !DIExpression()), !dbg !42
      store i32 42, i32* %d, align 4
      call void @llvm.dbg.declare(metadata i32* %bar, metadata !43, metadata !DIExpression()), !dbg !44
      store i32 0, i32* %bar, align 4
      %bar_ret = load i32, i32* %bar, align 4, !dbg !45
      ret i32 %bar_ret, !dbg !45
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__user_init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      call void @__init_foo(%foo* @f)
      call void @__user_init_prog(%prog* @prog_instance)
      call void @__user_init_foo(%foo* @f)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!28, !29}
    !llvm.dbg.cu = !{!30}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "x", scope: !2, file: !2, line: 3, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !5 = !DIGlobalVariableExpression(var: !6, expr: !DIExpression())
    !6 = distinct !DIGlobalVariable(name: "s", scope: !2, file: !2, line: 4, type: !7, isLocal: false, isDefinition: true)
    !7 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !8)
    !8 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !2, file: !2, baseType: !9, align: 8)
    !9 = !DICompositeType(tag: DW_TAG_array_type, baseType: !10, size: 648, align: 8, elements: !11)
    !10 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !11 = !{!12}
    !12 = !DISubrange(count: 81, lowerBound: 0)
    !13 = !DIGlobalVariableExpression(var: !14, expr: !DIExpression())
    !14 = distinct !DIGlobalVariable(name: "prog", scope: !2, file: !2, line: 8, type: !15, isLocal: false, isDefinition: true)
    !15 = !DICompositeType(tag: DW_TAG_structure_type, name: "prog", scope: !2, file: !2, line: 8, size: 96, align: 64, flags: DIFlagPublic, elements: !16, identifier: "prog")
    !16 = !{!17, !18, !19}
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "a", scope: !2, file: !2, line: 10, baseType: !3, size: 32, align: 32, flags: DIFlagPublic)
    !18 = !DIDerivedType(tag: DW_TAG_member, name: "b", scope: !2, file: !2, line: 10, baseType: !3, size: 32, align: 32, offset: 32, flags: DIFlagPublic)
    !19 = !DIDerivedType(tag: DW_TAG_member, name: "c", scope: !2, file: !2, line: 10, baseType: !3, size: 32, align: 32, offset: 64, flags: DIFlagPublic)
    !20 = !DIGlobalVariableExpression(var: !21, expr: !DIExpression())
    !21 = distinct !DIGlobalVariable(name: "__foo__init", scope: !2, file: !2, line: 14, type: !22, isLocal: false, isDefinition: true)
    !22 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !23)
    !23 = !DICompositeType(tag: DW_TAG_structure_type, name: "foo", scope: !2, file: !2, line: 14, size: 32, align: 64, flags: DIFlagPublic, elements: !24, identifier: "foo")
    !24 = !{!25}
    !25 = !DIDerivedType(tag: DW_TAG_member, name: "z", scope: !2, file: !2, line: 15, baseType: !4, size: 32, align: 32, flags: DIFlagPublic)
    !26 = !DIGlobalVariableExpression(var: !27, expr: !DIExpression())
    !27 = distinct !DIGlobalVariable(name: "f", scope: !2, file: !2, line: 5, type: !22, isLocal: false, isDefinition: true)
    !28 = !{i32 2, !"Dwarf Version", i32 5}
    !29 = !{i32 2, !"Debug Info Version", i32 3}
    !30 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !31, splitDebugInlining: false)
    !31 = !{!0, !5, !26, !20, !13}
    !32 = distinct !DISubprogram(name: "prog", linkageName: "prog", scope: !2, file: !2, line: 8, type: !33, scopeLine: 12, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !30, retainedNodes: !35)
    !33 = !DISubroutineType(flags: DIFlagPublic, types: !34)
    !34 = !{null, !15}
    !35 = !{}
    !36 = !DILocalVariable(name: "prog", scope: !32, file: !2, line: 12, type: !15)
    !37 = !DILocation(line: 12, column: 8, scope: !32)
    !38 = distinct !DISubprogram(name: "bar", linkageName: "bar", scope: !2, file: !2, line: 19, type: !39, scopeLine: 23, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !30, retainedNodes: !35)
    !39 = !DISubroutineType(flags: DIFlagPublic, types: !40)
    !40 = !{null}
    !41 = !DILocalVariable(name: "d", scope: !38, file: !2, line: 21, type: !3, align: 32)
    !42 = !DILocation(line: 21, column: 12, scope: !38)
    !43 = !DILocalVariable(name: "bar", scope: !38, file: !2, line: 19, type: !4, align: 32)
    !44 = !DILocation(line: 19, column: 17, scope: !38)
    !45 = !DILocation(line: 23, column: 8, scope: !38)
    "###);
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

    %myStruct = type { i32, i8 }

    @basic_ptr = global i32* null, !dbg !0
    @array_ptr = global [11 x i32]* null, !dbg !6
    @struct_ptr = global %myStruct* null, !dbg !13
    @string_ptr = global [81 x i8]* null, !dbg !22
    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer, !dbg !31
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @__init_mystruct(%myStruct* %0) {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      ret void
    }

    define void @__user_init_myStruct(%myStruct* %0) {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }

    !llvm.module.flags = !{!34, !35}
    !llvm.dbg.cu = !{!36}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "basic_ptr", scope: !2, file: !2, line: 3, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____global_basic_ptr", scope: !2, file: !2, baseType: !4, align: 64)
    !4 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_basic_ptr", baseType: !5, size: 64, align: 64, dwarfAddressSpace: 1)
    !5 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !6 = !DIGlobalVariableExpression(var: !7, expr: !DIExpression())
    !7 = distinct !DIGlobalVariable(name: "array_ptr", scope: !2, file: !2, line: 4, type: !8, isLocal: false, isDefinition: true)
    !8 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____global_array_ptr", scope: !2, file: !2, baseType: !9, align: 64)
    !9 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_array_ptr", baseType: !10, size: 64, align: 64, dwarfAddressSpace: 1)
    !10 = !DICompositeType(tag: DW_TAG_array_type, baseType: !5, size: 352, align: 32, elements: !11)
    !11 = !{!12}
    !12 = !DISubrange(count: 11, lowerBound: 0)
    !13 = !DIGlobalVariableExpression(var: !14, expr: !DIExpression())
    !14 = distinct !DIGlobalVariable(name: "struct_ptr", scope: !2, file: !2, line: 5, type: !15, isLocal: false, isDefinition: true)
    !15 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REF_TO____global_struct_ptr", scope: !2, file: !2, baseType: !16, align: 64)
    !16 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_struct_ptr", baseType: !17, size: 64, align: 64, dwarfAddressSpace: 1)
    !17 = !DICompositeType(tag: DW_TAG_structure_type, name: "myStruct", scope: !2, file: !2, line: 9, size: 64, align: 64, flags: DIFlagPublic, elements: !18, identifier: "myStruct")
    !18 = !{!19, !20}
    !19 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 10, baseType: !5, size: 32, align: 32, flags: DIFlagPublic)
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 11, baseType: !21, size: 8, align: 8, offset: 32, flags: DIFlagPublic)
    !21 = !DIBasicType(name: "BOOL", size: 8, encoding: DW_ATE_boolean, flags: DIFlagPublic)
    !22 = !DIGlobalVariableExpression(var: !23, expr: !DIExpression())
    !23 = distinct !DIGlobalVariable(name: "string_ptr", scope: !2, file: !2, line: 6, type: !24, isLocal: false, isDefinition: true)
    !24 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REF_TO____global_string_ptr", scope: !2, file: !2, baseType: !25, align: 64)
    !25 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_string_ptr", baseType: !26, size: 64, align: 64, dwarfAddressSpace: 1)
    !26 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !2, file: !2, baseType: !27, align: 8)
    !27 = !DICompositeType(tag: DW_TAG_array_type, baseType: !28, size: 648, align: 8, elements: !29)
    !28 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !29 = !{!30}
    !30 = !DISubrange(count: 81, lowerBound: 0)
    !31 = !DIGlobalVariableExpression(var: !32, expr: !DIExpression())
    !32 = distinct !DIGlobalVariable(name: "__myStruct__init", scope: !2, file: !2, line: 9, type: !33, isLocal: false, isDefinition: true)
    !33 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !17)
    !34 = !{i32 2, !"Dwarf Version", i32 5}
    !35 = !{i32 2, !"Debug Info Version", i32 3}
    !36 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !37, splitDebugInlining: false)
    !37 = !{!0, !6, !13, !31, !22}
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

    %test_with_ref_params = type { [81 x i8]*, [6 x i32]*, i32*, %myStruct*, i32* }
    %myStruct = type { i32, i8 }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @test_with_ref_params_instance = global %test_with_ref_params zeroinitializer, !dbg !0
    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer, !dbg !34

    define void @test_with_ref_params(%test_with_ref_params* %0) !dbg !41 {
    entry:
      call void @llvm.dbg.declare(metadata %test_with_ref_params* %0, metadata !45, metadata !DIExpression()), !dbg !46
      %input_ref = getelementptr inbounds %test_with_ref_params, %test_with_ref_params* %0, i32 0, i32 0
      %array_ref = getelementptr inbounds %test_with_ref_params, %test_with_ref_params* %0, i32 0, i32 1
      %inout_value = getelementptr inbounds %test_with_ref_params, %test_with_ref_params* %0, i32 0, i32 2
      %inout_struct = getelementptr inbounds %test_with_ref_params, %test_with_ref_params* %0, i32 0, i32 3
      %local_ref = getelementptr inbounds %test_with_ref_params, %test_with_ref_params* %0, i32 0, i32 4
      ret void, !dbg !46
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init_mystruct(%myStruct* %0) {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      ret void
    }

    define void @__init_test_with_ref_params(%test_with_ref_params* %0) {
    entry:
      %self = alloca %test_with_ref_params*, align 8
      store %test_with_ref_params* %0, %test_with_ref_params** %self, align 8
      ret void
    }

    define void @__user_init_test_with_ref_params(%test_with_ref_params* %0) {
    entry:
      %self = alloca %test_with_ref_params*, align 8
      store %test_with_ref_params* %0, %test_with_ref_params** %self, align 8
      ret void
    }

    define void @__user_init_myStruct(%myStruct* %0) {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_test_with_ref_params(%test_with_ref_params* @test_with_ref_params_instance)
      call void @__user_init_test_with_ref_params(%test_with_ref_params* @test_with_ref_params_instance)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!37, !38}
    !llvm.dbg.cu = !{!39}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "test_with_ref_params", scope: !2, file: !2, line: 2, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "test_with_ref_params", scope: !2, file: !2, line: 2, size: 320, align: 64, flags: DIFlagPublic, elements: !4, identifier: "test_with_ref_params")
    !4 = !{!5, !13, !20, !23, !31}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "input_ref", scope: !2, file: !2, line: 4, baseType: !6, size: 64, align: 64, flags: DIFlagPublic)
    !6 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____auto_pointer_to_STRING", scope: !2, file: !2, baseType: !7, align: 64)
    !7 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_STRING", baseType: !8, size: 64, align: 64, dwarfAddressSpace: 1)
    !8 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !2, file: !2, baseType: !9, align: 8)
    !9 = !DICompositeType(tag: DW_TAG_array_type, baseType: !10, size: 648, align: 8, elements: !11)
    !10 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !11 = !{!12}
    !12 = !DISubrange(count: 81, lowerBound: 0)
    !13 = !DIDerivedType(tag: DW_TAG_member, name: "array_ref", scope: !2, file: !2, line: 5, baseType: !14, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !14 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____auto_pointer_to___test_with_ref_params_array_ref", scope: !2, file: !2, baseType: !15, align: 64)
    !15 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to___test_with_ref_params_array_ref", baseType: !16, size: 64, align: 64, dwarfAddressSpace: 1)
    !16 = !DICompositeType(tag: DW_TAG_array_type, baseType: !17, size: 192, align: 32, elements: !18)
    !17 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !18 = !{!19}
    !19 = !DISubrange(count: 6, lowerBound: 0)
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "inout_value", scope: !2, file: !2, line: 8, baseType: !21, size: 64, align: 64, offset: 128, flags: DIFlagPublic)
    !21 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____auto_pointer_to_DINT", scope: !2, file: !2, baseType: !22, align: 64)
    !22 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_DINT", baseType: !17, size: 64, align: 64, dwarfAddressSpace: 1)
    !23 = !DIDerivedType(tag: DW_TAG_member, name: "inout_struct", scope: !2, file: !2, line: 9, baseType: !24, size: 64, align: 64, offset: 192, flags: DIFlagPublic)
    !24 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____auto_pointer_to_myStruct", scope: !2, file: !2, baseType: !25, align: 64)
    !25 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_myStruct", baseType: !26, size: 64, align: 64, dwarfAddressSpace: 1)
    !26 = !DICompositeType(tag: DW_TAG_structure_type, name: "myStruct", scope: !2, file: !2, line: 16, size: 64, align: 64, flags: DIFlagPublic, elements: !27, identifier: "myStruct")
    !27 = !{!28, !29}
    !28 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 17, baseType: !17, size: 32, align: 32, flags: DIFlagPublic)
    !29 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 18, baseType: !30, size: 8, align: 8, offset: 32, flags: DIFlagPublic)
    !30 = !DIBasicType(name: "BOOL", size: 8, encoding: DW_ATE_boolean, flags: DIFlagPublic)
    !31 = !DIDerivedType(tag: DW_TAG_member, name: "local_ref", scope: !2, file: !2, line: 12, baseType: !32, size: 64, align: 64, offset: 256, flags: DIFlagPublic)
    !32 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REF_TO____test_with_ref_params_local_ref", scope: !2, file: !2, baseType: !33, align: 64)
    !33 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__test_with_ref_params_local_ref", baseType: !17, size: 64, align: 64, dwarfAddressSpace: 1)
    !34 = !DIGlobalVariableExpression(var: !35, expr: !DIExpression())
    !35 = distinct !DIGlobalVariable(name: "__myStruct__init", scope: !2, file: !2, line: 16, type: !36, isLocal: false, isDefinition: true)
    !36 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !26)
    !37 = !{i32 2, !"Dwarf Version", i32 5}
    !38 = !{i32 2, !"Debug Info Version", i32 3}
    !39 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !40, splitDebugInlining: false)
    !40 = !{!0, !34}
    !41 = distinct !DISubprogram(name: "test_with_ref_params", linkageName: "test_with_ref_params", scope: !2, file: !2, line: 2, type: !42, scopeLine: 14, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !39, retainedNodes: !44)
    !42 = !DISubroutineType(flags: DIFlagPublic, types: !43)
    !43 = !{null, !3, !6, !14, !21, !24}
    !44 = !{}
    !45 = !DILocalVariable(name: "test_with_ref_params", scope: !41, file: !2, line: 14, type: !3)
    !46 = !DILocation(line: 14, column: 4, scope: !41)
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
    filtered_assert_snapshot!(codegen, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %myStruct = type { i32, i8 }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer, !dbg !0
    @global_struct = global %myStruct zeroinitializer, !dbg !10
    @global_var = global i32 42, !dbg !12
    @alias_int = global i32* null, !dbg !14
    @alias_struct = global %myStruct* null, !dbg !18

    define void @__init_mystruct(%myStruct* %0) {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      ret void
    }

    define void @__user_init_myStruct(%myStruct* %0) {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_mystruct(%myStruct* @global_struct)
      store i32* @global_var, i32** @alias_int, align 8
      store %myStruct* @global_struct, %myStruct** @alias_struct, align 8
      call void @__user_init_myStruct(%myStruct* @global_struct)
      ret void
    }

    !llvm.module.flags = !{!22, !23}
    !llvm.dbg.cu = !{!24}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "__myStruct__init", scope: !2, file: !2, line: 10, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !4)
    !4 = !DICompositeType(tag: DW_TAG_structure_type, name: "myStruct", scope: !2, file: !2, line: 10, size: 64, align: 64, flags: DIFlagPublic, elements: !5, identifier: "myStruct")
    !5 = !{!6, !8}
    !6 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 11, baseType: !7, size: 32, align: 32, flags: DIFlagPublic)
    !7 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !8 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 12, baseType: !9, size: 8, align: 8, offset: 32, flags: DIFlagPublic)
    !9 = !DIBasicType(name: "BOOL", size: 8, encoding: DW_ATE_boolean, flags: DIFlagPublic)
    !10 = !DIGlobalVariableExpression(var: !11, expr: !DIExpression())
    !11 = distinct !DIGlobalVariable(name: "global_struct", scope: !2, file: !2, line: 6, type: !4, isLocal: false, isDefinition: true)
    !12 = !DIGlobalVariableExpression(var: !13, expr: !DIExpression())
    !13 = distinct !DIGlobalVariable(name: "global_var", scope: !2, file: !2, line: 3, type: !7, isLocal: false, isDefinition: true)
    !14 = !DIGlobalVariableExpression(var: !15, expr: !DIExpression())
    !15 = distinct !DIGlobalVariable(name: "alias_int", scope: !2, file: !2, line: 4, type: !16, isLocal: false, isDefinition: true)
    !16 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____global_alias_int", scope: !2, file: !2, baseType: !17, align: 64)
    !17 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_alias_int", baseType: !7, size: 64, align: 64, dwarfAddressSpace: 1)
    !18 = !DIGlobalVariableExpression(var: !19, expr: !DIExpression())
    !19 = distinct !DIGlobalVariable(name: "alias_struct", scope: !2, file: !2, line: 7, type: !20, isLocal: false, isDefinition: true)
    !20 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____global_alias_struct", scope: !2, file: !2, baseType: !21, align: 64)
    !21 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_alias_struct", baseType: !4, size: 64, align: 64, dwarfAddressSpace: 1)
    !22 = !{i32 2, !"Dwarf Version", i32 5}
    !23 = !{i32 2, !"Debug Info Version", i32 3}
    !24 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !25, splitDebugInlining: false)
    !25 = !{!12, !14, !10, !0, !18}
    "###)
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

    %mixed_ptr = type { [81 x i8]*, i32*, i8*, i8* }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @mixed_ptr_instance = global %mixed_ptr zeroinitializer, !dbg !0
    @regular_ptr = global i32* null, !dbg !24
    @alias_var = global i32** null, !dbg !28

    define void @mixed_ptr(%mixed_ptr* %0) !dbg !38 {
    entry:
      call void @llvm.dbg.declare(metadata %mixed_ptr* %0, metadata !42, metadata !DIExpression()), !dbg !43
      %ref_param = getelementptr inbounds %mixed_ptr, %mixed_ptr* %0, i32 0, i32 0
      %inout_param = getelementptr inbounds %mixed_ptr, %mixed_ptr* %0, i32 0, i32 1
      %local_ptr = getelementptr inbounds %mixed_ptr, %mixed_ptr* %0, i32 0, i32 2
      %local_ref = getelementptr inbounds %mixed_ptr, %mixed_ptr* %0, i32 0, i32 3
      ret void, !dbg !43
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init_mixed_ptr(%mixed_ptr* %0) {
    entry:
      %self = alloca %mixed_ptr*, align 8
      store %mixed_ptr* %0, %mixed_ptr** %self, align 8
      ret void
    }

    define void @__user_init_mixed_ptr(%mixed_ptr* %0) {
    entry:
      %self = alloca %mixed_ptr*, align 8
      store %mixed_ptr* %0, %mixed_ptr** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_mixed_ptr(%mixed_ptr* @mixed_ptr_instance)
      store i32** @regular_ptr, i32*** @alias_var, align 8
      call void @__user_init_mixed_ptr(%mixed_ptr* @mixed_ptr_instance)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!34, !35}
    !llvm.dbg.cu = !{!36}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "mixed_ptr", scope: !2, file: !2, line: 7, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DICompositeType(tag: DW_TAG_structure_type, name: "mixed_ptr", scope: !2, file: !2, line: 7, size: 256, align: 64, flags: DIFlagPublic, elements: !4, identifier: "mixed_ptr")
    !4 = !{!5, !13, !17, !21}
    !5 = !DIDerivedType(tag: DW_TAG_member, name: "ref_param", scope: !2, file: !2, line: 9, baseType: !6, size: 64, align: 64, flags: DIFlagPublic)
    !6 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____auto_pointer_to_STRING", scope: !2, file: !2, baseType: !7, align: 64)
    !7 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_STRING", baseType: !8, size: 64, align: 64, dwarfAddressSpace: 1)
    !8 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !2, file: !2, baseType: !9, align: 8)
    !9 = !DICompositeType(tag: DW_TAG_array_type, baseType: !10, size: 648, align: 8, elements: !11)
    !10 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !11 = !{!12}
    !12 = !DISubrange(count: 81, lowerBound: 0)
    !13 = !DIDerivedType(tag: DW_TAG_member, name: "inout_param", scope: !2, file: !2, line: 12, baseType: !14, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !14 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____auto_pointer_to_DINT", scope: !2, file: !2, baseType: !15, align: 64)
    !15 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__auto_pointer_to_DINT", baseType: !16, size: 64, align: 64, dwarfAddressSpace: 1)
    !16 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !17 = !DIDerivedType(tag: DW_TAG_member, name: "local_ptr", scope: !2, file: !2, line: 15, baseType: !18, size: 64, align: 64, offset: 128, flags: DIFlagPublic)
    !18 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____mixed_ptr_local_ptr", scope: !2, file: !2, baseType: !19, align: 64)
    !19 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__mixed_ptr_local_ptr", baseType: !20, size: 64, align: 64, dwarfAddressSpace: 1)
    !20 = !DIBasicType(name: "BOOL", size: 8, encoding: DW_ATE_boolean, flags: DIFlagPublic)
    !21 = !DIDerivedType(tag: DW_TAG_member, name: "local_ref", scope: !2, file: !2, line: 16, baseType: !22, size: 64, align: 64, offset: 192, flags: DIFlagPublic)
    !22 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REF_TO____mixed_ptr_local_ref", scope: !2, file: !2, baseType: !23, align: 64)
    !23 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__mixed_ptr_local_ref", baseType: !20, size: 64, align: 64, dwarfAddressSpace: 1)
    !24 = !DIGlobalVariableExpression(var: !25, expr: !DIExpression())
    !25 = distinct !DIGlobalVariable(name: "regular_ptr", scope: !2, file: !2, line: 3, type: !26, isLocal: false, isDefinition: true)
    !26 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____global_regular_ptr", scope: !2, file: !2, baseType: !27, align: 64)
    !27 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_regular_ptr", baseType: !16, size: 64, align: 64, dwarfAddressSpace: 1)
    !28 = !DIGlobalVariableExpression(var: !29, expr: !DIExpression())
    !29 = distinct !DIGlobalVariable(name: "alias_var", scope: !2, file: !2, line: 4, type: !30, isLocal: false, isDefinition: true)
    !30 = !DIDerivedType(tag: DW_TAG_typedef, name: "__AUTO_DEREF____global_alias_var", scope: !2, file: !2, baseType: !31, align: 64)
    !31 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_alias_var", baseType: !32, size: 64, align: 64, dwarfAddressSpace: 1)
    !32 = !DIDerivedType(tag: DW_TAG_typedef, name: "__POINTER_TO____global_alias_var_", scope: !2, file: !2, baseType: !33, align: 64)
    !33 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_alias_var_", baseType: !16, size: 64, align: 64, dwarfAddressSpace: 1)
    !34 = !{i32 2, !"Dwarf Version", i32 5}
    !35 = !{i32 2, !"Debug Info Version", i32 3}
    !36 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !37, splitDebugInlining: false)
    !37 = !{!24, !28, !0}
    !38 = distinct !DISubprogram(name: "mixed_ptr", linkageName: "mixed_ptr", scope: !2, file: !2, line: 7, type: !39, scopeLine: 18, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !36, retainedNodes: !41)
    !39 = !DISubroutineType(flags: DIFlagPublic, types: !40)
    !40 = !{null, !3, !6, !14}
    !41 = !{}
    !42 = !DILocalVariable(name: "mixed_ptr", scope: !38, file: !2, line: 18, type: !3)
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

    %myStruct = type { i32, i8 }
    %test_with_reference_params = type { i32*, [6 x i8]*, %myStruct* }

    @basic_reference = global i32* null, !dbg !0
    @array_reference = global [11 x i32]* null, !dbg !6
    @struct_reference = global %myStruct* null, !dbg !13
    @string_reference = global [81 x i8]* null, !dbg !22
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @test_with_reference_params_instance = global %test_with_reference_params zeroinitializer, !dbg !31
    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer, !dbg !47

    define void @test_with_reference_params(%test_with_reference_params* %0) !dbg !54 {
    entry:
      call void @llvm.dbg.declare(metadata %test_with_reference_params* %0, metadata !58, metadata !DIExpression()), !dbg !59
      %ref_param = getelementptr inbounds %test_with_reference_params, %test_with_reference_params* %0, i32 0, i32 0
      %array_ref_param = getelementptr inbounds %test_with_reference_params, %test_with_reference_params* %0, i32 0, i32 1
      %local_reference = getelementptr inbounds %test_with_reference_params, %test_with_reference_params* %0, i32 0, i32 2
      ret void, !dbg !59
    }

    ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
    declare void @llvm.dbg.declare(metadata, metadata, metadata) #0

    define void @__init_mystruct(%myStruct* %0) {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      ret void
    }

    define void @__init_test_with_reference_params(%test_with_reference_params* %0) {
    entry:
      %self = alloca %test_with_reference_params*, align 8
      store %test_with_reference_params* %0, %test_with_reference_params** %self, align 8
      ret void
    }

    define void @__user_init_myStruct(%myStruct* %0) {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      ret void
    }

    define void @__user_init_test_with_reference_params(%test_with_reference_params* %0) {
    entry:
      %self = alloca %test_with_reference_params*, align 8
      store %test_with_reference_params* %0, %test_with_reference_params** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_test_with_reference_params(%test_with_reference_params* @test_with_reference_params_instance)
      call void @__user_init_test_with_reference_params(%test_with_reference_params* @test_with_reference_params_instance)
      ret void
    }

    attributes #0 = { nofree nosync nounwind readnone speculatable willreturn }

    !llvm.module.flags = !{!50, !51}
    !llvm.dbg.cu = !{!52}

    !0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
    !1 = distinct !DIGlobalVariable(name: "basic_reference", scope: !2, file: !2, line: 3, type: !3, isLocal: false, isDefinition: true)
    !2 = !DIFile(filename: "<internal>", directory: "")
    !3 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REFERENCE_TO____global_basic_reference", scope: !2, file: !2, baseType: !4, align: 64)
    !4 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_basic_reference", baseType: !5, size: 64, align: 64, dwarfAddressSpace: 1)
    !5 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !6 = !DIGlobalVariableExpression(var: !7, expr: !DIExpression())
    !7 = distinct !DIGlobalVariable(name: "array_reference", scope: !2, file: !2, line: 4, type: !8, isLocal: false, isDefinition: true)
    !8 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REFERENCE_TO____global_array_reference", scope: !2, file: !2, baseType: !9, align: 64)
    !9 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_array_reference", baseType: !10, size: 64, align: 64, dwarfAddressSpace: 1)
    !10 = !DICompositeType(tag: DW_TAG_array_type, baseType: !5, size: 352, align: 32, elements: !11)
    !11 = !{!12}
    !12 = !DISubrange(count: 11, lowerBound: 0)
    !13 = !DIGlobalVariableExpression(var: !14, expr: !DIExpression())
    !14 = distinct !DIGlobalVariable(name: "struct_reference", scope: !2, file: !2, line: 5, type: !15, isLocal: false, isDefinition: true)
    !15 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REFERENCE_TO____global_struct_reference", scope: !2, file: !2, baseType: !16, align: 64)
    !16 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_struct_reference", baseType: !17, size: 64, align: 64, dwarfAddressSpace: 1)
    !17 = !DICompositeType(tag: DW_TAG_structure_type, name: "myStruct", scope: !2, file: !2, line: 19, size: 64, align: 64, flags: DIFlagPublic, elements: !18, identifier: "myStruct")
    !18 = !{!19, !20}
    !19 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !2, file: !2, line: 20, baseType: !5, size: 32, align: 32, flags: DIFlagPublic)
    !20 = !DIDerivedType(tag: DW_TAG_member, name: "y", scope: !2, file: !2, line: 21, baseType: !21, size: 8, align: 8, offset: 32, flags: DIFlagPublic)
    !21 = !DIBasicType(name: "BOOL", size: 8, encoding: DW_ATE_boolean, flags: DIFlagPublic)
    !22 = !DIGlobalVariableExpression(var: !23, expr: !DIExpression())
    !23 = distinct !DIGlobalVariable(name: "string_reference", scope: !2, file: !2, line: 6, type: !24, isLocal: false, isDefinition: true)
    !24 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REFERENCE_TO____global_string_reference", scope: !2, file: !2, baseType: !25, align: 64)
    !25 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__global_string_reference", baseType: !26, size: 64, align: 64, dwarfAddressSpace: 1)
    !26 = !DIDerivedType(tag: DW_TAG_typedef, name: "__STRING__81", scope: !2, file: !2, baseType: !27, align: 8)
    !27 = !DICompositeType(tag: DW_TAG_array_type, baseType: !28, size: 648, align: 8, elements: !29)
    !28 = !DIBasicType(name: "CHAR", size: 8, encoding: DW_ATE_UTF, flags: DIFlagPublic)
    !29 = !{!30}
    !30 = !DISubrange(count: 81, lowerBound: 0)
    !31 = !DIGlobalVariableExpression(var: !32, expr: !DIExpression())
    !32 = distinct !DIGlobalVariable(name: "test_with_reference_params", scope: !2, file: !2, line: 9, type: !33, isLocal: false, isDefinition: true)
    !33 = !DICompositeType(tag: DW_TAG_structure_type, name: "test_with_reference_params", scope: !2, file: !2, line: 9, size: 192, align: 64, flags: DIFlagPublic, elements: !34, identifier: "test_with_reference_params")
    !34 = !{!35, !38, !44}
    !35 = !DIDerivedType(tag: DW_TAG_member, name: "ref_param", scope: !2, file: !2, line: 11, baseType: !36, size: 64, align: 64, flags: DIFlagPublic)
    !36 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REFERENCE_TO____test_with_reference_params_ref_param", scope: !2, file: !2, baseType: !37, align: 64)
    !37 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__test_with_reference_params_ref_param", baseType: !5, size: 64, align: 64, dwarfAddressSpace: 1)
    !38 = !DIDerivedType(tag: DW_TAG_member, name: "array_ref_param", scope: !2, file: !2, line: 12, baseType: !39, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
    !39 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REFERENCE_TO____test_with_reference_params_array_ref_param", scope: !2, file: !2, baseType: !40, align: 64)
    !40 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__test_with_reference_params_array_ref_param", baseType: !41, size: 64, align: 64, dwarfAddressSpace: 1)
    !41 = !DICompositeType(tag: DW_TAG_array_type, baseType: !21, size: 48, align: 8, elements: !42)
    !42 = !{!43}
    !43 = !DISubrange(count: 6, lowerBound: 0)
    !44 = !DIDerivedType(tag: DW_TAG_member, name: "local_reference", scope: !2, file: !2, line: 15, baseType: !45, size: 64, align: 64, offset: 128, flags: DIFlagPublic)
    !45 = !DIDerivedType(tag: DW_TAG_typedef, name: "__REFERENCE_TO____test_with_reference_params_local_reference", scope: !2, file: !2, baseType: !46, align: 64)
    !46 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "__test_with_reference_params_local_reference", baseType: !17, size: 64, align: 64, dwarfAddressSpace: 1)
    !47 = !DIGlobalVariableExpression(var: !48, expr: !DIExpression())
    !48 = distinct !DIGlobalVariable(name: "__myStruct__init", scope: !2, file: !2, line: 19, type: !49, isLocal: false, isDefinition: true)
    !49 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !17)
    !50 = !{i32 2, !"Dwarf Version", i32 5}
    !51 = !{i32 2, !"Debug Info Version", i32 3}
    !52 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "RuSTy Structured text Compiler", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !53, splitDebugInlining: false)
    !53 = !{!0, !6, !13, !47, !22, !31}
    !54 = distinct !DISubprogram(name: "test_with_reference_params", linkageName: "test_with_reference_params", scope: !2, file: !2, line: 9, type: !55, scopeLine: 17, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !52, retainedNodes: !57)
    !55 = !DISubroutineType(flags: DIFlagPublic, types: !56)
    !56 = !{null, !33, !36, !39}
    !57 = !{}
    !58 = !DILocalVariable(name: "test_with_reference_params", scope: !54, file: !2, line: 17, type: !33)
    !59 = !DILocation(line: 17, column: 4, scope: !54)
    "#)
}

#[test]
fn range_datatype_debug() {
    let codegen = codegen(
        r#"
        TYPE RangeType :
            DINT(0..100)  := 0;
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

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define i32 @main() !dbg !4 {
    entry:
      %main = alloca i32, align 4
      %r = alloca i32, align 4
      call void @llvm.dbg.declare(metadata i32* %r, metadata !8, metadata !DIExpression()), !dbg !11
      store i32 0, i32* %r, align 4
      call void @llvm.dbg.declare(metadata i32* %main, metadata !12, metadata !DIExpression()), !dbg !13
      store i32 0, i32* %main, align 4
      store i32 50, i32* %r, align 4, !dbg !14
      %load_r = load i32, i32* %r, align 4, !dbg !15
      store i32 %load_r, i32* %main, align 4, !dbg !15
      %main_ret = load i32, i32* %main, align 4, !dbg !16
      ret i32 %main_ret, !dbg !16
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
    !4 = distinct !DISubprogram(name: "main", linkageName: "main", scope: !3, file: !3, line: 6, type: !5, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !7)
    !5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
    !6 = !{null}
    !7 = !{}
    !8 = !DILocalVariable(name: "r", scope: !4, file: !3, line: 8, type: !9, align: 32)
    !9 = !DIDerivedType(tag: DW_TAG_typedef, name: "RangeType", scope: !3, file: !3, line: 2, baseType: !10, align: 32)
    !10 = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed, flags: DIFlagPublic)
    !11 = !DILocation(line: 8, column: 12, scope: !4)
    !12 = !DILocalVariable(name: "main", scope: !4, file: !3, line: 6, type: !10, align: 32)
    !13 = !DILocation(line: 6, column: 17, scope: !4)
    !14 = !DILocation(line: 10, column: 12, scope: !4)
    !15 = !DILocation(line: 11, column: 12, scope: !4)
    !16 = !DILocation(line: 12, column: 8, scope: !4)
    "#)
}
