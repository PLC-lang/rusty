use codespan_reporting::files::{Files, Location, SimpleFile, SimpleFiles};

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

            let file = self.files.get(file_id).ok();
            let start =
                self.files.location(file_id, location.span.to_range().map(|it| it.start).unwrap_or(0)).ok();
            let end =
                self.files.location(file_id, location.span.to_range().map(|it| it.end).unwrap_or(0)).ok();

            let res = self.build_diagnostic_msg(
                file,
                start.as_ref(),
                end.as_ref(),
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
    /// file-name:{range}: severity: message
    /// optional parameters that are none will not be included
    pub(crate) fn build_diagnostic_msg(
        &self,
        file: Option<&SimpleFile<String, String>>,
        start: Option<&Location>,
        end: Option<&Location>,
        code: &str,
        severity: &Severity,
        msg: &str,
    ) -> String {
        let mut str = String::new();
        // file name
        if let Some(f) = file {
            str.push_str(format!("{}:", f.name().as_str()).as_str());
            // range
            if let Some(s) = start {
                if let Some(e) = end {
                    // if start and end are equal there is no need to show the range
                    if s.eq(e) {
                        str.push_str(format!("{}:{}: ", s.line_number, s.column_number).as_str());
                    } else {
                        str.push_str(
                            format!(
                                "{}:{}:{{{}:{}-{}:{}}}: ",
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
                }
            } else {
                str.push(' ');
            }
        }
        // severity
        str.push_str(format!("{severity}[{code}]: ").as_str());
        // msg
        str.push_str(msg);

        str
    }
}
