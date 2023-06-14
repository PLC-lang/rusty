use std::collections::HashMap;

use quick_xml::events::Event;

use crate::{
    deserializer::{GetOrErr, Parseable, PrototypingToString},
    error::Error,
    reader::PeekableReader,
};

#[derive(Debug, PartialEq)]
pub(crate) struct Connector {
    pub kind: ConnectorKind,
    pub name: String,
    pub local_id: usize,
    pub ref_local_id: Option<usize>,
    pub global_id: Option<usize>,
    pub formal_parameter: Option<String>,
}

impl Connector {
    pub fn new(mut hm: HashMap<String, String>, kind: ConnectorKind) -> Result<Self, Error> {
        Ok(Self {
            kind,
            name: hm.get_or_err("name")?,
            local_id: hm.get_or_err("localId").map(|it| it.parse())??,
            ref_local_id: hm.get("refLocalId").map(|it| it.parse()).transpose()?,
            global_id: hm.get("globalId").map(|it| it.parse()).transpose()?,
            formal_parameter: hm.remove("formalParameter"),
        })
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum ConnectorKind {
    Source,
    Sink,
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
