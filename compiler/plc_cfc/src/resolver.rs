//! Indexes a CFC network's wiring ahead of transpilation.
//!
//! The transpiler walks the network one object at a time, but lowering a single object constantly requires
//! knowledge about the rest of the graph: what produced the value on a given wire, whether anyone consumes a
//! block's output, and where a connector/continuation pair really leads. Answering those from scratch every
//! time would mean rescanning the whole network over and over.
//!
//! The [`Resolver`] does that scan once, up front, and records the answers by ID:
//!
//! - `sources` maps a producing ID to the [`Object`] behind it: a free-standing variable, or a block output.
//! - `consumed` is the set of IDs that something reads, which lets the transpiler tell whether a block output
//!   is used (and so needs a temporary) or can be dropped.
//! - `aliases` collapses connector/continuation pairs, mapping a continuation's output ID back to the
//!   connector's input ID so the rest of the code can look straight through them.

use std::collections::{HashMap, HashSet};

use crate::model::{Block, CommonObject, DataSource, FbdObject, OutputVariable, Pou};

/// A one-time index of a network's wiring. See the module documentation for an overview.
#[derive(Debug)]
pub struct Resolver {
    /// A map of IDs and their object representing a data source, i.e. a free-standing variable or an output
    /// variable of a block.
    sources: HashMap<u64, Object>,

    /// A set of IDs that have been consumed, e.g. a variable fed into a block as an input variable
    consumed: HashSet<u64>,

    /// A map of resolved connector/continuation objects, where the key is the out and the value is the
    /// input ID. For example the key-value pair in the map would yield `(2, 1)` for the given example
    /// ```text
    /// a(1) ---> jmp (connector)
    /// jmp (continuation) ---> b(2)
    /// ```
    aliases: HashMap<u64, u64>,
}

#[derive(Debug)]
pub enum Object {
    Variable(DataSource),
    BlockOutput(Block, OutputVariable),
}

impl Resolver {
    pub fn resolve(pou: &Pou) -> Resolver {
        let mut sources = HashMap::new();
        let mut consumed = HashSet::new();
        let mut aliases = HashMap::new();

        // Nothing to do if the network is empty
        let Some(network) = pou.get_network() else {
            return Resolver { sources, consumed, aliases };
        };

        for object in &network.objects {
            match object {
                // A variable representing the left side of a connection, e.g. `foo (source) ---> bar (sink)`
                FbdObject::DataSource(source) => {
                    if let Some(out) = &source.connection_point_out {
                        sources.insert(out.id, Object::Variable(source.clone()));
                    }
                }

                // A variable representing the right side of a connection, e.g. `foo (source) ---> bar (sink)`
                FbdObject::DataSink(sink) => {
                    if let Some(pin) = &sink.connection_point_in {
                        consumed.extend(pin.connections.iter().map(|c| c.ref_connection_point_out_id));
                    }
                }

                // A block, representing a POU call
                FbdObject::Block(block) => {
                    for variable in block.get_input_variables() {
                        consumed.extend(variable.get_referenced_argument_id());
                    }

                    for variable in block.get_inout_variables() {
                        consumed.extend(variable.get_referenced_argument_id());
                    }

                    for variable in block.output_variables.iter().flat_map(|opt| &opt.variables) {
                        if let Some(out) = &variable.connection_point_out {
                            sources.insert(out.id, Object::BlockOutput(block.clone(), variable.clone()));
                        }
                    }
                }

                // A conditional return, e.g. `foo (source) ---> RETURN`
                FbdObject::Return(ret) => {
                    if let Some(pin) = &ret.connection_point_in
                        && let Some(connection) = pin.connections.first()
                    {
                        consumed.insert(connection.ref_connection_point_out_id);
                    }
                }

                _ => (),
            };
        }

        // An "invisible" jump from one element to another, e.g. `foo (Connector) ---> foo (Continuation)`
        let mut connector_inputs: HashMap<&str, u64> = HashMap::new();
        for common in &network.common_objects {
            if let CommonObject::Connector(connector) = common
                && let Some(id) = connector.get_referenced_argument_id()
            {
                connector_inputs.insert(&connector.label, id);
                consumed.insert(id);
            }
        }

        // An "invisible" jump from one element to another, e.g. `foo (Connector) ---> foo (Continuation)`
        for common in &network.common_objects {
            if let CommonObject::Continuation(continuation) = common
                && let (Some(&in_id), Some(out_id)) = (
                    connector_inputs.get(continuation.label.as_str()),
                    continuation.get_connection_point_out_id(),
                )
            {
                aliases.insert(out_id, in_id);
            }
        }

        Resolver { sources, consumed, aliases }
    }

    /// Returns a source object with the given id, if any
    pub fn get(&self, id: u64) -> Option<&Object> {
        self.sources.get(&id)
    }

    /// Returns true if the given id has been consumed by another object. For example the id of a variable
    /// (data source) consumed by another variable (data sink).
    pub fn is_consumed(&self, id: u64) -> bool {
        self.consumed.contains(&id)
    }

    /// Returns the direct connection to another object, eliminating any "noise" in between. For example
    /// ```text
    /// x (source) ---> jmp (connector)
    /// jmp (continuation) ---> y (sink)
    /// ```
    /// which would yield `y := x`.
    pub fn resolve_alias(&self, mut id: u64) -> u64 {
        let mut visited = HashSet::new();
        while let Some(&next) = self.aliases.get(&id) {
            if !visited.insert(id) {
                break;
            }

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
        let resolver = Resolver::resolve(&deserialized);

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
        let resolver = Resolver::resolve(&deserialized);

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
        let resolver = Resolver::resolve(&deserialized);

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
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 1);
        let Object::BlockOutput(block, _) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(block.type_name, "getOffset");
    }

    #[test]
    fn evaluation_order() {
        let xml = include_str!("../fixtures/evaluation_order/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 6);

        let Object::BlockOutput(mul, _) = resolver.get(6).unwrap() else { panic!() };
        assert_eq!(mul.type_name, "myMul");
        let Object::BlockOutput(add, _) = resolver.get(14).unwrap() else { panic!() };
        assert_eq!(add.type_name, "myAdd");
    }

    #[test]
    fn negated_input() {
        let xml = include_str!("../fixtures/negated_input/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

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
        let resolver = Resolver::resolve(&deserialized);

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
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 3);
        let Object::Variable(literal) = resolver.get(4).unwrap() else { panic!() };
        assert_eq!(literal.identifier, "100");
        let Object::BlockOutput(block, _) = resolver.get(6).unwrap() else { panic!() };
        assert_eq!(block.type_name, "myAdd");
    }

    #[test]
    fn expression_source() {
        let xml = include_str!("../fixtures/expression_source/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 1);
        let Object::Variable(expr) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(expr.identifier, "localA + 5");
    }

    #[test]
    fn function_pou() {
        let xml = include_str!("../fixtures/function_pou/myFunc.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 3);
        let Object::BlockOutput(block, _) = resolver.get(6).unwrap() else { panic!() };
        assert_eq!(block.type_name, "myAdd");
    }

    #[test]
    fn function_block_call() {
        let xml = include_str!("../fixtures/function_block_call/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 2);
        let Object::Variable(step) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(step.identifier, "localStep");
        let Object::BlockOutput(block, variable) = resolver.get(4).unwrap() else { panic!() };
        assert_eq!(block.type_name, "Counter");
        assert_eq!(block.instance_name.as_deref(), Some("myInstance"));
        assert_eq!(variable.parameter_name, "count");
    }

    #[test]
    fn action_call() {
        let xml = include_str!("../fixtures/action_call/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 0);
    }

    #[test]
    fn program_call() {
        let xml = include_str!("../fixtures/program_call/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

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
        let xml = include_str!("../fixtures/unconnected_arguments_function/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 2);
        let Object::Variable(a) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(a.identifier, "localA");
        let Object::BlockOutput(block, _) = resolver.get(5).unwrap() else { panic!() };
        assert_eq!(block.type_name, "myFunc");
    }

    #[test]
    fn unconnected_arguments_program() {
        let xml = include_str!("../fixtures/unconnected_arguments_program/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 1);
        let Object::Variable(a) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(a.identifier, "localA");
    }

    #[test]
    fn unconnected_output_function() {
        let xml = include_str!("../fixtures/unconnected_output_function/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert!(resolver.is_consumed(2));
        assert!(resolver.is_consumed(4));
        assert!(!resolver.is_consumed(5));
    }

    #[test]
    fn unconnected_output_program() {
        let xml = include_str!("../fixtures/unconnected_output_program/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert!(resolver.is_consumed(2));
        assert!(!resolver.is_consumed(4));
    }

    #[test]
    fn multiple_outputs() {
        let xml = include_str!("../fixtures/multiple_outputs/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        let Object::BlockOutput(block, _) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(block.type_name, "myFunctionBlock");
        assert_eq!(block.instance_name.as_deref(), Some("myInstance"));

        assert!(resolver.is_consumed(2));
        assert!(!resolver.is_consumed(3));
        assert!(resolver.is_consumed(4));
        assert!(!resolver.is_consumed(8));
        assert!(resolver.is_consumed(9));
        assert!(!resolver.is_consumed(10));
    }

    #[test]
    fn conditional_return() {
        let xml = include_str!("../fixtures/conditional_return/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 2);
        let Object::Variable(enable) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(enable.identifier, "enable");
        let Object::Variable(input) = resolver.get(5).unwrap() else { panic!() };
        assert_eq!(input.identifier, "input");

        assert!(resolver.is_consumed(2));
        assert!(resolver.is_consumed(5));
    }

    #[test]
    fn unconditional_return() {
        let xml = include_str!("../fixtures/unconditional_return/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 1);
        let Object::Variable(input) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(input.identifier, "input");
        assert!(resolver.is_consumed(2));
    }

    #[test]
    fn connector_continuation() {
        let xml = include_str!("../fixtures/connector_continuation/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 1);
        let Object::BlockOutput(block, _) = resolver.get(12).unwrap() else { panic!() };
        assert_eq!(block.type_name, "alwaysFive");

        assert!(resolver.is_consumed(12));
        assert_eq!(resolver.resolve_alias(7), 12);
    }

    #[test]
    fn connector_continuation_chain() {
        let xml = include_str!("../fixtures/connector_continuation_chain/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 1);
        let Object::BlockOutput(block, _) = resolver.get(10).unwrap() else { panic!() };
        assert_eq!(block.type_name, "alwaysFive");
        assert!(resolver.is_consumed(10));

        assert_eq!(resolver.resolve_alias(11), 10);
        assert_eq!(resolver.resolve_alias(12), 10);
        assert_eq!(resolver.resolve_alias(13), 10);
        assert_eq!(resolver.resolve_alias(14), 10);
    }

    #[test]
    fn connector_continuation_cycle() {
        let xml = include_str!("../fixtures/connector_continuation_cycle/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        // A cyclic alias chain terminates at its entry instead of looping or panicking.
        assert_eq!(resolver.resolve_alias(10), 10);
    }
}
