use codespan_reporting::files::{Files, Location, SimpleFile, SimpleFiles};
use plc_source::source_location::CodeSpan;

use crate::diagnostics::Severity;

use super::{DiagnosticReporter, ResolvedDiagnostics};

/// A reporter that reports diagnostics in clang format. Specifically the messages have the following
/// form `<filename>:<range>: <severity>: <message>`.
pub struct ClangFormatDiagnosticReporter {
    files: SimpleFiles<String, String>,
}

impl ClangFormatDiagnosticReporter {
    fn new() -> Self {
        ClangFormatDiagnosticReporter { files: SimpleFiles::new() }
    }
}

impl Default for ClangFormatDiagnosticReporter {
    fn default() -> Self {
        ClangFormatDiagnosticReporter::new()
    }
}

impl DiagnosticReporter for ClangFormatDiagnosticReporter {
    fn report(&mut self, diagnostics: &[ResolvedDiagnostics]) {
        for diagnostic in diagnostics.iter().filter(|it| it.severity > Severity::Ignore) {
            let file_id = diagnostic.main_location.file_handle;
            let location = &diagnostic.main_location;

            // Only a text range resolves to lines and columns in the file.
            let file = self.files.get(file_id).ok();
            let (start, end) = match location.span.to_range() {
                Some(range) => (
                    self.files.location(file_id, range.start).ok(),
                    self.files.location(file_id, range.end).ok(),
                ),
                None => (None, None),
            };

            let res = self.build_diagnostic_msg(
                file,
                start.as_ref(),
                end.as_ref(),
                &location.span,
                &diagnostic.code,
                &diagnostic.severity,
                &diagnostic.message,
            );

            eprintln!("{res}");
        }
    }
    fn register(&mut self, path: String, src: String) -> usize {
        self.files.add(path, src)
    }
}

impl ClangFormatDiagnosticReporter {
    /// returns diagnostic message in clang format
    /// file-name:{range}: severity: message for a text location,
    /// file-name.diagram:order[:pin]: severity: message for a diagram element
    /// optional parameters that are none will not be included
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn build_diagnostic_msg(
        &self,
        file: Option<&SimpleFile<String, String>>,
        start: Option<&Location>,
        end: Option<&Location>,
        span: &CodeSpan,
        code: &str,
        severity: &Severity,
        msg: &str,
    ) -> String {
        let mut str = String::new();
        // file name and position
        if let Some(f) = file {
            str.push_str(f.name().as_str());
            match (start, end) {
                // if start and end are equal there is no need to show the range
                (Some(s), Some(e)) if s.eq(e) => {
                    str.push_str(format!(":{}:{}: ", s.line_number, s.column_number).as_str());
                }
                (Some(s), Some(e)) => {
                    str.push_str(
                        format!(
                            ":{}:{}:{{{}:{}-{}:{}}}: ",
                            s.line_number,
                            s.column_number,
                            s.line_number,
                            s.column_number,
                            e.line_number,
                            e.column_number
                        )
                        .as_str(),
                    );
                }
                _ if matches!(span, CodeSpan::Block { .. }) => str.push_str(format!(".{span}: ").as_str()),
                _ => str.push_str(": "),
            }
        }
        // severity
        str.push_str(format!("{severity}[{code}]: ").as_str());
        // msg
        str.push_str(msg);

        str
    }
}
