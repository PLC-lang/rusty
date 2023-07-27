use codespan_reporting::{
    diagnostic::Label,
    files::SimpleFiles,
    term::{termcolor::Buffer, Config},
};

use crate::diagnostics::{DiagnosticReporter, ResolvedDiagnostics, Severity};

pub struct SnapshotDiagnosticReporter {
    files: SimpleFiles<String, String>,
    config: Config,
    buffer: Buffer,
}

impl SnapshotDiagnosticReporter {
    pub fn new(config: Config) -> Self {
        Self { files: SimpleFiles::new(), config, buffer: Buffer::no_color() }
    }
}

impl DiagnosticReporter for SnapshotDiagnosticReporter {
    fn report(&mut self, diagnostics: &[ResolvedDiagnostics]) {
        // TODO: This is pretty much the same as the codepsan reporter code with the exception of the buffer
        // we're using. Can we maybe morph them together somehow?
        for d in dbg!(diagnostics) {
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

            let result = codespan_reporting::term::emit(&mut self.buffer, &self.config, &self.files, &diag);
            if result.is_err() && d.main_location.is_internal() {
                eprintln!("<internal>: {}", d.message);
            }
        }
    }

    fn register(&mut self, path: String, src: String) -> usize {
        self.files.add(path, src)
    }

    fn buffer(&mut self) -> Option<String> {
        Some(String::from_utf8_lossy(self.buffer.as_slice()).to_string())
    }
}

impl Default for SnapshotDiagnosticReporter {
    fn default() -> Self {
        Self::new(codespan_reporting::term::Config {
            display_style: codespan_reporting::term::DisplayStyle::Rich,
            tab_width: 2,
            styles: codespan_reporting::term::Styles::default(),
            chars: codespan_reporting::term::Chars::default(),
            start_context_lines: 5,
            end_context_lines: 3,
        })
    }
}
