use std::{num::ParseIntError, str::Utf8Error};

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

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::Parse(ParseErrorKind::Integer(value))
    }
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEndOfFile(tokens) => {
                write!(
                    f,
                    "Expected token {:?} but reached end of file",
                    tokens
                        .iter()
                        .map(|it| std::str::from_utf8(it).unwrap().to_string())
                        .collect::<Vec<String>>()
                )
            }
            Self::MissingAttribute(key) => write!(f, "Failed to find attribute '{key}'"),
            Self::ReadEvent(why) => write!(f, "Failed to read XML; {why}"),
            Self::UnexpectedElement(element) => write!(f, "Found an unexpected element '{element}'"),
            Self::Encoding(why) => write!(f, "{why:#?}"),
            Self::Parse(why) => write!(f, "Failed to parse string to other type; {why:#?}"),
        }
    }
}
