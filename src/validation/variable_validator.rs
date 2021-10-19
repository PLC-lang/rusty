use crate::{
    ast::{
        DataType, DataTypeDeclaration, PouType, SourceRange, Variable, VariableBlock,
        VariableBlockType,
    },
    index::{const_expressions::ConstExpression, Index},
    typesystem::{DataTypeInformation, StructSource},
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
        if block.constant
            && !matches!(
                block.variable_block_type,
                VariableBlockType::Global | VariableBlockType::Local
            )
        {
            self.diagnostics
                .push(Diagnostic::invalid_constant_block(block.location.clone()))
        }

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
            println!(
                "validating: {:} : {:#?}",
                v_entry.get_name(),
                context
                    .index
                    .find_effective_type_by_name(v_entry.get_type_name())
            );

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
                Some(ConstExpression::Unresolved { statement, .. }) => {
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

            //check if we declared a constant fb-instance or class-instance
            if v_entry.is_constant()
                && data_type_is_fb_or_class_instance(v_entry.get_type_name(), context.index)
            {
                self.diagnostics.push(Diagnostic::invalid_constant(
                    v_entry.get_name(),
                    variable.location.clone(),
                ));
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

/// returns whether this data_type is a function block, a class or an array/pointer of/to these
fn data_type_is_fb_or_class_instance(type_name: &str, index: &Index) -> bool {
    let data_type = index.find_effective_type_by_name(type_name).map_or_else(
        || index.get_void_type().get_type_information(),
        crate::typesystem::DataType::get_type_information,
    );

    if let DataTypeInformation::Struct {
        source: StructSource::Pou(PouType::FunctionBlock) | StructSource::Pou(PouType::Class),
        ..
    } = data_type
    {
        return true;
    }

    match data_type {
        DataTypeInformation::Struct {
            member_names, name, ..
        } =>
        //see if any member is fb or class intance
        {
            member_names.iter().any(|member_name| {
                index
                    .find_member(name.as_str(), member_name.as_str())
                    .map_or(false, |v| {
                        data_type_is_fb_or_class_instance(v.get_type_name(), index)
                    })
            })
        }
        DataTypeInformation::Array {
            inner_type_name, ..
        } => data_type_is_fb_or_class_instance(inner_type_name.as_str(), index),
        DataTypeInformation::Pointer {
            inner_type_name, ..
        } => data_type_is_fb_or_class_instance(inner_type_name.as_str(), index),
        DataTypeInformation::Alias {
            referenced_type, ..
        } => data_type_is_fb_or_class_instance(referenced_type.as_str(), index),
        _ => false,
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
