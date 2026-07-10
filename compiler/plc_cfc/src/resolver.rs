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
    // The `OutputVariable` field is only read in tests, hence the allow
    #[allow(dead_code)]
    BlockOutput(Block, OutputVariable),
}

impl Resolver {
    pub fn resolve(pou: &Pou) -> Resolver {
        let mut sources = HashMap::new();
        let mut consumed = HashSet::new();
        let mut aliases = HashMap::new();

        let network = pou.network();
        for object in &network.objects {
            match object {
                // A variable representing the left side of a connection, e.g. `foo (source) ---> bar (sink)`
                FbdObject::DataSource(source) => {
                    sources.insert(source.connection_out, Object::Variable(source.clone()));
                }

                // A variable representing the right side of a connection, e.g. `foo (source) ---> bar (sink)`
                FbdObject::DataSink(sink) => {
                    consumed.extend(sink.connection_in);
                }

                // A block, representing a POU call
                FbdObject::Block(block) => {
                    for variable in block.inputs.iter().chain(&block.inouts) {
                        consumed.extend(variable.connection_in);
                    }

                    for variable in &block.outputs {
                        sources.insert(
                            variable.connection_out,
                            Object::BlockOutput(block.clone(), variable.clone()),
                        );
                    }
                }

                // A conditional return, e.g. `foo (source) ---> RETURN`
                FbdObject::Return(ret) => {
                    consumed.extend(ret.connection_in);
                }

                // A conditional jump, e.g. `foo (source) ---> JMP skip` (essentially a GOTO)
                FbdObject::Jump(_) => unimplemented!("CFC jumps are not yet supported"),

                // Nothing to do
                FbdObject::Unconnected(_) => (),
            };
        }

        // An "invisible" jump from one element to another, e.g. `foo (Connector) ---> foo (Continuation)`
        let mut connector_inputs: HashMap<&str, u64> = HashMap::new();
        for common in &network.common_objects {
            if let CommonObject::Connector(connector) = common
                && let Some(id) = connector.connection_in
            {
                connector_inputs.insert(&connector.label, id);
                consumed.insert(id);
            }
        }

        // An "invisible" jump from one element to another, e.g. `foo (Connector) ---> foo (Continuation)`
        for common in &network.common_objects {
            if let CommonObject::Continuation(continuation) = common
                && let Some(&in_id) = connector_inputs.get(continuation.label.as_str())
            {
                aliases.insert(continuation.connection_out, in_id);
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

    /// Returns true if the given id, after resolving any aliases, points to a source object in the network.
    pub fn is_resolvable(&self, id: u64) -> bool {
        self.get(self.resolve_alias(id)).is_some()
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
        //                      +-------- myAdd --------+  (1)
        //    localA  --------->| in1              myAdd|--------->  localResult  (2)
        //    localB  --------->| in2                   |
        //                      +-----------------------+
        //
        //    (1),(2)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/function_call/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 3);

        let Object::Variable(variable) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(variable.global_id, 1);
        assert_eq!(variable.identifier, "localA");

        let Object::Variable(variable) = resolver.get(4).unwrap() else { panic!() };
        assert_eq!(variable.global_id, 3);
        assert_eq!(variable.identifier, "localB");

        let Object::BlockOutput(block, variable) = resolver.get(8).unwrap() else { panic!() };
        assert_eq!(block.type_name, "myAdd");
        assert_eq!(variable.parameter_name, "myAdd");
        assert_eq!(variable.connection_out, 8);
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
        //                      +----- myAdd -----+ (2)       +----- myMul -----+ (3)
        //    localA  --------->| in1       myAdd |---------->| IN1       myMul |------->  localResultA  (4)
        //    localB  --+------>| in2             |       +-->| IN2             |
        //              |       +-----------------+       |   +-----------------+
        //              +-------------------------------- +
        //
        //    (2),(3),(4)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/chained_calls/mainProgram.cfc");
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
        //                     +--- getOffset ---+ (1)
        //                     |       getOffset |--->  localResult  (2)
        //                     +-----------------+
        //
        //    (1),(2)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/nullary_call/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 1);
        let Object::BlockOutput(block, _) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(block.type_name, "getOffset");
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
        //                      +----- myGate -----+ (1)
        //    localA  --o------>| a         myGate |--->  localResult  (2)
        //    localB  --------->| b                |
        //                      +------------------+
        //
        //    o        a negated input pin (wraps its value in NOT)
        //    (1),(2)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/negated_input/mainProgram.cfc");
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
        //                       +---- accumulate ----+ (1)
        //    localValue  ------>| value              |
        //                       |          accumulate|--->  localResult  (2)
        //    localSum  <------->| sum                |
        //                       +--------------------+
        //
        //    <-->     an in-out pin (passed by reference)
        //    (1),(2)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/inout_variable/mainProgram.cfc");
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
        //                      +----- myAdd -----+ (1)
        //    localA  --------->| in1       myAdd |--->  localResult  (2)
        //    100     --------->| in2             |
        //                      +-----------------+
        //
        //    (1),(2)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/literal_input/mainProgram.cfc");
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
        //    localA + 5  ----------->  result   (0)
        //
        //    (0)  evaluation-priority badge shown by the IDE
        let xml = include_str!("../fixtures/valid/expression_source/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 1);
        let Object::Variable(expr) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(expr.identifier, "localA + 5");
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
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 3);
        let Object::BlockOutput(block, _) = resolver.get(6).unwrap() else { panic!() };
        assert_eq!(block.type_name, "myAdd");
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
        //                   +-- function_block_0.myAction --+ (0)
        //    myInstance --->|                               |
        //                   +-------------------------------+
        //
        //    function_block_0.myAction   the action, called on instance myInstance
        //    (0)                         evaluation-priority badge shown by the IDE
        let xml = include_str!("../fixtures/valid/action_call/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 0);
    }

    #[test]
    fn program_call() {
        //                        +----- auxProgram -----+ (1)
        //    localIncrement ---->| increment      total |---->  localTotal  (2)
        //                        +----------------------+
        //
        //    (1),(2)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/program_call/mainProgram.cfc");
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
        //                   +------ myFunc ------+ (1)
        //    localA ------->| a           myFunc |------->  localResult  (2)
        //                   | b  (unconnected)   |
        //                   | io (unconnected)   |
        //                   +--------------------+
        //
        //    (unconnected)  a pin with no incoming wire
        //    (1),(2)        evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/unconnected_arguments_function/mainProgram.cfc");
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
        //                       +---- auxProgram ----+ (1)
        //    localA ----------->| a                  |
        //                       | b  (unconnected)   |
        //                       | io (unconnected)   |
        //                       +--------------------+
        //
        //    (unconnected)  a pin with no incoming wire
        //    (1)            evaluation-priority badge shown by the IDE
        let xml = include_str!("../fixtures/valid/unconnected_arguments_program/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 1);
        let Object::Variable(a) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(a.identifier, "localA");
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
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert!(resolver.is_consumed(2));
        assert!(resolver.is_consumed(4));
        assert!(!resolver.is_consumed(5));
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
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert!(resolver.is_consumed(2));
        assert!(!resolver.is_consumed(4));
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
        //    enable  --o--->| RETURN |  (0)
        //
        //    input   ------>  result    (1)
        //
        //    --o-->   a negated condition wire (returns when enable is FALSE)
        //    (0),(1)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/conditional_return/mainProgram.cfc");
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
        //    input  ------>  result    (0)
        //
        //                   | RETURN |  (1)
        //
        //    (no wire into RETURN -> unconditional)
        //    (0),(1)  evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/unconditional_return/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        assert_eq!(resolver.sources.len(), 1);
        let Object::Variable(input) = resolver.get(2).unwrap() else { panic!() };
        assert_eq!(input.identifier, "input");
        assert!(resolver.is_consumed(2));
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
        //    +-- alwaysFive --+ (0)
        //    |      alwaysFive|--id 10-->[Conn a]   [Cont a]--id 11-->[Conn b]   [Cont b]--id 12-->[Conn c]
        //    +----------------+                                                              |
        //         [Cont c]--id 13-->[Conn d]   [Cont d]--id 14-->  result  (1)  <------------+
        //
        //    a,b,c,d   labels matching each connector to its continuation
        //    (0),(1)   evaluation-priority badges shown by the IDE
        let xml = include_str!("../fixtures/valid/connector_continuation_chain/mainProgram.cfc");
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
        //    [Cont y]--id 11-->[Conn x]   [Cont x]--id 10-->[Conn y]   [Cont x]--id 10-->  result  (0)
        //         ^                                              |
        //         +----------------------------------------------+   (y feeds x feeds y ...)
        //
        //    x,y   labels; the two pairs reference each other's output
        //    (0)   evaluation-priority badge shown by the IDE
        let xml = include_str!("../fixtures/invalid/connector_continuation_cycle/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        // A cyclic alias chain terminates at its entry instead of looping or panicking.
        assert_eq!(resolver.resolve_alias(10), 10);
    }

    #[test]
    #[should_panic(expected = "CFC jumps are not yet supported")]
    fn jump_is_unsupported() {
        //    enable  ------>| JMP skip |  (0)
        //
        //    input   ------>  result      (1)
        //
        //    JMP skip   an (unsupported) conditional jump; resolving the network panics
        let xml = include_str!("../fixtures/unsupported/jump/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let _ = Resolver::resolve(&deserialized);
    }

    #[test]
    fn dangling_connection_is_not_resolvable() {
        //    localA  -->      ??? --id 999-->  result  (0)
        //
        //    the sink's wire references id 999, but the only producer is localA at id 2
        let xml = include_str!("../fixtures/invalid/dangling_connection/mainProgram.cfc");
        let deserialized = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&deserialized);

        // A real producer id resolves; the sink's dangling reference does not.
        assert!(resolver.is_resolvable(2));
        assert!(!resolver.is_resolvable(999));
    }
}
