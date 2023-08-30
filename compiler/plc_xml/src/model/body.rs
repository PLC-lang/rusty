use quick_xml::events::Event;

use super::fbd::FunctionBlockDiagram;
use crate::{error::Error, reader::PeekableReader, xml_parser::Parseable};

#[derive(Debug, Default)]
pub(crate) struct Body<'xml> {
    pub function_block_diagram: Option<FunctionBlockDiagram<'xml>>,
}

impl<'xml> Body<'xml> {
    fn new(fbd: Option<FunctionBlockDiagram<'xml>>) -> Result<Self, Error> {
        Ok(Self { function_block_diagram: fbd })
    }

    fn empty() -> Result<Self, Error> {
        Ok(Self { function_block_diagram: None })
    }
}

impl<'xml> Parseable for Body<'xml> {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        loop {
            match reader.peek()? {
                Event::Start(tag) => match tag.name().as_ref() {
                    b"FBD" => {
                        let fbd = FunctionBlockDiagram::visit(reader)?;
                        reader.consume_until(vec![b"body"])?;

                        return Body::new(Some(fbd));
                    }
                    _ => reader.consume()?,
                },
                Event::Empty(tag) if tag.name().as_ref() == b"FBD" => return Body::empty(),
                Event::Eof => return Err(Error::UnexpectedEndOfFile(vec![b"body"])),
                _ => reader.consume()?,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{
        model::body::Body,
        reader::PeekableReader,
        serializer::{XBlock, XBody, XFbd, XInOutVariables, XInputVariables, XOutputVariables, XVariable},
        xml_parser::Parseable,
    };

    #[test]
    fn empty() {
        let content = XBody::new().with_fbd(XFbd::new().close()).serialize();

        let mut reader = PeekableReader::new(&content);
        assert_debug_snapshot!(Body::visit(&mut reader).unwrap());
    }

    #[test]
    fn fbd_with_add_block() {
        let content = XBody::new()
            .with_fbd(
                XFbd::new().with_block(
                    XBlock::init("1", "ADD", "0")
                        .with_input_variables(
                            XInputVariables::new()
                                .with_variable(
                                    XVariable::init("a", false).with_connection_in_initialized("1"),
                                )
                                .with_variable(
                                    XVariable::init("b", false).with_connection_in_initialized("2"),
                                ),
                        )
                        .with_inout_variables(XInOutVariables::new().close())
                        .with_output_variables(
                            XOutputVariables::new()
                                .with_variable(XVariable::init("c", false).with_connection_out_initialized()),
                        ),
                ),
            )
            .serialize();

        let mut reader = PeekableReader::new(&content);
        assert_debug_snapshot!(Body::visit(&mut reader).unwrap());
    }
}
