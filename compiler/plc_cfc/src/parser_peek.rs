use std::{borrow::Cow, collections::HashMap};

use quick_xml::{events::Event, name::QName};

use crate::temp::PeekableReader;

#[test]
fn demo() {
    visit()
}

// pub fn stop_at() {}

pub fn visit() {
    let mut reader = PeekableReader::new(CONTENT);
    loop {
        match reader.peek() {
            Event::Start(tag) if tag.name().as_ref() == b"pou" => visit_pou(&mut reader),
            Event::Eof => break,
            _ => reader.consume(),
        }
    }
}

fn visit_pou(reader: &mut PeekableReader) {
    let attributes = extract_attributes(reader.next());
    loop {
        match reader.peek() {
            Event::Start(tag) => match tag.name().as_ref() {
                b"body" => visit_body(reader),
                _ => reader.consume(),
            },

            Event::End(tag) if tag.name().as_ref() == b"pou" => {
                reader.consume();
                break;
            }
            _ => reader.consume(),
        }
    }
}

fn visit_body(reader: &mut PeekableReader) {
    reader.consume();

    loop {
        match reader.peek() {
            Event::Start(tag) => match tag.name().as_ref() {
                b"FBD" => visit_fbd(reader),
                _ => reader.consume(),
            },

            Event::End(tag) if tag.name().as_ref() == b"body" => {
                reader.consume();
                break;
            }
            _ => reader.consume(),
        }
    }
}

fn visit_fbd(reader: &mut PeekableReader) {
    reader.consume();

    loop {
        match reader.peek() {
            Event::Start(tag) => match tag.name().as_ref() {
                b"block" => visit_block(reader),
                b"inVariable" | b"outVariable" => visit_fbd_variable(reader),
                _ => reader.consume(),
            },

            Event::End(tag) if tag.name().as_ref() == b"FBD" => {
                reader.consume();
                break;
            }
            _ => reader.consume(),
        }
    }
}

fn visit_fbd_variable(reader: &mut PeekableReader) {
    let mut attributes = extract_attributes(reader.next());
    loop {
        match reader.peek() {
            Event::Text(tag) => {
                attributes.insert("expression".into(), String::from_utf8(tag.as_ref().to_vec()).unwrap());
                reader.consume();
            }

            Event::End(tag) => match tag.name().as_ref() {
                b"inVariable" | b"outVariable" => {
                    reader.consume();
                    break;
                }
                _ => reader.consume(),
            },

            _ => reader.consume(),
        }
    }

    println!("FBD Variables: {attributes:#?}");
}

fn visit_block(reader: &mut PeekableReader) {
    let attributes = extract_attributes(reader.next());

    loop {
        match reader.peek() {
            Event::Start(tag) => match tag.name().as_ref() {
                b"inputVariables" | b"outputVariables" | b"inOutVariables" => vist_block_variable(reader),
                _ => reader.consume(),
            },

            Event::End(tag) if tag.name().as_ref() == b"block" => {
                reader.consume();
                break;
            }
            _ => reader.consume(),
        }
    }

    println!("Block: {attributes:#?}");
}

fn vist_block_variable(reader: &mut PeekableReader) {
    let kind = match reader.next() {
        Event::Start(tag) | Event::Empty(tag) => match tag.name().as_ref() {
            b"inputVariables" => "input",
            b"outputVariables" => "output",
            b"inOutVariables" => "inOut",
            _ => unreachable!(),
        },

        _ => unreachable!(),
    };

    loop {
        match reader.peek() {
            Event::Start(tag) if tag.name().as_ref() == b"variable" => visit_variable(reader),

            Event::End(tag) => match tag.name().as_ref() {
                b"inputVariables" | b"outputVariables" | b"inOut" => {
                    reader.consume();
                    break;
                }
                _ => unreachable!(),
            },

            _ => reader.consume(),
        }
    }
}

fn visit_variable(reader: &mut PeekableReader) {
    let mut attributes = HashMap::new();
    loop {
        match reader.peek() {
            Event::Start(tag) | Event::Empty(tag) => match tag.name().as_ref() {
                b"variable" | b"connection" => attributes.extend(extract_attributes(reader.next())),
                _ => reader.consume(),
            },

            Event::End(tag) if tag.name().as_ref() == b"variable" => {
                reader.consume();
                break;
            }
            _ => reader.consume(),
        }
    }

    println!("Variable: {attributes:#?}");
}

pub(crate) fn extract_attributes(event: Event) -> HashMap<String, String> {
    let tag = match event {
        Event::Start(tag) | Event::Empty(tag) => tag,
        _ => todo!(),
    };

    tag.attributes().flatten().map(|it| (it.key.to_string(), it.value.to_string())).collect()
    // tag.attributes().flat_map(|it| it).map(|it| (it.key.to_string(), it.value.to_string())).collect()
}

trait PrototypingToString {
    fn to_string(self) -> String;
}

impl<'a> PrototypingToString for QName<'a> {
    fn to_string(self) -> String {
        String::from_utf8(self.into_inner().to_vec()).unwrap()
    }
}

impl PrototypingToString for Cow<'_, [u8]> {
    fn to_string(self) -> String {
        String::from_utf8(self.to_vec()).unwrap()
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
