//! Structural resolution of an FBD network: classify each element, link every
//! sink and return to the concrete source that feeds it, and order the
//! statements by evaluation priority.
//!
//! Connector/continuation pairs are spliced out during linking: they carry no
//! statement of their own, they only reroute what a consumer ultimately reads.

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use crate::model::{FbdObject, Network};

pub(crate) struct Resolved<'model> {
    /// Network statements in evaluation-priority order.
    pub statements: Vec<Statement<'model>>,
    pub unconnected: Vec<&'model FbdObject>,
    /// Connectors that repeat a label already claimed by an earlier one.
    pub duplicate_connectors: Vec<&'model FbdObject>,
    /// Connectors/continuations where a consumed chain never reached a source.
    pub broken: Vec<Broken<'model>>,
}

pub(crate) enum Statement<'model> {
    Assignment { sink: &'model FbdObject, source: &'model FbdObject },
    Return { object: &'model FbdObject, condition: Option<&'model FbdObject> },
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
    Connector,
    Continuation,
    Unconnected,
    Other,
}

enum Trace<'model> {
    Reached(&'model FbdObject),
    DeadEnd(Broken<'model>),
    Unwired,
}

impl<'model> Statement<'model> {
    /// The element carrying this statement's priority and diagram location.
    fn anchor(&self) -> &'model FbdObject {
        match self {
            Statement::Assignment { sink, .. } => sink,
            Statement::Return { object, .. } => object,
        }
    }
}

pub(crate) fn resolve(network: &Network) -> Resolved<'_> {
    // Index the network so the linking pass below is pure lookup.
    let mut source_by_pin: HashMap<usize, &FbdObject> = HashMap::new();
    let mut connector_by_label: HashMap<&str, &FbdObject> = HashMap::new();
    let mut duplicate_connectors = Vec::new();
    for object in network.elements() {
        // A producer exposes an output pin; map that pin's id back to it so a
        // consumer wired to the pin can find what drives it.
        if let Some(out) = &object.connection_out {
            source_by_pin.insert(out.id, object);
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

    // Link each consumer to its source: a sink becomes an assignment, a return
    // a (guarded) return. A chain that never reaches a source is set aside for
    // validation; connectors/continuations are pure routing and emit nothing.
    let mut statements = Vec::new();
    let mut unconnected = Vec::new();
    let mut broken = Vec::new();
    for object in network.elements() {
        match role(object) {
            Role::Sink => match trace_source(object, &source_by_pin, &connector_by_label) {
                Trace::Reached(source) => statements.push(Statement::Assignment { sink: object, source }),
                Trace::DeadEnd(at) => broken.push(at),
                Trace::Unwired => {} // an unwired sink assigns nothing; drop it
            },

            Role::Return => match trace_source(object, &source_by_pin, &connector_by_label) {
                Trace::Reached(source) => {
                    statements.push(Statement::Return { object, condition: Some(source) })
                }
                Trace::DeadEnd(at) => broken.push(at),
                // No wire means no guard; validation flags the bare return (E085).
                Trace::Unwired => statements.push(Statement::Return { object, condition: None }),
            },

            Role::Unconnected => unconnected.push(object),
            Role::Source | Role::Connector | Role::Continuation | Role::Other => {}
        }
    }

    // Statements run in evaluation-priority order; a break fanned out across
    // several consumers points at one element, so report it just once.
    statements.sort_by_key(|statement| statement.anchor().priority().unwrap_or(usize::MAX));
    let mut seen = HashSet::new();
    broken.retain(|at| seen.insert(at.element.global_id));

    Resolved { statements, unconnected, duplicate_connectors, broken }
}

fn role(object: &FbdObject) -> Role {
    match object.kind.as_str() {
        "ppx:DataSource" => Role::Source,
        "ppx:DataSink" => Role::Sink,
        "ppx:Return" => Role::Return,
        "ppx:Connector" => Role::Connector,
        "ppx:Continuation" => Role::Continuation,
        "ppx:Unconnected" => Role::Unconnected,
        _ => Role::Other,
    }
}

/// The element on the far end of a consumer's single incoming wire.
fn wired_source<'model>(
    consumer: &FbdObject,
    source_by_pin: &HashMap<usize, &'model FbdObject>,
) -> Option<&'model FbdObject> {
    consumer
        .connection_in
        .as_ref()
        .and_then(|it| it.connections.first())
        .and_then(|connection| source_by_pin.get(&connection.ref_out_id).copied())
}

/// Follow a consumer's wire to the concrete source that feeds it, hopping
/// transparently through any connector/continuation pairs on the way.
fn trace_source<'model>(
    consumer: &FbdObject,
    source_by_pin: &HashMap<usize, &'model FbdObject>,
    connector_by_label: &HashMap<&str, &'model FbdObject>,
) -> Trace<'model> {
    let Some(mut element) = wired_source(consumer, source_by_pin) else {
        return Trace::Unwired;
    };

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
        let Some(connector) = connector_by_label.get(label).copied() else {
            return dangling(element);
        };

        // Step onto whatever feeds that connector; a connector with no input is
        // an open connector and produces no value.
        match wired_source(connector, source_by_pin) {
            Some(next) => element = next,
            None => {
                return Trace::DeadEnd(Broken { element: connector, reason: BrokenReason::OpenConnector })
            }
        }
    }

    Trace::Reached(element)
}

#[cfg(test)]
mod tests {
    use super::{resolve, Resolved, Statement};
    use crate::model::Pou;

    fn resolve_project(fixture: &str) -> String {
        let source = crate::test_utils::fixture_source(fixture);
        let pou = Pou::parse(&source.source).unwrap();
        render(&resolve(pou.content().network()))
    }

    fn render(resolved: &Resolved) -> String {
        let mut out = String::new();
        for statement in &resolved.statements {
            let line = match statement {
                Statement::Assignment { sink, source } => format!(
                    "{} := {}   [sink {} <- source {}]\n",
                    sink.identifier().unwrap_or("?"),
                    source.identifier().unwrap_or("?"),
                    sink.global_id,
                    source.global_id,
                ),
                Statement::Return { object, condition: Some(source) } => format!(
                    "RETURN {}{}   [return {} <- source {}]\n",
                    if object.negated() { "NOT " } else { "" },
                    source.identifier().unwrap_or("?"),
                    object.global_id,
                    source.global_id,
                ),
                Statement::Return { object, condition: None } => {
                    format!("RETURN <disconnected>   [return {}]\n", object.global_id)
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
}
