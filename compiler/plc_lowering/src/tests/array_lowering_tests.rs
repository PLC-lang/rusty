use plc_ast::{ast::AstStatement, control_statements::AstControlStatement};
use plc_driver::parse_and_annotate;
use plc_source::SourceCode;

/// Helper: parse + annotate (runs the full pipeline including array lowering)
fn lower(src: &str) -> plc_driver::pipelines::AnnotatedProject {
    let src: SourceCode = src.into();
    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    project
}

/// Finds an implementation by name in the project's annotated units.
fn find_impl_stmts<'a>(
    project: &'a plc_driver::pipelines::AnnotatedProject,
    name: &str,
) -> &'a [plc_ast::ast::AstNode] {
    for unit in &project.units {
        for imp in &unit.get_unit().implementations {
            if imp.name == name {
                return &imp.statements;
            }
        }
    }
    panic!("Implementation '{name}' not found");
}

/// Returns the number of assignments in a statement list.
fn count_assignments(stmts: &[plc_ast::ast::AstNode]) -> usize {
    stmts.iter().filter(|s| matches!(s.get_stmt(), AstStatement::Assignment(..))).count()
}

/// Returns true if any statement is a FOR loop.
fn has_for_loop(stmts: &[plc_ast::ast::AstNode]) -> bool {
    stmts
        .iter()
        .any(|s| matches!(s.get_stmt(), AstStatement::ControlStatement(AstControlStatement::ForLoop(..))))
}

/// Returns true if any statement is a WHILE loop.
fn has_while_loop(stmts: &[plc_ast::ast::AstNode]) -> bool {
    stmts
        .iter()
        .any(|s| matches!(s.get_stmt(), AstStatement::ControlStatement(AstControlStatement::WhileLoop(..))))
}

/// Returns true if any top-level assignment has a `LiteralArray` on the RHS.
fn has_literal_array(stmts: &[plc_ast::ast::AstNode]) -> bool {
    stmts.iter().any(|s| {
        if let AstStatement::Assignment(data) = s.get_stmt() {
            matches!(data.right.get_stmt(), AstStatement::Literal(plc_ast::literals::AstLiteral::Array(..)))
        } else {
            false
        }
    })
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(has_literal_array(stmts), "Constant array should NOT be lowered");
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(has_literal_array(stmts), "Constant expression list array should NOT be lowered");
}

#[test]
fn const_variable_as_multiplier_is_rewritten_and_not_lowered() {
    // `[(NB_BOOL)(0.0033)]` is parsed as a CallStatement but should be rewritten
    // into a MultipliedStatement when NB_BOOL is a constant integer. Since all
    // elements are constant REALs, the result should NOT be lowered further.
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(has_literal_array(stmts), "Constant multiplied array should NOT be lowered");
}

#[test]
fn const_variable_as_multiplier_in_global_is_rewritten() {
    // Same rewrite for global constant arrays with `[(CONST)(value)]` syntax.
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
    let stmts = find_impl_stmts(&project, "main");
    assert_eq!(count_assignments(stmts), 1);
}

#[test]
fn const_variable_as_multiplier_with_non_constant_element_is_lowered() {
    // `[(N)(ADR(x))]` — the multiplier is a constant but the element is a runtime
    // value (function call).  The CallStatement should be rewritten to a
    // MultipliedStatement, then lowered into individual assignments.
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts), "Non-constant elements should be lowered");
    assert_eq!(count_assignments(stmts), 3 + 1); // 3 element assignments + return assignment
}

#[test]
fn local_const_variable_as_multiplier_is_rewritten_and_not_lowered() {
    // `[(N)(42)]` where N is a POU-local `VAR CONSTANT` should be rewritten
    // into a MultipliedStatement.  Since all elements are constant, the result
    // should NOT be lowered further.
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(has_literal_array(stmts), "Local constant multiplied array should NOT be lowered");
}

#[test]
fn local_const_variable_as_multiplier_with_non_constant_element_is_lowered() {
    // `[(N)(ADR(x))]` where N is a POU-local `VAR CONSTANT` — the multiplier
    // resolves to a constant but the element is a runtime value, so the array
    // should be lowered into individual assignments.
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts), "Non-constant elements should be lowered");
    assert_eq!(count_assignments(stmts), 3 + 1); // 3 element assignments + return assignment
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts), "ADR() array should be lowered");
    // 3 indexed assignments + `main := 0`
    assert_eq!(count_assignments(stmts), 4);
    assert!(!has_for_loop(stmts), "Small arrays should be unrolled");
}

#[test]
fn large_adr_array_uses_while_loop() {
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts), "ADR() array should be lowered");
    assert!(!has_for_loop(stmts), "Array lowering must not synthesize FOR loops");
    assert!(has_while_loop(stmts), "Large ADR arrays should use a counted WHILE loop");
    // Counter initialization + `main := 0` as direct assignments.
    assert_eq!(count_assignments(stmts), 2);
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts), "ADR expression list should be lowered");
    // 3 indexed assignments + `main := 0`
    assert_eq!(count_assignments(stmts), 4);
}

/// Verify that FB member array non-constant initializers are lowered into
/// indexed assignments in the constructor body.
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
    let stmts = find_impl_stmts(&project, "Foo__ctor");
    assert!(!has_literal_array(stmts), "FB member non-const array should be lowered");
    // 3 indexed assignments for arr + 1 init for x
    assert_eq!(count_assignments(stmts), 4);
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts), "Struct literal array should be lowered");
    // 3 indexed assignments + `main := 0`
    assert_eq!(count_assignments(stmts), 4);
    assert!(!has_for_loop(stmts));
}

#[test]
fn large_struct_array_uses_while_loop() {
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts), "Struct literal array should be lowered");
    assert!(!has_for_loop(stmts), "Array lowering must not synthesize FOR loops");
    assert!(has_while_loop(stmts), "Large struct arrays should use a counted WHILE loop");
    assert_eq!(count_assignments(stmts), 2); // counter initialization + `main := 0`
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts));
    // 2 indexed assignments + `main := 0`
    assert_eq!(count_assignments(stmts), 3);
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts), "Variable-element array should be lowered");
    // 3 indexed assignments + `a := 42` + `main := 0`
    assert_eq!(count_assignments(stmts), 5);
}

#[test]
fn variable_elements_in_expression_list_are_lowered() {
    // The array uses an expression list where each element is a variable
    // reference (`[a, a, a]`), so the initializer is non-constant and must
    // be lowered into indexed runtime assignments.
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts), "Variable-element expression list should be lowered");
    // 3 indexed assignments + `a := 42` + `main := 0`
    assert_eq!(count_assignments(stmts), 5);
}

/// Verify that only the non-constant array is lowered while the constant one is left as-is.
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

    let stmts = find_impl_stmts(&project, "main");
    // 3 indexed assignments for lowered_arr + `seed := 42` + const_arr literal assignment + `main := 0`
    assert_eq!(count_assignments(stmts), 6);
    // const_arr's literal array assignment is still present
    assert!(has_literal_array(stmts), "Constant array should remain as literal");
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts));
    // 2 + 1 + 2 = 5 indexed assignments + `main := 0`
    assert_eq!(count_assignments(stmts), 6);
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts));
    // 3 indexed assignments + `main := 0`
    assert_eq!(count_assignments(stmts), 4);
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts));
    // 6 indexed assignments + `main := 0`
    assert_eq!(count_assignments(stmts), 7);
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
    let stmts = find_impl_stmts(&project, "tarr__ctor");
    assert!(!has_literal_array(stmts));
    assert_eq!(count_assignments(stmts), 3);
}

/// Verify that type-level struct array initializers are lowered in the constructor body.
#[test]
fn type_level_struct_array_is_lowered_in_ctor() {
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
    let stmts = find_impl_stmts(&project, "tarr__ctor");
    assert!(!has_literal_array(stmts), "Type-level struct array should be lowered");
    assert_eq!(count_assignments(stmts), 3);
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

/// `LOOP_THRESHOLD` (32) is checked per `MultipliedStatement` segment, not
/// against the total element count. An initializer split into multiple
/// sub-threshold segments - e.g. `[10(v), 10(v), 10(v), 10(v)]` totalling 40
/// elements - is fully unrolled into 40 individual assignments instead of
/// emitting a generated loop, because each segment's count (10) is below the
/// threshold. This documents the current behaviour so that any future fix is
/// intentional.
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts));
    // Each of the 4 segments (count=10, below threshold=32) is unrolled
    // individually, producing 40 assignments rather than a generated loop.
    // A fix would need to apply the threshold to the *total* element count.
    assert!(!has_while_loop(stmts), "Each segment is below threshold so no loop is emitted");
    assert!(!has_for_loop(stmts), "Array lowering must not synthesize FOR loops");
    // 40 indexed assignments + `v := 1` + `main := 0`
    assert_eq!(count_assignments(stmts), 42);
}
