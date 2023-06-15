use std::{borrow::Cow, collections::HashMap};

use quick_xml::{events::Event, name::QName};

use crate::{error::Error, model::project::Project, reader::PeekableReader};

pub(crate) fn visit(content: &str) -> Result<Project, Error> {
    let mut reader = PeekableReader::new(content);
    loop {
        match reader.peek()? {
            Event::Start(tag) if tag.name().as_ref() == b"pou" => return Project::pou_entry(&mut reader),
            Event::Start(tag) if tag.name().as_ref() == b"project" => return Project::visit(&mut reader),
            Event::Eof => return Err(Error::UnexpectedEndOfFile(vec![b"pou"])),
            _ => reader.consume()?,
        }
    }
}

pub(crate) trait Parseable {
    type Item;
    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error>;
}

pub(crate) trait PrototypingToString {
    fn try_to_string(self) -> Result<String, Error>;
}

impl<'a> PrototypingToString for &'a [u8] {
    fn try_to_string(self) -> Result<String, Error> {
        String::from_utf8(self.as_ref().to_vec()).map_err(|err| Error::Encoding(err.utf8_error()))
    }
}

impl<'a> PrototypingToString for QName<'a> {
    fn try_to_string(self) -> Result<String, Error> {
        String::from_utf8(self.into_inner().to_vec()).map_err(|err| Error::Encoding(err.utf8_error()))
    }
}

impl PrototypingToString for Cow<'_, [u8]> {
    fn try_to_string(self) -> Result<String, Error> {
        String::from_utf8(self.to_vec()).map_err(|err| Error::Encoding(err.utf8_error()))
    }
}
pub(crate) trait GetOrErr {
    fn get_or_err(&self, key: &str) -> Result<String, Error>;
}

impl GetOrErr for HashMap<String, String> {
    fn get_or_err(&self, key: &str) -> Result<String, Error> {
        self.get(key).map(|it| it.to_owned()).ok_or(Error::MissingAttribute(key.to_string()))
    }
}
