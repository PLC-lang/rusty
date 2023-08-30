use crate::xml_parser::Parseable;

use super::pou::Pou;

/// The Project root as specified in the official XSD
#[derive(Debug)]
pub(crate) struct Project<'xml> {
    pub pous: Vec<Pou<'xml>>,
    /*
    attributes,
    dataTypes,
    fileHeader,
    contentHeader,
    instances,
    addData,
    documentation
    */
}

impl<'xml> Parseable for Project<'xml> {
    type Item = Self;

    fn visit(_reader: &mut crate::reader::PeekableReader) -> Result<Self::Item, crate::error::Error> {
        unimplemented!()
    }
}

impl<'xml> Project<'xml> {
    pub fn pou_entry(reader: &mut crate::reader::PeekableReader) -> Result<Self, crate::error::Error> {
        Ok(Project { pous: vec![Pou::visit(reader)?] })
    }
}
