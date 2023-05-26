use std::{borrow::Cow, collections::HashMap, str::FromStr};

use quick_xml::{events::Event, name::QName};

use crate::{
    error::Error,
    model::{
        Block, BlockVariable, Body, Control, ControlKind, FunctionBlockDiagram, FunctionBlockVariable, Pou,
        VariableKind,
    },
    reader::PeekableReader,
};

pub(crate) trait PrototypingToString {
    fn try_to_string(self) -> Result<String, Error>;
}

impl<'a> PrototypingToString for &'a [u8] {
    fn try_to_string(self) -> Result<String, Error> {
        String::from_utf8(self.as_ref().to_vec()).map_err(|err| Error::Encoding(err.utf8_error()))
    }
}

impl<'a> PrototypingToString for QName<'a> {
    fn try_to_string(self) -> Result<String, Error> {
        String::from_utf8(self.into_inner().to_vec()).map_err(|err| Error::Encoding(err.utf8_error()))
    }
}

impl PrototypingToString for Cow<'_, [u8]> {
    fn try_to_string(self) -> Result<String, Error> {
        String::from_utf8(self.to_vec()).map_err(|err| Error::Encoding(err.utf8_error()))
    }
}

trait Parseable {
    type Item;
    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error>;
}

fn visit(content: &str) -> Result<Pou, Error> {
    let mut reader = PeekableReader::new(content);
    loop {
        match reader.peek()? {
            Event::Start(tag) if tag.name().as_ref() == b"pou" => return Pou::visit(&mut reader),
            Event::Eof => return Err(Error::UnexpectedEndOfFile(vec![b"pou"])),
            _ => reader.consume()?,
        }
    }
}

impl Parseable for Pou {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        let attributes = reader.attributes()?;
        loop {
            match reader.peek()? {
                Event::Start(tag) => match tag.name().as_ref() {
                    b"body" => {
                        let body = Body::visit(reader)?;
                        // TODO: change in order to parse INTERFACE, ACTION etc..
                        reader.consume_until(vec![b"pou"])?;
                        return Pou::new(attributes, body);
                    }

                    _ => reader.consume()?,
                },

                _ => reader.consume()?,
            }
        }
    }
}

impl Parseable for Body {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        let attributes = reader.attributes()?;
        loop {
            match reader.peek()? {
                Event::Start(tag) => match tag.name().as_ref() {
                    b"FBD" => {
                        let fbd = FunctionBlockDiagram::visit(reader)?;
                        reader.consume_until(vec![b"body"])?;

                        return Body::new(attributes, fbd);
                    }
                    _ => reader.consume()?,
                },

                Event::Eof => todo!(),
                _ => reader.consume()?,
            }
        }
    }
}

impl Parseable for FunctionBlockVariable {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        // peek next token to determine variable kind
        // token will be consumed when extracting attributes later
        let next = reader.peek()?;
        let kind = match &next {
            Event::Start(tag) | Event::Empty(tag) => match tag.name().as_ref() {
                b"inVariable" => VariableKind::Input,
                b"outVariable" => VariableKind::Output,
                b"inOutVariable" => VariableKind::InOut,
                _ => unreachable!(),
            },

            _ => unreachable!(),
        };

        let mut attributes = reader.attributes()?;
        loop {
            match reader.peek()? {
                Event::Text(tag) => {
                    attributes.insert("expression".into(), tag.as_ref().try_to_string()?);
                    reader.consume()?;
                }

                Event::End(tag) => match tag.name().as_ref() {
                    b"inVariable" | b"outVariable" => {
                        reader.consume()?;
                        break;
                    }
                    _ => reader.consume()?,
                },

                _ => reader.consume()?,
            }
        }

        FunctionBlockVariable::new(attributes, kind)
    }
}

impl Parseable for FunctionBlockDiagram {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        reader.consume()?;
        let mut blocks = Vec::new();
        let mut variables = Vec::new();
        let mut controls = Vec::new(); // TODO

        loop {
            match reader.peek()? {
                Event::Start(tag) => match tag.name().as_ref() {
                    b"block" => blocks.push(Block::visit(reader)?),
                    b"jump" | b"label" | b"return" => controls.push(Control::visit(reader)?),
                    b"inVariable" | b"outVariable" => variables.push(FunctionBlockVariable::visit(reader)?),
                    _ => reader.consume()?,
                },

                Event::End(tag) if tag.name().as_ref() == b"FBD" => {
                    reader.consume()?;
                    break;
                }
                _ => reader.consume()?,
            }
        }

        Ok(FunctionBlockDiagram { blocks, variables, controls })
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
                    b"negated" => attributes.extend(reader.attributes()?),
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

impl Parseable for Block {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        let attributes = reader.attributes()?;
        let mut variables = Vec::new();

        loop {
            match reader.peek()? {
                Event::Start(tag) => match tag.name().as_ref() {
                    b"inputVariables" | b"outputVariables" | b"inOutVariables" => {
                        variables.extend(BlockVariable::visit(reader)?)
                    }
                    _ => reader.consume()?,
                },

                Event::End(tag) if tag.name().as_ref() == b"block" => {
                    reader.consume()?;
                    break;
                }

                Event::Eof => return Err(Error::UnexpectedEndOfFile(vec![b"block"])),
                _ => reader.consume()?,
            }
        }

        Block::new(attributes, variables)
    }
}

impl Parseable for BlockVariable {
    type Item = Vec<Self>;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        let kind = match reader.next()? {
            Event::Start(tag) | Event::Empty(tag) => VariableKind::try_from(tag.name().as_ref())?,
            _ => unreachable!(),
        };

        let mut res = vec![];

        loop {
            match reader.peek()? {
                Event::Start(tag) if tag.name().as_ref() == b"variable" => {
                    let attributes = visit_variable(reader)?;
                    res.push(BlockVariable::new(attributes, kind)?);
                }

                Event::End(tag)
                    if matches!(
                        tag.name().as_ref(),
                        b"inputVariables" | b"outputVariables" | b"inOutVariables"
                    ) =>
                {
                    reader.consume()?;
                    return Ok(res);
                }

                Event::Eof => {
                    return Err(Error::UnexpectedEndOfFile(vec![
                        b"inputVariables",
                        b"outputVariables",
                        b"inOutVariables",
                    ]))
                }
                _ => reader.consume()?,
            };
        }
    }
}

fn visit_variable(reader: &mut PeekableReader) -> Result<HashMap<String, String>, Error> {
    let mut attributes = HashMap::new();
    loop {
        match reader.peek()? {
            Event::Start(tag) | Event::Empty(tag) => match tag.name().as_ref() {
                b"variable" | b"connection" => attributes.extend(reader.attributes()?),
                _ => reader.consume()?,
            },

            Event::End(tag) if tag.name().as_ref() == b"variable" => {
                reader.consume()?;
                break;
            }
            _ => reader.consume()?,
        }
    }

    Ok(attributes)
}

trait GetOrErr {
    fn get_or_error(&self, key: &str) -> Result<String, Error>;
}

impl GetOrErr for HashMap<String, String> {
    fn get_or_error(&self, key: &str) -> Result<String, Error> {
        self.get(key).map(|it| it.to_owned()).ok_or(Error::MissingAttribute(key.to_string()))
    }
}

const CONTENT: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="program_0" pouType="program">
    <!-- <interface>
        <localVars />
        <addData>
            <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                <textDeclaration>
                    <content>
                        PROGRAM program_0
                        VAR
                        local_a : DINT := 1;
                        local_b : DINT := 2;
                        local_c : DINT := 0;
                        local_add : MyAdd;
                        END_VAR</content>
                </textDeclaration>
            </data>
        </addData>
    </interface> -->
    <body>
        <FBD>
            <block localId="5" width="82" height="60" typeName="MyAdd" instanceName="local_add"
                executionOrderId="0">
                <position x="480" y="150" />
                <inputVariables>
                    <variable formalParameter="a" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="30" />
                            <connection refLocalId="6" />
                        </connectionPointIn>
                    </variable>
                    <variable formalParameter="b" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="50" />
                            <connection refLocalId="7" />
                        </connectionPointIn>
                    </variable>
                </inputVariables>
                <inOutVariables />
                <outputVariables>
                    <variable formalParameter="c" negated="false">
                        <connectionPointOut>
                            <relPosition x="82" y="30" />
                        </connectionPointOut>
                    </variable>
                </outputVariables>
            </block>
            <inVariable localId="6" height="20" width="82" negated="false">
                <position x="340" y="170" />
                <connectionPointOut>
                    <relPosition x="82" y="10" />
                </connectionPointOut>
                <expression>local_a</expression>
            </inVariable>
            <inVariable localId="7" height="20" width="82" negated="false">
                <position x="340" y="190" />
                <connectionPointOut>
                    <relPosition x="82" y="10" />
                </connectionPointOut>
                <expression>local_b</expression>
            </inVariable>
            <outVariable localId="8" height="20" width="82" executionOrderId="1" negated="false"
                storage="none">
                <position x="620" y="170" />
                <connectionPointIn>
                    <relPosition x="0" y="10" />
                    <connection refLocalId="5" formalParameter="c" />
                </connectionPointIn>
                <expression>local_c</expression>
            </outVariable>
        </FBD>
    </body>
</pou>
"#;

const CONTENT_WITH_LABELS: &str = r#"
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
	local_a : DINT := 1;
	local_b : DINT := 2;
	local_c:  DINT;
	local_bool : BOOL := TRUE;
	local_add : MyAdd;
END_VAR</content>
                </textDeclaration>
            </data>
        </addData>
    </interface>
    <body>
        <FBD>
            <block localId="1" width="82" height="60" typeName="MyAdd" instanceName="local_add" executionOrderId="0">
                <position x="310" y="150"/>
                <inputVariables>
                    <variable formalParameter="a" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="30"/>
                            <connection refLocalId="2"/>
                        </connectionPointIn>
                    </variable>
                    <variable formalParameter="b" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="50"/>
                            <connection refLocalId="3"/>
                        </connectionPointIn>
                    </variable>
                </inputVariables>
                <inOutVariables/>
                <outputVariables>
                    <variable formalParameter="c" negated="false">
                        <connectionPointOut>
                            <relPosition x="82" y="30"/>
                        </connectionPointOut>
                    </variable>
                </outputVariables>
            </block>
            <inVariable localId="2" height="20" width="106" negated="false">
                <position x="170" y="170"/>
                <connectionPointOut>
                    <relPosition x="106" y="10"/>
                </connectionPointOut>
                <expression>local_a + 1</expression>
            </inVariable>
            <inVariable localId="3" height="20" width="82" negated="false">
                <position x="170" y="190"/>
                <connectionPointOut>
                    <relPosition x="82" y="10"/>
                </connectionPointOut>
                <expression>local_b</expression>
            </inVariable>
            <jump localId="9" height="20" width="87" label="jumpy" executionOrderId="2">
                <position x="390" y="300"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="12"/>
                </connectionPointIn>
                <addData>
                    <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                        <negated value="false"/>
                    </data>
                </addData>
            </jump>
            <label localId="10" height="20" width="80" label="jumpy" executionOrderId="3">
                <position x="530" y="300"/>
            </label>
            <return localId="11" height="20" width="76" executionOrderId="4">
                <position x="790" y="220"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="15"/>
                </connectionPointIn>
                <addData>
                    <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                        <negated value="false"/>
                    </data>
                </addData>
            </return>
            <inVariable localId="12" height="20" width="100" negated="false">
                <position x="240" y="300"/>
                <connectionPointOut>
                    <relPosition x="100" y="10"/>
                </connectionPointOut>
                <expression>local_bool</expression>
            </inVariable>
            <continuation name="wifi" localId="15" height="20" width="116">
                <position x="630" y="220"/>
                <connectionPointOut>
                    <relPosition x="116" y="10"/>
                </connectionPointOut>
            </continuation>
            <connector name="wifi" localId="16" height="20" width="136">
                <position x="460" y="220"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="12"/>
                </connectionPointIn>
            </connector>
            <outVariable localId="17" height="20" width="82" executionOrderId="5" negated="false" storage="none">
                <position x="440" y="170"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="1" formalParameter="c"/>
                </connectionPointIn>
                <expression>local_c</expression>
            </outVariable>
        </FBD>
    </body>
</pou>
"#;

#[test]
fn demo() {
    insta::assert_debug_snapshot!(visit(CONTENT));
}

#[test]
fn labels() {
    insta::assert_debug_snapshot!(visit(CONTENT_WITH_LABELS));
}
