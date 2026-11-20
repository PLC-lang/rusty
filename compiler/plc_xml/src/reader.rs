use std::ops::{Deref, DerefMut};

use quick_xml::{
    escape::{resolve_predefined_entity, EscapeError},
    events::{BytesRef, BytesText},
};

use crate::error::Error;

pub struct Reader<'xml>(quick_xml::Reader<&'xml [u8]>);
impl<'xml> Deref for Reader<'xml> {
    type Target = quick_xml::Reader<&'xml [u8]>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Reader<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'xml> Reader<'xml> {
    pub fn new(content: &'xml str) -> Self {
        let mut reader = quick_xml::Reader::from_str(content);
        reader.config_mut().expand_empty_elements = true;
        Reader(reader)
    }
}

/// Collects the character data of an element.
///
/// The XML reader reports character data in pieces: literal text and references such as `&lt;` or `&#65;`
/// arrive as separate events. Push every piece, then call [`TextBuffer::take`] on the first event of any
/// other kind to get the text as it was written.
#[derive(Default)]
pub(crate) struct TextBuffer(String);

impl TextBuffer {
    pub fn push_text(&mut self, text: &BytesText) -> Result<(), Error> {
        self.0.push_str(&text.decode().map_err(quick_xml::Error::from)?);
        Ok(())
    }

    /// Appends the character a reference stands for; fails for references XML does not predefine.
    pub fn push_reference(&mut self, reference: &BytesRef) -> Result<(), Error> {
        if let Some(character) = reference.resolve_char_ref()? {
            self.0.push(character);
            return Ok(());
        }

        let name = reference.decode().map_err(quick_xml::Error::from)?;
        let Some(value) = resolve_predefined_entity(&name) else {
            let unrecognized = EscapeError::UnrecognizedEntity(0..name.len(), name.into_owned());
            return Err(quick_xml::Error::from(unrecognized).into());
        };

        self.0.push_str(value);
        Ok(())
    }

    /// Returns the collected text without surrounding whitespace and empties the buffer, or `None` if the
    /// buffer holds nothing but whitespace.
    pub fn take(&mut self) -> Option<String> {
        let text = std::mem::take(&mut self.0);
        let text = text.trim_matches([' ', '\t', '\r', '\n']);
        (!text.is_empty()).then(|| text.to_string())
    }
}

#[cfg(test)]
pub fn get_start_tag(event: quick_xml::events::Event) -> Option<quick_xml::events::BytesStart> {
    if let quick_xml::events::Event::Start(tag) = event {
        Some(tag)
    } else {
        None
    }
}
