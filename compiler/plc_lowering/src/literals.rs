//! This module is responsible for lowering literal statements into easier to compile instructions.
//! For example, an array of 10000 elements repeating would become a for loop.
//! An array of non repeating elements will become single assignments.
//!
//! ## Lowering rules
//!
//! Given `arr : ARRAY[start..end] OF T := [init];`, the assignment `arr := [init]` is
//! rewritten depending on the shape of `init`:
//!
//! | Pattern                      | Lowered form                                            |
//! |------------------------------|---------------------------------------------------------|
//! | `[N(val)]`  (N ≥ threshold)  | `FOR __idx := start TO start+N-1 DO arr[__idx] := val`  |
//! | `[N(val)]`  (N < threshold)  | `arr[start] := val; arr[start+1] := val; …`             |
//! | `[v1, v2, v3]`               | `arr[start] := v1; arr[start+1] := v2; …`               |
//! | `[3(v1), v2, 2(v3)]`         | three segments – loops and/or individual assignments     |
//!
//! Multi-dimensional arrays are also handled:
//! - Individual assignments use computed multi-dimensional indices
//!   (e.g. flat index 5 on `ARRAY[0..2, 0..2]` → `arr[1, 2]`).
//! - A single `MultipliedStatement` filling the entire array uses nested FOR loops
//!   (one per dimension).
//! - Partial multiplied segments on multi-dim arrays are unrolled to individual
//!   assignments with computed indices.

use std::collections::{BTreeSet, HashMap};

use plc::index::Index;
use plc_ast::{
    ast::{
        AstFactory, AstNode, AstStatement, CompilationUnit, DataTypeDeclaration, MultipliedStatement,
        Variable, VariableBlock, VariableBlockType,
    },
    control_statements::ForLoopStatement,
    literals::{Array, AstLiteral},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

/// The minimum multiplier count at which we emit a FOR loop instead of
/// unrolling into individual assignments.
const FOR_LOOP_THRESHOLD: u32 = 32;

/// Counter variable name used by generated single-dimension FOR loops.
const IDX_VAR: &str = "__literal_idx";

// ── Array dimension info ────────────────────────────────────────────────────

/// Information about a single dimension of an array.
pub(crate) struct DimInfo {
    pub(crate) start: i64,
    pub(crate) size: i64,
}

/// Collected dimension information for an array type.
pub(crate) struct ArrayInfo {
    pub(crate) dims: Vec<DimInfo>,
}

impl ArrayInfo {
    fn is_single_dim(&self) -> bool {
        self.dims.len() == 1
    }

    /// Total number of elements across all dimensions.
    fn total_elements(&self) -> i64 {
        self.dims.iter().map(|d| d.size).product()
    }

    /// Converts a 0-based flat index into actual multi-dimensional indices
    /// (with per-dimension start offsets applied).  Row-major order: the
    /// last dimension varies fastest.
    pub(crate) fn flat_to_indices(&self, flat: i64) -> Vec<i64> {
        let mut indices = vec![0i64; self.dims.len()];
        let mut remaining = flat;
        for i in (0..self.dims.len()).rev() {
            indices[i] = (remaining % self.dims[i].size) + self.dims[i].start;
            remaining /= self.dims[i].size;
        }
        indices
    }

    /// Returns counter variable names for multi-dim nested FOR loops.
    fn nested_counter_names(&self) -> Vec<String> {
        (0..self.dims.len()).map(|i| format!("__literal_idx_{i}")).collect()
    }
}

// ── Lowered result ──────────────────────────────────────────────────────────

struct LoweredResult {
    statements: Vec<AstNode>,
    /// Names of counter variables needed by FOR loops in this result.
    counter_names: BTreeSet<String>,
}

// ── Public entry point ──────────────────────────────────────────────────────

/// Walks every implementation in the compilation unit and rewrites assignments whose
/// right-hand side is an array literal (`LiteralArray`) into indexed assignments
/// and/or FOR loops.  Also strips the corresponding `LiteralArray` initializer
/// from variable declarations so that the data-type generator does not attempt
/// to codegen the (possibly problematic) literal constant.
pub fn lower_literal_arrays(unit: &mut CompilationUnit, index: &Index, id_provider: &mut IdProvider) {
    // Collect the array type names whose body assignments were successfully
    // lowered (e.g. "__main_arr", "tarr").  We strip the corresponding
    // initializer from variable and user-type declarations so the
    // data-type generator doesn't try to codegen the literal constant.
    let mut lowered_type_names: Vec<String> = Vec::new();
    // Track which POUs need which counter variables for FOR loops.
    let mut pou_counters: HashMap<String, BTreeSet<String>> = HashMap::new();

    for implementation in &mut unit.implementations {
        let mut new_statements = Vec::new();
        let mut counters: BTreeSet<String> = BTreeSet::new();

        for stmt in std::mem::take(&mut implementation.statements) {
            if let Some((type_name, lowered)) =
                try_lower_array_assignment(&stmt, index, &implementation.type_name, id_provider)
            {
                counters.extend(lowered.counter_names);
                new_statements.extend(lowered.statements);
                lowered_type_names.push(type_name);
            } else {
                new_statements.push(stmt);
            }
        }

        if !counters.is_empty() {
            pou_counters.insert(implementation.name.clone(), counters);
        }

        implementation.statements = new_statements;
    }

    // Strip the LiteralArray initializer from declarations whose body
    // assignments were lowered.  This prevents the data-type generator from
    // trying to codegen the literal as a const (which can cause infinite
    // recursion for struct-of-array patterns or O(n²) scaling for large arrays).
    if !lowered_type_names.is_empty() {
        strip_initializers(unit, &lowered_type_names);
    }

    // Add VAR_TEMP counter variables to POUs that need them for FOR loops.
    if !pou_counters.is_empty() {
        add_counter_variables(unit, &pou_counters);
    }
}

// ── Counter variable insertion ──────────────────────────────────────────────

/// Adds `VAR_TEMP` counter variables to POUs that have generated FOR loops.
fn add_counter_variables(unit: &mut CompilationUnit, pou_counters: &HashMap<String, BTreeSet<String>>) {
    for pou in &mut unit.pous {
        if let Some(counter_names) = pou_counters.get(&pou.name) {
            let variables: Vec<Variable> = counter_names
                .iter()
                .map(|name| Variable {
                    name: name.clone(),
                    data_type_declaration: DataTypeDeclaration::Reference {
                        referenced_type: "DINT".to_string(),
                        location: SourceLocation::internal(),
                    },
                    initializer: None,
                    address: None,
                    location: SourceLocation::internal(),
                })
                .collect();

            pou.variable_blocks.push(VariableBlock {
                variables,
                kind: VariableBlockType::Temp,
                ..Default::default()
            });
        }
    }
}

// ── Strip initializers ──────────────────────────────────────────────────────

/// Removes array-literal initializers from POU variable declarations and
/// user-type declarations whose array type was lowered into body statements.
fn strip_initializers(unit: &mut CompilationUnit, lowered_type_names: &[String]) {
    // Strip from POU variable declarations.
    for pou in &mut unit.pous {
        for block in &mut pou.variable_blocks {
            for var in &mut block.variables {
                if has_array_literal_initializer(var) && is_lowered_type(var, lowered_type_names) {
                    var.initializer = None;
                }
            }
        }
    }

    // Strip from user-type declarations (e.g. `TYPE tarr : ARRAY[...] := [...]; END_TYPE`).
    for udt in &mut unit.user_types {
        if udt.initializer.as_ref().is_some_and(is_literal_array_node) {
            let type_name = udt.data_type.get_name();
            if type_name.is_some_and(|n| lowered_type_names.iter().any(|lt| lt == n)) {
                udt.initializer = None;
            }
        }
    }
}

fn has_array_literal_initializer(var: &Variable) -> bool {
    var.initializer.as_ref().is_some_and(is_literal_array_node)
}

fn is_literal_array_node(node: &AstNode) -> bool {
    matches!(node.get_stmt(), AstStatement::Literal(AstLiteral::Array(_)))
}

fn is_lowered_type(var: &Variable, lowered_type_names: &[String]) -> bool {
    var.data_type_declaration.get_name().is_some_and(|name| lowered_type_names.iter().any(|lt| lt == name))
}

// ── Core lowering ───────────────────────────────────────────────────────────

/// Checks whether `stmt` is an assignment whose RHS is a `LiteralArray`. If so,
/// returns the resolved array type name and the lowered statements.
fn try_lower_array_assignment(
    stmt: &AstNode,
    index: &Index,
    pou_type_name: &str,
    id_provider: &mut IdProvider,
) -> Option<(String, LoweredResult)> {
    let AstStatement::Assignment(data) = stmt.get_stmt() else {
        return None;
    };

    let AstStatement::Literal(AstLiteral::Array(Array { elements: Some(elements) })) = data.right.get_stmt()
    else {
        return None;
    };

    // Look up the LHS variable type to get array dimensions.
    let lhs_type_name = find_lhs_type_name(data.left.as_ref(), index, pou_type_name)?;
    let type_info = index.find_effective_type_info(&lhs_type_name)?;

    let plc::typesystem::DataTypeInformation::Array { dimensions, .. } = type_info else {
        return None;
    };

    // Build ArrayInfo from all dimensions.
    let mut dims = Vec::new();
    for dim in dimensions {
        let start = dim.start_offset.as_int_value(index).ok()?;
        let end = dim.end_offset.as_int_value(index).ok()?;
        let size = end - start + 1;
        if size <= 0 {
            return None;
        }
        dims.push(DimInfo { start, size });
    }

    let array_info = ArrayInfo { dims };
    let lowered = lower_array_elements(data.left.as_ref(), elements, &array_info, id_provider);
    Some((lhs_type_name, lowered))
}

/// Determines the type name of the LHS of an assignment by consulting the index.
/// Handles `self.field`, `field`, and plain identifiers.
fn find_lhs_type_name(lhs: &AstNode, index: &Index, pou_type_name: &str) -> Option<String> {
    match lhs.get_stmt() {
        AstStatement::ReferenceExpr(plc_ast::ast::ReferenceExpr {
            access: plc_ast::ast::ReferenceAccess::Member(member),
            base,
        }) => {
            let member_name = member.get_flat_reference_name()?;

            if let Some(base) = base {
                // e.g. `self.arr` — resolve base type then find member
                let base_type = find_lhs_type_name(base, index, pou_type_name)?;
                let base_type_info = index.find_effective_type_info(&base_type)?;
                if let plc::typesystem::DataTypeInformation::Struct { name, .. } = base_type_info {
                    let member_entry = index.find_member(name, member_name)?;
                    Some(member_entry.get_type_name().to_string())
                } else {
                    None
                }
            } else if member_name == "self" {
                // `self` in a constructor refers to the POU/type being constructed.
                // It's a VAR_IN_OUT so its indexed type is a pointer
                // (`__auto_pointer_to_tarr`).  Dereference through the pointer to
                // get the actual type name (e.g. `tarr`).
                index.find_member(pou_type_name, "self").map(|v| {
                    let type_name = v.get_type_name();
                    match index.find_effective_type_info(type_name) {
                        Some(plc::typesystem::DataTypeInformation::Pointer { inner_type_name, .. }) => {
                            inner_type_name.to_string()
                        }
                        _ => type_name.to_string(),
                    }
                })
            } else {
                // Bare member reference (e.g. `arr` in function body) — the
                // initializer produces `ReferenceExpr { Member("arr"), base: None }`
                // for local/global variables. Resolve as a member of the current POU.
                index.find_member(pou_type_name, member_name).map(|v| v.get_type_name().to_string())
            }
        }
        AstStatement::Identifier(name) => {
            if name == "self" {
                Some(pou_type_name.to_string())
            } else {
                index
                    .find_member(pou_type_name, name)
                    .map(|v| v.get_type_name().to_string())
                    .or_else(|| index.find_effective_type_info(name).map(|_| name.clone()))
            }
        }
        _ => None,
    }
}

/// The core lowering: given the LHS reference, the array literal elements,
/// and the array dimension info, produces a list of statements.
fn lower_array_elements(
    lhs: &AstNode,
    elements: &AstNode,
    array_info: &ArrayInfo,
    id_provider: &mut IdProvider,
) -> LoweredResult {
    let mut statements = Vec::new();
    let mut counter_names = BTreeSet::new();
    let mut current_flat_offset: i64 = 0;

    match elements.get_stmt() {
        // [N(val)] — single multiplied segment
        AstStatement::MultipliedStatement(MultipliedStatement { multiplier, element }) => {
            let segment = lower_multiplied_segment(
                lhs,
                element,
                *multiplier,
                current_flat_offset,
                array_info,
                id_provider,
            );
            counter_names.extend(segment.counter_names);
            statements.extend(segment.statements);
        }

        // [e1, e2, ...] — expression list, may contain multiplied segments
        AstStatement::ExpressionList(expressions) => {
            for expr in expressions {
                match expr.get_stmt() {
                    AstStatement::MultipliedStatement(MultipliedStatement { multiplier, element }) => {
                        let segment = lower_multiplied_segment(
                            lhs,
                            element,
                            *multiplier,
                            current_flat_offset,
                            array_info,
                            id_provider,
                        );
                        counter_names.extend(segment.counter_names);
                        statements.extend(segment.statements);
                        current_flat_offset += i64::from(*multiplier);
                    }
                    _ => {
                        let indexed_lhs =
                            make_array_element_access(lhs, current_flat_offset, array_info, id_provider);
                        let assignment =
                            AstFactory::create_assignment(indexed_lhs, expr.clone(), id_provider.next_id());
                        statements.push(assignment);
                        current_flat_offset += 1;
                    }
                }
            }
        }

        // Single value (unusual but valid: `[val]`)
        _ => {
            let indexed_lhs = make_array_element_access(lhs, current_flat_offset, array_info, id_provider);
            let assignment =
                AstFactory::create_assignment(indexed_lhs, elements.clone(), id_provider.next_id());
            statements.push(assignment);
        }
    }

    LoweredResult { statements, counter_names }
}

/// Lowers a `MultipliedStatement(count, element)` segment.
/// Small counts → unrolled individual assignments.
/// Large counts on single-dim → a FOR loop.
/// Large counts on multi-dim filling the whole array → nested FOR loops.
/// Large counts on multi-dim (partial) → unrolled individual assignments.
fn lower_multiplied_segment(
    lhs: &AstNode,
    element: &AstNode,
    count: u32,
    flat_offset: i64,
    array_info: &ArrayInfo,
    id_provider: &mut IdProvider,
) -> LoweredResult {
    let use_for_loop = count >= FOR_LOOP_THRESHOLD;

    if !use_for_loop {
        // Small count: unroll to individual indexed assignments.
        return unroll_to_assignments(lhs, element, count, flat_offset, array_info, id_provider);
    }

    if array_info.is_single_dim() {
        // Single-dim, large: emit a FOR loop.
        let start = array_info.dims[0].start + flat_offset;
        let end = start + i64::from(count) - 1;
        let for_loop = make_for_loop(lhs, element, start, end, id_provider);
        LoweredResult { statements: vec![for_loop], counter_names: BTreeSet::from([IDX_VAR.to_string()]) }
    } else if flat_offset == 0 && i64::from(count) == array_info.total_elements() {
        // Multi-dim, full fill: emit nested FOR loops (one per dimension).
        let for_loop = make_nested_for_loops(lhs, element, array_info, id_provider);
        let counter_names = array_info.nested_counter_names().into_iter().collect();
        LoweredResult { statements: vec![for_loop], counter_names }
    } else {
        // Multi-dim, partial large: unroll with computed multi-dim indices.
        // This is rare but correct — each element gets its own assignment.
        unroll_to_assignments(lhs, element, count, flat_offset, array_info, id_provider)
    }
}

/// Unrolls a multiplied segment into individual indexed assignments.
fn unroll_to_assignments(
    lhs: &AstNode,
    element: &AstNode,
    count: u32,
    flat_offset: i64,
    array_info: &ArrayInfo,
    id_provider: &mut IdProvider,
) -> LoweredResult {
    let statements = (0..count)
        .map(|i| {
            let indexed_lhs =
                make_array_element_access(lhs, flat_offset + i64::from(i), array_info, id_provider);
            AstFactory::create_assignment(indexed_lhs, element.clone(), id_provider.next_id())
        })
        .collect();
    LoweredResult { statements, counter_names: BTreeSet::new() }
}

// ── AST construction helpers ────────────────────────────────────────────────

/// Creates the appropriate indexed access for a given flat offset.
/// For single-dim arrays: `lhs[start + flat_offset]`.
/// For multi-dim arrays: `lhs[i0, i1, ...]` with computed indices.
fn make_array_element_access(
    lhs: &AstNode,
    flat_offset: i64,
    array_info: &ArrayInfo,
    id_provider: &mut IdProvider,
) -> AstNode {
    if array_info.is_single_dim() {
        make_indexed_access(lhs, array_info.dims[0].start + flat_offset, id_provider)
    } else {
        let indices = array_info.flat_to_indices(flat_offset);
        make_multi_dim_indexed_access(lhs, &indices, id_provider)
    }
}

/// Creates `lhs[i0, i1, ...]` for multi-dimensional array access.
fn make_multi_dim_indexed_access(lhs: &AstNode, indices: &[i64], id_provider: &mut IdProvider) -> AstNode {
    let loc = SourceLocation::internal();
    let idx_nodes: Vec<AstNode> = indices.iter().map(|&i| make_int_literal(i, id_provider)).collect();
    let idx_expr = AstFactory::create_expression_list(idx_nodes, loc.clone(), id_provider.next_id());
    AstFactory::create_index_reference(idx_expr, Some(lhs.clone()), id_provider.next_id(), loc)
}

/// Creates `lhs[index_literal]` for single-dimension array access.
fn make_indexed_access(lhs: &AstNode, index: i64, id_provider: &mut IdProvider) -> AstNode {
    let idx_node = make_int_literal(index, id_provider);
    AstFactory::create_index_reference(
        idx_node,
        Some(lhs.clone()),
        id_provider.next_id(),
        SourceLocation::internal(),
    )
}

/// Creates: `FOR __literal_idx := start TO end DO lhs[__literal_idx] := element; END_FOR`
/// Used for single-dimension arrays.
fn make_for_loop(
    lhs: &AstNode,
    element: &AstNode,
    start: i64,
    end: i64,
    id_provider: &mut IdProvider,
) -> AstNode {
    let loc = SourceLocation::internal();

    let counter = make_member_reference(IDX_VAR, id_provider);
    let start_node = make_int_literal(start, id_provider);
    let end_node = make_int_literal(end, id_provider);

    // lhs[__literal_idx]
    let idx_ref = make_member_reference(IDX_VAR, id_provider);
    let indexed_lhs =
        AstFactory::create_index_reference(idx_ref, Some(lhs.clone()), id_provider.next_id(), loc.clone());

    let body_assignment = AstFactory::create_assignment(indexed_lhs, element.clone(), id_provider.next_id());

    AstFactory::create_for_loop(
        ForLoopStatement {
            counter: Box::new(counter),
            start: Box::new(start_node),
            end: Box::new(end_node),
            by_step: None,
            body: vec![body_assignment],
            end_location: loc.clone(),
        },
        loc,
        id_provider.next_id(),
    )
}

/// Creates nested FOR loops for multi-dimensional arrays:
/// ```text
/// FOR __literal_idx_0 := dim0_start TO dim0_end DO
///   FOR __literal_idx_1 := dim1_start TO dim1_end DO
///     ...
///       lhs[__literal_idx_0, __literal_idx_1, ...] := element;
///     ...
///   END_FOR
/// END_FOR
/// ```
fn make_nested_for_loops(
    lhs: &AstNode,
    element: &AstNode,
    array_info: &ArrayInfo,
    id_provider: &mut IdProvider,
) -> AstNode {
    let loc = SourceLocation::internal();
    let counter_names = array_info.nested_counter_names();

    // Create the indexed access: lhs[__literal_idx_0, __literal_idx_1, ...]
    let idx_refs: Vec<AstNode> =
        counter_names.iter().map(|name| make_member_reference(name, id_provider)).collect();
    let idx_expr = AstFactory::create_expression_list(idx_refs, loc.clone(), id_provider.next_id());
    let indexed_lhs =
        AstFactory::create_index_reference(idx_expr, Some(lhs.clone()), id_provider.next_id(), loc.clone());

    let assignment = AstFactory::create_assignment(indexed_lhs, element.clone(), id_provider.next_id());

    // Build nested FOR loops from innermost dimension to outermost.
    let mut body = vec![assignment];
    for i in (0..array_info.dims.len()).rev() {
        let counter = make_member_reference(&counter_names[i], id_provider);
        let start_node = make_int_literal(array_info.dims[i].start, id_provider);
        let end_node = make_int_literal(array_info.dims[i].start + array_info.dims[i].size - 1, id_provider);

        let for_loop = AstFactory::create_for_loop(
            ForLoopStatement {
                counter: Box::new(counter),
                start: Box::new(start_node),
                end: Box::new(end_node),
                by_step: None,
                body,
                end_location: loc.clone(),
            },
            loc.clone(),
            id_provider.next_id(),
        );
        body = vec![for_loop];
    }

    body.pop().expect("nested FOR loop construction should produce exactly one statement")
}

// ── Primitive helpers ───────────────────────────────────────────────────────

fn make_int_literal(value: i64, id_provider: &mut IdProvider) -> AstNode {
    AstFactory::create_literal(
        AstLiteral::Integer(value as i128),
        SourceLocation::internal(),
        id_provider.next_id(),
    )
}

fn make_member_reference(name: &str, id_provider: &mut IdProvider) -> AstNode {
    AstFactory::create_member_reference(
        AstFactory::create_identifier(name, SourceLocation::internal(), id_provider.next_id()),
        None,
        id_provider.next_id(),
    )
}
