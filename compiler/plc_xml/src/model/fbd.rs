use indexmap::{IndexMap, IndexSet};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocationFactory;
use quick_xml::events::Event;
use std::{cmp::Ordering, collections::HashMap, hash::Hash};

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

impl<'xml> FunctionBlockDiagram<'xml> {
    pub(crate) fn desugar(
        &mut self,
        source_location_factory: &SourceLocationFactory,
    ) -> Result<(), Vec<Diagnostic>> {
        self.nodes.desugar_connection_points(source_location_factory)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
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

    fn get_ref_ids(&self) -> Vec<Option<NodeId>> {
        match self {
            Node::Block(val) => val.variables.iter().map(|it| it.ref_local_id).collect(),
            Node::FunctionBlockVariable(val) => vec![val.ref_local_id],
            Node::Control(val) => vec![val.ref_local_id],
            Node::Connector(val) => vec![val.ref_local_id],
        }
    }

    fn get_name(&self) -> &str {
        match self {
            Node::Block(val) => {
                // TODO: check if the out variables are named after the type- or instance-name
                &val.type_name
            }
            Node::Connector(val) => &val.name,
            _ => "",
        }
    }

    fn set_ref_id(&mut self, param_idx: usize, ref_local_id: Option<usize>) {
        match self {
            Node::Block(block) => {
                if let Some(variable) = block.variables.get_mut(param_idx) {
                    variable.ref_local_id = ref_local_id;
                }
            }
            Node::FunctionBlockVariable(var) => var.ref_local_id = ref_local_id,
            Node::Control(control) => control.ref_local_id = ref_local_id,
            Node::Connector(connector) => connector.ref_local_id = ref_local_id,
        }
    }

    fn is_connector(&self) -> bool {
        matches!(self, Node::Connector(_))
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

        Ok(FunctionBlockDiagram { nodes })
    }
}

// IndexMap<NodeId, Node> interface for connection-point (sink/source) desugaring
trait ConnectionResolver<'xml> {
    fn desugar_connection_points(
        &mut self,
        source_location_factory: &SourceLocationFactory,
    ) -> Result<(), Vec<Diagnostic>>;
    fn get_source_references(&self) -> HashMap<&str, NodeId>;
    fn get_resolved_connection_id(
        &self,
        connection: NodeId,
        source_connections: &HashMap<&str, NodeId>,
        source_location_factory: &SourceLocationFactory,
    ) -> Result<NodeId, Diagnostic>;
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
    fn desugar_connection_points<'b>(
        &mut self,
        source_location_factory: &SourceLocationFactory,
    ) -> Result<(), Vec<Diagnostic>> {
        let sinks_to_sources = self.get_source_references();
        let mut update_operations: Vec<(NodeId, Option<NodeId>, usize)> = vec![]; /* (local_id, ref_local_id, parameter index) */
        let mut diagnostics = vec![];

        // for each node, check all nodes which reference/target it and collect the resolved ids
        for (node_id, node) in self.iter() {
            for (pos, id) in node.get_ref_ids().iter().enumerate() {
                let target_id = id
                    .map(|it| self.get_resolved_connection_id(it, &sinks_to_sources, source_location_factory))
                    .transpose()
                    .unwrap_or_else(|err| {
                        diagnostics.push(err);
                        None
                    });
                update_operations.push((*node_id, target_id, pos));
            }
        }

        // update nodes with resolved target id
        for (id, ref_id, param_idx) in update_operations {
            if let Some(node) = self.get_mut(&id) {
                node.set_ref_id(param_idx, ref_id);
            }
        }

        self.retain(|_, it| !it.is_connector());

        if !diagnostics.is_empty() {
            return Err(diagnostics);
        };

        Ok(())
    }

    /// Given a start connection, finds the final resuling connection
    /// by following the source/sink connection chains
    /// If the connection is not a source or a sink, the original connection is returned
    fn get_resolved_connection_id(
        &self,
        connection: NodeId,
        source_connections: &HashMap<&str, NodeId>,
        source_location_factory: &SourceLocationFactory,
    ) -> Result<NodeId, Diagnostic> {
        let mut current = connection;
        let mut visited = IndexSet::new();
        visited.insert(connection);
        loop {
            match self.get(&current) {
                Some(Node::Connector(Connector {
                    kind: ConnectorKind::Source, name, ref_local_id, ..
                })) => {
                    current = ref_local_id.ok_or_else(|| {
                        Diagnostic::unconnected_source(
                            name.as_ref(),
                            source_location_factory.create_block_location(current, None),
                        )
                    })?
                }
                Some(Node::Connector(Connector { kind: ConnectorKind::Sink, name, .. })) => {
                    current = *source_connections.get(name.as_ref()).ok_or_else(|| {
                        Diagnostic::sink_without_associated_source(
                            name.as_ref(),
                            source_location_factory.create_block_location(current, None),
                        )
                    })?
                }
                _ => return Ok(current),
            }

            if !visited.insert(current) {
                // problem: recursive connections
                let mut msg = String::new();
                for node in visited {
                    msg.push_str(self.get(&node).expect("Node exists").get_name());
                    msg.push_str(" -> ")
                }
                let node = self.get(&current).expect("Node exists");
                msg.push_str(node.get_name());

                return Err(Diagnostic::cyclic_connection(
                    msg.to_string(),
                    source_location_factory.create_block_location(node.get_id(), node.get_exec_id()),
                ));
            }
        }
    }

    /// Returns a list of all sources along with the id they are connected to
    fn get_source_references(&self) -> HashMap<&str, NodeId> {
        self.iter()
            .filter_map(|(_, node)| {
                if let Node::Connector(Connector {
                    local_id,
                    kind: ConnectorKind::Source,
                    name,
                    ref_local_id,
                    ..
                }) = node
                {
                    Some((name.as_ref(), ref_local_id.unwrap_or(*local_id)))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        model::{
            connector::Connector, fbd::FunctionBlockDiagram, pou::Pou, project::Project,
            variables::FunctionBlockVariable,
        },
        reader::PeekableReader,
        serializer::{
            XBlock, XConnection, XConnectionPointIn, XConnectionPointOut, XExpression, XFbd, XInOutVariables,
            XInVariable, XInputVariables, XOutVariable, XOutputVariables, XPosition, XRelPosition, XVariable,
        },
        xml_parser::Parseable,
    };
    use insta::assert_debug_snapshot;
    use plc_source::source_location::SourceLocationFactory;

    use super::Node;

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

    #[test]
    fn model_with_no_source_sink_unchanged() {
        let source_location_factory = SourceLocationFactory::internal("");
        let mut model = Project::default();
        let mut pou = Pou::default();
        pou.name = "TestProg".into();
        let fbd = FunctionBlockDiagram {
            nodes: [
                (
                    1,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Input,
                        local_id: 1,
                        negated: false,
                        expression: "a".into(),
                        execution_order_id: None,
                        ref_local_id: None,
                    }),
                ),
                (
                    2,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Output,
                        local_id: 2,
                        negated: false,
                        expression: "b".into(),
                        execution_order_id: Some(1),
                        ref_local_id: Some(1),
                    }),
                ),
            ]
            .into(),
        };
        pou.body.function_block_diagram = Some(fbd);
        model.pous.push(pou);

        model.desugar(&source_location_factory).unwrap();
        assert_debug_snapshot!(model)
    }

    #[test]
    fn source_to_sink_converted_to_connection() {
        let source_location_factory = SourceLocationFactory::internal("");
        let mut model = Project::default();
        let mut pou = Pou::default();
        pou.name = "TestProg".into();
        let fbd = FunctionBlockDiagram {
            nodes: [
                (
                    1,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Input,
                        local_id: 1,
                        negated: false,
                        expression: "a".into(),
                        execution_order_id: None,
                        ref_local_id: None,
                    }),
                ),
                (
                    2,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Output,
                        local_id: 2,
                        negated: false,
                        expression: "b".into(),
                        execution_order_id: Some(1),
                        ref_local_id: Some(4),
                    }),
                ),
                (
                    3,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Source,
                        name: "connection1".into(),
                        local_id: 3,
                        ref_local_id: Some(1),
                        formal_parameter: None,
                    }),
                ),
                (
                    4,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Sink,
                        name: "connection1".into(),
                        local_id: 4,
                        ref_local_id: None,
                        formal_parameter: None,
                    }),
                ),
            ]
            .into(),
        };
        pou.body.function_block_diagram = Some(fbd);
        model.pous.push(pou);

        model.desugar(&source_location_factory).unwrap();
        assert_debug_snapshot!(model)
    }

    #[test]
    fn two_sinks_for_single_source_converted_to_connections() {
        let source_location_factory = SourceLocationFactory::internal("");
        let mut model = Project::default();
        let mut pou = Pou::default();
        pou.name = "TestProg".into();
        let fbd = FunctionBlockDiagram {
            nodes: [
                (
                    1,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Input,
                        local_id: 1,
                        negated: false,
                        expression: "a".into(),
                        execution_order_id: None,
                        ref_local_id: None,
                    }),
                ),
                (
                    2,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Output,
                        local_id: 2,
                        negated: false,
                        expression: "b".into(),
                        execution_order_id: Some(1),
                        ref_local_id: Some(4),
                    }),
                ),
                (
                    3,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Source,
                        name: "connection1".into(),
                        local_id: 3,
                        ref_local_id: Some(1),
                        formal_parameter: None,
                    }),
                ),
                (
                    4,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Sink,
                        name: "connection1".into(),
                        local_id: 4,
                        ref_local_id: None,
                        formal_parameter: None,
                    }),
                ),
                (
                    5,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Sink,
                        name: "connection1".into(),
                        local_id: 5,
                        ref_local_id: None,
                        formal_parameter: None,
                    }),
                ),
                (
                    6,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Output,
                        local_id: 6,
                        negated: false,
                        expression: "c".into(),
                        execution_order_id: Some(2),
                        ref_local_id: Some(5),
                    }),
                ),
            ]
            .into(),
        };
        pou.body.function_block_diagram = Some(fbd);
        model.pous.push(pou);

        model.desugar(&source_location_factory).unwrap();
        assert_debug_snapshot!(model)
    }

    #[test]
    fn source_sink_chain_converted_to_connection() {
        let source_location_factory = SourceLocationFactory::internal("");
        let mut model = Project::default();
        let mut pou = Pou::default();
        pou.name = "TestProg".into();
        let fbd = FunctionBlockDiagram {
            nodes: [
                (
                    1,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Input,
                        local_id: 1,
                        negated: false,
                        expression: "a".into(),
                        execution_order_id: None,
                        ref_local_id: None,
                    }),
                ),
                (
                    2,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Output,
                        local_id: 2,
                        negated: false,
                        expression: "b".into(),
                        execution_order_id: Some(1),
                        ref_local_id: Some(6),
                    }),
                ),
                (
                    3,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Source,
                        name: "connection1".into(),
                        local_id: 3,
                        ref_local_id: Some(1),
                        formal_parameter: None,
                    }),
                ),
                (
                    4,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Source,
                        name: "connection2".into(),
                        local_id: 4,
                        ref_local_id: Some(5),
                        formal_parameter: None,
                    }),
                ),
                (
                    5,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Sink,
                        name: "connection1".into(),
                        local_id: 5,
                        ref_local_id: None,
                        formal_parameter: None,
                    }),
                ),
                (
                    6,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Sink,
                        name: "connection2".into(),
                        local_id: 6,
                        ref_local_id: None,
                        formal_parameter: None,
                    }),
                ),
            ]
            .into(),
        };
        pou.body.function_block_diagram = Some(fbd);
        model.pous.push(pou);

        model.desugar(&source_location_factory).unwrap();
        assert_debug_snapshot!(model)
    }

    #[test]
    fn unassociated_source_remains_in_model() {
        let source_location_factory = SourceLocationFactory::internal("");
        let mut model = Project::default();
        let mut pou = Pou::default();
        pou.name = "TestProg".into();
        let fbd = FunctionBlockDiagram {
            nodes: [
                (
                    1,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Input,
                        local_id: 1,
                        negated: false,
                        expression: "a".into(),
                        execution_order_id: None,
                        ref_local_id: None,
                    }),
                ),
                (
                    2,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Output,
                        local_id: 2,
                        negated: false,
                        expression: "b".into(),
                        execution_order_id: Some(1),
                        ref_local_id: None,
                    }),
                ),
                (
                    3,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Source,
                        name: "connection1".into(),
                        local_id: 3,
                        ref_local_id: Some(1),
                        formal_parameter: None,
                    }),
                ),
            ]
            .into(),
        };
        pou.body.function_block_diagram = Some(fbd);
        model.pous.push(pou);

        model.desugar(&source_location_factory).unwrap();
        assert_debug_snapshot!(model)
    }

    #[test]
    fn unassociated_sink_removed_from_model_with_error() {
        let source_location_factory = SourceLocationFactory::internal("");
        let mut model = Project::default();
        let mut pou = Pou::default();
        pou.name = "TestProg".into();
        let fbd = FunctionBlockDiagram {
            nodes: [
                (
                    1,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Input,
                        local_id: 1,
                        negated: false,
                        expression: "a".into(),
                        execution_order_id: None,
                        ref_local_id: None,
                    }),
                ),
                (
                    2,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Output,
                        local_id: 2,
                        negated: false,
                        expression: "b".into(),
                        execution_order_id: Some(1),
                        ref_local_id: Some(3),
                    }),
                ),
                (
                    3,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Sink,
                        name: "connection1".into(),
                        local_id: 3,
                        ref_local_id: None,
                        formal_parameter: None,
                    }),
                ),
            ]
            .into(),
        };
        pou.body.function_block_diagram = Some(fbd);
        model.pous.push(pou);
        //With diagnostic

        let err = model.desugar(&source_location_factory).unwrap_err();
        assert_debug_snapshot!(err);
        assert_debug_snapshot!(model);
    }

    #[test]
    fn recursive_sink_source_connections_are_an_error() {
        let source_location_factory = SourceLocationFactory::internal("");
        let mut model = Project::default();
        let mut pou = Pou::default();
        pou.name = "TestProg".into();
        let fbd = FunctionBlockDiagram {
            nodes: [
                (
                    1,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Input,
                        local_id: 1,
                        negated: false,
                        expression: "a".into(),
                        execution_order_id: None,
                        ref_local_id: None,
                    }),
                ),
                (
                    2,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Output,
                        local_id: 2,
                        negated: false,
                        expression: "b".into(),
                        execution_order_id: Some(1),
                        ref_local_id: Some(6),
                    }),
                ),
                (
                    3,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Source,
                        name: "connection1".into(),
                        local_id: 3,
                        ref_local_id: Some(6),
                        formal_parameter: None,
                    }),
                ),
                (
                    4,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Source,
                        name: "connection2".into(),
                        local_id: 4,
                        ref_local_id: Some(5),
                        formal_parameter: None,
                    }),
                ),
                (
                    5,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Sink,
                        name: "connection1".into(),
                        local_id: 5,
                        ref_local_id: None,
                        formal_parameter: None,
                    }),
                ),
                (
                    6,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Sink,
                        name: "connection2".into(),
                        local_id: 6,
                        ref_local_id: None,
                        formal_parameter: None,
                    }),
                ),
            ]
            .into(),
        };
        pou.body.function_block_diagram = Some(fbd);
        model.pous.push(pou);

        let err = model.desugar(&source_location_factory).unwrap_err();
        assert_debug_snapshot!(err);
        assert_debug_snapshot!(model);
    }

    #[test]
    fn unconnected_source_has_no_effect() {
        let source_location_factory = SourceLocationFactory::internal("");
        let mut model = Project::default();
        let mut pou = Pou::default();
        pou.name = "TestProg".into();
        let fbd = FunctionBlockDiagram {
            nodes: [
                (
                    1,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Input,
                        local_id: 1,
                        negated: false,
                        expression: "a".into(),
                        execution_order_id: None,
                        ref_local_id: None,
                    }),
                ),
                (
                    2,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Output,
                        local_id: 2,
                        negated: false,
                        expression: "b".into(),
                        execution_order_id: Some(1),
                        ref_local_id: Some(4),
                    }),
                ),
                (
                    3,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Source,
                        name: "connection1".into(),
                        local_id: 3,
                        ref_local_id: None,
                        formal_parameter: None,
                    }),
                ),
                (
                    4,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Sink,
                        name: "connection1".into(),
                        local_id: 4,
                        ref_local_id: None,
                        formal_parameter: None,
                    }),
                ),
            ]
            .into(),
        };
        pou.body.function_block_diagram = Some(fbd);
        model.pous.push(pou);

        let err = model.desugar(&source_location_factory).unwrap_err();
        assert_debug_snapshot!(err);
        assert_debug_snapshot!(model);
    }

    #[test]
    fn multiple_sink_are_ok_and_duplicate_sources_instances_are_reported() {
        //TODO: split into two tests
        let source_location_factory = SourceLocationFactory::internal("");
        let mut model = Project::default();
        let mut pou = Pou::default();
        pou.name = "TestProg".into();
        let fbd = FunctionBlockDiagram {
            nodes: [
                (
                    1,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Input,
                        local_id: 1,
                        negated: false,
                        expression: "a".into(),
                        execution_order_id: None,
                        ref_local_id: None,
                    }),
                ),
                (
                    2,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Input,
                        local_id: 2,
                        negated: false,
                        expression: "b".into(),
                        execution_order_id: None,
                        ref_local_id: None,
                    }),
                ),
                (
                    3,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Input,
                        local_id: 3,
                        negated: false,
                        expression: "c".into(),
                        execution_order_id: None,
                        ref_local_id: Some(7),
                    }),
                ),
                (
                    4,
                    Node::FunctionBlockVariable(FunctionBlockVariable {
                        kind: crate::model::variables::VariableKind::Input,
                        local_id: 4,
                        negated: false,
                        expression: "a".into(),
                        execution_order_id: None,
                        ref_local_id: Some(8),
                    }),
                ),
                (
                    5,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Source,
                        name: "connection1".into(),
                        local_id: 5,
                        ref_local_id: Some(1),
                        formal_parameter: None,
                    }),
                ),
                (
                    6,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Source,
                        name: "connection1".into(),
                        local_id: 6,
                        ref_local_id: Some(2),
                        formal_parameter: None,
                    }),
                ),
                (
                    7,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Sink,
                        name: "connection1".into(),
                        local_id: 7,
                        ref_local_id: Some(5),
                        formal_parameter: None,
                    }),
                ),
                (
                    8,
                    Node::Connector(Connector {
                        kind: crate::model::connector::ConnectorKind::Sink,
                        name: "connection1".into(),
                        local_id: 8,
                        ref_local_id: Some(6),
                        formal_parameter: None,
                    }),
                ),
            ]
            .into(),
        };
        pou.body.function_block_diagram = Some(fbd);
        model.pous.push(pou);

        model.desugar(&source_location_factory).unwrap();
        assert_debug_snapshot!(model)
    }
}
