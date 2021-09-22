use crate::{
    ast::{DataType, DataTypeDeclaration, SourceRange, Variable, VariableBlock},
    index::const_expressions::ConstExpression,
    Diagnostic,
};

use super::ValidationContext;

/// validates variables & datatypes

pub struct VariableValidator {
    pub diagnostics: Vec<Diagnostic>,
}

impl VariableValidator {
    pub fn new() -> VariableValidator {
        VariableValidator {
            diagnostics: Vec::new(),
        }
    }

    pub fn validate_variable_block(&mut self, block: &VariableBlock, context: &ValidationContext) {
        for variable in &block.variables {
            self.validate_variable(variable, context);
        }
    }

    pub fn validate_variable(&mut self, variable: &Variable, context: &ValidationContext) {
        if let Some(v_entry) = context
            .qualifier
            .and_then(|qualifier| context.index.find_member(qualifier, variable.name.as_str()))
            .or_else(|| context.index.find_global_variable(variable.name.as_str()))
        {
            match v_entry.initial_value.and_then(|initial_id| {
                context
                    .index
                    .get_const_expressions()
                    .find_const_expression(&initial_id)
            }) {
                Some(ConstExpression::Unresolvable { reason, statement }) => {
                    self.diagnostics.push(Diagnostic::unresolved_constant(
                        variable.name.as_str(),
                        Some(reason),
                        statement.get_location(),
                    ));
                }
                Some(ConstExpression::Unresolved(statement)) => {
                    self.diagnostics.push(Diagnostic::unresolved_constant(
                        variable.name.as_str(),
                        None,
                        statement.get_location(),
                    ));
                }
                None if v_entry.is_constant() => {
                    self.diagnostics.push(Diagnostic::unresolved_constant(
                        variable.name.as_str(),
                        None,
                        variable.location.clone(),
                    ));
                }
                _ => {}
            }
        }
    }

    pub fn validate_data_type_declaration(&self, _declaration: &DataTypeDeclaration) {}

    pub fn validate_data_type(&mut self, declaration: &DataType, location: &SourceRange) {
        match declaration {
            DataType::StructType { variables, .. } => {
                if variables.is_empty() {
                    self.diagnostics
                        .push(Diagnostic::empty_variable_block(location.clone()));
                }
            }
            DataType::EnumType { elements, .. } => {
                if elements.is_empty() {
                    self.diagnostics
                        .push(Diagnostic::empty_variable_block(location.clone()));
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod variable_validator_tests {
    use crate::{validation::tests::parse_and_validate, Diagnostic};

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
                Diagnostic::empty_variable_block((14..44).into()),
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
                Diagnostic::empty_variable_block((14..27).into()),
                Diagnostic::empty_variable_block((112..114).into())
            ]
        );
    }
}
