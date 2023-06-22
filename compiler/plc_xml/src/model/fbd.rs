use std::cmp::Ordering;

use indexmap::IndexMap;
use plc::lexer::IdProvider;
use quick_xml::events::Event;

use crate::{deserializer::Parseable, error::Error, model::variables::VariableKind, reader::PeekableReader};

use super::{block::Block, connector::Connector, control::Control, variables::FunctionBlockVariable};

pub(crate) type NodeId = usize;
pub(crate) type NodeIndex = IndexMap<NodeId, Node>;

#[derive(Debug, Default)]
pub(crate) struct FunctionBlockDiagram {
    pub nodes: NodeIndex,
}

impl FunctionBlockDiagram {
    pub fn with_temp_vars(mut self) -> Self {
        // get an id provider set to the last node id in the collection
        let mut id_provider = IdProvider::with_offset(self.latest_id() + 1);

        // find all the connections that need to be broken up with a temp variable
        let block_result_references = self.nodes.get_result_refs();
        block_result_references.into_iter().for_each(|(referenced_result, connections)| {
            // create a temporary variable that references the block-output
            let formal_param = format!(
                "__{}", // TODO: might have to mangle here
                self.nodes.get(&referenced_result).map(|it| it.get_name()).unwrap_or_default(),
            );
            let temp_var = Node::FunctionBlockVariable(FunctionBlockVariable {
                kind: super::variables::VariableKind::Temp,
                local_id: id_provider.next_id(),
                negated: false,
                expression: formal_param.clone(),
                execution_order_id: None,
                ref_local_id: Some(referenced_result),
            });

            // update the nodes that previously pointed to that block-output and change them
            // so they now point to the temp variable
            // XXX: do we also need to update/change the formalParameter strings?
            connections.iter().for_each(|connection_id| {
                self.nodes.entry(*connection_id).and_modify(|it| {
                    it.update_references(referenced_result, temp_var.get_id(), &formal_param)
                });
            });

            // insert the newly created temp-var into the fdb NodeIndex
            self.nodes.insert(temp_var.get_id(), temp_var);
        });

        self
    }

    fn latest_id(&self) -> NodeId {
        self.nodes.iter().map(|(id, _)| *id).max().unwrap_or(0)
    }
}

#[derive(Debug, PartialEq)]
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
            (None, Some(_)) => Some(Ordering::Greater),
            (Some(_), None) => Some(Ordering::Less),
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

    fn update_references(&mut self, previous_ref: NodeId, new_ref: NodeId, new_formal_param: &str) {
        match self {
            Node::Block(val) => val
                .variables
                .iter_mut()
                .filter(|it| it.ref_local_id.is_some_and(|it| it == previous_ref))
                .for_each(|var| {
                    var.ref_local_id = Some(new_ref);
                    var.formal_parameter = new_formal_param.into()
                }),
            Node::Control(_) => unimplemented!(),
            Node::Connector(_) => unimplemented!(),
            Node::FunctionBlockVariable(_) => unreachable!(),
        };
    }

    fn get_name(&self) -> String {
        if let Node::Block(val) = self {
            // TODO: check if the out variables are named after the type- or instance-name
            val.type_name.clone()
        } else {
            "".into()
        }
    }

    pub(crate) fn is_temp_var(&self) -> bool {
        let Node::FunctionBlockVariable(var) = self else {
            return false
        };

        matches!(var.kind, VariableKind::Temp)
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

        // Ok(FunctionBlockDiagram { blocks, variables, controls, connectors })
        Ok(FunctionBlockDiagram { nodes })
    }
}

trait DirectConnection {
    fn get_result_refs(&self) -> IndexMap<NodeId, Vec<NodeId>>;
}

impl DirectConnection for NodeIndex {
    fn get_result_refs(&self) -> IndexMap<NodeId, Vec<NodeId>> {
        let mut connections = IndexMap::new();
        self.iter().for_each(|(node_id, node)| {
            // XXX: assumption: ref_local_id pointing to another block should point to a temp-var of block's result instead
            if let Node::Block(block) = node {
                block
                    .variables
                    .iter()
                    .filter(|var| {
                        var.ref_local_id.is_some_and(|id| matches!(self.get(&id), Some(Node::Block(_))))
                    })
                    .for_each(|var| {
                        let entry = connections.entry(var.ref_local_id.unwrap()).or_insert(vec![]);
                        entry.push(*node_id);
                    });
            }
        });

        connections
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{
        deserializer::Parseable,
        model::fbd::FunctionBlockDiagram,
        reader::PeekableReader,
        serializer::{
            XBlock, XConnection, XConnectionPointIn, XConnectionPointOut, XExpression, XFbd, XInOutVariables,
            XInVariable, XInputVariables, XOutVariable, XOutputVariables, XPosition, XRelPosition, XVariable,
        },
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

        // FIXME: This test is currently wrong, because refLocalId isn't parsed, i.e. it's None in the snapshot
        let mut reader = PeekableReader::new(&content);
        assert_debug_snapshot!(FunctionBlockDiagram::visit(&mut reader).unwrap());
    }
}
