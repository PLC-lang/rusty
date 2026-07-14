//! Structural resolution of an FBD network: classify each element, link every
//! sink and return to the concrete source that feeds it, and order the
//! statements by evaluation priority.
//!
//! Connector/continuation pairs are spliced out during linking: they carry no
//! statement of their own, they only reroute what a consumer ultimately reads.

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use crate::model::{FbdObject, Network, Pin};

pub(crate) struct Resolved<'model> {
    /// Network statements in evaluation-priority order.
    pub statements: Vec<Statement<'model>>,
    pub unconnected: Vec<&'model FbdObject>,
    /// Connectors that repeat a label already claimed by an earlier one.
    pub duplicate_connectors: Vec<&'model FbdObject>,
    /// Connectors/continuations where a consumed chain never reached a source.
    pub broken: Vec<Broken<'model>>,
    /// Producer output-pin ids some consumer reads. A block output absent here is
    /// unread — a stateless function then needs no captured temporary for it.
    pub consumed_outputs: HashSet<usize>,
}

pub(crate) enum Statement<'model> {
    Assignment { sink: &'model FbdObject, source: Source<'model> },
    Return { object: &'model FbdObject, condition: Option<Source<'model>> },
    Call { block: &'model FbdObject, arguments: Vec<Argument<'model>> },
}

/// A block input/in_out pin paired with the value that feeds it.
pub(crate) struct Argument<'model> {
    pub pin: &'model Pin,
    pub source: Source<'model>,
}

/// The value a consumer reads: a plain variable/literal source, or one output
/// pin of a block — a block output is read back as a member of its instance.
pub(crate) enum Source<'model> {
    Variable(&'model FbdObject),
    Output { block: &'model FbdObject, pin: &'model Pin },
}

pub(crate) struct Broken<'model> {
    pub element: &'model FbdObject,
    pub reason: BrokenReason,
}

pub(crate) enum BrokenReason {
    OpenConnector,
    DanglingContinuation,
}

enum Role {
    Source,
    Sink,
    Return,
    Block,
    Connector,
    Continuation,
    Unconnected,
    Other,
}

enum Trace<'model> {
    Reached(Source<'model>),
    DeadEnd(Broken<'model>),
    Unwired,
}

/// The producers a consumer's wire can land on, indexed by output-pin id.
struct Producers<'model> {
    /// Every output pin (data source, continuation, or block) back to its owner,
    /// used to walk a wire to its origin.
    by_pin: HashMap<usize, &'model FbdObject>,
    /// Block output pins only, so a landed wire recovers the exact pin (its
    /// parameter name and inversion), not just the owning block.
    block_output: HashMap<usize, &'model Pin>,
    connector_by_label: HashMap<&'model str, &'model FbdObject>,
}

impl<'model> Statement<'model> {
    /// The element carrying this statement's priority and diagram location.
    fn anchor(&self) -> &'model FbdObject {
        match self {
            Statement::Assignment { sink, .. } => sink,
            Statement::Return { object, .. } => object,
            Statement::Call { block, .. } => block,
        }
    }
}

impl<'model> Source<'model> {
    /// The underlying data source, when this is a plain variable/literal rather
    /// than a block output. Used by validation to vet free-text identifiers.
    pub(crate) fn variable(&self) -> Option<&'model FbdObject> {
        match self {
            Source::Variable(object) => Some(object),
            Source::Output { .. } => None,
        }
    }
}

pub(crate) fn resolve(network: &Network) -> Resolved<'_> {
    let (producers, duplicate_connectors) = index(network);

    // Link each consumer to its source: a sink becomes an assignment, a return
    // a (guarded) return, a block a call over its wired inputs. A chain that
    // never reaches a source is set aside for validation; connectors and
    // continuations are pure routing and emit nothing.
    let mut statements = Vec::new();
    let mut unconnected = Vec::new();
    let mut broken = Vec::new();
    let mut consumed_outputs = HashSet::new();
    for object in network.elements() {
        match role(object) {
            Role::Sink => match trace(source_pin(object), &producers) {
                Trace::Reached(source) => {
                    record_consumed(&source, &mut consumed_outputs);
                    statements.push(Statement::Assignment { sink: object, source });
                }
                Trace::DeadEnd(at) => broken.push(at),
                Trace::Unwired => {} // an unwired sink assigns nothing; drop it
            },

            Role::Return => match trace(source_pin(object), &producers) {
                Trace::Reached(source) => {
                    record_consumed(&source, &mut consumed_outputs);
                    statements.push(Statement::Return { object, condition: Some(source) });
                }
                Trace::DeadEnd(at) => broken.push(at),
                // No wire means no guard; validation flags the bare return (E085).
                Trace::Unwired => statements.push(Statement::Return { object, condition: None }),
            },

            // A block always emits its call so its state advances; only its wired
            // inputs and in_outs become arguments (an unwired input keeps last
            // cycle's value). Outputs are read back separately, as members.
            Role::Block => {
                let mut arguments = Vec::new();
                for pin in object.input_pins().iter().chain(object.inout_pins()) {
                    match trace(pin.source_pin(), &producers) {
                        Trace::Reached(source) => {
                            record_consumed(&source, &mut consumed_outputs);
                            arguments.push(Argument { pin, source });
                        }
                        Trace::DeadEnd(at) => broken.push(at),
                        Trace::Unwired => {}
                    }
                }
                statements.push(Statement::Call { block: object, arguments });
            }

            Role::Unconnected => unconnected.push(object),
            Role::Source | Role::Connector | Role::Continuation | Role::Other => {}
        }
    }

    // Statements run in evaluation-priority order; a break fanned out across
    // several consumers points at one element, so report it just once.
    statements.sort_by_key(|statement| statement.anchor().priority().unwrap_or(usize::MAX));
    let mut seen = HashSet::new();
    broken.retain(|at| seen.insert(at.element.global_id));

    Resolved { statements, unconnected, duplicate_connectors, broken, consumed_outputs }
}

/// Note a block output as read, so an unread function output can skip its temp.
fn record_consumed(source: &Source, consumed: &mut HashSet<usize>) {
    if let Source::Output { pin, .. } = source {
        if let Some(id) = pin.output_pin() {
            consumed.insert(id);
        }
    }
}

/// Index the network so the linking pass is pure lookup: map every output pin to
/// its producer (and, for blocks, the exact pin), and claim connector labels.
fn index(network: &Network) -> (Producers<'_>, Vec<&FbdObject>) {
    let mut by_pin = HashMap::new();
    let mut block_output = HashMap::new();
    let mut connector_by_label = HashMap::new();
    let mut duplicate_connectors = Vec::new();

    for object in network.elements() {
        // A data source or continuation exposes a single output pin; a block
        // exposes one per output parameter. Map each back so a consumer wired to
        // the pin can find what drives it.
        if let Some(out) = &object.connection_out {
            by_pin.insert(out.id, object);
        }
        if matches!(role(object), Role::Block) {
            for pin in object.output_pins() {
                if let Some(id) = pin.output_pin() {
                    by_pin.insert(id, object);
                    block_output.insert(id, pin);
                }
            }
        }

        // A connector names a source via its label; the first to claim a label
        // owns it, and any later connector reusing it is a duplicate (E081).
        if matches!(role(object), Role::Connector) {
            if let Some(label) = object.label() {
                match connector_by_label.entry(label) {
                    Entry::Vacant(entry) => {
                        entry.insert(object);
                    }
                    Entry::Occupied(_) => duplicate_connectors.push(object),
                }
            }
        }
    }

    (Producers { by_pin, block_output, connector_by_label }, duplicate_connectors)
}

fn role(object: &FbdObject) -> Role {
    match object.kind.as_str() {
        "ppx:DataSource" => Role::Source,
        "ppx:DataSink" => Role::Sink,
        "ppx:Return" => Role::Return,
        "ppx:Block" => Role::Block,
        "ppx:Connector" => Role::Connector,
        "ppx:Continuation" => Role::Continuation,
        "ppx:Unconnected" => Role::Unconnected,
        _ => Role::Other,
    }
}

/// The producer pin a consumer's single incoming wire references.
fn source_pin(consumer: &FbdObject) -> Option<usize> {
    consumer
        .connection_in
        .as_ref()
        .and_then(|it| it.connections.first())
        .map(|connection| connection.ref_out_id)
}

/// Follow a wire from a producer pin to the concrete source that feeds it,
/// hopping transparently through any connector/continuation pairs on the way.
/// Landing on a block output pin ends the walk — a block output is a real
/// producer, so this is what stops a feedback loop from chasing itself.
fn trace<'model>(start: Option<usize>, producers: &Producers<'model>) -> Trace<'model> {
    let Some(mut pin) = start else { return Trace::Unwired };
    let Some(mut element) = producers.by_pin.get(&pin).copied() else { return Trace::Unwired };

    // A continuation is a stand-in for its connector's input, so replace it with
    // that input and keep going until the wire lands on a real producer.
    let mut visited = HashSet::new();
    while matches!(role(element), Role::Continuation) {
        // Every way a continuation fails to resolve is a dangling read.
        let dangling =
            |element| Trace::DeadEnd(Broken { element, reason: BrokenReason::DanglingContinuation });

        // Resolve the continuation's label to the connector that owns it; a
        // missing label, a cycle, or a label no connector claims all dangle.
        let Some(label) = element.label() else { return dangling(element) };
        if !visited.insert(label) {
            return dangling(element);
        }
        let Some(connector) = producers.connector_by_label.get(label).copied() else {
            return dangling(element);
        };

        // Step onto whatever feeds that connector; a connector with no input is
        // an open connector and produces no value.
        match source_pin(connector).and_then(|id| producers.by_pin.get(&id).copied().map(|next| (id, next))) {
            Some((id, next)) => (pin, element) = (id, next),
            None => {
                return Trace::DeadEnd(Broken { element: connector, reason: BrokenReason::OpenConnector })
            }
        }
    }

    // A landed wire is a block output pin (read as a member) or a plain source.
    match producers.block_output.get(&pin).copied() {
        Some(output) => Trace::Reached(Source::Output { block: element, pin: output }),
        None => Trace::Reached(Source::Variable(element)),
    }
}

#[cfg(test)]
mod tests {
    use super::{resolve, Resolved, Source, Statement};
    use crate::model::Pou;

    fn resolve_project(fixture: &str) -> String {
        let source = crate::test_utils::fixture_source(fixture);
        let pou = Pou::parse(&source.source).unwrap();
        render(&resolve(pou.content().network()))
    }

    /// A source rendered as its diagram text and the global id it originates at.
    fn render_source(source: &Source) -> (String, usize) {
        match source {
            Source::Variable(object) => (object.identifier().unwrap_or("?").to_string(), object.global_id),
            Source::Output { block, pin } => {
                (format!("{}.{}", block.instance().unwrap_or("?"), pin.parameter_name), block.global_id)
            }
        }
    }

    fn render(resolved: &Resolved) -> String {
        let mut out = String::new();
        for statement in &resolved.statements {
            let line = match statement {
                Statement::Assignment { sink, source } => {
                    let (text, id) = render_source(source);
                    format!(
                        "{} := {text}   [sink {} <- source {id}]\n",
                        sink.identifier().unwrap_or("?"),
                        sink.global_id
                    )
                }
                Statement::Return { object, condition: Some(source) } => {
                    let (text, id) = render_source(source);
                    format!(
                        "RETURN {}{text}   [return {} <- source {id}]\n",
                        if object.negated() { "NOT " } else { "" },
                        object.global_id,
                    )
                }
                Statement::Return { object, condition: None } => {
                    format!("RETURN <disconnected>   [return {}]\n", object.global_id)
                }
                Statement::Call { block, arguments } => {
                    let args = arguments
                        .iter()
                        .map(|arg| {
                            let (text, _) = render_source(&arg.source);
                            let negate = if arg.pin.negated { "NOT " } else { "" };
                            format!("{} := {negate}{text}", arg.pin.parameter_name)
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    let target = block.call_target().unwrap_or_default();
                    format!("{target}({args})   [call {}]\n", block.global_id)
                }
            };
            out.push_str(&line);
        }
        let unconnected: Vec<_> =
            resolved.unconnected.iter().map(|it| it.identifier().unwrap_or("?")).collect();
        match unconnected.is_empty() {
            true => out.push_str("unconnected: (none)"),
            false => out.push_str(&format!("unconnected: {}", unconnected.join(", "))),
        }
        out
    }

    mod variables {
        use super::resolve_project;

        #[test]
        fn simple_assignment() {
            insta::assert_snapshot!(resolve_project("variables/valid/simple_assignment"), @r"
        bar := foo   [sink 3 <- source 1]
        unconnected: (none)");
        }

        #[test]
        fn reciprocal_assignment() {
            insta::assert_snapshot!(resolve_project("variables/valid/reciprocal_assignment"), @r"
        bar := foo   [sink 3 <- source 1]
        foo := bar   [sink 6 <- source 4]
        unconnected: (none)");
        }

        #[test]
        fn fan_out() {
            insta::assert_snapshot!(resolve_project("variables/valid/fan_out"), @r"
        bar := foo   [sink 3 <- source 1]
        baz := foo   [sink 4 <- source 1]
        unconnected: (none)");
        }

        #[test]
        fn literal_assignment() {
            insta::assert_snapshot!(resolve_project("variables/valid/literal_assignment"), @r"
        foo := 5   [sink 3 <- source 1]
        unconnected: (none)");
        }

        #[test]
        fn indexed_assignment() {
            insta::assert_snapshot!(resolve_project("variables/valid/indexed_assignment"), @r"
        values[1] := source   [sink 3 <- source 1]
        unconnected: (none)");
        }

        #[test]
        fn unconnected_variables() {
            insta::assert_snapshot!(resolve_project("variables/valid/unconnected_variables"), @r"
        bar := foo   [sink 8 <- source 6]
        unconnected: foo, bar");
        }
    }

    mod returns {
        use super::resolve_project;

        #[test]
        fn conditional_return() {
            insta::assert_snapshot!(resolve_project("returns/valid/conditional_return"), @r"
        RETURN myCondition   [return 3 <- source 1]
        unconnected: (none)");
        }

        #[test]
        fn negated_return() {
            insta::assert_snapshot!(resolve_project("returns/valid/negated_return"), @r"
        RETURN NOT myCondition   [return 3 <- source 1]
        unconnected: (none)");
        }

        #[test]
        fn disconnected_return() {
            insta::assert_snapshot!(resolve_project("returns/invalid/disconnected_return"), @r"
        RETURN myCondition   [return 3 <- source 1]
        RETURN <disconnected>   [return 4]
        unconnected: (none)");
        }
    }

    mod connectors {
        use super::resolve_project;

        #[test]
        fn assignment() {
            insta::assert_snapshot!(resolve_project("connectors/valid/assignment"), @r"
        bar := foo   [sink 3 <- source 1]
        unconnected: (none)");
        }

        #[test]
        fn fan_out() {
            insta::assert_snapshot!(resolve_project("connectors/valid/fan_out"), @r"
        bar := foo   [sink 3 <- source 1]
        baz := foo   [sink 9 <- source 1]
        unconnected: (none)");
        }

        #[test]
        fn chain() {
            insta::assert_snapshot!(resolve_project("connectors/valid/chain"), @r"
        bar := foo   [sink 12 <- source 1]
        unconnected: (none)");
        }
    }

    mod blocks {
        use super::resolve_project;

        #[test]
        fn program_call() {
            insta::assert_snapshot!(resolve_project("blocks/valid/program_call"), @r"
        counter(in := countIn)   [call 3]
        countOut := counter.out   [sink 5 <- source 3]
        unconnected: (none)");
        }

        #[test]
        fn program_chain() {
            insta::assert_snapshot!(resolve_project("blocks/valid/program_chain"), @r"
        counter(in := seed)   [call 3]
        doubler(in := counter.out)   [call 5]
        result := doubler.out   [sink 7 <- source 5]
        unconnected: (none)");
        }

        #[test]
        fn program_feedback() {
            insta::assert_snapshot!(resolve_project("blocks/valid/program_feedback"), @r"
        counter(in := counter.out)   [call 1]
        unconnected: (none)");
        }

        #[test]
        fn program_negated() {
            insta::assert_snapshot!(resolve_project("blocks/valid/program_negated"), @r"
        program_0(in := NOT localIn, inout := NOT localInOut)   [call 7]
        localOut := program_0.out   [sink 13 <- source 7]
        unconnected: (none)");
        }

        #[test]
        fn fb_call() {
            insta::assert_snapshot!(resolve_project("blocks/valid/fb_call"), @r"
        inst(in := localIn)   [call 3]
        localOut := inst.out   [sink 5 <- source 3]
        unconnected: (none)");
        }

        #[test]
        fn fb_instances() {
            insta::assert_snapshot!(resolve_project("blocks/valid/fb_instances"), @r"
        a(in := seed)   [call 3]
        b(in := a.out)   [call 5]
        result := b.out   [sink 7 <- source 5]
        unconnected: (none)");
        }

        #[test]
        fn action_fb() {
            insta::assert_snapshot!(resolve_project("blocks/valid/action_fb"), @r"
        inst.increment(in := localIn)   [call 3]
        localOut := inst.out   [sink 5 <- source 3]
        unconnected: (none)");
        }

        #[test]
        fn action_program() {
            insta::assert_snapshot!(resolve_project("blocks/valid/action_program"), @r"
        P.bump(step := localIn)   [call 3]
        localOut := P.out   [sink 5 <- source 3]
        unconnected: (none)");
        }
    }
}
