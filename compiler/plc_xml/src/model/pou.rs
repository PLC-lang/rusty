use std::{borrow::Cow, collections::HashMap, str::FromStr};

use plc_diagnostics::diagnostics::Diagnostic;
use quick_xml::events::Event;

use crate::{error::Error, extensions::GetOrErr, reader::PeekableReader, xml_parser::Parseable};

use super::{action::Action, body::Body, interface::Interface};

#[derive(Debug, Default)]
pub(crate) struct Pou<'xml> {
    pub name: Cow<'xml, str>,
    pub pou_type: PouType,
    pub body: Body<'xml>,
    pub actions: Vec<Action<'xml>>,
    pub interface: Option<Interface>,
}

impl<'xml> Pou<'xml> {
    fn with_attributes(self, attributes: HashMap<String, String>) -> Result<Self, Error> {
        Ok(Pou {
            name: Cow::from(attributes.get_or_err("name")?),
            pou_type: attributes.get_or_err("pouType").map(|it| it.parse())??,
            body: self.body,
            actions: self.actions,
            interface: self.interface,
        })
    }

    pub(crate) fn desugar(
        &mut self,
        source_location_factory: &plc_source::source_location::SourceLocationFactory,
    ) -> Result<(), Vec<Diagnostic>> {
        if let Some(ref mut fbd) = self.body.function_block_diagram {
            fbd.desugar(source_location_factory)
        } else {
            Ok(())
        }
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

impl<'xml> Parseable for Pou<'xml> {
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
                        if let Some(interface) = pou.interface {
                            pou.interface = Some(interface.append_end_keyword(&pou.pou_type));
                        }

                        // TODO: change in order to parse INTERFACE, ACTION etc..
                        reader.consume_until(vec![b"pou"])?;
                        return Ok(pou);
                    }

                    _ => reader.consume()?,
                },

                Event::End(tag) if tag.name().as_ref() == b"pou" => return Ok(pou),

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
