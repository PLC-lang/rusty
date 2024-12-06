use plc_diagnostics::diagnostics::Diagnostic;

use crate::{reader::Reader, xml_parser::Parseable};

use super::pou::Pou;

/// The Project root as specified in the official XSD
#[derive(Debug, Default)]
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

impl Parseable for Project<'_> {
    fn visit(
        _reader: &mut Reader,
        _tag: Option<quick_xml::events::BytesStart>,
    ) -> Result<Self, crate::error::Error> {
        unimplemented!()
    }
}

impl Project<'_> {
    pub(crate) fn desugar(
        &mut self,
        source_location_factory: &plc_source::source_location::SourceLocationFactory,
    ) -> Result<(), Vec<Diagnostic>> {
        let mut diagnostics = vec![];
        self.pous.iter_mut().for_each(|pou| {
            let _ = pou.desugar(source_location_factory).map_err(|e| diagnostics.extend(e));
        });

        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }
}
