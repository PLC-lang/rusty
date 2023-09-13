use std::{borrow::Cow, collections::HashMap};

use quick_xml::events::Event;

use crate::{
    error::Error,
    extensions::{GetOrErr, TryToString},
    reader::PeekableReader,
    xml_parser::Parseable,
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct Connector<'xml> {
    pub kind: ConnectorKind,
    pub name: Cow<'xml, str>,
    pub local_id: usize,
    pub ref_local_id: Option<usize>,
    pub formal_parameter: Option<Cow<'xml, str>>,
}

impl<'xml> Connector<'xml> {
    pub fn new(mut hm: HashMap<String, String>, kind: ConnectorKind) -> Result<Self, Error> {
        Ok(Self {
            kind,
            name: Cow::from(hm.get_or_err("name")?),
            local_id: hm.get_or_err("localId").map(|it| it.parse())??,
            ref_local_id: hm.get("refLocalId").map(|it| it.parse()).transpose()?,
            formal_parameter: hm.remove("formalParameter").map(Cow::from),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum ConnectorKind {
    Source,
    Sink,
}

impl<'xml> Parseable for Connector<'xml> {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        let next = reader.peek()?;
        let kind = match &next {
            Event::Start(tag) | Event::Empty(tag) => match tag.name().as_ref() {
                b"connector" => ConnectorKind::Source,
                b"continuation" => ConnectorKind::Sink,
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
