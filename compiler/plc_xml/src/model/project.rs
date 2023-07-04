use crate::xml_parser::Parseable;

use super::pou::Pou;

/// The Project root as specified in the official XSD
#[derive(Debug)]
pub(crate) struct Project {
    pub pous: Vec<Pou>,
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

impl Parseable for Project {
    type Item = Self;

    fn visit(_reader: &mut crate::reader::PeekableReader) -> Result<Self::Item, crate::error::Error> {
        unimplemented!()
    }
}

impl Project {
    pub fn pou_entry(reader: &mut crate::reader::PeekableReader) -> Result<Self, crate::error::Error> {
        Ok(Project { pous: vec![Pou::visit(reader)?] })
    }
}
