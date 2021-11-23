use std::ops::Range;

use codespan_reporting::{
    diagnostic::Label,
    files::SimpleFiles,
    term::termcolor::{ColorChoice, StandardStream},
};
use inkwell::support::LLVMString;

use crate::ast::{DataTypeDeclaration, PouType, SourceRange};

pub const INTERNAL_LLVM_ERROR: &str = "internal llvm codegen error";

#[derive(PartialEq, Debug, Clone)]
pub enum Diagnostic {
    SyntaxError {
        message: String,
        range: SourceRange,
        err_no: ErrNo,
    },
    GeneralError {
        message: String,
        err_no: ErrNo,
    },
    ImprovementSuggestion {
        message: String,
        range: SourceRange,
    },
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug, Clone)]
pub enum ErrNo {
    undefined,

    //general
    general__io_err,

    //syntax
    syntax__generic_error,
    syntax__missing_token,
    syntax__unexpected_token,

    //semantic
    // pou related
    pou__missing_return_type,
    pou__unexpected_return_type,
    pou__unsupported_return_type,
    pou__empty_variable_block,

    //variable related
    var__unresolved_constant,
    var__invalid_constant_block,
    var__invalid_constant,
    var__cannot_assign_to_const,

    //reference related
    reference__unresolved,

    //type related
    type__cast_error,
    type__unknown_type,
    type__literal_out_of_range,
    type__incompatible_literal_cast,
    type__incompatible_directaccess,
    type__incompatible_directaccess_variable,
    type__incompatible_directaccess_range,
    type__expected_literal,

    //codegen related
    codegen__general,
    codegen__missing_function,
}

impl Diagnostic {
    pub fn syntax_error(message: &str, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: message.to_string(),
            range,
            err_no: ErrNo::syntax__generic_error,
        }
    }

    pub fn unexpected_token_found(expected: &str, found: &str, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Unexpected token: expected {} but found {}",
                expected, found
            ),
            range,
            err_no: ErrNo::syntax__unexpected_token,
        }
    }

    pub fn unexpected_initializer_on_function_return(range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Return types cannot have a default value".into(),
            range,
            err_no: ErrNo::syntax__unexpected_token,
        }
    }

    pub fn return_type_not_supported(pou_type: &PouType, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "POU Type {:?} does not support a return type. Did you mean Function?",
                pou_type
            ),
            range,
            err_no: ErrNo::pou__unexpected_return_type,
        }
    }

    pub fn function_unsupported_return_type(data_type: &DataTypeDeclaration) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Data Type {:?} not supported as a function return type!",
                data_type
            ),
            range: data_type.get_location(),
            err_no: ErrNo::pou__unsupported_return_type,
        }
    }

    pub fn function_return_missing(range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Function Return type missing".into(),
            range,
            err_no: ErrNo::pou__missing_return_type,
        }
    }

    pub fn missing_function(location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Cannot generate code outside of function context.".into(),
            range: location,
            err_no: ErrNo::codegen__missing_function,
        }
    }

    pub fn missing_token(epxected_token: &str, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Missing expected Token {}", epxected_token),
            range,
            err_no: ErrNo::syntax__missing_token,
        }
    }

    pub fn missing_action_container(range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Missing Actions Container Name".to_string(),
            range,
            err_no: ErrNo::undefined,
        }
    }

    pub fn unresolved_reference(reference: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Could not resolve reference to {:}", reference),
            range: location,
            err_no: ErrNo::reference__unresolved,
        }
    }

    pub fn unknown_type(type_name: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Unknown type: {:}", type_name),
            range: location,
            err_no: ErrNo::type__unknown_type,
        }
    }

    pub fn casting_error(type_name: &str, target_type: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Cannot cast from {:} to {:}", type_name, target_type),
            range: location,
            err_no: ErrNo::type__cast_error,
        }
    }

    pub fn incompatible_directaccess(
        access_type: &str,
        access_size: u64,
        location: SourceRange,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "{}-Wise access requires a Numerical type larger than {} bits",
                access_type, access_size
            ),
            range: location,
            err_no: ErrNo::type__incompatible_directaccess,
        }
    }

    pub fn incompatible_directaccess_range(
        access_type: &str,
        target_type: &str,
        access_range: Range<u64>,
        location: SourceRange,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "{}-Wise access for type {} must be in the range {}..{}",
                access_type, target_type, access_range.start, access_range.end
            ),
            range: location,
            err_no: ErrNo::type__incompatible_directaccess_range,
        }
    }

    pub fn incompatible_directaccess_variable(
        access_type: &str,
        location: SourceRange,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Invalid type {} for direct variable access. Only variables of Integer types are allowed",
                access_type
            ),
            range: location,
            err_no: ErrNo::type__incompatible_directaccess_variable,
        }
    }

    pub fn incompatible_literal_cast(
        cast_type: &str,
        literal_type: &str,
        location: SourceRange,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Literal {:} is not campatible to {:}",
                literal_type, cast_type
            ),
            range: location,
            err_no: ErrNo::type__incompatible_literal_cast,
        }
    }

    pub fn literal_expected(location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Expected literal".into(),
            range: location,
            err_no: ErrNo::type__expected_literal,
        }
    }

    pub fn literal_out_of_range(
        literal: &str,
        range_hint: &str,
        location: SourceRange,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Literal {:} out of range ({})", literal, range_hint),
            range: location,
            err_no: ErrNo::type__literal_out_of_range,
        }
    }

    pub fn empty_variable_block(location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Variable block is empty".into(),
            range: location,
            err_no: ErrNo::pou__empty_variable_block,
        }
    }

    pub fn unresolved_constant(
        constant_name: &str,
        reason: Option<&str>,
        location: SourceRange,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Unresolved constant '{:}' variable{:}",
                constant_name,
                reason
                    .map(|it| format!(": {:}", it))
                    .unwrap_or_else(|| "".into()),
            ),
            range: location,
            err_no: ErrNo::pou__empty_variable_block,
        }
    }

    pub fn invalid_constant_block(location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "This variable block does not support the CONSTANT modifier".to_string(),
            range: location,
            err_no: ErrNo::var__invalid_constant_block,
        }
    }

    pub fn invalid_constant(constant_name: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Invalid constant {:} - Functionblock- and Class-instances cannot be delcared constant", constant_name),
            range: location,
            err_no: ErrNo::var__invalid_constant,
        }
    }

    pub fn cannot_assign_to_constant(qualified_name: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Cannot assign to CONSTANT '{:}'", qualified_name),
            range: location,
            err_no: ErrNo::var__cannot_assign_to_const,
        }
    }

    pub fn cannot_generate_initializer(variable_name: &str, location: SourceRange) -> Diagnostic {
        Self::codegen_error(
            &format!(
                "Cannot generate literal initializer for '{:}': Value can not be derived",
                variable_name
            ),
            location,
        )
    }

    pub fn codegen_error(message: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: message.into(),
            range: location,
            err_no: ErrNo::codegen__general,
        }
    }

    pub fn io_read_error(file: &str, reason: &str) -> Diagnostic {
        Diagnostic::GeneralError {
            message: format!("Cannot read file '{:}': {:}'", file, reason),
            err_no: ErrNo::general__io_err,
        }
    }

    pub fn io_write_error(file: &str, reason: &str) -> Diagnostic {
        Diagnostic::GeneralError {
            message: format!("Cannot write file '{:}': {:}'", file, reason),
            err_no: ErrNo::general__io_err,
        }
    }

    pub fn llvm_error(file: &str, llvm_error: &LLVMString) -> Diagnostic {
        Diagnostic::GeneralError {
            message: format!(
                "{:}: Internal llvm error: {:}",
                file,
                llvm_error.to_string()
            ),
            err_no: ErrNo::general__io_err,
        }
    }

    pub fn get_message(&self) -> &str {
        match self {
            Diagnostic::SyntaxError { message, .. }
            | Diagnostic::ImprovementSuggestion { message, .. }
            | Diagnostic::GeneralError { message, .. } => message.as_str(),
        }
    }

    pub fn get_location(&self) -> SourceRange {
        match self {
            Diagnostic::SyntaxError { range, .. }
            | Diagnostic::ImprovementSuggestion { range, .. } => range.clone(),
            Diagnostic::GeneralError { .. } => SourceRange::undefined(),
        }
    }

    pub fn get_type(&self) -> &ErrNo {
        match self {
            Diagnostic::SyntaxError { err_no, .. } | Diagnostic::GeneralError { err_no, .. } => {
                err_no
            }
            Diagnostic::ImprovementSuggestion { .. } => &ErrNo::undefined,
        }
    }

    /**
     * relocates the given diagnostic to the given location if possible and returns it back
     */
    pub fn relocate(it: Diagnostic, new_location: SourceRange) -> Diagnostic {
        match it {
            Diagnostic::SyntaxError {
                message, err_no, ..
            } => Diagnostic::SyntaxError {
                message,
                range: new_location,
                err_no,
            },
            Diagnostic::ImprovementSuggestion { message, .. } => {
                Diagnostic::ImprovementSuggestion {
                    message,
                    range: new_location,
                }
            }
            _ => it,
        }
    }
}

pub enum Severity {
    Error,
    Warning,
    _Info,
}

pub trait DiagnosticAssessor {
    /// determines the severity of the given diagnostic
    fn assess(&self, d: Diagnostic) -> AssessedDiagnostic;
    fn assess_all(&self, d: Vec<Diagnostic>) -> Vec<AssessedDiagnostic> {
        d.into_iter().map(|it| self.assess(it)).collect()
    }
}

pub struct DefaultDiagnostician {}

impl DefaultDiagnostician {
    pub fn new() -> Self {
        DefaultDiagnostician {}
    }
}

pub struct AssessedDiagnostic {
    pub diagnostic: Diagnostic,
    pub severity: Severity,
}

impl DiagnosticAssessor for DefaultDiagnostician {
    fn assess(&self, d: Diagnostic) -> AssessedDiagnostic {
        let severity = match d {
            // improvements become warnings
            Diagnostic::ImprovementSuggestion { .. } => Severity::Warning,
            // everything else becomes an error
            _ => Severity::Error,
        };

        AssessedDiagnostic {
            diagnostic: d,
            severity,
        }
    }
}

pub trait DiagnosticReporter {
    /// reports the given diagnostic
    fn report(&self, diagnostics: &[AssessedDiagnostic], file_id: usize);
}

pub struct StdOutDiagnosticReporter {
    files: SimpleFiles<String, String>,
}

impl StdOutDiagnosticReporter {
    pub fn new(files: SimpleFiles<String, String>) -> Self {
        StdOutDiagnosticReporter { files }
    }
}

impl DiagnosticReporter for StdOutDiagnosticReporter {
    fn report(&self, diagnostics: &[AssessedDiagnostic], file_id: usize) {
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config {
            display_style: codespan_reporting::term::DisplayStyle::Rich,
            tab_width: 2,
            styles: codespan_reporting::term::Styles::default(),
            chars: codespan_reporting::term::Chars::default(),
            start_context_lines: 5,
            end_context_lines: 3,
        };

        for ad in diagnostics {
            let d = &ad.diagnostic;
            let location = d.get_location();

            let diagnostic_factory = match ad.severity {
                Severity::Error => codespan_reporting::diagnostic::Diagnostic::error(),
                Severity::Warning => codespan_reporting::diagnostic::Diagnostic::warning(),
                Severity::_Info => codespan_reporting::diagnostic::Diagnostic::note(),
            };

            let diag = diagnostic_factory
                .with_message(d.get_message())
                .with_labels(vec![Label::primary(
                    file_id,
                    location.get_start()..location.get_end(),
                )]);
            let result =
                codespan_reporting::term::emit(&mut writer.lock(), &config, &self.files, &diag);
            if let Err(err) = result {
                println!("Unable to report diagnostics: {}", err);
            }
        }
    }
}
