use std::cmp::Ordering;

use indexmap::IndexMap;
use quick_xml::events::Event;

use super::{block::Block, connector::Connector, control::Control, variables::FunctionBlockVariable};
use crate::{error::Error, reader::PeekableReader, xml_parser::Parseable};

/// Represent either a `localId` or `refLocalId`
pub(crate) type NodeId = usize;
pub(crate) type NodeIndex = IndexMap<NodeId, Node>;

#[derive(Debug, Default)]
pub(crate) struct FunctionBlockDiagram {
    pub nodes: NodeIndex,
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
        Ok(FunctionBlockDiagram { nodes })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::serializer2::{YBlock, YFbd, YInVariable, YOutVariable, YVariable};
    use crate::{model::fbd::FunctionBlockDiagram, reader::PeekableReader, xml_parser::Parseable};

    #[test]
    fn add_block() {
        let content = YFbd::new()
            .children(vec![
                &YBlock::init("ADD", 1, 0)
                    .with_input_variables(vec![
                        &YVariable::new().with_name("a").connect_in(1),
                        &YVariable::new().with_name("b").connect_in(2),
                    ])
                    .with_output_variables(vec![&YVariable::new().with_name("c")]),
                &YInVariable::new().with_id(2).with_expression("a"),
                &YInVariable::new().with_id(3).with_expression("b"),
                &YOutVariable::new().with_id(4).with_expression("c").with_execution_id(1).connect_in(1),
            ])
            .serialize();

        println!("{content}");
        let mut reader = PeekableReader::new(&content);
        assert_debug_snapshot!(FunctionBlockDiagram::visit(&mut reader).unwrap());
    }
}
