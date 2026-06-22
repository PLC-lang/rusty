//! Resolver tracking the ownership of elements.
//!
//! In CFC elements are referenced by their ID, that is elements yield an ID with a `connectionPointOutId`
//! field and other elements make use of it referencing said ID. For example
//! ```xml
//! <!-- Source variables with `localA` mapping to ID 1 and `localB` to ID 2 -->
//! <FbdObject xsi:type="DataSource" identifier="localA">
//!     <ConnectionPointOut connectionPointOutId="1"/>
//! </FbdObject>
//! <FbdObject xsi:type="DataSource" identifier="localB">
//!     <ConnectionPointOut connectionPointOutId="2"/>
//! </FbdObject>
//!
//! <!-- Sink variables (received their value from another element) with `localResult` mapping to ID 3 -->
//! <FbdObject xsi:type="DataSink" identifier="localResult">
//!     <ConnectionPointIn>
//!         <Connection refConnectionPointOutId="3"/>
//!     </ConnectionPointIn>
//! </FbdObject>
//!
//! <!-- A "block" (function call in this case) with node 1 and 2 (localA, localB) as its arguments
//!      and 3 (localResult) as its output result value -->
//! <FbdObject xsi:type="Block" typeName="myAdd">
//!     <InputVariables>
//!         <InputVariable parameterName="in1">
//!             <ConnectionPointIn>
//!                 <Connection refConnectionPointOutId="1"/>
//!             </ConnectionPointIn>
//!         </InputVariable>
//!         <InputVariable parameterName="in2">
//!             <ConnectionPointIn>
//!                 <Connection refConnectionPointOutId="2"/>
//!             </ConnectionPointIn>
//!         </InputVariable>
//!     </InputVariables>
//!     <OutputVariables>
//!         <OutputVariable parameterName="myAdd">
//!             <ConnectionPointOut connectionPointOutId="3"/>
//!         </OutputVariable>
//!     </OutputVariables>
//! </FbdObject>
//! ```
//!
//! Visualized it looks like this in the IDE
//! ```text
//!                      +-------- myAdd --------+  (2)
//!    localA  --------->| in1              myAdd|--------->  localResult  (1)
//!    localB  --------->| in2                   |
//!                      +-----------------------+
//!
//!    (2),(1)  evaluation-priority badges shown by the IDE
//! ```
//!
//! The resolver indexes each wire by the object that *produces* the value behind its ID
//! ([`Resolver::get`]); consumers resolve that value forward on demand.

use std::collections::{HashMap, HashSet};

use crate::model::{Block, CommonObject, DataSource, FbdObject, OutputVariable, Pou};

#[derive(Debug)]
pub struct Resolver {
    /// Maps a `connectionPointOutId` to the object producing the value on that wire.
    sources: HashMap<u64, Object>,

    /// Every `connectionPointOutId` that some element reads — a sink, or a block input/in-out
    /// pin. A block output whose id is absent here feeds nothing, so it can be lowered as an
    /// empty `param => ` rather than a temp no one reads (see [`Resolver::is_consumed`]).
    consumed: HashSet<u64>,

    /// Maps a continuation's output `connectionPointOutId` to the wire feeding the connector of
    /// the same label, so a named virtual wire (a connector/continuation pair) resolves to its
    /// real producer (see [`Resolver::resolve_alias`]).
    aliases: HashMap<u64, u64>,
}

#[derive(Debug)]
pub enum Object {
    Variable(DataSource),
    BlockOutput(Block, OutputVariable),
}

impl Resolver {
    pub fn index(pou: &Pou) -> Resolver {
        let mut sources = HashMap::new();
        let mut consumed = HashSet::new();
        let mut aliases = HashMap::new();

        let network = pou.get_network().expect("todo error handling");
        for object in &network.objects {
            match object {
                // A "producing" variable, e.g. `foo --> <other element>`
                FbdObject::DataSource(source) => {
                    sources.insert(
                        source.connection_point_out.as_ref().expect("todo error handling").id,
                        Object::Variable(source.clone()),
                    );
                }

                // A "consuming" variable, e.g. `<other element> --> foo`
                FbdObject::DataSink(sink) => {
                    if let Some(pin) = &sink.connection_point_in {
                        consumed.extend(pin.connections.iter().map(|c| c.ref_connection_point_out_id));
                    }
                }

                // A block: its output pins produce values, its input/in-out pins consume them.
                FbdObject::Block(block) => {
                    for variable in block.output_variables.iter().flat_map(|opt| &opt.variables) {
                        sources.insert(
                            variable.connection_point_out.as_ref().expect("todo error handling").id,
                            Object::BlockOutput(block.clone(), variable.clone()),
                        );
                    }

                    for variable in block.get_input_variables() {
                        consumed.extend(variable.get_referenced_argument_id());
                    }

                    for variable in block.get_inout_variables() {
                        consumed.extend(variable.get_referenced_argument_id());
                    }
                }

                // A return consumes its condition wire, like a sink consumes its value.
                FbdObject::Return(ret) => {
                    if let Some(pin) = &ret.connection_point_in {
                        consumed.extend(pin.connections.iter().map(|c| c.ref_connection_point_out_id));
                    }
                }

                // TODO: Support once needed
                _ => (),
            };
        }

        // Connector/continuation pairs form named virtual wires. A connector names the wire feeding
        // it; a continuation of the same label re-emits that wire elsewhere. Resolve each pair into
        // an alias from the continuation's output id to the wire the connector consumes, so a
        // downstream reader resolves straight through to the real producer.
        let mut connector_inputs: HashMap<&str, u64> = HashMap::new();
        for common in &network.common_objects {
            if let CommonObject::Connector(connector) = common {
                if let Some(id) = connector.get_referenced_argument_id() {
                    connector_inputs.insert(&connector.label, id);
                    consumed.insert(id); // the connector consumes the wire it names
                }
            }
        }

        for common in &network.common_objects {
            if let CommonObject::Continuation(continuation) = common {
                if let (Some(&in_id), Some(out_id) ) = (
                    connector_inputs.get(continuation.label.as_str()),
                    continuation.get_connection_point_out_id(),
                ) {
                    aliases.insert(out_id, in_id);
                }
            }
        }

        Resolver { sources, consumed, aliases }
    }

    pub fn get(&self, id: u64) -> Option<&Object> {
        self.sources.get(&id)
    }

    /// Whether the value on the given `connectionPointOutId` is read anywhere — by a sink or a
    /// block input/in-out pin. A block output that is *not* consumed feeds nothing.
    pub fn is_consumed(&self, id: u64) -> bool {
        self.consumed.contains(&id)
    }

    /// Follows connector/continuation aliases to the real producing wire, transitively (a
    /// connector may itself be fed by another continuation). A wire that is not a continuation
    /// output is returned unchanged.
    ///
    /// Panics on a cyclic chain; proper diagnostics are deferred until this crate gains an error
    /// story.
    pub fn resolve_alias(&self, mut id: u64) -> u64 {
        let mut visited = HashSet::new();
        while let Some(&next) = self.aliases.get(&id) {
            assert!(visited.insert(id), "cyclic connector/continuation chain at wire {id}");
            id = next;
        }
        id
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        model,
        resolver::{Object, Resolver},
    };

    #[test]
    fn function_call() {
        let xml = include_str!("../fixtures/function_call/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        assert_eq!(resolver.sources.len(), 3);

        let Object::Variable(variable) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(variable.global_id.unwrap(), 1);
        assert_eq!(variable.identifier, "localA");

        let Object::Variable(variable) = resolver.get(4).unwrap() else { panic!() };
        assert_eq!(variable.global_id.unwrap(), 3);
        assert_eq!(variable.identifier, "localB");

        let Object::BlockOutput(block, variable) = resolver.get(8).unwrap() else { panic!() };
        assert_eq!(block.type_name, "myAdd");
        assert_eq!(variable.parameter_name, "myAdd");
        assert_eq!(variable.connection_point_out.as_ref().unwrap().id, 8);
    }

    #[test]
    fn shared_result() {
        let xml = include_str!("../fixtures/shared_result/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        assert_eq!(resolver.sources.len(), 3);

        let Object::Variable(variable) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(variable.identifier, "localA");

        let Object::Variable(variable) = resolver.get(4).unwrap() else { panic!() };
        assert_eq!(variable.identifier, "localB");

        let Object::BlockOutput(block, variable) = resolver.get(8).unwrap() else { panic!() };
        assert_eq!(block.type_name, "myAdd");
        assert_eq!(variable.parameter_name, "myAdd");
    }

    #[test]
    fn chained_calls() {
        let xml = include_str!("../fixtures/chained_calls/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        // Two data sources and two block outputs.
        assert_eq!(resolver.sources.len(), 4);

        let Object::Variable(a) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(a.identifier, "localA");
        let Object::Variable(b) = resolver.get(4).unwrap() else { panic!() };
        assert_eq!(b.identifier, "localB");

        let Object::BlockOutput(add, _) = resolver.get(6).unwrap() else { panic!() };
        assert_eq!(add.type_name, "myAdd");
        let Object::BlockOutput(mul, _) = resolver.get(10).unwrap() else { panic!() };
        assert_eq!(mul.type_name, "myMul");
    }

    #[test]
    fn nullary_call() {
        let xml = include_str!("../fixtures/nullary_call/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        // A function with no inputs: the only source is the block's return value.
        assert_eq!(resolver.sources.len(), 1);
        let Object::BlockOutput(block, _) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(block.type_name, "getOffset");
    }

    #[test]
    fn evaluation_order() {
        let xml = include_str!("../fixtures/evaluation_order/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        // Four data sources and two independent block outputs.
        assert_eq!(resolver.sources.len(), 6);

        let Object::BlockOutput(mul, _) = resolver.get(6).unwrap() else { panic!() };
        assert_eq!(mul.type_name, "myMul");
        let Object::BlockOutput(add, _) = resolver.get(14).unwrap() else { panic!() };
        assert_eq!(add.type_name, "myAdd");
    }

    #[test]
    fn negated_input() {
        // Negation is a transpiler concern; the resolver indexes the wiring as usual.
        let xml = include_str!("../fixtures/negated_input/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        assert_eq!(resolver.sources.len(), 3);
        let Object::Variable(a) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(a.identifier, "localA");
        let Object::BlockOutput(block, _) = resolver.get(6).unwrap() else { panic!() };
        assert_eq!(block.type_name, "myGate");
    }

    #[test]
    fn inout_variable() {
        let xml = include_str!("../fixtures/inout_variable/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        // The in-out's bound variable is an ordinary data source; the resolver only indexes
        // block output variables as sources, so the in-out pin itself isn't indexed.
        assert_eq!(resolver.sources.len(), 3);
        let Object::Variable(value) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(value.identifier, "localValue");
        let Object::Variable(sum) = resolver.get(4).unwrap() else { panic!() };
        assert_eq!(sum.identifier, "localSum");
        let Object::BlockOutput(block, _) = resolver.get(6).unwrap() else { panic!() };
        assert_eq!(block.type_name, "accumulate");
    }

    #[test]
    fn literal_input() {
        let xml = include_str!("../fixtures/literal_input/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        // A literal is modelled as a data source whose identifier is the literal text.
        assert_eq!(resolver.sources.len(), 3);
        let Object::Variable(literal) = resolver.get(4).unwrap() else { panic!() };
        assert_eq!(literal.identifier, "100");
        let Object::BlockOutput(block, _) = resolver.get(6).unwrap() else { panic!() };
        assert_eq!(block.type_name, "myAdd");
    }

    #[test]
    fn function_pou() {
        // The container POU is a FUNCTION; the resolver is POU-kind-agnostic and indexes the
        // network as usual — the function result is just a sink named after the function.
        let xml = include_str!("../fixtures/function_pou/myFunc.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        assert_eq!(resolver.sources.len(), 3);
        let Object::BlockOutput(block, _) = resolver.get(6).unwrap() else { panic!() };
        assert_eq!(block.type_name, "myAdd");
    }

    #[test]
    fn function_block_pou() {
        // The container POU is a FUNCTION_BLOCK; its VAR_OUTPUT is just a sink named after it.
        let xml = include_str!("../fixtures/function_block_pou/myFb.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        assert_eq!(resolver.sources.len(), 3);
        let Object::BlockOutput(block, _) = resolver.get(6).unwrap() else { panic!() };
        assert_eq!(block.type_name, "myAdd");
    }

    #[test]
    fn function_block_call() {
        // An FB-instance block: the resolver indexes its output like any block; the FB nature
        // (its `instanceName`) only matters during lowering.
        let xml = include_str!("../fixtures/function_block_call/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        assert_eq!(resolver.sources.len(), 2);
        let Object::Variable(step) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(step.identifier, "localStep");
        let Object::BlockOutput(block, variable) = resolver.get(4).unwrap() else { panic!() };
        assert_eq!(block.type_name, "Counter");
        assert_eq!(block.instance_name.as_deref(), Some("myInstance"));
        assert_eq!(variable.parameter_name, "count");
    }

    #[test]
    fn program_call() {
        // A program block carries no `instanceName`; the resolver indexes its output like any block.
        let xml = include_str!("../fixtures/program_call/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        assert_eq!(resolver.sources.len(), 2);
        let Object::Variable(increment) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(increment.identifier, "localIncrement");
        let Object::BlockOutput(block, variable) = resolver.get(4).unwrap() else { panic!() };
        assert_eq!(block.type_name, "auxProgram");
        assert_eq!(block.instance_name, None);
        assert_eq!(variable.parameter_name, "total");
    }

    #[test]
    fn unconnected_arguments_function() {
        // An unconnected pin still indexes normally; only the wired `localA` and the block's
        // return output are sources (the unconnected in-out's ConnectionPointOut is not indexed).
        let xml = include_str!("../fixtures/unconnected_arguments_function/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        assert_eq!(resolver.sources.len(), 2);
        let Object::Variable(a) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(a.identifier, "localA");
        let Object::BlockOutput(block, _) = resolver.get(5).unwrap() else { panic!() };
        assert_eq!(block.type_name, "myFunc");
    }

    #[test]
    fn unconnected_arguments_program() {
        // A standalone program block has no output; only the wired `localA` is a source.
        let xml = include_str!("../fixtures/unconnected_arguments_program/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        assert_eq!(resolver.sources.len(), 1);
        let Object::Variable(a) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(a.identifier, "localA");
    }

    #[test]
    fn unconnected_arguments_function_block() {
        // A standalone FB-instance block has no output; only the wired `localA` is a source.
        let xml = include_str!("../fixtures/unconnected_arguments_function_block/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        assert_eq!(resolver.sources.len(), 1);
        let Object::Variable(a) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(a.identifier, "localA");
    }

    #[test]
    fn unconnected_output_function() {
        // The wired result pin (id 4) is consumed by the sink; the unconnected `extra` pin (id 5)
        // is not, so the transpiler can lower it as an empty `extra => `.
        let xml = include_str!("../fixtures/unconnected_output_function/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        assert!(resolver.is_consumed(2)); // localA feeds the block input
        assert!(resolver.is_consumed(4)); // the result pin feeds the sink
        assert!(!resolver.is_consumed(5)); // `extra` feeds nothing
    }

    #[test]
    fn unconnected_output_program() {
        // The lone `result` output (id 4) is consumed by nothing.
        let xml = include_str!("../fixtures/unconnected_output_program/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        assert!(resolver.is_consumed(2)); // localA feeds the block input
        assert!(!resolver.is_consumed(4)); // `result` feeds nothing
    }

    #[test]
    fn unconnected_output_function_block() {
        let xml = include_str!("../fixtures/unconnected_output_function_block/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        assert!(resolver.is_consumed(2)); // localA feeds the block input
        assert!(!resolver.is_consumed(4)); // `result` feeds nothing
    }

    #[test]
    fn multiple_outputs() {
        let xml = include_str!("../fixtures/multiple_outputs/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        // Every block output is indexed as a source (two blocks, six outputs).
        let Object::BlockOutput(block, _) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(block.type_name, "myFunctionBlock");
        assert_eq!(block.instance_name.as_deref(), Some("myInstance"));

        // myInstance: a (2) and c (4) feed sinks; b (3) is unconnected.
        assert!(resolver.is_consumed(2));
        assert!(!resolver.is_consumed(3));
        assert!(resolver.is_consumed(4));
        // myFunction: the return (8) and b (10) feed nothing; a (9) feeds a sink.
        assert!(!resolver.is_consumed(8));
        assert!(resolver.is_consumed(9));
        assert!(!resolver.is_consumed(10));
    }

    #[test]
    fn conditional_return() {
        // The two data sources are indexed; the return consumes its condition wire just like the
        // sink consumes its value.
        let xml = include_str!("../fixtures/conditional_return/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        assert_eq!(resolver.sources.len(), 2);
        let Object::Variable(enable) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(enable.identifier, "enable");
        let Object::Variable(input) = resolver.get(5).unwrap() else { panic!() };
        assert_eq!(input.identifier, "input");

        assert!(resolver.is_consumed(2)); // enable feeds the return condition
        assert!(resolver.is_consumed(5)); // input feeds the sink
    }

    #[test]
    fn unconditional_return() {
        // The unconditional return has no condition wire, so it contributes nothing to the index;
        // only `input` (feeding the sink) is a source.
        let xml = include_str!("../fixtures/unconditional_return/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        assert_eq!(resolver.sources.len(), 1);
        let Object::Variable(input) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(input.identifier, "input");
        assert!(resolver.is_consumed(2)); // input feeds the sink
    }

    #[test]
    fn connector_continuation() {
        let xml = include_str!("../fixtures/connector_continuation/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        // The block's result is the only real source; the connector/continuation are not.
        assert_eq!(resolver.sources.len(), 1);
        let Object::BlockOutput(block, _) = resolver.get(12).unwrap() else { panic!() };
        assert_eq!(block.type_name, "alwaysFive");

        // The connector consumes the block's result (so it still earns a temp), and the
        // continuation's output (7) aliases through to it.
        assert!(resolver.is_consumed(12));
        assert_eq!(resolver.resolve_alias(7), 12);
    }

    #[test]
    fn connector_continuation_chain() {
        let xml = include_str!("../fixtures/connector_continuation_chain/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        // Only alwaysFive's result is a real source.
        assert_eq!(resolver.sources.len(), 1);
        let Object::BlockOutput(block, _) = resolver.get(10).unwrap() else { panic!() };
        assert_eq!(block.type_name, "alwaysFive");
        assert!(resolver.is_consumed(10));

        // Every hop resolves transitively back to the producer (wire 10).
        assert_eq!(resolver.resolve_alias(11), 10); // continuation a
        assert_eq!(resolver.resolve_alias(12), 10); // continuation b
        assert_eq!(resolver.resolve_alias(13), 10); // continuation c
        assert_eq!(resolver.resolve_alias(14), 10); // continuation d (the sink's wire)
    }

    // TODO: a cyclic chain currently panics; once this crate has an error story it should yield a
    // proper diagnostic instead (cf. plc_xml's E085). The test pins the present behaviour.
    #[test]
    #[should_panic(expected = "cyclic connector/continuation chain")]
    fn connector_continuation_cycle() {
        let xml = include_str!("../fixtures/connector_continuation_cycle/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::index(&deserialized);

        // x → y → x → ... never terminates.
        resolver.resolve_alias(10);
    }
}
