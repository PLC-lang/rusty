use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};

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

    /// Advances the reader consuming the event without returning it.
    pub fn consume(&mut self) {
        self.next();
    }

    pub fn next(&mut self) -> Event<'xml> {
        if let Some(event) = self.peek.take() {
            return event;
        }

        self.reader.read_event().unwrap()
    }

    pub fn peek(&mut self) -> &Event<'xml> {
        if self.peek.is_none() {
            self.peek = Some(self.reader.read_event().unwrap());
        }

        match self.peek.as_ref() {
            Some(val) => val,
            None => unreachable!(),
        }
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
    assert_eq!(temp.peek(), &Event::Start(BytesStart::new("body")));
    assert_eq!(temp.next(), Event::Start(BytesStart::new("body")));
}
