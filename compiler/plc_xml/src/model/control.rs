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
                    // TODO: Hella ugly
                    b"negated" => {
                        // While variables have the negated field as an attribute on their element, e.g.
                        // <inVariable negated="..."> the return-control statement has it inside its data field
                        // e.g. <addData><data><negated value="true">, which is hella weird - I want whatever
                        // they smoked when they designed that XSD
                        let value = reader.attributes()?;
                        attributes.insert(
                            "negated".to_string(),
                            (value.get("value").unwrap() == "true").to_string(),
                        );
                    }
                    b"connection" => attributes.extend(reader.attributes()?),
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

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{deserializer::Parseable, reader::PeekableReader};

    use super::Control;

    #[test]
    fn simple_return() {
        let content = r#"
            <return localId="13" height="20" width="76" executionOrderId="0">
                <position x="350" y="100"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="12"/>
                </connectionPointIn>
                <addData>
                    <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                        <negated value="false"/>
                    </data>
                </addData>
            </return>
        "#;

        let mut reader = PeekableReader::new(content);
        assert_debug_snapshot!(Control::visit(&mut reader))
    }

    #[test]
    fn simple_return_negated() {
        let content = r#"
            <return localId="13" height="20" width="76" executionOrderId="0">
                <position x="350" y="100"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="12"/>
                </connectionPointIn>
                <addData>
                    <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                        <negated value="true"/>
                    </data>
                </addData>
            </return>
        "#;

        let mut reader = PeekableReader::new(content);
        assert_debug_snapshot!(Control::visit(&mut reader))
    }
}
