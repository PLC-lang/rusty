use std::{backtrace, fmt::Display};

use inkwell::builder::BuilderError;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;
use thiserror::Error;

#[derive(Error, Debug)]
pub struct CodegenDiagnostic {
    message: String,
    source: BuilderError,
    location: SourceLocation,
}

impl Display for CodegenDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Codegen error: {}. Source: {} at {}", &self.message, &self.source, &self.location)
    }
}

impl From<BuilderError> for CodegenDiagnostic {
    fn from(source: BuilderError) -> Self {
        // println!("Backtrace: {} ", backtrace::Backtrace::force_capture());
        CodegenDiagnostic { message: source.to_string(), source, location: SourceLocation::undefined() }
    }
}

impl CodegenDiagnostic {
    pub(crate) fn with_location(self, location: SourceLocation) -> CodegenDiagnostic {
        CodegenDiagnostic { location, ..self }
    }
}

impl From<CodegenDiagnostic> for Diagnostic {
    fn from(value: CodegenDiagnostic) -> Self {
        Diagnostic::codegen_error(&value.message, value.location.clone()).with_internal_error(value.into())
    }
}
