use plc_ast::ast::{ArgumentProperty, Pou, PouType, Variable, VariableBlock, VariableBlockType};
use plc_diagnostics::diagnostics::Diagnostic;

use crate::{index::const_expressions::ConstExpression, resolver::AnnotationMap};

use super::{
    array::validate_array_assignment,
    statement::{validate_enum_variant_assignment, visit_statement},
    types::{data_type_is_fb_or_class_instance, visit_data_type_declaration},
    ValidationContext, Validator, Validators,
};

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
                .with_location(block.location.clone()),
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
                    .with_location(variable.location.clone()),
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
            .with_location(variable.location.clone()),
        ),

        (PouType::Program, _) => validator.push_diagnostic(
            Diagnostic::new("Variable Length Arrays are not allowed to be defined inside a Program")
                .with_error_code("E044")
                .with_location(variable.location.clone()),
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
            .with_location(variable.location.clone()),
        ),
    }
}

fn validate_variable<T: AnnotationMap>(
    validator: &mut Validator,
    variable: &Variable,
    context: &ValidationContext<T>,
) {
    if let Some(v_entry) = context
        .qualifier
        .and_then(|qualifier| context.index.find_member(qualifier, variable.name.as_str()))
        .or_else(|| context.index.find_global_variable(variable.name.as_str()))
    {
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
                        .with_location(variable.location.clone()),
                );
            }
            _ => {
                if let Some(rhs) = variable.initializer.as_ref() {
                    validate_enum_variant_assignment(
                        validator,
                        context
                            .index
                            .get_effective_type_or_void_by_name(v_entry.get_type_name())
                            .get_type_information(),
                        context.annotations.get_type_or_void(rhs, context.index).get_type_information(),
                        v_entry.get_qualified_name(),
                        rhs.get_location(),
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
                .with_location(variable.location.clone()),
            );
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

    #[test]
    fn validate_enum_variant_initializer() {
        let diagnostics = parse_and_validate_buffered(
            "VAR_GLOBAL
                x : (red, yellow, green) := 2; // error
            END_VAR

            PROGRAM  main
            VAR
                y : (metallic := 1, matte := 2, neon := 3) := red; // error
            END_VAR
            VAR
                var1 : (x1 := 1, x2 := 2, x3 := 3) := yellow; // error
                var2 : (x5, x6, x7) := neon; // error
                var3 : (a, b, c) := 7; // error
            END_VAR
            END_PROGRAM",
        );
        assert_snapshot!(diagnostics);
    }
}
