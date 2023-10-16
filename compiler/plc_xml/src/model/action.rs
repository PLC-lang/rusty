use std::borrow::Cow;

use quick_xml::events::Event;

use crate::{
    reader::Reader,
    xml_parser::{get_attributes, Parseable}, error::Error,
};

use super::body::Body;

#[derive(Debug, Default)]
pub(crate) struct Action<'xml> {
    pub name: Cow<'xml, str>,
    pub type_name: Cow<'xml, str>,
    pub body: Body<'xml>,
}

impl Action<'_> {
    pub fn new(name: &str) -> Self {
        Action { name: name.to_string().into(), ..Default::default() }
    }
}

impl Parseable for Vec<Action<'_>> {
    fn visit(
        reader: &mut Reader,
        _tag: Option<quick_xml::events::BytesStart>,
    ) -> Result<Self, crate::error::Error> {
        let mut res = vec![];
        loop {
            match reader.read_event()? {
                Event::Start(tag) if tag.name().as_ref() == b"action" => {
                    res.push(Action::visit(reader, Some(tag))?);
                }
                Event::End(tag) if tag.name().as_ref() == b"actions" => break,
                Event::Eof => return Err(Error::UnexpectedEndOfFile(vec![b"actions"])),
                _ => {}
            }
        }

        Ok(res)
    }
}

impl Parseable for Action<'_> {
    fn visit(
        reader: &mut Reader,
        tag: Option<quick_xml::events::BytesStart>,
    ) -> Result<Self, crate::error::Error> {
        let Some(tag) = tag else {
            unreachable!()
        };

        let attributes = get_attributes(tag.attributes())?;
        let Some(name) = attributes.get("name") else {
            todo!()
        };
        let mut action = Action::new(name);
        loop {
            match reader.read_event()? {
                Event::Start(tag) if tag.name().as_ref() == b"body" => {
                    action.body = Body::visit(reader, Some(tag))?;
                }
                Event::End(tag) if tag.name().as_ref() == b"action" => break,
                Event::Eof => return Err(Error::UnexpectedEndOfFile(vec![b"actions"])),
                _ => {}
            }
        }

        Ok(action)
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::xml_parser;

    #[test]
    fn list_of_actions_parsed_to_model() {
        let src = r###"
<?xml version="1.0" encoding="UTF-8"?>
<pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="program_0" pouType="program">
    <interface>
        <localVars/>
        <addData>
            <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                <textDeclaration>
                    <content>
PROGRAM program_0
VAR
	a : DINT;
END_VAR
					</content>
                </textDeclaration>
            </data>
        </addData>
    </interface>
    <actions>
        <action name="newAction">
            <body>
                <FBD>
                    <block localId="1" width="80" height="60" typeName="foo" executionOrderId="0">
                        <position x="420" y="110"/>
                        <inputVariables>
                            <variable formalParameter="IN1" negated="false">
                                <connectionPointIn>
                                    <relPosition x="0" y="30"/>
                                </connectionPointIn>
                            </variable>
                            <variable formalParameter="IN2" negated="false">
                                <connectionPointIn>
                                    <relPosition x="0" y="50"/>
                                </connectionPointIn>
                            </variable>
                        </inputVariables>
                        <inOutVariables/>
                        <outputVariables>
                            <variable formalParameter="OUT" negated="false">
                                <connectionPointOut>
                                    <relPosition x="80" y="30"/>
                                </connectionPointOut>
                            </variable>
                        </outputVariables>
                    </block>
                </FBD>
            </body>
        </action>
        <action name="newAction2">
            <body>
                <FBD>
                    <inOutVariable localId="1" height="20" width="80" negatedIn="false" storageIn="none" negatedOut="false">
                        <position x="200" y="70"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                        </connectionPointIn>
                        <connectionPointOut>
                            <relPosition x="80" y="10"/>
                        </connectionPointOut>
                        <expression>a</expression>
                    </inOutVariable>
                </FBD>
            </body>
        </action>
    </actions>
    <body>
        <FBD/>
    </body>
</pou>
        "###;

        assert_debug_snapshot!(xml_parser::visit(src).unwrap())
    }
}
