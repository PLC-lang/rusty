//! Project-wide map from declaration to use sites — powers
//! `textDocument/references`, plus the call-hierarchy filter.
//!
//! - **Keyed by declaration `SourceLocation`.** Identity, not display.
//!   Two distinct declarations with the same name (`a.b.c` vs `x.b.c`)
//!   get distinct keys.
//! - **Built post-compile on the worker.** Walks `annotated.units`
//!   once, looks up each `AstNode`'s annotation via
//!   `position::resolve_at`, records `(declaration_location →
//!   ReferenceEntry)` pairs. Pure consumer of the existing
//!   `AnnotationMap` — no annotator hooks, no resolver changes.
//! - **Filtering.** Drop entries whose declaration or usage location
//!   is `<internal>` (synthetic node from lowering / preprocessor) or
//!   undefined.
//! - **`includeDeclaration`.** Handler prepends the declaration's own
//!   location when the LSP request asks for it; the index itself only
//!   stores uses.
//! - **`is_call` + `container_pou` (phase 11).** Per-entry metadata so
//!   call-hierarchy incoming can filter to call sites and group them
//!   by their containing POU without a second walk. Set during the
//!   single collection pass via `visit_call_statement` (operator slot)
//!   and `visit_pou` (current container) overrides.
//!
//! Trade-offs (prototype scope):
//!
//! - One full AST walk per compile. Linear in source size — cheap
//!   relative to parse/annotate. If incremental compile lands later,
//!   the same walk can be done per-unit and merged.

use std::collections::HashMap;

use plc_ast::ast::{AstId, CallStatement, DataTypeDeclaration, Implementation, Pou};
use plc_ast::visitor::{AstVisitor, Walker};
use plc_driver::pipelines::AnnotatedProject;
use plc_index::GlobalContext;
use plc_source::source_location::SourceLocation;

use crate::position::resolve_at;

const INTERNAL_FILENAME: &str = "<internal>";

/// One recorded use of a declaration.
#[derive(Debug, Clone)]
pub struct ReferenceEntry {
    pub location: SourceLocation,
    /// `true` when the AST node sits in the operator slot of a
    /// `CallStatement`. Used by call-hierarchy incoming to filter the
    /// "calls" subset of references. `false` for variable reads, type
    /// references, member accesses on the RHS of an assignment, etc.
    pub is_call: bool,
    /// `SourceLocation` of the POU whose body contains this usage —
    /// the POU's `name_location` for stable identity. `None` for
    /// usages that don't sit inside any POU body (e.g. globals'
    /// initialisers). Call-hierarchy incoming groups by this field.
    pub container_pou: Option<SourceLocation>,
}

/// Declaration `SourceLocation` → list of use sites in the project.
///
/// Owned (not `Arc`), shipped from the worker to the main thread
/// inside `CompileResult` following the D10 hand-off pattern.
#[derive(Default, Debug)]
pub struct ReverseIndex {
    by_declaration: HashMap<SourceLocation, Vec<ReferenceEntry>>,
}

impl ReverseIndex {
    /// Build the index by walking every `CompilationUnit` in `annotated`.
    pub fn build(annotated: &AnnotatedProject, ctxt: &GlobalContext) -> Self {
        let mut walker = ReferenceCollector {
            annotated,
            ctxt,
            pairs: Vec::new(),
            ancestor_stack: Vec::new(),
            container_pou: None,
            in_call_operator: false,
        };
        for unit in annotated.units.iter().map(|au| au.get_unit()) {
            unit.walk(&mut walker);
        }

        let mut by_declaration: HashMap<SourceLocation, Vec<ReferenceEntry>> = HashMap::new();
        for (decl, entry) in walker.pairs {
            by_declaration.entry(decl).or_default().push(entry);
        }
        // For each decl, drop entries whose location *encloses* another
        // entry's location in the same file. The annotator copies a
        // resolution onto both the outer `ReferenceExpr` and the inner
        // `Identifier` (`src/resolver.rs:2415-2416`), so a member
        // access like `s.myInt` records both the whole
        // `s.myInt` span and the bare `myInt` span. Find-references
        // would show two overlapping highlights; rename would emit
        // overlapping `TextEdit`s and corrupt the file. Keeping only
        // the tightest (inner) span fixes both at the source.
        for entries in by_declaration.values_mut() {
            dedupe_overlapping(entries);
        }
        ReverseIndex { by_declaration }
    }

    /// All recorded use entries for a declaration. Returns an empty
    /// slice when nothing was indexed against the given key.
    pub fn lookup(&self, declaration: &SourceLocation) -> &[ReferenceEntry] {
        self.by_declaration.get(declaration).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn is_empty(&self) -> bool {
        self.by_declaration.is_empty()
    }

    pub fn len(&self) -> usize {
        self.by_declaration.len()
    }
}

/// Drop entries whose `location` *strictly encloses* another entry's
/// `location` in the same file. The outer entry is invariably the
/// `ReferenceExpr` wrapper the resolver also annotated; the inner
/// entry is the tighter identifier span we want for hover / rename /
/// references.
///
/// Also dedupes identical locations (`==`) — same as the previous
/// behaviour for the simpler "outer and inner have the same range"
/// case.
fn dedupe_overlapping(entries: &mut Vec<ReferenceEntry>) {
    // Indices to drop. `Vec::retain` preserves the rest.
    let mut drop_idx: Vec<usize> = Vec::new();
    for (i, a) in entries.iter().enumerate() {
        let Some(file_a) = a.location.get_file_name() else { continue };
        let Some(range_a) = a.location.to_range() else { continue };
        let mut should_drop = false;
        for (j, b) in entries.iter().enumerate() {
            if i == j {
                continue;
            }
            if a.location == b.location {
                // Identical locations: keep the earlier index, drop later.
                if i > j {
                    should_drop = true;
                    break;
                }
                continue;
            }
            if file_a != b.location.get_file_name().unwrap_or("") {
                continue;
            }
            let Some(range_b) = b.location.to_range() else { continue };
            let strictly_encloses =
                range_a.start <= range_b.start && range_a.end >= range_b.end && range_a != range_b;
            if strictly_encloses {
                should_drop = true;
                break;
            }
        }
        if should_drop {
            drop_idx.push(i);
        }
    }
    if drop_idx.is_empty() {
        return;
    }
    let drop_set: std::collections::HashSet<usize> = drop_idx.into_iter().collect();
    let mut i = 0usize;
    entries.retain(|_| {
        let keep = !drop_set.contains(&i);
        i += 1;
        keep
    });
}

/// Single-pass visitor recording `(declaration → ReferenceEntry)` pairs
/// for every AST node whose annotation resolves to a known declaration.
///
/// `container_pou` and `in_call_operator` are set with strict
/// save/restore semantics around the `visit_pou` / `visit_call_statement`
/// scopes, so nested cases (e.g. a call statement whose operator is
/// itself a `CallStatement`) carry the right flag values into each
/// inner `visit`. Mirrors the existing compiler convention of
/// scope-bounded state on visitors rather than free-floating mutable
/// flags.
struct ReferenceCollector<'a> {
    annotated: &'a AnnotatedProject,
    ctxt: &'a GlobalContext,
    pairs: Vec<(SourceLocation, ReferenceEntry)>,
    ancestor_stack: Vec<AstId>,
    container_pou: Option<SourceLocation>,
    in_call_operator: bool,
}

impl AstVisitor for ReferenceCollector<'_> {
    fn visit_pou(&mut self, pou: &Pou) {
        // Sets the container while walking the POU's *header* (var
        // blocks, methods, actions, properties). Implementation bodies
        // are visited separately via `visit_implementation` (see
        // `CompilationUnit::walk`'s split between `unit.pous` and
        // `unit.implementations`).
        let prev = self.container_pou.replace(pou.name_location.clone());
        Walker::walk(pou, self);
        self.container_pou = prev;
    }

    fn visit_implementation(&mut self, implementation: &Implementation) {
        // `CompilationUnit::walk` visits POU headers (`unit.pous`) and
        // bodies (`unit.implementations`) as separate top-level loops.
        // Without this hook, container_pou would never be set for any
        // usage inside a body — incoming call hierarchy would always
        // return zero groups.
        let prev = self.container_pou.replace(implementation.name_location.clone());
        Walker::walk(implementation, self);
        self.container_pou = prev;
    }

    fn visit_call_statement(&mut self, stmt: &CallStatement, _node: &plc_ast::ast::AstNode) {
        // Mark the operator slot only — params are NOT calls of the
        // operator. Save/restore handles nested calls in the operator
        // subtree (rare but possible: `getFn()()`).
        let prev = self.in_call_operator;
        self.in_call_operator = true;
        self.visit(&stmt.operator);
        self.in_call_operator = prev;
        if let Some(params) = &stmt.parameters {
            self.visit(params);
        }
    }

    fn visit_data_type_declaration(&mut self, dt: &DataTypeDeclaration) {
        // Type references in declarations (`x : myType`, function
        // return types, …) are *uses* of a type. They're not
        // `AstNode`s, so `visit(&AstNode)` doesn't see them.
        if let DataTypeDeclaration::Reference { referenced_type, location } = dt {
            if let Some(ty) = self
                .annotated
                .index
                .find_type(referenced_type)
                .or_else(|| self.annotated.index.find_pou_type(referenced_type))
            {
                let usage_ok = location.to_range().is_some()
                    && !matches!(location.get_file_name(), Some(name) if name == INTERNAL_FILENAME);
                let decl_ok = !ty.location.is_undefined()
                    && !matches!(ty.location.get_file_name(), Some(name) if name == INTERNAL_FILENAME);
                if usage_ok && decl_ok && ty.location != *location {
                    self.pairs.push((
                        ty.location.clone(),
                        ReferenceEntry {
                            location: location.clone(),
                            // A type reference is never a call.
                            is_call: false,
                            container_pou: self.container_pou.clone(),
                        },
                    ));
                }
            }
        }
        Walker::walk(dt, self);
    }

    fn visit(&mut self, node: &plc_ast::ast::AstNode) {
        // Skip emitting for synthetic / `<internal>` nodes. Still
        // descend, because lowering wraps real user code in location-
        // less wrappers (the synthetic `ExpressionList` around call
        // params) — pruning here would lose every named/positional
        // argument reference in the project.
        let usage_has_range = node.location.to_range().is_some();
        let usage_file_ok = !matches!(node.location.get_file_name(), Some(name) if name == INTERNAL_FILENAME);

        if usage_has_range && usage_file_ok {
            let usage_text = self.ctxt.slice(&node.location);
            if let Some(resolved) = resolve_at(node.id, &usage_text, &self.ancestor_stack, self.annotated) {
                let decl_ok = !resolved.declaration_location.is_undefined()
                    && !matches!(
                        resolved.declaration_location.get_file_name(),
                        Some(name) if name == INTERNAL_FILENAME
                    );
                // Skip self-references (the declaration site referring
                // to itself). The handler reinserts the declaration
                // when `includeDeclaration: true`.
                if decl_ok && resolved.declaration_location != node.location {
                    self.pairs.push((
                        resolved.declaration_location,
                        ReferenceEntry {
                            location: node.location.clone(),
                            is_call: self.in_call_operator,
                            container_pou: self.container_pou.clone(),
                        },
                    ));
                }
            }
        }

        self.ancestor_stack.push(node.id);
        node.walk(self);
        self.ancestor_stack.pop();
    }
}
