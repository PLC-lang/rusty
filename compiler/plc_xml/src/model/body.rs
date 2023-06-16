use std::collections::HashMap;

use quick_xml::events::Event;

use super::fbd::FunctionBlockDiagram;
use crate::{deserializer::Parseable, error::Error, reader::PeekableReader};

#[derive(Debug, Default)]
pub(crate) struct Body {
    pub function_block_diagram: Option<FunctionBlockDiagram>,
    // pub global_id: Option<usize>,
}

impl Body {
    fn new(_hm: HashMap<String, String>, fbd: Option<FunctionBlockDiagram>) -> Result<Self, Error> {
        Ok(Self {
            function_block_diagram: fbd,
            // global_id: hm.get("globalId").map(|it| it.parse()).transpose()?,
        })
    }

    fn empty() -> Result<Self, Error> {
        Ok(Self { function_block_diagram: None })
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

                        return Body::new(attributes, Some(fbd));
                    }
                    _ => reader.consume()?,
                },
                Event::Empty(tag) if tag.name().as_ref() == b"FBD" => return Body::empty(),
                Event::Eof => todo!("error-handling"),
                _ => reader.consume()?,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{
        deserializer::Parseable,
        model::body::Body,
        reader::PeekableReader,
        serializer::{XBody, XFbd},
    };

    #[test]
    fn empty() {
        let content = XBody::new().with_fbd(XFbd::new()).serialize();
        todo!("an empty body actually does not have an FDB inside it");

        let mut reader = PeekableReader::new(&content);
        assert_debug_snapshot!(Body::visit(&mut reader).unwrap());
    }
}
