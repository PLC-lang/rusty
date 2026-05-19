//! In-memory document store for buffers the client has open.
//!
//! Intentionally LSP-agnostic on the inside: doesn't know about language
//! ids, URIs received on the wire, or notification semantics. Filtering
//! (which `language_id`s to accept) and dispatch (which LSP message maps
//! to which method) live in lib.rs.

use std::collections::HashMap;

use lsp_types::Uri;

/// In-memory state for a single open document.
#[derive(Debug, Clone)]
pub struct DocumentBuffer {
    pub content: String,
    pub version: i32,
    pub language_id: String,
}

/// Map of every editor-open buffer the server is currently tracking.
#[derive(Debug, Default)]
pub struct DocumentStore {
    docs: HashMap<Uri, DocumentBuffer>,
}

impl DocumentStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a buffer on `didOpen`. A duplicate open without an intervening
    /// close is a client-side protocol error, but we tolerate it by
    /// replacing the prior entry.
    pub fn open(&mut self, uri: Uri, language_id: String, version: i32, text: String) {
        self.docs.insert(uri, DocumentBuffer { content: text, version, language_id });
    }

    /// Replace the content of a buffer on `didChange` (Full sync). Logs and
    /// ignores changes for unknown URIs; the protocol allows races between
    /// the client's didClose and a late didChange.
    pub fn change(&mut self, uri: &Uri, version: i32, text: String) {
        match self.docs.get_mut(uri) {
            Some(buf) => {
                buf.content = text;
                buf.version = version;
            }
            None => log::warn!("ignoring didChange for unknown URI: {uri:?}"),
        }
    }

    /// Drop a buffer on `didClose`. Logs and ignores closes for unknown URIs.
    pub fn close(&mut self, uri: &Uri) {
        if self.docs.remove(uri).is_none() {
            log::warn!("ignoring didClose for unknown URI: {uri:?}");
        }
    }

    pub fn get(&self, uri: &Uri) -> Option<&DocumentBuffer> {
        self.docs.get(uri)
    }

    pub fn len(&self) -> usize {
        self.docs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.docs.is_empty()
    }

    /// Iterate `(uri, buffer)` pairs for every tracked document.
    /// Used by the compile snapshot builder.
    pub fn iter(&self) -> impl Iterator<Item = (&Uri, &DocumentBuffer)> {
        self.docs.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn uri(s: &str) -> Uri {
        s.parse().expect("test URI must parse")
    }

    #[test]
    fn open_inserts_a_buffer() {
        let mut store = DocumentStore::new();
        store.open(uri("file:///a.st"), "st".to_string(), 1, "PROGRAM a; END_PROGRAM".to_string());

        let buf = store.get(&uri("file:///a.st")).expect("buffer must be present");
        assert_eq!(buf.version, 1);
        assert_eq!(buf.language_id, "st");
        assert_eq!(buf.content, "PROGRAM a; END_PROGRAM");
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn change_replaces_content_and_bumps_version() {
        let mut store = DocumentStore::new();
        let u = uri("file:///a.st");
        store.open(u.clone(), "st".to_string(), 1, "old".to_string());
        store.change(&u, 2, "new".to_string());

        let buf = store.get(&u).expect("buffer must be present");
        assert_eq!(buf.version, 2);
        assert_eq!(buf.content, "new");
    }

    #[test]
    fn change_for_unknown_uri_is_ignored() {
        let mut store = DocumentStore::new();
        // Should not panic; logs at warn and moves on.
        store.change(&uri("file:///never-opened.st"), 1, "hello".to_string());
        assert!(store.is_empty());
    }

    #[test]
    fn close_removes_the_buffer() {
        let mut store = DocumentStore::new();
        let u = uri("file:///a.st");
        store.open(u.clone(), "st".to_string(), 1, "x".to_string());
        store.close(&u);
        assert!(store.get(&u).is_none());
        assert!(store.is_empty());
    }

    #[test]
    fn close_for_unknown_uri_is_ignored() {
        let mut store = DocumentStore::new();
        store.close(&uri("file:///never-opened.st"));
        assert!(store.is_empty());
    }

    #[test]
    fn duplicate_open_replaces_the_buffer() {
        let mut store = DocumentStore::new();
        let u = uri("file:///a.st");
        store.open(u.clone(), "st".to_string(), 1, "first".to_string());
        store.open(u.clone(), "st".to_string(), 5, "second".to_string());

        let buf = store.get(&u).expect("buffer must be present");
        assert_eq!(buf.version, 5);
        assert_eq!(buf.content, "second");
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn multiple_buffers_are_independent() {
        let mut store = DocumentStore::new();
        store.open(uri("file:///a.st"), "st".to_string(), 1, "A".to_string());
        store.open(uri("file:///b.st"), "st".to_string(), 1, "B".to_string());

        assert_eq!(store.len(), 2);
        assert_eq!(store.get(&uri("file:///a.st")).unwrap().content, "A");
        assert_eq!(store.get(&uri("file:///b.st")).unwrap().content, "B");

        store.close(&uri("file:///a.st"));
        assert_eq!(store.len(), 1);
        assert!(store.get(&uri("file:///a.st")).is_none());
        assert_eq!(store.get(&uri("file:///b.st")).unwrap().content, "B");
    }
}
