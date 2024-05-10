use elsa::sync::FrozenMap;

use super::FxIndexMap;

// TODO: Rename this to CaseInsensitiveSymbolMap and also think how to make this as generic as possible such that we have a CaseInsensitiveHashMap also
#[derive(Debug)]
pub struct CachedStringSymbolMap<V> {
    inner_map: FxIndexMap<String, Vec<V>>,

    // TODO: Do we need cache-invalidation? Or can we assume the SymbolMap in later staged (e.g. resolver) will soley be used for read-operations
    /// 1:1 binding between the queried string and its equivalent lowercased key located in the items map
    pub keys: FrozenMap<String, Box<Option<String>>>,
}

impl<V> Default for CachedStringSymbolMap<V> {
    fn default() -> Self {
        Self { inner_map: FxIndexMap::default(), keys: FrozenMap::new() }
    }
}

impl<V> CachedStringSymbolMap<V> {
    pub fn new() -> Self {
        Self { inner_map: FxIndexMap::default(), keys: FrozenMap::new() }
    }

    /// associates the given value with the give key. Existing associations are
    /// not overwritten, rather an additional association is added
    pub fn insert(&mut self, key: impl Into<String> + Clone, value: V) {
        let lowercased_key = key.into().to_lowercase(); // TODO: into().to_lowercase()
        self.inner_map.entry(lowercased_key).or_default().push(value);
    }

    /// returns the first element associated with the given key or None if
    /// this key was never associated with an element
    pub fn get(&self, key: &str) -> Option<&V> {
        if let Some(cache) = self.keys.get(key) {
            // TODO: assert_eq this
            if self.inner_map.get(&key.to_lowercase()).is_some() && cache.is_none() {
                panic!("fuck")
            }

            let lowercased_key = cache.as_ref()?;
            return self.inner_map.get(lowercased_key).and_then(|it| it.first());
        }

        let lowercased_key = key.to_lowercase();
        match self.inner_map.get(&lowercased_key) {
            Some(_) => {
                self.keys.insert(key.to_string(), Box::new(Some(lowercased_key)));
                return self.get(key);
            }
            None => {
                self.keys.insert(key.to_string(), Box::new(None));
                return None;
            }
        }
    }

    /// returns all elements associated with the given key or None if
    /// this key was never associated with an element
    pub fn get_all(&self, key: &str) -> Option<&Vec<V>> {
        if let Some(cache) = self.keys.get(key) {
            // TODO: assert_eq this
            if self.inner_map.get(&key.to_lowercase()).is_some() && cache.is_none() {
                panic!("fuck");
            }

            let lowercased_key = cache.as_ref()?;
            return self.inner_map.get(lowercased_key);
        }

        let lowercased_key = key.to_lowercase();
        match self.inner_map.get(&lowercased_key) {
            Some(_) => {
                self.keys.insert(key.to_string(), Box::new(Some(lowercased_key)));
                return self.get_all(key);
            }

            None => {
                self.keys.insert(key.to_string(), Box::new(None));
                return None;
            }
        }
    }

    /// removes and returns all elements in the SymbolMap
    pub fn drain(
        &mut self,
        range: std::ops::RangeFull,
    ) -> indexmap::map::Drain<'_, String, std::vec::Vec<V>> {
        self.inner_map.drain(range)
    }

    /// inserts all given elements and associates them with the given key
    pub fn insert_many<T: IntoIterator<Item = V>>(&mut self, key: String, values: T) {
        let lowercased_key = key.to_lowercase();
        self.inner_map.entry(lowercased_key).or_default().extend(values);
    }

    // TODO: This is not used, remove?
    /// returns an iterator over all elements key-value tuples in their order. Note that
    /// keys with `n` associated elements will emit `n` key-value tuples in the returned
    /// iterator
    pub fn elements(&self) -> impl Iterator<Item = (&'_ String, &'_ V)> {
        self.inner_map.iter().flat_map(|(k, v)| v.iter().map(move |v| (k, v)))
    }

    /// returns an iterator over the keys in the map, in their order
    pub fn keys(&self) -> impl Iterator<Item = &'_ String> {
        self.inner_map.keys()
    }

    pub fn clear_cache(&mut self) {
        self.keys = FrozenMap::new();
    }

    /// returns an iterator over all entries of this map as pairs of
    /// (K, Vec<V>) in their order. While `elements(...)` emits one pair
    /// per associated value, this method emits one pair per key.
    pub fn entries(&self) -> impl Iterator<Item = (&'_ String, &'_ Vec<V>)> {
        self.inner_map.iter()
    }

    /// returns an iterator over the values in the map
    /// The order of these values may not reflect the insertion order of the
    /// values, rather the insertion order of the keys associated to the values.
    pub fn values(&self) -> impl Iterator<Item = &'_ V> {
        self.inner_map.iter().flat_map(|(_, v)| v.iter())
    }

    /// extends the map with the contents of an iterator.
    pub fn extend(&mut self, other: CachedStringSymbolMap<V>) {
        for (k, v) in other.inner_map.into_iter() {
            self.insert_many(k, v)
        }
    }

    /// return `true` if an equivalent to key exists in the map.
    pub fn contains_key(&self, key: &String) -> bool {
        self.inner_map.contains_key(&key.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use crate::index::cache::CachedStringSymbolMap;

    #[test]
    fn insert_lowercased() {
        let mut map = CachedStringSymbolMap::new();

        map.insert("FooBar", 69);
        assert_eq!(map.get("foobar"), Some(&69));
    }

    #[test]
    fn cache_internals() {
        let mut map = CachedStringSymbolMap::new();

        map.insert("foo", 77);
        map.insert("foo", 20);
        map.insert("foo", 30);

        // The "cache" is empty because no query execution happened yet
        assert_eq!(map.keys.get("foo"), None);
        assert_eq!(map.keys.get("fOo"), None);
        assert_eq!(map.keys.get("foO"), None);

        // All of these will initially have a cache-miss, update the cache (see below) and return the value if any exists
        assert_eq!(map.get("foo"), Some(&77));
        assert_eq!(map.get("fOo"), Some(&77));
        assert_eq!(map.get("foO"), Some(&77));

        // Populated cache after misses
        assert_eq!(map.keys.get("foo").unwrap().as_deref(), Some("foo"));
        assert_eq!(map.keys.get("fOo").unwrap().as_deref(), Some("foo"));
        assert_eq!(map.keys.get("foO").unwrap().as_deref(), Some("foo"));
    }

    #[test]
    fn get_all() {
        let mut map = CachedStringSymbolMap::new();

        map.insert("foo", 1);
        map.insert("foo", 2);
        map.insert("foo", 3);

        map.insert("bar", 4);
        map.insert("bar", 5);
        map.insert("bar", 6);

        assert_eq!(map.get_all("foo"), Some(&vec![1, 2, 3]));
        assert_eq!(map.get_all("bar"), Some(&vec![4, 5, 6]));
    }

    #[test]
    fn get_none() {
        let mut map = CachedStringSymbolMap::new();

        map.insert("foo", 1);
        assert_eq!(map.get("bar"), None);
    }

    // #[test]
    // fn symbol_map_can_add_multiple_values_at_once() {
    //     // GIVEN a SymbolMap
    //     let mut map = SymbolMap::default();

    //     // WHEN i insert 3 values under the same key
    //     map.insert_many(1, [77, 20, 30]);

    //     // THEN i expect all 3 values associated
    //     assert_eq!(map.get_all(&1), Some(&vec![77, 20, 30]));
    // }

    // #[test]
    // fn elements_should_return_all_pairs_in_order() {
    //     // GIVEN a SymbolMap with some entries
    //     let mut map = SymbolMap::default();
    //     map.insert_many(1, [77, 20, 30]);
    //     map.insert_many(2, [100, 200, 300]);
    //     map.insert_many(3, [1, 2, 3]);

    //     // WHEN i iterate the elements
    //     let iter = map.elements();

    //     // THEN i expect all pairs in their order
    //     assert_eq!(
    //         iter.collect::<Vec<_>>(),
    //         vec![
    //             (&1, &77),
    //             (&1, &20),
    //             (&1, &30),
    //             (&2, &100),
    //             (&2, &200),
    //             (&2, &300),
    //             (&3, &1),
    //             (&3, &2),
    //             (&3, &3),
    //         ],
    //     );
    // }

    // #[test]
    // fn values_should_return_all_values_in_order() {
    //     // GIVEN a SymbolMap with some entries
    //     let mut map = SymbolMap::default();
    //     map.insert_many(1, [77, 20, 30]);
    //     map.insert_many(2, [100, 200, 300]);
    //     map.insert_many(3, [1, 2, 3]);

    //     // WHEN i iterate the values
    //     let iter = map.values();

    //     // THEN i expect all pairs in their order
    //     assert_eq!(iter.collect::<Vec<_>>(), vec![&77, &20, &30, &100, &200, &300, &1, &2, &3,],);
    // }

    // #[test]
    // fn entries_should_return_all_key_vec_pairs_in_order() {
    //     // GIVEN a SymbolMap with some entries
    //     let mut map = SymbolMap::default();
    //     map.insert_many(1, [77, 20, 30]);
    //     map.insert_many(2, [100, 200, 300]);
    //     map.insert_many(3, [1, 2, 3]);

    //     // WHEN i iterate the values
    //     let iter = map.entries();

    //     // THEN i expect all pairs in their order
    //     assert_eq!(
    //         iter.collect::<Vec<_>>(),
    //         vec![(&1, &vec![77, 20, 30]), (&2, &vec![100, 200, 300]), (&3, &vec![1, 2, 3])],
    //     );
    // }
}
