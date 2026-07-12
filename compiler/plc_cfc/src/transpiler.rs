//! CFC (XML) to AST transpiler
//!
//! Converts a CFC file into an ST equivalent AST, which the compiler pipeline can then use like any other
//! compilation unit. In total there are five core concepts the transpiler needs to handle to create a
//! correct AST. Each one of them is described in the following sections.
//!
//! ## Global IDs
//!
//! Every element in a CFC network carries a unique numeric ID, and most of what the transpiler does comes
//! down to looking values up by those IDs. Two kinds are worth keeping apart.
//!
//! The first is the `globalId` on each object (a source, sink, block, and so on). It identifies the object
//! itself, and the transpiler uses it to give every generated statement a source location, so that a later
//! diagnostic can point back at the right element in the graphical editor.
//!
//! The second lives on connection points. An output point exposes a value under a `connectionPointOutId`, and
//! whatever consumes that value references the same ID through a `refConnectionPointOutId`. This is how the
//! graph is wired: there is no textual nesting as in ST, so a connection is simply one element naming another
//! by ID. Every section below leans on this; whenever an element needs a value another produced, it finds it
//! by following the connection back to the matching ID.
//!
//! ## Evaluation Priority
//!
//! A CFC network is a graph, not a sequence. Its objects have positions and wires but no inherent order to
//! run in, whereas Structured Text is a list of statements that execute top to bottom. So before emitting
//! anything, the transpiler has to settle on an order.
//!
//! CFC supplies one directly: every object carries an evaluation priority, assigned in the editor and shown
//! as the `(n)` badge beside each object in the diagrams that follow. The transpiler gathers all of a
//! network's objects, sorts them by that priority and only then walks them to produce statements, lowest
//! first.
//!
//! The priority is stored in an `<EvaluationPriority priorityInNetwork="...">` entry on the object. Objects
//! without one are rejected at deserialize time — a missing priority would silently sort ahead of the whole
//! network, see [`crate::model`].
//!
//! ## Source & Sink
//!
//! The simplest thing to do in CFC is to assign the value of one data element to another. Sources and sinks
//! represent just that, where a source produces a value and a sink consumes one. For example, a very simple
//! assignment between a source and sink element might have the following form
//!
//! ```xml
//! <!-- Source -->
//! <ppx:FbdObject xsi:type="ppx:DataSource" identifier="x + 5" globalId="1">
//!     <ppx:ConnectionPointOut connectionPointOutId="2"> <!-- The produced value has an ID of 2 -->
//!         <!-- ... -->
//!     </ppx:ConnectionPointOut>
//! </ppx:FbdObject>
//!
//! <!-- Sink -->
//! <ppx:FbdObject xsi:type="ppx:DataSink" identifier="y" globalId="3">
//!     <ppx:AddData>
//!         <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
//!             <EvaluationPriority priorityInNetwork="0"/>
//!         </ppx:Data>
//!     </ppx:AddData>
//!     <ppx:ConnectionPointIn>
//!         <ppx:Connection refConnectionPointOutId="2"/> <!-- The referenced value has an ID of 2 -->
//!     </ppx:ConnectionPointIn>
//! </ppx:FbdObject>
//! ```
//!
//! which is the graphical way of writing `x + 5 (source) --> y (sink)`. To turn that into an ST assignment we
//! follow the connection backwards. A sink references the value it wants to consume by ID, through its
//! `refConnectionPointOutId` field; a source, on the other hand, exposes its value under the matching
//! `connectionPointOutId`. So the transpiler starts at the sink, follows that ID to the source it points at
//! and builds the assignment from the two. This results in `y := x + 5`.
//!
//! Worth noting: producing and referencing values by ID is universal, the same idea shows up in one form or
//! another in every section that follows.
//!
//!
//! ## Block
//!
//! Blocks represent call statements in CFC. Wires coming in on the left are the call's arguments, wires
//! leaving on the right are its results. Take a function `increment` with an input `step`, an inout
//! `counter`, an output `overflow` and (because it's a function) a return value `increment`
//!
//! ```xml
//! <ppx:FbdObject xsi:type="ppx:Block" typeName="increment" globalId="1">
//!     <ppx:InputVariables>
//!         <ppx:InputVariable parameterName="step">
//!             <ppx:ConnectionPointIn>
//!                 <ppx:Connection refConnectionPointOutId="2"/> <!-- source: delta -->
//!             </ppx:ConnectionPointIn>
//!         </ppx:InputVariable>
//!     </ppx:InputVariables>
//!     <ppx:InOutVariables>
//!         <ppx:InOutVariable parameterName="counter">
//!             <ppx:ConnectionPointIn>
//!                 <ppx:Connection refConnectionPointOutId="3"/> <!-- source: ticks -->
//!             </ppx:ConnectionPointIn>
//!         </ppx:InOutVariable>
//!     </ppx:InOutVariables>
//!     <ppx:OutputVariables>
//!         <ppx:OutputVariable parameterName="increment"> <!-- same name as the block => the return value -->
//!             <ppx:ConnectionPointOut connectionPointOutId="4"/> <!-- sink: nextValue -->
//!         </ppx:OutputVariable>
//!         <ppx:OutputVariable parameterName="overflow">
//!             <ppx:ConnectionPointOut connectionPointOutId="5"/> <!-- sink: wrapped -->
//!         </ppx:OutputVariable>
//!     </ppx:OutputVariables>
//! </ppx:FbdObject>
//! ```
//!
//! which graphically looks like
//! ```text
//!                     +-------- increment ----------+  (0)
//!    delta  --------->| step              increment |--------->  nextValue  (1)
//!    ticks  <-------->| counter            overflow |--------->  wrapped    (2)
//!                     +-----------------------------+
//!
//!    <-->   an in-out pin, bound by reference (read and written in place)
//!    (n)    evaluation-priority badge shown by the IDE
//! ```
//!
//! Transpiling this is the same idea as the previous section, only now there are several wires to follow
//! instead of one. We start with the input and inout pins: each becomes a named argument of the form
//! `<parameter> := <value>`, and we find `<value>` exactly like a sink does; take the pin's
//! `refConnectionPointOutId` and follow it back to whoever produced it. For our example that gives the
//! argument list so far:
//!
//! ```text
//! increment(step := delta, counter := ticks, /* ... */)
//! ```
//!
//! Output pins go the other way. An output produces a value, but a block is only evaluated once per cycle, so
//! we cannot re-run the call for every consumer that reads it. Instead every consumed output is routed into a
//! generated result variable: the call gets `<output> => __<type>_res_<n>` and every consumer reads that
//! variable at its own evaluation slot. These variables are created before any statement is emitted — a
//! consumer may be emitted before its producer — and live in a plain `VAR` block so they persist across
//! cycles (unlike `VAR_TEMP`). Because CFC executes cyclically, this gives consumers scheduled *before* the
//! producer well-defined semantics: they read the value the producer wrote in the *previous* cycle — the
//! idiomatic way to draw feedback loops.
//!
//! The treatment is uniform across all block kinds. Technically the internal state of stateful POUs could be
//! read directly (`<instance>.<pin>` for a function block, `<program>.<pin>` for a program's singleton), but
//! the XML gives us no way to tell the POU type behind a block element, and a result variable per output pin
//! is correct for every kind — it even snapshots per block, so two blocks calling the same instance keep
//! their consumers independent.
//!
//! The return value follows the same principle, only the form differs. A function's result is the value of the
//! call itself rather than a named pin, so instead of `=> __..._res_...` we assign the whole call to the
//! variable: `__increment_res_0 := increment(...)`. The motivation is again the once-per-cycle rule: if two
//! sinks read an uncaptured result, inlining the call would invoke `increment` twice and run its side effects
//! twice, so we call it once and let both read the variable.
//!
//! Put together, the block lowers to (statement order follows evaluation priority):
//!
//! ```text
//! VAR
//!     __increment_res_0 : __return@increment;
//!     __increment_res_1 : __output@increment@overflow;
//! END_VAR
//!
//! __increment_res_0 := increment(step := delta, counter := ticks, overflow => __increment_res_1);
//! nextValue := __increment_res_0;
//! wrapped := __increment_res_1;
//! ```
//!
//! Those temp types are placeholders; a later pass resolves `__return@increment` and
//! `__output@increment@overflow` to their real types, see [`crate::placeholder`].
//!
//! Our example was a function, but every block kind transpiles the same way; only the call operator changes.
//! Programs are, like functions, called by their type name, function blocks through their instance
//! (`myInstance(...)`), and actions and methods through a qualified `myInstance.myAction(...)` (where the
//! block's `typeName` is `<fb>.<member>`). Only functions have a return value.
//!
//! ## Jump
//!
//! TODO: Hasn't been implemented yet.
//!
//! ## Return
//!
//! A return ends the POU early. It has a single optional input wire carrying a boolean condition: with the
//! wire, the POU returns when the condition holds; without it, the return is unconditional.
//!
//! ```text
//!    enable  --o--->| RETURN |  (0)
//!
//!    input   ------>  result    (1)
//!
//!    --o-->   a negated condition wire (returns when enable is FALSE)
//!    (n)      evaluation-priority badge shown by the IDE
//! ```
//!
//! We resolve the condition by ID, exactly like any other consumed value, and emit it as a guarded
//! `IF ... THEN RETURN; END_IF;`. A return can also be negated (the `negated` flag in its `AddData`), in which
//! case the condition is wrapped in `NOT` first. The network above lowers to:
//!
//! ```text
//! IF NOT enable THEN RETURN; END_IF;
//! result := input;
//! ```
//!
//! With no condition wired in there is nothing to guard on, and the return lowers to a bare `RETURN;`.
//!
//! ## Connector & Continuation
//!
//! Sometimes a value has to travel across the diagram to a far-away consumer. Instead of drawing one long
//! wire, CFC lets you drop two labeled stubs: a connector at the producing end and a continuation with the
//! same label at the consuming end. Matched by their label, the pair behaves exactly like a direct wire.
//!
//! ```text
//!    +-- alwaysFive --+ (0)
//!    |      alwaysFive|--id 12-->[ Connector "five" ]
//!    +----------------+
//!
//!    [ Continuation "five" ]--id 7-->  result  (1)
//!
//!    "five"   the label matching connector to continuation
//!    (n)      evaluation-priority badge shown by the IDE
//! ```
//!
//! The pair carries no behavior of its own; the transpiler resolves it away. While scanning the network it
//! records each continuation's output ID as an alias for the connector's input ID, and whenever it resolves a
//! consumed value it first follows any such aliases back to the real producer. So the network above is
//! transpiled as if `result` were wired straight to the block:
//!
//! ```text
//! __alwaysFive_res_0 := alwaysFive();
//! result := __alwaysFive_res_0;
//! ```
//!
//! Chains (a continuation feeding another connector) are followed transitively, and a cycle terminates instead
//! of looping forever.
//!

use indexmap::IndexMap;

use ast::ast::{
    AstFactory, AstId, AstNode, CompilationUnit, DataTypeDeclaration, LinkageType, Variable, VariableBlock,
};
use ast::provider::IdProvider;
use plc::{lexer, parser};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::{SourceLocation, SourceLocationFactory};

use crate::{
    model::{Block, DataSink, Operation, Pou, Return},
    placeholder,
    resolver::{Object, Resolver},
};

pub struct Transpiler {
    pou: Pou,
    resolver: Resolver,
    temporaries: IndexMap<u64, Variable>,
    id_provider: IdProvider,
    range_factory: SourceLocationFactory,
}

impl Transpiler {
    pub fn new(pou: Pou, id_provider: IdProvider, range_factory: SourceLocationFactory) -> Transpiler {
        let resolver = Resolver::resolve(&pou);
        let temporaries = create_temporaries(&pou, &resolver);

        Transpiler { pou, resolver, temporaries, id_provider, range_factory }
    }

    pub fn transpile(mut self) -> Result<CompilationUnit, Diagnostic> {
        if self.pou.declaration().trim().is_empty() {
            return Ok(CompilationUnit::new(self.range_factory.get_file_name().unwrap_or_default()));
        }

        // Parse the declaration first
        let (mut unit, diagnostics) = {
            let declaration = {
                let content = self.pou.declaration();
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

        if !diagnostics.is_empty() {
            return Err(Diagnostic::new("Invalid ST declaration").with_sub_diagnostics(diagnostics));
        }

        // Then the actual graphical units. The declaration may contain method implementations, which
        // the parser pushes ahead of the POU's own body — find the body by name (methods carry
        // qualified `<pou>.<method>` names) rather than assuming its position.
        let statements = self.transpile_network();
        let name = self.pou.name();
        let Some(implementation) =
            unit.implementations.iter_mut().find(|imp| imp.name.eq_ignore_ascii_case(name))
        else {
            return Err(Diagnostic::new(format!(
                "Declaration does not contain an implementation for `{name}`"
            )));
        };
        implementation.statements = statements;

        // Inject the generated temporary variables, if any. They live in a plain VAR block so they
        // persist across cycles in a program or function block body (previous-cycle reads, feedback
        // loops); only inside a function body do they remain per-invocation storage. Like the
        // implementation above, the POU is selected by name rather than position.
        if !self.temporaries.is_empty() {
            let Some(pou) = unit.pous.iter_mut().find(|pou| pou.name.eq_ignore_ascii_case(name)) else {
                return Err(Diagnostic::new(format!("Declaration does not contain a POU named `{name}`")));
            };

            pou.variable_blocks.push(VariableBlock::local(self.temporaries.into_values().collect()));
        }

        Ok(unit)
    }

    fn transpile_network(&mut self) -> Vec<AstNode> {
        // Aggregate all objects, sorted by their evaluation priority such that we can transpile
        // correct ST code
        let operations = self.pou.network().get_ordered_operations();

        // Then do actual transpiling
        let mut statements = Vec::new();
        for operation in &operations {
            let statement = match operation {
                Operation::Call(block) => Some(self.transpile_call(block)),
                Operation::Sink(sink) => self.transpile_sink(sink),
                Operation::Return(ret) => Some(self.transpile_return(ret)),
            };

            statements.extend(statement);
        }

        statements
    }

    /// Transpiles a block into a function call
    fn transpile_call(&mut self, block: &Block) -> AstNode {
        let location = self.create_object_location(block.global_id, block.add_data.priority);

        let mut arguments = Vec::new();

        // Input and InOut variables; `<parameter> := <argument>` (or `<parameter> := ` when unconnected)
        for parameter in block.inputs.iter().chain(&block.inouts) {
            let id = parameter.connection_in;
            arguments.push(self.create_argument(&parameter.parameter_name, id, parameter.negated, &location));
        }

        // Output variables; similar to input and inout variables with the exception of also routing the
        // values into result variables. Technically we could directly access the internal state of stateful
        // POUs (`<instance>.<pin>` for function blocks, `<program>.<pin>` for programs) but the XML provides
        // no way to identify a POU type by its block element, hence the unified approach for all of them.
        let mut return_value = None;
        for output in &block.outputs {
            // Pre-created by [`create_temporaries`]; `None` means the pin is unconsumed
            let temp = self.temporaries.get(&output.connection_out).map(|var| var.name.clone());

            // We're dealing with a function if both the output variable and block name are identical
            // in which case the whole call is assigned to the temporary rather than routing the value
            // through an output argument.
            if output.parameter_name == block.type_name {
                return_value = temp;
                continue;
            }

            // Ordinary output variable, either `<parameter> => __..._res_...` or `<parameter> =>` if empty
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

    /// Transpile a data sink into an assignment, or nothing when the sink has no incoming connection
    /// (the validator reports that case as a warning).
    fn transpile_sink(&mut self, sink: &DataSink) -> Option<AstNode> {
        let location = self.create_object_location(sink.global_id, sink.add_data.priority);
        let value = self.resolve(sink.connection_in?, &location);
        let target = self.create_member_reference(&sink.identifier, &location);

        Some(AstFactory::create_assignment(target, value, self.next_id()))
    }

    /// Transpiles a conditional return so a return (duh)
    fn transpile_return(&mut self, ret: &Return) -> AstNode {
        let location = self.create_object_location(ret.global_id, ret.add_data.priority);

        let condition = match ret.connection_in {
            Some(id) => {
                let mut node = self.resolve(id, &location);
                if ret.add_data.negated {
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

        if let Some(temp) = self.temporaries.get(&id) {
            let name = temp.name.clone();
            let node = self.create_member_reference(&name, location);

            // A negated output pin inverts the value it feeds to each of its consumers
            return match self.resolver.get(id) {
                Some(Object::BlockOutput(_, output)) if output.negated => {
                    node.negate(self.id_provider.clone())
                }
                _ => node,
            };
        }

        match self.resolver.get(id) {
            Some(Object::Variable(source)) => self.parse_expression(&source.identifier),
            Some(Object::BlockOutput(..)) => {
                unreachable!("consumed block outputs are pre-created as temporaries")
            }
            None => panic!("should have been caught by validation (E081)"),
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

    fn create_object_location(&self, global_id: u64, priority: Option<u64>) -> SourceLocation {
        self.range_factory.create_block_location(global_id as usize, priority.map(|p| p as usize))
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

/// Creates a `__<type>_res_<n>` variable for every consumed block output pin, keyed by the pin's
/// `connectionPointOutId`. Creating them before the network walk lets consumers be emitted before
/// their producers (previous-cycle reads, feedback loops); walking in evaluation order keeps the
/// numbering aligned with the emission order.
fn create_temporaries(pou: &Pou, resolver: &Resolver) -> IndexMap<u64, Variable> {
    let mut temporaries = IndexMap::new();

    for operation in pou.network().get_ordered_operations() {
        let Operation::Call(block) = operation else { continue };

        for output in &block.outputs {
            let id = output.connection_out;
            if !resolver.is_consumed(id) {
                continue;
            }

            let placeholder = if output.parameter_name == block.type_name {
                placeholder::return_placeholder(&block.type_name)
            } else {
                placeholder::output_placeholder(&block.type_name, &output.parameter_name)
            };

            let name = format!("__{}_res_{}", block.type_name, temporaries.len());
            temporaries.insert(id, create_internal_variable(&name, placeholder));
        }
    }

    temporaries
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
            .unwrap();

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
        let xml = include_str!("../fixtures/valid/function_call/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localB : DINT;
            localResult : DINT;
        END_VAR
        VAR
            __myAdd_res_0 : __return@myAdd;
        END_VAR
            __myAdd_res_0 := myAdd(in1 := localA, in2 := localB);
            localResult := __myAdd_res_0;
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
        let xml = include_str!("../fixtures/valid/shared_result/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localB : DINT;
            localResultA : DINT;
            localResultB : DINT;
        END_VAR
        VAR
            __myAdd_res_0 : __return@myAdd;
        END_VAR
            __myAdd_res_0 := myAdd(in1 := localA, in2 := localB);
            localResultA := __myAdd_res_0;
            localResultB := __myAdd_res_0;
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
        let xml = include_str!("../fixtures/valid/chained_calls/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localB : DINT;
            localResultA : DINT;
        END_VAR
        VAR
            __myAdd_res_0 : __return@myAdd;
            __myMul_res_1 : __return@myMul;
        END_VAR
            __myAdd_res_0 := myAdd(in1 := localA, in2 := localB);
            __myMul_res_1 := myMul(IN1 := __myAdd_res_0, IN2 := localB);
            localResultA := __myMul_res_1;
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
        let xml = include_str!("../fixtures/valid/nullary_call/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM mainProgram
        VAR
            localResult : DINT;
        END_VAR
        VAR
            __getOffset_res_0 : __return@getOffset;
        END_VAR
            __getOffset_res_0 := getOffset();
            localResult := __getOffset_res_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn evaluation_order() {
        //                      +----- myMul -----+ (1)
        //    localA  -------->| in1       myMul |--->  resultMul  (2)
        //    localB  -------->| in2             |
        //                      +-----------------+
        //                      +----- myAdd -----+ (3)
        //    localC  -------->| in1       myAdd |--->  resultAdd  (4)
        //    localD  -------->| in2             |
        //                      +-----------------+
        //
        //    (1)-(4)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/evaluation_order/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localB : DINT;
            localC : DINT;
            localD : DINT;
            resultMul : DINT;
            resultAdd : DINT;
        END_VAR
        VAR
            __myMul_res_0 : __return@myMul;
            __myAdd_res_1 : __return@myAdd;
        END_VAR
            __myMul_res_0 := myMul(in1 := localA, in2 := localB);
            resultMul := __myMul_res_0;
            __myAdd_res_1 := myAdd(in1 := localC, in2 := localD);
            resultAdd := __myAdd_res_1;
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
        let xml = include_str!("../fixtures/valid/negated_input/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM mainProgram
        VAR
            localA : BOOL;
            localB : BOOL;
            localResult : BOOL;
        END_VAR
        VAR
            __myGate_res_0 : __return@myGate;
        END_VAR
            __myGate_res_0 := myGate(a := NOT localA, b := localB);
            localResult := __myGate_res_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn negated_output() {
        //                      +----- myGate -----+ (1)
        //    localA  --------->| a         myGate |--o--->  localResult  (2)
        //    localB  --------->| b                |
        //                      +------------------+
        //
        //    o        a negated output pin (consumers read NOT the pin's value)
        //    (1),(2)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/negated_output/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM mainProgram
        VAR
            localA : BOOL;
            localB : BOOL;
            localResult : BOOL;
        END_VAR
        VAR
            __myGate_res_0 : __return@myGate;
        END_VAR
            __myGate_res_0 := myGate(a := localA, b := localB);
            localResult := NOT __myGate_res_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn negated_output_fan_out() {
        //                       +----- Toggle -----+ (1)
        //    localEnable  ----->| enable      isOn |--o--+---->  localOff  (2)
        //                       +------------------+     |
        //                                                |    +----- myGate -----+ (3)
        //                                                +--o-| a         myGate |--->  localResult  (4)
        //                       localB  ----------------------| b                |
        //                                                     +------------------+
        //
        //    o (pin)   the negated isOn output pin (consumers read NOT the pin's value)
        //    o (wire)  the negated a input pin (wraps its value in NOT)
        //    (1)-(4)   evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/negated_output_fan_out/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM mainProgram
        VAR
            myInstance : Toggle;
            localEnable : BOOL;
            localB : BOOL;
            localOff : BOOL;
            localResult : BOOL;
        END_VAR
        VAR
            __Toggle_res_0 : __output@Toggle@isOn;
            __myGate_res_1 : __return@myGate;
        END_VAR
            myInstance(enable := localEnable, isOn => __Toggle_res_0);
            localOff := NOT __Toggle_res_0;
            __myGate_res_1 := myGate(a := NOT NOT __Toggle_res_0, b := localB);
            localResult := __myGate_res_1;
        END_PROGRAM
        ");
    }

    #[test]
    fn inout_variable() {
        //                       +---- accumulate ----+ (1)
        //    localValue  ------>| value              |
        //                       |          accumulate|--->  localResult  (2)
        //    localSum  <------->| sum                |
        //                       +--------------------+
        //
        //    <-->     an in-out pin (passed by reference)
        //    (1),(2)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/inout_variable/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM mainProgram
        VAR
            localValue : DINT;
            localSum : DINT;
            localResult : DINT;
        END_VAR
        VAR
            __accumulate_res_0 : __return@accumulate;
        END_VAR
            __accumulate_res_0 := accumulate(value := localValue, sum := localSum);
            localResult := __accumulate_res_0;
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
        let xml = include_str!("../fixtures/valid/literal_input/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localResult : DINT;
        END_VAR
        VAR
            __myAdd_res_0 : __return@myAdd;
        END_VAR
            __myAdd_res_0 := myAdd(in1 := localA, in2 := 100);
            localResult := __myAdd_res_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn expression_source() {
        //    localA + 5  ----------->  result   (0)
        //
        //    (0)  evaluation-priority badge shown by the IDE
        let xml = include_str!("../fixtures/valid/expression_source/mainProgram.cfc");
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
        //               +----- myAdd -----+ (1)
        //    a  ------->| in1       myAdd |--->  myFunc  (2)
        //    b  ------->| in2             |
        //               +-----------------+
        //
        //    myFunc   the FUNCTION's return value (a sink named after the function)
        //    (1),(2)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/function_pou/myFunc.cfc");
        assert_snapshot!(transpile(xml), @"
        FUNCTION myFunc : DINT
        VAR_INPUT
            a : DINT;
            b : DINT;
        END_VAR
        VAR
            __myAdd_res_0 : __return@myAdd;
        END_VAR
            __myAdd_res_0 := myAdd(in1 := a, in2 := b);
            myFunc := __myAdd_res_0;
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
        //    sum      a VAR_OUTPUT of the function block (a sink named after the output)
        //    (1),(2)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/function_block_pou/myFb.cfc");
        assert_snapshot!(transpile(xml), @"
        FUNCTION_BLOCK myFb
        VAR_INPUT
            a : DINT;
            b : DINT;
        END_VAR
        VAR_OUTPUT
            sum : DINT;
        END_VAR
        VAR
            __myAdd_res_0 : __return@myAdd;
        END_VAR
            __myAdd_res_0 := myAdd(in1 := a, in2 := b);
            sum := __myAdd_res_0;
        END_FUNCTION_BLOCK
        ");
    }

    #[test]
    fn function_block_call() {
        //                   +------ Counter ------+ (1)
        //    localStep ---->| step          count |---->  localCount  (2)
        //                   +---------------------+
        //
        //    Counter   called on instance myInstance (the block's instanceName)
        //    (1),(2)   evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/function_block_call/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM mainProgram
        VAR
            myInstance : Counter;
            localStep : DINT;
            localCount : DINT;
        END_VAR
        VAR
            __Counter_res_0 : __output@Counter@count;
        END_VAR
            myInstance(step := localStep, count => __Counter_res_0);
            localCount := __Counter_res_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn action_call() {
        //                   +-- function_block_0.myAction --+ (0)
        //    myInstance --->|                               |
        //                   +-------------------------------+
        //
        //    function_block_0.myAction   the action, called on instance myInstance
        //    (0)                         evaluation-priority badge shown by the IDE
        let xml = include_str!("../fixtures/valid/action_call/mainProgram.cfc");
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
        //                        +----- auxProgram -----+ (1)
        //    localIncrement ---->| increment      total |---->  localTotal  (2)
        //                        +----------------------+
        //
        //    (1),(2)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/program_call/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM mainProgram
        VAR
            localIncrement : DINT;
            localTotal : DINT;
        END_VAR
        VAR
            __auxProgram_res_0 : __output@auxProgram@total;
        END_VAR
            auxProgram(increment := localIncrement, total => __auxProgram_res_0);
            localTotal := __auxProgram_res_0;
        END_PROGRAM
        ");
    }

    // Note: the unconnected inout (`io := `) transpiles but is invalid; the pipeline's ST validation
    // rejects it later with E031.
    #[test]
    fn unconnected_arguments_function() {
        //                   +------ myFunc ------+ (1)
        //    localA ------->| a           myFunc |------->  localResult  (2)
        //                   | b  (unconnected)   |
        //                   | io (unconnected)   |
        //                   +--------------------+
        //
        //    (unconnected)  a pin with no incoming wire
        //    (1),(2)        evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/unconnected_arguments_function/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localResult : DINT;
        END_VAR
        VAR
            __myFunc_res_0 : __return@myFunc;
        END_VAR
            __myFunc_res_0 := myFunc(a := localA, b := , io := );
            localResult := __myFunc_res_0;
        END_PROGRAM
        ");
    }

    // Note: the unconnected inout (`io := `) transpiles but is invalid; the pipeline's ST validation
    // rejects it later with E031.
    #[test]
    fn unconnected_arguments_program() {
        //                       +---- auxProgram ----+ (1)
        //    localA ----------->| a                  |
        //                       | b  (unconnected)   |
        //                       | io (unconnected)   |
        //                       +--------------------+
        //
        //    (unconnected)  a pin with no incoming wire
        //    (1)            evaluation-priority badge shown by the IDE
        let xml = include_str!("../fixtures/valid/unconnected_arguments_program/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @r"
        PROGRAM mainProgram
        VAR
            localA : DINT;
        END_VAR
            auxProgram(a := localA, b := , io := );
        END_PROGRAM
        ");
    }

    // Note: the unconnected inout (`io := `) transpiles but is invalid; the pipeline's ST validation
    // rejects it later with E031.
    #[test]
    fn unconnected_arguments_function_block() {
        //                   +------- myFb -------+ (1)
        //    localA ------->| a                  |
        //                   | b  (unconnected)   |
        //                   | io (unconnected)   |
        //                   +--------------------+
        //
        //    myFb           called on instance myInstance (the block's instanceName)
        //    (unconnected)  a pin with no incoming wire
        //    (1)            evaluation-priority badge shown by the IDE
        let xml = include_str!("../fixtures/valid/unconnected_arguments_function_block/mainProgram.cfc");
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
        //                   +------ myFunc ------+ (1)
        //    localA ------->| a           myFunc |------>  localResult  (2)
        //                   |              extra |   (unconnected)
        //                   +--------------------+
        //
        //    extra    an output pin with no outgoing wire
        //    (1),(2)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/unconnected_output_function/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM mainProgram
        VAR
            localA : DINT;
            localResult : DINT;
        END_VAR
        VAR
            __myFunc_res_0 : __return@myFunc;
        END_VAR
            __myFunc_res_0 := myFunc(a := localA, extra => );
            localResult := __myFunc_res_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn unconnected_output_program() {
        //                       +---- auxProgram ----+ (1)
        //    localA ----------->| a           result |   (result unconnected)
        //                       +--------------------+
        //
        //    result  an output pin with no outgoing wire
        //    (1)     evaluation-priority badge shown by the IDE
        let xml = include_str!("../fixtures/valid/unconnected_output_program/mainProgram.cfc");
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
        //                   +------- myFb -------+ (1)
        //    localA ------->| a           result |   (result unconnected)
        //                   +--------------------+
        //
        //    myFb     called on instance myInstance (the block's instanceName)
        //    result   an output pin with no outgoing wire
        //    (1)      evaluation-priority badge shown by the IDE
        let xml = include_str!("../fixtures/valid/unconnected_output_function_block/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
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
        //
        //    +-------------- myFunction ------------+ (3)
        //    |                           myFunction |        (return, unconnected)
        //    |                                    a |--->  localA  (4)
        //    |                                    b |        (unconnected)
        //    +--------------------------------------+
        //
        //    (unconnected)  an output pin with no outgoing wire
        //    (0)..(4)       evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/multiple_outputs/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM mainProgram
        VAR
            myInstance : myFunctionBlock;
            localA : DINT;
            localB : DINT;
        END_VAR
        VAR
            __myFunctionBlock_res_0 : __output@myFunctionBlock@a;
            __myFunctionBlock_res_1 : __output@myFunctionBlock@c;
            __myFunction_res_2 : __output@myFunction@a;
        END_VAR
            myInstance(a => __myFunctionBlock_res_0, b => , c => __myFunctionBlock_res_1);
            localA := __myFunctionBlock_res_0;
            localB := __myFunctionBlock_res_1;
            myFunction(a => __myFunction_res_2, b => );
            localA := __myFunction_res_2;
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
        let xml = include_str!("../fixtures/valid/conditional_return/mainProgram.cfc");
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
        let xml = include_str!("../fixtures/valid/unconditional_return/mainProgram.cfc");
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
        //    |      alwaysFive|--id 12-->[ Connector "five" ]
        //    +----------------+
        //
        //                       [ Continuation "five" ]--id 7-->  result  (1)
        //
        //    "five"    the label matching the connector to the continuation
        //    (0),(1)   evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/connector_continuation/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM mainProgram
        VAR
            result : DINT;
        END_VAR
        VAR
            __alwaysFive_res_0 : __return@alwaysFive;
        END_VAR
            __alwaysFive_res_0 := alwaysFive();
            result := __alwaysFive_res_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn connector_continuation_chain() {
        //    +-- alwaysFive --+ (0)
        //    |      alwaysFive|--id 10-->[Conn a]   [Cont a]--id 11-->[Conn b]   [Cont b]--id 12-->[Conn c]
        //    +----------------+                                                              |
        //         [Cont c]--id 13-->[Conn d]   [Cont d]--id 14-->  result  (1)  <------------+
        //
        //    a,b,c,d   labels matching each connector to its continuation
        //    (0),(1)   evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/connector_continuation_chain/mainProgram.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM mainProgram
        VAR
            result : DINT;
        END_VAR
        VAR
            __alwaysFive_res_0 : __return@alwaysFive;
        END_VAR
            __alwaysFive_res_0 := alwaysFive();
            result := __alwaysFive_res_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn previous_cycle_sink() {
        //    +-- alwaysFive --+ (1)
        //    |      alwaysFive|------->  result  (0)
        //    +----------------+
        //
        //    (n)   evaluation-priority badges shown by the IDE; the sink runs BEFORE the block,
        //          so it reads the temporary's previous-cycle value (type default on cycle 1)
        let xml = include_str!("../fixtures/valid/previous_cycle_read/sink.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM sink
        VAR
            result : DINT;
        END_VAR
        VAR
            __alwaysFive_res_0 : __return@alwaysFive;
        END_VAR
            result := __alwaysFive_res_0;
            __alwaysFive_res_0 := alwaysFive();
        END_PROGRAM
        ");
    }

    #[test]
    fn previous_cycle_block_argument() {
        //    +-- alwaysFive --+ (1)      +---- square ----+ (0)
        //    |      alwaysFive|--------->| x        square|------->  result  (2)
        //    +----------------+          +----------------+
        //
        //    square runs first, computing on alwaysFive's previous-cycle result
        let xml = include_str!("../fixtures/valid/previous_cycle_read/block_argument.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM block_argument
        VAR
            result : DINT;
        END_VAR
        VAR
            __square_res_0 : __return@square;
            __alwaysFive_res_1 : __return@alwaysFive;
        END_VAR
            __square_res_0 := square(x := __alwaysFive_res_1);
            __alwaysFive_res_1 := alwaysFive();
            result := __square_res_0;
        END_PROGRAM
        ");
    }

    #[test]
    fn previous_cycle_conditional_return() {
        //    +--- isReady ----+ (1)
        //    |         isReady|------->| RETURN |  (0)
        //    +----------------+
        //
        //    the return guards on isReady's previous-cycle result
        let xml = include_str!("../fixtures/valid/previous_cycle_read/conditional_return.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM conditional_return
        VAR
            __isReady_res_0 : __return@isReady;
        END_VAR
            IF __isReady_res_0 THEN RETURN; END_IF;
            __isReady_res_0 := isReady();
        END_PROGRAM
        ");
    }

    #[test]
    fn previous_cycle_aliased_sink() {
        //    +-- alwaysFive --+ (1)
        //    |      alwaysFive|------->[ Connector "relay" ]
        //    +----------------+
        //
        //                       [ Continuation "relay" ]------->  result  (0)
        let xml = include_str!("../fixtures/valid/previous_cycle_read/alias.cfc");
        assert_snapshot!(transpile(xml), @"
        PROGRAM alias
        VAR
            result : DINT;
        END_VAR
        VAR
            __alwaysFive_res_0 : __return@alwaysFive;
        END_VAR
            result := __alwaysFive_res_0;
            __alwaysFive_res_0 := alwaysFive();
        END_PROGRAM
        ");
    }

    #[test]
    #[should_panic(expected = "CFC jumps are not yet supported")]
    fn jump_is_unsupported() {
        //    enable  ------>| JMP skip |  (0)
        //
        //    input   ------>  result      (1)
        //
        //    JMP skip   an (unsupported) conditional jump; transpiling the network panics
        let xml = include_str!("../fixtures/unsupported/jump/mainProgram.cfc");
        transpile(xml);
    }
}
