use std::collections::HashMap;

use crate::{
    diagnostics::Diagnostic,
    reporter::{
        clang::ClangFormatDiagnosticReporter, codespan::CodeSpanDiagnosticReporter,
        null::NullDiagnosticReporter, DiagnosticReporter, ResolvedDiagnostics, ResolvedLocation,
    },
};

/// the Diagnostician handle's Diangostics with the help of a
/// assessor and a reporter
pub struct Diagnostician {
    reporter: Box<dyn DiagnosticReporter>,
    assessor: Box<dyn DiagnosticAssessor>,
    filename_fileid_mapping: HashMap<String, usize>,
}

impl Diagnostician {
    /// registers the given source-code at the diagnostician, so it can
    /// preview errors in the source
    /// returns the id to use to reference the given file
    pub fn register_file(&mut self, id: String, src: String) -> usize {
        let handle = self.reporter.register(id.clone(), src);
        self.filename_fileid_mapping.insert(id, handle);
        handle
    }

    fn get_file_handle(&self, file_name: Option<&str>) -> Option<usize> {
        file_name.and_then(|it| self.filename_fileid_mapping.get(it).cloned())
    }

    /// Assess and reports the given diagnostics.
    pub fn handle(&mut self, diagnostics: Vec<Diagnostic>) {
        let resolved_diagnostics = diagnostics
            .iter()
            .flat_map(|it| match it {
                Diagnostic::CombinedDiagnostic { inner_diagnostics, .. } => {
                    let mut res = vec![it];
                    res.extend(inner_diagnostics.iter().collect::<Vec<&Diagnostic>>());
                    res
                }
                _ => vec![it],
            })
            .map(|d| ResolvedDiagnostics {
                message: d.get_message().to_string(),
                severity: self.assess(d),
                main_location: ResolvedLocation {
                    file_handle: self
                        .get_file_handle(d.get_location().get_file_name().or(Some("<internal>")))
                        .unwrap_or(usize::MAX),
                    span: d.get_location().get_span().clone(),
                },
                additional_locations: d.get_secondary_locations().map(|it| {
                    it.iter()
                        .map(|l| ResolvedLocation {
                            file_handle: self
                                .get_file_handle(l.get_file_name().or(Some("<internal>")))
                                .unwrap_or(usize::MAX),
                            span: l.get_span().clone(),
                        })
                        .collect()
                }),
            });

        self.report(resolved_diagnostics.collect::<Vec<_>>().as_slice());
    }

    /// Creates a null-diagnostician that does not report diagnostics
    pub fn null_diagnostician() -> Diagnostician {
        Diagnostician {
            assessor: Box::<DefaultDiagnosticAssessor>::default(),
            reporter: Box::<NullDiagnosticReporter>::default(),
            filename_fileid_mapping: HashMap::new(),
        }
    }

    /// Creates a buffered-diagnostician that saves its reports in a buffer
    pub fn buffered() -> Diagnostician {
        Diagnostician {
            assessor: Box::<DefaultDiagnosticAssessor>::default(),
            reporter: Box::new(CodeSpanDiagnosticReporter::buffered()),
            filename_fileid_mapping: HashMap::new(),
        }
    }

    /// Creates a clang-format-diagnostician that reports diagnostics in clang format
    pub fn clang_format_diagnostician() -> Diagnostician {
        Diagnostician {
            reporter: Box::<ClangFormatDiagnosticReporter>::default(),
            assessor: Box::<DefaultDiagnosticAssessor>::default(),
            filename_fileid_mapping: HashMap::new(),
        }
    }
}

impl DiagnosticReporter for Diagnostician {
    fn report(&mut self, diagnostics: &[ResolvedDiagnostics]) {
        //delegate to reporter
        self.reporter.report(diagnostics);
    }

    fn register(&mut self, path: String, src: String) -> usize {
        //delegate to reporter
        self.reporter.register(path, src)
    }

    fn buffer(&self) -> Option<String> {
        self.reporter.buffer()
    }
}

impl DiagnosticAssessor for Diagnostician {
    fn assess(&self, d: &Diagnostic) -> Severity {
        //delegate to assesor
        self.assessor.assess(d)
    }
}

//This clippy lint is wrong her because the trait is expecting dyn
#[allow(clippy::derivable_impls)]
impl Default for Diagnostician {
    fn default() -> Self {
        Self {
            reporter: Box::<CodeSpanDiagnosticReporter>::default(),
            assessor: Box::<DefaultDiagnosticAssessor>::default(),
            filename_fileid_mapping: HashMap::new(),
        }
    }
}

/// the assessor determins the severity of a diagnostic
/// this trait allows for different implementations for different usecases
/// (e.g. default, compiler-settings, tests)
pub trait DiagnosticAssessor {
    /// determines the severity of the given diagnostic
    fn assess(&self, d: &Diagnostic) -> Severity;
}

/// the default assessor will treat ImprovementSuggestions as warnings
/// and everything else as errors
#[derive(Default)]
pub struct DefaultDiagnosticAssessor;

impl DiagnosticAssessor for DefaultDiagnosticAssessor {
    fn assess(&self, d: &Diagnostic) -> Severity {
        match d {
            // improvements become warnings
            Diagnostic::ImprovementSuggestion { .. } => Severity::Warning,
            // everything else becomes an error
            _ => Severity::Error,
        }
    }
}

/// a diagnostics severity
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    _Info,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let severity = match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::_Info => "info",
        };
        write!(f, "{severity}")
    }
}
