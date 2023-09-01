use indexmap::IndexMap;
use quick_xml::events::Event;
use std::cmp::Ordering;

use crate::{error::Error, reader::PeekableReader, xml_parser::Parseable};

use super::{
    block::Block,
    connector::{Connector, ConnectorKind},
    control::Control,
    variables::FunctionBlockVariable,
};

/// Represent either a `localId` or `refLocalId`
pub(crate) type NodeId = usize;
pub(crate) type NodeIndex<'xml> = IndexMap<NodeId, Node<'xml>>;

#[derive(Debug, Default)]
pub(crate) struct FunctionBlockDiagram<'xml> {
    pub nodes: NodeIndex<'xml>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Node<'xml> {
    Block(Block<'xml>),
    FunctionBlockVariable(FunctionBlockVariable<'xml>),
    Control(Control<'xml>),
    Connector(Connector<'xml>),
}

impl<'xml> PartialOrd for Node<'xml> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left = self.get_exec_id();
        let right = other.get_exec_id();

        match (left, right) {
            (None, None) => Some(Ordering::Equal),
            (None, Some(_)) => Some(Ordering::Less),
            (Some(_), None) => Some(Ordering::Greater),
            (Some(left), Some(right)) => Some(left.cmp(&right)),
        }
    }
}

impl<'xml> Node<'xml> {
    pub(crate) fn get_exec_id(&self) -> Option<NodeId> {
        match self {
            Node::Block(val) => val.execution_order_id,
            Node::FunctionBlockVariable(val) => val.execution_order_id,
            Node::Control(val) => val.execution_order_id,
            _ => None,
        }
    }

    fn get_id(&self) -> NodeId {
        match self {
            Node::Block(val) => val.local_id,
            Node::FunctionBlockVariable(val) => val.local_id,
            Node::Control(val) => val.local_id,
            Node::Connector(val) => val.local_id,
        }
    }

    fn get_name(&self) -> String {
        if let Node::Block(val) = self {
            // TODO: check if the out variables are named after the type- or instance-name
            val.type_name.to_string()
        } else {
            "".into()
        }
    }
}

impl<'xml> Parseable for FunctionBlockDiagram<'xml> {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        reader.consume()?;
        let mut nodes = IndexMap::new();

        loop {
            match reader.peek()? {
                Event::Start(tag) => match tag.name().as_ref() {
                    b"block" => {
                        let node = Block::visit(reader)?;
                        nodes.insert(node.local_id, Node::Block(node));
                    }
                    b"jump" | b"label" | b"return" => {
                        let node = Control::visit(reader)?;
                        nodes.insert(node.local_id, Node::Control(node));
                    }
                    b"inVariable" | b"outVariable" => {
                        let node = FunctionBlockVariable::visit(reader)?;
                        nodes.insert(node.local_id, Node::FunctionBlockVariable(node));
                    }
                    b"continuation" | b"connector" => {
                        let node = Connector::visit(reader)?;
                        nodes.insert(node.local_id, Node::Connector(node));
                    }
                    _ => reader.consume()?,
                },

                Event::End(tag) if tag.name().as_ref() == b"FBD" => {
                    reader.consume()?;
                    break;
                }
                _ => reader.consume()?,
            }
        }

        nodes.sort_by(|_, b, _, d| b.partial_cmp(d).unwrap()); // This _shouldn't_ panic because our `partial_cmp` method covers all cases
        resolve_connection_points(&mut nodes);

        Ok(FunctionBlockDiagram { nodes })
    }
}

enum ConnectionReference<'xml> {
    Id(NodeId),
    Name(&'xml str),
}

/// Checks if given node is pointing to a `Sink` and updates the referenced ID to directly point to the element referenced
/// by the matching `Source`
macro_rules! update_connection_ref_id_if_needed {
    ($node:ident, $source_connections:ident, $nodes:ident) => {
        if let Some(ref_id) = $node.ref_local_id {
            if let Some(Node::Connector(Connector { kind: ConnectorKind::Sink, name, .. })) =
                $nodes.get(&ref_id)
            {
                $node.ref_local_id = get_inner_connection_ref(name.as_ref(), &$source_connections);
            }
        }
    };
}

fn get_inner_connection_ref<'a>(
    name: &str,
    source_connections: &IndexMap<&'a str, ConnectionReference<'a>>,
) -> Option<NodeId> {
    let Some(source) = source_connections.get(name) else {
        todo!("unconnected source")
    };

    match source {
        ConnectionReference::Id(id) => Some(*id),
        // for direct sink-to-source connections, we need to recurse to find the actual value
        ConnectionReference::Name(name) => get_inner_connection_ref(name, source_connections),
    }
}

/// Updates all nodes in the index, which are connected via connection-points (sink/source) to be treated as
/// if they are connected directly instead.
///
/// ```st
/// // assignments using sink and source
/// INPUT ━━━━> SOURCE ┅┅┅> SINK ━┳━━> OUT1
///                               ┣━━> OUT2
///                               ┗━━> OUT3
/// // resolve to
/// INPUT ━┳━━> OUT1
///        ┣━━> OUT2
///        ┗━━> OUT3
/// ```
fn resolve_connection_points(nodes: &mut NodeIndex) {
    fn get_connection_references<'b>(
        node: &'b Node,
        nodes: &'b IndexMap<usize, Node<'_>>,
    ) -> Option<(&'b str, ConnectionReference<'b>)> {
        if let Node::Connector(Connector { kind: ConnectorKind::Source, name, ref_local_id, .. }) = node {
            ref_local_id
                .map(|ref_id| {
                    if let Some(Node::Connector(Connector {
                        kind: ConnectorKind::Sink,
                        name: name_sink,
                        ..
                    })) = nodes.get(&ref_id)
                    {
                        // source points directly to another sink -> will be resolved via name
                        Some((name.as_ref(), ConnectionReference::Name(name_sink)))
                    } else {
                        // source points to an assignable element -> will be resolved directly via ref ID
                        Some((name.as_ref(), ConnectionReference::Id(ref_id)))
                    }
                })
                .unwrap_or_else(|| None /* TODO: diagnostic */)
        } else {
            None
        }
    }

    let lookup = nodes.clone();

    let source_connections = lookup
        .iter()
        .filter_map(|(_, node)| get_connection_references(node, &lookup))
        .collect::<IndexMap<_, _>>();

    nodes.into_iter().for_each(|(_, node)| {
        match node {
            Node::Block(block) => {
                for var in &mut block.variables {
                    update_connection_ref_id_if_needed!(var, source_connections, lookup);
                }
            }
            Node::FunctionBlockVariable(fbd_var) => {
                update_connection_ref_id_if_needed!(fbd_var, source_connections, lookup)
            }
            Node::Control(control) => {
                update_connection_ref_id_if_needed!(control, source_connections, lookup)
            }
            _ => (),
        };
    });

    nodes.retain(|_, node| !matches!(node, Node::Connector(_)));
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{
        model::fbd::FunctionBlockDiagram,
        reader::PeekableReader,
        serializer::{
            XBlock, XConnection, XConnectionPointIn, XConnectionPointOut, XExpression, XFbd, XInOutVariables,
            XInVariable, XInputVariables, XOutVariable, XOutputVariables, XPosition, XRelPosition, XVariable,
        },
        xml_parser::Parseable,
    };

    #[test]
    fn add_block() {
        let content = XFbd::new()
            .with_block(
                XBlock::init("1", "ADD", "0")
                    .with_input_variables(
                        XInputVariables::new()
                            .with_variable(XVariable::init("a", false).with_connection_in_initialized("1"))
                            .with_variable(XVariable::init("b", false).with_connection_in_initialized("2")),
                    )
                    .with_inout_variables(XInOutVariables::new().close())
                    .with_output_variables(
                        XOutputVariables::new()
                            .with_variable(XVariable::init("c", false).with_connection_out_initialized()),
                    ),
            )
            .with_in_variable(
                XInVariable::init("2", false)
                    .with_position(XPosition::new().close())
                    .with_connection_point_out(
                        XConnectionPointOut::new().with_rel_position(XRelPosition::init()),
                    )
                    .with_expression(XExpression::new().with_data("a")),
            )
            .with_in_variable(
                XInVariable::init("3", false)
                    .with_position(XPosition::new().close())
                    .with_connection_point_out(
                        XConnectionPointOut::new().with_rel_position(XRelPosition::init()),
                    )
                    .with_expression(XExpression::new().with_data("b")),
            )
            .with_out_variable(
                XOutVariable::init("4", false)
                    .with_position(XPosition::new().close())
                    .with_attribute("executionOrderId", "1")
                    .with_connection_point_in(
                        XConnectionPointIn::new()
                            .with_rel_position(XRelPosition::init())
                            .with_connection(XConnection::new().with_attribute("refLocalId", "1")),
                    )
                    .with_expression(XExpression::new().with_data("c")),
            )
            .serialize();

        let mut reader = PeekableReader::new(&content);
        assert_debug_snapshot!(FunctionBlockDiagram::visit(&mut reader).unwrap());
    }
}
