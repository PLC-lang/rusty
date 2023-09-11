use std::collections::HashMap;

use quick_xml::{events::Event, name::QName, Reader};

use crate::{error::Error, extensions::TryToString};

pub(crate) struct PeekableReader<'xml> {
    reader: Reader<&'xml [u8]>,
    peeked: Option<Event<'xml>>,
}

impl<'xml> PeekableReader<'xml> {
    pub fn new(content: &'xml str) -> Self {
        PeekableReader {
            reader: {
                let mut reader = Reader::from_str(content);
                reader.trim_text(true);
                reader
            },
            peeked: None,
        }
    }

    pub fn next(&mut self) -> Result<Event<'xml>, Error> {
        if let Some(event) = self.peeked.take() {
            return Ok(event);
        }

        self.reader.read_event().map_err(Error::ReadEvent)
    }

    pub fn peek(&mut self) -> Result<&Event<'xml>, Error> {
        if self.peeked.is_none() {
            self.peeked = Some(self.reader.read_event().map_err(Error::ReadEvent)?);
        }

        match self.peeked.as_ref() {
            Some(val) => Ok(val),
            None => unreachable!(),
        }
    }

    // Advances the reader until it sees one of the defined token and stops after consuming it
    pub fn consume_until(&mut self, tokens: Vec<&'static [u8]>) -> Result<(), Error> {
        loop {
            match self.next()? {
                Event::End(tag) if tokens.contains(&tag.name().as_ref()) => break,
                Event::Eof => return Err(Error::UnexpectedEndOfFile(tokens)),
                _ => continue,
            }
        }

        Ok(())
    }

    // Advances the reader until it sees the defined token and stops without consuming it
    pub fn consume_until_start(&mut self, token: &'static [u8]) -> Result<(), Error> {
        loop {
            match self.peek()? {
                Event::Start(tag) if token == tag.name().as_ref() => break,
                Event::Eof => return Err(Error::UnexpectedEndOfFile(vec![token])),
                _ => self.consume()?,
            }
        }

        Ok(())
    }

    /// Advances the reader consuming the event without returning it.
    pub fn consume(&mut self) -> Result<(), Error> {
        self.next()?;
        Ok(())
    }

    /// Advances the reader, consuming the event returning its attributes.
    pub fn attributes(&mut self) -> Result<HashMap<String, String>, Error> {
        let tag = match self.next()? {
            Event::Start(tag) | Event::Empty(tag) => tag,
            _ => todo!(),
        };

        let mut hm = HashMap::new();
        for it in tag.attributes().flatten() {
            hm.insert(it.key.try_to_string()?, it.value.try_to_string()?);
        }

        Ok(hm)
    }

    pub fn read_text(&mut self, name: QName) -> Result<String, Error> {
        Ok(self.reader.read_text(name).map_err(Error::ReadEvent)?.to_string())
    }
}

#[test]
fn peek() {
    const CONTENT: &str = r#"
        <body>
            <FBD>
                <block localId="5" width="82" height="60" typeName="MyAdd" instanceName="local_add" executionOrderId="0">
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
                </block>
                <inVariable localId="6" height="20" width="82" negated="false">
                    <position x="340" y="170" />
                    <connectionPointOut>
                        <relPosition x="82" y="10" />
                    </connectionPointOut>
                    <expression>local_a</expression>
                </inVariable>
            <FBD>
        </body>
    "#;

    let mut temp = PeekableReader::new(CONTENT);
    assert_eq!(temp.peek().unwrap(), &Event::Start(quick_xml::events::BytesStart::new("body")));
    assert_eq!(temp.next().unwrap(), Event::Start(quick_xml::events::BytesStart::new("body")));
}
