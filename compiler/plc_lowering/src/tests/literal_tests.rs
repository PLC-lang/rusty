use plc_ast::{ast::AstStatement, control_statements::AstControlStatement};
use plc_driver::parse_and_annotate;
use plc_source::SourceCode;

/// Helper: parse + annotate (runs the full pipeline including literal lowering)
/// and return the annotated project.
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

/// Returns true if any statement is a `LiteralArray`.
fn has_literal_array(stmts: &[plc_ast::ast::AstNode]) -> bool {
    stmts.iter().any(|s| {
        if let AstStatement::Assignment(data) = s.get_stmt() {
            matches!(data.right.get_stmt(), AstStatement::Literal(plc_ast::literals::AstLiteral::Array(..)))
        } else {
            false
        }
    })
}

// ── Small multiplied arrays (unrolled) ──────────────────────────────────

#[test]
fn small_multiplied_array_is_unrolled_to_indexed_assignments() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..2] OF DINT := [3(42)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    let stmts = find_impl_stmts(&project, "main");

    // Should have 3 individual assignments + the original `main := 0` + ctor call
    // No LiteralArray should remain
    assert!(!has_literal_array(stmts), "LiteralArray should have been lowered");
    // 3 indexed assignments + 1 `main := 0`
    assert_eq!(count_assignments(stmts), 4);
    // No FOR loop for small arrays
    assert!(!has_for_loop(stmts), "Small arrays should be unrolled, not use FOR loops");
}

#[test]
fn small_multiplied_array_produces_correct_number_of_assignments() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..4] OF DINT := [5(99)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    let stmts = find_impl_stmts(&project, "main");
    // 5 indexed assignments + 1 `main := 0`
    assert_eq!(count_assignments(stmts), 6);
    assert!(!has_for_loop(stmts));
}

// ── Large multiplied arrays (FOR loop) ──────────────────────────────────

#[test]
fn large_multiplied_array_produces_for_loop() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..99] OF DINT := [100(7)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );
    let stmts = find_impl_stmts(&project, "main");

    // Should have a FOR loop instead of 100 individual assignments
    assert!(has_for_loop(stmts), "Large arrays should use FOR loops");
    assert!(!has_literal_array(stmts), "LiteralArray should have been lowered");
    // Only 1 assignment: `main := 0`  (the array init is a FOR loop, not assignments)
    assert_eq!(count_assignments(stmts), 1);
}

// ── Variable initializer stripping ──────────────────────────────────────

#[test]
fn variable_initializer_is_stripped_after_lowering() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..2] OF DINT := [3(42)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );

    // Find the `arr` variable in the POU and verify its initializer was cleared
    let pou = project.units[0].get_unit().pous.iter().find(|p| p.name == "main").unwrap();
    let arr_var = pou
        .variable_blocks
        .iter()
        .flat_map(|b| &b.variables)
        .find(|v| v.name == "arr")
        .expect("arr variable should exist");

    assert!(arr_var.initializer.is_none(), "Variable initializer should be stripped after lowering");
}

// ── Type-level initializer ──────────────────────────────────────────────

#[test]
fn type_level_initializer_is_stripped_after_lowering() {
    let project = lower(
        "
        TYPE tarr : ARRAY[0..4] OF DINT := [5(42)]; END_TYPE

        FUNCTION main : DINT
        VAR
            arr : tarr;
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );

    // The user type declaration's initializer should be None
    let udt = project.units[0]
        .get_unit()
        .user_types
        .iter()
        .find(|u| u.data_type.get_name() == Some("tarr"))
        .expect("tarr type should exist");

    assert!(udt.initializer.is_none(), "Type-level initializer should be stripped after lowering");
}

#[test]
fn type_level_ctor_body_is_lowered() {
    let project = lower(
        "
        TYPE tarr : ARRAY[0..2] OF DINT := [3(42)]; END_TYPE

        FUNCTION main : DINT
        VAR
            arr : tarr;
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );

    let stmts = find_impl_stmts(&project, "tarr__ctor");
    // The ctor should have 3 indexed assignments (unrolled, since 3 < threshold)
    assert_eq!(count_assignments(stmts), 3);
    assert!(!has_literal_array(stmts), "LiteralArray should have been lowered in ctor");
}

// ── Struct array initializers ───────────────────────────────────────────

#[test]
fn struct_array_multiplied_initializer_is_lowered() {
    let project = lower(
        "
        TYPE MyStruct : STRUCT
            a : DINT;
            b : DINT;
        END_STRUCT END_TYPE

        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..2] OF MyStruct := [3((a := 5, b := 10))];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );

    let stmts = find_impl_stmts(&project, "main");
    // Should have 3 indexed struct assignments + `main := 0` + ctor call
    assert!(!has_literal_array(stmts), "LiteralArray should have been lowered");
    // 3 struct init assignments + 1 `main := 0`
    assert_eq!(count_assignments(stmts), 4);
}

// ── Non-zero offset arrays ──────────────────────────────────────────────

#[test]
fn nonzero_offset_array_is_lowered_correctly() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            arr : ARRAY[5..7] OF DINT := [3(77)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );

    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts));
    // 3 indexed assignments + 1 `main := 0`
    assert_eq!(count_assignments(stmts), 4);
}

// ── Mixed initializers (expression list with multiplied segments) ───────

#[test]
fn mixed_array_initializer_is_lowered() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..5] OF DINT := [2(10), 2(20), 99, 100];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );

    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts));
    // 2 + 2 + 1 + 1 = 6 indexed assignments + 1 `main := 0`
    assert_eq!(count_assignments(stmts), 7);
}

// ── Partial initialization ──────────────────────────────────────────────

#[test]
fn partial_array_initializer_is_lowered() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..3] OF DINT := [2(42)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );

    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts));
    // 2 indexed assignments + 1 `main := 0`
    assert_eq!(count_assignments(stmts), 3);
}

// ── Variable override ───────────────────────────────────────────────────

#[test]
fn variable_override_of_type_default_is_lowered() {
    let project = lower(
        "
        TYPE tarr : ARRAY[0..3] OF DINT := [4(42)]; END_TYPE

        FUNCTION main : DINT
        VAR
            arr : tarr := [4(99)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );

    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts));
    // The override should produce 4 indexed assignments + `main := 0`
    assert_eq!(count_assignments(stmts), 5);
}

// ── Large type-level init uses FOR loop ─────────────────────────────────

#[test]
fn large_type_level_init_uses_for_loop_in_ctor() {
    let project = lower(
        "
        TYPE tarr : ARRAY[0..99] OF DINT := [100(7)]; END_TYPE

        FUNCTION main : DINT
        VAR
            arr : tarr;
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );

    let stmts = find_impl_stmts(&project, "tarr__ctor");
    assert!(has_for_loop(stmts), "Large type-level init should produce FOR loop in ctor");
    assert!(!has_literal_array(stmts));
}

// ── Multi-dimensional arrays ────────────────────────────────────────────

#[test]
fn multi_dim_expression_list_is_lowered_to_indexed_assignments() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..1, 0..2] OF DINT := [10, 20, 30, 40, 50, 60];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );

    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts), "LiteralArray should have been lowered");
    // 6 indexed assignments + 1 `main := 0`
    assert_eq!(count_assignments(stmts), 7);
    assert!(!has_for_loop(stmts), "Small multi-dim should be unrolled");
}

#[test]
fn multi_dim_small_multiplied_is_unrolled() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..1, 0..2] OF DINT := [6(42)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );

    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts));
    // 6 indexed assignments + 1 `main := 0`
    assert_eq!(count_assignments(stmts), 7);
    assert!(!has_for_loop(stmts), "Small multi-dim multiplied should be unrolled");
}

#[test]
fn multi_dim_large_full_fill_uses_nested_for_loops() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..9, 0..9] OF DINT := [100(0)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );

    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts));
    assert!(has_for_loop(stmts), "Large multi-dim full fill should use nested FOR loops");
    // Only `main := 0` as a direct assignment; the array fill is inside nested FOR loops
    assert_eq!(count_assignments(stmts), 1);
}

#[test]
fn multi_dim_3d_expression_list_is_lowered() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..2, 0..2, 0..2] OF DINT := [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );

    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts), "3D LiteralArray should have been lowered");
    // 27 indexed assignments + 1 `main := 0`
    assert_eq!(count_assignments(stmts), 28);
}

#[test]
fn multi_dim_nonzero_offset_is_lowered() {
    let project = lower(
        "
        FUNCTION main : DINT
        VAR
            arr : ARRAY[1..2, 1..3] OF DINT := [6(99)];
        END_VAR
            main := 0;
        END_FUNCTION
        ",
    );

    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts));
    // 6 indexed assignments + 1 `main := 0`
    assert_eq!(count_assignments(stmts), 7);
}

// ── flat_to_indices unit tests ──────────────────────────────────────────

#[test]
fn flat_to_indices_2d_zero_based() {
    use crate::literals::{ArrayInfo, DimInfo};
    // ARRAY[0..2, 0..2] — 3x3, row-major
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
    use crate::literals::{ArrayInfo, DimInfo};
    // ARRAY[1..3, 5..7] — 3x3 with offsets
    let info = ArrayInfo { dims: vec![DimInfo { start: 1, size: 3 }, DimInfo { start: 5, size: 3 }] };
    assert_eq!(info.flat_to_indices(0), vec![1, 5]);
    assert_eq!(info.flat_to_indices(1), vec![1, 6]);
    assert_eq!(info.flat_to_indices(3), vec![2, 5]);
    assert_eq!(info.flat_to_indices(8), vec![3, 7]);
}

#[test]
fn flat_to_indices_3d() {
    use crate::literals::{ArrayInfo, DimInfo};
    // ARRAY[0..2, 0..2, 0..2] — 3x3x3
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
