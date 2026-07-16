//! Lowering of a resolved CFC network into a Structured Text AST.
//!
//! The POU interface comes from the textual declaration, parsed by the ST
//! parser; the body statements are built directly as AST nodes, each anchored
//! to its element's `globalId` so diagnostics can point back into the diagram.

use std::collections::HashSet;

use plc_ast::ast::{AstFactory, AstNode, CompilationUnit, VariableBlock};
use plc_ast::provider::IdProvider;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::{SourceLocation, SourceLocationFactory};
use plc_source::SourceCode;

use crate::lowering;
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

    // A stateless function's outputs are read through temporaries; `resolve`
    // already recorded which output pins a consumer reads, so only those are
    // captured.
    let factory = SourceLocationFactory::for_source(source);
    let consumed = &resolved.consumed_outputs;

    // Build the body: one AST node per resolved statement, already in order. A
    // disconnected return carries no condition and contributes nothing.
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
                Some(transpile_call(block, arguments, consumed, &factory, &mut ids))
            }
        })
        .collect();

    if let Some(implementation) = unit.implementations.first_mut() {
        implementation.statements = statements;
    }

    // Declare the captured function outputs as persistent temporaries; their
    // placeholder types are resolved after annotation.
    let temporaries = lowering::temporaries(resolved, consumed, &factory);
    if let (Some(pou), false) = (unit.pous.first_mut(), temporaries.is_empty()) {
        pou.variable_blocks.push(VariableBlock::default().with_variables(temporaries));
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
    consumed: &HashSet<usize>,
    factory: &SourceLocationFactory,
    ids: &mut IdProvider,
) -> AstNode {
    let location = factory.create_block_location(block.global_id);

    let target = block.call_target().unwrap_or_default();
    let mut operator = helper::parse_identifier(&target, ids.clone());
    operator.location = location.clone();

    // A stateful block passes only its wired inputs (an unwired one keeps last
    // cycle); a function must supply every parameter, so it fills the gaps.
    let parameters = match block.is_function() {
        true => function_arguments(block, arguments, consumed, &location, ids),
        false => arguments.iter().map(|argument| input_argument(argument, &location, ids)).collect(),
    };

    let parameters = (!parameters.is_empty())
        .then(|| AstFactory::create_expression_list(parameters, location.clone(), ids.next_id()));
    let call = AstFactory::create_call_statement(operator, parameters, ids.next_id(), location.clone());

    // A consumed return pin turns the call into `temp := fn(..)`.
    match block.is_function().then(|| block.return_pin()).flatten() {
        Some(pin) if lowering::is_consumed(pin, consumed) => {
            let mut left = helper::parse_identifier(&lowering::temp_name(block, pin), ids.clone());
            left.location = location.clone();
            AstFactory::create_assignment(left, call, ids.next_id())
        }
        _ => call,
    }
}

/// A function call must supply every parameter (the compiler rejects a partial
/// list). Each input/in_out takes its wired value or an empty argument
/// (`param := `); each non-return output is captured into its temporary when
/// read, or discarded through an empty sink (`param => `).
fn function_arguments(
    block: &FbdObject,
    arguments: &[Argument],
    consumed: &HashSet<usize>,
    location: &SourceLocation,
    ids: &mut IdProvider,
) -> Vec<AstNode> {
    let mut parameters = Vec::new();

    for pin in block.input_pins().iter().chain(block.inout_pins()) {
        parameters.push(match arguments.iter().find(|argument| std::ptr::eq(argument.pin, pin)) {
            Some(argument) => input_argument(argument, location, ids),
            None => empty_argument(&pin.parameter_name, location, ids),
        });
    }

    for pin in block.output_pins() {
        if block.is_return_pin(pin) {
            continue;
        }
        let mut left = helper::parse_identifier(&pin.parameter_name, ids.clone());
        left.location = location.clone();
        let right = match lowering::is_consumed(pin, consumed) {
            true => {
                let mut temp = helper::parse_identifier(&lowering::temp_name(block, pin), ids.clone());
                temp.location = location.clone();
                temp
            }
            false => AstFactory::create_empty_statement(location.clone(), ids.next_id()),
        };
        parameters.push(AstFactory::create_output_assignment(left, right, ids.next_id()));
    }

    parameters
}

/// A wired `param := value` association; a negated input pin inverts its value.
fn input_argument(argument: &Argument, location: &SourceLocation, ids: &mut IdProvider) -> AstNode {
    let mut left = helper::parse_identifier(&argument.pin.parameter_name, ids.clone());
    left.location = location.clone();
    let mut right = render_source(&argument.source, location, ids);
    if argument.pin.negated {
        right = AstFactory::create_not_expression(right, location.clone(), ids.next_id());
    }
    AstFactory::create_assignment(left, right, ids.next_id())
}

/// An empty input argument (`param := `) for a parameter with no wired value.
fn empty_argument(parameter: &str, location: &SourceLocation, ids: &mut IdProvider) -> AstNode {
    let mut left = helper::parse_identifier(parameter, ids.clone());
    left.location = location.clone();
    let right = AstFactory::create_empty_statement(location.clone(), ids.next_id());
    AstFactory::create_assignment(left, right, ids.next_id())
}

/// Build the expression a consumer reads: a plain source is its identifier; a
/// block output is read through its instance member (stateful block) or its
/// generated temporary (stateless function), inverted when the pin is negated.
/// The result is anchored to the consumer's diagram block.
fn render_source(source: &Source, location: &SourceLocation, ids: &mut IdProvider) -> AstNode {
    match source {
        Source::Variable(object) => {
            let mut expression =
                helper::parse_identifier(object.identifier().unwrap_or_default(), ids.clone());
            expression.location = location.clone();
            expression
        }
        Source::Output { block, pin } => {
            let mut expression = helper::parse_identifier(&helper::output_read(block, pin), ids.clone());
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

    use crate::lowering;
    use crate::model::{FbdObject, Pin, Pou, PouKind};

    /// How a block output is read: a stateful block exposes it as an instance
    /// member; a stateless function exposes it only through its temporary.
    pub(super) fn output_read(block: &FbdObject, pin: &Pin) -> String {
        match block.is_function() {
            true => lowering::temp_name(block, pin),
            false => format!("{}.{}", block.instance().unwrap_or_default(), pin.parameter_name),
        }
    }

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

    // Stateless functions: a block with no `instanceName` whose output pins carry
    // a return (the pin named after the callee). Outputs don't persist, so a
    // consumed output is captured into a generated persistent temporary named
    // `__out_<paramName>_<globalId>` (the return through `__t := fn(..)`, other
    // outputs through `=>`) and reads reference the temporary instead of
    // `inst.member`. A function must receive *every* parameter, so unwired
    // inputs and unread outputs are passed as empty arguments (`p := ` / `p => `).
    // Temp types are placeholders (`return@<fn>`, `output@<fn>@<pin>`) resolved
    // after annotation.
    mod functions {
        use crate::test_utils::transpile_project;

        #[test]
        fn function_call() {
            insta::assert_snapshot!(transpile_project("blocks/valid/function_call").unwrap(), @r"
        FUNCTION function_call : INT
        VAR_INPUT
            in1 : DINT;
            in2 : DINT;
        END_VAR
        VAR_OUTPUT
            doubledOut : DINT;
        END_VAR
        VAR
            __out_myAdd_1 : return@myAdd;
            __out_myAddDoubled_1 : output@myAdd@myAddDoubled;
        END_VAR
            __out_myAdd_1 := myAdd(in1 := in1, in2 := in2, myAddDoubled => __out_myAddDoubled_1);
            function_call := __out_myAdd_1;
            doubledOut := __out_myAddDoubled_1;
        END_FUNCTION");
        }

        #[test]
        fn function_return_only() {
            insta::assert_snapshot!(transpile_project("blocks/valid/function_return_only").unwrap(), @r"
        PROGRAM function_return_only
        VAR
            a : DINT;
            b : DINT;
            sum : DINT;
        END_VAR
        VAR
            __out_myAdd_1 : return@myAdd;
        END_VAR
            __out_myAdd_1 := myAdd(in1 := a, in2 := b, myAddDoubled => );
            sum := __out_myAdd_1;
        END_PROGRAM");
        }

        #[test]
        fn function_fan_out() {
            insta::assert_snapshot!(transpile_project("blocks/valid/function_fan_out").unwrap(), @r"
        PROGRAM function_fan_out
        VAR
            a : DINT;
            b : DINT;
            x : DINT;
            y : DINT;
        END_VAR
        VAR
            __out_myAdd_1 : return@myAdd;
        END_VAR
            __out_myAdd_1 := myAdd(in1 := a, in2 := b, myAddDoubled => );
            x := __out_myAdd_1;
            y := __out_myAdd_1;
        END_PROGRAM");
        }

        #[test]
        fn function_chain() {
            insta::assert_snapshot!(transpile_project("blocks/valid/function_chain").unwrap(), @r"
        PROGRAM function_chain
        VAR
            seed : DINT;
            k : DINT;
            result : DINT;
        END_VAR
        VAR
            __out_myAdd_1 : return@myAdd;
            __out_myAdd_10 : return@myAdd;
        END_VAR
            __out_myAdd_1 := myAdd(in1 := seed, in2 := k, myAddDoubled => );
            __out_myAdd_10 := myAdd(in1 := __out_myAdd_1, in2 := k, myAddDoubled => );
            result := __out_myAdd_10;
        END_PROGRAM");
        }

        // Scrambled priorities: `early` (0) reads the return before the call (1).
        // Order is preserved, so the read observes the persisted prior-cycle temp.
        #[test]
        fn function_scrambled() {
            insta::assert_snapshot!(transpile_project("blocks/valid/function_scrambled").unwrap(), @r"
        PROGRAM function_scrambled
        VAR
            a : DINT;
            b : DINT;
            early : DINT;
            late : DINT;
        END_VAR
        VAR
            __out_myAdd_1 : return@myAdd;
            __out_myAddDoubled_1 : output@myAdd@myAddDoubled;
        END_VAR
            early := __out_myAdd_1;
            __out_myAdd_1 := myAdd(in1 := a, in2 := b, myAddDoubled => __out_myAddDoubled_1);
            late := __out_myAddDoubled_1;
        END_PROGRAM");
        }

        // A self-cycle: the return feeds its own `in1`, reading last cycle's temp.
        #[test]
        fn function_feedback() {
            insta::assert_snapshot!(transpile_project("blocks/valid/function_feedback").unwrap(), @r"
        PROGRAM function_feedback
        VAR
            seed : DINT;
            acc : DINT;
        END_VAR
        VAR
            __out_myAdd_1 : return@myAdd;
        END_VAR
            __out_myAdd_1 := myAdd(in1 := __out_myAdd_1, in2 := seed, myAddDoubled => );
            acc := __out_myAdd_1;
        END_PROGRAM");
        }

        // Negated input inverts the argument; negated return inverts the read.
        #[test]
        fn function_negated() {
            insta::assert_snapshot!(transpile_project("blocks/valid/function_negated").unwrap(), @r"
        PROGRAM function_negated
        VAR
            a : DINT;
            b : DINT;
            r : DINT;
        END_VAR
        VAR
            __out_myAdd_1 : return@myAdd;
        END_VAR
            __out_myAdd_1 := myAdd(in1 := NOT a, in2 := b, myAddDoubled => );
            r := NOT __out_myAdd_1;
        END_PROGRAM");
        }

        // Nothing consumes it: no temporaries, and the unread output is passed
        // as an empty argument (a function must still receive every parameter).
        #[test]
        fn function_bare() {
            insta::assert_snapshot!(transpile_project("blocks/valid/function_bare").unwrap(), @r"
        PROGRAM function_bare
        VAR
            a : DINT;
            b : DINT;
        END_VAR
            myAdd(in1 := a, in2 := b, myAddDoubled => );
        END_PROGRAM");
        }

        // A wired in_out is passed by reference like an input (`acc := total`).
        #[test]
        fn function_inout() {
            insta::assert_snapshot!(transpile_project("blocks/valid/function_inout").unwrap(), @r"
        PROGRAM function_inout
        VAR
            total : DINT;
            result : DINT;
        END_VAR
        VAR
            __out_addInto_1 : return@addInto;
        END_VAR
            __out_addInto_1 := addInto(delta := 5, acc := total);
            result := __out_addInto_1;
        END_PROGRAM");
        }

        // An unwired in_out is still emitted (every parameter is supplied), as an
        // empty argument. Transpile-only: the main pipeline rejects it (E031),
        // since an in_out must be a reference — we emit faithfully, as with a
        // negated in_out, rather than pre-validate.
        #[test]
        fn function_inout_unwired() {
            insta::assert_snapshot!(transpile_project("blocks/valid/function_inout_unwired").unwrap(), @r"
        PROGRAM function_inout_unwired
        VAR
            result : DINT;
        END_VAR
        VAR
            __out_addInto_1 : return@addInto;
        END_VAR
            __out_addInto_1 := addInto(delta := 5, acc := );
            result := __out_addInto_1;
        END_PROGRAM");
        }
    }
}
