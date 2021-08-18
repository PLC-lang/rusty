use super::ValidationContext;
use crate::{
    ast::{AstStatement, SourceRange},
    Diagnostic,
};

/// validates control-statements, assignments

pub struct StatementValidator {
    pub diagnostics: Vec<Diagnostic>,
}

impl StatementValidator {
    pub fn new() -> StatementValidator {
        StatementValidator {
            diagnostics: Vec::new(),
        }
    }

    pub fn validate_statement(&mut self, statement: &AstStatement, context: &ValidationContext) {
        if let AstStatement::Reference {
            name, location, id, ..
        } = statement
        {
            self.validate_reference(id, name, location, context);
        }
    }

    fn validate_reference(
        &mut self,
        id: &usize,
        ref_name: &str,
        location: &SourceRange,
        context: &ValidationContext,
    ) {
        if !context.ast_annotation.has_type_annotation(id) {
            self.diagnostics
                .push(Diagnostic::unrseolved_reference(ref_name, location.clone()));
        }
    }
}
