use plc_ast::ast::{ArgumentProperty, Pou, PouType, Variable, VariableBlock, VariableBlockType};
use plc_diagnostics::diagnostics::Diagnostic;

use super::{
    array::validate_array_assignment,
    statement::{validate_enum_variant_assignment, visit_statement},
    types::{data_type_is_fb_or_class_instance, visit_data_type_declaration},
    ValidationContext, Validator, Validators,
};
use crate::index::VariableIndexEntry;
use crate::typesystem::DataTypeInformation;
use crate::{index::const_expressions::ConstExpression, resolver::AnnotationMap};

pub fn visit_variable_block<T: AnnotationMap>(
    validator: &mut Validator,
    pou: Option<&Pou>,
    block: &VariableBlock,
    context: &ValidationContext<T>,
) {
    validate_variable_block(validator, block);

    for variable in &block.variables {
        visit_variable(validator, variable, context);

        if let Some(referenced_type) = variable.data_type_declaration.get_referenced_type() {
            if context.index.get_type_information_or_void(&referenced_type).is_vla() {
                validate_vla(validator, pou, block, variable);
            }
        }
    }
}

fn validate_variable_block(validator: &mut Validator, block: &VariableBlock) {
    if block.constant
        && !matches!(block.variable_block_type, VariableBlockType::Global | VariableBlockType::Local)
    {
        validator.push_diagnostic(
            Diagnostic::new("This variable block does not support the CONSTANT modifier")
                .with_error_code("E034")
                .with_location(&block.location),
        )
    }
}

pub fn visit_variable<T: AnnotationMap>(
    validator: &mut Validator,
    variable: &Variable,
    context: &ValidationContext<T>,
) {
    validate_variable(validator, variable, context);
    visit_data_type_declaration(validator, &variable.data_type_declaration, context);
}

/// Validates Variable Length Arrays as specified in the IEC61131-3, i.e. VLAs are only allowed to be defined
/// inside the following Variable Block and POU combinations
/// - Input, Output and InOut within a Function or Method or
/// - InOut within Function-Block
fn validate_vla(validator: &mut Validator, pou: Option<&Pou>, block: &VariableBlock, variable: &Variable) {
    let Some(pou) = pou else {
        if matches!(block.variable_block_type, VariableBlockType::Global) {
            validator.push_diagnostic(
                Diagnostic::new("VLAs can not be defined as global variables")
                    .with_error_code("E044")
                    .with_location(&variable.location),
            )
        }

        return;
    };

    match (&pou.pou_type, block.variable_block_type) {
        (PouType::Function, VariableBlockType::Input(ArgumentProperty::ByVal)) => validator.push_diagnostic(
            Diagnostic::new(
                "Variable Length Arrays are always by-ref, even when declared in a by-value block",
            )
            .with_error_code("E047")
            .with_location(&variable.location),
        ),

        (PouType::Program, _) => validator.push_diagnostic(
            Diagnostic::new("Variable Length Arrays are not allowed to be defined inside a Program")
                .with_error_code("E044")
                .with_location(&variable.location),
        ),

        (
            PouType::Function | PouType::Method { .. },
            VariableBlockType::Input(ArgumentProperty::ByRef)
            | VariableBlockType::Output
            | VariableBlockType::InOut,
        )
        | (PouType::FunctionBlock, VariableBlockType::InOut) => (),

        _ => validator.push_diagnostic(
            Diagnostic::new(format!(
                "Variable Length Arrays are not allowed to be defined as {} variables inside a {}",
                block.variable_block_type, pou.pou_type
            ))
            .with_error_code("E044")
            .with_location(&variable.location),
        ),
    }
}

fn validate_array_ranges<T>(validator: &mut Validator, variable: &Variable, context: &ValidationContext<T>)
where
    T: AnnotationMap,
{
    let ty_name = variable.data_type_declaration.get_name().unwrap_or_default();
    let ty_info = context.index.get_effective_type_or_void_by_name(ty_name).get_type_information();

    if ty_info.is_array() {
        let mut types = vec![];
        ty_info.get_inner_array_types(&mut types, context.index);

        for ty in types {
            let DataTypeInformation::Array { dimensions, .. } = ty else {
                unreachable!("`get_inner_types()` only operates on Arrays");
            };

            for dimension in dimensions.iter().filter_map(|dim| dim.get_range(context.index).ok()) {
                let std::ops::Range { start, end } = dimension;

                if start > end {
                    validator.push_diagnostic(
                        Diagnostic::new(format!(
                            "Invalid range `{start}..{end}`, did you mean `{end}..{start}`?"
                        ))
                        .with_location(variable.location.clone())
                        .with_error_code("E097"),
                    );
                }
            }
        }
    }
}

fn validate_variable<T: AnnotationMap>(
    validator: &mut Validator,
    variable: &Variable,
    context: &ValidationContext<T>,
) {
    validate_array_ranges(validator, variable, context);

    if let Some(v_entry) = context.index.find_variable(context.qualifier, &[&variable.name]) {
        validate_reference_to_declaration(validator, context, variable, v_entry);

        if let Some(initializer) = &variable.initializer {
            // Assume `foo : ARRAY[1..5] OF DINT := [...]`, here the first function call validates the
            // assignment as a whole whereas the second function call (`visit_statement`) validates the
            // initializer in case it has further sub-assignments.
            validate_array_assignment(validator, context, variable);
            visit_statement(validator, initializer, context);
        }

        match v_entry
            .initial_value
            .and_then(|initial_id| context.index.get_const_expressions().find_const_expression(&initial_id))
        {
            Some(ConstExpression::Unresolvable { reason, statement }) if reason.is_misc() => {
                validator.push_diagnostic(
                    Diagnostic::new(format!(
                        "Unresolved constant `{}` variable: {}",
                        variable.name.as_str(),
                        reason.get_reason()
                    ))
                    .with_error_code("E033")
                    .with_location(statement.get_location()),
                );
            }
            Some(ConstExpression::Unresolved { statement, .. }) => {
                validator.push_diagnostic(
                    Diagnostic::new(format!("Unresolved constant `{}` variable", variable.name.as_str(),))
                        .with_error_code("E033")
                        .with_location(statement.get_location()),
                );
            }
            None if v_entry.is_constant() => {
                validator.push_diagnostic(
                    Diagnostic::new(format!("Unresolved constant `{}` variable", variable.name.as_str(),))
                        .with_error_code("E033")
                        .with_location(&variable.location),
                );
            }
            _ => {
                if let Some(rhs) = variable.initializer.as_ref() {
                    validate_enum_variant_assignment(
                        context,
                        validator,
                        v_entry.get_qualified_name(),
                        context.index.get_effective_type_or_void_by_name(v_entry.get_type_name()),
                        rhs,
                    )
                }
            }
        }

        // check if we declared a constant fb-instance or class-instance
        if v_entry.is_constant() && data_type_is_fb_or_class_instance(v_entry.get_type_name(), context.index)
        {
            validator.push_diagnostic(
                Diagnostic::new(format!(
                    "Invalid constant {} - Functionblock- and Class-instances cannot be delcared constant",
                    v_entry.get_name()
                ))
                .with_error_code("E035")
                .with_location(&variable.location),
            );
        }
    }
}

/// Returns a diagnostic if a `REFERENCE TO` variable is incorrectly declared (or initialized).
fn validate_reference_to_declaration<T: AnnotationMap>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    variable: &Variable,
    variable_entry: &VariableIndexEntry,
) {
    if let Some(variable_type) = context.index.find_effective_type_by_name(variable_entry.get_type_name()) {
        if variable_type.get_type_information().is_reference_to() {
            let DataTypeInformation::Pointer { inner_type_name, .. } = variable_type.get_type_information()
            else {
                unreachable!("`REFERENCE TO` is defined as a pointer, hence this must exist")
            };

            // Assert that no initializers are present in the `REFERENCE TO` declaration
            if let Some(ref initializer) = variable.initializer {
                if variable_type.get_type_information().is_reference_to() {
                    validator.push_diagnostic(
                        Diagnostic::new("REFERENCE TO variables can not be initialized in their declaration")
                            .with_location(&initializer.location)
                            .with_error_code("E099"),
                    );
                }
            }

            // Assert that the referenced type is no variable reference
            if context.index.find_member(context.qualifier.unwrap_or_default(), &inner_type_name).is_some() {
                validator.push_diagnostic(
                    Diagnostic::new("Invalid type, reference")
                        .with_location(&variable_type.location)
                        .with_error_code("E099"),
                );
            }

            // Lastly assert that the referenced type is no array, pointer or bit
            let inner_type = context.index.find_effective_type_by_name(&inner_type_name);
            if inner_type.is_some_and(|ty| ty.is_array() || ty.is_pointer() || ty.is_bit()) {
                validator.push_diagnostic(
                    Diagnostic::new("Invalid type: array, pointer or bit ")
                        .with_location(&variable.location)
                        .with_error_code("E099"),
                );
            }
        }
    }
}

#[cfg(test)]
mod variable_validator_tests {
    use insta::assert_snapshot;

    use crate::test_utils::tests::parse_and_validate_buffered;

    #[test]
    fn validate_empty_struct_declaration() {
        let diagnostics = parse_and_validate_buffered(
            "
        TYPE the_struct : STRUCT END_STRUCT END_TYPE

        PROGRAM prg
            VAR
                my_struct : STRUCT
                END_STRUCT
            END_VAR
        END_PROGRAM
        ",
        );
        assert_snapshot!(diagnostics);
    }

    #[test]
    fn validate_empty_enum_declaration() {
        let diagnostics = parse_and_validate_buffered(
            "
        TYPE my_enum : (); END_TYPE

        PROGRAM prg
            VAR
                my_enum : ();
            END_VAR
        END_PROGRAM
        ",
        );
        assert_snapshot!(diagnostics);
    }
}
