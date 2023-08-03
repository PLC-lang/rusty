use std::ops::Range;

use plc_ast::ast::SourceRange;

use crate::diagnostician::Severity;

pub mod clang;
pub mod codespan;
pub mod null;

/// the DiagnosticReporter decides on the format and where to report the diagnostic to.
/// possible implementations could print to either std-out, std-err or a file, etc.
pub trait DiagnosticReporter {
    /// reports the given diagnostic
    fn report(&self, diagnostics: &[ResolvedDiagnostics]);
    /// register the given path & src and returns an ID to indicate
    /// a relationship the given src (diagnostics for this src need
    /// to use this id)
    fn register(&mut self, path: String, src: String) -> usize;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedLocation {
    pub file_handle: usize,
    pub range: Range<usize>,
}

impl ResolvedLocation {
    pub fn is_internal(&self) -> bool {
        self.range == SourceRange::undefined().to_range()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedDiagnostics {
    pub message: String,
    pub severity: Severity,
    pub main_location: ResolvedLocation,
    pub additional_locations: Option<Vec<ResolvedLocation>>,
}
