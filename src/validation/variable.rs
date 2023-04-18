use crate::{
    ast::{AstStatement, DataTypeDeclaration, Variable, VariableBlock, VariableBlockType},
    index::const_expressions::ConstExpression,
    resolver::const_evaluator,
    typesystem::DataTypeInformation,
    Diagnostic,
};

use super::{
    types::{data_type_is_fb_or_class_instance, visit_data_type_declaration},
    validate_for_array_assignment, ValidationContext, Validator, Validators,
};

pub fn visit_variable_block(validator: &mut Validator, block: &VariableBlock, context: &ValidationContext) {
    validate_variable_block(validator, block);

    for variable in &block.variables {
        visit_variable(validator, variable, context);
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
    temp(validator, variable, context);

    visit_data_type_declaration(validator, &variable.data_type_declaration, context);
}

fn temp(validator: &mut Validator, variable: &Variable, context: &ValidationContext) {
    let Some(initializer) = &variable.initializer else { return };
    let Ok(initializer) = const_evaluator::evaluate(&initializer, None, context.index) else { return };

    let DataTypeDeclaration::DataTypeReference { referenced_type, .. } = &variable.data_type_declaration else {
        return
    };

    let Some(dt) = context.index.find_effective_type_by_name(&referenced_type) else { return };
    let err = match dt.get_type_information() {
        DataTypeInformation::Integer { signed, size, .. } => match (size, signed, initializer) {
            (8, true, Some(AstStatement::LiteralInteger { value, .. })) => i8::try_from(value).is_err(),
            (16, true, Some(AstStatement::LiteralInteger { value, .. })) => i16::try_from(value).is_err(),
            (32, true, Some(AstStatement::LiteralInteger { value, .. })) => i32::try_from(value).is_err(),
            (64, true, Some(AstStatement::LiteralInteger { value, .. })) => i64::try_from(value).is_err(),

            (8, false, Some(AstStatement::LiteralInteger { value, .. })) => i8::try_from(value).is_err(),
            (16, false, Some(AstStatement::LiteralInteger { value, .. })) => i16::try_from(value).is_err(),
            (32, false, Some(AstStatement::LiteralInteger { value, .. })) => i32::try_from(value).is_err(),
            (64, false, Some(AstStatement::LiteralInteger { value, .. })) => i64::try_from(value).is_err(),

            _ => return,
        },

        // DataTypeInformation::Float { size, .. } => match (size, initializer) {
        //     (32, Some(AstStatement::LiteralReal { value, .. })) => value.parse::<f32>().is_err(),
        //     (64, Some(AstStatement::LiteralReal { value, .. })) => value.parse::<f64>().is_err(),
        // },
        _ => return,
    };
    if err {
        validator.push_diagnostic(Diagnostic::SemanticError {
            message: format!("Literal out of range for {}", dt.get_name()),
            range: vec![variable.initializer.as_ref().unwrap().get_location()],
            err_no: crate::diagnostics::ErrNo::type__incompatible_literal_cast,
        });
    }
}

// TODO: document what the fuck this does
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
