use rustc_hash::FxHashMap;

use crate::{
    diagnostics::{
        diagnostics_registry::{DiagnosticsConfiguration, DiagnosticsRegistry},
        Diagnostic, Severity,
    },
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
    filename_fileid_mapping: FxHashMap<String, usize>,
}

unsafe impl Send for Diagnostician {}
unsafe impl Sync for Diagnostician {}

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
    pub fn handle(&mut self, diagnostics: &[Diagnostic]) -> Severity {
        let resolved_diagnostics = diagnostics
            .iter()
            .flat_map(|it| {
                let mut res = vec![it];
                res.extend(it.get_sub_diagnostics());
                res
            })
            .map(|d| ResolvedDiagnostics {
                code: d.get_error_code().to_string(),
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
            })
            .collect::<Vec<_>>();

        self.report(resolved_diagnostics.as_slice());

        resolved_diagnostics.iter().map(|it| it.severity).max().unwrap_or_default()
    }

    /// Creates a null-diagnostician that does not report diagnostics
    pub fn null_diagnostician() -> Diagnostician {
        Diagnostician {
            assessor: Box::<DiagnosticsRegistry>::default(),
            reporter: Box::<NullDiagnosticReporter>::default(),
            filename_fileid_mapping: FxHashMap::default(),
        }
    }

    /// Creates a buffered-diagnostician that saves its reports in a buffer
    pub fn buffered() -> Diagnostician {
        Diagnostician {
            assessor: Box::<DiagnosticsRegistry>::default(),
            reporter: Box::new(CodeSpanDiagnosticReporter::buffered()),
            filename_fileid_mapping: FxHashMap::default(),
        }
    }

    /// Creates a clang-format-diagnostician that reports diagnostics in clang format
    pub fn clang_format_diagnostician() -> Diagnostician {
        Diagnostician {
            reporter: Box::<ClangFormatDiagnosticReporter>::default(),
            assessor: Box::<DiagnosticsRegistry>::default(),
            filename_fileid_mapping: FxHashMap::default(),
        }
    }

    pub fn with_configuration(self, configuration: DiagnosticsConfiguration) -> Self {
        let mut res = self;
        let registry = DiagnosticsRegistry::default().with_configuration(configuration);
        res.assessor = Box::new(registry);
        res
    }

    /// Explain the error with the given code by consulting the diagnostics registry
    pub fn explain(&self, error: &str) -> String {
        self.assessor.explain(error)
    }
    pub fn get_diagnostic_configuration(&self) -> String {
        self.assessor.get_diagnostic_configuration()
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

    fn explain(&self, _error: &str) -> String {
        unimplemented!("Error explanation is not supported on this diagnostian")
    }

    fn get_diagnostic_configuration(&self) -> String {
        unimplemented!("Diagnostic configuration is not supported on this diagnostian")
    }
}

//This clippy lint is wrong her because the trait is expecting dyn
#[allow(clippy::derivable_impls)]
impl Default for Diagnostician {
    fn default() -> Self {
        Self {
            reporter: Box::<CodeSpanDiagnosticReporter>::default(),
            assessor: Box::<DiagnosticsRegistry>::default(),
            filename_fileid_mapping: FxHashMap::default(),
        }
    }
}

/// the assessor determins the severity of a diagnostic
/// this trait allows for different implementations for different usecases
/// (e.g. default, compiler-settings, tests)
pub trait DiagnosticAssessor {
    /// determines the severity of the given diagnostic
    fn assess(&self, d: &Diagnostic) -> Severity;
    //TODO should these be results
    /// Explains the given error based on the diagnostician information
    fn explain(&self, error: &str) -> String;
    /// Returns a serialized version of the diagnostics configuration this diagnostician will use
    /// to assess errors
    fn get_diagnostic_configuration(&self) -> String;
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let severity = match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
            Severity::Ignore => "ignore",
        };
        write!(f, "{severity}")
    }
}
