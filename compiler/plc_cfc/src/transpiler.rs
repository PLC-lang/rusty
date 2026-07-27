use plc_ast::ast::{AstFactory, AstNode, CompilationUnit, DataTypeDeclaration, Variable, VariableBlock};
use plc_ast::provider::IdProvider;
use plc_source::source_location::SourceLocation;

use crate::network::{Argument, Network, Statement};
use crate::st;

pub struct Transpiler {
    ids: IdProvider,
}

impl Transpiler {
    pub fn new(ids: IdProvider) -> Self {
        Self { ids }
    }

    pub fn transpile(mut self, mut unit: CompilationUnit, network: Network) -> CompilationUnit {
        // Render each statement into the interface-only unit's empty body.
        let mut statements = Vec::new();
        for statement in network.statements {
            statements.push(self.render(statement));
        }

        if let Some(implementation) = unit.implementations.first_mut() {
            implementation.statements = statements;
        }

        // Declare the captured function outputs in an own VAR block.
        let mut temporaries = Vec::new();
        for temporary in network.temporaries {
            let data_type = DataTypeDeclaration::reference(temporary.data_type, temporary.location.clone());
            temporaries.push(Variable::new(temporary.name, data_type, temporary.location));
        }

        if let (Some(pou), false) = (unit.pous.first_mut(), temporaries.is_empty()) {
            pou.variable_blocks.push(VariableBlock::default().with_variables(temporaries));
        }

        unit
    }

    fn render(&mut self, statement: Statement) -> AstNode {
        match statement {
            Statement::Assignment { sink, source } => {
                AstFactory::create_assignment(sink, source, self.ids.next_id())
            }
            Statement::Return { condition, location } => {
                AstFactory::create_return_statement(Some(condition), location, self.ids.next_id())
            }
            Statement::Jump { condition, target, location } => {
                // No wired condition lowers to a guard the jump can never pass.
                let condition = condition.unwrap_or_else(|| self.reference("FALSE", &location));
                let target = self.reference(&target, &location);

                AstFactory::create_jump_statement(
                    Box::new(condition),
                    Box::new(target),
                    location,
                    self.ids.next_id(),
                )
            }
            Statement::Label { name, location } => {
                AstFactory::create_label_statement(name, location, self.ids.next_id())
            }
            Statement::Call { target, arguments, capture, location } => {
                let operator = self.reference(&target, &location);

                let mut parameters = Vec::new();
                for argument in arguments {
                    parameters.push(self.argument(argument, &location));
                }

                // A bare call carries no parameter list.
                let parameters = if parameters.is_empty() {
                    None
                } else {
                    Some(AstFactory::create_expression_list(parameters, location.clone(), self.ids.next_id()))
                };

                let call = AstFactory::create_call_statement(
                    operator,
                    parameters,
                    self.ids.next_id(),
                    location.clone(),
                );

                // A captured call assigns the return value to its temporary.
                match capture {
                    Some(temporary) => {
                        let temporary = self.reference(&temporary, &location);
                        AstFactory::create_assignment(temporary, call, self.ids.next_id())
                    }
                    None => call,
                }
            }
        }
    }

    fn argument(&mut self, argument: Argument, location: &SourceLocation) -> AstNode {
        match argument {
            // `parameter := value`
            Argument::Input { parameter, value } => {
                let parameter = self.reference(&parameter, location);
                AstFactory::create_assignment(parameter, *value, self.ids.next_id())
            }

            // `parameter => capture`, or an empty `parameter => ` discard.
            Argument::Output { parameter, capture } => {
                let parameter = self.reference(&parameter, location);
                let capture = match capture {
                    Some(temporary) => self.reference(&temporary, location),
                    None => AstFactory::create_empty_statement(location.clone(), self.ids.next_id()),
                };

                AstFactory::create_output_assignment(parameter, capture, self.ids.next_id())
            }
        }
    }

    fn reference(&mut self, name: &str, location: &SourceLocation) -> AstNode {
        let mut node = st::parse_expression(name, self.ids.clone());
        node.location = location.clone();
        node
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

    mod jumps {
        use crate::test_utils::transpile_project;

        #[test]
        fn conditional_jump() {
            insta::assert_snapshot!(transpile_project("jumps/valid/conditional_jump").unwrap(), @r"
        PROGRAM conditional_jump
        VAR
            x : DINT;
            y : DINT;
            myCondition : BOOL;
        END_VAR
            IF myCondition THEN GOTO skipAssignment;
            y := x;
            LABEL: skipAssignment
        END_PROGRAM");
        }

        #[test]
        fn negated_jump() {
            insta::assert_snapshot!(transpile_project("jumps/valid/negated_jump").unwrap(), @r"
        PROGRAM negated_jump
        VAR
            x : DINT;
            y : DINT;
            myCondition : BOOL;
        END_VAR
            IF NOT myCondition THEN GOTO skipAssignment;
            y := x;
            LABEL: skipAssignment
        END_PROGRAM");
        }

        #[test]
        fn disconnected_jump() {
            insta::assert_snapshot!(transpile_project("jumps/valid/disconnected_jump").unwrap(), @r"
        PROGRAM disconnected_jump
        VAR
            x : DINT;
            y : DINT;
            myCondition : BOOL;
        END_VAR
            IF FALSE THEN GOTO skipAssignment;
            y := x;
            LABEL: skipAssignment
        END_PROGRAM");
        }

        #[test]
        fn scrambled() {
            insta::assert_snapshot!(transpile_project("jumps/valid/scrambled").unwrap(), @r"
        PROGRAM scrambled
        VAR
            g1 : BOOL;
            g2 : BOOL;
            g3 : BOOL;
            x : DINT;
            a : DINT;
            b : DINT;
            c : DINT;
        END_VAR
            IF g1 THEN GOTO mid;
            a := x;
            IF g2 THEN GOTO end;
            LABEL: mid
            b := x;
            IF g3 THEN GOTO end;
            c := x;
            LABEL: end
        END_PROGRAM");
        }

        #[test]
        fn backward_jump() {
            insta::assert_snapshot!(transpile_project("jumps/valid/backward_jump").unwrap(), @r"
        PROGRAM backward_jump
        VAR
            cond : BOOL;
            i : DINT;
            x : DINT;
        END_VAR
            LABEL: top
            x := i;
            IF cond THEN GOTO top;
        END_PROGRAM");
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

    mod functions {
        use crate::test_utils::transpile_project;

        #[test]
        fn function_void() {
            insta::assert_snapshot!(transpile_project("blocks/valid/function_void").unwrap(), @r"
        PROGRAM function_void
        VAR
            localIn : DINT := 3;
            localOut : DINT;
        END_VAR
        VAR
            __out_out_3 : DINT;
        END_VAR
            myVoid(in := localIn, out => __out_out_3);
            localOut := __out_out_3;
        END_PROGRAM");
        }

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
            __out_myAdd_1 : DINT;
            __out_myAddDoubled_1 : DINT;
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
            __out_myAdd_1 : DINT;
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
            __out_myAdd_1 : DINT;
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
            __out_myAdd_1 : DINT;
            __out_myAdd_10 : DINT;
        END_VAR
            __out_myAdd_1 := myAdd(in1 := seed, in2 := k, myAddDoubled => );
            __out_myAdd_10 := myAdd(in1 := __out_myAdd_1, in2 := k, myAddDoubled => );
            result := __out_myAdd_10;
        END_PROGRAM");
        }

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
            __out_myAdd_1 : DINT;
            __out_myAddDoubled_1 : DINT;
        END_VAR
            early := __out_myAdd_1;
            __out_myAdd_1 := myAdd(in1 := a, in2 := b, myAddDoubled => __out_myAddDoubled_1);
            late := __out_myAddDoubled_1;
        END_PROGRAM");
        }

        #[test]
        fn function_feedback() {
            insta::assert_snapshot!(transpile_project("blocks/valid/function_feedback").unwrap(), @r"
        PROGRAM function_feedback
        VAR
            seed : DINT;
            acc : DINT;
        END_VAR
        VAR
            __out_myAdd_1 : DINT;
        END_VAR
            __out_myAdd_1 := myAdd(in1 := __out_myAdd_1, in2 := seed, myAddDoubled => );
            acc := __out_myAdd_1;
        END_PROGRAM");
        }

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
            __out_myAdd_1 : DINT;
        END_VAR
            __out_myAdd_1 := myAdd(in1 := NOT a, in2 := b, myAddDoubled => );
            r := NOT __out_myAdd_1;
        END_PROGRAM");
        }

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

        #[test]
        fn function_inout() {
            insta::assert_snapshot!(transpile_project("blocks/valid/function_inout").unwrap(), @r"
        PROGRAM function_inout
        VAR
            total : DINT;
            result : DINT;
        END_VAR
        VAR
            __out_addInto_1 : DINT;
        END_VAR
            __out_addInto_1 := addInto(delta := 5, acc := total);
            result := __out_addInto_1;
        END_PROGRAM");
        }

        #[test]
        fn function_inout_unwired() {
            insta::assert_snapshot!(transpile_project("blocks/valid/function_inout_unwired").unwrap(), @r"
        PROGRAM function_inout_unwired
        VAR
            result : DINT;
        END_VAR
        VAR
            __out_addInto_1 : DINT;
        END_VAR
            __out_addInto_1 := addInto(delta := 5, acc := );
            result := __out_addInto_1;
        END_PROGRAM");
        }
    }
}
