use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display},
    ops::Range,
};

use codespan_reporting::{
    diagnostic::Label,
    files::{Files, Location, SimpleFile, SimpleFiles},
    term::termcolor::{ColorChoice, StandardStream},
};
use inkwell::support::LLVMString;

use crate::{
    ast::{DataTypeDeclaration, DiagnosticInfo, PouType, SourceRange},
    index::VariableType,
};

pub const INTERNAL_LLVM_ERROR: &str = "internal llvm codegen error";

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Diagnostic {
    SyntaxError { message: String, range: Vec<SourceRange>, err_no: ErrNo },
    SemanticError { message: String, range: Vec<SourceRange>, err_no: ErrNo },
    GeneralError { message: String, err_no: ErrNo },
    ImprovementSuggestion { message: String, range: Vec<SourceRange> },
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ErrNo {
    undefined,

    //general
    general__io_err,
    general__param_err,
    duplicate_symbol,

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
    pou__missing_action_container,
    pou__recursive_data_structure,

    // call
    call__invalid_parameter_type,

    //variable related
    var__unresolved_constant,
    var__invalid_constant_block,
    var__invalid_constant,
    var__cannot_assign_to_const,
    var__invalid_assignment,
    var__missing_type,

    //array related
    arr__invalid_array_assignment,

    //reference related
    reference__unresolved,
    reference__illegal_access,
    reference__expected,

    //type related
    type__cast_error,
    type__unknown_type,
    type__invalid_type,
    type__literal_out_of_range,
    type__incompatible_literal_cast,
    type__incompatible_directaccess,
    type__incompatible_directaccess_variable,
    type__incompatible_directaccess_range,
    type__incompatible_arrayaccess_range,
    type__incompatible_arrayaccess_variable,
    type__incompatible_arrayaccess_type,
    type__expected_literal,
    type__invalid_nature,
    type__unknown_nature,
    type__unresolved_generic,
    type__incompatible_size,
    type__invalid_operation,

    //codegen related
    codegen__general,
    codegen__missing_function,
    codegen__missing_compare_function,

    //Debug code
    debug_general,
    //linker
    linker__generic_error,

    //switch case
    case__duplicate_condition,
    case__case_condition_outside_case_statement,
    case__invalid_case_condition,
}

impl<T: Error> From<T> for Diagnostic {
    fn from(e: T) -> Self {
        Diagnostic::GeneralError { message: e.to_string(), err_no: ErrNo::general__io_err }
    }
}

impl Diagnostic {
    /// Creates a new diagnostic with additional ranges
    pub fn with_extra_ranges(&self, ranges: &[SourceRange]) -> Diagnostic {
        match self {
            Diagnostic::SyntaxError { message, range, err_no } => {
                let mut range = range.to_vec();
                range.extend_from_slice(ranges);
                Diagnostic::SyntaxError { message: message.to_string(), range, err_no: *err_no }
            }
            Diagnostic::SemanticError { message, range, err_no } => {
                let mut range = range.to_vec();
                range.extend_from_slice(ranges);
                Diagnostic::SyntaxError { message: message.to_string(), range, err_no: *err_no }
            }
            Diagnostic::ImprovementSuggestion { message, range } => {
                let mut range = range.to_vec();
                range.extend_from_slice(ranges);
                Diagnostic::ImprovementSuggestion { message: message.to_string(), range }
            }
            Diagnostic::GeneralError { .. } => self.clone(),
        }
    }

    pub fn get_affected_ranges(&self) -> &[SourceRange] {
        match self {
            Diagnostic::SyntaxError { range, .. }
            | Diagnostic::SemanticError { range, .. }
            | Diagnostic::ImprovementSuggestion { range, .. } => range.as_slice(),
            Diagnostic::GeneralError { .. } => &[],
        }
    }

    pub fn syntax_error(message: &str, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: message.to_string(),
            range: vec![range],
            err_no: ErrNo::syntax__generic_error,
        }
    }

    pub fn unexpected_token_found(expected: &str, found: &str, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Unexpected token: expected {expected} but found {found}"),
            range: vec![range],
            err_no: ErrNo::syntax__unexpected_token,
        }
    }

    pub fn unexpected_initializer_on_function_return(range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Return types cannot have a default value".into(),
            range: vec![range],
            err_no: ErrNo::syntax__unexpected_token,
        }
    }

    pub fn return_type_not_supported(pou_type: &PouType, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("POU Type {pou_type:?} does not support a return type. Did you mean Function?"),
            range: vec![range],
            err_no: ErrNo::pou__unexpected_return_type,
        }
    }

    pub fn function_unsupported_return_type(data_type: &DataTypeDeclaration) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Data Type {data_type:?} not supported as a function return type!"),
            range: vec![data_type.get_location()],
            err_no: ErrNo::pou__unsupported_return_type,
        }
    }

    pub fn function_return_missing(range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Function Return type missing".into(),
            range: vec![range],
            err_no: ErrNo::pou__missing_return_type,
        }
    }

    pub fn missing_function(location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Cannot generate code outside of function context.".into(),
            range: vec![location],
            err_no: ErrNo::codegen__missing_function,
        }
    }

    pub fn missing_compare_function(
        function_name: &str,
        data_type: &str,
        location: SourceRange,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Missing compare function 'FUNCTION {function_name} : BOOL VAR_INPUT a,b : {data_type}; END_VAR ...'."
            ),
            range: vec![location],
            err_no: ErrNo::codegen__missing_compare_function,
        }
    }

    pub fn missing_token(expected_token: &str, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Missing expected Token {expected_token}"),
            range: vec![range],
            err_no: ErrNo::syntax__missing_token,
        }
    }

    pub fn missing_action_container(range: SourceRange) -> Diagnostic {
        Diagnostic::ImprovementSuggestion {
            message: "Missing Actions Container Name".to_string(),
            range: vec![range],
        }
    }

    pub fn unresolved_reference(reference: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Could not resolve reference to {reference:}"),
            range: vec![location],
            err_no: ErrNo::reference__unresolved,
        }
    }

    pub fn illegal_access(reference: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Illegal access to private member {reference:}"),
            range: vec![location],
            err_no: ErrNo::reference__illegal_access,
        }
    }

    pub fn unresolved_generic_type(symbol: &str, nature: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Could not resolve generic type {symbol} with nature {nature}"),
            range: vec![location],
            err_no: ErrNo::type__unresolved_generic,
        }
    }

    pub fn unknown_type(type_name: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Unknown type: {type_name:}"),
            range: vec![location],
            err_no: ErrNo::type__unknown_type,
        }
    }

    pub fn casting_error(type_name: &str, target_type: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Cannot cast from {type_name:} to {target_type:}"),
            range: vec![location],
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
                "{access_type}-Wise access requires a Numerical type larger than {access_size} bits"
            ),
            range: vec![location],
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
                "{access_type}-Wise access for type {target_type} must be in the range {}..{}",
                access_range.start, access_range.end
            ),
            range: vec![location],
            err_no: ErrNo::type__incompatible_directaccess_range,
        }
    }

    pub fn incompatible_directaccess_variable(access_type: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Invalid type {access_type} for direct variable access. Only variables of Integer types are allowed",
            ),
            range: vec![location],
            err_no: ErrNo::type__incompatible_directaccess_variable,
        }
    }

    pub fn incompatible_array_access_range(range: Range<i64>, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Array access must be in the range {}..{}", range.start, range.end),
            range: vec![location],
            err_no: ErrNo::type__incompatible_arrayaccess_range,
        }
    }

    pub fn incompatible_array_access_variable(access_type: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Invalid type {access_type} for array access. Only variables of Array types are allowed",
            ),
            range: vec![location],
            err_no: ErrNo::type__incompatible_arrayaccess_variable,
        }
    }

    pub fn incompatible_array_access_type(access_type: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Invalid type {access_type} for array access. Only variables of Integer types are allowed to access an array",
            ),
            range: vec![location],
            err_no: ErrNo::type__incompatible_arrayaccess_variable,
        }
    }

    pub fn incompatible_literal_cast(
        cast_type: &str,
        literal_type: &str,
        location: SourceRange,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Literal {literal_type} is not campatible to {cast_type}"),
            range: vec![location],
            err_no: ErrNo::type__incompatible_literal_cast,
        }
    }

    pub fn literal_expected(location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Expected literal".into(),
            range: vec![location],
            err_no: ErrNo::type__expected_literal,
        }
    }

    pub fn literal_out_of_range(literal: &str, range_hint: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Literal {literal} out of range ({range_hint})"),
            range: vec![location],
            err_no: ErrNo::type__literal_out_of_range,
        }
    }

    pub fn reference_expected(location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Expression is not assignable".into(),
            range: vec![location],
            err_no: ErrNo::reference__expected,
        }
    }

    pub fn empty_variable_block(location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Variable block is empty".into(),
            range: vec![location],
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
                "Unresolved constant '{constant_name:}' variable{:}",
                reason.map(|it| format!(": {it}",)).unwrap_or_else(|| "".into()),
            ),
            range: vec![location],
            err_no: ErrNo::var__unresolved_constant,
        }
    }

    pub fn invalid_constant_block(location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "This variable block does not support the CONSTANT modifier".to_string(),
            range: vec![location],
            err_no: ErrNo::var__invalid_constant_block,
        }
    }

    pub fn invalid_constant(constant_name: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Invalid constant {constant_name} - Functionblock- and Class-instances cannot be delcared constant",
            ),
            range: vec![location],
            err_no: ErrNo::var__invalid_constant,
        }
    }

    pub fn cannot_assign_to_constant(qualified_name: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Cannot assign to CONSTANT '{qualified_name}'"),
            range: vec![location],
            err_no: ErrNo::var__cannot_assign_to_const,
        }
    }

    pub fn cannot_generate_initializer(variable_name: &str, location: SourceRange) -> Diagnostic {
        Self::codegen_error(
            &format!("Cannot generate literal initializer for '{variable_name}': Value cannot be derived"),
            location,
        )
    }

    pub fn codegen_error(message: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: message.into(),
            range: vec![location],
            err_no: ErrNo::codegen__general,
        }
    }

    pub fn debug_error<T: Into<String>>(message: T) -> Diagnostic {
        Diagnostic::GeneralError { message: message.into(), err_no: ErrNo::debug_general }
    }

    pub fn cannot_generate_call_statement<T: DiagnosticInfo>(operator: &T) -> Diagnostic {
        Diagnostic::codegen_error(
            &format!("cannot generate call statement for {:?}", operator.get_description()),
            operator.get_location(),
        )
    }

    pub fn io_read_error(file: &str, reason: &str) -> Diagnostic {
        Diagnostic::GeneralError {
            message: format!("Cannot read file '{file}': {reason}'"),
            err_no: ErrNo::general__io_err,
        }
    }

    pub fn io_write_error(file: &str, reason: &str) -> Diagnostic {
        Diagnostic::GeneralError {
            message: format!("Cannot write file {file} {reason}'"),
            err_no: ErrNo::general__io_err,
        }
    }

    pub fn param_error(reason: &str) -> Diagnostic {
        Diagnostic::GeneralError { message: reason.to_string(), err_no: ErrNo::general__param_err }
    }

    pub fn llvm_error(file: &str, llvm_error: &LLVMString) -> Diagnostic {
        Diagnostic::GeneralError {
            message: format!("{file}: Internal llvm error: {:}", llvm_error.to_string()),
            err_no: ErrNo::general__io_err,
        }
    }

    pub fn cannot_generate_from_empty_literal(type_name: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::codegen_error(
            format!("Cannot generate {type_name} from empty literal").as_str(),
            location,
        )
    }

    pub fn cannot_generate_string_literal(type_name: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::codegen_error(
            format!("Cannot generate String-Literal for type {type_name}").as_str(),
            location,
        )
    }

    pub fn invalid_assignment(right_type: &str, left_type: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Invalid assignment: cannot assign '{right_type}' to '{left_type}'"),
            range: vec![location],
            err_no: ErrNo::var__invalid_assignment,
        }
    }

    pub fn invalid_type_nature(actual: &str, expected: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Invalid type nature for generic argument. {actual} is no {expected}."),
            range: vec![location],
            err_no: ErrNo::type__invalid_nature,
        }
    }

    pub fn unknown_type_nature(nature: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Unknown type nature {nature}."),
            range: vec![location],
            err_no: ErrNo::type__unknown_nature,
        }
    }

    pub fn missing_datatype(reason: Option<&str>, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Missing datatype {}", reason.unwrap_or("")),
            range: vec![location],
            err_no: ErrNo::var__missing_type,
        }
    }

    pub fn incompatible_type_size(nature: &str, size: u32, error: &str, location: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("The type {nature} {size} is too small to {error} Pointer"),
            range: vec![location],
            err_no: ErrNo::type__incompatible_size,
        }
    }

    pub fn link_error(error: &str) -> Diagnostic {
        Diagnostic::GeneralError { err_no: ErrNo::linker__generic_error, message: error.to_string() }
    }

    pub fn get_message(&self) -> &str {
        match self {
            Diagnostic::SyntaxError { message, .. }
            | Diagnostic::SemanticError { message, .. }
            | Diagnostic::ImprovementSuggestion { message, .. }
            | Diagnostic::GeneralError { message, .. } => message.as_str(),
        }
    }

    pub fn get_location(&self) -> SourceRange {
        match self {
            Diagnostic::SyntaxError { range, .. }
            | Diagnostic::SemanticError { range, .. }
            | Diagnostic::ImprovementSuggestion { range, .. } => {
                range.get(0).cloned().unwrap_or_else(SourceRange::undefined)
            }
            Diagnostic::GeneralError { .. } => SourceRange::undefined(),
        }
    }

    pub fn get_secondary_locations(&self) -> Option<&[SourceRange]> {
        match self {
            Diagnostic::SyntaxError { range, .. }
            | Diagnostic::SemanticError { range, .. }
            | Diagnostic::ImprovementSuggestion { range, .. }
                if range.len() > 1 =>
            {
                Some(&range[1..])
            }
            _ => None,
        }
    }

    pub fn get_type(&self) -> &ErrNo {
        match self {
            Diagnostic::SyntaxError { err_no, .. }
            | Diagnostic::SemanticError { err_no, .. }
            | Diagnostic::GeneralError { err_no, .. } => err_no,
            Diagnostic::ImprovementSuggestion { .. } => &ErrNo::undefined,
        }
    }

    /**
     * relocates the given diagnostic to the given location if possible and returns it back
     */
    pub fn relocate(it: Diagnostic, new_location: SourceRange) -> Diagnostic {
        match it {
            Diagnostic::SyntaxError { message, err_no, .. } => {
                Diagnostic::SyntaxError { message, range: vec![new_location], err_no }
            }
            Diagnostic::ImprovementSuggestion { message, .. } => {
                Diagnostic::ImprovementSuggestion { message, range: vec![new_location] }
            }
            _ => it,
        }
    }

    pub fn invalid_pragma_location(message: &str, range: SourceRange) -> Diagnostic {
        Diagnostic::ImprovementSuggestion {
            message: format!("Invalid pragma location: {message}"),
            range: vec![range],
        }
    }

    pub fn non_constant_case_condition(case: &str, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("{case}. Non constant variables are not supported in case conditions"),
            range: vec![range],
            err_no: ErrNo::type__invalid_type,
        }
    }

    pub fn duplicate_case_condition(value: &i128, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Duplicate condition value: {value}. Occurred more than once!"),
            range: vec![range],
            err_no: ErrNo::case__duplicate_condition,
        }
    }

    pub fn case_condition_used_outside_case_statement(range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Case condition used outside of case statement! Did you mean to use ';'?".into(),
            range: vec![range],
            err_no: ErrNo::case__case_condition_outside_case_statement,
        }
    }

    pub fn invalid_case_condition(range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Invalid case condition!".into(),
            range: vec![range],
            err_no: ErrNo::case__case_condition_outside_case_statement,
        }
    }

    pub fn missing_inout_parameter(parameter: &str, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Missing inout parameter: {parameter}"),
            range: vec![range],
            err_no: ErrNo::pou__missing_action_container,
        }
    }

    pub fn invalid_parameter_type(range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Cannot mix implicit and explicit call parameters!".into(),
            range: vec![range],
            err_no: ErrNo::call__invalid_parameter_type,
        }
    }

    pub fn invalid_argument_type(
        parameter_name: &str,
        parameter_type: VariableType,
        range: SourceRange,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Expected a reference for parameter {parameter_name} because their type is {parameter_type:?}"
            ),
            range: vec![range],
            err_no: ErrNo::call__invalid_parameter_type,
        }
    }

    pub fn duplicate_pou(name: &str, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Duplicate POU {name}"),
            range: vec![range],
            err_no: ErrNo::duplicate_symbol,
        }
    }

    pub(crate) fn global_name_conflict(
        name: &str,
        location: SourceRange,
        conflicts: Vec<SourceRange>,
    ) -> Diagnostic {
        Diagnostic::global_name_conflict_with_text(name, location, conflicts, "Duplicate symbol.")
    }

    pub(crate) fn global_name_conflict_with_text(
        name: &str,
        location: SourceRange,
        conflicts: Vec<SourceRange>,
        additional_text: &str,
    ) -> Diagnostic {
        let mut locations = vec![location];
        locations.extend(conflicts.into_iter());
        Diagnostic::SyntaxError {
            message: format!("{name}: {additional_text}"),
            range: locations,
            err_no: ErrNo::duplicate_symbol,
        }
    }

    pub fn invalid_operation(message: &str, range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: message.to_string(),
            range: vec![range],
            err_no: ErrNo::type__invalid_operation,
        }
    }

    pub fn array_expected_initializer_list(range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Array initializer must be an initializer list!".to_string(),
            range: vec![range],
            err_no: ErrNo::arr__invalid_array_assignment,
        }
    }

    pub fn array_expected_identifier_or_round_bracket(range: SourceRange) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Expected identifier or '('".to_string(),
            range: vec![range],
            err_no: ErrNo::arr__invalid_array_assignment,
        }
    }

    pub fn recursive_datastructure(path: &str, range: Vec<SourceRange>) -> Diagnostic {
        Diagnostic::SemanticError {
            message: format!("Recursive data structure `{path}` has infinite size"),
            range,
            err_no: ErrNo::pou__recursive_data_structure,
        }
    }

    pub fn implicit_downcast(
        actual_type_name: &str,
        assigned_type_name: &str,
        range: SourceRange,
    ) -> Diagnostic {
        Diagnostic::ImprovementSuggestion {
            message: format!(
                "Potential loss of information due to assigning '{assigned_type_name}' to variable of type '{actual_type_name}'."
            ),
            range: vec![range],
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

impl Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let severity = match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::_Info => "info",
        };
        write!(f, "{severity}")
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
pub struct DefaultDiagnosticAssessor {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedLocation {
    pub file_handle: usize,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedDiagnostics {
    pub message: String,
    pub severity: Severity,
    pub main_location: ResolvedLocation,
    pub additional_locations: Option<Vec<ResolvedLocation>>,
}

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

/// the DiagnosticReporter decides on the format and where to report the diagnostic to.
/// possible implementations could print to either std-out, std-err or a file, etc.
pub trait DiagnosticReporter {
    /// reports the given diagnostic
    fn report(&self, diagnostics: &[ResolvedDiagnostics]);
    /// register the given path & src and returns an ID to indicate
    /// a relationship the given src (diagnostics for this src need
    /// to use this id)
    fn register(&mut self, path: String, src: String) -> usize;
}

/// a DiagnosticReporter that reports diagnostics using codespan_reporting
pub struct CodeSpanDiagnosticReporter {
    files: SimpleFiles<String, String>,
    config: codespan_reporting::term::Config,
    writer: StandardStream,
}

impl CodeSpanDiagnosticReporter {
    /// creates a new reporter with the given codespan_reporting configuration
    fn new(config: codespan_reporting::term::Config, writer: StandardStream) -> Self {
        CodeSpanDiagnosticReporter { files: SimpleFiles::new(), config, writer }
    }
}

impl Default for CodeSpanDiagnosticReporter {
    /// creates the default CodeSpanDiagnosticReporter reporting to StdErr, with colors
    fn default() -> Self {
        Self::new(
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
    fn report(&self, diagnostics: &[ResolvedDiagnostics]) {
        for d in diagnostics {
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

            let result =
                codespan_reporting::term::emit(&mut self.writer.lock(), &self.config, &self.files, &diag);
            if result.is_err() {
                eprintln!("<internal>: {}", d.message);
            }
        }
    }

    fn register(&mut self, path: String, src: String) -> usize {
        self.files.add(path, src)
    }
}

/// a DiagnosticReporter that reports diagnostics using clang-format
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
    fn report(&self, diagnostics: &[ResolvedDiagnostics]) {
        for diagnostic in diagnostics {
            let file_id = diagnostic.main_location.file_handle;
            let location = &diagnostic.main_location;

            let file = self.files.get(file_id).ok();
            let start = self.files.location(file_id, location.range.start).ok();
            let end = self.files.location(file_id, location.range.end).ok();

            let res = self.build_diagnostic_msg(
                file,
                start.as_ref(),
                end.as_ref(),
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
    fn build_diagnostic_msg(
        &self,
        file: Option<&SimpleFile<String, String>>,
        start: Option<&Location>,
        end: Option<&Location>,
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
        str.push_str(format!("{severity}: ").as_str());
        // msg
        str.push_str(msg);

        str
    }
}

/// a DiagnosticReporter that swallows all diagnostics
#[derive(Default)]
pub struct NullDiagnosticReporter {
    last_id: usize,
}

impl DiagnosticReporter for NullDiagnosticReporter {
    fn report(&self, _diagnostics: &[ResolvedDiagnostics]) {
        //ignore
    }

    fn register(&mut self, _path: String, _src: String) -> usize {
        // at least provide some unique ids
        self.last_id += 1;
        self.last_id
    }
}

/// the Diagnostician handle's Diangostics with the help of a
/// assessor and a reporter
pub struct Diagnostician {
    pub reporter: Box<dyn DiagnosticReporter>,
    pub assessor: Box<dyn DiagnosticAssessor>,
    pub(crate) filename_fileid_mapping: HashMap<String, usize>,
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

    /// creates a null-diagnostician that does not report diagnostics
    pub fn null_diagnostician() -> Diagnostician {
        Diagnostician {
            assessor: Box::<DefaultDiagnosticAssessor>::default(),
            reporter: Box::<NullDiagnosticReporter>::default(),
            filename_fileid_mapping: HashMap::new(),
        }
    }

    /// creates a clang-format-diagnostician that reports diagnostics in clang format
    pub fn clang_format_diagnostician() -> Diagnostician {
        Diagnostician {
            reporter: Box::<ClangFormatDiagnosticReporter>::default(),
            assessor: Box::<DefaultDiagnosticAssessor>::default(),
            filename_fileid_mapping: HashMap::new(),
        }
    }

    /// assess and reports the given diagnostics
    pub fn handle(&self, diagnostics: Vec<Diagnostic>) {
        let resolved_diagnostics = diagnostics.iter().map(|d| ResolvedDiagnostics {
            message: d.get_message().to_string(),
            severity: self.assess(d),
            main_location: ResolvedLocation {
                file_handle: self
                    .get_file_handle(d.get_location().get_file_name())
                    .unwrap_or(usize::max_value()),
                range: d.get_location().to_range(),
            },
            additional_locations: d.get_secondary_locations().map(|it| {
                it.iter()
                    .map(|l| ResolvedLocation {
                        file_handle: self.get_file_handle(l.get_file_name()).unwrap_or(usize::max_value()),
                        range: l.to_range(),
                    })
                    .collect()
            }),
        });

        self.report(resolved_diagnostics.collect::<Vec<_>>().as_slice());
    }

    fn get_file_handle(&self, file_name: Option<&str>) -> Option<usize> {
        file_name.and_then(|it| self.filename_fileid_mapping.get(it).cloned())
    }
}

impl DiagnosticReporter for Diagnostician {
    fn report(&self, diagnostics: &[ResolvedDiagnostics]) {
        //delegate to reporter
        self.reporter.report(diagnostics);
    }

    fn register(&mut self, path: String, src: String) -> usize {
        //delegate to reporter
        self.reporter.register(path, src)
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

#[cfg(test)]
mod diagnostics_tests {
    use codespan_reporting::files::{Location, SimpleFile};

    use super::ClangFormatDiagnosticReporter;

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
            &super::Severity::Error,
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
            &super::Severity::Error,
            "This is an error",
        );

        assert_eq!(res, "test.st:4:1: error: This is an error");
    }

    #[test]
    fn test_build_diagnostic_msg_no_location() {
        let reporter = ClangFormatDiagnosticReporter::default();
        let file = SimpleFile::new("test.st".to_string(), "source".to_string());
        let res = reporter.build_diagnostic_msg(
            Some(&file),
            None,
            None,
            &super::Severity::Error,
            "This is an error",
        );

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
            &super::Severity::Error,
            "This is an error",
        );

        assert_eq!(res, "error: This is an error");
    }

    #[test]
    fn test_build_diagnostic_msg_no_file_no_location() {
        let reporter = ClangFormatDiagnosticReporter::default();
        let res =
            reporter.build_diagnostic_msg(None, None, None, &super::Severity::Error, "This is an error");

        assert_eq!(res, "error: This is an error");
    }
}
