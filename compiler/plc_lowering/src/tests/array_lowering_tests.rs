use insta::assert_snapshot;
use plc_driver::parse_and_annotate;
use plc_source::SourceCode;

/// Helper: parse + annotate (runs the full pipeline including array lowering)
fn lower(src: &str) -> plc_driver::pipelines::AnnotatedProject {
    let src: SourceCode = src.into();
    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    project
}

/// Finds an implementation by name and returns its statements as pseudo-ST.
fn lowered_statements(project: &plc_driver::pipelines::AnnotatedProject, name: &str) -> String {
    for unit in &project.units {
        for imp in &unit.get_unit().implementations {
            if imp.name == name {
                return imp.statements.iter().map(|s| s.as_string()).collect::<Vec<_>>().join("\n");
            }
        }
    }
    panic!("Implementation '{name}' not found");
}

// ═══════════════════════════════════════════════════════════════════════════
// Guard: constant arrays are NOT lowered (handled at codegen via memcpy)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn constant_int_array_is_not_lowered() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR arr : ARRAY[0..4] OF DINT := [5(42)]; END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    __main_arr__ctor(arr)
    arr := 
    main := 0
    "#);
}

#[test]
fn constant_expression_list_is_not_lowered() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR arr : ARRAY[0..2] OF DINT := [10, 20, 30]; END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    __main_arr__ctor(arr)
    arr := 
    main := 0
    "#);
}

#[test]
fn const_variable_as_multiplier_is_rewritten_and_not_lowered() {
    let project = lower(
        "
        VAR_GLOBAL CONSTANT
            NB_BOOL : DINT := 12;
        END_VAR

        FUNCTION main : DINT
        VAR
            MAX_TIME_BOOL : ARRAY [1..NB_BOOL] OF REAL := [(NB_BOOL)(0.0033)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    __main_MAX_TIME_BOOL__ctor(MAX_TIME_BOOL)
    MAX_TIME_BOOL := 
    main := 0
    "#);
}

#[test]
fn const_variable_as_multiplier_in_global_is_rewritten() {
    let project = lower(
        "
        VAR_GLOBAL CONSTANT
            NB_BOOL : DINT := 12;
            MAX_TIME_BOOL : ARRAY [1..NB_BOOL] OF REAL := [(NB_BOOL)(0.0033)];
        END_VAR

        FUNCTION main : DINT
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @"main := 0");
}

#[test]
fn const_variable_as_multiplier_with_non_constant_element_is_lowered() {
    let project = lower(
        "
        VAR_GLOBAL CONSTANT
            N : DINT := 3;
        END_VAR

        FUNCTION main : DINT
        VAR
            x : DINT;
            arr : ARRAY[0..2] OF REF_TO DINT := [(N)(ADR(x))];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    __main_arr__ctor(arr)
    arr[0] := ADR(x)
    arr[1] := ADR(x)
    arr[2] := ADR(x)
    main := 0
    "#);
}

#[test]
fn local_const_variable_as_multiplier_is_rewritten_and_not_lowered() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR CONSTANT
            N : DINT := 5;
        END_VAR
        VAR
            arr : ARRAY [0..4] OF DINT := [(N)(42)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    __main_arr__ctor(arr)
    arr := 
    main := 0
    "#);
}

#[test]
fn local_const_variable_as_multiplier_with_non_constant_element_is_lowered() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR CONSTANT
            N : DINT := 3;
        END_VAR
        VAR
            x : DINT;
            arr : ARRAY[0..2] OF REF_TO DINT := [(N)(ADR(x))];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    __main_arr__ctor(arr)
    arr[0] := ADR(x)
    arr[1] := ADR(x)
    arr[2] := ADR(x)
    main := 0
    "#);
}

// ═══════════════════════════════════════════════════════════════════════════
// Non-constant variable multipliers — lowered to FOR loops
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn non_constant_variable_multiplier_is_lowered_to_for_loop() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            n : DINT := 5;
            arr : ARRAY[0..4] OF DINT := [(n)(42)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    n := 5
    __main_arr__ctor(arr)
    FOR __literal_idx := 0 + 0 TO 0 + 0 + n - 1 DO
        arr[__literal_idx] := 42
    END_FOR
    main := 0
    "#);
}

#[test]
fn non_constant_variable_multiplier_mixed_segments_lowered() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            n : DINT := 3;
            arr : ARRAY[0..4] OF DINT := [(n)(10), 20, 30];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    n := 3
    __main_arr__ctor(arr)
    FOR __literal_idx := 0 + 0 TO 0 + 0 + n - 1 DO
        arr[__literal_idx] := 10
    END_FOR
    arr[0 + 0 + n] := 20
    arr[0 + 0 + n + 1] := 30
    main := 0
    "#);
}

// ═══════════════════════════════════════════════════════════════════════════
// Function calls (ADR, etc.) — runtime values, must be lowered
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn small_adr_array_is_unrolled() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            x : DINT;
            arr : ARRAY[0..2] OF REF_TO DINT := [3(ADR(x))];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    __main_arr__ctor(arr)
    arr[0] := ADR(x)
    arr[1] := ADR(x)
    arr[2] := ADR(x)
    main := 0
    "#);
}

#[test]
fn large_adr_array_uses_for_loop() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            x : DINT;
            arr : ARRAY[0..99] OF REF_TO DINT := [100(ADR(x))];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    __main_arr__ctor(arr)
    FOR __literal_idx := 0 TO 99 DO
        arr[__literal_idx] := ADR(x)
    END_FOR
    main := 0
    "#);
}

#[test]
fn adr_expression_list_is_lowered() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            a : DINT;
            b : DINT;
            c : DINT;
            arr : ARRAY[0..2] OF REF_TO DINT := [ADR(a), ADR(b), ADR(c)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    __main_arr__ctor(arr)
    arr[0] := ADR(a)
    arr[1] := ADR(b)
    arr[2] := ADR(c)
    main := 0
    "#);
}

#[test]
fn fb_member_array_non_const_initializer_is_lowered() {
    let project = lower(
        "
        FUNCTION_BLOCK Foo
        VAR
            x : DINT;
            arr : ARRAY[0..2] OF REF_TO DINT := [3(ADR(x))];
        END_VAR
        END_FUNCTION_BLOCK
        ",
    );
    assert_snapshot!(lowered_statements(&project, "Foo__ctor"), @r#"
    __Foo___vtable__ctor(self.__vtable)
    __Foo_arr__ctor(self.arr)
    self.arr[0] := ADR(x)
    self.arr[1] := ADR(x)
    self.arr[2] := ADR(x)
    self.__vtable := ADR(__vtable_Foo_instance)
    "#);
}

// ═══════════════════════════════════════════════════════════════════════════
// Struct literals — constant data but codegen can't handle them in arrays yet
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn small_struct_array_is_unrolled() {
    let project = lower(
        "
        TYPE MyStruct : STRUCT a : DINT; b : DINT; END_STRUCT END_TYPE

        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..2] OF MyStruct := [3((a := 5, b := 10))];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    __main_arr__ctor(arr)
    arr[0] := (a := 5, b := 10)
    arr[1] := (a := 5, b := 10)
    arr[2] := (a := 5, b := 10)
    main := 0
    "#);
}

#[test]
fn large_struct_array_uses_for_loop() {
    let project = lower(
        "
        TYPE MyStruct : STRUCT a : DINT; END_STRUCT END_TYPE

        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..99] OF MyStruct := [100((a := 7))];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    __main_arr__ctor(arr)
    FOR __literal_idx := 0 TO 99 DO
        arr[__literal_idx] := (a := 7)
    END_FOR
    main := 0
    "#);
}

#[test]
fn struct_expression_list_is_lowered() {
    let project = lower(
        "
        TYPE MyStruct : STRUCT a : DINT; b : DINT; END_STRUCT END_TYPE

        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..1] OF MyStruct := [(a := 1, b := 2), (a := 3, b := 4)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    __main_arr__ctor(arr)
    arr[0] := (a := 1, b := 2)
    arr[1] := (a := 3, b := 4)
    main := 0
    "#);
}

// ═══════════════════════════════════════════════════════════════════════════
// Variable references — runtime values, must be lowered
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn variable_as_element_value_is_lowered() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            a : DINT := 42;
            arr : ARRAY[0..2] OF DINT := [3(a)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    a := 42
    __main_arr__ctor(arr)
    arr[0] := a
    arr[1] := a
    arr[2] := a
    main := 0
    "#);
}

#[test]
fn variable_elements_in_expression_list_are_lowered() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            a : DINT := 42;
            arr : ARRAY[0..2] OF DINT := [a, a, a];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    a := 42
    __main_arr__ctor(arr)
    arr[0] := a
    arr[1] := a
    arr[2] := a
    main := 0
    "#);
}

#[test]
fn shared_type_non_const_is_lowered_const_is_not() {
    let project = lower(
        "
        TYPE tarr : ARRAY[0..2] OF DINT; END_TYPE

        FUNCTION main : DINT
        VAR
            seed : DINT := 42;
            lowered_arr : tarr := [3(seed)];
            const_arr : tarr := [3(7)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    seed := 42
    tarr__ctor(lowered_arr)
    lowered_arr[0] := seed
    lowered_arr[1] := seed
    lowered_arr[2] := seed
    tarr__ctor(const_arr)
    const_arr := 
    main := 0
    "#);
}

// ═══════════════════════════════════════════════════════════════════════════
// Mixed segments
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn mixed_adr_segments_are_lowered() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            x : DINT;
            y : DINT;
            arr : ARRAY[0..4] OF REF_TO DINT := [2(ADR(x)), ADR(y), 2(ADR(x))];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    __main_arr__ctor(arr)
    arr[0] := ADR(x)
    arr[1] := ADR(x)
    arr[2] := ADR(y)
    arr[3] := ADR(x)
    arr[4] := ADR(x)
    main := 0
    "#);
}

// ═══════════════════════════════════════════════════════════════════════════
// Non-zero offset
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn nonzero_offset_adr_array_is_lowered() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            x : DINT;
            arr : ARRAY[5..7] OF REF_TO DINT := [3(ADR(x))];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    __main_arr__ctor(arr)
    arr[5] := ADR(x)
    arr[6] := ADR(x)
    arr[7] := ADR(x)
    main := 0
    "#);
}

// ═══════════════════════════════════════════════════════════════════════════
// Multi-dimensional with non-constant expressions
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn multi_dim_adr_array_is_lowered() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            x : DINT;
            arr : ARRAY[0..1, 0..2] OF REF_TO DINT := [6(ADR(x))];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    __main_arr__ctor(arr)
    arr[0, 0] := ADR(x)
    arr[0, 1] := ADR(x)
    arr[0, 2] := ADR(x)
    arr[1, 0] := ADR(x)
    arr[1, 1] := ADR(x)
    arr[1, 2] := ADR(x)
    main := 0
    "#);
}

// ═══════════════════════════════════════════════════════════════════════════
// Type-level with non-constant expressions
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn type_level_struct_array_ctor_is_lowered() {
    let project = lower(
        "
        TYPE MyStruct : STRUCT a : DINT; END_STRUCT END_TYPE
        TYPE tarr : ARRAY[0..2] OF MyStruct := [3((a := 42))]; END_TYPE

        FUNCTION main : DINT
        VAR arr : tarr; END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "tarr__ctor"), @r#"
    self[0] := (a := 42)
    self[1] := (a := 42)
    self[2] := (a := 42)
    "#);
}

// ═══════════════════════════════════════════════════════════════════════════
// flat_to_indices unit tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn flat_to_indices_2d_zero_based() {
    use crate::array_lowering::{ArrayInfo, DimInfo};
    let info = ArrayInfo { dims: vec![DimInfo { start: 0, size: 3 }, DimInfo { start: 0, size: 3 }] };
    assert_eq!(info.flat_to_indices(0), vec![0, 0]);
    assert_eq!(info.flat_to_indices(1), vec![0, 1]);
    assert_eq!(info.flat_to_indices(2), vec![0, 2]);
    assert_eq!(info.flat_to_indices(3), vec![1, 0]);
    assert_eq!(info.flat_to_indices(5), vec![1, 2]);
    assert_eq!(info.flat_to_indices(8), vec![2, 2]);
}

#[test]
fn flat_to_indices_2d_nonzero_start() {
    use crate::array_lowering::{ArrayInfo, DimInfo};
    let info = ArrayInfo { dims: vec![DimInfo { start: 1, size: 3 }, DimInfo { start: 5, size: 3 }] };
    assert_eq!(info.flat_to_indices(0), vec![1, 5]);
    assert_eq!(info.flat_to_indices(1), vec![1, 6]);
    assert_eq!(info.flat_to_indices(3), vec![2, 5]);
    assert_eq!(info.flat_to_indices(8), vec![3, 7]);
}

#[test]
fn flat_to_indices_3d() {
    use crate::array_lowering::{ArrayInfo, DimInfo};
    let info = ArrayInfo {
        dims: vec![
            DimInfo { start: 0, size: 3 },
            DimInfo { start: 0, size: 3 },
            DimInfo { start: 0, size: 3 },
        ],
    };
    assert_eq!(info.flat_to_indices(0), vec![0, 0, 0]);
    assert_eq!(info.flat_to_indices(1), vec![0, 0, 1]);
    assert_eq!(info.flat_to_indices(2), vec![0, 0, 2]);
    assert_eq!(info.flat_to_indices(3), vec![0, 1, 0]);
    assert_eq!(info.flat_to_indices(9), vec![1, 0, 0]);
    assert_eq!(info.flat_to_indices(26), vec![2, 2, 2]);
}

/// `FOR_LOOP_THRESHOLD` (32) is checked per `MultipliedStatement`
/// segment, not against the total element count. An initializer split into
/// multiple sub-threshold segments — e.g. `[10(v), 10(v), 10(v), 10(v)]`
/// totalling 40 elements — is fully unrolled into 40 individual assignments
/// instead of emitting a FOR loop, because each segment's count (10) is below
/// the threshold. This documents the current behaviour so that
/// any future fix is intentional.
#[test]
fn multi_segment_above_threshold_is_unrolled_per_segment() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            v : DINT := 1;
            arr : ARRAY[0..39] OF DINT := [10(v), 10(v), 10(v), 10(v)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    assert_snapshot!(lowered_statements(&project, "main"), @r#"
    v := 1
    __main_arr__ctor(arr)
    arr[0] := v
    arr[1] := v
    arr[2] := v
    arr[3] := v
    arr[4] := v
    arr[5] := v
    arr[6] := v
    arr[7] := v
    arr[8] := v
    arr[9] := v
    arr[10] := v
    arr[11] := v
    arr[12] := v
    arr[13] := v
    arr[14] := v
    arr[15] := v
    arr[16] := v
    arr[17] := v
    arr[18] := v
    arr[19] := v
    arr[20] := v
    arr[21] := v
    arr[22] := v
    arr[23] := v
    arr[24] := v
    arr[25] := v
    arr[26] := v
    arr[27] := v
    arr[28] := v
    arr[29] := v
    arr[30] := v
    arr[31] := v
    arr[32] := v
    arr[33] := v
    arr[34] := v
    arr[35] := v
    arr[36] := v
    arr[37] := v
    arr[38] := v
    arr[39] := v
    main := 0
    "#);
}
