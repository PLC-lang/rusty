use indexmap::IndexMap;

use ast::ast::{
    AstFactory, AstId, AstNode, CompilationUnit, DataTypeDeclaration, LinkageType, Variable, VariableBlock,
};
use ast::provider::IdProvider;
use plc::{lexer, parser};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::{SourceLocation, SourceLocationFactory};

use crate::{
    model::{Block, DataSink, FbdObject, Pou, Return},
    placeholder,
    resolver::{Object, Resolver},
};

pub struct Transpiler {
    pou: Pou,
    resolver: Resolver,

    /// Temporary variables introduced for block outputs, keyed by the wire id that produces them.
    /// Keyed for lookup during resolution; insertion order is preserved for the emitted `VAR_TEMP`.
    temps: IndexMap<u64, Variable>,

    id_provider: IdProvider,
    range_factory: SourceLocationFactory,
}

enum Operation {
    Call(Block),
    Sink(DataSink),
    Return(Return),
}

impl Transpiler {
    pub fn new(pou: Pou, id_provider: IdProvider, range_factory: SourceLocationFactory) -> Transpiler {
        let resolver = Resolver::index(&pou);

        Transpiler { pou, resolver, temps: IndexMap::new(), id_provider, range_factory }
    }

    pub fn transpile(mut self) -> Result<CompilationUnit, Diagnostic> {
        // Parse the declaration first
        // TODO: Use the diagnostics
        let (mut unit, _diagnostics) = {
            let declaration = {
                let content = self.pou.text_declaration().expect("todo error handling");
                let content_end_kw = match &self.pou {
                    Pou::Program(_) => "END_PROGRAM",
                    Pou::FunctionBlock(_) => "END_FUNCTION_BLOCK",
                    Pou::Function(_) => "END_FUNCTION",
                };

                format!("{content}\n{content_end_kw}")
            };

            plc::parser::parse(
                lexer::lex_with_ids(&declaration, self.id_provider.clone(), self.range_factory.clone()),
                LinkageType::Internal,
                self.range_factory.get_file_name().unwrap_or_default(),
            )
        };

        // Then the actual graphical units
        unit.implementations[0].statements = self.transpile_network();

        // Inject the generated temporary variables, if any
        if !self.temps.is_empty() {
            unit.pous[0].variable_blocks.push(VariableBlock::temp(self.temps.into_values().collect()));
        }

        Ok(unit)
    }

    fn transpile_network(&mut self) -> Vec<AstNode> {
        let mut operations = Vec::new();

        // Aggregate all objects
        for object in self.pou.get_network().expect("todo: error handling").objects.iter() {
            match object {
                FbdObject::Block(block) => operations.push(Operation::Call(block.clone())),
                FbdObject::DataSink(sink) => operations.push(Operation::Sink(sink.clone())),
                FbdObject::Return(ret) => operations.push(Operation::Return(ret.clone())),
                _ => (),
            }
        }

        // Sort objects by their evaluation priorty such that we can transpile correct ST code
        operations.sort_by_key(Operation::priority);

        // Then do actual transpiling
        let mut statements = Vec::new();
        for operation in &operations {
            let statement = match operation {
                Operation::Call(block) => self.transpile_call(block),
                Operation::Sink(sink) => self.transpile_sink(sink),
                Operation::Return(ret) => self.transpile_return(ret),
            };

            statements.push(statement);
        }

        statements
    }

    /// Transpiles a block into a function call
    fn transpile_call(&mut self, block: &Block) -> AstNode {
        let location = self.create_object_location(block.global_id, block.get_priority());

        let mut arguments = Vec::new();

        // Input and InOut variables; `<parameter> := <argument>` (or `<parameter> := ` when unconnected)
        for parameter in block.get_input_variables() {
            let id = parameter.get_referenced_argument_id();
            arguments.push(self.create_argument(&parameter.parameter_name, id, parameter.negated, &location));
        }

        for parameter in block.get_inout_variables() {
            let id = parameter.get_referenced_argument_id();
            arguments.push(self.create_argument(&parameter.parameter_name, id, parameter.negated, &location));
        }

        // Output variables; similar to input and inout variables with the exception of also introducing
        // temporary variables.
        let mut return_value = None;
        for output in block.get_output_variables() {
            let id = output.connection_point_out.as_ref().expect("todo: error handling").id;

            // We're dealing with a function if both the output variable and block name are identical in which
            // case we want to persist the return value into a temporary variable.
            if output.parameter_name == block.type_name {
                if self.resolver.is_consumed(id) {
                    let temp = self.create_temp(id, placeholder::return_placeholder(&block.type_name));
                    return_value = Some(temp);
                }

                continue;
            }

            // Ordinary output variable, either `<parameter> => __temp...` or `<parameter> =>` if empty
            let temp = if self.resolver.is_consumed(id) {
                let name = placeholder::output_placeholder(&block.type_name, &output.parameter_name);
                Some(self.create_temp(id, name))
            } else {
                None
            };

            arguments.push(self.create_output_argument(&output.parameter_name, temp.as_deref(), &location));
        }

        let operator = self.create_call_operator(block, &location);
        let call = self.create_call_statement(operator, arguments, &location);

        // Assign the return value of a **function** to a temporary variable, otherwise return the call as is.
        match return_value {
            // `__temp... := <function call>`
            Some(temp) => {
                let target = self.create_member_reference(&temp, &location);
                AstFactory::create_assignment(target, call, self.next_id())
            }

            // <function call>
            None => call,
        }
    }

    /// Transpile a (connected) data sink into an assignment
    fn transpile_sink(&mut self, sink: &DataSink) -> AstNode {
        let location = self.create_object_location(sink.global_id, sink.get_priority());
        let value = self.resolve(sink.get_referenced_argument_id().unwrap(), &location);
        let target = self.create_member_reference(&sink.identifier, &location);

        AstFactory::create_assignment(target, value, self.next_id())
    }

    /// Transpiles a conditional return so a return (duh)
    fn transpile_return(&mut self, ret: &Return) -> AstNode {
        let location = self.create_object_location(ret.global_id, ret.get_priority());

        let condition = match ret.get_condition_id() {
            Some(id) => {
                let mut node = self.resolve(id, &location);
                if ret.is_negated() {
                    node = node.negate(self.id_provider.clone());
                }

                Some(node)
            }

            None => None,
        };

        AstFactory::create_return_statement(condition, location, self.next_id())
    }

    /// Returns an AST node associated with the given ID, if any
    fn resolve(&mut self, id: u64, location: &SourceLocation) -> AstNode {
        let id = self.resolver.resolve_alias(id);

        if let Some(temp) = self.temps.get(&id) {
            let name = temp.name.clone();
            return self.create_member_reference(&name, location);
        }

        match self.resolver.get(id) {
            Some(Object::Variable(source)) => self.parse_expression(&source.identifier),
            Some(Object::BlockOutput(..)) => {
                unreachable!("a block output must be evaluated into a temp before it is consumed")
            }
            None => panic!("queried a reference that does not exist"),
        }
    }
}

// Helper Function
impl Transpiler {
    fn next_id(&mut self) -> AstId {
        self.id_provider.next_id()
    }

    fn parse_expression(&self, source: &str) -> AstNode {
        parser::expressions_parser::parse_expression(&mut lexer::lex_with_ids(
            source,
            self.id_provider.clone(),
            self.range_factory.clone(),
        ))
    }

    fn create_temp(&mut self, id: u64, placeholder: String) -> String {
        let name = format!("__temp_{}", self.temps.len());
        self.temps.insert(id, create_internal_variable(&name, placeholder));

        name
    }

    /// An empty statement, used as the value of an unconnected parameter (`<parameter> :=`/`=>`).
    fn create_empty(&mut self, location: &SourceLocation) -> AstNode {
        AstFactory::create_empty_statement(location.clone(), self.next_id())
    }

    /// `<name>`, or `<base>.<name>` when a base is given.
    fn create_reference(&mut self, name: &str, base: Option<AstNode>, location: &SourceLocation) -> AstNode {
        let identifier = AstFactory::create_identifier(name, location.clone(), self.next_id());
        AstFactory::create_member_reference(identifier, base, self.next_id())
    }

    fn create_member_reference(&mut self, name: &str, location: &SourceLocation) -> AstNode {
        self.create_reference(name, None, location)
    }

    fn create_call_operator(&mut self, block: &Block, location: &SourceLocation) -> AstNode {
        let Some(instance) = &block.instance_name else {
            return self.create_member_reference(&block.type_name, location);
        };

        // Action call: the type name carries an `<fb>.<action>` qualifier, lowered to `<instance>.<action>`
        match block.type_name.rsplit_once('.') {
            Some((_, action)) => {
                let base = self.create_member_reference(instance, location);
                self.create_reference(action, Some(base), location)
            }
            None => self.create_member_reference(instance, location),
        }
    }

    fn create_object_location(&self, global_id: Option<u64>, priority: Option<u64>) -> SourceLocation {
        self.range_factory
            .create_block_location(global_id.unwrap_or_default() as usize, priority.map(|p| p as usize))
    }

    /// `<parameter> := <argument>`, or `<parameter> :=` when the parameter is unconnected.
    fn create_argument(
        &mut self,
        parameter_name: &str,
        argument_id: Option<u64>,
        negated: bool,
        location: &SourceLocation,
    ) -> AstNode {
        let target = self.create_member_reference(parameter_name, location);
        let value = match argument_id {
            Some(id) => {
                let value = self.resolve(id, location);
                if negated { value.negate(self.id_provider.clone()) } else { value }
            }
            None => self.create_empty(location),
        };

        AstFactory::create_assignment(target, value, self.next_id())
    }

    /// `<parameter> => <target>`, or `<parameter> =>` when the output is unconnected.
    fn create_output_argument(
        &mut self,
        parameter_name: &str,
        target: Option<&str>,
        location: &SourceLocation,
    ) -> AstNode {
        let parameter = self.create_member_reference(parameter_name, location);
        let value = match target {
            Some(target) => self.create_member_reference(target, location),
            None => self.create_empty(location),
        };

        AstFactory::create_output_assignment(parameter, value, self.next_id())
    }

    fn create_call_statement(
        &mut self,
        operator: AstNode,
        arguments: Vec<AstNode>,
        location: &SourceLocation,
    ) -> AstNode {
        let arguments = match arguments.len() {
            0 => None,
            1 => arguments.into_iter().next(),
            _ => Some(AstFactory::create_expression_list(arguments, location.clone(), self.next_id())),
        };

        AstFactory::create_call_statement(operator, arguments, self.next_id(), location.clone())
    }
}

fn create_internal_variable(name: &str, placeholder: String) -> Variable {
    Variable {
        name: name.to_string(),
        data_type_declaration: DataTypeDeclaration::Reference {
            referenced_type: placeholder,
            location: SourceLocation::undefined(),
        },
        initializer: None,
        address: None,
        location: SourceLocation::undefined(),
    }
}

impl Operation {
    fn priority(&self) -> Option<u64> {
        match self {
            Operation::Call(block) => block.get_priority(),
            Operation::Sink(sink) => sink.get_priority(),
            Operation::Return(ret) => ret.get_priority(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Transpiler;
    use crate::model;
    use ast::provider::IdProvider;
    use ast::ser::AstSerializer;
    use insta::assert_snapshot;
    use plc_source::source_location::SourceLocationFactory;

    fn transpile(xml: &str) -> String {
        let pou = model::from_str(xml).unwrap();
        let unit = Transpiler::new(pou, IdProvider::default(), SourceLocationFactory::internal(xml))
            .transpile()
            .expect("todo error handling");

        AstSerializer::format_unit(&unit)
    }

    #[test]
    fn function_call() {
        let xml = include_str!("../fixtures/function_call/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localB : DINT;
            localResult : DINT;
        END_VAR
        VAR_TEMP
            __temp_0 : __return@myAdd;
        END_VAR
            __temp_0 := myAdd(in1 := localA, in2 := localB);
            localResult := __temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn shared_result() {
        let xml = include_str!("../fixtures/shared_result/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localB : DINT;
            localResultA : DINT;
            localResultB : DINT;
        END_VAR
        VAR_TEMP
            __temp_0 : __return@myAdd;
        END_VAR
            __temp_0 := myAdd(in1 := localA, in2 := localB);
            localResultA := __temp_0;
            localResultB := __temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn chained_calls() {
        let xml = include_str!("../fixtures/chained_calls/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localB : DINT;
            localResultA : DINT;
        END_VAR
        VAR_TEMP
            __temp_0 : __return@myAdd;
            __temp_1 : __return@myMul;
        END_VAR
            __temp_0 := myAdd(in1 := localA, in2 := localB);
            __temp_1 := myMul(IN1 := __temp_0, IN2 := localB);
            localResultA := __temp_1;
        END_PROGRAM
        ");
    }

    #[test]
    fn nullary_call() {
        let xml = include_str!("../fixtures/nullary_call/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localResult : DINT;
        END_VAR
        VAR_TEMP
            __temp_0 : __return@getOffset;
        END_VAR
            __temp_0 := getOffset();
            localResult := __temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn evaluation_order() {
        let xml = include_str!("../fixtures/evaluation_order/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localB : DINT;
            localC : DINT;
            localD : DINT;
            resultMul : DINT;
            resultAdd : DINT;
        END_VAR
        VAR_TEMP
            __temp_0 : __return@myMul;
            __temp_1 : __return@myAdd;
        END_VAR
            __temp_0 := myMul(in1 := localA, in2 := localB);
            resultMul := __temp_0;
            __temp_1 := myAdd(in1 := localC, in2 := localD);
            resultAdd := __temp_1;
        END_PROGRAM
        ");
    }

    #[test]
    fn negated_input() {
        let xml = include_str!("../fixtures/negated_input/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : BOOL;
            localB : BOOL;
            localResult : BOOL;
        END_VAR
        VAR_TEMP
            __temp_0 : __return@myGate;
        END_VAR
            __temp_0 := myGate(a := NOT localA, b := localB);
            localResult := __temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn inout_variable() {
        let xml = include_str!("../fixtures/inout_variable/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localValue : DINT;
            localSum : DINT;
            localResult : DINT;
        END_VAR
        VAR_TEMP
            __temp_0 : __return@accumulate;
        END_VAR
            __temp_0 := accumulate(value := localValue, sum := localSum);
            localResult := __temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn literal_input() {
        let xml = include_str!("../fixtures/literal_input/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localResult : DINT;
        END_VAR
        VAR_TEMP
            __temp_0 : __return@myAdd;
        END_VAR
            __temp_0 := myAdd(in1 := localA, in2 := 100);
            localResult := __temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn expression_source() {
        let xml = include_str!("../fixtures/expression_source/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            result : DINT;
        END_VAR
            result := localA + 5;
        END_PROGRAM
        ");
    }

    #[test]
    fn function_pou() {
        let xml = include_str!("../fixtures/function_pou/myFunc.cfc");
        assert_snapshot!(transpile(xml), @r"
        FUNCTION myFunc : DINT
        VAR_INPUT
            a : DINT;
            b : DINT;
        END_VAR
        VAR_TEMP
            __temp_0 : __return@myAdd;
        END_VAR
            __temp_0 := myAdd(in1 := a, in2 := b);
            myFunc := __temp_0;
        END_FUNCTION
        ");
    }

    #[test]
    fn function_block_pou() {
        let xml = include_str!("../fixtures/function_block_pou/myFb.cfc");
        assert_snapshot!(transpile(xml), @r"
        FUNCTION_BLOCK myFb
        VAR_INPUT
            a : DINT;
            b : DINT;
        END_VAR
        VAR_OUTPUT
            sum : DINT;
        END_VAR
        VAR_TEMP
            __temp_0 : __return@myAdd;
        END_VAR
            __temp_0 := myAdd(in1 := a, in2 := b);
            sum := __temp_0;
        END_FUNCTION_BLOCK
        ");
    }

    #[test]
    fn function_block_call() {
        let xml = include_str!("../fixtures/function_block_call/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            myInstance : Counter;
            localStep : DINT;
            localCount : DINT;
        END_VAR
        VAR_TEMP
            __temp_0 : __output@Counter@count;
        END_VAR
            myInstance(step := localStep, count => __temp_0);
            localCount := __temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn action_call() {
        let xml = include_str!("../fixtures/action_call/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            myInstance : function_block_0;
        END_VAR
            myInstance.myAction();
        END_PROGRAM
        ");
    }

    #[test]
    fn program_call() {
        let xml = include_str!("../fixtures/program_call/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localIncrement : DINT;
            localTotal : DINT;
        END_VAR
        VAR_TEMP
            __temp_0 : __output@auxProgram@total;
        END_VAR
            auxProgram(increment := localIncrement, total => __temp_0);
            localTotal := __temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn unconnected_arguments_function() {
        let xml = include_str!("../fixtures/unconnected_arguments_function/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localResult : DINT;
        END_VAR
        VAR_TEMP
            __temp_0 : __return@myFunc;
        END_VAR
            __temp_0 := myFunc(a := localA, b := , io := );
            localResult := __temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn unconnected_arguments_program() {
        let xml = include_str!("../fixtures/unconnected_arguments_program/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : DINT;
        END_VAR
            auxProgram(a := localA, b := , io := );
        END_PROGRAM
        ");
    }

    #[test]
    fn unconnected_arguments_function_block() {
        let xml = include_str!("../fixtures/unconnected_arguments_function_block/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            myInstance : myFb;
            localA : DINT;
        END_VAR
            myInstance(a := localA, b := , io := );
        END_PROGRAM
        ");
    }

    #[test]
    fn unconnected_output_function() {
        let xml = include_str!("../fixtures/unconnected_output_function/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localResult : DINT;
        END_VAR
        VAR_TEMP
            __temp_0 : __return@myFunc;
        END_VAR
            __temp_0 := myFunc(a := localA, extra => );
            localResult := __temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn unconnected_output_program() {
        let xml = include_str!("../fixtures/unconnected_output_program/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : DINT;
        END_VAR
            auxProgram(a := localA, result => );
        END_PROGRAM
        ");
    }

    #[test]
    fn unconnected_output_function_block() {
        let xml = include_str!("../fixtures/unconnected_output_function_block/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            myInstance : myFb;
            localA : DINT;
        END_VAR
            myInstance(a := localA, result => );
        END_PROGRAM
        ");
    }

    #[test]
    fn multiple_outputs() {
        let xml = include_str!("../fixtures/multiple_outputs/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            myInstance : myFunctionBlock;
            localA : DINT;
            localB : DINT;
        END_VAR
        VAR_TEMP
            __temp_0 : __output@myFunctionBlock@a;
            __temp_1 : __output@myFunctionBlock@c;
            __temp_2 : __output@myFunction@a;
        END_VAR
            myInstance(a => __temp_0, b => , c => __temp_1);
            localA := __temp_0;
            localB := __temp_1;
            myFunction(a => __temp_2, b => );
            localA := __temp_2;
        END_PROGRAM
        ");
    }

    #[test]
    fn conditional_return() {
        let xml = include_str!("../fixtures/conditional_return/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            enable : BOOL;
            input : DINT;
            result : DINT;
        END_VAR
            IF NOT enable THEN RETURN; END_IF;
            result := input;
        END_PROGRAM
        ");
    }

    #[test]
    fn unconditional_return() {
        let xml = include_str!("../fixtures/unconditional_return/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            input : DINT;
            result : DINT;
        END_VAR
            result := input;
            RETURN;
        END_PROGRAM
        ");
    }

    #[test]
    fn connector_continuation() {
        let xml = include_str!("../fixtures/connector_continuation/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            result : DINT;
        END_VAR
        VAR_TEMP
            __temp_0 : __return@alwaysFive;
        END_VAR
            __temp_0 := alwaysFive();
            result := __temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn connector_continuation_chain() {
        let xml = include_str!("../fixtures/connector_continuation_chain/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            result : DINT;
        END_VAR
        VAR_TEMP
            __temp_0 : __return@alwaysFive;
        END_VAR
            __temp_0 := alwaysFive();
            result := __temp_0;
        END_PROGRAM
        ");
    }
}
