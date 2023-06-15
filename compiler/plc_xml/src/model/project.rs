use crate::deserializer::Parseable;

use super::pou::Pou;

// the project root
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

    fn visit(reader: &mut crate::reader::PeekableReader) -> Result<Self::Item, crate::error::Error> {
        unimplemented!()
    }
}

impl Project {
    pub fn pou_entry(reader: &mut crate::reader::PeekableReader) -> Result<Self, crate::error::Error> {
        Ok(Project { pous: vec![Pou::visit(reader)?] })
    }
}
