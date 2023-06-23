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
        Ok(Self { function_block_diagram: fbd })
    }

    fn empty() -> Result<Self, Error> {
        Ok(Self { function_block_diagram: None })
    }

    pub fn with_temp_vars(self) -> Self {
        Body { function_block_diagram: self.function_block_diagram.map(|it| it.with_temp_vars()) }
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
        serializer::{XBlock, XBody, XFbd, XInOutVariables, XInputVariables, XOutputVariables, XVariable},
    };

    #[test]
    fn empty() {
        let content = XBody::new().with_fbd(XFbd::new().close()).serialize();

        let mut reader = PeekableReader::new(&content);
        assert_debug_snapshot!(Body::visit(&mut reader).unwrap());
    }

    // TODO: Add two add blocks
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
