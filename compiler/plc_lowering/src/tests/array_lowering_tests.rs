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
    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts), "ADR() array should be lowered");
    assert!(has_for_loop(stmts), "Large ADR arrays should use FOR loop");
    // Only `main := 0` as direct assignment
    assert_eq!(count_assignments(stmts), 1);
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

#[test]
fn adr_variable_initializer_is_stripped() {
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
    let pou = project.units[0].get_unit().pous.iter().find(|p| p.name == "main").unwrap();
    let arr_var = pou
        .variable_blocks
        .iter()
        .flat_map(|b| &b.variables)
        .find(|v| v.name == "arr")
        .expect("arr variable should exist");
    assert!(arr_var.initializer.is_none(), "Lowered array initializer should be stripped");
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
    let stmts = find_impl_stmts(&project, "main");
    assert!(!has_literal_array(stmts), "Struct literal array should be lowered");
    assert!(has_for_loop(stmts), "Large struct arrays should use FOR loop");
    assert_eq!(count_assignments(stmts), 1); // only `main := 0`
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
fn variable_as_multiplier_count_with_literal_value_is_lowered() {
    // The multiplier in `(a)(40)` is syntactically a variable reference,
    // but more importantly the expression tree has a non-literal multiplier.
    // The element `40` is constant but the overall construct is non-constant
    // because the repetition count is runtime.
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

#[test]
fn type_level_struct_array_initializer_is_stripped() {
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
    let udt = project.units[0]
        .get_unit()
        .user_types
        .iter()
        .find(|u| u.data_type.get_name() == Some("tarr"))
        .expect("tarr type should exist");
    assert!(udt.initializer.is_none(), "Lowered type initializer should be stripped");
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
