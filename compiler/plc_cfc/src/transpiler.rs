use std::collections::HashMap;

use ast::ast::{AstFactory, AstNode, CompilationUnit, LinkageType};
use ast::provider::IdProvider;
use plc::{lexer, parser};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::{SourceLocation, SourceLocationFactory};

use crate::{
    model::{Block, DataSink, FbdObject, Pou, Return},
    resolver::{Object, Resolver},
};

pub struct Transpiler {
    pou: Pou,
    statements: Vec<AstNode>,
    resolver: Resolver,

    /// Block outputs that have already been evaluated into a `VAR_TEMP` variable,
    /// keyed by the producing `connectionPointOutId`.
    ///
    /// A block in CFC is executed exactly once. When its output feeds several
    /// consumers we evaluate it once into a temp and let the consumers reference
    /// that temp, i.e.
    /// ```text
    /// temp_0 := producer(...);
    /// consumer_a(in := temp_0, ...);
    /// consumer_b(in := temp_0, ...);
    /// ```
    /// rather than inlining `producer(...)` into each consumer (which would call
    /// it once per consumer). The values here are exactly the temps that need a
    /// `VAR_TEMP` declaration; data-source references (`localA`, `foo + 5`) are
    /// resolved on demand and deliberately not stored here.
    temps: HashMap<u64, String>,

    /// Shared with the rest of the compilation unit so synthesized nodes get
    /// globally-unique IDs; the factory ties their locations back to the source.
    id_provider: IdProvider,
    range_factory: SourceLocationFactory,

    /// Running index for the `temp_N` variables introduced for block outputs.
    temp_counter: usize,
}

/// A statement-producing network element, ordered by evaluation priority.
enum Operation {
    Call(Block),
    Sink(DataSink),
    Return(Return),
}

impl Transpiler {
    pub fn new(pou: Pou, id_provider: IdProvider, range_factory: SourceLocationFactory) -> Transpiler {
        let resolver = Resolver::index(&pou);
        Transpiler {
            pou,
            statements: Vec::new(),
            resolver,
            temps: HashMap::new(),
            id_provider,
            range_factory,
            temp_counter: 0,
        }
    }

    pub fn transpile(mut self) -> Result<CompilationUnit, Diagnostic> {
        let mut unit = self.parse_declaration();
        self.transpile_network();
        unit.implementations[0].statements = self.statements;

        Ok(unit)
    }

    /// Transpiles the network's statement-producing elements in evaluation order, so a
    /// producer's temp is created before its consumers resolve it. Blocks become call
    /// statements and data sinks become assignments; both carry a `priorityInNetwork`.
    fn transpile_network(&mut self) {
        // Collect the operations as owned values so the loop below doesn't keep `self.pou`
        // borrowed while it needs `&mut self`. Data sources are resolved on demand; the
        // remaining object kinds aren't supported yet.
        let mut operations = Vec::new();
        for object in self.pou.get_network().expect("todo: error handling").objects.iter() {
            match object {
                FbdObject::Block(block) => operations.push(Operation::Call(block.clone())),
                FbdObject::DataSink(sink) => operations.push(Operation::Sink(sink.clone())),
                FbdObject::Return(ret) => operations.push(Operation::Return(ret.clone())),
                _ => (),
            }
        }

        operations.sort_by_key(Operation::priority);

        // TODO: Return AstNode in transpile_* calls below instead of pushing directly into self.statements? Might not even need statements as a member field.
        for operation in &operations {
            match operation {
                Operation::Call(block) => self.transpile_call(block),
                Operation::Sink(sink) => self.transpile_sink(sink),
                Operation::Return(ret) => self.transpile_return(ret),
            }
        }
    }

    fn transpile_call(&mut self, block: &Block) {
        let location = self.object_location(block.global_id, block.get_priority());

        // Inputs and in-outs feed a value in through a ConnectionPointIn: `param := source`. An
        // unconnected pin is still emitted, with an empty value (`param := `), so the call keeps its
        // full named-argument list: an input then falls back to the callee's declared default, and
        // an unconnected in-out is left for downstream validation to reject (`E031`).
        let mut arguments = Vec::new();

        // Input Variables
        for parameter in block.get_input_variables() {
            arguments.push(match parameter.get_referenced_argument_id() {
                Some(id) => self.argument(&parameter.parameter_name, id, parameter.negated, &location),
                None => self.empty_argument(&parameter.parameter_name, &location),
            });
        }

        // In-Out Variables. (A negated in-out would emit `param := NOT value`, nonsensical for
        // a by-reference pin; no fixture exercises it — revisit if one ever does.)
        for parameter in block.get_inout_variables() {
            arguments.push(match parameter.get_referenced_argument_id() {
                Some(id) => self.argument(&parameter.parameter_name, id, parameter.negated, &location),
                None => self.empty_argument(&parameter.parameter_name, &location),
            });
        }

        // Output Variables
        let mut return_value = None;
        for output in block.get_output_variables() {
            let id = output.connection_point_out.as_ref().expect("todo: error handling").id;

            // The return value is part of the output variable list, but it is not a conventional
            // `... => ...` output. It is captured in a temp — so block-to-block / fan-out consumers
            // resolve to it — only when something reads it; an unconsumed result is discarded and the
            // call stands alone (a function may be called purely for its VAR_OUTPUT pins).
            if output.parameter_name == block.type_name {
                if self.resolver.is_consumed(id) {
                    return_value = Some(self.create_temp(id));
                }
                continue;
            }

            // A named output (`param => ...`). When its value feeds something, it is evaluated once
            // into a temp so fan-out consumers and downstream blocks resolve to it uniformly (the
            // sinks reading it become `<sink> := temp` statements, see `transpile_sink`). When it
            // feeds nothing, it is emitted with an empty value (`param => `) instead — mirroring an
            // unconnected input, with no temp left for no one to read.
            arguments.push(if self.resolver.is_consumed(id) {
                let temp = self.create_temp(id);
                self.output_argument(&output.parameter_name, &temp, &location)
            } else {
                self.empty_output_argument(&output.parameter_name, &location)
            });
        }

        // An FB block is called on its instance; a function or program is called by its name (the
        // type name). Only FB blocks carry an `instanceName`.
        let operator = block.instance_name.as_deref().unwrap_or(block.type_name.as_str());
        let call = self.call_statement(operator, arguments, &location);

        // A return value turns the call into `temp := fn(...)`; otherwise it stands alone.
        let statement = match return_value {
            Some(temp) => {
                let target = self.reference(&temp, &location);
                AstFactory::create_assignment(target, call, self.id_provider.next_id())
            }
            None => call,
        };

        self.statements.push(statement);
    }

    /// Transpiles a data sink into a `sink := <wired value>` assignment.
    fn transpile_sink(&mut self, sink: &DataSink) {
        let location = self.object_location(sink.global_id, sink.get_priority());

        // A sink consumes exactly one wire; resolve it to the value that wire carries.
        let pin = sink.connection_point_in.as_ref().expect("todo: error handling");
        let value = self.resolve(pin.connections[0].ref_connection_point_out_id, &location);

        let target = self.reference(&sink.identifier, &location);
        self.statements.push(AstFactory::create_assignment(target, value, self.id_provider.next_id()));
    }

    /// Transpiles a return into a `RETURN` statement. Its optional input wire carries a boolean
    /// condition: wired means a conditional return (return when the condition holds), unwired means
    /// unconditional. A negated return inverts the condition, so it returns when the wire is false.
    fn transpile_return(&mut self, ret: &Return) {
        let location = self.object_location(ret.global_id, ret.get_priority());

        let condition = match ret.get_condition_id() {
            Some(id) => {
                let mut value = self.resolve(id, &location);
                if ret.is_negated() {
                    value = value.negate(self.id_provider.clone());
                }

                Some(value)
            }

            None => None,
        };

        let statement = AstFactory::create_return_statement(condition, location, self.id_provider.next_id());
        self.statements.push(statement);
    }
}

// TODO: error handling, many unwraps currently
// Transpiler helper functions
impl Transpiler {
    /// Parses the POU's text declaration (its interface) into a compilation unit. Its single
    /// implementation has an empty body, which `transpile` then fills with the transpiled FBD.
    fn parse_declaration(&self) -> CompilationUnit {
        // The text declaration carries the POU header and VAR blocks but no body, so append the
        // closing keyword for its kind to make it a complete POU the ST parser accepts.
        let end_keyword = match &self.pou {
            Pou::Program(_) => "END_PROGRAM",
            Pou::FunctionBlock(_) => "END_FUNCTION_BLOCK",
            Pou::Function(_) => "END_FUNCTION",
        };
        let declaration =
            format!("{}\n{end_keyword}", self.pou.text_declaration().expect("todo error handling"));

        let (unit, _diagnostics) = plc::parser::parse(
            lexer::lex_with_ids(&declaration, self.id_provider.clone(), self.range_factory.clone()),
            LinkageType::Internal,
            self.range_factory.get_file_name().unwrap_or_default(),
        );

        unit
    }

    /// Resolves the value on a wire (its `connectionPointOutId`) to an AST expression.
    fn resolve(&mut self, id: u64, location: &SourceLocation) -> AstNode {
        // Follow connector/continuation aliases so a named virtual wire resolves to its producer.
        let id = self.resolver.resolve_alias(id);

        // A block output already evaluated into a temp: reference that temp.
        if let Some(temp) = self.temps.get(&id).cloned() {
            return self.reference(&temp, location);
        }

        match self.resolver.get(id) {
            // A data source feeds an arbitrary ST expression — usually a plain
            // identifier (`localA`), but possibly something like `foo + 5` — so we
            // parse it as a leaf rather than assume it is a bare reference.
            Some(Object::Variable(source)) => self.parse_expression(&source.identifier),
            // TODO: emit a diagnostic instead of panicking once error handling lands.
            Some(Object::BlockOutput(..)) => {
                unreachable!("a block output must be evaluated into a temp before it is consumed")
            }
            None => panic!("queried a reference that does not exist"),
        }
    }

    /// Parses a leaf data-source expression (e.g. `localA` or `foo + 5`) into an AST node.
    fn parse_expression(&self, source: &str) -> AstNode {
        parser::expressions_parser::parse_expression(&mut lexer::lex_with_ids(
            source,
            self.id_provider.clone(),
            self.range_factory.clone(),
        ))
    }

    /// A fresh `temp_N`, recorded under `id` so consumers of that wire resolve to it.
    fn create_temp(&mut self, id: u64) -> String {
        let temp = format!("temp_{}", self.temp_counter);
        self.temp_counter += 1;
        self.temps.insert(id, temp.clone());
        temp
    }

    /// A member reference to `name` (e.g. a parameter, variable, or temp).
    fn reference(&mut self, name: &str, location: &SourceLocation) -> AstNode {
        let identifier = AstFactory::create_identifier(name, location.clone(), self.id_provider.next_id());

        AstFactory::create_member_reference(identifier, None, self.id_provider.next_id())
    }

    // TODO: Remove the priority from the location altogether once plc_cfc is on par with
    // plc_xml; only the global id is actually used for diagnostics today.
    /// A source location tied back to the originating graphical object (keyed by its
    /// `globalId` and evaluation priority) so later diagnostics can point at the diagram.
    fn object_location(&self, global_id: Option<u64>, priority: Option<u64>) -> SourceLocation {
        self.range_factory
            .create_block_location(global_id.unwrap_or_default() as usize, priority.map(|p| p as usize))
    }

    /// Builds a `param := <resolved argument>` assignment for an input/in-out pin.
    /// A negated pin wraps its incoming value in `NOT`.
    fn argument(
        &mut self,
        parameter_name: &str,
        argument_id: u64,
        negated: bool,
        location: &SourceLocation,
    ) -> AstNode {
        let target = self.reference(parameter_name, location);
        let mut argument = self.resolve(argument_id, location);
        if negated {
            argument = argument.negate(self.id_provider.clone());
        }

        AstFactory::create_assignment(target, argument, self.id_provider.next_id())
    }

    /// Builds a `param := ` assignment with an empty right-hand side, for an unconnected pin.
    /// This is exactly the AST the ST parser produces for `foo(param := )`, so downstream
    /// handling is identical: an input falls back to its declared default, while an unconnected
    /// in-out is rejected by validation (`E031`). Lowering it faithfully keeps validation —
    /// judging whether the empty value is legal — out of this layer.
    fn empty_argument(&mut self, parameter_name: &str, location: &SourceLocation) -> AstNode {
        let target = self.reference(parameter_name, location);
        let empty = AstFactory::create_empty_statement(location.clone(), self.id_provider.next_id());

        AstFactory::create_assignment(target, empty, self.id_provider.next_id())
    }

    /// Builds a `param => target` output assignment for a VAR_OUTPUT pin.
    fn output_argument(&mut self, parameter_name: &str, target: &str, location: &SourceLocation) -> AstNode {
        let parameter = self.reference(parameter_name, location);
        let target = self.reference(target, location);

        AstFactory::create_output_assignment(parameter, target, self.id_provider.next_id())
    }

    /// Builds a `param => ` output assignment with an empty right-hand side, for a VAR_OUTPUT pin
    /// whose value feeds nothing — the output counterpart of [`Transpiler::empty_argument`]. The
    /// output is simply discarded downstream; emitting it keeps the call's full argument list and
    /// avoids a temp no one reads.
    fn empty_output_argument(&mut self, parameter_name: &str, location: &SourceLocation) -> AstNode {
        let parameter = self.reference(parameter_name, location);
        let empty = AstFactory::create_empty_statement(location.clone(), self.id_provider.next_id());

        AstFactory::create_output_assignment(parameter, empty, self.id_provider.next_id())
    }

    /// Builds a `name(<arguments>)` call statement, wrapping the arguments into a
    /// parameter list the way the parser would: `None` for none, a bare node for a
    /// single argument, an expression list for several.
    fn call_statement(&mut self, name: &str, arguments: Vec<AstNode>, location: &SourceLocation) -> AstNode {
        let operator = self.reference(name, location);
        let arguments = match arguments.len() {
            0 => None,
            1 => arguments.into_iter().next(),
            _ => Some(AstFactory::create_expression_list(
                arguments,
                location.clone(),
                self.id_provider.next_id(),
            )),
        };

        AstFactory::create_call_statement(operator, arguments, self.id_provider.next_id(), location.clone())
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

    /// Transpiles a fixture into a compilation unit and renders it back as ST.
    fn transpile(xml: &str) -> String {
        let pou = model::from_str(xml).unwrap();
        let unit = Transpiler::new(pou, IdProvider::default(), SourceLocationFactory::internal(xml))
            .transpile()
            .expect("todo error handling");

        AstSerializer::format_unit(&unit)
    }

    #[test]
    fn function_call() {
        //                      +-------- myAdd --------+  (1)
        //    localA  --------->| in1              myAdd|--------->  localResult  (2)
        //    localB  --------->| in2                   |
        //                      +-----------------------+
        //
        //    (1),(2)  evaluation-priority badges shown by the IDE
        //
        // The block is evaluated once into `temp_0`; the sink then assigns it.
        let xml = include_str!("../fixtures/function_call/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localB : DINT;
            localResult : DINT;
        END_VAR
            temp_0 := myAdd(in1 := localA, in2 := localB);
            localResult := temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn shared_result() {
        //                      +-------- myAdd --------+  (1)
        //    localA  --------->| in1              myAdd|-------+-------->  localResultA  (2)
        //    localB  --------->| in2                   |       |
        //                      +-----------------------+       +-------->  localResultB  (3)
        //
        //    (1),(2),(3)  evaluation-priority badges shown by the IDE
        //
        // The shared result is evaluated once into `temp_0`; both sinks then read it.
        let xml = include_str!("../fixtures/shared_result/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localB : DINT;
            localResultA : DINT;
            localResultB : DINT;
        END_VAR
            temp_0 := myAdd(in1 := localA, in2 := localB);
            localResultA := temp_0;
            localResultB := temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn chained_calls() {
        //                      +----- myAdd -----+ (2)       +----- myMul -----+ (3)
        //    localA  --------->| in1       myAdd |---------->| IN1       myMul |------->  localResultA  (4)
        //    localB  --+------>| in2             |       +-->| IN2             |
        //              |       +-----------------+       |   +-----------------+
        //              +-------------------------------- +
        //
        //    (2),(3),(4)  evaluation-priority badges shown by the IDE
        //
        // myAdd's result feeds myMul, so it goes through `temp_0` (a block output consumed
        // by another block always needs a temp); `localB` feeds both blocks.
        let xml = include_str!("../fixtures/chained_calls/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localB : DINT;
            localResultA : DINT;
        END_VAR
            temp_0 := myAdd(in1 := localA, in2 := localB);
            temp_1 := myMul(IN1 := temp_0, IN2 := localB);
            localResultA := temp_1;
        END_PROGRAM
        ");
    }

    #[test]
    fn nullary_call() {
        //                     +--- getOffset ---+ (1)
        //                     |       getOffset |--->  localResult  (2)
        //                     +-----------------+
        //
        //    (1),(2)  evaluation-priority badges shown by the IDE
        //
        // A function with no inputs still routes its result through a temp.
        let xml = include_str!("../fixtures/nullary_call/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localResult : DINT;
        END_VAR
            temp_0 := getOffset();
            localResult := temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn evaluation_order() {
        //                     +----- myMul -----+ (1)
        //    localA  -------->| in1       myMul |--->  resultMul  (2)
        //    localB  -------->| in2             |
        //                     +-----------------+
        //                     +----- myAdd -----+ (3)
        //    localC  -------->| in1       myAdd |--->  resultAdd  (4)
        //    localD  -------->| in2             |
        //                     +-----------------+
        //
        //    (1)-(4)  evaluation-priority badges shown by the IDE
        //
        // myAdd appears first in the document, but myMul has the lower priority and is
        // emitted first — statements follow evaluation order, not document order.
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
            temp_0 := myMul(in1 := localA, in2 := localB);
            resultMul := temp_0;
            temp_1 := myAdd(in1 := localC, in2 := localD);
            resultAdd := temp_1;
        END_PROGRAM
        ");
    }

    #[test]
    fn negated_input() {
        //                      +----- myGate -----+ (1)
        //    localA  --o------>| a         myGate |--->  localResult  (2)
        //    localB  --------->| b                |
        //                      +------------------+
        //
        //    o        a negated input pin (wraps its value in NOT)
        //    (1),(2)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/negated_input/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : BOOL;
            localB : BOOL;
            localResult : BOOL;
        END_VAR
            temp_0 := myGate(a := NOT localA, b := localB);
            localResult := temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn inout_variable() {
        //                      +---- accumulate ----+ (1)
        //    localValue  ----->| value              |
        //                      |          accumulate|--->  localResult  (2)
        //    localSum  <------>| sum                |
        //                      +--------------------+
        //
        //    <-->     an in-out pin (passed by reference)
        //    (1),(2)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/inout_variable/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localValue : DINT;
            localSum : DINT;
            localResult : DINT;
        END_VAR
            temp_0 := accumulate(value := localValue, sum := localSum);
            localResult := temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn literal_input() {
        //                      +----- myAdd -----+ (1)
        //    localA  --------->| in1       myAdd |--->  localResult  (2)
        //    100     --------->| in2             |
        //                      +-----------------+
        //
        //    (1),(2)  evaluation-priority badges shown by the IDE
        //
        // A data source identifier is parsed as an ST expression, so the literal `100`
        // flows straight into the call argument.
        let xml = include_str!("../fixtures/literal_input/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localResult : DINT;
        END_VAR
            temp_0 := myAdd(in1 := localA, in2 := 100);
            localResult := temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn function_pou() {
        //               +----- myAdd -----+ (1)
        //    a  ------->| in1       myAdd |--->  myFunc  (2)
        //    b  ------->| in2             |
        //               +-----------------+
        //
        // The transpiled POU is a FUNCTION; its result is a sink named after the function,
        // so it lowers to `myFunc := temp_0` inside `FUNCTION myFunc ... END_FUNCTION`.
        let xml = include_str!("../fixtures/function_pou/myFunc.cfc");
        assert_snapshot!(transpile(xml), @r"
        FUNCTION myFunc : DINT
        VAR_INPUT
            a : DINT;
            b : DINT;
        END_VAR
            temp_0 := myAdd(in1 := a, in2 := b);
            myFunc := temp_0;
        END_FUNCTION
        ");
    }

    #[test]
    fn function_block_pou() {
        //               +----- myAdd -----+ (1)
        //    a  ------->| in1       myAdd |--->  sum  (2)
        //    b  ------->| in2             |
        //               +-----------------+
        //
        // The transpiled POU is a FUNCTION_BLOCK; its VAR_OUTPUT is a sink named after the
        // output, so it lowers to `sum := temp_0` inside `FUNCTION_BLOCK myFb ... END_FUNCTION_BLOCK`.
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
            temp_0 := myAdd(in1 := a, in2 := b);
            sum := temp_0;
        END_FUNCTION_BLOCK
        ");
    }

    #[test]
    fn function_block_call() {
        //                   +------ Counter ------+ (1)
        //    localStep ---->| step          count |---->  localCount  (2)
        //                   +---------------------+
        //
        //    (1),(2)  evaluation-priority badges shown by the IDE
        //
        // The block targets an FB instance, so it is called on `myInstance` (not the type) and
        // has no return value; its `count` output goes through a temp the sink then reads.
        let xml = include_str!("../fixtures/function_block_call/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            myInstance : Counter;
            localStep : DINT;
            localCount : DINT;
        END_VAR
            myInstance(step := localStep, count => temp_0);
            localCount := temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn program_call() {
        //                        +----- auxProgram -----+ (1)
        //    localIncrement ---->| increment      total |---->  localTotal  (2)
        //                        +----------------------+
        //
        //    (1),(2)  evaluation-priority badges shown by the IDE
        //
        // The block targets a program: a global singleton called by name, with no return value.
        // Its `total` output goes through a temp the sink then reads.
        let xml = include_str!("../fixtures/program_call/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localIncrement : DINT;
            localTotal : DINT;
        END_VAR
            auxProgram(increment := localIncrement, total => temp_0);
            localTotal := temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn unconnected_arguments_function() {
        //                  +------ myFunc ------+ (1)
        //    localA ------>| a           myFunc |------>  localResult  (2)
        //                  | b  (unconnected)   |
        //                  | io (unconnected)   |
        //                  +--------------------+
        //
        //    (1),(2)  evaluation-priority badges shown by the IDE
        //
        // Only `a` is wired. The unconnected input `b` and in-out `io` are still emitted, with an
        // empty value, so the call keeps its full argument list (`b` defaults downstream; the empty
        // in-out is left for validation to reject).
        let xml = include_str!("../fixtures/unconnected_arguments_function/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localResult : DINT;
        END_VAR
            temp_0 := myFunc(a := localA, b := , io := );
            localResult := temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn unconnected_arguments_program() {
        //                      +---- auxProgram ----+ (1)
        //    localA ---------->| a                  |
        //                      | b  (unconnected)   |
        //                      | io (unconnected)   |
        //                      +--------------------+
        //
        //    (1)  evaluation-priority badge shown by the IDE
        //
        // Same partly-wired block, targeting a program (called by name, no return value).
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
        //                  +------- myFb -------+ (1)
        //    localA ------>| a                  |
        //                  | b  (unconnected)   |
        //                  | io (unconnected)   |
        //                  +--------------------+
        //
        //    (1)  evaluation-priority badge shown by the IDE
        //
        // Same partly-wired block, called on the FB instance `myInstance`.
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
        //                  +------ myFunc ------+ (1)
        //    localA ------>| a           myFunc |------>  localResult  (2)
        //                  |              extra |   (unconnected)
        //                  +--------------------+
        //
        //    (1),(2)  evaluation-priority badges shown by the IDE
        //
        // The result pin is wired and goes through a temp as usual; the unconnected `extra` output
        // feeds nothing, so it is emitted with an empty value rather than a temp no one reads.
        let xml = include_str!("../fixtures/unconnected_output_function/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localResult : DINT;
        END_VAR
            temp_0 := myFunc(a := localA, extra => );
            localResult := temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn unconnected_output_program() {
        //                      +---- auxProgram ----+ (1)
        //    localA ---------->| a           result |   (unconnected)
        //                      +--------------------+
        //
        //    (1)  evaluation-priority badge shown by the IDE
        //
        // The unconnected `result` output feeds nothing, so it is emitted empty.
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
        //                  +------- myFb -------+ (1)
        //    localA ------>| a           result |   (unconnected)
        //                  +--------------------+
        //
        //    (1)  evaluation-priority badge shown by the IDE
        //
        // Same unconnected output, called on the FB instance `myInstance`.
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
        //    +---- myFunctionBlock (myInstance) ----+ (0)
        //    |                                    a |--->  localA  (1)
        //    |                                    b |        (unconnected)
        //    |                                    c |--->  localB  (2)
        //    +--------------------------------------+
        //    +-------------- myFunction ------------+ (3)
        //    |                           myFunction |        (return, unconnected)
        //    |                                    a |--->  localA  (4)
        //    |                                    b |        (unconnected)
        //    +--------------------------------------+
        //
        //    (0)..(4)  evaluation-priority badges shown by the IDE
        //
        // An official multi-output export: wired outputs go through temps, unconnected ones are
        // emitted empty, and myFunction's unconnected return makes that call stand alone (it is
        // invoked only for its VAR_OUTPUT pins).
        let xml = include_str!("../fixtures/multiple_outputs/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            myInstance : myFunctionBlock;
            localA : DINT;
            localB : DINT;
        END_VAR
            myInstance(a => temp_0, b => , c => temp_1);
            localA := temp_0;
            localB := temp_1;
            myFunction(a => temp_2, b => );
            localA := temp_2;
        END_PROGRAM
        ");
    }

    #[test]
    fn conditional_return() {
        //    enable  --o--->| RETURN |  (0)
        //
        //    input   ------>  result    (1)
        //
        //    --o-->   a negated condition wire (returns when enable is FALSE)
        //    (0),(1)  evaluation-priority badges shown by the IDE
        //
        // The negated return is emitted first (lower priority); its condition is wrapped in NOT,
        // and the assignment that follows runs only when the early return did not fire.
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
        //    input  ------>  result    (0)
        //
        //                   | RETURN |  (1)
        //
        //    (no wire into RETURN -> unconditional)
        //    (0),(1)  evaluation-priority badges shown by the IDE
        //
        // A return with no condition wire lowers to a bare `RETURN;`, emitted after the assignment
        // (its higher priority orders it last).
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
        //    +-- alwaysFive --+ (0)
        //    |      alwaysFive|--(12)-->[ Connector "five" ]
        //    +----------------+
        //                       [ Continuation "five" ]--(7)-->  result  (1)
        //
        //    "five"    the label matching the connector to the continuation
        //    (0),(1)   evaluation-priority badges shown by the IDE
        //
        // The connector/continuation pair is a named virtual wire that resolves away: `result`
        // reads it straight through to alwaysFive's result, exactly as a direct wire would.
        let xml = include_str!("../fixtures/connector_continuation/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            result : DINT;
        END_VAR
            temp_0 := alwaysFive();
            result := temp_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn connector_continuation_chain() {
        // alwaysFive --(10)--> [Conn a]~[Cont a] --(11)--> [Conn b]~[Cont b] --(12)-->
        //   [Conn c]~[Cont c] --(13)--> [Conn d]~[Cont d] --(14)--> result
        //
        // Four named virtual wires linked end to end. The chain resolves transitively to the one
        // real producer, collapsing to a direct wire.
        let xml = include_str!("../fixtures/connector_continuation_chain/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            result : DINT;
        END_VAR
            temp_0 := alwaysFive();
            result := temp_0;
        END_PROGRAM
        ");
    }

    // TODO: a cyclic connector/continuation chain currently panics; once this crate has an error
    // story it should yield a proper diagnostic instead (cf. plc_xml's E085). Pins the behaviour.
    #[test]
    #[should_panic(expected = "cyclic connector/continuation chain")]
    fn connector_continuation_cycle() {
        let xml = include_str!("../fixtures/connector_continuation_cycle/mainProgram.cfc");
        transpile(xml);
    }

    // A CFC block can target an ACTION, which is called in ST as `instance.action()`. The
    // `<Block>` model has no field for the action name and the IDE's encoding is unknown, so
    // this is out of scope until that shape is settled.
    #[test]
    #[ignore = "TODO: actions"]
    fn action_call() {
        todo!("blocks targeting actions are not yet modeled")
    }
}
