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

    pub fn with_temp_vars(self) -> Self {
        let body = self.body.with_temp_vars(&self.name);
        Pou {
            name: self.name,
            pou_type: self.pou_type,
            body,
            actions: self.actions, // TODO: also create temp vars where needed,
            interface: self.interface,
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

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{
        deserializer::Parseable,
        model::pou::Pou,
        reader::PeekableReader,
        serializer::{
            XAddData, XBody, XContent, XData, XFbd, XInterface, XLocalVars, XPou, XTextDeclaration,
        },
    };

    #[test]
    fn empty() {
        let content = XPou::new()
            .with_attribute("xmlns", "http://www.plcopen.org/xml/tc6_0201")
            .with_attribute("name", "foo")
            .with_attribute("pouType", "program")
            .with_interface(
                XInterface::new().with_local_vars(XLocalVars::new().close()).with_add_data(
                    XAddData::new().with_data_data(
                        XData::new()
                            .with_attribute("name", "www.bachmann.at/plc/plcopenxml")
                            .with_attribute("handleUnknown", "implementation")
                            .with_text_declaration(XTextDeclaration::new().with_content(
                                XContent::new().with_data(
                                    r#"
PROGRAM foo
VAR

END_VAR
                    "#,
                                ),
                            )),
                    ),
                ),
            )
            .with_body(XBody::new().with_fbd(XFbd::new().close()))
            .serialize();

        let mut reader = PeekableReader::new(&content);
        assert_debug_snapshot!(Pou::visit(&mut reader));
    }

    #[test]
    fn poutype_program() {
        let content =
            XPou::new().with_attribute("name", "foo").with_attribute("pouType", "program").serialize();

        let mut reader = PeekableReader::new(&content);
        assert_debug_snapshot!(Pou::visit(&mut reader));
    }

    #[test]
    fn poutype_function() {
        let content =
            XPou::new().with_attribute("name", "foo").with_attribute("pouType", "function").serialize();

        let mut reader = PeekableReader::new(&content);
        assert_debug_snapshot!(Pou::visit(&mut reader));
    }

    #[test]
    fn poutype_function_block() {
        let content =
            XPou::new().with_attribute("name", "foo").with_attribute("pouType", "functionBlock").serialize();

        let mut reader = PeekableReader::new(&content);
        assert_debug_snapshot!(Pou::visit(&mut reader));
    }

    #[test]
    fn poutype_unknown() {
        let content =
            XPou::new().with_attribute("name", "foo").with_attribute("pouType", "asdasd").serialize();

        let mut reader = PeekableReader::new(&content);
        assert_eq!(
            Pou::visit(&mut reader).unwrap_err().to_string(),
            "Found an unexpected element 'asdasd'".to_string()
        );
    }
}
