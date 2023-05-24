use std::{borrow::Cow, collections::HashMap};

use quick_xml::{events::Event, name::QName};

use crate::{
    model::{
        Block, BlockVariable, Body, Error, FunctionBlockDiagram, FunctionBlockVariable, Pou, PouType,
        VariableKind,
    },
    reader::PeekableReader,
};

#[test]
fn demo() {
    let res = visit(CONTENT);
    insta::assert_debug_snapshot!(res);
}

trait PrototypingToString {
    fn to_string(self) -> Result<String, Error>;
}

impl<'a> PrototypingToString for QName<'a> {
    fn to_string(self) -> Result<String, Error> {
        String::from_utf8(self.into_inner().to_vec()).map_err(|err| Error::Encoding(err.utf8_error()))
    }
}

impl PrototypingToString for Cow<'_, [u8]> {
    fn to_string(self) -> Result<String, Error> {
        String::from_utf8(self.to_vec()).map_err(|err| Error::Encoding(err.utf8_error()))
    }
}

trait Parseable {
    type Item;
    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error>;
}

pub fn visit(content: &str) -> Result<Pou, Error> {
    let mut reader = PeekableReader::new(content);
    loop {
        match reader.peek()? {
            Event::Start(tag) if tag.name().as_ref() == b"pou" => return Pou::visit(&mut reader),
            Event::Eof => return Err(Error::UnexpectedEndOfFile),
            _ => reader.consume()?,
        }
    }
}

impl Parseable for Pou {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        let attributes = extract_attributes(reader.next()?)?;
        loop {
            match reader.peek()? {
                Event::Start(tag) => match tag.name().as_ref() {
                    b"body" => {
                        let body = Body::visit(reader)?;
                        // TODO: change in order to parse INTERFACE, ACTION etc..
                        reader.consume_until(vec![b"pou"])?;
                        return Ok(Pou {
                            name: attributes.get_or_error("name")?,
                            pou_type: PouType::try_from(attributes.get_or_error("pouType")?.as_str())?,
                            body,
                        });
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
        reader.consume()?;

        loop {
            match reader.peek()? {
                Event::Start(tag) => match tag.name().as_ref() {
                    b"FBD" => {
                        let fbd = FunctionBlockDiagram::visit(reader)?;
                        // consume
                        reader.consume_until(vec![b"body"])?;
                        return Ok(Body { function_block_diagram: fbd });
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
        let next = reader.next()?;
        let kind = match &next {
            Event::Start(tag) | Event::Empty(tag) => match tag.name().as_ref() {
                b"inVariable" => VariableKind::Input,
                b"outVariable" => VariableKind::Output,
                b"inOutVariable" => VariableKind::InOut,
                _ => unreachable!(),
            },

            _ => unreachable!(),
        };

        let mut attributes = extract_attributes(next)?;
        loop {
            match reader.peek()? {
                Event::Text(tag) => {
                    attributes.insert(
                        "expression".into(),
                        String::from_utf8(tag.as_ref().to_vec())
                            .map_err(|err| Error::Encoding(err.utf8_error()))?,
                    );
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

        Ok(FunctionBlockVariable {
            kind,
            local_id: attributes.get_or_error("localId")?,
            negated: attributes.get_or_error("negated")?,
            expression: attributes.get_or_error("expression")?,
            execution_order_id: attributes.get("executionOrderId").cloned(),
            ref_local_id: attributes.get("refLocalId").cloned(),
        })
    }
}

impl Parseable for FunctionBlockDiagram {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        reader.consume()?;
        let mut blocks = Vec::new();
        let mut variables = Vec::new();

        loop {
            match reader.peek()? {
                Event::Start(tag) => match tag.name().as_ref() {
                    b"block" => blocks.push(Block::visit(reader)?),
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

        Ok(FunctionBlockDiagram { blocks, variables })
    }
}

impl Parseable for Block {
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        let attributes = extract_attributes(reader.next()?)?;
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
                _ => reader.consume()?,
            }
        }

        Ok(Block {
            local_id: attributes.get_or_error("localId")?,
            type_name: attributes.get_or_error("typeName")?,
            instance_name: attributes.get("instanceName").cloned(),
            variables,
            execution_order_id: attributes.get("executionOrderId").cloned(),
        })
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
                    // reader.consume_until(vec![b"variable"])?;

                    res.push(BlockVariable {
                        kind,
                        formal_parameter: attributes.get_or_error("formalParameter")?,
                        negated: attributes.get_or_error("negated")?,
                        ref_local_id: attributes.get("refLocalId").cloned(),
                    });
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

                Event::Eof => return Err(Error::UnexpectedEndOfFile),
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
                b"variable" | b"connection" => attributes.extend(extract_attributes(reader.next()?)?),
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

pub(crate) fn extract_attributes(event: Event) -> Result<HashMap<String, String>, Error> {
    let tag = match event {
        Event::Start(tag) | Event::Empty(tag) => tag,
        _ => todo!(),
    };

    let mut hm = HashMap::new();
    for it in tag.attributes().flatten() {
        hm.insert(it.key.to_string()?, it.value.to_string()?);
    }

    Ok(hm)
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
