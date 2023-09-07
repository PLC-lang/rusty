use ast::ast::SourceRange;
use indexmap::{IndexMap, IndexSet};
use plc_diagnostics::diagnostics::Diagnostic;
use quick_xml::events::Event;
use std::{cmp::Ordering, collections::HashMap};

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

#[derive(Debug, PartialEq)]
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
        nodes.desugar_connection_points().map_err(Error::Lowering)?;

        Ok(FunctionBlockDiagram { nodes })
    }
}

enum SourceReference<'a> {
    Value(NodeId),
    Connector(&'a str),
    Unconnected,
}

struct ResolvedConnection {
    id: NodeId,
    resolved_ref_id: NodeId,
    block_parameter_index: Option<usize>,
}

// IndexMap<NodeId, Node> interface for connection-point (sink/source) desugaring
trait ConnectionResolver<'xml> {
    fn desugar_connection_points(&mut self) -> Result<(), Vec<Diagnostic>>;
    fn get_source_references(&self) -> HashMap<&str, SourceReference>;
    fn get_resolved_connection_ids(
        &self,
        source_connections: &HashMap<&str, SourceReference<'_>>,
    ) -> Result<Vec<ResolvedConnection>, Vec<Diagnostic>>;
}

impl<'xml> ConnectionResolver<'xml> for NodeIndex<'xml> {
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
    fn desugar_connection_points(&mut self) -> Result<(), Vec<Diagnostic>> {
        let source_connections = self.get_source_references();
        let resolved_connections = match self.get_resolved_connection_ids(&source_connections) {
            Ok(resolved_connections) => resolved_connections,
            Err(e) => return Err(e),
        };

        // update sink-connections to point to assignable values of associated source
        for ResolvedConnection { id, resolved_ref_id, block_parameter_index } in resolved_connections {
            self.entry(id).and_modify(|node| match node {
                Node::Block(block) => {
                    block.variables.get_mut(block_parameter_index.unwrap()).unwrap().ref_local_id =
                        Some(resolved_ref_id);
                }
                Node::FunctionBlockVariable(fbd_var) => fbd_var.ref_local_id = Some(resolved_ref_id),
                Node::Control(ctrl) => ctrl.ref_local_id = Some(resolved_ref_id),
                _ => (),
            });
        }

        // XXX: removing all connector nodes after resolving might mess with validation later on - revisit
        self.retain(|_, node| !matches!(node, Node::Connector(_)));

        Ok(())
    }

    fn get_source_references(&self) -> HashMap<&str, SourceReference> {
        self.iter()
            .filter_map(|(_, node)| {
                if let Node::Connector(Connector {
                    kind: ConnectorKind::Source, name, ref_local_id, ..
                }) = node
                {
                    ref_local_id
                        .map(|ref_id| {
                            if let Some(Node::Connector(Connector {
                                kind: ConnectorKind::Sink,
                                name: name_sink,
                                ..
                            })) = self.get(&ref_id)
                            {
                                // source points directly to another sink -> will be resolved via name
                                Some((name.as_ref(), SourceReference::Connector(name_sink)))
                            } else {
                                // source points to an assignable element -> will be resolved directly via ref ID
                                Some((name.as_ref(), SourceReference::Value(ref_id)))
                            }
                        })
                        .unwrap_or(Some((name.as_ref(), SourceReference::Unconnected)))
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_resolved_connection_ids(
        &self,
        source_connections: &HashMap<&str, SourceReference>,
    ) -> Result<Vec<ResolvedConnection>, Vec<Diagnostic>> {
        let mut resolved_connections = vec![];
        let mut diagnostics = vec![];

        let mut resolve_sink_connection_if_needed =
            |id: &NodeId, ref_id: NodeId, param_idx: Option<usize>| {
                if let Some(Node::Connector(Connector { kind: ConnectorKind::Sink, name, .. })) =
                    self.get(&ref_id)
                {
                    let mut path: IndexSet<&str> = IndexSet::new();

                    match find_assignable_sink_value(name, source_connections, &mut path) {
                        Ok(resolved_ref_id) => {
                            resolved_connections.push(ResolvedConnection {
                                id: *id,
                                resolved_ref_id,
                                block_parameter_index: param_idx,
                            });
                        }
                        Err(e) => diagnostics.push(e),
                    }
                };
            };

        // collect all relevant information on nodes that reference
        self.iter().for_each(|(id, node)| match node {
            Node::Block(block) => block.variables.iter().enumerate().for_each(|(param_idx, var)| {
                if let Some(ref_id) = var.ref_local_id {
                    resolve_sink_connection_if_needed(id, ref_id, Some(param_idx))
                }
            }),
            Node::FunctionBlockVariable(fbd_var) => {
                if let Some(ref_id) = fbd_var.ref_local_id {
                    resolve_sink_connection_if_needed(id, ref_id, None)
                }
            }
            Node::Control(control) => {
                if let Some(ref_id) = control.ref_local_id {
                    resolve_sink_connection_if_needed(id, ref_id, None)
                }
            }
            Node::Connector(conn) => {
                if let Some(ref_id) = conn.ref_local_id {
                    resolve_sink_connection_if_needed(id, ref_id, None)
                }
            }
        });

        if diagnostics.is_empty() {
            Ok(resolved_connections)
        } else {
            Err(diagnostics)
        }
    }
}

/// Attempts to resolve the LValue or RValue of a sink connection-point by checking the associated node referenced in the
/// provided `source_connection` HashMap. Should the associated source point to another sink (i.e. not an assignable value),
/// this function will be called recursively until a valid value is found.
/// Additionally, cyclic connections are detected and reported.
fn find_assignable_sink_value<'a>(
    connector_name: &str,
    source_connections: &'a HashMap<&str, SourceReference>,
    connector_path: &mut IndexSet<&'a str>,
) -> Result<NodeId, Diagnostic> {
    {
        let Some(source) = source_connections.get(connector_name) else {
            return Err(Diagnostic::sink_without_associated_source(connector_name, SourceRange::undefined()))
        };

        match source {
            SourceReference::Value(id) => Ok(*id),
            // for direct sink-to-source connections, we need to recurse to find the actual value
            SourceReference::Connector(name) => {
                if connector_path.insert(name) {
                    find_assignable_sink_value(name, source_connections, connector_path)
                } else {
                    // data-recursion detected
                    let mut message = String::new();
                    for connector_name in connector_path.iter() {
                        message = format!("{message}{connector_name} -> ")
                    }

                    message = format!("{message}{name}");

                    Err(Diagnostic::cyclic_connection(message, SourceRange::undefined()))
                }
            }
            SourceReference::Unconnected => {
                Err(Diagnostic::unconnected_source(connector_name, SourceRange::undefined()))
            }
        }
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
