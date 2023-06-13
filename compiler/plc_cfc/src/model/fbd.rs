use quick_xml::events::Event;

use crate::{deserializer::Parseable, error::Error, reader::PeekableReader};

use super::{block::Block, connector::Connector, control::Control, variables::FunctionBlockVariable};

#[derive(Debug)]
pub(crate) struct FunctionBlockDiagram {
    pub blocks: Vec<Block>,
    pub variables: Vec<FunctionBlockVariable>,
    pub controls: Vec<Control>,
    pub connectors: Vec<Connector>,
}

impl Parseable for FunctionBlockDiagram {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        reader.consume()?;
        let mut blocks = Vec::new();
        let mut variables = Vec::new();
        let mut controls = Vec::new(); // TODO
        let mut connectors = Vec::new(); // TODO

        loop {
            match reader.peek()? {
                Event::Start(tag) => match tag.name().as_ref() {
                    b"block" => blocks.push(Block::visit(reader)?),
                    b"jump" | b"label" | b"return" => controls.push(Control::visit(reader)?),
                    b"inVariable" | b"outVariable" => variables.push(FunctionBlockVariable::visit(reader)?),
                    b"continuation" | b"connector" => connectors.push(Connector::visit(reader)?),
                    _ => reader.consume()?,
                },

                Event::End(tag) if tag.name().as_ref() == b"FBD" => {
                    reader.consume()?;
                    break;
                }
                _ => reader.consume()?,
            }
        }

        Ok(FunctionBlockDiagram { blocks, variables, controls, connectors })
    }
}

impl FunctionBlockDiagram {
    pub fn sort_by_execution_order(&mut self) {
        self.blocks.sort_by_key(|it| it.execution_order_id);
        self.variables.sort_by_key(|it| it.execution_order_id);
    }
}
