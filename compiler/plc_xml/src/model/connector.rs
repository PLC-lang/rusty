use std::borrow::Cow;

use quick_xml::events::Event;
use rustc_hash::FxHashMap;

use crate::{
    error::Error,
    extensions::{GetOrErr, TryToString},
    reader::Reader,
    xml_parser::{get_attributes, Parseable},
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
    pub fn new(mut hm: FxHashMap<String, String>, kind: ConnectorKind) -> Result<Self, Error> {
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
    fn visit(reader: &mut Reader, tag: Option<quick_xml::events::BytesStart>) -> Result<Self, Error> {
        let Some(tag) = tag else { unreachable!() };

        let kind = match tag.name().as_ref() {
            b"connector" => ConnectorKind::Source,
            b"continuation" => ConnectorKind::Sink,
            _ => return Err(Error::UnexpectedElement(tag.name().try_to_string()?)),
        };

        let mut attributes = get_attributes(tag.attributes())?;
        loop {
            match reader.read_event().map_err(Error::ReadEvent)? {
                Event::Start(tag) | Event::Empty(tag) if tag.name().as_ref() == b"connection" => {
                    attributes.extend(get_attributes(tag.attributes())?)
                }
                Event::End(tag) if matches!(tag.name().as_ref(), b"connector" | b"continuation") => break,
                _ => {}
            }
        }
        Connector::new(attributes, kind)
    }
}
