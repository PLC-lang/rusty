use std::ops::{Deref, DerefMut};

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
        reader.expand_empty_elements(true).trim_text(true);
        Reader(reader)
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
