use plc_ast::{
    ast::{
        AstNode, AstStatement, AutoDerefType, DataType, DataTypeDeclaration, PouType, RangeStatement,
        UserTypeDeclaration,
    },
    literals::AstLiteral,
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;

use crate::{
    index::Index,
    resolver::AnnotationMap,
    typesystem::{self, DataTypeInformation, Dimension, StructSource},
};

use super::{
    array::validate_array_assignment, variable::visit_variable, ValidationContext, Validator, Validators,
};

pub fn visit_data_type_declaration<T: AnnotationMap>(
    validator: &mut Validator,
    declaration: &DataTypeDeclaration,
    context: &ValidationContext<T>,
) {
    if declaration.get_location().is_internal() {
        return;
    }

    match declaration {
        DataTypeDeclaration::Reference { referenced_type, location } => {
            if context.index.find_effective_type_by_name(referenced_type).is_none() {
                validator.push_diagnostic(Diagnostic::unknown_type(referenced_type, location));
            };
        }
        DataTypeDeclaration::Definition { data_type, location, .. } => {
            visit_data_type(validator, data_type, location, context)
        }
        DataTypeDeclaration::Aggregate { .. } => {}
    }
}

pub fn visit_data_type<T: AnnotationMap>(
    validator: &mut Validator,
    data_type: &DataType,
    location: &SourceLocation,
    context: &ValidationContext<T>,
) {
    validate_data_type(validator, data_type, location);

    let context = &context.with_optional_qualifier(data_type.get_name());
    match data_type {
        DataType::StructType { variables, .. } => {
            variables.iter().for_each(|v| visit_variable(validator, v, context))
        }
        DataType::ArrayType { referenced_type, bounds, is_variable_length: false, .. } => {
            visit_data_type_declaration(validator, referenced_type, context);
            validate_array_bounds(validator, bounds, context);

            // Arrays of automatically dereferenced `REFERENCE TO` pointers are not allowed.
            let declaration = referenced_type.as_ref();
            if !declaration.get_location().is_internal() {
                if let DataTypeDeclaration::Reference { referenced_type, location } = declaration {
                    if let Some(data_type) = context.index.find_effective_type_by_name(referenced_type) {
                        if let DataTypeInformation::Pointer {
                            auto_deref: Some(AutoDerefType::Reference),
                            ..
                        } = data_type.get_type_information()
                        {
                            validator.push_diagnostic(
                                Diagnostic::new("Invalid reference to declaration. Arrays of automatically dereferenced references are not allowed.")
                                .with_error_code("E099")
                                .with_location(location),
                            );
                        }
                    };
                }
            }
        }
        DataType::ArrayType { referenced_type, .. } => {
            visit_data_type_declaration(validator, referenced_type, context);

            // Arrays of automatically dereferenced `REFERENCE TO` pointers are not allowed.
            let declaration = referenced_type.as_ref();
            if !declaration.get_location().is_internal() {
                if let DataTypeDeclaration::Reference { referenced_type, location } = declaration {
                    if let Some(data_type) = context.index.find_effective_type_by_name(referenced_type) {
                        if let DataTypeInformation::Pointer {
                            auto_deref: Some(AutoDerefType::Reference),
                            ..
                        } = data_type.get_type_information()
                        {
                            validator.push_diagnostic(
                                Diagnostic::new("Invalid reference to declaration. Arrays of automatically dereferenced references are not allowed.")
                                .with_error_code("E099")
                                .with_location(location),
                            );
                        }
                    };
                }
            }
        }
        DataType::VarArgs { referenced_type: Some(referenced_type), .. } => {
            visit_data_type_declaration(validator, referenced_type.as_ref(), context);
        }
        DataType::PointerType { referenced_type, .. } => {
            visit_data_type_declaration(validator, referenced_type.as_ref(), context);

            // References to automatically dereferenced `REFERENCE TO` pointers are not allowed.
            let declaration = referenced_type.as_ref();
            if !declaration.get_location().is_internal() {
                if let DataTypeDeclaration::Reference { referenced_type, location } = declaration {
                    if let Some(data_type) = context.index.find_effective_type_by_name(referenced_type) {
                        if let DataTypeInformation::Pointer {
                            auto_deref: Some(AutoDerefType::Reference),
                            ..
                        } = data_type.get_type_information()
                        {
                            validator.push_diagnostic(
                                Diagnostic::new("Invalid reference to declaration. References to automatically dereferenced references are not allowed.")
                                .with_error_code("E099")
                                .with_location(location),
                            );
                        }
                    };
                }
            }
        }
        DataType::EnumType { numeric_type, .. } => {
            if let Some(resolved_type) = context.index.find_effective_type_by_name(numeric_type) {
                let type_info = resolved_type.get_type_information();
                if !type_info.is_int() || type_info.is_date_or_time_type() {
                    validator.push_diagnostic(
                        Diagnostic::new(format!(
                            "Invalid type '{}' for enum. Only integer types are allowed",
                            numeric_type
                        ))
                        .with_error_code("E122")
                        .with_location(location),
                    );
                }
            } else {
                validator.push_diagnostic(Diagnostic::unknown_type(numeric_type, location));
            }
        }
        _ => {}
    }
}

fn validate_data_type(validator: &mut Validator, data_type: &DataType, location: &SourceLocation) {
    if location.is_internal() {
        return;
    }

    match data_type {
        DataType::StructType { variables, .. } if variables.is_empty() => {
            validator.push_diagnostic(
                Diagnostic::new("Variable block is empty").with_error_code("E028").with_location(location),
            );
        }
        DataType::EnumType {
            elements: AstNode { stmt: AstStatement::ExpressionList(expressions), .. },
            ..
        } if expressions.is_empty() => {
            validator.push_diagnostic(
                Diagnostic::new("Variable block is empty").with_error_code("E028").with_location(location),
            );
        }
        DataType::VarArgs { referenced_type: None, sized: true } => validator.push_diagnostic(
            Diagnostic::new("Missing datatype: Sized Variadics require a known datatype.")
                .with_error_code("E038")
                .with_location(location),
        ),
        _ => {}
    }
}

/// Validate that each range bound of a statically-sized array has an integer type.
/// Rejects non-integer bounds (REAL, STRING, TIME, ...) and also BOOL — BOOL is
/// represented as an integer internally, but its literal form does not lower to an
/// integer constant expression and would otherwise crash codegen.
fn validate_array_bounds<T: AnnotationMap>(
    validator: &mut Validator,
    bounds: &AstNode,
    context: &ValidationContext<T>,
) {
    for bound in bounds.get_as_list() {
        let AstStatement::RangeStatement(RangeStatement { start, end }) = bound.get_stmt() else {
            continue;
        };
        check_array_bound_type(validator, start, context);
        check_array_bound_type(validator, end, context);
    }
}

fn check_array_bound_type<T: AnnotationMap>(
    validator: &mut Validator,
    expr: &AstNode,
    context: &ValidationContext<T>,
) {
    let type_info = context.annotations.get_type_or_void(expr, context.index).get_type_information();

    let accepted = if type_info.is_void() {
        // Resolver didn't annotate — fall back to literal inspection. Accept integer
        // literals; reject any other literal form.
        matches!(expr.get_stmt_peeled(), AstStatement::Literal(AstLiteral::Integer(_)))
            || !matches!(expr.get_stmt_peeled(), AstStatement::Literal(_))
    } else {
        // `is_int()` includes integer-backed semantic types (notably BOOL and date/time),
        // so those must be excluded explicitly.
        type_info.is_int() && !type_info.is_bool() && !type_info.is_date_or_time_type()
    };

    if accepted {
        return;
    }

    let type_name = type_info.get_name();
    validator.push_diagnostic(
        Diagnostic::new(format!(
            "Invalid type '{type_name}' for array bounds. Only integer types are allowed"
        ))
        .with_error_code("E008")
        .with_location(expr.get_location()),
    );
}

pub fn visit_user_type_declaration<T: AnnotationMap>(
    validator: &mut Validator,
    user_type: &UserTypeDeclaration,
    context: &ValidationContext<T>,
) {
    visit_data_type(validator, &user_type.data_type, &user_type.location, context);
    validate_array_assignment(validator, context, user_type);
}

pub fn data_type_is_fb_or_class_instance(type_name: &str, index: &Index) -> bool {
    let data_type_info = index.find_effective_type_by_name(type_name).map_or_else(
        || index.get_void_type().get_type_information(),
        crate::typesystem::DataType::get_type_information,
    );

    if let DataTypeInformation::Struct {
        source: StructSource::Pou(PouType::FunctionBlock) | StructSource::Pou(PouType::Class),
        ..
    } = data_type_info
    {
        return true;
    }

    match data_type_info {
        DataTypeInformation::Struct { members, .. } =>
        //see if any member is fb or class intance
        {
            members.iter().any(|member| data_type_is_fb_or_class_instance(member.get_type_name(), index))
        }
        DataTypeInformation::Array { inner_type_name, .. } => {
            data_type_is_fb_or_class_instance(inner_type_name.as_str(), index)
        }
        DataTypeInformation::Pointer { inner_type_name, .. } => {
            data_type_is_fb_or_class_instance(inner_type_name.as_str(), index)
        }
        DataTypeInformation::Alias { referenced_type, .. } => {
            data_type_is_fb_or_class_instance(referenced_type.as_str(), index)
        }
        _ => false,
    }
}

pub fn are_equal_types(index: &Index, left: &typesystem::DataType, right: &typesystem::DataType) -> bool {
    let left_type_info = left.get_type_information();
    let right_type_info = right.get_type_information();

    if left_type_info == right_type_info {
        return true;
    }

    match (left_type_info, right_type_info) {
        (
            DataTypeInformation::Array {
                inner_type_name: left_inner_type_name,
                dimensions: left_dimensions,
                ..
            },
            DataTypeInformation::Array {
                inner_type_name: right_inner_type_name,
                dimensions: right_dimensions,
                ..
            },
        ) => {
            are_equal_array_dimensions(index, left_dimensions, right_dimensions)
                && are_equal_types(
                    index,
                    index.get_effective_type_or_void_by_name(left_inner_type_name),
                    index.get_effective_type_or_void_by_name(right_inner_type_name),
                )
        }
        (
            DataTypeInformation::Struct { name: left_name, .. },
            DataTypeInformation::Struct { name: right_name, .. },
        )
        | (
            DataTypeInformation::Enum { name: left_name, .. },
            DataTypeInformation::Enum { name: right_name, .. },
        ) => {
            // Given we have a global namespace only, checking by names is sufficient for equality check
            // for structs and enums. The "Ambiguous datatype" validation does the heavy lifting here.
            left_name.eq_ignore_ascii_case(right_name)
        }
        (
            DataTypeInformation::Alias { referenced_type: left_reference_type, .. },
            DataTypeInformation::Alias { referenced_type: right_reference_type, .. },
        ) => {
            let left = index.find_effective_type_by_name(left_reference_type);
            let right = index.find_effective_type_by_name(right_reference_type);

            left == right
        }

        _ => false,
    }
}

fn are_equal_array_dimensions(index: &Index, left: &[Dimension], right: &[Dimension]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    for (left_dimension, right_dimension) in left.iter().zip(right) {
        let Some(left_range) = left_dimension.get_range(index).ok() else {
            return false;
        };

        let Some(right_range) = right_dimension.get_range(index).ok() else {
            return false;
        };

        if left_range != right_range {
            return false;
        }
    }

    true
}
