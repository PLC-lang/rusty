use std::{borrow::Cow, str::FromStr};

use quick_xml::events::Event;
use rustc_hash::FxHashMap;

use crate::{
    error::Error,
    extensions::{GetOrErr, TryToString},
    reader::Reader,
    xml_parser::{get_attributes, Parseable},
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct Control<'xml> {
    pub kind: ControlKind,
    pub name: Option<Cow<'xml, str>>,
    pub local_id: usize,
    pub ref_local_id: Option<usize>,
    pub execution_order_id: Option<usize>,
    pub negated: bool,
}

impl<'xml> Control<'xml> {
    pub fn new(mut hm: FxHashMap<String, String>, kind: ControlKind) -> Result<Self, Error> {
        Ok(Self {
            kind,
            name: hm.remove("label").map(Cow::from),
            local_id: hm.get_or_err("localId").map(|it| it.parse())??,
            ref_local_id: hm.get("refLocalId").map(|it| it.parse()).transpose()?,
            execution_order_id: hm.get("executionOrderId").map(|it| it.parse()).transpose()?,
            negated: hm.get("negated").map(|it| it == "true").unwrap_or(false),
        })
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
pub(crate) enum ControlKind {
    Jump,
    Label,
    Return,
}

impl FromStr for ControlKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "jump" => Ok(ControlKind::Jump),
            "label" => Ok(ControlKind::Label),
            "return" => Ok(ControlKind::Return),
            _ => Err(Error::UnexpectedElement(s.to_string())),
        }
    }
}

impl<'xml> Parseable for Control<'xml> {
    fn visit(reader: &mut Reader, tag: Option<quick_xml::events::BytesStart>) -> Result<Self, Error> {
        let Some(tag) = tag else { unreachable!() };

        let kind = ControlKind::from_str(&tag.name().try_to_string()?)?;
        let mut attributes = get_attributes(tag.attributes())?;
        loop {
            match reader.read_event().map_err(Error::ReadEvent)? {
                Event::Start(tag) => {
                    match tag.name().as_ref() {
                        b"connection" => attributes.extend(get_attributes(tag.attributes())?),

                        // As opposed to e.g. variables where the negation information is directly stored in its
                        // attributes (e.g. `<inVariable negated="false" .../>`) return elements store their
                        // negation information in a seperate nested element called `negated` with the form of
                        // `<negated value="..."/>`.
                        // Hence we search for a negate element and extract its information from their attributes.
                        b"negated" => {
                            let value = get_attributes(tag.attributes())?;
                            attributes.insert(
                                "negated".to_string(),
                                (value.get_or_err("value")? == "true").to_string(),
                            );
                        }
                        _ => {}
                    }
                }
                Event::End(tag) if matches!(tag.name().as_ref(), b"jump" | b"label" | b"return") => break,
                Event::Eof => return Err(Error::UnexpectedEndOfFile(vec![b"block"])),
                _ => {}
            }
        }

        Control::new(attributes, kind)
    }
}
