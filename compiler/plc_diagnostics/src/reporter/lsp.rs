//! Reporter that an LSP server can drain after a compile completes.
//!
//! Diagnostician owns the boxed reporter; the LSP server wants typed access
//! to the same data. Shared ownership via `Arc<Mutex<…>>` lets the server
//! hand a clone to the Diagnostician and keep its own typed handle for
//! draining once the compile finishes.

use std::sync::{Arc, Mutex};

use rustc_hash::FxHashMap;

use crate::reporter::{DiagnosticReporter, ResolvedDiagnostics};

#[derive(Default)]
struct Inner {
    collected: Vec<ResolvedDiagnostics>,
    file_paths: FxHashMap<usize, String>,
    last_id: usize,
}

/// Clones share the same backing state. Cheap to clone.
#[derive(Default, Clone)]
pub struct LspReporter(Arc<Mutex<Inner>>);

impl LspReporter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Drain everything `report` has collected so far.
    pub fn take_collected(&self) -> Vec<ResolvedDiagnostics> {
        std::mem::take(&mut self.0.lock().unwrap().collected)
    }

    /// Snapshot of the handle → file-path mapping built up by `register` calls.
    /// Phase 4 of the LSP uses this to resolve both primary and secondary
    /// `file_handle`s in `ResolvedDiagnostics` back to paths.
    pub fn file_paths(&self) -> FxHashMap<usize, String> {
        self.0.lock().unwrap().file_paths.clone()
    }
}

impl DiagnosticReporter for LspReporter {
    fn report(&mut self, diagnostics: &[ResolvedDiagnostics]) {
        self.0.lock().unwrap().collected.extend(diagnostics.iter().cloned());
    }

    fn register(&mut self, path: String, _src: String) -> usize {
        let mut inner = self.0.lock().unwrap();
        inner.last_id += 1;
        let id = inner.last_id;
        inner.file_paths.insert(id, path);
        id
    }
}

#[cfg(test)]
mod tests {
    use plc_source::source_location::CodeSpan;

    use super::*;
    use crate::diagnostics::Severity;
    use crate::reporter::ResolvedLocation;

    fn sample(code: &str, file_handle: usize) -> ResolvedDiagnostics {
        ResolvedDiagnostics {
            code: code.to_string(),
            message: format!("{code} message"),
            severity: Severity::Error,
            main_location: ResolvedLocation { file_handle, span: CodeSpan::None },
            additional_locations: None,
        }
    }

    #[test]
    fn cloned_handle_observes_data_pushed_through_the_other() {
        // Mirrors the LSP usage pattern: one clone goes into the Diagnostician
        // (consumed via DiagnosticReporter), the other stays with plc_lsp.
        let mut for_diagnostician = LspReporter::new();
        let for_lsp = for_diagnostician.clone();

        let handle_a = for_diagnostician.register("/a.st".to_string(), "src a".to_string());
        let handle_b = for_diagnostician.register("/b.st".to_string(), "src b".to_string());
        for_diagnostician.report(&[sample("E001", handle_a), sample("W002", handle_b)]);

        let collected = for_lsp.take_collected();
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0].code, "E001");
        assert_eq!(collected[0].main_location.file_handle, handle_a);
        assert_eq!(collected[1].code, "W002");
        assert_eq!(collected[1].main_location.file_handle, handle_b);

        let paths = for_lsp.file_paths();
        assert_eq!(paths.get(&handle_a).map(String::as_str), Some("/a.st"));
        assert_eq!(paths.get(&handle_b).map(String::as_str), Some("/b.st"));
    }

    #[test]
    fn take_collected_drains() {
        let mut reporter = LspReporter::new();
        let handle = reporter.register("/x.st".to_string(), String::new());
        reporter.report(&[sample("E001", handle)]);

        assert_eq!(reporter.take_collected().len(), 1);
        // Second drain returns empty — collected was drained, not just read.
        assert!(reporter.take_collected().is_empty());
    }

    #[test]
    fn register_issues_distinct_ids() {
        let mut reporter = LspReporter::new();
        let a = reporter.register("/a.st".to_string(), String::new());
        let b = reporter.register("/b.st".to_string(), String::new());
        let c = reporter.register("/c.st".to_string(), String::new());
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_ne!(a, c);
    }
}
