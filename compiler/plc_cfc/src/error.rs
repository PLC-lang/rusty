use std::{fmt::Display, num::ParseIntError, str::Utf8Error};

#[derive(Debug)]
pub enum Error {
    UnexpectedEndOfFile(Vec<&'static [u8]>),

    /// Indicates that the reader expected the new line to be ...
    UnexpectedElement(String),

    /// Indicates that converting a `[u8]` to a String failed due to encoding issues.
    Encoding(Utf8Error),

    /// Indicates that attributes of a XML element are missing. For example if we expect an element
    /// `<foo .../>` to have an attribute `id` such that the element has the structure `<foo id="..." />` but
    /// `id` does not exist.
    MissingAttribute(String),

    /// Indicates that reading the next line of the current XML file failed.
    ReadEvent(quick_xml::Error),

    /// Indicates that converting a string to a given type failed.
    Parse(ParseErrorKind),
}

#[derive(Debug)]
pub enum ParseErrorKind {
    Integer(ParseIntError),
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::Parse(ParseErrorKind::Integer(value))
    }
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnexpectedEndOfFile(tokens) => {
                let tokens = tokens.iter().map(|it| std::str::from_utf8(it).unwrap());
                write!(f, "Expected token {tokens:#?} but reached end of file")
            }
            Error::MissingAttribute(key) => write!(f, "Expected attribute {key} but found none"),
            Error::ReadEvent(why) => write!(f, "Failed to read XML; {why}"),
            Error::UnexpectedElement(element) => write!(f, "{element}"),
            Error::Encoding(why) => write!(f, "{why:#?}"),
            Error::Parse(why) => write!(f, "Failed to parse string to other type; {why}"),
        }
    }
}
