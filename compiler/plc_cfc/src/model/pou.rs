use std::{collections::HashMap, str::FromStr};

use quick_xml::events::Event;

use crate::{
    deserializer::{GetOrErr, Parseable},
    error::Error,
    reader::PeekableReader,
};

use super::body::Body;

#[derive(Debug)]
pub(crate) struct Pou {
    // TODO: interface, action
    pub name: String,
    pub pou_type: PouType,
    pub body: Body,
    pub declaration: String,
}

impl Pou {
    pub fn new(hm: HashMap<String, String>, body: Body, declaration: String) -> Result<Self, Error> {
        Ok(Self {
            name: hm.get_or_err("name")?,
            pou_type: hm.get_or_err("pouType").map(|it| it.parse())??,
            body,
            declaration,
        })
    }

    // pub fn sort_by_execution_order(mut self) -> Self {
    //     self.body.function_block_diagram.sort_by_execution_order();

    //     self
    // }
}

#[derive(Debug)]
pub(crate) enum PouType {
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
        let attributes = reader.attributes()?;
        let mut declaration = String::new();
        loop {
            match reader.peek()? {
                Event::Start(tag) => match tag.name().as_ref() {
                    b"interface" => {
                        reader.consume_until_start(b"content")?;
                        match reader.next()? {
                            Event::Start(tag) => declaration = reader.read_text(tag.name())?,
                            _ => reader.consume()?,
                        }
                    }
                    b"body" => {
                        let body = Body::visit(reader)?;
                        // TODO: change in order to parse INTERFACE, ACTION etc..
                        reader.consume_until(vec![b"pou"])?;
                        let mut pou = Pou::new(attributes, body, declaration)?;

                        // XXX: Should we explicitly check if the declaration variable is empty or not
                        // We have to append a END_... to the declaration as it is missing by default
                        pou.declaration = format!("{}END_{}", pou.declaration, pou.pou_type.to_string());
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
