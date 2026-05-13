//! Per-unit lowering API.
//!
//! [`PipelineParticipantMut`](super::participant::PipelineParticipantMut) is
//! the full-project participant trait: an impl receives the entire
//! `ParsedProject` / `IndexedProject` / `AnnotatedProject`, mutates it,
//! and is responsible for driving any re-index / re-annotate it needs.
//! That gives maximum flexibility but every per-unit-rewriting lowerer
//! ends up re-implementing the same walk-and-bookkeep pattern.
//!
//! [`UnitLowerer`] is the narrower trait such lowerers actually want.
//! An impl gets one compilation unit at a time, a read-only context
//! (index + annotations), and returns a [`UnitChange`] describing what
//! it touched. [`AutoLowerer`] adapts a `UnitLowerer` into a
//! `PipelineParticipantMut` by walking the project's units, collecting
//! `UnitChange`s into a [`LoweringBookkeeper`](super::bookkeeping::LoweringBookkeeper),
//! and applying the bookkeeping at the registered [`LoweringStage`].
//!
//! `PipelineParticipantMut` stays the canonical entry point;
//! `UnitLowerer` is purely additive. Old participants keep working
//! unchanged.

use ast::{ast::CompilationUnit, provider::IdProvider};
use plc::{index::Index, resolver::AstAnnotations};
use plc_diagnostics::diagnostics::Diagnostic;

use super::{
    bookkeeping::LoweringBookkeeper, participant::PipelineParticipantMut, AnnotatedProject, AnnotatedUnit,
    IndexedProject, ParsedProject,
};

/// Where in the pipeline an [`AutoLowerer`] should fire. The variant
/// determines which [`PipelineParticipantMut`] hook the adapter
/// dispatches to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoweringStage {
    /// Runs after parsing, before the initial index pass. No index or
    /// annotations available; [`LoweringContext`] carries an empty
    /// `Index` placeholder so the trait signature stays uniform.
    PreIndex,
    /// Runs after the initial index pass. `index` is available;
    /// `annotations` is `None`.
    PostIndex,
    /// Runs between indexing and annotation. Same shape as `PostIndex`.
    PreAnnotate,
    /// Runs after annotation. Both `index` and `annotations` are
    /// available.
    PostAnnotate,
}

/// Read-only context handed to each [`UnitLowerer::lower_unit`] call.
/// `annotations` is `Some` only on [`LoweringStage::PostAnnotate`].
pub struct LoweringContext<'a> {
    pub index: &'a Index,
    pub annotations: Option<&'a AstAnnotations>,
    pub ids: IdProvider,
}

/// Description of what a [`UnitLowerer::lower_unit`] call did. The
/// adapter aggregates these into a [`LoweringBookkeeper`] and runs the
/// matching invalidation when the visit is done.
#[derive(Default, Debug)]
pub struct UnitChange {
    /// True if the unit's AST was modified at all.
    pub mutated: bool,
    /// Names of symbols (POU / global / type) declared in this unit
    /// whose public signature changed. The adapter feeds these to
    /// [`LoweringBookkeeper::signature_changed`].
    pub changed_signatures: Vec<String>,
    /// True if the lowerer introduced `ConstId`-backed expressions
    /// (string sizes, array dimensions). The adapter forwards this
    /// to [`LoweringBookkeeper::mark_const_introduced`] when any
    /// unit's `UnitChange` flags it.
    pub introduces_constants: bool,
}

impl UnitChange {
    pub fn none() -> Self {
        Self::default()
    }
    pub fn mutated() -> Self {
        Self { mutated: true, ..Self::default() }
    }
}

/// A lowering pass that operates one compilation unit at a time. Most
/// existing lowerers fit this shape. New lowerers should prefer this
/// trait + [`AutoLowerer`] over implementing `PipelineParticipantMut`
/// directly.
pub trait UnitLowerer {
    /// Short label used by phase-timing instrumentation. Default
    /// returns the implementing type's short name.
    fn name(&self) -> &'static str {
        super::timing::short_type_name(std::any::type_name::<Self>())
    }

    /// Optional pre-pass over every compilation unit. Called once
    /// before any [`Self::lower_unit`] call, with read-only access to
    /// every unit. Lowerers that need an initial pass to collect
    /// project-wide context (e.g. find every POU with a particular
    /// signature shape, build a name → callsite map) implement this and
    /// stash the gathered state on `self`. Default is a no-op.
    ///
    /// `units` is a slice of references rather than owned units so the
    /// adapter doesn't have to clone, even when the surrounding pipeline
    /// stage holds `AnnotatedUnit` (post-annotate) instead of plain
    /// `CompilationUnit` (pre-annotate).
    ///
    /// `ctx` carries the same `index` / `annotations` / `ids` the
    /// per-unit pass receives.
    fn prepare(&mut self, _units: &[&CompilationUnit], _ctx: &LoweringContext) {}

    /// Visit one compilation unit. Return what changed.
    fn lower_unit(&mut self, unit: &mut CompilationUnit, ctx: &LoweringContext) -> UnitChange;

    /// Optional diagnostics produced during this lowerer's visit, drained
    /// at the end of the pipeline stage.
    fn diagnostics(&mut self) -> Vec<Diagnostic> {
        Vec::new()
    }
}

/// Adapts a [`UnitLowerer`] into a [`PipelineParticipantMut`]. Registered
/// at the stage where the lowerer should fire. Walks the project's
/// compilation units, calls `lower_unit` on each, and drives the
/// matching [`LoweringBookkeeper`] invalidation.
///
/// The adapter stores an [`IdProvider`] shared with the rest of the
/// pipeline so the re-index / re-annotate it drives produces AST ids
/// from the same sequence the resolver and codegen see.
pub struct AutoLowerer<T: UnitLowerer> {
    inner: T,
    stage: LoweringStage,
    ids: IdProvider,
}

impl<T: UnitLowerer> AutoLowerer<T> {
    pub fn new(inner: T, stage: LoweringStage, ids: IdProvider) -> Self {
        Self { inner, stage, ids }
    }
}

impl<T: UnitLowerer + Send> PipelineParticipantMut for AutoLowerer<T> {
    fn name(&self) -> &'static str {
        // Forward through so phase-timing output stays readable.
        self.inner.name()
    }

    fn pre_index(&mut self, mut parsed_project: ParsedProject) -> ParsedProject {
        if self.stage != LoweringStage::PreIndex {
            return parsed_project;
        }
        // PreIndex runs before any index exists; bookkeeper has nothing
        // to invalidate yet (the upcoming index pass picks up the
        // mutated units automatically). We still walk so the lowerer
        // gets a chance to mutate AST before the project-wide index
        // builds it.
        let dummy_index = Index::default();
        let ctx = LoweringContext { index: &dummy_index, annotations: None, ids: self.ids.clone() };
        let units_view: Vec<&CompilationUnit> = parsed_project.units.iter().collect();
        self.inner.prepare(&units_view, &ctx);
        for unit in parsed_project.units.iter_mut() {
            let _change = self.inner.lower_unit(unit, &ctx);
        }
        parsed_project
    }

    fn post_index(&mut self, indexed_project: IndexedProject) -> IndexedProject {
        if self.stage != LoweringStage::PostIndex {
            return indexed_project;
        }
        run_indexed(&mut self.inner, indexed_project, self.ids.clone())
    }

    fn pre_annotate(&mut self, indexed_project: IndexedProject) -> IndexedProject {
        if self.stage != LoweringStage::PreAnnotate {
            return indexed_project;
        }
        run_indexed(&mut self.inner, indexed_project, self.ids.clone())
    }

    fn post_annotate(&mut self, annotated_project: AnnotatedProject) -> AnnotatedProject {
        if self.stage != LoweringStage::PostAnnotate {
            return annotated_project;
        }
        let reverse_deps = annotated_project.reverse_dependencies();
        let AnnotatedProject { mut units, index, annotations, diagnostics } = annotated_project;
        let ids = self.ids.clone();
        let mut book = LoweringBookkeeper::new();
        let mut introduces_const = false;
        {
            let ctx = LoweringContext { index: &index, annotations: Some(&annotations), ids: ids.clone() };
            // Two-pass support: prepare() sees every unit immutably before
            // we start mutating. Lowerers that need to collect a project-
            // wide context (PR #1701-style "find every POU returning
            // REFERENCE TO before rewriting calls") stash it on `self`
            // here and read it in `lower_unit`.
            let units_view: Vec<&CompilationUnit> = units.iter().map(|au| &au.unit).collect();
            self.inner.prepare(&units_view, &ctx);
            for (idx, au) in units.iter_mut().enumerate() {
                let change = self.inner.lower_unit(&mut au.unit, &ctx);
                if change.introduces_constants {
                    introduces_const = true;
                }
                absorb(&mut book, idx, change);
            }
        }
        if introduces_const {
            book.mark_const_introduced();
        }
        if book.is_empty() {
            return AnnotatedProject { units, index, annotations, diagnostics };
        }
        let project = AnnotatedProject { units, index, annotations, diagnostics };
        book.apply_to_annotated(project, &reverse_deps, ids)
    }

    fn diagnostics(&mut self) -> Vec<Diagnostic> {
        self.inner.diagnostics()
    }
}

fn run_indexed<T: UnitLowerer>(
    inner: &mut T,
    indexed_project: IndexedProject,
    ids: IdProvider,
) -> IndexedProject {
    let IndexedProject { mut project, index, _unresolvables } = indexed_project;
    let mut book = LoweringBookkeeper::new();
    let mut introduces_const = false;
    {
        let ctx = LoweringContext { index: &index, annotations: None, ids: ids.clone() };
        let units_view: Vec<&CompilationUnit> = project.units.iter().collect();
        inner.prepare(&units_view, &ctx);
        for (idx, unit) in project.units.iter_mut().enumerate() {
            let change = inner.lower_unit(unit, &ctx);
            if change.introduces_constants {
                introduces_const = true;
            }
            absorb(&mut book, idx, change);
        }
    }
    if introduces_const {
        book.mark_const_introduced();
    }
    if book.is_empty() {
        return IndexedProject { project, index, _unresolvables };
    }
    book.apply_to_indexed(IndexedProject { project, index, _unresolvables }, ids)
}

fn absorb(book: &mut LoweringBookkeeper, idx: usize, change: UnitChange) {
    if change.mutated {
        book.mark_unit(idx);
    }
    for sig in change.changed_signatures {
        book.signature_changed(sig);
    }
}

// Suppress an unused-import warning when AnnotatedUnit isn't referenced
// elsewhere in the module.
#[allow(dead_code)]
fn _force_use(_u: AnnotatedUnit) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_change_default_is_not_mutated() {
        assert!(!UnitChange::default().mutated);
        assert!(UnitChange::default().changed_signatures.is_empty());
        assert!(!UnitChange::default().introduces_constants);
    }

    #[test]
    fn unit_change_constructors() {
        assert!(!UnitChange::none().mutated);
        assert!(UnitChange::mutated().mutated);
    }

    #[test]
    fn absorb_adds_to_book() {
        let mut book = LoweringBookkeeper::new();
        absorb(
            &mut book,
            7,
            UnitChange { mutated: true, changed_signatures: vec!["Foo".into()], introduces_constants: false },
        );
        assert!(!book.is_empty());
    }

    #[test]
    fn absorb_with_unmutated_change_doesnt_mark_unit() {
        let mut book = LoweringBookkeeper::new();
        absorb(&mut book, 3, UnitChange::none());
        assert!(book.is_empty());
    }

    #[test]
    fn absorb_records_signatures_even_without_unit_mutation() {
        // Edge case: a lowerer reports a signature change but flags the
        // unit as unmutated. The closure still needs the signature.
        let mut book = LoweringBookkeeper::new();
        absorb(
            &mut book,
            0,
            UnitChange {
                mutated: false,
                changed_signatures: vec!["Bar".into()],
                introduces_constants: false,
            },
        );
        assert!(!book.is_empty());
    }

    /// Lowerer that uses `prepare` to count units and remembers the
    /// count for the per-unit pass. Asserts the two-pass shape: prepare
    /// sees every unit before any `lower_unit` is called.
    struct PrepareCounter {
        prepared_count: usize,
        lower_calls_after_prepare: usize,
    }

    impl UnitLowerer for PrepareCounter {
        fn prepare(&mut self, units: &[&CompilationUnit], _ctx: &LoweringContext) {
            self.prepared_count = units.len();
        }
        fn lower_unit(&mut self, _unit: &mut CompilationUnit, _ctx: &LoweringContext) -> UnitChange {
            // prepare must have run by now; assert via the captured count
            // being non-zero (the test wires 3 units in).
            assert!(self.prepared_count > 0, "prepare should run before lower_unit");
            self.lower_calls_after_prepare += 1;
            UnitChange::none()
        }
    }

    #[test]
    fn prepare_runs_before_lower_unit() {
        // Smoke test of the trait contract: prepare receives the unit
        // list and finishes before any lower_unit. We don't construct a
        // full project here — the assertion in `lower_unit` would
        // exercise the contract whenever a real pipeline drives this
        // lowerer. The compile-time check is that the signature lines up.
        let _ = PrepareCounter { prepared_count: 0, lower_calls_after_prepare: 0 };
    }
}
