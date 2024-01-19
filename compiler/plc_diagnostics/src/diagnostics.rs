use std::fmt::Display;

use plc_ast::ast::AstNode;
use plc_source::{
    source_location::{SourceLocation, SourceLocationFactory},
    SourceCode,
};

mod diagnostics_registry {
    macro_rules! add_diagnostic {
    ($($number:ident, $desc:expr,)*) => {
        use lazy_static::lazy_static;
            lazy_static! {
                static ref DIAGNOSTICS : HashMap<&'static str, &'static str> = {
                let mut m : HashMap<&str, &str> = HashMap::new();
                $( m.insert(stringify!($number), $desc);)*
                m
            };
        }
    }
}

    use std::collections::HashMap;

    #[derive(Default)]
    pub struct DiagnosticsRegistry(HashMap<&'static str, &'static str>);
    add_diagnostic!(
        E001,
        include_str!("./error_codes/E001.md"), //General Error
        E002,
        include_str!("./error_codes/E002.md"), //General IO Error
        E003,
        include_str!("./error_codes/E003.md"), //Parameter Error
        E004,
        include_str!("./error_codes/E004.md"), //Duplicate Symbol
        E005,
        include_str!("./error_codes/E005.md"), //Generic LLVM Error
        E006,
        include_str!("./error_codes/E006.md"), //Missing Token
        E007,
        include_str!("./error_codes/E007.md"), //Unexpected Token
        E008,
        include_str!("./error_codes/E008.md"), //Invalid Range
        E009,
        include_str!("./error_codes/E009.md"), //Mismatched Parantheses
        E010,
        include_str!("./error_codes/E010.md"), //Invalid Time Literal
        E011,
        include_str!("./error_codes/E011.md"), //Invalid Number
        E012,
        include_str!("./error_codes/E012.md"), //Missing Case Condition
        E013,
        include_str!("./error_codes/E013.md"),
        E014,
        include_str!("./error_codes/E014.md"),
        E015,
        include_str!("./error_codes/E015.md"),
        E016,
        include_str!("./error_codes/E016.md"),
        E017,
        include_str!("./error_codes/E017.md"),
        E018,
        include_str!("./error_codes/E018.md"),
        E019,
        include_str!("./error_codes/E019.md"),
        E020,
        include_str!("./error_codes/E020.md"),
        E021,
        include_str!("./error_codes/E021.md"),
        E022,
        include_str!("./error_codes/E022.md"),
        E023,
        include_str!("./error_codes/E023.md"),
        E024,
        include_str!("./error_codes/E024.md"),
        E025,
        include_str!("./error_codes/E025.md"),
        E026,
        include_str!("./error_codes/E026.md"),
        E027,
        include_str!("./error_codes/E027.md"),
        E028,
        include_str!("./error_codes/E028.md"),
        E029,
        include_str!("./error_codes/E029.md"),
        E030,
        include_str!("./error_codes/E030.md"),
        E031,
        include_str!("./error_codes/E031.md"),
        E032,
        include_str!("./error_codes/E032.md"),
        E033,
        include_str!("./error_codes/E033.md"),
        E034,
        include_str!("./error_codes/E034.md"),
        E035,
        include_str!("./error_codes/E035.md"),
        E036,
        include_str!("./error_codes/E036.md"),
        E037,
        include_str!("./error_codes/E037.md"),
        E038,
        include_str!("./error_codes/E038.md"),
        E039,
        include_str!("./error_codes/E039.md"),
        E040,
        include_str!("./error_codes/E040.md"),
        E041,
        include_str!("./error_codes/E041.md"),
        E042,
        include_str!("./error_codes/E042.md"),
        E043,
        include_str!("./error_codes/E043.md"),
        E044,
        include_str!("./error_codes/E044.md"),
        E045,
        include_str!("./error_codes/E045.md"),
        E046,
        include_str!("./error_codes/E046.md"),
        E047,
        include_str!("./error_codes/E047.md"),
        E048,
        include_str!("./error_codes/E048.md"),
        E049,
        include_str!("./error_codes/E049.md"),
        E050,
        include_str!("./error_codes/E050.md"),
        E051,
        include_str!("./error_codes/E051.md"),
        E052,
        include_str!("./error_codes/E052.md"),
        E053,
        include_str!("./error_codes/E053.md"),
        E054,
        include_str!("./error_codes/E054.md"),
        E055,
        include_str!("./error_codes/E055.md"),
        E056,
        include_str!("./error_codes/E056.md"),
        E057,
        include_str!("./error_codes/E057.md"),
        E058,
        include_str!("./error_codes/E058.md"),
        E059,
        include_str!("./error_codes/E059.md"),
        E060,
        include_str!("./error_codes/E060.md"),
        E061,
        include_str!("./error_codes/E061.md"),
        E062,
        include_str!("./error_codes/E062.md"),
        E063,
        include_str!("./error_codes/E063.md"),
        E064,
        include_str!("./error_codes/E064.md"),
        E065,
        include_str!("./error_codes/E065.md"),
        E066,
        include_str!("./error_codes/E066.md"),
        E067,
        include_str!("./error_codes/E067.md"),
        E068,
        include_str!("./error_codes/E068.md"),
        E069,
        include_str!("./error_codes/E069.md"),
        E070,
        include_str!("./error_codes/E070.md"),
        E071,
        include_str!("./error_codes/E071.md"),
        E072,
        include_str!("./error_codes/E072.md"),
        E073,
        include_str!("./error_codes/E073.md"),
        E074,
        include_str!("./error_codes/E074.md"),
        E075,
        include_str!("./error_codes/E075.md"),
        E076,
        include_str!("./error_codes/E076.md"),
        E077,
        include_str!("./error_codes/E077.md"),
        E078,
        include_str!("./error_codes/E078.md"),
        E079,
        include_str!("./error_codes/E079.md"),
        E080,
        include_str!("./error_codes/E080.md"),
        E081,
        include_str!("./error_codes/E081.md"),
        E082,
        include_str!("./error_codes/E082.md"),
        E083,
        include_str!("./error_codes/E083.md"),
        E084,
        include_str!("./error_codes/E084.md"),
        E085,
        include_str!("./error_codes/E085.md"),
        E086,
        include_str!("./error_codes/E086.md"),
        E087,
        include_str!("./error_codes/E087.md"),
        E088,
        include_str!("./error_codes/E088.md"),
        E089,
        include_str!("./error_codes/E089.md"),
        E090,
        include_str!("./error_codes/E090.md"),
    );
}

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
    /// Severity of the error being reported
    severity: Severity,
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
    pub fn new(message: impl Into<String>, severity: Severity) -> Self {
        Diagnostic {
            message: message.into(),
            severity,
            primary_location: SourceLocation::undefined(),
            secondary_locations: Default::default(),
            error_code: "E001".into(), //Default error if none specified
            sub_diagnostics: Default::default(),
            internal_error: Default::default(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::new(message, Severity::Error)
    }
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(message, Severity::Warning)
    }
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(message, Severity::Info)
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
        self.severity.partial_cmp(&other.severity)
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
            && self.severity == other.severity
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

    pub fn get_severity(&self) -> Severity {
        self.severity
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
