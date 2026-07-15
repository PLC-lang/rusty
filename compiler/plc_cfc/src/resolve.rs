//! Structural resolution of an FBD network: classify each element, link every
//! sink and return to its source, and order the statements by evaluation
//! priority.

use std::collections::HashMap;

use crate::model::{FbdObject, Network};

pub(crate) struct Resolved<'model> {
    /// Network statements in evaluation-priority order.
    pub statements: Vec<Statement<'model>>,
    pub unconnected: Vec<&'model FbdObject>,
}

pub(crate) enum Statement<'model> {
    Assignment { sink: &'model FbdObject, source: &'model FbdObject },
    Return { object: &'model FbdObject, condition: Option<&'model FbdObject> },
}

enum Role {
    Source,
    Sink,
    Return,
    Unconnected,
    Other,
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

fn role(object: &FbdObject) -> Role {
    match object.kind.as_str() {
        "ppx:DataSource" => Role::Source,
        "ppx:DataSink" => Role::Sink,
        "ppx:Return" => Role::Return,
        "ppx:Unconnected" => Role::Unconnected,
        _ => Role::Other,
    }
}

fn source_of<'model>(
    object: &FbdObject,
    source_by_pin: &HashMap<usize, &'model FbdObject>,
) -> Option<&'model FbdObject> {
    object
        .connection_in
        .as_ref()
        .and_then(|it| it.connections.first())
        .and_then(|connection| source_by_pin.get(&connection.ref_out_id).copied())
}

pub(crate) fn resolve(network: &Network) -> Resolved<'_> {
    // Index every output pin so a consumer can find its source in one hop.
    let mut source_by_pin: HashMap<usize, &FbdObject> = HashMap::new();
    for object in &network.objects {
        if let Some(out) = &object.connection_out {
            source_by_pin.insert(out.id, object);
        }
    }

    let mut statements = Vec::new();
    let mut unconnected = Vec::new();

    for object in &network.objects {
        match role(object) {
            Role::Sink => {
                if let Some(source) = source_of(object, &source_by_pin) {
                    statements.push(Statement::Assignment { sink: object, source });
                }
            }
            Role::Return => {
                statements.push(Statement::Return { object, condition: source_of(object, &source_by_pin) });
            }
            Role::Unconnected => unconnected.push(object),
            Role::Source | Role::Other => {}
        }
    }

    statements.sort_by_key(|statement| statement.anchor().priority().unwrap_or(usize::MAX));

    Resolved { statements, unconnected }
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
}
