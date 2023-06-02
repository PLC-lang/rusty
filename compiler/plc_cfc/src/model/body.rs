use std::collections::HashMap;

use quick_xml::events::Event;

use super::fbd::FunctionBlockDiagram;
use crate::{deserializer::Parseable, error::Error, reader::PeekableReader};

#[derive(Debug)]
pub(crate) struct Body {
    pub function_block_diagram: FunctionBlockDiagram,
    pub global_id: Option<usize>,
}

impl Body {
    pub fn new(hm: HashMap<String, String>, fbd: FunctionBlockDiagram) -> Result<Self, Error> {
        Ok(Self {
            function_block_diagram: fbd,
            global_id: hm.get("globalId").map(|it| it.parse()).transpose()?,
        })
    }
}

impl Parseable for Body {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        let attributes = reader.attributes()?;
        loop {
            match reader.peek()? {
                Event::Start(tag) => match tag.name().as_ref() {
                    b"FBD" => {
                        let fbd = FunctionBlockDiagram::visit(reader)?;
                        reader.consume_until(vec![b"body"])?;

                        return Body::new(attributes, fbd);
                    }
                    _ => reader.consume()?,
                },

                Event::Eof => todo!(),
                _ => reader.consume()?,
            }
        }
    }
}
