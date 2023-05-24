use quick_xml::{events::Event, Reader};

use crate::model::Error;

pub struct PeekableReader<'xml> {
    reader: Reader<&'xml [u8]>,
    peek: Option<Event<'xml>>,
}

impl<'xml> PeekableReader<'xml> {
    pub fn new(content: &'xml str) -> Self {
        PeekableReader {
            reader: {
                let mut reader = Reader::from_str(content);
                reader.trim_text(true);
                reader
            },
            peek: None,
        }
    }

    pub fn next(&mut self) -> Result<Event<'xml>, Error> {
        if let Some(event) = self.peek.take() {
            return Ok(event);
        }

        self.reader.read_event().map_err(Error::ReadEvent)
    }

    pub fn peek(&mut self) -> Result<&Event<'xml>, Error> {
        if self.peek.is_none() {
            self.peek = Some(self.reader.read_event().map_err(Error::ReadEvent)?);
        }

        match self.peek.as_ref() {
            Some(val) => Ok(val),
            None => unreachable!(),
        }
    }

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

    /// Advances the reader consuming the event without returning it.
    pub fn consume(&mut self) -> Result<(), Error> {
        self.next()?;
        Ok(())
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
