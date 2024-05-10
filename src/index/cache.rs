// (foo, None)
// (foo, Some(a))

use crate::index::FxIndexMap;
use elsa::sync::FrozenMap;
use std::fmt::Formatter;

pub struct CaseInsensitiveSymbolMap<V> {
    items: FxIndexMap<String, Vec<V>>,

    // ("FOO", "foo"),
    // ("Foo", "foo"),
    // ("bar", None)
    keys: FrozenMap<String, Box<Option<String>>>,
}

impl<V> Default for CaseInsensitiveSymbolMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> std::fmt::Debug for CaseInsensitiveSymbolMap<V>
where
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let CaseInsensitiveSymbolMap { items, .. } = self;
        write!(f, "{items:#?}")
    }
}

impl<V> CaseInsensitiveSymbolMap<V> {
    pub fn new() -> Self {
        Self { items: FxIndexMap::default(), keys: FrozenMap::new() }
    }

    // TODO: It would make sense to directly populate the `keys` field
    pub fn insert(&mut self, key: &str, value: V) {
        // let key_lowered = key.to_lowercase();
        // self.items.entry(key_lowered).or_default().push(value);
        self.insert_many(key, vec![value]);
    }

    pub fn insert_many<T>(&mut self, key: &str, values: T)
    where
        T: IntoIterator<Item = V>,
    {
        let key_lowered = key.to_lowercase();
        self.items.entry(key_lowered).or_default().extend(values)
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        self.get_all(key)?.first()
    }

    pub fn get_all(&self, key: &str) -> Option<&Vec<V>> {
        // if key == "test__T" {
        //     println!("debug me");
        // }
        if let Some(entry) = self.keys.get(key) {
            // if cfg!(debug_assertions) {
            //     if self.items.get(&key.to_lowercase()).is_some() && entry.is_none() {
            //         panic!("fucking hell ({key}, {entry:?})")
            //     }
            // }
            return self.items.get(entry.as_ref()?);
        }

        let key_lowered = key.to_lowercase();
        if self.items.get(&key_lowered).is_some() {
            self.keys.insert(key.to_string(), Box::new(Some(key_lowered)));
            return self.get_all(key); // Do we want recursion here?
        }

        self.keys.insert(key.to_string(), Box::new(None));
        return None;
    }

    // ------------------

    pub fn drain(&mut self, range: std::ops::RangeFull) -> indexmap::map::Drain<'_, String, Vec<V>> {
        self.items.drain(range)
    }

    pub fn keys(&self) -> impl Iterator<Item = &'_ String> {
        self.items.keys()
    }

    pub fn entries(&self) -> impl Iterator<Item = (&'_ String, &'_ Vec<V>)> {
        self.items.iter()
    }

    pub fn values(&self) -> impl Iterator<Item = &'_ V> {
        self.items.iter().flat_map(|(_, v)| v.iter())
    }

    pub fn contains_key(&self, key: &str) -> bool {
        let key_lowered = key.to_lowercase();
        self.items.contains_key(&key_lowered)
    }
    //
    // pub fn get_cache(&self) -> &FrozenMap<String, Box<Option<String>>> {
    //     &self.keys
    // }
    //
    // pub fn extend_cache(&self, key: String, value: Box<Option<String>>) {
    //     dbg!((&key, &value), &self.keys.get(&key));
    //     if value.is_some() && self.keys.get(&key).is_some_and(|value| value.is_none()) {
    //         self.keys.insert(key, value);
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use crate::index::cache::CaseInsensitiveSymbolMap;

    #[test]
    fn write() {
        let mut map = CaseInsensitiveSymbolMap::new();
        map.insert("foo", 1);
        map.insert("bar", 2);
        map.insert("bar", 3);

        // Assert that the "cache" is empty because no read-access happened yet
        assert!(map.keys.clone().into_tuple_vec().is_empty());

        // Assert that the inner map is populated correctly
        assert_eq!(map.get("foo"), Some(&1));
        assert_eq!(map.get("bar"), Some(&2));
        assert_eq!(map.get_all("foo"), Some(&vec![1]));
        assert_eq!(map.get_all("bar"), Some(&vec![2, 3]));

        // Assert that the "cache" is populated by the previous input values
        assert_eq!(map.keys.get("foo").unwrap().as_deref(), Some("foo"));
        assert_eq!(map.keys.get("bar").unwrap().as_deref(), Some("bar"));
        assert_eq!(map.keys.into_tuple_vec().len(), 2);
    }

    #[test]
    fn read_write_case_insensitive() {
        let mut map = CaseInsensitiveSymbolMap::new();
        map.insert("foo", 1);
        map.insert("Foo", 2);
        map.insert("FOo", 3);
        map.insert("FOO", 4);

        // Assert that the inner map is populated with its lower-cased string value
        assert_eq!(map.items.get("foo"), Some(&vec![1, 2, 3, 4]));

        // Assert that the "cache" is empty because no read-access happened yet
        assert!(map.keys.clone().into_tuple_vec().is_empty());

        // Assert that the read-access delivers correct results regardless of the input strings case
        assert_eq!(map.get_all("foo"), Some(&vec![1, 2, 3, 4]));
        assert_eq!(map.get_all("Foo"), Some(&vec![1, 2, 3, 4]));
        assert_eq!(map.get_all("FOo"), Some(&vec![1, 2, 3, 4]));
        assert_eq!(map.get_all("FOO"), Some(&vec![1, 2, 3, 4]));

        // Assert that the "cache" is populated by the previous input values
        assert_eq!(map.keys.get("foo").unwrap().as_deref(), Some("foo"));
        assert_eq!(map.keys.get("Foo").unwrap().as_deref(), Some("foo"));
        assert_eq!(map.keys.get("FOo").unwrap().as_deref(), Some("foo"));
        assert_eq!(map.keys.get("FOO").unwrap().as_deref(), Some("foo"));
        assert_eq!(map.keys.into_tuple_vec().len(), 4);
    }

    #[test]
    fn temp() {
        let mut map = CaseInsensitiveSymbolMap::new();
        map.get("FOO");
        assert_eq!(*map.keys.get("FOO").unwrap(), None);

        map.insert("foo", 1);
        // FIXME: This should now be Some("foo") but it's None
        assert_ne!(map.get("FOO"), None, "cache invalid");
    }
}
