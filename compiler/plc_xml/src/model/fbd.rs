use std::{borrow::BorrowMut, cmp::Ordering};

use indexmap::IndexMap;
use quick_xml::events::Event;

use crate::{error::Error, reader::PeekableReader, xml_parser::Parseable};

use super::{
    block::Block,
    connector::{Connector, ConnectorKind},
    control::Control,
    variables::FunctionBlockVariable,
};

/// Represent either a `localId` or `refLocalId`
pub(crate) type NodeId = usize;
pub(crate) type NodeIndex = IndexMap<NodeId, Node>;

#[derive(Debug, Default)]
pub(crate) struct FunctionBlockDiagram {
    pub nodes: NodeIndex,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Node {
    Block(Block),
    FunctionBlockVariable(FunctionBlockVariable),
    Control(Control),
    Connector(Connector),
}

impl PartialOrd for Node {
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

impl Node {
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
            val.type_name.clone()
        } else {
            "".into()
        }
    }
}

impl Parseable for FunctionBlockDiagram {
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

fn resolve_connection_points(nodes: &mut NodeIndex) {
    fn find_source_connections(nodes: &NodeIndex) -> IndexMap<&str, NodeId> {
        nodes
            .iter()
            .filter_map(|(_, node)| {
                if let Node::Connector(Connector {
                    kind: ConnectorKind::Source, name, ref_local_id, ..
                }) = node
                {
                    if let Some(ref_id) = ref_local_id {
                        Some((name.as_str(), *ref_id))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<IndexMap<&str, NodeId>>()
    }

    let lookup = nodes.clone(); // this smells real bad

    let source_connections = find_source_connections(&lookup);

    for (_, node) in nodes {
        match dbg!(node) {
            Node::Block(block) => todo!(),
            Node::FunctionBlockVariable(fbd_var) => {
                let Some(ref_id) = fbd_var.ref_local_id else {
                    continue
                };

                if let Some(Node::Connector(Connector { kind: ConnectorKind::Sink, name, .. })) =
                    lookup.get(&ref_id)
                {
                    let Some(actual_source) = &source_connections.get(name.as_str()).copied() else {
                        todo!("unconnected source")
                    };

                    let _ = std::mem::replace(&mut fbd_var.ref_local_id, dbg!(Some(*actual_source)));
                }
            }
            Node::Control(control) => todo!(),
            _ => continue,
        };
    }
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
