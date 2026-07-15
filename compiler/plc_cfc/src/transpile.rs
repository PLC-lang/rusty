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

use crate::model::Pou;
use crate::resolve::{Assignment, Resolved};

pub(crate) fn transpile(
    pou: &Pou,
    resolved: &Resolved,
    source: &SourceCode,
    mut ids: IdProvider,
) -> (CompilationUnit, Vec<Diagnostic>) {
    let (mut unit, diagnostics) = helper::parse_interface(pou, source, ids.clone());

    let factory = SourceLocationFactory::for_source(source);
    let statements = resolved
        .assignments
        .iter()
        .map(|assignment| transpile_assignment(assignment, &factory, &mut ids))
        .collect();
    if let Some(implementation) = unit.implementations.first_mut() {
        implementation.statements = statements;
    }

    (unit, diagnostics)
}

fn transpile_assignment(
    assignment: &Assignment,
    factory: &SourceLocationFactory,
    ids: &mut IdProvider,
) -> AstNode {
    let location = factory.create_block_location(assignment.sink.global_id);

    let mut left = helper::parse_identifier(assignment.sink.identifier().unwrap_or_default(), ids.clone());
    let mut right = helper::parse_identifier(assignment.source.identifier().unwrap_or_default(), ids.clone());
    left.location = location.clone();
    right.location = location;

    AstFactory::create_assignment(left, right, ids.next_id())
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
}
