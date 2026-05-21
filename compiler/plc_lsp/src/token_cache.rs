//! Per-file cache of the lexer-with-trivia output, keyed by path.
//!
//! Stored on `ServerState`. Callers retrieve tokens via `get_or_recompute`,
//! passing the current source contents; if the hash matches the cached
//! entry the previously computed `Vec<LspToken>` is returned unchanged.
//! Mismatched hash (or no entry) triggers a fresh lex pass.
//!
//! Hash collision risk is the standard 1-in-2^64 from `FxHasher`; for an
//! editor cache that's acceptable. If a collision ever silently masks a
//! real edit, the user can save the file (which re-publishes diagnostics
//! through the regular compile pipeline) to force the cache to refresh.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use plc::lexer::{lex_with_trivia, LspToken};
use rustc_hash::{FxHashMap, FxHasher};
use std::hash::{Hash, Hasher};

#[derive(Default)]
pub struct TokenCache {
    entries: FxHashMap<PathBuf, CacheEntry>,
}

struct CacheEntry {
    content_hash: u64,
    tokens: Arc<Vec<LspToken>>,
}

impl TokenCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Return cached tokens for `path` if the content hash still matches
    /// `source`; otherwise re-lex and update the cache.
    pub fn get_or_recompute(&mut self, path: &Path, source: &str) -> Arc<Vec<LspToken>> {
        let h = hash_content(source);
        if let Some(entry) = self.entries.get(path) {
            if entry.content_hash == h {
                return entry.tokens.clone();
            }
        }
        let tokens = Arc::new(lex_with_trivia(source));
        self.entries.insert(path.to_path_buf(), CacheEntry { content_hash: h, tokens: tokens.clone() });
        tokens
    }

    /// Drop any cached entry for `path`. Used on `didClose` and on
    /// out-of-band changes detected by the file-watcher.
    pub fn invalidate(&mut self, path: &Path) {
        self.entries.remove(path);
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[cfg(test)]
    pub fn contains(&self, path: &Path) -> bool {
        self.entries.contains_key(path)
    }
}

fn hash_content(s: &str) -> u64 {
    let mut h = FxHasher::default();
    s.hash(&mut h);
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn miss_then_hit_returns_same_arc() {
        let mut cache = TokenCache::new();
        let path = PathBuf::from("/tmp/a.st");
        let first = cache.get_or_recompute(&path, "VAR x : INT; END_VAR");
        let second = cache.get_or_recompute(&path, "VAR x : INT; END_VAR");
        assert!(Arc::ptr_eq(&first, &second), "identical source must hit cache");
    }

    #[test]
    fn content_change_invalidates_entry() {
        let mut cache = TokenCache::new();
        let path = PathBuf::from("/tmp/a.st");
        let first = cache.get_or_recompute(&path, "VAR x : INT; END_VAR");
        let second = cache.get_or_recompute(&path, "VAR y : INT; END_VAR");
        assert!(!Arc::ptr_eq(&first, &second), "content change must produce a new arc");
        assert_eq!(cache.len(), 1, "still only one entry — replaced, not duplicated");
    }

    #[test]
    fn invalidate_drops_entry() {
        let mut cache = TokenCache::new();
        let path = PathBuf::from("/tmp/a.st");
        cache.get_or_recompute(&path, "VAR x : INT; END_VAR");
        assert!(cache.contains(&path));
        cache.invalidate(&path);
        assert!(!cache.contains(&path));
    }

    #[test]
    fn multiple_paths_kept_independently() {
        let mut cache = TokenCache::new();
        let a = PathBuf::from("/tmp/a.st");
        let b = PathBuf::from("/tmp/b.st");
        cache.get_or_recompute(&a, "VAR");
        cache.get_or_recompute(&b, "FUNCTION");
        assert!(cache.contains(&a));
        assert!(cache.contains(&b));
        assert_eq!(cache.len(), 2);
    }
}
