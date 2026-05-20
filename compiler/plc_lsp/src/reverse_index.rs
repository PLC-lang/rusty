//! Project-wide map from declaration to use sites — powers
//! `textDocument/references`.
//!
//! Design (cf. `.baseline/lsp-plan-phases-7-10.md` §3.4):
//!
//! - **Keyed by declaration `SourceLocation`.** Identity, not display
//!   (§3.5). Two distinct declarations with the same name (`a.b.c` vs
//!   `x.b.c`) get distinct keys.
//! - **Built post-compile on the worker.** Walks `annotated.units`
//!   once, looks up each `AstNode`'s annotation via
//!   `position::resolve_at`, records `(declaration_location →
//!   usage_location)` pairs. Pure consumer of the existing
//!   `AnnotationMap` — no annotator hooks, no resolver changes.
//! - **Filtering.** Drop entries whose declaration or usage location
//!   is `<internal>` (synthetic node from lowering / preprocessor) or
//!   undefined.
//! - **`includeDeclaration`.** Handler prepends the declaration's own
//!   location when the LSP request asks for it; the index itself only
//!   stores uses.
//!
//! Trade-offs (prototype scope):
//!
//! - One full AST walk per compile. Linear in source size — cheap
//!   relative to parse/annotate. If incremental compile lands later,
//!   the same walk can be done per-unit and merged.
//! - No deduplication of identical usage locations. The reverse-index
//!   walker visits each node once and locations within a single
//!   compile are unique, so dedup wasn't necessary; might matter if
//!   the strategy changes.

use std::collections::HashMap;

use plc_ast::ast::AstId;
use plc_ast::visitor::{AstVisitor, Walker};
use plc_driver::pipelines::AnnotatedProject;
use plc_index::GlobalContext;
use plc_source::source_location::SourceLocation;

use crate::position::resolve_at;

const INTERNAL_FILENAME: &str = "<internal>";

/// Declaration `SourceLocation` → list of use sites in the project.
///
/// Owned (not `Arc`), shipped from the worker to the main thread
/// inside `CompileResult` following the D10 hand-off pattern.
#[derive(Default, Debug)]
pub struct ReverseIndex {
    by_declaration: HashMap<SourceLocation, Vec<SourceLocation>>,
}

impl ReverseIndex {
    /// Build the index by walking every `CompilationUnit` in `annotated`.
    /// Source-text lookups go through `ctxt` so the ancestor-context
    /// fallback can produce a `usage_text` for member resolution
    /// (struct-literal initializers, etc.).
    pub fn build(annotated: &AnnotatedProject, ctxt: &GlobalContext) -> Self {
        let mut walker =
            ReferenceCollector { annotated, ctxt, pairs: Vec::new(), ancestor_stack: Vec::new() };
        for unit in annotated.units.iter().map(|au| au.get_unit()) {
            unit.walk(&mut walker);
        }

        let mut by_declaration: HashMap<SourceLocation, Vec<SourceLocation>> = HashMap::new();
        for (decl, usage) in walker.pairs {
            let entries = by_declaration.entry(decl).or_default();
            // Same identifier often gets annotated on both an outer
            // `ReferenceExpr` and its inner `Identifier` (resolver
            // copies the annotation onto both), so we'd otherwise emit
            // each use twice. Dedup linearly — usage lists per decl
            // stay small.
            if !entries.contains(&usage) {
                entries.push(usage);
            }
        }
        ReverseIndex { by_declaration }
    }

    /// All recorded use sites for a declaration. Returns an empty slice
    /// when nothing was indexed against the given key (file outside
    /// the project, declaration only referenced from synthetic code,
    /// or the cursor was on a kind we don't track yet).
    pub fn lookup(&self, declaration: &SourceLocation) -> &[SourceLocation] {
        self.by_declaration.get(declaration).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn is_empty(&self) -> bool {
        self.by_declaration.is_empty()
    }

    pub fn len(&self) -> usize {
        self.by_declaration.len()
    }
}

/// Single-pass visitor that records `(declaration → usage)` pairs for
/// every AST node whose annotation resolves to a known declaration.
struct ReferenceCollector<'a> {
    annotated: &'a AnnotatedProject,
    ctxt: &'a GlobalContext,
    pairs: Vec<(SourceLocation, SourceLocation)>,
    ancestor_stack: Vec<AstId>,
}

impl AstVisitor for ReferenceCollector<'_> {
    fn visit_data_type_declaration(&mut self, dt: &plc_ast::ast::DataTypeDeclaration) {
        // Type references in declarations (`x : myType`, function
        // return types, …) are *uses* of a type. They live as
        // `String + SourceLocation` on `DataTypeDeclaration::Reference`
        // — not as `AstNode`s — so the `visit(&AstNode)` path above
        // doesn't see them. Without this hook the reverse-index has
        // no entries for type names, and find-references at a TYPE
        // declaration returns nothing.
        if let plc_ast::ast::DataTypeDeclaration::Reference { referenced_type, location } = dt {
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
                    self.pairs.push((ty.location.clone(), location.clone()));
                }
            }
        }
        plc_ast::visitor::Walker::walk(dt, self);
    }

    fn visit(&mut self, node: &plc_ast::ast::AstNode) {
        // Skip emitting for synthetic / `<internal>` nodes — they
        // can't sensibly appear in a user-facing references list. We
        // still descend, because lowering participants rewrap real
        // user code in location-less wrappers (e.g. CallStatement
        // parameters get wrapped in a synthetic ExpressionList);
        // pruning here would lose every named/positional argument
        // reference in the project.
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
                    self.pairs.push((resolved.declaration_location, node.location.clone()));
                }
            }
        }

        self.ancestor_stack.push(node.id);
        node.walk(self);
        self.ancestor_stack.pop();
    }
}
