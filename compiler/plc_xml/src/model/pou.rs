use std::{borrow::Cow, collections::HashMap, str::FromStr};

use plc_diagnostics::diagnostics::Diagnostic;
use quick_xml::events::{BytesStart, Event};

use crate::{
    error::Error,
    extensions::GetOrErr,
    reader::Reader,
    xml_parser::{get_attributes, Parseable},
};

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

impl<'xml> Parseable for Pou<'xml> {
    fn visit(reader: &mut Reader, tag: Option<BytesStart>) -> Result<Self, Error> {
        let Some(tag) = tag else {
            unreachable!()
        };
        let attributes = get_attributes(tag.attributes())?;
        let mut pou = Pou::default().with_attributes(attributes)?;
        loop {
            match reader.read_event()? {
                Event::Start(tag) if tag.name().as_ref() == b"interface" => {
                    // XXX: this is very specific to our own xml schema, but does not adhere to the plc open standard
                    pou.interface =
                        Some(Interface::visit(reader, Some(tag))?.append_end_keyword(&pou.pou_type));
                    // reader.consume_until_start(b"content")?;
                    // match reader.next()? {
                    //     Event::Start(tag) => {
                    //         pou.interface = Some(Interface::new(&reader.read_text(tag.name())?))
                    //         pou.interface = Some(pou.interface.unwrap().append_end_keyword(&pou.pou_type));
                    //     }
                    //     _ => reader.consume()?,
                    // }
                }
                Event::Start(tag) if tag.name().as_ref() == b"body" => {
                    pou.body = Body::visit(reader, Some(tag))?;
                }
                Event::End(tag) if tag.name().as_ref() == b"pou" => break,
                Event::Eof => return Err(Error::UnexpectedEndOfFile(vec![b"pou"])),

                _ => {}
            }
        }
        Ok(pou)
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
        model::pou::Pou,
        reader::{get_start_tag, Reader},
        serializer::{
            XAddData, XBody, XContent, XData, XFbd, XInterface, XLocalVars, XPou, XTextDeclaration,
        },
        xml_parser::{self, Parseable},
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

        assert_debug_snapshot!(xml_parser::visit(&content));
    }

    #[test]
    fn poutype_program() {
        let content =
            XPou::new().with_attribute("name", "foo").with_attribute("pouType", "program").serialize();

        let mut reader = Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        assert_debug_snapshot!(Pou::visit(&mut reader, tag));
    }

    #[test]
    fn poutype_function() {
        let content =
            XPou::new().with_attribute("name", "foo").with_attribute("pouType", "function").serialize();

        let mut reader = Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        assert_debug_snapshot!(Pou::visit(&mut reader, tag));
    }

    #[test]
    fn poutype_function_block() {
        let content =
            XPou::new().with_attribute("name", "foo").with_attribute("pouType", "functionBlock").serialize();

        let mut reader = Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        assert_debug_snapshot!(Pou::visit(&mut reader, tag));
    }

    #[test]
    fn poutype_unknown() {
        let content =
            XPou::new().with_attribute("name", "foo").with_attribute("pouType", "asdasd").serialize();

        let mut reader = Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        assert_eq!(
            Pou::visit(&mut reader, tag).unwrap_err().to_string(),
            "Found an unexpected element 'asdasd'".to_string()
        );
    }
}
