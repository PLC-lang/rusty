use std::collections::HashMap;

use quick_xml::events::Event;

use crate::{deserializer::Parseable, error::Error, reader::PeekableReader};

use super::{block::Block, connector::Connector, control::Control, variables::FunctionBlockVariable};

#[derive(Debug)]
pub(crate) struct FunctionBlockDiagram {
    pub nodes: HashMap<usize, Node>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Node {
    Block(Block),
    FunctionBlockVariable(FunctionBlockVariable),
    Control(Control),
    Connector(Connector),
}

impl Parseable for FunctionBlockDiagram {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        reader.consume()?;
        // let mut blocks = Vec::new();
        // let mut variables = Vec::new();
        // let mut controls = Vec::new(); // TODO
        // let mut connectors = Vec::new(); // TODO
        let mut nodes = HashMap::new();

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

// impl FunctionBlockDiagram {
//     pub fn sort_by_execution_order(&mut self) {
//         self.blocks.sort_by_key(|it| it.execution_order_id);
//         self.variables.sort_by_key(|it| it.execution_order_id);
//     }
// }
