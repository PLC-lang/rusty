//! Lowering of a resolved CFC network into a Structured Text AST.
//!
//! The POU interface comes from the textual declaration, parsed by the ST
//! parser; the body statements are built directly as AST nodes, each anchored
//! to its element's `globalId` so diagnostics can point back into the diagram.

use plc_ast::ast::{AstFactory, AstNode, CompilationUnit};
use plc_ast::provider::IdProvider;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::{SourceLocation, SourceLocationFactory};
use plc_source::SourceCode;

use crate::model::{FbdObject, Pou};
use crate::resolve::{Argument, Resolved, Source, Statement};

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
                condition.as_ref().map(|condition| transpile_return(object, condition, &factory, &mut ids))
            }
            Statement::Call { block, arguments } => {
                Some(transpile_call(block, arguments, &factory, &mut ids))
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
    source: &Source,
    factory: &SourceLocationFactory,
    ids: &mut IdProvider,
) -> AstNode {
    let location = factory.create_block_location(sink.global_id);

    // Anchor both sides to the sink's diagram block so diagnostics point at the
    // element, not the synthetic identifier text.
    let mut left = helper::parse_identifier(sink.identifier().unwrap_or_default(), ids.clone());
    left.location = location.clone();
    let right = render_source(source, &location, ids);

    AstFactory::create_assignment(left, right, ids.next_id())
}

fn transpile_return(
    object: &FbdObject,
    condition: &Source,
    factory: &SourceLocationFactory,
    ids: &mut IdProvider,
) -> AstNode {
    let location = factory.create_block_location(object.global_id);

    // The wired input guards the return; a negated return element inverts it.
    let condition = render_source(condition, &location, ids);
    let condition = match object.negated() {
        true => AstFactory::create_not_expression(condition, location.clone(), ids.next_id()),
        false => condition,
    };

    AstFactory::create_return_statement(Some(condition), location, ids.next_id())
}

fn transpile_call(
    block: &FbdObject,
    arguments: &[Argument],
    factory: &SourceLocationFactory,
    ids: &mut IdProvider,
) -> AstNode {
    let location = factory.create_block_location(block.global_id);

    let target = block.call_target().unwrap_or_default();
    let mut operator = helper::parse_identifier(&target, ids.clone());
    operator.location = location.clone();

    // Each wired input becomes a `param := value` association; a negated input
    // pin inverts its value with a `NOT`.
    let parameters: Vec<AstNode> = arguments
        .iter()
        .map(|argument| {
            let mut left = helper::parse_identifier(&argument.pin.parameter_name, ids.clone());
            left.location = location.clone();
            let mut right = render_source(&argument.source, &location, ids);
            if argument.pin.negated {
                right = AstFactory::create_not_expression(right, location.clone(), ids.next_id());
            }
            AstFactory::create_assignment(left, right, ids.next_id())
        })
        .collect();

    let parameters = (!parameters.is_empty())
        .then(|| AstFactory::create_expression_list(parameters, location.clone(), ids.next_id()));

    AstFactory::create_call_statement(operator, parameters, ids.next_id(), location)
}

/// Build the expression a consumer reads: a plain source is its identifier; a
/// block output is a member of the block's instance, inverted when the output
/// pin is negated. The result is anchored to the consumer's diagram block.
fn render_source(source: &Source, location: &SourceLocation, ids: &mut IdProvider) -> AstNode {
    match source {
        Source::Variable(object) => {
            let mut expression = helper::parse_identifier(object.identifier().unwrap_or_default(), ids.clone());
            expression.location = location.clone();
            expression
        }
        Source::Output { block, pin } => {
            let member = format!("{}.{}", block.instance().unwrap_or_default(), pin.parameter_name);
            let mut expression = helper::parse_identifier(&member, ids.clone());
            expression.location = location.clone();
            match pin.negated {
                true => AstFactory::create_not_expression(expression, location.clone(), ids.next_id()),
                false => expression,
            }
        }
    }
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

    // Expected output for program-block calls, agreed before implementation
    // exists. A block lowers to a call carrying only its inputs; every output is
    // consumed as a member access on the program's persistent global, and
    // statements stay in raw priority order (no reordering, no temporaries).
    mod blocks {
        use crate::test_utils::transpile_project;

        #[test]
        fn program_call() {
            insta::assert_snapshot!(transpile_project("blocks/valid/program_call").unwrap(), @r"
        PROGRAM program_call
        VAR
            countIn : DINT;
            countOut : DINT;
        END_VAR
            counter(in := countIn);
            countOut := counter.out;
        END_PROGRAM");
        }

        #[test]
        fn program_feedback() {
            insta::assert_snapshot!(transpile_project("blocks/valid/program_feedback").unwrap(), @r"
        PROGRAM program_feedback
            counter(in := counter.out);
        END_PROGRAM");
        }

        #[test]
        fn program_fan_out() {
            insta::assert_snapshot!(transpile_project("blocks/valid/program_fan_out").unwrap(), @r"
        PROGRAM program_fan_out
        VAR
            seed : DINT;
            a : DINT;
            b : DINT;
        END_VAR
            counter(in := seed);
            a := counter.out;
            b := counter.out;
        END_PROGRAM");
        }

        #[test]
        fn program_chain() {
            insta::assert_snapshot!(transpile_project("blocks/valid/program_chain").unwrap(), @r"
        PROGRAM program_chain
        VAR
            seed : DINT;
            result : DINT;
        END_VAR
            counter(in := seed);
            doubler(in := counter.out);
            result := doubler.out;
        END_PROGRAM");
        }

        #[test]
        fn program_unordered() {
            insta::assert_snapshot!(transpile_project("blocks/valid/program_unordered").unwrap(), @r"
        PROGRAM program_unordered
        VAR
            countIn : DINT;
            countOut : DINT;
        END_VAR
            countOut := counter.out;
            counter(in := countIn);
        END_PROGRAM");
        }

        #[test]
        fn program_chain_scrambled() {
            insta::assert_snapshot!(transpile_project("blocks/valid/program_chain_scrambled").unwrap(), @r"
        PROGRAM program_chain_scrambled
        VAR
            seed : DINT;
            result : DINT;
        END_VAR
            c(in := b.out);
            result := c.out;
            a(in := seed);
            b(in := a.out);
        END_PROGRAM");
        }

        #[test]
        fn program_cycle() {
            insta::assert_snapshot!(transpile_project("blocks/valid/program_cycle").unwrap(), @r"
        PROGRAM program_cycle
            pong(in := ping.out);
            ping(in := pong.out);
        END_PROGRAM");
        }

        #[test]
        fn program_straddle() {
            insta::assert_snapshot!(transpile_project("blocks/valid/program_straddle").unwrap(), @r"
        PROGRAM program_straddle
        VAR
            seed : DINT;
            before : DINT;
            after : DINT;
        END_VAR
            before := counter.out;
            counter(in := seed);
            after := counter.out;
        END_PROGRAM");
        }

        #[test]
        fn program_mixed() {
            insta::assert_snapshot!(transpile_project("blocks/valid/program_mixed").unwrap(), @r"
        PROGRAM program_mixed
        VAR
            seed : DINT;
            p : DINT;
            q : DINT;
            r : DINT;
        END_VAR
            q := p;
            r := counter.out;
            counter(in := seed);
        END_PROGRAM");
        }

        // A negated input/in_out inverts its wired value; a negated output
        // inverts each read. The in_out's `NOT` is invalid downstream (E031), so
        // this fixture is transpile-only.
        #[test]
        fn program_negated() {
            insta::assert_snapshot!(transpile_project("blocks/valid/program_negated").unwrap(), @r"
        PROGRAM program_negated
        VAR
            localIn : DINT;
            localInOut : DINT;
            localOut : DINT;
        END_VAR
            program_0(in := NOT localIn, inout := NOT localInOut);
            localOut := NOT program_0.out;
        END_PROGRAM");
        }

        // A function-block block carries `instanceName`, so the call and the
        // output read run on the instance rather than the (program) type name.
        #[test]
        fn fb_call() {
            insta::assert_snapshot!(transpile_project("blocks/valid/fb_call").unwrap(), @r"
        PROGRAM fb_call
        VAR
            inst : counter;
            localIn : DINT;
            localOut : DINT;
        END_VAR
            inst(in := localIn);
            localOut := inst.out;
        END_PROGRAM");
        }

        // Distinct instances of one function block keep separate state, so the
        // chain that is degenerate for a program is meaningful here.
        #[test]
        fn fb_instances() {
            insta::assert_snapshot!(transpile_project("blocks/valid/fb_instances").unwrap(), @r"
        PROGRAM fb_instances
        VAR
            a : counter;
            b : counter;
            seed : DINT;
            result : DINT;
        END_VAR
            a(in := seed);
            b(in := a.out);
            result := b.out;
        END_PROGRAM");
        }

        #[test]
        fn fb_feedback() {
            insta::assert_snapshot!(transpile_project("blocks/valid/fb_feedback").unwrap(), @r"
        PROGRAM fb_feedback
        VAR
            inst : counter;
        END_VAR
            inst(in := inst.out);
        END_PROGRAM");
        }

        // Scrambled priority holds for an instance too: the output is read once
        // before the call and once after, off the same persistent member.
        #[test]
        fn fb_straddle() {
            insta::assert_snapshot!(transpile_project("blocks/valid/fb_straddle").unwrap(), @r"
        PROGRAM fb_straddle
        VAR
            inst : counter;
            seed : DINT;
            before : DINT;
            after : DINT;
        END_VAR
            before := inst.out;
            inst(in := seed);
            after := inst.out;
        END_PROGRAM");
        }

        // An action call: `typeName` is qualified (`owner.action`), so the call
        // dispatches to `inst.action` while the output is read off the instance.
        #[test]
        fn action_fb() {
            insta::assert_snapshot!(transpile_project("blocks/valid/action_fb").unwrap(), @r"
        PROGRAM action_fb
        VAR
            inst : counter;
            localIn : DINT;
            localOut : DINT;
        END_VAR
            inst.increment(in := localIn);
            localOut := inst.out;
        END_PROGRAM");
        }

        // A program-owned action has no instance; the owner is the singleton.
        #[test]
        fn action_program() {
            insta::assert_snapshot!(transpile_project("blocks/valid/action_program").unwrap(), @r"
        PROGRAM action_program
        VAR
            localIn : DINT;
            localOut : DINT;
        END_VAR
            P.bump(step := localIn);
            localOut := P.out;
        END_PROGRAM");
        }

        #[test]
        fn action_bare() {
            insta::assert_snapshot!(transpile_project("blocks/valid/action_bare").unwrap(), @r"
        PROGRAM action_bare
        VAR
            inst : counter;
        END_VAR
            inst.reset();
        END_PROGRAM");
        }
    }
}
