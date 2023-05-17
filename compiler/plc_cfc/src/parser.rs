use std::{
    borrow::{Borrow, Cow},
    collections::HashMap,
    str::FromStr,
};

use quick_xml::{
    events::{BytesStart, Event, attributes::Attr},
    name::QName,
    Reader,
};

use crate::model::{
    constant::{
        BLOCK, BODY, FBD, INPUT_VARIABLES, IN_OUT_VARIABLE, IN_OUT_VARIABLES, IN_VARIABLE, OUTPUT_VARIABLES,
        OUT_VARIABLE, POU,
    },
    Attributes, Block, BlockVariable, BlockVariableKind, Body, FbdVariable, FunctionBlockDiagram, Pou,
    PouElement,
};

#[derive(Debug)]
pub enum Error {
    /// ...
    UnexpectedAttribute(String),

    ///
    MissingAttribute(&'static str),

    /// Indicates that we reached EOF inside any parsing method other than [`parse`]
    UnexpectedEndOfFile,

    Read,

    TryFrom,
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnexpectedAttribute(inner) => println!("{inner:#?}"),
            Error::MissingAttribute(inner) => println!("{inner:#?}"),
            Error::UnexpectedEndOfFile => println!("EOF"),
            Error::Read => println!("Read"),
            Error::TryFrom => (),
        }

        Ok(())
    }
}

// struct Tag {
//     result

//     for tag in result
//         match tag
//             block ... Block::parse()
// }


pub(crate) struct Transformer<'rdr> {
    pub reader: Reader<&'rdr [u8]>,
}

pub(crate) enum Tag {
    Start(String, Attributes),
    Empty(String, Attributes),
    Skip,
    End
}

impl<'rdr> Transformer<'rdr> {
    fn new(content: &'rdr str) -> Self {
        Transformer { reader: Reader::from_str(content) }
    }

    pub fn next(&mut self) -> Result<Tag, Error> {
        match self.reader.read_event().unwrap() {
            Event::Start(tag) => Ok(Tag::Start(tag.name().to_string(), extract_attributes(tag))),
            Event::End(tag) => Ok(Tag::End),       
            Event::Empty(tag) => Ok(Tag::Empty(tag.name().to_string(), extract_attributes(tag))),
            Event::Eof => panic!(),
            _ => Ok(Tag::Skip),
        }
    }
}



mod tests {
    #[test]
    fn inputVariables() {
        let content = r#"
            <block localId="1" width="77" height="60" typeName="myAdd" executionOrderId="2">
                <inputVariables>
                    <variable formalParameter="a" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="30" />
                            <connection refLocalId="2" />
                        </connectionPointIn>
                    </variable>
                    <variable formalParameter="b" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="50" />
                            <connection refLocalId="3" />
                        </connectionPointIn>
                    </variable>
                </inputVariables>
                <inOutVariables />
                <outputVariables>
                    <variable formalParameter="x" negated="false">
                        <connectionPointOut>
                            <relPosition x="77" y="30" />
                        </connectionPointOut>
                    </variable>
                    <variable formalParameter="myAdd" negated="false">
                        <connectionPointOut>
                            <relPosition x="77" y="50" />
                        </connectionPointOut>
                    </variable>
                </outputVariables>
            </block>
        "#;
        /*
         1. Deserialize (XML -> Model)
         2. Transform   (Model -> simplified Model) ???
         3. Map         (Simplified Model -> AST)
         */
        
        // let result = Parse::parse(content).unwrap();
        let result: Pou = Deserializer::deserialize(content).unwrap();

        let variables = result.get("inputVariables").unwrap(); -> Vec<_>
        assert_eq!(variables.len(), 2);
        
    }


}

// trait Parseable: Sized {
//     fn parse() -> Self;
// }

// struct Blockkk {
//     name: String,
//     names: Vec<String>,
// }

// impl Parseable for Blockkk {
//     fn parse() -> Self {
//         todo!()
//     }
// }

// HashMap<String, T: ParseAble>
// let inputVariables = map.get("inputVariables");
// for var in inputVariables {
//     var.parse()
// }

// struct Block {
//     ...
// }

// impl Parseable for Block {
//     fn parse() -> Self {

//     }
// }

// TODO: remove (crate) and return Vec<AstStatment>
pub(crate) fn parse(filename: &str) -> Result<Pou, Error> {
    let content = std::fs::read_to_string(filename).unwrap();
    let mut reader = Reader::from_str(&content);
    reader.trim_text(true);
    do_parse(&mut reader)
}

pub(crate) fn do_parse(reader: &mut Reader<&[u8]>) -> Result<Pou, Error> {
    let mut pou = Pou::default();
    loop {
        match reader.read_event().map_err(|_| Error::Read)? {
            Event::Start(tag) => match tag.name().as_ref() {
                POU => return Pou::new(tag, parse_pou(reader)?),
                _ => {}
            },

            Event::Eof => break,
            _ => {}
        }
    }

    Ok(pou)
}

fn parse_pou(reader: &mut Reader<&[u8]>) -> Result<Vec<PouElement>, Error> {
    let mut elements = vec![];
    loop {
        match reader.read_event().map_err(|_| Error::Read)? {
            Event::Start(tag) => match tag.name().into_inner() {
                BODY => elements.push(PouElement::Body(parse_pou_body(reader)?)),
                // INTERFACE => todo!(),
                // ACTIONS => todo!(),
                // TRANSITIONS => todo!(),
                _ => {}
            },

            Event::End(tag) if tag.as_ref() == POU => break,

            _ => {}
        }
    }

    Ok(elements)
}

fn parse_pou_body(reader: &mut Reader<&[u8]>) -> Result<Body, Error> {
    let mut body = Body::default();
    loop {
        match reader.read_event().map_err(|_| Error::Read)? {
            Event::Start(tag) => match tag.name().into_inner() {
                FBD => body.fbd = parse_fbd(reader, tag)?,
                _ => todo!(),
            },

            Event::End(tag) if tag.as_ref() == BODY => break,

            Event::Eof => return Err(Error::UnexpectedEndOfFile),

            _ => {}
        }
    }

    Ok(body)
}

// let result: (reader, tag) = parse_until(fbd);
// parse_fbd(result)
// let result = parse_until(BLOCK, IN_VARIABLE, OUT_VARIABLE, IN_OUT_VARIABLE, LABEL, JUMP)
// if result == BLOCK: parse_$ident
// if result == ... : parse...

fn parse_fbd(reader: &mut Reader<&[u8]>, tag: BytesStart) -> Result<FunctionBlockDiagram, Error> {
    let mut fbd = FunctionBlockDiagram::default();

    loop {
        match reader.read_event().map_err(|_| Error::Read)? {
            Event::Start(tag) => match tag.name().into_inner() {
                BLOCK => fbd.blocks.push(parse_block(reader, tag)?),                
                IN_VARIABLE | OUT_VARIABLE | IN_OUT_VARIABLE => {
                    // TODO:
                    // fbd.variables.push(parse_fbd_variable(reader, tag)?)
                }
                _ => {}
                // b"label" => todo!(),
                // b"jump" => todo!(),
                // _ => todo!(),
            },

            Event::End(tag) if tag.name().as_ref() == FBD => break,

            Event::Eof => break,

            _ => {}
        }
    }

    Ok(fbd)
}

fn parse_fbd_variable(reader: &mut Reader<&[u8]>, tag: BytesStart) -> Result<FbdVariable, Error> {
    todo!()
}

fn parse_block(reader: &mut Reader<&[u8]>, tag: BytesStart) -> Result<Block, Error> {
    let mut block = Block::new(tag)?;

    loop {
        match reader.read_event().map_err(|_| Error::Read)? {
            Event::Start(tag) => match tag.name().into_inner() {
                INPUT_VARIABLES | OUTPUT_VARIABLES | IN_OUT_VARIABLES => {
                    block.variables.append(&mut parse_block_variable(reader, tag)?)
                }
                _ => todo!(),
            },

            Event::End(tag) if tag.name().as_ref() == BLOCK => break,

            Event::Eof => break,

            _ => {}
        }
    }

    Ok(block)
}

fn parse_block_variable(reader: &mut Reader<&[u8]>, tag: BytesStart) -> Result<Vec<BlockVariable>, Error> {
    let mut variables = vec![];
    let kind = BlockVariableKind::from(tag.as_ref());

    loop {
        match reader.read_event().map_err(|_| Error::Read)? {
            Event::Start(tag) => match tag.name().as_ref() {
                // TODO: remove None
                b"variable" => variables.push(BlockVariable::new(tag, None, &kind)?),

                _ => {} // _ => return Err(Error::UnexpectedAttribute(tag.name().to_string())),
            },

            Event::End(tag)
                if matches!(tag.as_ref(), INPUT_VARIABLES | OUTPUT_VARIABLES | IN_OUT_VARIABLES) =>
            {
                break
            }
            _ => {}
        }
    }

    Ok(variables)
}

pub(crate) fn extract_attributes(tag: BytesStart) -> HashMap<String, String> {
    tag.attributes().flat_map(|it| it).map(|it| (it.key.to_string(), it.value.to_string())).collect()
}

pub(crate) trait HashMapExt {
    fn get_or_err(&self, key: &'static str) -> Result<String, Error>;
}

impl HashMapExt for HashMap<String, String> {
    fn get_or_err(&self, key: &'static str) -> Result<String, Error> {
        self.get(key).ok_or(Error::MissingAttribute(key)).cloned()
    }
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

#[test]
fn demo() {
    let mut reader = Reader::from_str(CONTENT);
    println!("{:#?}", crate::parser::do_parse(&mut reader).unwrap());
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
