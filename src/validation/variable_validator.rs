use crate::{
    ast::{DataType, DataTypeDeclaration, SourceRange, Variable, VariableBlock},
    index::Index,
    Diagnostic,
};

use super::ValidationContext;

/// validates variables & datatypes

pub struct VariableValidator<'i> {
    index: &'i Index,
}

impl<'i> VariableValidator<'i> {
    pub fn new(index: &'i Index) -> VariableValidator {
        VariableValidator { index }
    }

    pub fn validate_variable_block(&self, _block: &VariableBlock, _da: &mut ValidationContext) {}

    pub fn validate_variable(&self, _variable: &Variable, _da: &mut ValidationContext) {}

    pub fn validate_data_type_declaration(
        &self,
        _declaration: &DataTypeDeclaration,
        _da: &mut ValidationContext,
    ) {
    }

    pub fn validate_data_type(
        &self,
        declaration: &DataType,
        location: &SourceRange,
        context: &mut ValidationContext,
    ) {
        match declaration {
            DataType::StructType { variables, .. } => {
                if variables.is_empty() {
                    context.report(Diagnostic::empty_variable_block(location.clone()));
                }
            }
            DataType::EnumType { elements, .. } => {
                if elements.is_empty() {
                    context.report(Diagnostic::empty_variable_block(location.clone()));
                }
            }
            DataType::SubRangeType {
                name,
                referenced_type,
                bounds,
            } => todo!(),
            DataType::ArrayType {
                name,
                bounds,
                referenced_type,
            } => todo!(),
            DataType::StringType {
                name,
                is_wide,
                size,
            } => todo!(),
            DataType::VarArgs { referenced_type } => todo!(),
        }
    }
}

#[cfg(test)]
mod variable_validator_tests {
    use crate::{
        ast,
        index::{self, Index},
        lexer::lex,
        parser::{parse, PResult},
        validation::Validator,
        Diagnostic,
    };

    fn parse_and_validate(src: &str) -> PResult<Vec<Diagnostic>> {
        let mut idx = Index::new();
        let (mut ast, _) = parse(lex(src))?;
        ast::pre_process(&mut ast);
        idx.import(index::visitor::visit(&ast));

        let mut validator = Validator::new(&idx);
        validator.visit_unit(&ast);

        let diagnostics = validator.diagnostics().cloned().collect();
        Ok(diagnostics)
    }

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
        )
        .unwrap();

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
        )
        .unwrap();

        assert_eq!(
            diagnostics,
            vec![
                Diagnostic::empty_variable_block((14..27).into()),
                Diagnostic::empty_variable_block((112..114).into())
            ]
        );
    }
}
