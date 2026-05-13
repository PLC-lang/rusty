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

use std::cell::Cell;

#[derive(Debug, Default)]
pub struct MutationTracker {
    dirty: Cell<bool>,
}

impl MutationTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn touch(&self) {
        self.dirty.set(true);
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty.get()
    }

    pub fn reset(&self) {
        self.dirty.set(false);
    }
}

#[cfg(test)]
mod tests {
    use super::MutationTracker;

    #[test]
    fn defaults_to_clean() {
        let t = MutationTracker::new();
        assert!(!t.is_dirty());
    }

    #[test]
    fn touch_marks_dirty() {
        let t = MutationTracker::new();
        t.touch();
        assert!(t.is_dirty());
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
    fn reset_clears_dirty() {
        let t = MutationTracker::new();
        t.touch();
        t.reset();
        assert!(!t.is_dirty());
    }

    #[test]
    fn dirty_survives_repeated_reads() {
        let t = MutationTracker::new();
        t.touch();
        assert!(t.is_dirty());
        assert!(t.is_dirty());
    }
}
