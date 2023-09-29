use std::borrow::Cow;

use crate::{reader::Reader, xml_parser::Parseable};

use super::body::Body;

#[derive(Debug)]
pub(crate) struct Action<'xml> {
    pub name: Cow<'xml, str>,
    pub type_name: Cow<'xml, str>,
    pub body: Body<'xml>,
}

impl Parseable for Action<'_> {
    fn visit(
        _reader: &mut Reader,
        _tag: Option<quick_xml::events::BytesStart>,
    ) -> Result<Self, crate::error::Error> {
        todo!()
    }
}
