//! Structural resolution of an FBD network: classify each element, link every
//! sink to its single source, and order the sinks by evaluation priority.

use std::collections::HashMap;

use crate::model::{FbdObject, Network};

pub(crate) struct Resolved<'model> {
    /// Sinks paired with their source, in evaluation-priority order.
    pub assignments: Vec<Assignment<'model>>,
    pub unconnected: Vec<&'model FbdObject>,
}

pub(crate) struct Assignment<'model> {
    pub sink: &'model FbdObject,
    pub source: &'model FbdObject,
}

enum Role {
    Source,
    Sink,
    Unconnected,
    Other,
}

fn role(object: &FbdObject) -> Role {
    match object.kind.as_str() {
        "ppx:DataSource" => Role::Source,
        "ppx:DataSink" => Role::Sink,
        "ppx:Unconnected" => Role::Unconnected,
        _ => Role::Other,
    }
}

pub(crate) fn resolve(network: &Network) -> Resolved<'_> {
    // Index every output pin so a sink can find its source in one hop.
    let mut source_by_pin: HashMap<usize, &FbdObject> = HashMap::new();
    for object in &network.objects {
        if let Some(out) = &object.connection_out {
            source_by_pin.insert(out.id, object);
        }
    }

    let mut assignments = Vec::new();
    let mut unconnected = Vec::new();

    for object in &network.objects {
        match role(object) {
            Role::Sink => {
                let source = object
                    .connection_in
                    .as_ref()
                    .and_then(|it| it.connections.first())
                    .and_then(|connection| source_by_pin.get(&connection.ref_out_id).copied());

                if let Some(source) = source {
                    assignments.push(Assignment { sink: object, source });
                }
            }
            Role::Unconnected => unconnected.push(object),
            Role::Source | Role::Other => {}
        }
    }

    assignments.sort_by_key(|it| it.sink.priority().unwrap_or(usize::MAX));

    Resolved { assignments, unconnected }
}

#[cfg(test)]
mod tests {
    use super::{resolve, Resolved};
    use crate::model::Pou;

    fn resolve_project(fixture: &str) -> String {
        let source = crate::test_utils::fixture_source(fixture);
        let pou = Pou::parse(&source.source).unwrap();
        render(&resolve(pou.content().network()))
    }

    fn render(resolved: &Resolved) -> String {
        let mut out = String::new();
        for it in &resolved.assignments {
            let (sink, source) = (it.sink, it.source);
            let line = format!(
                "{} := {}   [sink {} <- source {}]\n",
                sink.identifier().unwrap_or("?"),
                source.identifier().unwrap_or("?"),
                sink.global_id,
                source.global_id,
            );
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
}
