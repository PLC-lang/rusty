//! Transparent mutation tracking for visitor-based lowerers.
//!
//! A visitor that may rewrite its input needs to tell the surrounding
//! pipeline whether a given unit actually changed, so partial re-index /
//! re-annotation can be scoped to just the touched units. The naive
//! pattern — bumping a counter at every mutation site — relies on the
//! author remembering to bump it; a missed increment lets a unit
//! silently skip re-indexing.
//!
//! `MutationTracker` moves the bump into the mutation primitives
//! (helpers like `push_pre_statement`, signature-rewrite helpers, etc.):
//! the call site asks the helper to do the mutation, and the helper
//! marks the tracker dirty. The lowerer reads `is_dirty()` once per
//! unit.
//!
//! Body mutations (statement inserts) and signature mutations (POU
//! interface rewrites) are tracked separately. A unit may be dirty
//! without any signature change — body-only edits should still force
//! re-indexing of that unit, but they should not invalidate every
//! caller of every POU in the unit.

use std::cell::{Cell, RefCell};

#[derive(Debug, Default)]
pub struct MutationTracker {
    dirty: Cell<bool>,
    signature_changed: RefCell<Vec<String>>,
}

impl MutationTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a body-only mutation. Marks the tracker dirty but does
    /// not flag any POU's signature as changed.
    pub fn touch(&self) {
        self.dirty.set(true);
    }

    /// Records that `pou_name`'s public signature was rewritten. Implies
    /// `touch()` — a signature change is also a mutation.
    pub fn signature_changed(&self, pou_name: impl Into<String>) {
        self.signature_changed.borrow_mut().push(pou_name.into());
        self.dirty.set(true);
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty.get()
    }

    /// Returns the names of POUs whose signatures were rewritten since
    /// the last [`reset`](Self::reset). Empty when only body mutations
    /// occurred.
    pub fn signature_changed_pous(&self) -> Vec<String> {
        self.signature_changed.borrow().clone()
    }

    pub fn reset(&self) {
        self.dirty.set(false);
        self.signature_changed.borrow_mut().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::MutationTracker;

    #[test]
    fn defaults_to_clean() {
        let t = MutationTracker::new();
        assert!(!t.is_dirty());
        assert!(t.signature_changed_pous().is_empty());
    }

    #[test]
    fn touch_marks_dirty_without_signature_change() {
        let t = MutationTracker::new();
        t.touch();
        assert!(t.is_dirty());
        assert!(t.signature_changed_pous().is_empty());
    }

    #[test]
    fn signature_changed_marks_dirty_and_records_name() {
        let t = MutationTracker::new();
        t.signature_changed("foo");
        assert!(t.is_dirty());
        assert_eq!(t.signature_changed_pous(), vec!["foo".to_string()]);
    }

    #[test]
    fn multiple_signature_changes_accumulate() {
        let t = MutationTracker::new();
        t.signature_changed("foo");
        t.signature_changed("bar.method");
        assert_eq!(t.signature_changed_pous(), vec!["foo".to_string(), "bar.method".to_string()]);
    }

    #[test]
    fn touch_is_idempotent() {
        let t = MutationTracker::new();
        t.touch();
        t.touch();
        t.touch();
        assert!(t.is_dirty());
    }

    #[test]
    fn reset_clears_dirty_and_signature_list() {
        let t = MutationTracker::new();
        t.signature_changed("foo");
        t.touch();
        t.reset();
        assert!(!t.is_dirty());
        assert!(t.signature_changed_pous().is_empty());
    }

    #[test]
    fn dirty_survives_repeated_reads() {
        let t = MutationTracker::new();
        t.touch();
        assert!(t.is_dirty());
        assert!(t.is_dirty());
    }
}
