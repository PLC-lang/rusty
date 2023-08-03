use codespan_reporting::{
    diagnostic::Label,
    files::SimpleFiles,
    term::termcolor::{ColorChoice, StandardStream},
};

use crate::diagnostician::Severity;

use super::{DiagnosticReporter, ResolvedDiagnostics};

/// A reporter that reports diagnostics using [`codespan_reporting`].
pub struct CodeSpanDiagnosticReporter {
    files: SimpleFiles<String, String>,
    config: codespan_reporting::term::Config,
    writer: StandardStream,
}

impl CodeSpanDiagnosticReporter {
    /// creates a new reporter with the given codespan_reporting configuration
    fn new(config: codespan_reporting::term::Config, writer: StandardStream) -> Self {
        CodeSpanDiagnosticReporter { files: SimpleFiles::new(), config, writer }
    }
}

impl Default for CodeSpanDiagnosticReporter {
    /// creates the default CodeSpanDiagnosticReporter reporting to StdErr, with colors
    fn default() -> Self {
        Self::new(
            codespan_reporting::term::Config {
                display_style: codespan_reporting::term::DisplayStyle::Rich,
                tab_width: 2,
                styles: codespan_reporting::term::Styles::default(),
                chars: codespan_reporting::term::Chars::default(),
                start_context_lines: 5,
                end_context_lines: 3,
            },
            StandardStream::stderr(ColorChoice::Always),
        )
    }
}

impl DiagnosticReporter for CodeSpanDiagnosticReporter {
    fn report(&self, diagnostics: &[ResolvedDiagnostics]) {
        for d in diagnostics {
            let diagnostic_factory = match d.severity {
                Severity::Error => codespan_reporting::diagnostic::Diagnostic::error(),
                Severity::Warning => codespan_reporting::diagnostic::Diagnostic::warning(),
                Severity::_Info => codespan_reporting::diagnostic::Diagnostic::note(),
            };

            let mut labels = vec![Label::primary(d.main_location.file_handle, d.main_location.range.clone())
                .with_message(d.message.as_str())];

            if let Some(additional_locations) = &d.additional_locations {
                labels.extend(
                    additional_locations.iter().map(|it| {
                        Label::secondary(it.file_handle, it.range.clone()).with_message("see also")
                    }),
                );
            }

            let diag = diagnostic_factory.with_labels(labels).with_message(d.message.as_str());

            let result =
                codespan_reporting::term::emit(&mut self.writer.lock(), &self.config, &self.files, &diag);
            if result.is_err() && d.main_location.is_internal() {
                eprintln!("<internal>: {}", d.message);
            }
        }
    }

    fn register(&mut self, path: String, src: String) -> usize {
        self.files.add(path, src)
    }
}
