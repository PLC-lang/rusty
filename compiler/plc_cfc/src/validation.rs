//! Semantic validation of a resolved CFC network.

use std::collections::HashSet;

use plc_ast::provider::IdProvider;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocationFactory;
use plc_source::SourceCode;

use crate::model::FbdObject;
use crate::resolve::{BrokenReason, Resolved, Statement};
use crate::transpile::helper::parse_identifier;

pub(crate) fn validate(resolved: &Resolved, source: &SourceCode, ids: IdProvider) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    diagnostics.extend(validate_expressions(resolved, source, ids));
    diagnostics.extend(validate_returns(resolved, source));
    diagnostics.extend(validate_jumps(resolved, source));
    diagnostics.extend(validate_connectors(resolved, source));
    diagnostics.extend(validate_unconnected(resolved, source));
    diagnostics
}

fn validate_expressions(resolved: &Resolved, source: &SourceCode, ids: IdProvider) -> Vec<Diagnostic> {
    let factory = SourceLocationFactory::for_source(source);
    let mut diagnostics = Vec::new();

    // Flag any identifier that isn't a plain reference or literal (E083); a data
    // source/sink can't carry a call or a compound expression.
    let mut check = |object: &FbdObject| {
        let Some(text) = object.identifier() else { return };
        if !helper::is_supported(&parse_identifier(text, ids.clone()).stmt) {
            let location = factory.create_block_location(object.global_id);
            diagnostics.push(Diagnostic::unsupported_cfc_expression(text, location));
        }
    };

    // A return's condition is left to the main pipeline's validator, which sees
    // the full interface; here we only have the untyped model. Block outputs are
    // synthesized member references, so only free-text data sources are vetted.
    for statement in &resolved.statements {
        match statement {
            Statement::Assignment { sink, source } => {
                check(sink);
                if let Some(object) = source.variable() {
                    check(object);
                }
            }
            Statement::Call { arguments, .. } => {
                for argument in arguments {
                    if let Some(object) = argument.source.variable() {
                        check(object);
                    }
                }
            }
            // A return/jump condition is left to the main pipeline, like any
            // other typed expression; a label carries no data source.
            Statement::Return { .. } | Statement::Jump { .. } | Statement::Label { .. } => {}
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

fn validate_jumps(resolved: &Resolved, source: &SourceCode) -> Vec<Diagnostic> {
    let factory = SourceLocationFactory::for_source(source);
    let mut diagnostics = Vec::new();

    // The labels a jump can land on, and the targets some jump names. Both drive
    // the cross-checks below; a label and its jump exist as separate elements.
    let labels: HashSet<&str> = resolved
        .statements
        .iter()
        .filter_map(|it| match it {
            Statement::Label { object } => object.label(),
            _ => None,
        })
        .collect();
    let targets: HashSet<&str> = resolved
        .statements
        .iter()
        .filter_map(|it| match it {
            Statement::Jump { object, .. } => object.target_label(),
            _ => None,
        })
        .collect();

    for statement in &resolved.statements {
        match statement {
            // A jump with no wired condition can never be taken; the FALSE guard
            // it lowers to is valid ST, so only this pass can flag it (E145).
            Statement::Jump { object, condition } => {
                let location = factory.create_block_location(object.global_id);
                if condition.is_none() {
                    diagnostics.push(Diagnostic::disconnected_jump(location.clone()));
                }
                // A jump to a name no label defines has nowhere to land (E142).
                let target = object.target_label().unwrap_or_default();
                if !labels.contains(target) {
                    diagnostics.push(Diagnostic::undefined_jump_target(target, location));
                }
            }
            // A label no jump names is dead routing — kept, but flagged (E143).
            Statement::Label { object } => {
                let name = object.label().unwrap_or_default();
                if !targets.contains(name) {
                    let location = factory.create_block_location(object.global_id);
                    diagnostics.push(Diagnostic::unused_label(name, location));
                }
            }
            _ => {}
        }
    }

    // Two labels sharing a name make a jump to it ambiguous (E144).
    for label in &resolved.duplicate_labels {
        let location = factory.create_block_location(label.global_id);
        diagnostics.push(Diagnostic::duplicate_label(label.label().unwrap_or_default(), location));
    }

    diagnostics
}

fn validate_connectors(resolved: &Resolved, source: &SourceCode) -> Vec<Diagnostic> {
    let factory = SourceLocationFactory::for_source(source);
    let mut diagnostics = Vec::new();

    // Two connectors claiming one label make the named source ambiguous (E081).
    for connector in &resolved.duplicate_connectors {
        let location = factory.create_block_location(connector.global_id);
        diagnostics.push(Diagnostic::duplicate_connector(connector.label().unwrap_or_default(), location));
    }

    // A consumed chain that never reached a source: an input-less connector
    // (E086) or a continuation whose label no connector claims (E082).
    for broken in &resolved.broken {
        let location = factory.create_block_location(broken.element.global_id);
        let label = broken.element.label().unwrap_or_default();
        diagnostics.push(match broken.reason {
            BrokenReason::OpenConnector => Diagnostic::open_connector(label, location),
            BrokenReason::DanglingContinuation => Diagnostic::dangling_continuation(label, location),
        });
    }

    diagnostics
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

    mod jumps {
        use crate::test_utils::{diagnostics, transpile_project};

        // A jump with no wired condition can never be taken (warning, valid).
        #[test]
        fn disconnected_jump() {
            insta::assert_snapshot!(diagnostics("jumps/valid/disconnected_jump"), @r"
        warning[E145]: Jump element is not connected to a condition and can never be taken
         = disconnected_jump.cfc: Block 1
        ");
        }

        // A label no jump targets is emitted but flagged (warning, valid).
        #[test]
        fn unused_label() {
            insta::assert_snapshot!(diagnostics("jumps/valid/unused_label"), @r"
        warning[E143]: Label `orphan` is not referenced by any jump
         = unused_label.cfc: Block 4
        ");
        }

        #[test]
        fn undefined_jump_target() {
            insta::assert_snapshot!(transpile_project("jumps/invalid/undefined_jump_target").unwrap_err(), @r"
        error[E142]: Jump refers to undefined label `missing`
         = undefined_jump_target.cfc: Block 3
        ");
        }

        #[test]
        fn duplicate_label() {
            insta::assert_snapshot!(transpile_project("jumps/invalid/duplicate_label").unwrap_err(), @r"
        error[E144]: Label `dup` is already defined
         = duplicate_label.cfc: Block 5
        ");
        }
    }

    mod connectors {
        use crate::test_utils::{diagnostics, transpile_project};

        #[test]
        fn duplicate_connector() {
            insta::assert_snapshot!(transpile_project("connectors/invalid/duplicate_connector").unwrap_err(), @r"
        error[E081]: Connector `x` is already defined
         = duplicate_connector.cfc: Block 10
        ");
        }

        #[test]
        fn without_source() {
            insta::assert_snapshot!(transpile_project("connectors/invalid/without_source").unwrap_err(), @r"
        error[E086]: Connector `x` has no incoming connection
         = without_source.cfc: Block 5
        ");
        }

        #[test]
        fn dangling_continuation() {
            insta::assert_snapshot!(transpile_project("connectors/invalid/dangling_continuation").unwrap_err(), @r"
        error[E082]: Continuation `x` has no matching connector
         = dangling_continuation.cfc: Block 6
        ");
        }

        #[test]
        fn unused_is_quiet() {
            // A connector/continuation nobody reads emits no statement and, being
            // unconsumed, no diagnostic — even with the connector left open.
            insta::assert_snapshot!(diagnostics("connectors/valid/unused"), @"");
        }
    }
}
