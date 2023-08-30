use std::borrow::Cow;

#[derive(Debug)]
pub(crate) struct Action<'xml> {
    pub name: Cow<'xml, str>,
    pub type_name: Cow<'xml, str>,
}
