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
//!         <InputVariable parameterName="x">
//!             <ConnectionPointIn>
//!                 <Connection refConnectionPointOutId="1"/>
//!             </ConnectionPointIn>
//!         </InputVariable>
//!         <InputVariable parameterName="y">
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
//!              +--myAdd--+
//! localA------>|         |------>localResult
//! localB------>|         |
//!              +---------+
//! ```
//!
//! [`Resolver::index`] walks a network once, recording for every `connectionPointOutId`
//! the pin declaring it (a [`PinSource`]) and how many input pins reference it. During
//! lowering the transpiler resolves each `ConnectionPointIn` to its producer with
//! [`Resolver::resolve_input`] and consults [`Resolver::use_count`] to decide what a
//! block output becomes: inlined into its consumer (count 1), a standalone call
//! statement (count 0), or a not-yet-supported diagnostic (count > 1 — without
//! temp-variable lowering, inlining would duplicate the call).
//!
//! The resolver is pure graph topology and trusts the IDE: generated documents are
//! assumed well-formed — out-ids are unique, input pins have at most one connection,
//! references never dangle — so none of that is surfaced as errors. Debug assertions
//! trip on hand-written fixtures violating the contract. The only outcome besides a
//! producer is an unconnected pin, which resolves to `None`.

use std::collections::HashMap;

use crate::model::{Block, ConnectionPointIn, DataSource, FbdNetwork, FbdObject, OutputVariable};

/// What a `refConnectionPointOutId` resolves to: the element (and pin) producing the
/// value. Borrows from the network — the resolver owns no model data.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PinSource<'net> {
    /// A variable or literal read by other elements, in other words a [`DataSource`]
    Data(&'net DataSource),

    /// An output pin of a [`Block`]
    BlockOutput { block: &'net Block, pin: &'net OutputVariable },
}

pub struct Resolver<'net> {
    /// `connectionPointOutId` → the pin that declares it.
    sources: HashMap<u64, PinSource<'net>>,
    /// `connectionPointOutId` → number of references from input pins (plain and
    /// feedback connections alike).
    use_counts: HashMap<u64, usize>,
}

impl<'net> Resolver<'net> {
    /// Walks the network once: every output pin goes into the source map, every
    /// input-pin reference bumps the use count.
    pub fn index(network: &'net FbdNetwork) -> Self {
        let mut resolver = Resolver { sources: HashMap::new(), use_counts: HashMap::new() };

        for object in &network.objects {
            match object {
                // A variable or literal read by other elements (producer, "foo" --> <other element>)
                FbdObject::DataSource(source) => {
                    if let Some(out) = &source.connection_point_out {
                        resolver.declare(out.id, PinSource::Data(source));
                    }
                }

                // A function call
                FbdObject::Block(block) => {
                    for pin in block.input_variables.iter().flat_map(|inputs| &inputs.variables) {
                        resolver.count_uses(pin.connection_point_in.as_ref());
                    }

                    for pin in block.output_variables.iter().flat_map(|outputs| &outputs.variables) {
                        if let Some(out) = &pin.connection_point_out {
                            resolver.declare(out.id, PinSource::BlockOutput { block, pin });
                        }
                    }

                    // in-out pins consume like inputs; their out side needs the deferred
                    // `PinSource::InOut` arm before it can be declared
                    for pin in block.in_out_variables.iter().flat_map(|in_outs| &in_outs.variables) {
                        resolver.count_uses(pin.connection_point_in.as_ref());
                    }
                }

                // A variable assigned by another element (consumer, <other element> --> foo)
                FbdObject::DataSink(sink) => resolver.count_uses(sink.connection_point_in.as_ref()),

                FbdObject::Jump(jump) => resolver.count_uses(jump.connection_point_in.as_ref()),

                FbdObject::Return(ret) => resolver.count_uses(ret.connection_point_in.as_ref()),

                // its out side is an explicitly producer-less pin — deferred with its arm
                FbdObject::Unconnected(unconnected) => {
                    resolver.count_uses(unconnected.connection_point_in.as_ref())
                }
            }
        }

        resolver
    }

    /// The producer behind an out-ID. Mostly an implementation detail of
    /// [`Self::resolve_input`], public for direct lookups.
    pub fn resolve(&self, id: u64) -> Option<&PinSource<'net>> {
        self.sources.get(&id)
    }

    /// The upstream producer of an input pin; `None` means the pin is unconnected (no
    /// `ConnectionPointIn` element, or one without a `Connection`). Feedback
    /// connections are not resolved here — the transpiler rejects them as unsupported
    /// before asking.
    pub fn resolve_input(&self, pin: Option<&ConnectionPointIn>) -> Option<&PinSource<'net>> {
        let connection = pin?.connections.first()?;
        let source = self.resolve(connection.ref_connection_point_out_id);
        debug_assert!(
            source.is_some(),
            "no pin declares out-id {}: the IDE guarantees wiring integrity",
            connection.ref_connection_point_out_id
        );
        source
    }

    /// How many input pins consume this out-ID (0 = nobody does).
    pub fn use_count(&self, id: u64) -> usize {
        self.use_counts.get(&id).copied().unwrap_or(0)
    }

    fn declare(&mut self, id: u64, source: PinSource<'net>) {
        let previous = self.sources.insert(id, source);
        debug_assert!(previous.is_none(), "out-id {id} declared twice: the IDE guarantees unique ids");
    }

    fn count_uses(&mut self, pin: Option<&ConnectionPointIn>) {
        let Some(pin) = pin else { return };
        let plain = pin.connections.iter().map(|c| c.ref_connection_point_out_id);
        let feedback = pin.feedback_connections.iter().map(|c| c.ref_connection_point_out_id);
        for id in plain.chain(feedback) {
            *self.use_counts.entry(id).or_default() += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{self, BodyContent, Network};

    const SIMPLE_FUNCTION_CALL: &str = include_str!("../fixtures/simple_function_call/myMain.cfc");

    /// The single network of a program's FBD body.
    fn network_of(xml: &str) -> FbdNetwork {
        let pou = model::from_str(xml).unwrap();
        let BodyContent::Fbd(fbd) = &pou.main_body().unwrap().body_content[0];
        let Network::Fbd(network) = &fbd.networks[0];
        network.clone()
    }

    fn sinks(network: &FbdNetwork) -> Vec<&model::DataSink> {
        network
            .objects
            .iter()
            .filter_map(|object| match object {
                FbdObject::DataSink(sink) => Some(sink),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn indexes_the_simple_function_call_fixture() {
        let network = network_of(SIMPLE_FUNCTION_CALL);
        let resolver = Resolver::index(&network);

        // localA -> 1, localB -> 2, myAdd's result pin -> 3
        let Some(PinSource::Data(source)) = resolver.resolve(1) else {
            panic!("expected localA behind id 1");
        };
        assert_eq!(source.identifier, "localA");

        let Some(PinSource::Data(source)) = resolver.resolve(2) else {
            panic!("expected localB behind id 2");
        };
        assert_eq!(source.identifier, "localB");

        let Some(PinSource::BlockOutput { block, pin }) = resolver.resolve(3) else {
            panic!("expected the myAdd result pin behind id 3");
        };
        assert_eq!(block.type_name, "myAdd");
        assert_eq!(pin.parameter_name, "myAdd");

        assert_eq!(resolver.resolve(4), None);

        // every pin has exactly one consumer
        assert_eq!(resolver.use_count(1), 1);
        assert_eq!(resolver.use_count(2), 1);
        assert_eq!(resolver.use_count(3), 1);
        assert_eq!(resolver.use_count(4), 0);
    }

    #[test]
    fn resolves_the_sinks_input_to_the_block_output() {
        let network = network_of(SIMPLE_FUNCTION_CALL);
        let resolver = Resolver::index(&network);

        let all_sinks = sinks(&network);
        let [sink] = all_sinks.as_slice() else {
            panic!("expected exactly one sink");
        };
        let source = resolver.resolve_input(sink.connection_point_in.as_ref()).unwrap();
        assert!(matches!(source, PinSource::BlockOutput { block, .. } if block.type_name == "myAdd"));
    }

    /// Pins without a `ConnectionPointIn` (or without a `Connection` inside it) have no
    /// producer; everything beyond that is guaranteed well-formed by the IDE.
    #[test]
    fn unconnected_input_pins_resolve_to_none() {
        let xml = r#"
            <Program name="prog">
                <MainBody>
                    <BodyContent xsi:type="FBD" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                        <Network xsi:type="FbdNetwork" evaluationOrder="1">
                            <FbdObject xsi:type="DataSource" identifier="a">
                                <ConnectionPointOut connectionPointOutId="1"/>
                            </FbdObject>
                            <FbdObject xsi:type="DataSink" identifier="noConnectionPointIn"/>
                            <FbdObject xsi:type="DataSink" identifier="emptyConnectionPointIn">
                                <ConnectionPointIn/>
                            </FbdObject>
                        </Network>
                    </BodyContent>
                </MainBody>
            </Program>"#;

        let network = network_of(xml);
        let resolver = Resolver::index(&network);

        let sinks = sinks(&network);
        assert!(resolver.resolve_input(sinks[0].connection_point_in.as_ref()).is_none());
        assert!(resolver.resolve_input(sinks[1].connection_point_in.as_ref()).is_none());
    }

    /// `use_count` counts every consuming pin — plain and feedback connections alike.
    #[test]
    fn use_count_counts_every_consumer() {
        let xml = r#"
            <Program name="prog">
                <MainBody>
                    <BodyContent xsi:type="FBD" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                        <Network xsi:type="FbdNetwork" evaluationOrder="1">
                            <FbdObject xsi:type="DataSource" identifier="a">
                                <ConnectionPointOut connectionPointOutId="1"/>
                            </FbdObject>
                            <FbdObject xsi:type="DataSink" identifier="x">
                                <ConnectionPointIn>
                                    <Connection refConnectionPointOutId="1"/>
                                </ConnectionPointIn>
                            </FbdObject>
                            <FbdObject xsi:type="DataSink" identifier="y">
                                <ConnectionPointIn>
                                    <Connection refConnectionPointOutId="1"/>
                                </ConnectionPointIn>
                            </FbdObject>
                            <FbdObject xsi:type="Block" typeName="next">
                                <InputVariables>
                                    <InputVariable parameterName="in">
                                        <ConnectionPointIn>
                                            <FeedbackConnection refConnectionPointOutId="1" feedbackVariable="loop"/>
                                        </ConnectionPointIn>
                                    </InputVariable>
                                </InputVariables>
                            </FbdObject>
                        </Network>
                    </BodyContent>
                </MainBody>
            </Program>"#;

        let network = network_of(xml);
        let resolver = Resolver::index(&network);
        assert_eq!(resolver.use_count(1), 3);
    }
}
