use std::{collections::HashMap, str::FromStr};

use quick_xml::events::Event;

use crate::{
    error::Error,
    extensions::{GetOrErr, TryToString},
    reader::PeekableReader,
    xml_parser::Parseable,
};

#[derive(Debug, PartialEq)]
pub(crate) struct Control {
    pub kind: ControlKind,
    pub name: Option<String>,
    pub local_id: usize,
    pub ref_local_id: Option<usize>,
    pub execution_order_id: Option<usize>,
    pub negated: bool,
}

impl Control {
    pub fn new(mut hm: HashMap<String, String>, kind: ControlKind) -> Result<Self, Error> {
        Ok(Self {
            kind,
            name: hm.remove("label"),
            local_id: hm.get_or_err("localId").map(|it| it.parse())??,
            ref_local_id: hm.get("refLocalId").map(|it| it.parse()).transpose()?,
            execution_order_id: hm.get("executionOrderId").map(|it| it.parse()).transpose()?,
            negated: hm.get("negated").map(|it| it == "true").unwrap_or(false),
        })
    }
}

#[derive(Debug, PartialEq)]
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
                    b"connection" => attributes.extend(reader.attributes()?),

                    // As opposed to e.g. inVariables, return elements do not store negation information in
                    // attributes but rather in a seperate sub-element, i.e. `<negated value="false">` inside
                    // the `<addData>` field. Hence when finding such an element we extract its information
                    // from the `value` attribute.
                    b"negated" => {
                        let value = reader.attributes()?;
                        attributes.insert(
                            "negated".to_string(),
                            (value.get_or_err("value")? == "true").to_string(),
                        );
                    }

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
