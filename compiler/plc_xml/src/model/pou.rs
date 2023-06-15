use std::{collections::HashMap, str::FromStr};

use quick_xml::events::Event;

use crate::{
    deserializer::{GetOrErr, Parseable},
    error::Error,
    reader::PeekableReader,
};

use super::{action::Action, body::Body, interface::Interface};

// todo: change declaration string to interface
#[derive(Debug, Default)]
pub(crate) struct Pou {
    pub name: String,
    pub pou_type: PouType,
    pub body: Body,
    pub actions: Vec<Action>,
    pub interface: Option<Interface>,
}

impl Pou {
    fn with_attributes(self, attributes: HashMap<String, String>) -> Result<Self, Error> {
        Ok(Pou {
            name: attributes.get_or_err("name")?,
            pou_type: attributes.get_or_err("pouType").map(|it| it.parse())??,
            body: self.body,
            actions: self.actions,
            interface: self.interface,
        })
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub(crate) enum PouType {
    #[default]
    Program,
    Function,
    FunctionBlock,
}

impl std::fmt::Display for PouType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PouType::Program => write!(f, "PROGRAM"),
            PouType::Function => write!(f, "FUNCTION"),
            PouType::FunctionBlock => write!(f, "FUNCTION_BLOCK"),
        }
    }
}

impl TryFrom<&str> for PouType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "program" => Ok(PouType::Program),
            "function" => Ok(PouType::Function),
            "functionBlock" => Ok(PouType::FunctionBlock),
            _ => Err(Error::UnexpectedElement(value.to_string())),
        }
    }
}

impl Parseable for Pou {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        let mut pou = Pou::default().with_attributes(reader.attributes()?)?;
        loop {
            match reader.peek()? {
                Event::Start(tag) => match tag.name().as_ref() {
                    b"interface" => {
                        // XXX: this is very specific to our own xml schema, but does not adhere to the plc open standard
                        reader.consume_until_start(b"content")?;
                        match reader.next()? {
                            Event::Start(tag) => {
                                pou.interface = Some(Interface::new(&reader.read_text(tag.name())?))
                            }
                            _ => reader.consume()?,
                        }
                    }
                    b"body" => {
                        pou.body = Body::visit(reader)?;
                        // TODO: change in order to parse INTERFACE, ACTION etc..
                        reader.consume_until(vec![b"pou"])?;
                        return Ok(pou);
                    }

                    _ => reader.consume()?,
                },

                _ => reader.consume()?,
            }
        }
    }
}

impl FromStr for PouType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "program" => Ok(PouType::Program),
            "function" => Ok(PouType::Function),
            "functionBlock" => Ok(PouType::FunctionBlock),
            _ => Err(Error::UnexpectedElement(s.to_string())),
        }
    }
}
