use plc_diagnostics::diagnostics::Diagnostic;
use plc_index::GlobalContext;

use crate::index::{
    const_expressions::{ConstExpression, UnresolvableKind},
    Index,
};

use super::Validators;

pub struct ConstExpressionValidator<'ctx> {
    context: &'ctx GlobalContext,
    pub diagnostics: Vec<Diagnostic>,
}

impl<'a> Validators for ConstExpressionValidator<'a> {
    fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }
    fn take_diagnostics(&mut self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.diagnostics)
    }
}

impl<'ctx> ConstExpressionValidator<'ctx> {
    pub fn new(context: &'ctx GlobalContext) -> Self {
        Self { context, diagnostics: vec![] }
    }

    pub fn validate(&mut self, index: &Index) {
        for it in index.get_const_expressions().into_iter() {
            let Some(expr) = index.get_const_expressions().find_const_expression(&it.0) else { continue };

            match expr {
                ConstExpression::Unresolvable {
                    reason: UnresolvableKind::Overflow(reason, location),
                    ..
                } => self.push_diagnostic(
                    Diagnostic::warning(reason).with_error_code("E038").with_location(location.to_owned()),
                ),
                ConstExpression::Unresolvable {
                    statement,
                    reason: UnresolvableKind::NonConstant(reason),
                } => self.push_diagnostic(
                    Diagnostic::error(format!("Expression must be constant. {reason}",))
                        .with_error_code("E033")
                        .with_location(statement.get_location()),
                ),
                ConstExpression::Unresolved { statement, .. } => {
                    if let Some(name) = statement.get_flat_reference_name() {
                        self.push_diagnostic(
                            Diagnostic::error(format!("Unresolved constant reference: `{name}`",))
                                .with_error_code("E033")
                                .with_location(statement.get_location()),
                        )
                    } else {
                        let expr = self.context.slice(&statement.location);
                        self.push_diagnostic(
                            Diagnostic::error(format!("Unresolved constant expression: `{}`", expr))
                                .with_error_code("E033")
                                .with_location(statement.get_location()),
                        )
                    }
                }
                _ => continue,
            }
        }
    }
}
