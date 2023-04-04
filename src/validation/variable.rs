use crate::{
    ast::{ArgumentProperty, AstStatement, Pou, PouType, Variable, VariableBlock, VariableBlockType},
    index::const_expressions::ConstExpression,
    Diagnostic,
};

use super::{
    types::{data_type_is_fb_or_class_instance, visit_data_type_declaration},
    validate_for_array_assignment, ValidationContext, Validator, Validators,
};

pub fn visit_variable_block(
    validator: &mut Validator,
    pou: Option<&Pou>,
    block: &VariableBlock,
    context: &ValidationContext,
) {
    validate_variable_block(validator, block);

    for variable in &block.variables {
        visit_variable(validator, variable, context);

        // TODO: ugly af
        if let Some(referenced_type) = variable.data_type_declaration.get_referenced_type() {
            if context.index.get_type_information_or_void(&referenced_type).is_vla() && pou.is_some() {
                validate_vla(validator, pou.as_ref().unwrap(), block, variable);
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

pub fn visit_variable(validator: &mut Validator, variable: &Variable, context: &ValidationContext) {
    validate_variable(validator, variable, context);

    visit_data_type_declaration(validator, &variable.data_type_declaration, context);
}

fn validate_vla(validator: &mut Validator, pou: &Pou, block: &VariableBlock, variable: &Variable) {
    match (&pou.pou_type, block.variable_block_type) {
        (PouType::Function, VariableBlockType::Input(ArgumentProperty::ByVal)) => {
            validator.push_diagnostic(Diagnostic::vla_input_by_val(variable.location.clone()))
        }

        (PouType::Program, _) => validator.push_diagnostic(Diagnostic::invalid_vla_container(
            format!("Variable Length Arrays are not allowed to be defined inside a Program",),
            variable.location.clone(),
        )),

        (
            PouType::Function | PouType::Method { .. },
            VariableBlockType::Input(_) | VariableBlockType::Output | VariableBlockType::InOut,
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

fn validate_variable(validator: &mut Validator, variable: &Variable, context: &ValidationContext) {
    if let Some(v_entry) = context
        .qualifier
        .and_then(|qualifier| context.index.find_member(qualifier, variable.name.as_str()))
        .or_else(|| context.index.find_global_variable(variable.name.as_str()))
    {
        if let Some(AstStatement::ExpressionList { expressions, .. }) = &variable.initializer {
            validate_for_array_assignment(validator, expressions, context);
        }

        match v_entry
            .initial_value
            .and_then(|initial_id| context.index.get_const_expressions().find_const_expression(&initial_id))
        {
            Some(ConstExpression::Unresolvable { reason, statement }) => {
                validator.push_diagnostic(Diagnostic::unresolved_constant(
                    variable.name.as_str(),
                    Some(reason),
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
            _ => {}
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
    use crate::test_utils::tests::parse_and_validate;
    use crate::Diagnostic;

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
}
