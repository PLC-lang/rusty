use plc_ast::ast::{ArgumentProperty, Pou, PouType, Variable, VariableBlock, VariableBlockType};
use plc_diagnostics::diagnostics::Diagnostic;

use super::{
    array::validate_array_assignment,
    statement::{validate_enum_variant_assignment, validate_pointer_assignment, visit_statement},
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
                    "Invalid constant {}, FUNCTION_BLOCK- and CLASS-instances cannot be declared constant",
                    v_entry.get_name()
                ))
                .with_error_code("E035")
                .with_location(&variable.location),
            );
        }
    }
}

/// Returns a diagnostic if a `REFERENCE TO` variable is incorrectly declared.
fn validate_reference_to_declaration<T: AnnotationMap>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    variable: &Variable,
    variable_entry: &VariableIndexEntry,
) {
    let Some(variable_ty) = context.index.find_effective_type_by_name(variable_entry.get_type_name()) else {
        return;
    };

    if !variable_ty.get_type_information().is_reference_to() && !variable_ty.get_type_information().is_alias()
    {
        return;
    }

    let Some(inner_ty_name) = variable_ty.get_type_information().get_inner_pointer_type_name() else {
        unreachable!("`REFERENCE TO` is defined as a pointer, hence this must exist")
    };

    // Assert that the referenced type is no variable reference
    let qualifier = context.qualifier.unwrap_or_default();
    let inner_ty_is_local_var = context.index.find_member(qualifier, inner_ty_name).is_some();
    let inner_ty_is_global_var = context.index.find_global_variable(inner_ty_name).is_some();

    if inner_ty_is_local_var || inner_ty_is_global_var {
        validator.push_diagnostic(
            Diagnostic::new("REFERENCE TO variables can not reference other variables")
                .with_location(&variable_ty.location)
                .with_error_code("E099"),
        );
    }

    if let Some(ref initializer) = variable.initializer {
        let type_lhs = context.index.find_type(inner_ty_name).unwrap();
        let type_rhs = context.annotations.get_type(initializer, context.index).unwrap();

        validate_pointer_assignment(context, validator, type_lhs, type_rhs, &initializer.location);
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
