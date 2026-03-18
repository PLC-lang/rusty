//! # Array Initializer Lowering
//!
//! This pass rewrites array initializer assignments whose elements cannot be
//! evaluated at compile time into sequences of indexed assignments and/or FOR
//! loops that the codegen backend can handle.
//!
//! ## Why this exists
//!
//! The initializer pass (`initializer.rs`) emits constructor bodies that assign
//! array literals to their targets: `self.arr := [init];`.  When every element
//! of the literal is a compile-time constant (integers, reals, bools, …) codegen
//! materializes the whole array as an anonymous global constant and copies it
//! with `memcpy` — no lowering needed.
//!
//! However, some element expressions are **not** compile-time constants:
//!
//! - **Function calls** such as `ADR(x)` produce addresses that depend on
//!   runtime memory layout.  These can never appear in a global constant.
//!
//! - **Struct literal initializers** like `(a := 5, b := 10)` are semantically
//!   constant but codegen's `generate_literal_array_value` currently cannot
//!   evaluate them inside an array context (it stack-overflows in the
//!   context-free expression generator).  This is a codegen limitation, not a
//!   fundamental one — once fixed, struct literals can be removed from the
//!   trigger list.
//!
//! For these cases the assignment must be decomposed into individual element
//! stores that the expression generator can handle one at a time.
//!
//! ## When this pass triggers
//!
//! The pass walks every implementation body looking for assignments whose RHS
//! is a `LiteralArray`.  It checks the array's element tree for non-constant
//! expressions via [`contains_non_constant_expression`]:
//!
//! | Element kind             | Constant? | Triggers lowering?                   |
//! |--------------------------|-----------|--------------------------------------|
//! | Literal (int, real, …)   | yes       | no                                   |
//! | String literal           | yes       | no                                   |
//! | Variable reference       | **no**    | **yes** — runtime value              |
//! | Function call (`ADR(x)`) | **no**    | **yes** — runtime value              |
//! | Struct literal `(a:=1)`  | **no***   | **yes** — codegen limitation         |
//!
//! *Struct literals are semantically constant but are treated as non-constant
//! because codegen's `generate_literal_array_value` cannot currently evaluate
//! them inside an array context.
//!
//! If no non-constant expression is found, the literal is left as-is for
//! codegen's memcpy path.
//!
//! ## Lowering rules
//!
//! Given `arr : ARRAY[start..end] OF T := [init];` where `init` contains
//! non-constant expressions:
//!
//! | Pattern                      | Lowered form                                            |
//! |------------------------------|---------------------------------------------------------|
//! | `[N(val)]`  (N ≥ threshold)  | `FOR __idx := start TO start+N-1 DO arr[__idx] := val`  |
//! | `[N(val)]`  (N < threshold)  | `arr[start] := val; arr[start+1] := val; …`             |
//! | `[v1, v2, v3]`               | `arr[start] := v1; arr[start+1] := v2; …`               |
//! | `[3(v1), v2, 2(v3)]`         | mixed — each segment lowered independently              |
//!
//! Multi-dimensional arrays are also handled:
//! - Individual assignments use computed multi-dimensional indices via
//!   [`ArrayInfo::flat_to_indices`] (e.g. flat index 5 on `ARRAY[0..2, 0..2]`
//!   becomes `arr[1, 2]`).
//! - A single `MultipliedStatement` filling the entire array uses nested FOR
//!   loops (one per dimension).
//! - Partial multiplied segments on multi-dim arrays are unrolled to individual
//!   assignments with computed indices.
//!
//! ## Side effects
//!
//! After lowering, the pass:
//! 1. **Strips the original initializer** from the variable / user-type
//!    declaration so the data-type generator does not attempt to create a
//!    `__init` global for an expression it cannot evaluate.
//! 2. **Adds `VAR_TEMP` counter variables** (e.g. `__literal_idx`) to POUs
//!    that contain generated FOR loops.

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

struct LoweredAssignment {
    type_name: String,
    variable_name: Option<String>,
    lowered: LoweredResult,
}

// ── Public entry point ──────────────────────────────────────────────────────

/// Walks every implementation in the compilation unit and rewrites assignments whose
/// right-hand side is an array literal (`LiteralArray`) into indexed assignments
/// and/or FOR loops.  Also strips the corresponding `LiteralArray` initializer
/// from variable declarations so that the data-type generator does not attempt
/// to codegen the (possibly problematic) literal constant.
pub fn lower_literal_arrays(unit: &mut CompilationUnit, index: &Index, id_provider: &mut IdProvider) {
    // Track variable declarations whose initializers were lowered in body code.
    // Keyed by POU/type name -> variable names.
    let mut lowered_variables: HashMap<String, BTreeSet<String>> = HashMap::new();
    // Track user-type declarations whose type-level initializer was lowered.
    let mut lowered_user_types: BTreeSet<String> = BTreeSet::new();
    // Track which POUs need which counter variables for FOR loops.
    let mut pou_counters: HashMap<String, BTreeSet<String>> = HashMap::new();

    // Resolve constant references in MultipliedStatement multipliers to literal
    // integers.  The parser emits `(NB_BOOL)` as a reference node; with the index
    // available we can replace it with the actual value so that downstream passes
    // (flatten_expression_list, codegen memcpy) see a plain literal.
    resolve_const_multipliers(unit, index);

    for implementation in &mut unit.implementations {
        let mut new_statements = Vec::new();
        let mut counters: BTreeSet<String> = BTreeSet::new();

        for stmt in std::mem::take(&mut implementation.statements) {
            if let Some(lowered_assignment) =
                try_lower_array_assignment(&stmt, index, &implementation.type_name, id_provider)
            {
                counters.extend(lowered_assignment.lowered.counter_names);
                new_statements.extend(lowered_assignment.lowered.statements);

                if let Some(variable_name) = lowered_assignment.variable_name {
                    lowered_variables
                        .entry(implementation.type_name.clone())
                        .or_default()
                        .insert(variable_name);
                } else {
                    lowered_user_types.insert(lowered_assignment.type_name);
                }
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
    if !lowered_variables.is_empty() || !lowered_user_types.is_empty() {
        strip_initializers(unit, &lowered_variables, &lowered_user_types);
    }

    // Add VAR_TEMP counter variables to POUs that need them for FOR loops.
    if !pou_counters.is_empty() {
        add_counter_variables(unit, &pou_counters);
    }
}

// ── Constant multiplier resolution ─────────────────────────────────────────

/// Walks all `MultipliedStatement` nodes in the compilation unit and resolves
/// constant-reference multipliers to literal integers.  This ensures that
/// `[(NB_BOOL)(0.0033)]` where `NB_BOOL` is a constant produces a literal
/// multiplier that downstream passes (flatten, codegen memcpy) can handle.
fn resolve_const_multipliers(unit: &mut CompilationUnit, index: &Index) {
    for pou in &mut unit.pous {
        let pou_name = pou.name.clone();
        for block in &mut pou.variable_blocks {
            for var in &mut block.variables {
                if let Some(init) = &mut var.initializer {
                    resolve_multipliers_in_node(init, index, Some(&pou_name));
                }
            }
        }
    }
    for block in &mut unit.global_vars {
        for var in &mut block.variables {
            if let Some(init) = &mut var.initializer {
                resolve_multipliers_in_node(init, index, None);
            }
        }
    }
    for udt in &mut unit.user_types {
        if let Some(init) = &mut udt.initializer {
            resolve_multipliers_in_node(init, index, None);
        }
    }
    for implementation in &mut unit.implementations {
        let type_name = implementation.type_name.clone();
        for stmt in &mut implementation.statements {
            if let AstStatement::Assignment(data) = stmt.get_stmt_mut() {
                resolve_multipliers_in_node(&mut data.right, index, Some(&type_name));
            }
        }
    }
}

/// Recursively walks a node tree and resolves constant-reference multipliers
/// inside `MultipliedStatement` nodes to literal integers.
fn resolve_multipliers_in_node(node: &mut AstNode, index: &Index, pou_name: Option<&str>) {
    match node.get_stmt_mut() {
        AstStatement::Literal(AstLiteral::Array(Array { elements: Some(elements) })) => {
            resolve_multipliers_in_node(elements, index, pou_name);
        }
        AstStatement::ExpressionList(exprs) => {
            for expr in exprs.iter_mut() {
                resolve_multipliers_in_node(expr, index, pou_name);
            }
        }
        AstStatement::MultipliedStatement(data) => {
            // Try to resolve the multiplier to a literal integer.
            if data.multiplier.get_literal_integer_value().is_none() {
                if let Some(value) = resolve_const_reference(&data.multiplier, index, pou_name) {
                    *data.multiplier = AstFactory::create_literal(
                        AstLiteral::Integer(value as i128),
                        data.multiplier.get_location(),
                        data.multiplier.get_id(),
                    );
                }
            }
            // Also recurse into the element (for nested multiplied statements).
            resolve_multipliers_in_node(&mut data.element, index, pou_name);
        }
        _ => {}
    }
}

/// Tries to resolve a reference node to a constant integer value via the index.
fn resolve_const_reference(node: &AstNode, index: &Index, pou_name: Option<&str>) -> Option<i64> {
    let ref_name = node.get_flat_reference_name()?;
    let variable = pou_name
        .and_then(|pou| index.find_member(pou, ref_name))
        .or_else(|| index.find_global_variable(ref_name))?;

    if !variable.is_constant() {
        return None;
    }

    let initial_value = index.get_initial_value(&variable.initial_value)?;
    initial_value.get_literal_integer_value().map(|v| v as i64)
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
/// user-type declarations that were specifically lowered.
fn strip_initializers(
    unit: &mut CompilationUnit,
    lowered_variables: &HashMap<String, BTreeSet<String>>,
    lowered_user_types: &BTreeSet<String>,
) {
    // Strip from specific POU variable declarations.
    for pou in &mut unit.pous {
        let Some(lowered_names) = lowered_variables.get(&pou.name) else {
            continue;
        };

        for block in &mut pou.variable_blocks {
            for var in &mut block.variables {
                if has_array_literal_initializer(var) && lowered_names.contains(&var.name) {
                    var.initializer = None;
                }
            }
        }
    }

    // Strip from user-type declarations (e.g. `TYPE tarr : ARRAY[...] := [...]; END_TYPE`).
    for udt in &mut unit.user_types {
        if udt.initializer.as_ref().is_some_and(is_literal_array_node) {
            let type_name = udt.data_type.get_name();
            if type_name.is_some_and(|n| lowered_user_types.contains(n)) {
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

// ── Core lowering ───────────────────────────────────────────────────────────

/// Checks whether `stmt` is an assignment whose RHS is a `LiteralArray`. If so,
/// returns the resolved array type name and the lowered statements.
///
/// Only arrays whose elements contain non-constant expressions (e.g. function calls)
/// are lowered. Constant array literals are handled efficiently at codegen level via
/// memcpy from a materialized global constant.
fn try_lower_array_assignment(
    stmt: &AstNode,
    index: &Index,
    pou_type_name: &str,
    id_provider: &mut IdProvider,
) -> Option<LoweredAssignment> {
    let AstStatement::Assignment(data) = stmt.get_stmt() else {
        return None;
    };

    let AstStatement::Literal(AstLiteral::Array(Array { elements: Some(elements) })) = data.right.get_stmt()
    else {
        return None;
    };

    // Only lower if the array contains non-constant expressions (function calls).
    // Constant array literals are handled at codegen via memcpy from a global.
    if !contains_non_constant_expression(elements) {
        return None;
    }

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
    let variable_name = find_lhs_variable_name(data.left.as_ref());

    Some(LoweredAssignment { type_name: lhs_type_name, variable_name, lowered })
}

/// Returns `true` if the expression tree contains any non-constant expression
/// that cannot be evaluated at compile time, such as function calls or struct
/// literal initializers `(a := 1, b := 2)`.
fn contains_non_constant_expression(node: &AstNode) -> bool {
    !is_constant_expression(node)
}

/// Returns `true` if every leaf in the expression tree is a compile-time constant
/// (literal integer, real, bool, string, etc.).  Anything else — variable references,
/// function calls, struct literal assignments — is considered non-constant.
fn is_constant_expression(node: &AstNode) -> bool {
    match node.get_stmt() {
        AstStatement::Literal(..) => true,
        AstStatement::ExpressionList(exprs) => exprs.iter().all(is_constant_expression),
        AstStatement::MultipliedStatement(MultipliedStatement { multiplier, element }) => {
            is_constant_expression(multiplier) && is_constant_expression(element)
        }
        AstStatement::ParenExpression(inner) => is_constant_expression(inner),
        // Everything else: variable references, function calls, struct literal
        // assignments, etc. — not a compile-time constant.
        _ => false,
    }
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

/// Best-effort extraction of the variable/member name assigned on the LHS.
/// Returns `None` for `self := ...` assignments (type-level ctor init).
fn find_lhs_variable_name(lhs: &AstNode) -> Option<String> {
    match lhs.get_stmt() {
        AstStatement::ReferenceExpr(plc_ast::ast::ReferenceExpr {
            access: plc_ast::ast::ReferenceAccess::Member(member),
            base,
        }) => {
            let member_name = member.get_flat_reference_name()?.to_string();
            if member_name == "self" {
                return None;
            }

            if let Some(base) = base {
                // `self.arr` (or any member access ending in a member name) -> `arr`
                if base.get_flat_reference_name().is_some() {
                    return Some(member_name);
                }
                None
            } else {
                Some(member_name)
            }
        }
        AstStatement::Identifier(name) if name != "self" => Some(name.clone()),
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
    // The flat offset tracks where the next element goes. It is an AstNode
    // expression so that non-literal multipliers (variables) can be accumulated
    // into the offset for subsequent segments.
    let mut current_flat_offset = make_int_literal(0, id_provider);

    match elements.get_stmt() {
        // [N(val)] — single multiplied segment
        AstStatement::MultipliedStatement(MultipliedStatement { multiplier, element }) => {
            let segment = lower_multiplied_segment(
                lhs,
                element,
                multiplier,
                &current_flat_offset,
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
                            multiplier,
                            &current_flat_offset,
                            array_info,
                            id_provider,
                        );
                        counter_names.extend(segment.counter_names);
                        statements.extend(segment.statements);
                        current_flat_offset = advance_offset(&current_flat_offset, multiplier, id_provider);
                    }
                    _ => {
                        let indexed_lhs = make_array_element_access_expr(
                            lhs,
                            &current_flat_offset,
                            array_info,
                            id_provider,
                        );
                        let assignment =
                            AstFactory::create_assignment(indexed_lhs, expr.clone(), id_provider.next_id());
                        statements.push(assignment);
                        current_flat_offset = advance_offset(
                            &current_flat_offset,
                            &make_int_literal(1, id_provider),
                            id_provider,
                        );
                    }
                }
            }
        }

        // Single value (unusual but valid: `[val]`)
        _ => {
            let indexed_lhs =
                make_array_element_access_expr(lhs, &current_flat_offset, array_info, id_provider);
            let assignment =
                AstFactory::create_assignment(indexed_lhs, elements.clone(), id_provider.next_id());
            statements.push(assignment);
        }
    }

    LoweredResult { statements, counter_names }
}

/// Lowers a `MultipliedStatement(multiplier, element)` segment.
///
/// When the multiplier resolves to a literal integer:
/// - Small counts → unrolled individual assignments.
/// - Large counts on single-dim → a FOR loop.
/// - Large counts on multi-dim filling the whole array → nested FOR loops.
/// - Large counts on multi-dim (partial) → unrolled individual assignments.
///
/// When the multiplier is a non-constant expression (e.g. a variable):
/// - Always emits a FOR loop using the expression as the loop bound.
fn lower_multiplied_segment(
    lhs: &AstNode,
    element: &AstNode,
    multiplier: &AstNode,
    flat_offset: &AstNode,
    array_info: &ArrayInfo,
    id_provider: &mut IdProvider,
) -> LoweredResult {
    let count = resolve_multiplier_value(multiplier);
    let flat_offset_val = flat_offset.get_literal_integer_value().map(|v| v as i64);

    // When both the multiplier and the offset are known literals, we can use
    // optimised paths (unrolling for small counts, nested loops for multi-dim).
    if let (Some(count), Some(offset)) = (count, flat_offset_val) {
        let use_for_loop = count >= FOR_LOOP_THRESHOLD;

        if !use_for_loop {
            return unroll_to_assignments(lhs, element, count, offset, array_info, id_provider);
        }

        if array_info.is_single_dim() {
            let start = array_info.dims[0].start + offset;
            let end = start + i64::from(count) - 1;
            let for_loop = make_for_loop(lhs, element, start, end, id_provider);
            return LoweredResult {
                statements: vec![for_loop],
                counter_names: BTreeSet::from([IDX_VAR.to_string()]),
            };
        } else if offset == 0 && i64::from(count) == array_info.total_elements() {
            let for_loop = make_nested_for_loops(lhs, element, array_info, id_provider);
            let counter_names = array_info.nested_counter_names().into_iter().collect();
            return LoweredResult { statements: vec![for_loop], counter_names };
        } else {
            return unroll_to_assignments(lhs, element, count, offset, array_info, id_provider);
        }
    }

    // Either the multiplier or the offset (or both) are non-literal expressions.
    // Emit a FOR loop with expression-based bounds.
    lower_expression_multiplier(lhs, element, multiplier, flat_offset, array_info, id_provider)
}

/// Extracts the multiplier as a `u32` if it is a literal integer.
fn resolve_multiplier_value(multiplier: &AstNode) -> Option<u32> {
    multiplier.get_literal_integer_value().and_then(|v| u32::try_from(v).ok())
}

/// Lowers a multiplied segment with expression-based multiplier and/or offset into a
/// FOR loop: `FOR __idx := dim_start + offset TO dim_start + offset + multiplier - 1 DO ... END_FOR`
fn lower_expression_multiplier(
    lhs: &AstNode,
    element: &AstNode,
    multiplier: &AstNode,
    flat_offset: &AstNode,
    array_info: &ArrayInfo,
    id_provider: &mut IdProvider,
) -> LoweredResult {
    let loc = SourceLocation::internal();
    let dim_start = make_int_literal(array_info.dims[0].start, id_provider);

    // start = dim_start + flat_offset
    let start_node = AstFactory::create_binary_expression(
        dim_start,
        plc_ast::ast::Operator::Plus,
        flat_offset.clone(),
        id_provider.next_id(),
    );

    // end = start + multiplier - 1
    let start_plus_mult = AstFactory::create_binary_expression(
        start_node.clone(),
        plc_ast::ast::Operator::Plus,
        multiplier.clone(),
        id_provider.next_id(),
    );
    let end_node = AstFactory::create_binary_expression(
        start_plus_mult,
        plc_ast::ast::Operator::Minus,
        make_int_literal(1, id_provider),
        id_provider.next_id(),
    );

    let counter = make_member_reference(IDX_VAR, id_provider);

    // lhs[__literal_idx]
    let idx_ref = make_member_reference(IDX_VAR, id_provider);
    let indexed_lhs =
        AstFactory::create_index_reference(idx_ref, Some(lhs.clone()), id_provider.next_id(), loc.clone());

    let body_assignment = AstFactory::create_assignment(indexed_lhs, element.clone(), id_provider.next_id());

    let for_loop = AstFactory::create_for_loop(
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
    );

    LoweredResult { statements: vec![for_loop], counter_names: BTreeSet::from([IDX_VAR.to_string()]) }
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

/// Advances the flat offset by the given increment (which may be a variable expression).
/// If both are literal integers, produces a simplified literal sum. Otherwise
/// produces a binary `offset + increment` expression.
fn advance_offset(offset: &AstNode, increment: &AstNode, id_provider: &mut IdProvider) -> AstNode {
    if let (Some(a), Some(b)) = (offset.get_literal_integer_value(), increment.get_literal_integer_value()) {
        make_int_literal(a as i64 + b as i64, id_provider)
    } else {
        AstFactory::create_binary_expression(
            offset.clone(),
            plc_ast::ast::Operator::Plus,
            increment.clone(),
            id_provider.next_id(),
        )
    }
}

/// Expression-based array element access.  For single-dim arrays, produces
/// `lhs[dim_start + flat_offset_expr]`.  Multi-dim arrays with non-literal
/// offsets are not supported (would require runtime division); this falls
/// back to `make_array_element_access` when the offset is a literal.
fn make_array_element_access_expr(
    lhs: &AstNode,
    flat_offset: &AstNode,
    array_info: &ArrayInfo,
    id_provider: &mut IdProvider,
) -> AstNode {
    // Fast path: if the offset is a literal, use the existing i64-based function.
    if let Some(offset_val) = flat_offset.get_literal_integer_value() {
        return make_array_element_access(lhs, offset_val as i64, array_info, id_provider);
    }

    // Non-literal offset — only supported for single-dim arrays.
    let dim_start = make_int_literal(array_info.dims[0].start, id_provider);
    let idx_expr = AstFactory::create_binary_expression(
        dim_start,
        plc_ast::ast::Operator::Plus,
        flat_offset.clone(),
        id_provider.next_id(),
    );
    AstFactory::create_index_reference(
        idx_expr,
        Some(lhs.clone()),
        id_provider.next_id(),
        SourceLocation::internal(),
    )
}

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
