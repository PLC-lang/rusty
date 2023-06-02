use std::{borrow::Cow, collections::HashMap, str::FromStr};

use quick_xml::{events::Event, name::QName};

use crate::{
    error::Error,
    model::{
        Block, BlockVariable, Body, Connector, ConnectorKind, Control, ControlKind, FunctionBlockDiagram,
        FunctionBlockVariable, Pou, VariableKind,
    },
    reader::PeekableReader,
};

pub(crate) trait PrototypingToString {
    fn try_to_string(self) -> Result<String, Error>;
}

impl<'a> PrototypingToString for &'a [u8] {
    fn try_to_string(self) -> Result<String, Error> {
        String::from_utf8(self.as_ref().to_vec()).map_err(|err| Error::Encoding(err.utf8_error()))
    }
}

impl<'a> PrototypingToString for QName<'a> {
    fn try_to_string(self) -> Result<String, Error> {
        String::from_utf8(self.into_inner().to_vec()).map_err(|err| Error::Encoding(err.utf8_error()))
    }
}

impl PrototypingToString for Cow<'_, [u8]> {
    fn try_to_string(self) -> Result<String, Error> {
        String::from_utf8(self.to_vec()).map_err(|err| Error::Encoding(err.utf8_error()))
    }
}

pub(crate) trait Parseable {
    type Item;
    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error>;
}

pub(crate) fn visit(content: &str) -> Result<Pou, Error> {
    let mut reader = PeekableReader::new(content);
    loop {
        match reader.peek()? {
            Event::Start(tag) if tag.name().as_ref() == b"pou" => return Pou::visit(&mut reader),
            Event::Eof => return Err(Error::UnexpectedEndOfFile(vec![b"pou"])),
            _ => reader.consume()?,
        }
    }
}

impl Parseable for Pou {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        let attributes = reader.attributes()?;
        loop {
            match reader.peek()? {
                Event::Start(tag) => match tag.name().as_ref() {
                    b"body" => {
                        let body = Body::visit(reader)?;
                        // TODO: change in order to parse INTERFACE, ACTION etc..
                        reader.consume_until(vec![b"pou"])?;
                        return Pou::new(attributes, body);
                    }

                    _ => reader.consume()?,
                },

                _ => reader.consume()?,
            }
        }
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

impl Parseable for FunctionBlockVariable {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        // peek next token to determine variable kind
        // token will be consumed when extracting attributes later
        let next = reader.peek()?;
        let kind = match &next {
            Event::Start(tag) | Event::Empty(tag) => match tag.name().as_ref() {
                b"inVariable" => VariableKind::Input,
                b"outVariable" => VariableKind::Output,
                b"inOutVariable" => VariableKind::InOut,
                _ => unreachable!(),
            },

            _ => unreachable!(),
        };

        let mut attributes = reader.attributes()?;
        loop {
            match reader.peek()? {
                Event::Text(tag) => {
                    attributes.insert("expression".into(), tag.as_ref().try_to_string()?);
                    reader.consume()?;
                }

                Event::End(tag) => match tag.name().as_ref() {
                    b"inVariable" | b"outVariable" => {
                        reader.consume()?;
                        break;
                    }
                    _ => reader.consume()?,
                },

                _ => reader.consume()?,
            }
        }

        FunctionBlockVariable::new(attributes, kind)
    }
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

impl Parseable for Control {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        let kind = match reader.peek()? {
            Event::Start(tag) | Event::Empty(tag) => ControlKind::from_str(&tag.name().try_to_string()?)?,
            _ => unreachable!(),
        };
        let mut attributes = reader.attributes()?;

        loop {
            match reader.peek()? {
                Event::Start(tag) | Event::Empty(tag) => match tag.name().as_ref() {
                    b"negated" => attributes.extend(reader.attributes()?),
                    b"connection" => attributes.extend(reader.attributes()?),
                    _ => reader.consume()?,
                },

                Event::End(tag) if matches!(tag.name().as_ref(), b"jump" | b"label" | b"return") => {
                    reader.consume()?;
                    break;
                }

                Event::Eof => return Err(Error::UnexpectedEndOfFile(vec![b"block"])),
                _ => reader.consume()?,
            }
        }

        Control::new(attributes, kind)
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

impl Parseable for BlockVariable {
    type Item = Vec<Self>;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        let kind = match reader.next()? {
            Event::Start(tag) | Event::Empty(tag) => VariableKind::try_from(tag.name().as_ref())?,
            _ => unreachable!(),
        };

        let mut res = vec![];

        loop {
            match reader.peek()? {
                Event::Start(tag) if tag.name().as_ref() == b"variable" => {
                    let attributes = visit_variable(reader)?;
                    res.push(BlockVariable::new(attributes, kind)?);
                }

                Event::End(tag)
                    if matches!(
                        tag.name().as_ref(),
                        b"inputVariables" | b"outputVariables" | b"inOutVariables"
                    ) =>
                {
                    reader.consume()?;
                    return Ok(res);
                }

                Event::Eof => {
                    return Err(Error::UnexpectedEndOfFile(vec![
                        b"inputVariables",
                        b"outputVariables",
                        b"inOutVariables",
                    ]))
                }
                _ => reader.consume()?,
            };
        }
    }
}

fn visit_variable(reader: &mut PeekableReader) -> Result<HashMap<String, String>, Error> {
    let mut attributes = HashMap::new();
    loop {
        match reader.peek()? {
            Event::Start(tag) | Event::Empty(tag) => match tag.name().as_ref() {
                b"variable" | b"connection" => attributes.extend(reader.attributes()?),
                _ => reader.consume()?,
            },

            Event::End(tag) if tag.name().as_ref() == b"variable" => {
                reader.consume()?;
                break;
            }
            _ => reader.consume()?,
        }
    }

    Ok(attributes)
}

impl Parseable for Connector {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        let next = reader.peek()?;
        let kind = match &next {
            Event::Start(tag) | Event::Empty(tag) => match tag.name().as_ref() {
                b"connector" => ConnectorKind::Sink,
                b"continuation" => ConnectorKind::Source,
                _ => return Err(Error::UnexpectedElement(tag.name().try_to_string()?)),
            },

            _ => unreachable!(),
        };

        let mut attributes = reader.attributes()?;
        loop {
            match reader.peek()? {
                Event::Start(tag) | Event::Empty(tag) => match tag.name().as_ref() {
                    b"connection" => attributes.extend(reader.attributes()?),
                    _ => reader.consume()?,
                },

                Event::End(tag) if matches!(tag.name().as_ref(), b"connector" | b"continuation") => {
                    reader.consume()?;
                    break;
                }

                _ => reader.consume()?,
            }
        }

        Connector::new(attributes, kind)
    }
}

trait GetOrErr {
    fn get_or_error(&self, key: &str) -> Result<String, Error>;
}

impl GetOrErr for HashMap<String, String> {
    fn get_or_error(&self, key: &str) -> Result<String, Error> {
        self.get(key).map(|it| it.to_owned()).ok_or(Error::MissingAttribute(key.to_string()))
    }
}
