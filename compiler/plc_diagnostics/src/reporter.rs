use crate::diagnostics::Severity;
use plc_ast::source_location::CodeSpan;

pub mod clang;
pub mod codespan;
pub mod null;

/// the DiagnosticReporter decides on the format and where to report the diagnostic to.
/// possible implementations could print to either std-out, std-err or a file, etc.
pub trait DiagnosticReporter {
    /// reports the given diagnostic
    fn report(&mut self, diagnostics: &[ResolvedDiagnostics]);
    /// register the given path & src and returns an ID to indicate
    /// a relationship the given src (diagnostics for this src need
    /// to use this id)
    fn register(&mut self, path: String, src: String) -> usize;

    fn buffer(&self) -> Option<String> {
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedLocation {
    pub file_handle: usize,
    pub span: CodeSpan,
}

impl ResolvedLocation {
    pub(crate) fn is_internal(&self) -> bool {
        self.span == CodeSpan::None
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedDiagnostics {
    pub code: String,
    pub message: String,
    pub severity: Severity,
    pub main_location: ResolvedLocation,
    pub additional_locations: Option<Vec<ResolvedLocation>>,
}
