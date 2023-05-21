use std::{borrow::Cow, collections::HashMap};

use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    name::QName,
    Reader,
};

use crate::model::VariableKind;

#[test]
fn demo() {
    parse()
}

// 0: local_add(a := local_a + 1, b := local_b, c => local_c);
fn parse() {
    let mut reader = Reader::from_str(CONTENT);
    loop {
        /* peekable_reader.peek(), if block => parse_block { let attr = reader.next().attr(); match reader.peek(); _ => reader.next()} */
        match reader.read_event().unwrap() {
            Event::Start(tag) => match tag.name().as_ref() {
                b"block" => parse_block(&mut reader, tag),
                _ => {}
            },

            Event::Eof => break,
            _ => {}
        }
    }
}

fn parse_block(reader: &mut Reader<&[u8]>, tag: BytesStart) {
    let attr = attributes(tag);
    println!("{attr:#?}");

    loop {
        match reader.read_event().unwrap() {
            Event::Start(tag) => match tag.name().as_ref() {
                b"inputVariables" => parse_variable(reader, VariableKind::Input),
                b"outputVariables" => parse_variable(reader, VariableKind::Output),
                _ => {}
            },

            Event::End(tag) if tag.name().as_ref() == b"block" => break,
            _ => {}
        }
    }
}

fn parse_variable(reader: &mut Reader<&[u8]>, kind: VariableKind) {
    fn var(reader: &mut Reader<&[u8]>, kind: VariableKind, tag: BytesStart) {
        let mut attr = attributes(tag);
        loop {
            match reader.read_event().unwrap() {
                Event::Start(tag) | Event::Empty(tag) => match tag.name().as_ref() {
                    b"connection" => attr.extend(attributes(tag)),
                    _ => {}
                },

                Event::End(tag) if tag.name().as_ref() == b"variable" => break,
                _ => {}
            }
        }

        println!("{attr:#?}");
    }

    let end = match kind {
        VariableKind::Input => BytesEnd::new("inputVariables"),
        VariableKind::InOut => BytesEnd::new("inoutVariables"),
        VariableKind::Output => BytesEnd::new("outputVariables"),
    };

    loop {
        match reader.read_event().unwrap() {
            Event::Start(tag) => match tag.name().as_ref() {
                b"variable" => var(reader, kind.clone(), tag),
                _ => {}
            },

            Event::End(tag) if tag == end => break,
            _ => {}
        }
    }
}

fn attributes(tag: BytesStart) -> HashMap<String, String> {
    tag.attributes().flatten().map(|it| (it.key.to_string(), it.value.to_string())).collect()
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
