use crate::{Diagnostic, ast::{DataType, DataTypeDeclaration, SourceRange, Variable, VariableBlock}, index::Index};

use super::SemanticDiagnosticAcceptor;

/// validates variables & datatypes

pub struct VariableValidator<'i> {
    index: &'i Index,
}

impl<'i> VariableValidator<'i> {
    pub fn new(index: &'i Index) -> VariableValidator {
        VariableValidator {
            index
        }
    }

    pub fn validate_variable_block(&self, _block: &VariableBlock, _da: &mut dyn SemanticDiagnosticAcceptor) {
    }

    pub fn validate_variable(&self, _variable: &Variable, _da: &mut dyn SemanticDiagnosticAcceptor) {
    }

    pub fn validate_data_type_declaration(&self, _declaration: &DataTypeDeclaration, _da: &mut dyn SemanticDiagnosticAcceptor) {
    }

    pub fn validate_data_type(&self, _declaration: &DataType, _da: &mut dyn SemanticDiagnosticAcceptor) {
    }
}
