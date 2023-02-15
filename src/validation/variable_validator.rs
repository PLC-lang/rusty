use crate::{
    ast::{
        AstStatement, DataType, DataTypeDeclaration, PouType, SourceRange, Variable, VariableBlock,
        VariableBlockType,
    },
    index::{const_expressions::ConstExpression, Index},
    typesystem::{DataTypeInformation, StructSource},
    Diagnostic,
};

use super::{validate_for_array_assignment, ValidationContext, Validators};

/// validates variables & datatypes
#[derive(Default)]
pub struct VariableValidator {
    diagnostics: Vec<Diagnostic>,
}

impl Validators for VariableValidator {
    fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn take_diagnostics(&mut self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.diagnostics)
    }
}

impl VariableValidator {
    pub fn new() -> VariableValidator {
        VariableValidator { diagnostics: Vec::new() }
    }

    pub fn visit_variable_block(&mut self, context: &ValidationContext, block: &VariableBlock) {
        self.validate_variable_block(block);

        for variable in &block.variables {
            self.visit_variable(context, variable);
        }
    }

    pub fn validate_variable_block(&mut self, block: &VariableBlock) {
        if block.constant
            && !matches!(block.variable_block_type, VariableBlockType::Global | VariableBlockType::Local)
        {
            self.push_diagnostic(Diagnostic::invalid_constant_block(block.location.clone()))
        }
    }

    pub fn visit_variable(&mut self, context: &ValidationContext, variable: &Variable) {
        self.validate_variable(variable, context);

        self.visit_data_type_declaration(context, &variable.data_type);
    }

    pub fn validate_variable(&mut self, variable: &Variable, context: &ValidationContext) {
        if let Some(v_entry) = context
            .qualifier
            .and_then(|qualifier| context.index.find_member(qualifier, variable.name.as_str()))
            .or_else(|| context.index.find_global_variable(variable.name.as_str()))
        {
            if let Some(AstStatement::ExpressionList { expressions, .. }) = &variable.initializer {
                validate_for_array_assignment(self, expressions, context);
            }

            match v_entry.initial_value.and_then(|initial_id| {
                context.index.get_const_expressions().find_const_expression(&initial_id)
            }) {
                Some(ConstExpression::Unresolvable { reason, statement }) => {
                    self.push_diagnostic(Diagnostic::unresolved_constant(
                        variable.name.as_str(),
                        Some(reason),
                        statement.get_location(),
                    ));
                }
                Some(ConstExpression::Unresolved { statement, .. }) => {
                    self.push_diagnostic(Diagnostic::unresolved_constant(
                        variable.name.as_str(),
                        None,
                        statement.get_location(),
                    ));
                }
                None if v_entry.is_constant() => {
                    self.push_diagnostic(Diagnostic::unresolved_constant(
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
                self.diagnostics
                    .push(Diagnostic::invalid_constant(v_entry.get_name(), variable.location.clone()));
            }
        }
    }

    pub fn visit_data_type_declaration(
        &mut self,
        context: &ValidationContext,
        declaration: &DataTypeDeclaration,
    ) {
        if let DataTypeDeclaration::DataTypeDefinition { data_type, location, .. } = declaration {
            self.visit_data_type(context, data_type, location);
        }
    }

    pub fn visit_data_type(
        &mut self,
        context: &ValidationContext,
        data_type: &DataType,
        location: &SourceRange,
    ) {
        self.validate_data_type(data_type, location);

        match data_type {
            DataType::StructType { variables, .. } => {
                variables.iter().for_each(|v| self.visit_variable(context, v))
            }
            DataType::ArrayType { referenced_type, .. } => {
                self.visit_data_type_declaration(context, referenced_type)
            }
            DataType::VarArgs { referenced_type: Some(referenced_type), .. } => {
                self.visit_data_type_declaration(context, referenced_type.as_ref());
            }
            _ => {}
        }
    }

    pub fn validate_data_type(&mut self, data_type: &DataType, location: &SourceRange) {
        match data_type {
            DataType::StructType { variables, .. } => {
                if variables.is_empty() {
                    self.push_diagnostic(Diagnostic::empty_variable_block(location.clone()));
                }
            }
            DataType::EnumType { elements: AstStatement::ExpressionList { expressions, .. }, .. }
                if expressions.is_empty() =>
            {
                self.push_diagnostic(Diagnostic::empty_variable_block(location.clone()));
            }
            DataType::VarArgs { referenced_type: None, sized: true } => {
                self.push_diagnostic(Diagnostic::missing_datatype(
                    Some(": Sized Variadics require a known datatype."),
                    location.clone(),
                ))
            }
            _ => {}
        }
    }
}

fn data_type_is_fb_or_class_instance(type_name: &str, index: &Index) -> bool {
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
