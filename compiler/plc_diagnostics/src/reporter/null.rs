use crate::diagnostics::{DiagnosticReporter, ResolvedDiagnostics};

/// a DiagnosticReporter that swallows all diagnostics
#[derive(Default)]
pub struct NullDiagnosticReporter {
    last_id: usize,
}

impl DiagnosticReporter for NullDiagnosticReporter {
    fn report(&self, _diagnostics: &[ResolvedDiagnostics]) {
        //ignore
    }

    fn register(&mut self, _path: String, _src: String) -> usize {
        // at least provide some unique ids
        self.last_id += 1;
        self.last_id
    }
}
