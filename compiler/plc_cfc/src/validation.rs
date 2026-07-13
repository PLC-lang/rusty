//! Semantic validation of a resolved CFC network.

use plc_ast::provider::IdProvider;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocationFactory;
use plc_source::SourceCode;

use crate::model::FbdObject;
use crate::resolve::{Resolved, Statement};
use crate::transpile::helper::parse_identifier;

pub(crate) fn validate(resolved: &Resolved, source: &SourceCode, ids: IdProvider) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    diagnostics.extend(validate_expressions(resolved, source, ids));
    diagnostics.extend(validate_returns(resolved, source));
    diagnostics.extend(validate_unconnected(resolved, source));
    diagnostics
}

fn validate_expressions(resolved: &Resolved, source: &SourceCode, ids: IdProvider) -> Vec<Diagnostic> {
    let factory = SourceLocationFactory::for_source(source);
    let mut diagnostics = Vec::new();

    let mut check = |object: &FbdObject| {
        let Some(text) = object.identifier() else { return };
        if !helper::is_supported(&parse_identifier(text, ids.clone()).stmt) {
            let location = factory.create_block_location(object.global_id);
            diagnostics.push(Diagnostic::unsupported_cfc_expression(text, location));
        }
    };

    // A return's condition is left to the main pipeline's validator, which sees
    // the full interface; here we only have the untyped model.
    for statement in &resolved.statements {
        if let Statement::Assignment { sink, source } = statement {
            check(sink);
            check(source);
        }
    }

    diagnostics
}

fn validate_returns(resolved: &Resolved, source: &SourceCode) -> Vec<Diagnostic> {
    let factory = SourceLocationFactory::for_source(source);
    resolved
        .statements
        .iter()
        .filter_map(|statement| match statement {
            Statement::Return { object, condition: None } => {
                Some(Diagnostic::disconnected_return(factory.create_block_location(object.global_id)))
            }
            _ => None,
        })
        .collect()
}

fn validate_unconnected(resolved: &Resolved, source: &SourceCode) -> Vec<Diagnostic> {
    let factory = SourceLocationFactory::for_source(source);
    resolved
        .unconnected
        .iter()
        .map(|object| {
            let location = factory.create_block_location(object.global_id);
            Diagnostic::unconnected_element(object.identifier().unwrap_or("<unnamed>"), location)
        })
        .collect()
}

mod helper {
    use plc_ast::ast::AstStatement;

    pub(super) fn is_supported(statement: &AstStatement) -> bool {
        match statement {
            // See through parentheses, e.g. `(foo)` or `((5))`.
            AstStatement::ParenExpression(inner) => is_supported(&inner.stmt),
            AstStatement::Literal(_) | AstStatement::ReferenceExpr(_) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    mod variables {
        use crate::test_utils::{diagnostics, transpile_project};

        #[test]
        fn call_expression() {
            insta::assert_snapshot!(transpile_project("variables/invalid/call_expression").unwrap_err(), @r"
        error[E083]: Unsupported CFC expression: `MAX(foo, bar)`
         = call_expression.cfc: Block 1
        ");
        }

        #[test]
        fn binary_expression() {
            insta::assert_snapshot!(transpile_project("variables/invalid/binary_expression").unwrap_err(), @r"
        error[E083]: Unsupported CFC expression: `foo + 1`
         = binary_expression.cfc: Block 1
        ");
        }

        #[test]
        fn unconnected_variables() {
            insta::assert_snapshot!(diagnostics("variables/valid/unconnected_variables"), @r"
        warning[E084]: Element `foo` is unconnected and will be ignored
         = unconnected_variables.cfc: Block 1

        warning[E084]: Element `bar` is unconnected and will be ignored
         = unconnected_variables.cfc: Block 4
        ");
        }
    }

    mod returns {
        use crate::test_utils::transpile_project;

        #[test]
        fn disconnected_return() {
            insta::assert_snapshot!(transpile_project("returns/invalid/disconnected_return").unwrap_err(), @r"
        error[E085]: Return element is not connected to a condition
         = disconnected_return.cfc: Block 4
        ");
        }
    }
}
