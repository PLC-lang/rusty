use std::collections::HashMap;

use quick_xml::events::Event;

use crate::{error::Error, extensions::GetOrErr, reader::PeekableReader, xml_parser::Parseable};

use super::variables::BlockVariable;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Block {
    pub local_id: usize,
    pub type_name: String,
    pub instance_name: Option<String>,
    pub execution_order_id: Option<usize>,
    pub variables: Vec<BlockVariable>,
}

impl Block {
    pub fn new(mut hm: HashMap<String, String>, variables: Vec<BlockVariable>) -> Result<Self, Error> {
        Ok(Self {
            local_id: hm.get_or_err("localId").map(|it| it.parse())??,
            type_name: hm.get_or_err("typeName")?,
            instance_name: hm.remove("instanceName"),
            execution_order_id: hm.get("executionOrderId").map(|it| it.parse()).transpose()?,
            variables,
        })
    }
}

impl Parseable for Block {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        let attributes = reader.attributes()?;
        let mut variables = Vec::new();

        loop {
            match reader.peek()? {
                Event::Start(tag) => match tag.name().as_ref() {
                    b"inputVariables" | b"outputVariables" | b"inOutVariables" => {
                        variables.extend(BlockVariable::visit(reader)?)
                    }
                    _ => reader.consume()?,
                },

                Event::End(tag) if tag.name().as_ref() == b"block" => {
                    reader.consume()?;
                    break;
                }

                Event::Eof => return Err(Error::UnexpectedEndOfFile(vec![b"block"])),
                _ => reader.consume()?,
            }
        }

        Block::new(attributes, variables)
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{
        model::block::Block,
        reader::PeekableReader,
        serializer::{XBlock, XInOutVariables, XInputVariables, XOutputVariables, XVariable},
        xml_parser::Parseable,
    };

    #[test]
    fn add_block() {
        let content = XBlock::init("1", "ADD", "0")
            .with_input_variables(
                XInputVariables::new()
                    .with_variable(XVariable::init("a", false).with_connection_in_initialized("1"))
                    .with_variable(XVariable::init("b", false).with_connection_in_initialized("2")),
            )
            .with_inout_variables(XInOutVariables::new().close())
            .with_output_variables(
                XOutputVariables::new()
                    .with_variable(XVariable::init("c", false).with_connection_out_initialized()),
            )
            .serialize();

        let mut reader = PeekableReader::new(&content);
        assert_debug_snapshot!(Block::visit(&mut reader).unwrap());
    }
}
