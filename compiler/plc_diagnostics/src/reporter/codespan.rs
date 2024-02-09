use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
    term::termcolor::{Buffer, ColorChoice, StandardStream, WriteColor},
};
use plc_source::source_location::CodeSpan;

use crate::diagnostics::Severity;

use super::{DiagnosticReporter, ResolvedDiagnostics};

enum Writer {
    /// Indicates that the writer will store its output into a buffer
    Buffer(Buffer),

    /// Indicates that the writer will redirect its output to the terminal
    Stream(StandardStream),
}

impl std::io::Write for Writer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Writer::Buffer(writer) => writer.write(buf),
            Writer::Stream(writer) => writer.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Writer::Buffer(writer) => writer.flush(),
            Writer::Stream(writer) => writer.flush(),
        }
    }
}

impl WriteColor for Writer {
    fn supports_color(&self) -> bool {
        match self {
            Writer::Buffer(writer) => writer.supports_color(),
            Writer::Stream(writer) => writer.supports_color(),
        }
    }

    fn set_color(&mut self, spec: &codespan_reporting::term::termcolor::ColorSpec) -> std::io::Result<()> {
        match self {
            Writer::Buffer(writer) => writer.set_color(spec),
            Writer::Stream(writer) => writer.set_color(spec),
        }
    }

    fn reset(&mut self) -> std::io::Result<()> {
        match self {
            Writer::Buffer(writer) => writer.reset(),
            Writer::Stream(writer) => writer.reset(),
        }
    }
}

/// A reporter that reports diagnostics using [`codespan_reporting`].
pub struct CodeSpanDiagnosticReporter {
    files: SimpleFiles<String, String>,
    config: codespan_reporting::term::Config,
    writer: Writer,
}

impl CodeSpanDiagnosticReporter {
    /// Creates a new reporter which redirects its output to the terminal
    pub(crate) fn terminal(config: codespan_reporting::term::Config, writer: StandardStream) -> Self {
        CodeSpanDiagnosticReporter { files: SimpleFiles::new(), config, writer: Writer::Stream(writer) }
    }

    /// Creates a new reporter which stores its output in a buffer
    pub(crate) fn buffered() -> CodeSpanDiagnosticReporter {
        CodeSpanDiagnosticReporter { writer: Writer::Buffer(Buffer::no_color()), ..Default::default() }
    }

    fn emit(&mut self, diag: Diagnostic<usize>) -> Result<(), codespan_reporting::files::Error> {
        codespan_reporting::term::emit(&mut self.writer, &self.config, &self.files, &diag)
    }
}

impl Default for CodeSpanDiagnosticReporter {
    /// creates the default CodeSpanDiagnosticReporter reporting to StdErr, with colors
    fn default() -> Self {
        Self::terminal(
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
    fn report(&mut self, diagnostics: &[ResolvedDiagnostics]) {
        for d in diagnostics {
            let diagnostic_factory = match d.severity {
                Severity::Error => codespan_reporting::diagnostic::Diagnostic::error(),
                Severity::Warning => codespan_reporting::diagnostic::Diagnostic::warning(),
                Severity::Info => codespan_reporting::diagnostic::Diagnostic::note(),
                Severity::Ignore => {
                    log::debug!("Ignoring diagnostic: {}", &d.message);
                    continue;
                }
            };

            let mut labels = vec![];

            if !matches!(d.main_location.span, CodeSpan::None) {
                labels.push(
                    Label::primary(
                        d.main_location.file_handle,
                        d.main_location.span.to_range().unwrap_or_else(|| 0..0),
                    )
                    .with_message(d.message.as_str()),
                );
            }

            if let Some(additional_locations) = &d.additional_locations {
                labels.extend(additional_locations.iter().filter_map(|it| {
                    if !matches!(it.span, CodeSpan::None) {
                        Some(
                            Label::secondary(it.file_handle, it.span.to_range().unwrap_or_else(|| 0..0))
                                .with_message("see also"),
                        )
                    } else {
                        None
                    }
                }));
            }

            let diag = diagnostic_factory.with_labels(labels).with_message(d.message.as_str());

            let result = self.emit(diag);
            if result.is_err() && d.main_location.is_internal() {
                eprintln!("<internal>: {}", d.message);
            }
        }
    }

    fn register(&mut self, path: String, src: String) -> usize {
        self.files.add(path, src)
    }

    fn buffer(&self) -> Option<String> {
        match &self.writer {
            Writer::Buffer(buffer) => Some(String::from_utf8_lossy(buffer.as_slice()).to_string()),
            Writer::Stream(_) => None,
        }
    }
}
