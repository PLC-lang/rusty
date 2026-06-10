use std::hash::BuildHasherDefault;

use rustc_hash::FxHasher;

/// An [`indexmap::IndexMap`] using the fast [`FxHasher`], analogous to `FxHashMap`.
pub type FxIndexMap<K, V> = indexmap::IndexMap<K, V, BuildHasherDefault<FxHasher>>;

/// An [`indexmap::IndexSet`] using the fast [`FxHasher`], analogous to `FxHashSet`.
pub type FxIndexSet<K> = indexmap::IndexSet<K, BuildHasherDefault<FxHasher>>;
