use crate::{
    ast::{ArgumentProperty, AstStatement, Pou, PouType, Variable, VariableBlock, VariableBlockType},
    index::const_expressions::ConstExpression,
    resolver::AnnotationMap,
    Diagnostic,
};

use super::{
    array::__validate_array_initialization,
    statement::validate_enum_variant_assignment,
    types::{data_type_is_fb_or_class_instance, visit_data_type_declaration},
    validate_array_assignment, ValidationContext, Validator, Validators,
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
        validator.push_diagnostic(Diagnostic::invalid_constant_block(block.location.clone()))
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
            validator.push_diagnostic(Diagnostic::invalid_vla_container(
                "VLAs can not be defined as global variables".to_string(),
                variable.location.clone())
            )
        }

        return;
    };

    match (&pou.pou_type, block.variable_block_type) {
        (PouType::Function, VariableBlockType::Input(ArgumentProperty::ByVal)) => {
            validator.push_diagnostic(Diagnostic::vla_by_val_warning(variable.location.clone()))
        }

        (PouType::Program, _) => validator.push_diagnostic(Diagnostic::invalid_vla_container(
            "Variable Length Arrays are not allowed to be defined inside a Program".to_string(),
            variable.location.clone(),
        )),

        (
            PouType::Function | PouType::Method { .. },
            VariableBlockType::Input(ArgumentProperty::ByRef)
            | VariableBlockType::Output
            | VariableBlockType::InOut,
        )
        | (PouType::FunctionBlock, VariableBlockType::InOut) => (),

        _ => validator.push_diagnostic(Diagnostic::invalid_vla_container(
            format!(
                "Variable Length Arrays are not allowed to be defined as {} variables inside a {}",
                block.variable_block_type, pou.pou_type
            ),
            variable.location.clone(),
        )),
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
        __validate_array_initialization(validator, variable, context);

        match v_entry
            .initial_value
            .and_then(|initial_id| context.index.get_const_expressions().find_const_expression(&initial_id))
        {
            Some(ConstExpression::Unresolvable { reason, statement }) if reason.is_misc() => {
                validator.push_diagnostic(Diagnostic::unresolved_constant(
                    variable.name.as_str(),
                    Some(reason.get_reason()),
                    statement.get_location(),
                ));
            }
            Some(ConstExpression::Unresolved { statement, .. }) => {
                validator.push_diagnostic(Diagnostic::unresolved_constant(
                    variable.name.as_str(),
                    None,
                    statement.get_location(),
                ));
            }
            None if v_entry.is_constant() => {
                validator.push_diagnostic(Diagnostic::unresolved_constant(
                    variable.name.as_str(),
                    None,
                    variable.location.clone(),
                ));
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
            validator
                .push_diagnostic(Diagnostic::invalid_constant(v_entry.get_name(), variable.location.clone()));
        }
    }
}

#[cfg(test)]
mod variable_validator_tests {
    use crate::{assert_validation_snapshot, test_utils::tests::parse_and_validate, Diagnostic};

    #[test]
    fn validate_empty_struct_declaration() {
        let diagnostics = parse_and_validate(
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

        assert_eq!(
            diagnostics,
            vec![
                Diagnostic::empty_variable_block((14..24).into()),
                Diagnostic::empty_variable_block((131..164).into())
            ]
        );
    }

    #[test]
    fn validate_empty_enum_declaration() {
        let diagnostics = parse_and_validate(
            "
        TYPE my_enum : (); END_TYPE
            
        PROGRAM prg
            VAR
                my_enum : ();
            END_VAR
        END_PROGRAM
        ",
        );

        assert_eq!(
            diagnostics,
            vec![
                Diagnostic::empty_variable_block((14..21).into()),
                Diagnostic::empty_variable_block((112..114).into())
            ]
        );
    }

    #[test]
    fn validate_enum_variant_initializer() {
        let diagnostics = parse_and_validate(
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

        assert_validation_snapshot!(diagnostics);
    }
}
