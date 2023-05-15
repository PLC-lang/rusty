use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};

const POU: &[u8] = b"pou";
const BODY: &[u8] = b"body";
const INTERFACE: &[u8] = b"interface";
const FBD: &[u8] = b"FBD";
const BLOCK: &[u8] = b"block";

fn parse() {
    // let content = std::fs::read_to_string("res/demo.xml").unwrap();
    let mut reader = Reader::from_str(CONTENT);
    reader.trim_text(true);
    reader.parse();
}

pub trait CfcParser {
    fn parse(&mut self);
    fn parse_pou(&mut self);
    fn parse_fbd(&mut self);
}

impl CfcParser for Reader<&[u8]> {
    fn parse(&mut self) {
        loop {
            match self.read_event().unwrap() {
                Event::Start(tag) => match tag.name().into_inner() {
                    POU => self.parse_pou(),
                    _ => {}
                },

                Event::Eof => {
                    println!("done");
                    break;
                }

                _ => {}
            }
        }
    }

    fn parse_pou(&mut self) {
        loop {
            let Ok(event) = self.read_event() else {
          unreachable!()
        };

            match event {
                Event::Start(tag) => match tag.name().into_inner() {
                    BODY | INTERFACE => continue,
                    FBD => self.parse_fbd(),
                    _ => {
                        dbg!(tag);
                    }
                },
                Event::Eof => {
                    break;
                }

                _ => {}
            }
        }
    }

    fn parse_fbd(&mut self) {
        loop {
            match self.read_event().unwrap() {
                Event::Start(tag) => match tag.name().into_inner() {
                    BLOCK => parse_block_attributes(tag),
                    _ => {}
                },

                Event::End(tag) => match tag.name().into_inner() {
                    BLOCK => break,
                    _ => {}
                },

                Event::Eof => break,

                _ => {}
            }
        }
    }
}

fn parse_block_attributes(tag: BytesStart) {
    println!("{:#?}", tag.attributes().collect::<Vec<_>>());
}

#[test]
fn demo() {
    crate::parser::parse();
}

const CONTENT: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="program_0" pouType="program">
  <interface>
    <localVars/>
    <addData>
      <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
        <textDeclaration>
          <content>PROGRAM program_0 VAR&#13; a: INT := 0;&#13; b: INT := 0; fb : ARRAY[0..100] OF xxx; END_VAR VAR_TEMP addTemp : INT; END_VAR </content>
        </textDeclaration>
      </data>
    </addData>
  </interface>
  <body>
    <FBD>
      <block localId="1" width="77" height="60" typeName="myAdd" executionOrderId="2">
        <position x="140" y="120"/>
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
          <variable formalParameter="x" negated="false">
            <connectionPointOut>
              <relPosition x="77" y="30"/>
            </connectionPointOut>
          </variable>
          <variable formalParameter="myAdd" negated="false">
            <connectionPointOut>
              <relPosition x="77" y="50"/>
            </connectionPointOut>
          </variable>
        </outputVariables>
      </block>
      <inVariable localId="2" height="20" width="80" negated="false">
        <position x="-10" y="140"/>
        <connectionPointOut>
          <relPosition x="80" y="10"/>
        </connectionPointOut>
        <expression>a</expression>
      </inVariable>
      <inVariable localId="3" height="20" width="80" negated="false">
        <position x="-10" y="160"/>
        <connectionPointOut>
          <relPosition x="80" y="10"/>
        </connectionPointOut>
        <expression>1</expression>
      </inVariable>
      <outVariable localId="8" height="20" width="80" executionOrderId="5" negated="false" storage="none">
        <position x="670" y="140"/>
        <connectionPointIn>
          <relPosition x="0" y="10"/>
          <connection refLocalId="9" formalParameter="myAdd"/>
        </connectionPointIn>
        <expression>b</expression>
      </outVariable>
      <inVariable localId="10" height="20" width="80" negated="false">
        <position x="390" y="140"/>
        <connectionPointOut>
          <relPosition x="80" y="10"/>
        </connectionPointOut>
        <expression>b</expression>
      </inVariable>
      <inVariable localId="11" height="20" width="80" negated="false">
        <position x="390" y="160"/>
        <connectionPointOut>
          <relPosition x="80" y="10"/>
        </connectionPointOut>
        <expression>2</expression>
      </inVariable>
      <block localId="9" width="74" height="60" typeName="myAdd" executionOrderId="6">
        <position x="540" y="120"/>
        <inputVariables>
          <variable formalParameter="a" negated="false">
            <connectionPointIn>
              <relPosition x="0" y="30"/>
              <connection refLocalId="10"/>
            </connectionPointIn>
          </variable>
          <variable formalParameter="b" negated="false">
            <connectionPointIn>
              <relPosition x="0" y="50"/>
              <connection refLocalId="11"/>
            </connectionPointIn>
          </variable>
        </inputVariables>
        <inOutVariables/>
        <outputVariables>
          <variable formalParameter="myAdd" negated="false">
            <connectionPointOut>
              <relPosition x="74" y="30"/>
            </connectionPointOut>
          </variable>
        </outputVariables>
      </block>
      <block localId="12" width="60" height="40" typeName="xxx" instanceName="fb[0]" executionOrderId="7">
        <position x="70" y="30"/>
        <inputVariables/>
        <inOutVariables/>
        <outputVariables>
          <variable formalParameter="x" negated="false">
            <connectionPointOut>
              <relPosition x="60" y="30"/>
            </connectionPointOut>
          </variable>
        </outputVariables>
      </block>
      <outVariable localId="13" height="20" width="80" executionOrderId="8" negated="false" storage="none">
        <position x="290" y="100"/>
        <connectionPointIn>
          <relPosition x="0" y="10"/>
          <connection refLocalId="14"/>
        </connectionPointIn>
        <expression>a</expression>
      </outVariable>
      <inVariable localId="14" height="20" width="80" negated="false">
        <position x="160" y="80"/>
        <connectionPointOut>
          <relPosition x="80" y="10"/>
        </connectionPointOut>
        <expression>a</expression>
      </inVariable>
      <block localId="15" width="74" height="60" typeName="myAdd" executionOrderId="10">
        <position x="270" y="150"/>
        <inputVariables>
          <variable formalParameter="a" negated="false">
            <connectionPointIn>
              <relPosition x="0" y="30"/>
              <connection refLocalId="1" formalParameter="x"/>
            </connectionPointIn>
          </variable>
          <variable formalParameter="b" negated="false">
            <connectionPointIn>
              <relPosition x="0" y="50"/>
            </connectionPointIn>
          </variable>
        </inputVariables>
        <inOutVariables/>
        <outputVariables>
          <variable formalParameter="x" negated="false">
            <connectionPointOut>
              <relPosition x="74" y="30"/>
            </connectionPointOut>
          </variable>
          <variable formalParameter="myAdd" negated="false">
            <connectionPointOut>
              <relPosition x="74" y="50"/>
            </connectionPointOut>
          </variable>
        </outputVariables>
      </block>
    </FBD>
  </body>
</pou>
"#;
