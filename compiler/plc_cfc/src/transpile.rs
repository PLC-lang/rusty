//! Lowering of a resolved CFC network into a Structured Text AST.
//!
//! The POU interface comes from the textual declaration, parsed by the ST
//! parser; the body statements are built directly as AST nodes, each anchored
//! to its element's `globalId` so diagnostics can point back into the diagram.

use plc_ast::ast::{AstFactory, AstNode, CompilationUnit};
use plc_ast::provider::IdProvider;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocationFactory;
use plc_source::SourceCode;

use crate::model::{FbdObject, Pou};
use crate::resolve::{Resolved, Statement};

pub(crate) fn transpile(
    pou: &Pou,
    resolved: &Resolved,
    source: &SourceCode,
    mut ids: IdProvider,
) -> (CompilationUnit, Vec<Diagnostic>) {
    // The interface — signature and VAR blocks — comes from the declaration.
    let (mut unit, diagnostics) = helper::parse_interface(pou, source, ids.clone());

    // Build the body: one AST node per resolved statement, already in order. A
    // disconnected return carries no condition and contributes nothing.
    let factory = SourceLocationFactory::for_source(source);
    let statements = resolved
        .statements
        .iter()
        .filter_map(|statement| match statement {
            Statement::Assignment { sink, source } => {
                Some(transpile_assignment(sink, source, &factory, &mut ids))
            }
            Statement::Return { object, condition } => {
                condition.map(|condition| transpile_return(object, condition, &factory, &mut ids))
            }
        })
        .collect();

    if let Some(implementation) = unit.implementations.first_mut() {
        implementation.statements = statements;
    }

    (unit, diagnostics)
}

fn transpile_assignment(
    sink: &FbdObject,
    source: &FbdObject,
    factory: &SourceLocationFactory,
    ids: &mut IdProvider,
) -> AstNode {
    let location = factory.create_block_location(sink.global_id);

    // Parse both sides as expressions, then anchor them to the sink's diagram
    // block so diagnostics point at the element, not the synthetic identifier text.
    let mut left = helper::parse_identifier(sink.identifier().unwrap_or_default(), ids.clone());
    let mut right = helper::parse_identifier(source.identifier().unwrap_or_default(), ids.clone());
    left.location = location.clone();
    right.location = location;

    AstFactory::create_assignment(left, right, ids.next_id())
}

fn transpile_return(
    object: &FbdObject,
    condition: &FbdObject,
    factory: &SourceLocationFactory,
    ids: &mut IdProvider,
) -> AstNode {
    let location = factory.create_block_location(object.global_id);

    // The wired input guards the return; a negated pin inverts it with a `NOT`.
    let mut condition = helper::parse_identifier(condition.identifier().unwrap_or_default(), ids.clone());
    condition.location = location.clone();
    let condition = match object.negated() {
        true => AstFactory::create_not_expression(condition, location.clone(), ids.next_id()),
        false => condition,
    };

    AstFactory::create_return_statement(Some(condition), location, ids.next_id())
}

pub(crate) mod helper {
    use plc::lexer;
    use plc::parser::{self, expressions_parser::parse_expression};
    use plc_ast::ast::{AstNode, CompilationUnit, LinkageType};
    use plc_ast::provider::IdProvider;
    use plc_diagnostics::diagnostics::Diagnostic;
    use plc_source::source_location::SourceLocationFactory;
    use plc_source::{SourceCode, SourceContainer};

    use crate::model::{Pou, PouKind};

    /// Parse an identifier field into an expression. Its own text locations are
    /// discarded by the caller in favour of a block location.
    pub(crate) fn parse_identifier(text: &str, ids: IdProvider) -> AstNode {
        let factory = SourceLocationFactory::internal(text);
        let mut session = lexer::lex_with_ids(text, ids, factory);
        parse_expression(&mut session)
    }

    pub(super) fn parse_interface(
        pou: &Pou,
        source: &SourceCode,
        ids: IdProvider,
    ) -> (CompilationUnit, Vec<Diagnostic>) {
        // The declaration omits its closing keyword; re-attach it so the ST
        // parser sees a complete POU with an empty body.
        let end_keyword = match pou.kind() {
            PouKind::Function => "END_FUNCTION",
            PouKind::FunctionBlock => "END_FUNCTION_BLOCK",
            PouKind::Program => "END_PROGRAM",
        };
        let declaration = format!("{}\n{end_keyword}", pou.content().declaration().unwrap_or_default());

        let declaration = SourceCode { source: declaration, path: source.path.clone() };
        let factory = SourceLocationFactory::for_source(&declaration);
        let session = lexer::lex_with_ids(&declaration.source, ids, factory);
        parser::parse(session, LinkageType::Internal, source.get_location_str())
    }
}

#[cfg(test)]
mod tests {
    mod variables {
        use crate::test_utils::transpile_project;

        #[test]
        fn simple_assignment() {
            insta::assert_snapshot!(transpile_project("variables/valid/simple_assignment").unwrap(), @r"
        FUNCTION simple_assignment : INT
        VAR
            foo : DINT;
            bar : DINT;
        END_VAR
            bar := foo;
        END_FUNCTION");
        }

        #[test]
        fn reciprocal_assignment() {
            insta::assert_snapshot!(transpile_project("variables/valid/reciprocal_assignment").unwrap(), @r"
        PROGRAM reciprocal_assignment
        VAR
            foo : DINT;
            bar : DINT;
        END_VAR
            bar := foo;
            foo := bar;
        END_PROGRAM");
        }

        #[test]
        fn fan_out() {
            insta::assert_snapshot!(transpile_project("variables/valid/fan_out").unwrap(), @r"
        FUNCTION_BLOCK fan_out
        VAR
            foo : DINT;
            bar : DINT;
            baz : DINT;
        END_VAR
            bar := foo;
            baz := foo;
        END_FUNCTION_BLOCK");
        }

        #[test]
        fn literal_assignment() {
            insta::assert_snapshot!(transpile_project("variables/valid/literal_assignment").unwrap(), @r"
        FUNCTION literal_assignment : INT
        VAR
            foo : DINT;
        END_VAR
            foo := 5;
        END_FUNCTION");
        }

        #[test]
        fn unconnected_variables() {
            insta::assert_snapshot!(transpile_project("variables/valid/unconnected_variables").unwrap(), @r"
        FUNCTION unconnected_variables : INT
        VAR
            foo : DINT;
            bar : DINT;
        END_VAR
            bar := foo;
        END_FUNCTION");
        }
    }

    mod returns {
        use crate::test_utils::transpile_project;

        #[test]
        fn conditional_return() {
            insta::assert_snapshot!(transpile_project("returns/valid/conditional_return").unwrap(), @r"
        FUNCTION conditional_return : INT
        VAR
            myCondition : BOOL;
        END_VAR
            IF myCondition THEN RETURN; END_IF;
        END_FUNCTION");
        }

        #[test]
        fn negated_return() {
            insta::assert_snapshot!(transpile_project("returns/valid/negated_return").unwrap(), @r"
        FUNCTION negated_return : INT
        VAR
            myCondition : BOOL;
        END_VAR
            IF NOT myCondition THEN RETURN; END_IF;
        END_FUNCTION");
        }
    }

    mod connectors {
        use crate::test_utils::transpile_project;

        #[test]
        fn assignment() {
            insta::assert_snapshot!(transpile_project("connectors/valid/assignment").unwrap(), @r"
        PROGRAM assignment
        VAR
            foo : DINT;
            bar : DINT;
        END_VAR
            bar := foo;
        END_PROGRAM");
        }

        #[test]
        fn fan_out() {
            insta::assert_snapshot!(transpile_project("connectors/valid/fan_out").unwrap(), @r"
        PROGRAM fan_out
        VAR
            foo : DINT;
            bar : DINT;
            baz : DINT;
        END_VAR
            bar := foo;
            baz := foo;
        END_PROGRAM");
        }

        #[test]
        fn chain() {
            insta::assert_snapshot!(transpile_project("connectors/valid/chain").unwrap(), @r"
        PROGRAM chain
        VAR
            foo : DINT;
            bar : DINT;
        END_VAR
            bar := foo;
        END_PROGRAM");
        }

        #[test]
        fn unused() {
            insta::assert_snapshot!(transpile_project("connectors/valid/unused").unwrap(), @r"
        PROGRAM unused
        VAR
            foo : DINT;
            bar : DINT;
        END_VAR
        END_PROGRAM");
        }
    }
}
