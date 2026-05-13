//! Identification of which compilation unit owns an entry in the global
//! [`Index`](super::Index).
//!
//! Every entry merged into the global index is associated with a [`UnitId`].
//! For source-derived entries the id is the unit's position in the project's
//! source list; built-in and synthetic symbols use reserved sentinels.
//! Tracking ownership lets the pipeline drop a single unit's contributions
//! from the global index when that unit is rebuilt, without rebuilding the
//! whole index from scratch.

use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

/// Identifies the compilation unit that contributed an entry to the global
/// index. Source units get sequentially-numbered ids; the high values are
/// reserved sentinels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UnitId(u32);

impl UnitId {
    /// Reserved for built-in symbols (primitive types, built-in functions)
    /// registered before any source unit is indexed.
    pub const BUILTIN: UnitId = UnitId(u32::MAX);

    /// Reserved for symbols produced by lowering passes or generic-
    /// instantiation registration that don't belong to a single source unit.
    pub const SYNTHETIC: UnitId = UnitId(u32::MAX - 1);

    /// Used for entries imported through the legacy [`Index::import`] path
    /// that have not yet been migrated to provide an explicit owning unit.
    /// Existing tests and call sites that don't care about ownership see this
    /// value.
    pub const UNTAGGED: UnitId = UnitId(u32::MAX - 2);

    /// Constructs a [`UnitId`] for the source unit at position `idx` in the
    /// project's source list.
    ///
    /// # Panics
    /// Panics if `idx` collides with a reserved sentinel value (i.e. there
    /// are more than `u32::MAX - 3` source units).
    pub fn source(idx: usize) -> Self {
        let raw = u32::try_from(idx).expect("source unit count exceeds u32");
        assert!(raw < Self::UNTAGGED.0, "source-unit index collides with reserved sentinel");
        UnitId(raw)
    }

    pub fn raw(self) -> u32 {
        self.0
    }

    pub fn is_source(self) -> bool {
        self.0 < Self::UNTAGGED.0
    }

    pub fn is_builtin(self) -> bool {
        self == Self::BUILTIN
    }

    pub fn is_synthetic(self) -> bool {
        self == Self::SYNTHETIC
    }
}

/// The kind of symbol an [`OwnedSymbol`] refers to. Used to dispatch removal
/// to the correct map inside the index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymbolKind {
    GlobalVariable,
    GlobalInitializer,
    EnumGlobalVariable,
    Pou,
    Interface,
    Property,
    Implementation,
    Type,
    PouType,
}

/// A single symbol contributed by a unit, used by [`UnitSymbolIndex`].
///
/// `map_key` is the key under which the entry lives in its target `SymbolMap`;
/// `identifier` is a distinguishing string (typically the entry's qualified
/// name) that the removal path uses to tell entries apart when multiple units
/// have contributed under the same `map_key`. When `identifier` and `map_key`
/// are the same string the caller can pass either through both fields.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedSymbol {
    pub kind: SymbolKind,
    pub map_key: String,
    pub identifier: String,
}

impl OwnedSymbol {
    pub fn new(kind: SymbolKind, map_key: impl Into<String>, identifier: impl Into<String>) -> Self {
        OwnedSymbol { kind, map_key: map_key.into(), identifier: identifier.into() }
    }

    /// Convenience for entries whose map key already uniquely identifies them
    /// (no cross-unit overlap is possible). The `identifier` mirrors the
    /// `map_key`.
    pub fn unique(kind: SymbolKind, name: impl Into<String>) -> Self {
        let name = name.into();
        OwnedSymbol { kind, map_key: name.clone(), identifier: name }
    }
}

/// The set of symbols contributed by each [`UnitId`], inverse of the per-entry
/// `unit_id` tags. Populated by `Index::import_with_unit` and used by
/// `Index::remove_unit` to know which keys in which maps to revisit.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UnitSymbolIndex {
    entries: FxHashMap<UnitId, Vec<OwnedSymbol>>,
}

impl UnitSymbolIndex {
    /// Records that `unit` contributed `symbol`. Multiple calls with the same
    /// pair are tolerated (the symbol will be listed multiple times); callers
    /// don't need to dedup.
    pub fn record(&mut self, unit: UnitId, symbol: OwnedSymbol) {
        self.entries.entry(unit).or_default().push(symbol);
    }

    /// Returns all symbols recorded for `unit`, or an empty slice if `unit`
    /// has no recorded contributions.
    pub fn for_unit(&self, unit: UnitId) -> &[OwnedSymbol] {
        self.entries.get(&unit).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Drops and returns the recorded symbols for `unit`.
    pub fn take(&mut self, unit: UnitId) -> Vec<OwnedSymbol> {
        self.entries.remove(&unit).unwrap_or_default()
    }

    /// Returns true if no unit has any recorded symbols.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_ids_round_trip() {
        let id = UnitId::source(0);
        assert!(id.is_source());
        assert!(!id.is_builtin());
        assert!(!id.is_synthetic());
        assert_eq!(id.raw(), 0);

        let id2 = UnitId::source(42);
        assert_eq!(id2.raw(), 42);
        assert_ne!(id, id2);
    }

    #[test]
    fn sentinels_are_distinct() {
        assert_ne!(UnitId::BUILTIN, UnitId::SYNTHETIC);
        assert_ne!(UnitId::BUILTIN, UnitId::UNTAGGED);
        assert_ne!(UnitId::SYNTHETIC, UnitId::UNTAGGED);
        assert!(UnitId::BUILTIN.is_builtin());
        assert!(UnitId::SYNTHETIC.is_synthetic());
        assert!(!UnitId::BUILTIN.is_source());
        assert!(!UnitId::SYNTHETIC.is_source());
        assert!(!UnitId::UNTAGGED.is_source());
    }

    #[test]
    #[should_panic(expected = "reserved sentinel")]
    fn source_panics_for_reserved_index() {
        // UnitId::UNTAGGED is u32::MAX - 2; anything >= that is reserved.
        let _ = UnitId::source((u32::MAX - 2) as usize);
    }

    #[test]
    fn unit_symbol_index_records_and_takes() {
        let mut idx = UnitSymbolIndex::default();
        let u = UnitId::source(0);
        idx.record(u, OwnedSymbol::unique(SymbolKind::Pou, "FOO"));
        idx.record(u, OwnedSymbol::unique(SymbolKind::Type, "TBar"));
        assert_eq!(idx.for_unit(u).len(), 2);

        let taken = idx.take(u);
        assert_eq!(taken.len(), 2);
        assert!(idx.for_unit(u).is_empty());
    }
}
