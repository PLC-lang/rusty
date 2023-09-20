use std::{error::Error, fmt::Display, ops::Range};

use plc_ast::ast::{AstNode, DataTypeDeclaration, DiagnosticInfo, PouType};
use plc_source::source_location::SourceLocation;

use crate::errno::ErrNo;

pub const INTERNAL_LLVM_ERROR: &str = "internal llvm codegen error";

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Diagnostic {
    SyntaxError { message: String, range: Vec<SourceLocation>, err_no: ErrNo },
    SemanticError { message: String, range: Vec<SourceLocation>, err_no: ErrNo },
    GeneralError { message: String, err_no: ErrNo },
    ImprovementSuggestion { message: String, range: Vec<SourceLocation> },
    CombinedDiagnostic { message: String, inner_diagnostics: Vec<Diagnostic>, err_no: ErrNo },
}

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

impl<T: Error> From<T> for Diagnostic {
    fn from(e: T) -> Self {
        Diagnostic::GeneralError { message: e.to_string(), err_no: ErrNo::general__io_err }
    }
}

impl Diagnostic {
    /// Creates a new diagnostic with additional ranges
    pub fn with_extra_ranges(self, ranges: &[SourceLocation]) -> Diagnostic {
        match self {
            Diagnostic::SyntaxError { message, mut range, err_no } => {
                range.extend_from_slice(ranges);
                Diagnostic::SyntaxError { message, range, err_no }
            }
            Diagnostic::SemanticError { message, mut range, err_no } => {
                range.extend_from_slice(ranges);
                Diagnostic::SyntaxError { message, range, err_no }
            }
            Diagnostic::ImprovementSuggestion { message, mut range } => {
                range.extend_from_slice(ranges);
                Diagnostic::ImprovementSuggestion { message, range }
            }
            _ => self,
        }
    }

    pub fn syntax_error(message: &str, range: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: message.to_string(),
            range: vec![range],
            err_no: ErrNo::syntax__generic_error,
        }
    }

    pub fn unexpected_token_found(expected: &str, found: &str, range: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Unexpected token: expected {expected} but found {found}"),
            range: vec![range],
            err_no: ErrNo::syntax__unexpected_token,
        }
    }

    pub fn unexpected_initializer_on_function_return(range: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Return types cannot have a default value".into(),
            range: vec![range],
            err_no: ErrNo::syntax__unexpected_token,
        }
    }

    pub fn return_type_not_supported(pou_type: &PouType, range: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("POU Type {pou_type:?} does not support a return type. Did you mean Function?"),
            range: vec![range],
            err_no: ErrNo::pou__unexpected_return_type,
        }
    }

    //TODO: This prints a debug version of the datatype, it should have a user readable version instead
    pub fn function_unsupported_return_type(data_type: &DataTypeDeclaration) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Data Type {data_type:?} not supported as a function return type!"),
            range: vec![data_type.get_location()],
            err_no: ErrNo::pou__unsupported_return_type,
        }
    }

    pub fn function_return_missing(range: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Function Return type missing".into(),
            range: vec![range],
            err_no: ErrNo::pou__missing_return_type,
        }
    }

    pub fn missing_function(location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Cannot generate code outside of function context.".into(),
            range: vec![location],
            err_no: ErrNo::codegen__missing_function,
        }
    }

    pub fn missing_compare_function(
        function_name: &str,
        data_type: &str,
        location: SourceLocation,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Missing compare function 'FUNCTION {function_name} : BOOL VAR_INPUT a,b : {data_type}; END_VAR ...'."
            ),
            range: vec![location],
            err_no: ErrNo::codegen__missing_compare_function,
        }
    }

    pub fn missing_token(expected_token: &str, range: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Missing expected Token {expected_token}"),
            range: vec![range],
            err_no: ErrNo::syntax__missing_token,
        }
    }

    pub fn missing_action_container(range: SourceLocation) -> Diagnostic {
        Diagnostic::ImprovementSuggestion {
            message: "Missing Actions Container Name".to_string(),
            range: vec![range],
        }
    }

    pub fn unresolved_reference(reference: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Could not resolve reference to {reference:}"),
            range: vec![location],
            err_no: ErrNo::reference__unresolved,
        }
    }

    pub fn illegal_access(reference: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Illegal access to private member {reference:}"),
            range: vec![location],
            err_no: ErrNo::reference__illegal_access,
        }
    }

    pub fn unresolved_generic_type(symbol: &str, nature: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Could not resolve generic type {symbol} with nature {nature}"),
            range: vec![location],
            err_no: ErrNo::type__unresolved_generic,
        }
    }

    pub fn unknown_type(type_name: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Unknown type: {type_name:}"),
            range: vec![location],
            err_no: ErrNo::type__unknown_type,
        }
    }

    pub fn casting_error(type_name: &str, target_type: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Cannot cast from {type_name:} to {target_type:}"),
            range: vec![location],
            err_no: ErrNo::type__cast_error,
        }
    }

    pub fn incompatible_directaccess(
        access_type: &str,
        access_size: u64,
        location: SourceLocation,
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
        location: SourceLocation,
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

    pub fn incompatible_directaccess_variable(access_type: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Invalid type {access_type} for direct variable access. Only variables of Integer types are allowed",
            ),
            range: vec![location],
            err_no: ErrNo::type__incompatible_directaccess_variable,
        }
    }

    pub fn incompatible_array_access_range(range: Range<i64>, location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Array access must be in the range {}..{}", range.start, range.end),
            range: vec![location],
            err_no: ErrNo::type__incompatible_arrayaccess_range,
        }
    }

    pub fn incompatible_array_access_variable(access_type: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Invalid type {access_type} for array access. Only variables of Array types are allowed",
            ),
            range: vec![location],
            err_no: ErrNo::type__incompatible_arrayaccess_variable,
        }
    }

    pub fn incompatible_array_access_type(access_type: &str, location: SourceLocation) -> Diagnostic {
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
        location: SourceLocation,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Literal {literal_type} is not compatible to {cast_type}"),
            range: vec![location],
            err_no: ErrNo::type__incompatible_literal_cast,
        }
    }

    pub fn literal_expected(location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Expected literal".into(),
            range: vec![location],
            err_no: ErrNo::type__expected_literal,
        }
    }

    pub fn literal_out_of_range(literal: &str, range_hint: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Literal {literal} out of range ({range_hint})"),
            range: vec![location],
            err_no: ErrNo::type__literal_out_of_range,
        }
    }

    pub fn reference_expected(location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Expression is not assignable".into(),
            range: vec![location],
            err_no: ErrNo::reference__expected,
        }
    }

    pub fn empty_variable_block(location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Variable block is empty".into(),
            range: vec![location],
            err_no: ErrNo::pou__empty_variable_block,
        }
    }

    pub fn unresolved_constant(
        constant_name: &str,
        reason: Option<&str>,
        location: SourceLocation,
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

    pub fn invalid_constant_block(location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "This variable block does not support the CONSTANT modifier".to_string(),
            range: vec![location],
            err_no: ErrNo::var__invalid_constant_block,
        }
    }

    pub fn invalid_constant(constant_name: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Invalid constant {constant_name} - Functionblock- and Class-instances cannot be delcared constant",
            ),
            range: vec![location],
            err_no: ErrNo::var__invalid_constant,
        }
    }

    pub fn cannot_assign_to_constant(qualified_name: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Cannot assign to CONSTANT '{qualified_name}'"),
            range: vec![location],
            err_no: ErrNo::var__cannot_assign_to_const,
        }
    }

    pub fn cannot_generate_initializer(variable_name: &str, location: SourceLocation) -> Diagnostic {
        Self::codegen_error(
            &format!("Cannot generate literal initializer for '{variable_name}': Value cannot be derived"),
            location,
        )
    }

    pub fn codegen_error(message: &str, location: SourceLocation) -> Diagnostic {
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

    pub fn llvm_error(file: &str, llvm_error: &str) -> Diagnostic {
        Diagnostic::GeneralError {
            message: format!("{file}: Internal llvm error: {:}", llvm_error),
            err_no: ErrNo::general__io_err,
        }
    }

    pub fn cannot_generate_from_empty_literal(type_name: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::codegen_error(
            format!("Cannot generate {type_name} from empty literal").as_str(),
            location,
        )
    }

    pub fn cannot_generate_string_literal(type_name: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::codegen_error(
            format!("Cannot generate String-Literal for type {type_name}").as_str(),
            location,
        )
    }

    pub fn invalid_assignment(right_type: &str, left_type: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Invalid assignment: cannot assign '{right_type}' to '{left_type}'"),
            range: vec![location],
            err_no: ErrNo::var__invalid_assignment,
        }
    }

    pub fn invalid_type_nature(actual: &str, expected: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Invalid type nature for generic argument. {actual} is no {expected}."),
            range: vec![location],
            err_no: ErrNo::type__invalid_nature,
        }
    }

    pub fn unknown_type_nature(nature: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Unknown type nature {nature}."),
            range: vec![location],
            err_no: ErrNo::type__unknown_nature,
        }
    }

    pub fn missing_datatype(reason: Option<&str>, location: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Missing datatype {}", reason.unwrap_or("")),
            range: vec![location],
            err_no: ErrNo::var__missing_type,
        }
    }

    pub fn incompatible_type_size(
        nature: &str,
        size: u32,
        error: &str,
        location: SourceLocation,
    ) -> Diagnostic {
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
            | Diagnostic::GeneralError { message, .. }
            | Diagnostic::CombinedDiagnostic { message, .. } => message.as_str(),
        }
    }

    pub fn get_location(&self) -> SourceLocation {
        match self {
            Diagnostic::SyntaxError { range, .. }
            | Diagnostic::SemanticError { range, .. }
            | Diagnostic::ImprovementSuggestion { range, .. } => {
                range.get(0).cloned().unwrap_or_else(SourceLocation::undefined)
            }
            _ => SourceLocation::undefined(),
        }
    }

    pub fn get_secondary_locations(&self) -> Option<&[SourceLocation]> {
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
            | Diagnostic::GeneralError { err_no, .. }
            | Diagnostic::CombinedDiagnostic { err_no, .. } => err_no,
            Diagnostic::ImprovementSuggestion { .. } => &ErrNo::undefined,
        }
    }

    /**
     * relocates the given diagnostic to the given location if possible and returns it back
     */
    pub fn relocate(it: Diagnostic, new_location: SourceLocation) -> Diagnostic {
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

    pub fn invalid_pragma_location(message: &str, range: SourceLocation) -> Diagnostic {
        Diagnostic::ImprovementSuggestion {
            message: format!("Invalid pragma location: {message}"),
            range: vec![range],
        }
    }

    pub fn non_constant_case_condition(case: &str, range: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("{case}. Non constant variables are not supported in case conditions"),
            range: vec![range],
            err_no: ErrNo::type__invalid_type,
        }
    }

    pub fn duplicate_case_condition(value: &i128, range: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Duplicate condition value: {value}. Occurred more than once!"),
            range: vec![range],
            err_no: ErrNo::case__duplicate_condition,
        }
    }

    pub fn case_condition_used_outside_case_statement(range: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Case condition used outside of case statement! Did you mean to use ';'?".into(),
            range: vec![range],
            err_no: ErrNo::case__case_condition_outside_case_statement,
        }
    }

    pub fn invalid_case_condition(range: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Invalid case condition!".into(),
            range: vec![range],
            err_no: ErrNo::case__case_condition_outside_case_statement,
        }
    }

    pub fn missing_inout_parameter(parameter: &str, range: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Missing inout parameter: {parameter}"),
            range: vec![range],
            err_no: ErrNo::pou__missing_action_container,
        }
    }

    pub fn invalid_parameter_type(range: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Cannot mix implicit and explicit call parameters!".into(),
            range: vec![range],
            err_no: ErrNo::call__invalid_parameter_type,
        }
    }

    pub fn invalid_parameter_count(expected: usize, received: usize, range: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Invalid parameter count. Received {received} parameters while {expected} parameters were expected.",
            ),
            range: vec![range],
            err_no: ErrNo::call__invalid_parameter_count,
        }
    }

    pub fn implicit_downcast(
        actual_type_name: &str,
        assigned_type_name: &str,
        range: SourceLocation,
    ) -> Diagnostic {
        Diagnostic::ImprovementSuggestion {
            message: format!(
                "Potential loss of information due to assigning '{assigned_type_name}' to variable of type '{actual_type_name}'."
            ),
            range: vec![range],
        }
    }

    pub fn invalid_argument_type(
        parameter_name: &str,
        parameter_type: &str,
        range: SourceLocation,
    ) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!(
                "Expected a reference for parameter {parameter_name} because their type is {parameter_type}"
            ),
            range: vec![range],
            err_no: ErrNo::call__invalid_parameter_type,
        }
    }

    pub fn invalid_type_name(name: &str, range: Vec<SourceLocation>) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("{name} can not be used as a name because it is a built-in datatype"),
            range,
            err_no: ErrNo::type__invalid_name,
        }
    }

    pub fn global_name_conflict(
        name: &str,
        location: SourceLocation,
        conflicts: Vec<SourceLocation>,
    ) -> Diagnostic {
        Diagnostic::global_name_conflict_with_text(name, location, conflicts, "Duplicate symbol.")
    }

    pub fn global_name_conflict_with_text(
        name: &str,
        location: SourceLocation,
        conflicts: Vec<SourceLocation>,
        additional_text: &str,
    ) -> Diagnostic {
        let mut locations = vec![location];
        locations.extend(conflicts);
        Diagnostic::SyntaxError {
            message: format!("{name}: {additional_text}"),
            range: locations,
            err_no: ErrNo::duplicate_symbol,
        }
    }

    pub fn invalid_operation(message: &str, range: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: message.to_string(),
            range: vec![range],
            err_no: ErrNo::type__invalid_operation,
        }
    }

    pub fn array_assignment(range: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: "Array assignments must be surrounded with `[]`".to_string(),
            range: vec![range],
            err_no: ErrNo::arr__invalid_array_assignment,
        }
    }

    pub fn array_size(name: &str, len_lhs: usize, len_rhs: usize, range: SourceLocation) -> Diagnostic {
        Diagnostic::SemanticError {
            message: format!("Array {name} has a size of {len_lhs}, but {len_rhs} elements were provided"),
            range: vec![range],
            err_no: ErrNo::arr__invalid_array_assignment,
        }
    }

    pub fn recursive_datastructure(path: &str, range: Vec<SourceLocation>) -> Diagnostic {
        Diagnostic::SemanticError {
            message: format!("Recursive data structure `{path}` has infinite size"),
            range,
            err_no: ErrNo::pou__recursive_data_structure,
        }
    }

    pub fn vla_by_val_warning(range: SourceLocation) -> Diagnostic {
        Diagnostic::ImprovementSuggestion {
            message: "Variable Length Arrays are always by-ref, even when declared in a by-value block"
                .to_string(),
            range: vec![range],
        }
    }

    pub fn invalid_vla_container(message: String, range: SourceLocation) -> Diagnostic {
        Diagnostic::SemanticError { message, range: vec![range], err_no: ErrNo::vla__invalid_container }
    }

    pub fn invalid_array_access(expected: usize, actual: usize, range: SourceLocation) -> Diagnostic {
        Diagnostic::SemanticError {
            message: format!("Expected array access with {expected} dimensions, found {actual}"),
            range: vec![range],
            err_no: ErrNo::vla__invalid_array_access,
        }
    }

    pub fn invalid_range_statement(entity: &AstNode, range: SourceLocation) -> Diagnostic {
        Diagnostic::SyntaxError {
            message: format!("Expected a range statement, got {entity:?} instead"),
            range: vec![range],
            err_no: ErrNo::syntax__unexpected_token,
        }
    }

    pub fn var_input_ref_assignment(location: SourceLocation) -> Diagnostic {
        Diagnostic::ImprovementSuggestion {
            message:
                "VAR_INPUT {ref} variables are mutable and changes to them will also affect the referenced variable. For increased clarity use VAR_IN_OUT instead."
                    .into(),
            range: vec![location],
        }
    }

    pub fn overflow(message: String, location: SourceLocation) -> Diagnostic {
        Diagnostic::SemanticError { message, range: vec![location], err_no: ErrNo::var__overflow }
    }

    pub fn index_out_of_bounds(range: SourceLocation) -> Diagnostic {
        Diagnostic::SemanticError {
            message: "Index out of bounds.".into(),
            range: vec![range],
            err_no: ErrNo::vla__dimension_idx_out_of_bounds,
        }
    }

    pub fn enum_variant_mismatch(enum_type: &str, range: SourceLocation) -> Diagnostic {
        Diagnostic::SemanticError {
            message: format!("Assigned value is not a variant of {enum_type}"),
            range: vec![range],
            err_no: ErrNo::var__invalid_enum_variant,
        }
    }
}

// CFC related diagnostics
impl Diagnostic {
    pub fn empty_control_statement(range: SourceLocation) -> Diagnostic {
        Diagnostic::SemanticError {
            message: "Control statement has no connection".to_string(),
            range: vec![range],
            err_no: ErrNo::cfc__empty_control_statement,
        }
    }

    pub fn undefined_node(local_id: usize, ref_local_id: usize, range: SourceLocation) -> Diagnostic {
        Diagnostic::SemanticError {
            message: format!("Node {local_id} is referencing a non-existing element with ID {ref_local_id}"),
            range: vec![range],
            err_no: ErrNo::cfc__undefined_node,
        }
    }

    pub fn unexpected_nodes(range: SourceLocation) -> Diagnostic {
        Diagnostic::SemanticError {
            message: "Unexpected relationship between nodes".to_string(),
            range: vec![range],
            err_no: ErrNo::cfc__unexpected_node,
        }
    }

    pub fn unconnected_source(connector_name: &str, range: SourceLocation) -> Diagnostic {
        Diagnostic::SemanticError {
            message: format!("Source '{connector_name}' is not connected."),
            range: vec![range],
            err_no: ErrNo::cfc__unconnected_source,
        }
    }

    pub fn sink_without_associated_source(connector_name: &str, range: SourceLocation) -> Diagnostic {
        Diagnostic::SemanticError {
            message: format!("Expected a corresponding source-connection mark for sink '{connector_name}', but could not find one."),
            range: vec![range],
            err_no: ErrNo::cfc__no_associated_connector,
        }
    }

    pub fn cyclic_connection(message: String, range: SourceLocation) -> Diagnostic {
        Diagnostic::SemanticError {
            message: format!("Sink is connected to itself. Found the following recursion: {message}"),
            range: vec![range],
            err_no: ErrNo::cfc__cyclic_connection,
        }
    }
}

#[cfg(test)]
mod tests {
    use codespan_reporting::files::{Location, SimpleFile};

    use crate::{diagnostician::Severity, reporter::clang::ClangFormatDiagnosticReporter};

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
