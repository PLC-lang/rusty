use std::borrow::Cow;

use quick_xml::events::{BytesStart, Event};
use rustc_hash::FxHashMap;

use crate::{
    error::Error,
    extensions::GetOrErr,
    reader::Reader,
    xml_parser::{get_attributes, Parseable},
};

use super::variables::BlockVariable;

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct Block<'xml> {
    pub local_id: usize,
    pub type_name: Cow<'xml, str>,
    pub instance_name: Option<Cow<'xml, str>>,
    pub execution_order_id: Option<usize>,
    pub variables: Vec<BlockVariable>,
}

impl<'xml> Block<'xml> {
    pub fn new(mut hm: FxHashMap<String, String>, variables: Vec<BlockVariable>) -> Result<Self, Error> {
        Ok(Self {
            local_id: hm.get_or_err("localId").map(|it| it.parse())??,
            type_name: Cow::from(hm.get_or_err("typeName")?),
            instance_name: hm.remove("instanceName").map(Cow::from),
            execution_order_id: hm.get("executionOrderId").map(|it| it.parse()).transpose()?,
            variables,
        })
    }
}

impl<'xml> Parseable for Block<'xml> {
    fn visit(reader: &mut Reader, tag: Option<BytesStart>) -> Result<Self, Error> {
        let Some(tag) = tag else { unreachable!() };
        let attributes = get_attributes(tag.attributes())?;
        let mut variables = Vec::new();

        loop {
            match reader.read_event().map_err(Error::ReadEvent)? {
                Event::Start(tag) => match tag.name().as_ref() {
                    b"inputVariables" | b"outputVariables" | b"inOutVariables" => {
                        let new_vars: Vec<BlockVariable> = Parseable::visit(reader, Some(tag))?;
                        variables.extend(new_vars);
                    }
                    _ => {}
                },

                Event::End(tag) if tag.name().as_ref() == b"block" => {
                    break;
                }

                Event::Eof => return Err(Error::UnexpectedEndOfFile(vec![b"block"])),
                _ => {}
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
        reader::{get_start_tag, Reader},
        serializer::{SBlock, SVariable},
        xml_parser::Parseable,
    };

    #[test]
    fn add_block() {
        let content = SBlock::init("ADD", 1, 0)
            .with_input(vec![
                &SVariable::new().with_name("a").connect(1),
                &SVariable::new().with_name("b").connect(2),
            ])
            .with_output(vec![&SVariable::new().with_name("c")])
            .serialize();

        let mut reader = Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        assert_debug_snapshot!(Block::visit(&mut reader, tag).unwrap());
    }
}
