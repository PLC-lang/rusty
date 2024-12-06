use quick_xml::events::{BytesStart, Event};

use super::fbd::FunctionBlockDiagram;
use crate::{error::Error, reader::Reader, xml_parser::Parseable};

#[derive(Debug, Default)]
pub(crate) struct Body<'xml> {
    pub function_block_diagram: FunctionBlockDiagram<'xml>,
}

impl<'xml> Body<'xml> {
    fn new(fbd: FunctionBlockDiagram<'xml>) -> Result<Self, Error> {
        Ok(Self { function_block_diagram: fbd })
    }

    fn empty() -> Result<Self, Error> {
        Ok(Self { function_block_diagram: FunctionBlockDiagram::default() })
    }
}

impl Parseable for Body<'_> {
    fn visit(reader: &mut Reader, _tag: Option<BytesStart>) -> Result<Self, Error> {
        let mut body = Body::default();
        loop {
            match reader.read_event().map_err(Error::ReadEvent)? {
                Event::Start(tag) if tag.name().as_ref() == b"FBD" => {
                    body.function_block_diagram = FunctionBlockDiagram::visit(reader, Some(tag))?
                }
                Event::End(tag) if tag.name().as_ref() == b"body" => break,
                Event::Eof => return Err(Error::UnexpectedEndOfFile(vec![b"body"])),
                _ => {}
            }
        }

        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{
        model::body::Body,
        reader::{get_start_tag, Reader},
        serializer::{SBlock, SBody, SVariable},
        xml_parser::Parseable,
    };

    #[test]
    fn empty() {
        let content = SBody::new().with_fbd(vec![]).serialize();

        let mut reader = Reader::new(&content);
        assert_debug_snapshot!(Body::visit(&mut reader, None).unwrap());
    }

    #[test]
    fn fbd_with_add_block() {
        let content = SBody::new()
            .with_fbd(vec![&SBlock::init("ADD", 1, 0)
                .with_input(vec![
                    &SVariable::new().with_name("a").connect(1),
                    &SVariable::new().with_name("b").connect(2),
                ])
                .with_output(vec![&SVariable::new().with_name("c")])
                .with_inout(vec![])])
            .serialize();

        let mut reader = Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        assert_debug_snapshot!(Body::visit(&mut reader, tag).unwrap());
    }
}
