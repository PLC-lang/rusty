use std::borrow::Cow;

use crate::xml_parser::Parseable2;

use super::body::Body;

#[derive(Debug)]
pub(crate) struct Action<'xml> {
    pub name: Cow<'xml, str>,
    pub type_name: Cow<'xml, str>,
    pub body: Body<'xml>,
}

impl Parseable2 for Action<'_> {
    fn visit2(
        _reader: &mut quick_xml::Reader<&[u8]>,
        _tag: Option<quick_xml::events::BytesStart>,
    ) -> Result<Self, crate::error::Error> {
        todo!()
    }
}
