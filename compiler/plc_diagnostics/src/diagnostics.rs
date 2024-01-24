use std::fmt::Display;

use plc_ast::ast::AstNode;
use plc_source::{
    source_location::{SourceLocation, SourceLocationFactory},
    SourceCode,
};

pub mod diagnostics_registry;

pub const INTERNAL_LLVM_ERROR: &str = "internal llvm codegen error";

/// a diagnostics severity
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    #[default]
    Info,
    Warning,
    Error,
}

/// The `Diagnostics` struct describes an issue encountered during compile time.
/// The issue is defined by an `error_code` and had a defined `severity`
/// Diagnostic severity can be overridden when being reported.
#[derive(Debug)]
pub struct Diagnostic {
    /// The Description of the error being reported.
    message: String,
    /// Primary location where the diagnostic occurred
    primary_location: SourceLocation,
    /// Seconday locations relevant to the diagnostics
    secondary_locations: Option<Vec<SourceLocation>>,
    /// Error code for reference in the documentation
    error_code: &'static str,
    /// Children of the current diagnostic
    sub_diagnostics: Vec<Diagnostic>,
    /// If the diagnostic is caused by an error, this field contains the original error
    internal_error: Option<anyhow::Error>,
}

impl std::error::Error for Diagnostic {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.internal_error.as_ref().and_then(|it| it.source())
    }
}

impl From<std::io::Error> for Diagnostic {
    fn from(value: std::io::Error) -> Self {
        Diagnostic::error(value.to_string()).with_error_code("E002").with_internal_error(value.into())
    }
}

/// Builder for Diagnostics
impl Diagnostic {
    pub fn new(message: impl Into<String>) -> Self {
        Diagnostic {
            message: message.into(),
            primary_location: SourceLocation::undefined(),
            secondary_locations: Default::default(),
            error_code: "E001", //Default error if none specified
            sub_diagnostics: Default::default(),
            internal_error: Default::default(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::new(message)
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.primary_location = location;
        self
    }

    pub fn with_secondary_location(mut self, location: SourceLocation) -> Self {
        self.secondary_locations.get_or_insert_with(Default::default).push(location);
        self
    }

    pub fn with_secondary_locations(mut self, locations: Vec<SourceLocation>) -> Self {
        self.secondary_locations.get_or_insert_with(Default::default).extend(locations);
        self
    }

    pub fn with_error_code(mut self, error_code: &'static str) -> Self {
        self.error_code = error_code;
        self
    }

    pub fn with_sub_diagnostic(mut self, diagnostic: Diagnostic) -> Self {
        self.sub_diagnostics.push(diagnostic);
        self
    }

    pub fn with_sub_diagnostics(mut self, diagnostics: Vec<Diagnostic>) -> Self {
        self.sub_diagnostics.extend(diagnostics);
        self
    }

    pub fn with_internal_error(mut self, error: anyhow::Error) -> Self {
        self.internal_error = Some(error);
        self
    }

    pub fn from_serde_error(error: serde_json::Error, source: &SourceCode) -> Self {
        let factory = SourceLocationFactory::for_source(source);
        let line = error.line();
        let column = error.column();

        // remove line, column from message
        let message = error.to_string();
        let message = if let Some(pos) = message.find("at line") {
            message.chars().take(pos).collect()
        } else {
            message
        };
        let range = factory.create_range_to_end_of_line(line, column);
        Diagnostic::error(message).with_error_code("E088").with_location(range)
    }
}

impl PartialOrd for Diagnostic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Diagnostic {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.severity.cmp(&other.severity)
    }
}

impl PartialEq for Diagnostic {
    fn eq(&self, other: &Self) -> bool {
        self.error_code == other.error_code
            && self.message == other.message
            && self.primary_location == other.primary_location
            && self.secondary_locations == other.secondary_locations
            && self.sub_diagnostics == other.sub_diagnostics
    }
}

impl Eq for Diagnostic {}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.get_type(), self.get_message())?;
        let location = self.get_location();
        if !location.is_undefined() {
            write!(f, " at: {location}")
        } else {
            Ok(())
        }
    }
}

impl Diagnostic {
    pub fn get_message(&self) -> &str {
        self.message.as_str()
    }

    pub fn get_location(&self) -> SourceLocation {
        self.primary_location.clone()
    }

    pub fn get_secondary_locations(&self) -> Option<&[SourceLocation]> {
        self.secondary_locations.as_deref()
    }

    pub fn get_type(&self) -> &'static str {
        self.error_code
    }

    pub fn get_sub_diagnostics(&self) -> &[Diagnostic] {
        &self.sub_diagnostics
    }
}

//Helper methods for diagnostics
impl Diagnostic {
    pub fn unexpected_token_found(expected: &str, found: &str, range: SourceLocation) -> Diagnostic {
        Diagnostic::error(format!("Unexpected token: expected {expected} but found {found}"))
            .with_error_code("E007")
            .with_location(range)
    }

    pub fn missing_function(location: SourceLocation) -> Diagnostic {
        Diagnostic::error("Cannot generate code outside of function context.")
            .with_error_code("E072")
            .with_location(location)
    }

    pub fn codegen_error(message: impl Into<String>, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(message).with_location(location).with_error_code("E071")
    }

    pub fn llvm_error(file: &str, llvm_error: &str) -> Diagnostic {
        Diagnostic::error(format!("{file}: Internal llvm error: {:}", llvm_error)).with_error_code("E005")
    }

    pub fn missing_token(expected_token: &str, range: SourceLocation) -> Diagnostic {
        Diagnostic::error(format!("Missing expected Token {expected_token}"))
            .with_location(range)
            .with_error_code("E006")
    }

    pub fn invalid_parameter_count(expected: usize, received: usize, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(
             format!(
                "Invalid parameter count. Received {received} parameters while {expected} parameters were expected.",
            )).with_error_code("E032")
            .with_location(location)
    }

    pub fn unknown_type(type_name: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(format!("Unknown type: {type_name:}"))
            .with_error_code("E052")
            .with_location(location)
    }

    pub fn unresolved_reference(reference: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(format!("Could not resolve reference to {reference:}"))
            .with_error_code("E048")
            .with_location(location)
    }

    pub fn invalid_assignment(right_type: &str, left_type: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(format!("Invalid assignment: cannot assign '{right_type}' to '{left_type}'"))
            .with_error_code("E037")
            .with_location(location)
    }

    pub fn cannot_generate_initializer(variable_name: &str, location: SourceLocation) -> Diagnostic {
        Self::error(format!(
            "Cannot generate literal initializer for '{variable_name}': Value cannot be derived"
        ))
        .with_error_code("E041")
        .with_location(location)
    }

    pub fn cannot_generate_call_statement(operator: &AstNode) -> Diagnostic {
        //TODO: We could probably get a better slice here
        Diagnostic::codegen_error(
            format!("cannot generate call statement for {:?}", operator),
            operator.get_location(),
        )
    }

    pub fn cannot_generate_from_empty_literal(type_name: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::codegen_error(
            format!("Cannot generate {type_name} from empty literal").as_str(),
            location,
        )
    }
}

// CFC related diagnostics
impl Diagnostic {
    pub fn unnamed_control(location: SourceLocation) -> Diagnostic {
        Diagnostic::error("Unnamed control").with_error_code("E087").with_location(location)
    }
}

#[cfg(test)]
mod tests {
    use codespan_reporting::files::{Location, SimpleFile};

    use crate::{diagnostics::Severity, reporter::clang::ClangFormatDiagnosticReporter};

    #[test]
    fn test_build_diagnostic_msg() {
        let reporter = ClangFormatDiagnosticReporter::default();
        let file = SimpleFile::new("test.st".to_string(), "source".to_string());
        let start = Location { line_number: 4, column_number: 1 };
        let end = Location { line_number: 4, column_number: 4 };
        let res = reporter.build_diagnostic_msg(
            Some(&file),
            Some(&start),
            Some(&end),
            &Severity::Error,
            "This is an error",
        );

        assert_eq!(res, "test.st:4:1:{4:1-4:4}: error: This is an error");
    }

    #[test]
    fn test_build_diagnostic_msg_equal_start_end() {
        let reporter = ClangFormatDiagnosticReporter::default();
        let file = SimpleFile::new("test.st".to_string(), "source".to_string());
        let start = Location { line_number: 4, column_number: 1 };
        let end = Location { line_number: 4, column_number: 1 };
        let res = reporter.build_diagnostic_msg(
            Some(&file),
            Some(&start),
            Some(&end),
            &Severity::Error,
            "This is an error",
        );

        assert_eq!(res, "test.st:4:1: error: This is an error");
    }

    #[test]
    fn test_build_diagnostic_msg_no_location() {
        let reporter = ClangFormatDiagnosticReporter::default();
        let file = SimpleFile::new("test.st".to_string(), "source".to_string());
        let res =
            reporter.build_diagnostic_msg(Some(&file), None, None, &Severity::Error, "This is an error");

        assert_eq!(res, "test.st: error: This is an error");
    }

    #[test]
    fn test_build_diagnostic_msg_no_file() {
        let reporter = ClangFormatDiagnosticReporter::default();
        let start = Location { line_number: 4, column_number: 1 };
        let end = Location { line_number: 4, column_number: 4 };
        let res = reporter.build_diagnostic_msg(
            None,
            Some(&start),
            Some(&end),
            &Severity::Error,
            "This is an error",
        );

        assert_eq!(res, "error: This is an error");
    }

    #[test]
    fn test_build_diagnostic_msg_no_file_no_location() {
        let reporter = ClangFormatDiagnosticReporter::default();
        let res = reporter.build_diagnostic_msg(None, None, None, &Severity::Error, "This is an error");

        assert_eq!(res, "error: This is an error");
    }
}
