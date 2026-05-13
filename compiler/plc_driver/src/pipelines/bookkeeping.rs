//! Bookkeeping helper for lowering participants.
//!
//! A lowering pass that mutates compilation units has to invalidate
//! downstream state in lockstep: re-index the units it touched, evaluate
//! any new `ConstId`-backed expressions, compute the reverse-dependency
//! closure of signature changes, and partial-re-annotate the closure.
//! Every per-unit-reindexing participant added in Phases 3–4 ran a
//! variant of that recipe by hand.
//!
//! [`LoweringBookkeeper`] centralises the recipe: a participant tells it
//! what it mutated and which symbols' public signatures changed, then
//! hands the project off via [`LoweringBookkeeper::apply_to_indexed`] or
//! [`LoweringBookkeeper::apply_to_annotated`]. The hidden invariants
//! (per-unit reindex order, `evaluate_constants` placement, closure
//! computation against the *pre-mutation* reverse-dep graph,
//! [`UnitId::SYNTHETIC`] tagging of the resolver's `new_index`) live in
//! one place rather than five.

use std::collections::BTreeSet;

use ast::provider::IdProvider;
use plc::index::UnitId;

use super::{AnnotatedProject, IndexedProject, ReverseDependencyGraph};

/// Accumulates the per-unit effects of a lowering pass and applies the
/// matching invalidation when the visit is done.
#[derive(Default, Debug)]
pub struct LoweringBookkeeper {
    /// Positional indices of compilation units the lowerer mutated.
    /// Coalesced so a unit marked twice is only re-indexed once.
    mutated_units: BTreeSet<usize>,
    /// Symbol names (POU / global / type) whose public signature changed.
    /// Used to expand the re-annotation closure via the reverse-dep graph.
    changed_signatures: Vec<String>,
    /// True if the lowerer introduced `ConstId`-backed expressions
    /// (string sizes, array dimensions) that need a fresh
    /// `evaluate_constants` pass after the per-unit reindex.
    introduces_constants: bool,
}

impl LoweringBookkeeper {
    pub fn new() -> Self {
        Self::default()
    }

    /// Records that the compilation unit at positional `idx` was mutated.
    pub fn mark_unit(&mut self, idx: usize) {
        self.mutated_units.insert(idx);
    }

    /// Records that `name`'s public signature changed. The reannotation
    /// closure in [`Self::apply_to_annotated`] looks `name` up in the
    /// reverse-dep graph and adds every dependent unit.
    pub fn signature_changed(&mut self, name: impl Into<String>) {
        self.changed_signatures.push(name.into());
    }

    /// Flags that the lowerer introduced new `ConstId`-backed expressions
    /// (e.g. a `STRING[K+1]` derived from a freshly synthesised type).
    /// Apply will run `const_evaluator::evaluate_constants` once after
    /// the per-unit reindex.
    pub fn mark_const_introduced(&mut self) {
        self.introduces_constants = true;
    }

    /// True if no unit was marked and no signature change recorded. A
    /// participant that calls only this can skip the apply step entirely.
    pub fn is_empty(&self) -> bool {
        self.mutated_units.is_empty() && self.changed_signatures.is_empty()
    }

    /// Drives invalidation against an [`IndexedProject`]. Used by
    /// participants that mutate in `pre_index`, `post_index`, or
    /// `pre_annotate` — anything that runs before the resolver. Re-indexes
    /// only the units in `mutated_units`; runs `evaluate_constants` if
    /// flagged.
    pub fn apply_to_indexed(self, indexed: IndexedProject, ids: IdProvider) -> IndexedProject {
        let IndexedProject { mut project, mut index, _unresolvables } = indexed;

        for &idx in &self.mutated_units {
            let unit_id = UnitId::source(idx);
            index.reindex_unit(unit_id, &mut project.units[idx], ids.clone());
        }

        if self.introduces_constants {
            let (next_index, _unresolved) = plc::resolver::const_evaluator::evaluate_constants(index);
            return IndexedProject { project, index: next_index, _unresolvables };
        }

        IndexedProject { project, index, _unresolvables }
    }

    /// Drives invalidation against an [`AnnotatedProject`]. Used by
    /// `post_annotate` participants. Re-indexes mutated units, evaluates
    /// constants if flagged, computes the reverse-dep closure from
    /// `reverse_deps_pre`, and partial-reannotates the closure in
    /// parallel through [`AnnotatedProject::reannotate_units`].
    ///
    /// `reverse_deps_pre` must be captured *before* the lowerer's
    /// mutation — the closure asks "who used the symbol before it
    /// changed?", not after.
    pub fn apply_to_annotated(
        self,
        project: AnnotatedProject,
        reverse_deps_pre: &ReverseDependencyGraph,
        ids: IdProvider,
    ) -> AnnotatedProject {
        let AnnotatedProject { mut units, mut index, annotations, diagnostics } = project;

        for &idx in &self.mutated_units {
            let unit_id = UnitId::source(idx);
            index.reindex_unit(unit_id, &mut units[idx].unit, ids.clone());
        }

        if self.introduces_constants {
            let (next_index, _unresolved) = plc::resolver::const_evaluator::evaluate_constants(index);
            index = next_index;
        }

        // Closure = mutated units ∪ every unit that depended on a
        // signature we changed. BTreeSet dedups and gives a stable order
        // for the parallel re-annotate.
        let mut closure: BTreeSet<usize> = self.mutated_units.clone();
        for sig in &self.changed_signatures {
            if let Some(dependents) = reverse_deps_pre.dependents(sig) {
                for unit_id in dependents {
                    if let Some(idx) = unit_id.source_index() {
                        closure.insert(idx);
                    }
                }
            }
        }
        let closure: Vec<usize> = closure.into_iter().collect();

        let mut project = AnnotatedProject { units, index, annotations, diagnostics: Vec::new() };
        project.reannotate_units(&closure, ids);
        project.diagnostics = diagnostics;
        project
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_empty() {
        let b = LoweringBookkeeper::new();
        assert!(b.is_empty());
    }

    #[test]
    fn mark_unit_makes_non_empty() {
        let mut b = LoweringBookkeeper::new();
        b.mark_unit(0);
        assert!(!b.is_empty());
    }

    #[test]
    fn signature_change_makes_non_empty_even_without_unit() {
        let mut b = LoweringBookkeeper::new();
        b.signature_changed("Foo");
        assert!(!b.is_empty());
    }

    #[test]
    fn const_flag_without_other_changes_is_empty() {
        // A pass that only flags const-introduction without actually
        // mutating anything has nothing to invalidate. The flag matters
        // only when paired with mutated units; treat it as effectively
        // empty for the gating check.
        let mut b = LoweringBookkeeper::new();
        b.mark_const_introduced();
        assert!(b.is_empty());
    }

    #[test]
    fn mark_unit_dedups() {
        let mut b = LoweringBookkeeper::new();
        b.mark_unit(3);
        b.mark_unit(3);
        b.mark_unit(1);
        assert_eq!(b.mutated_units.iter().copied().collect::<Vec<_>>(), vec![1, 3]);
    }
}
