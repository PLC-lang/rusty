// Copyright (c) 2022 Ghaith Hachem and Mathias Rieder

use crate::ast::{NewLines, SourceRange};
use indexmap::IndexMap;
use std::hash::Hash;

/// Location information of a Symbol in the index consisting of the line_number
/// and the detailled SourceRange information consisting the file and the range inside the source-string
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SymbolLocation {
    /// the line-number of this symbol in the source-file
    pub line_number: u32,
    /// the exact location of the symbol and the file_name
    pub source_range: SourceRange,
}

/// A multi-map implementation with a stable order of elements. When iterating
/// the keys or the values, the iterator reflects the order of insertion.
#[derive(Debug)]
pub struct SymbolMap<K, V> {
    /// internal storage of the SymbolMap that uses an *
    /// IndexMap of Vectors
    inner_map: IndexMap<K, Vec<V>>,
}

impl<K, V> Default for SymbolMap<K, V> {
    fn default() -> Self {
        Self { inner_map: Default::default() }
    }
}

impl<K, V> SymbolMap<K, V>
where
    K: Hash + Eq,
{
    /// returns the first element associated with the given key or None if
    /// this key was never associated with an element
    pub fn get(&self, key: &K) -> Option<&V> {
        self.get_all(key).and_then(|it| it.get(0))
    }

    /// returns all elements associated with the given key or None if
    /// this key was never associated with an element
    pub fn get_all(&self, key: &K) -> Option<&Vec<V>> {
        self.inner_map.get(key)
    }

    /// associates the given value with the give key. Existing associations are
    /// not overwritten, rather an additional association is added
    pub fn insert(&mut self, key: K, value: V) {
        self.inner_map.entry(key).or_default().push(value);
    }

    /// removes and returns all elements in the SymbolMap
    pub fn drain(&mut self, range: std::ops::RangeFull) -> indexmap::map::Drain<'_, K, std::vec::Vec<V>> {
        self.inner_map.drain(range)
    }

    /// inserts all given elements and associates them with the given key
    pub fn insert_many<T: IntoIterator<Item = V>>(&mut self, key: K, values: T) {
        self.inner_map.entry(key).or_default().extend(values);
    }

    /// returns an iterator over all elements key-value tuples in their order. Note that
    /// keys with `n` associated elements will emit `n` key-value tuples in the returned
    /// iterator
    pub fn elements(&self) -> impl Iterator<Item = (&'_ K, &'_ V)> {
        self.inner_map.iter().flat_map(|(k, v)| v.iter().map(move |v| (k, v)))
    }

    /// returns an iterator over the keys in the map, in their order
    pub fn keys(&self) -> impl Iterator<Item = &'_ K> {
        self.inner_map.keys()
    }

    /// returns an iterator over all entries of this map as pairs of
    /// (K, Vec<V>) in their order. While `elements(...)` emits one pair
    /// per associated value, this method emits one pair per key.
    pub fn entries(&self) -> impl Iterator<Item = (&'_ K, &'_ Vec<V>)> {
        self.inner_map.iter()
    }

    /// returns an iterator over the values in the map
    /// The order of these values may not reflect the insertion order of the
    /// values, rather the insertion order of the keys associated to the values.
    pub fn values(&self) -> impl Iterator<Item = &'_ V> {
        self.inner_map.iter().flat_map(|(_, v)| v.iter())
    }

    /// extends the map with the contents of an iterator.
    pub fn extend(&mut self, other: SymbolMap<K, V>) {
        for (k, v) in other.inner_map.into_iter() {
            self.insert_many(k, v)
        }
    }

    /// return `true` if an equivalent to key exists in the map.
    pub fn contains_key(&self, key: &K) -> bool {
        self.inner_map.contains_key(key)
    }
}

impl SymbolLocation {
    const INTERNAL_LINE: u32 = u32::max_value();
    /// creates a SymbolLocation with undefined source_range used for
    /// symbols that are created by the compiler on-the-fly.
    pub fn internal() -> SymbolLocation {
        SymbolLocation { line_number: SymbolLocation::INTERNAL_LINE, source_range: SourceRange::undefined() }
    }

    pub fn is_internal(&self) -> bool {
        self.line_number == SymbolLocation::INTERNAL_LINE && self.source_range.is_undefined()
    }
}

/// a factory to create SymbolLocations from SourceRanges, that automatically resolves
/// the line-nr attribute
pub struct SymbolLocationFactory<'a> {
    new_lines: &'a NewLines,
}

impl<'a> SymbolLocationFactory<'a> {
    /// creates a new SymbolLocationFactory with the given NewLines
    pub fn new(new_lines: &'a NewLines) -> SymbolLocationFactory<'a> {
        SymbolLocationFactory { new_lines }
    }

    /// creats a new SymbolLocation for the given source_range and automatically calculates
    /// the resulting line-number usign the source_range's start and the factory's
    /// NewLines
    pub fn create_symbol_location(&self, source_range: &SourceRange) -> SymbolLocation {
        SymbolLocation {
            source_range: source_range.clone(),
            line_number: self.new_lines.get_line_nr(source_range.get_start()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SymbolMap;

    #[test]
    fn symbol_map_stores_multiple_values() {
        // GIVEN a SymbolMap
        let mut map = SymbolMap::default();

        // WHEN i insert 3 values under the same key
        map.insert(1, 10);
        map.insert(1, 20);
        map.insert(1, 30);

        // WHEN i insert 3 values under another key
        map.insert(5, 100);
        map.insert(5, 200);
        map.insert(5, 300);

        // THEN get_all should return all 3 values associated
        assert_eq!(map.get_all(&1), Some(&vec![10, 20, 30]));
        assert_eq!(map.get_all(&5), Some(&vec![100, 200, 300]));
    }

    #[test]
    fn symbol_map_returns_the_first_associated_value() {
        // GIVEN a SymbolMap
        let mut map = SymbolMap::default();

        // WHEN i insert 3 values under the same key
        map.insert(1, 77);
        map.insert(1, 20);
        map.insert(1, 30);

        // THEN get should return the first values associated
        assert_eq!(map.get(&1), Some(&77));
    }

    #[test]
    fn symbol_map_can_add_multiple_values_at_once() {
        // GIVEN a SymbolMap
        let mut map = SymbolMap::default();

        // WHEN i insert 3 values under the same key
        map.insert_many(1, [77, 20, 30]);

        // THEN i expect all 3 values associated
        assert_eq!(map.get_all(&1), Some(&vec![77, 20, 30]));
    }

    #[test]
    fn elements_should_return_all_pairs_in_order() {
        // GIVEN a SymbolMap with some entries
        let mut map = SymbolMap::default();
        map.insert_many(1, [77, 20, 30]);
        map.insert_many(2, [100, 200, 300]);
        map.insert_many(3, [1, 2, 3]);

        // WHEN i iterate the elements
        let iter = map.elements();

        // THEN i expect all pairs in their order
        assert_eq!(
            iter.collect::<Vec<_>>(),
            vec![
                (&1, &77),
                (&1, &20),
                (&1, &30),
                (&2, &100),
                (&2, &200),
                (&2, &300),
                (&3, &1),
                (&3, &2),
                (&3, &3),
            ],
        );
    }

    #[test]
    fn values_should_return_all_values_in_order() {
        // GIVEN a SymbolMap with some entries
        let mut map = SymbolMap::default();
        map.insert_many(1, [77, 20, 30]);
        map.insert_many(2, [100, 200, 300]);
        map.insert_many(3, [1, 2, 3]);

        // WHEN i iterate the values
        let iter = map.values();

        // THEN i expect all pairs in their order
        assert_eq!(iter.collect::<Vec<_>>(), vec![&77, &20, &30, &100, &200, &300, &1, &2, &3,],);
    }

    #[test]
    fn entries_should_return_all_key_vec_pairs_in_order() {
        // GIVEN a SymbolMap with some entries
        let mut map = SymbolMap::default();
        map.insert_many(1, [77, 20, 30]);
        map.insert_many(2, [100, 200, 300]);
        map.insert_many(3, [1, 2, 3]);

        // WHEN i iterate the values
        let iter = map.entries();

        // THEN i expect all pairs in their order
        assert_eq!(
            iter.collect::<Vec<_>>(),
            vec![(&1, &vec![77, 20, 30]), (&2, &vec![100, 200, 300]), (&3, &vec![1, 2, 3])],
        );
    }
}
