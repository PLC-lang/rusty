//! CFC (Continuous Function Chart) frontend, loosely based on the
//! IEC 61131-10 XML exchange format. See `CLAUDE.md` for the conventions
//! this crate follows and how it deviates from the strict standard.

mod model;
mod resolver;
mod transpiler;

use ast::ast::{CompilationUnit, LinkageType};
use ast::provider::IdProvider;
use plc_diagnostics::diagnostician::Diagnostician;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::{SourceCode, SourceContainer};

/// Parses a `.cfc` source into a [`CompilationUnit`].
///
/// Signature-compatible with `plc::parser::parse_file` so the driver can
/// dispatch on the source type (`plc_driver::pipelines`):
///
/// ```ignore
/// let parse_func = match source.get_type() {
///     source_code::SourceType::Text => parse_file,
///     source_code::SourceType::Xml => plc_cfc::parse_file,
///     source_code::SourceType::Unknown => unreachable!(),
/// };
/// ```
///
/// (That dispatch currently points at the legacy `plc_xml` crate; swapping
/// it to this function is the only driver change needed.)
pub fn parse_file(
    source: &SourceCode,
    _: LinkageType,
    id_provider: IdProvider,
    diagnostician: &mut Diagnostician,
) -> Result<CompilationUnit, Diagnostic> {
    match transpiler::transpile(source, id_provider) {
        Ok(unit) => Ok(unit),
        Err(err) => {
            diagnostician.register_file(source.get_location_str().to_string(), source.source.clone());
            Err(Diagnostic::new("Compilation aborted due to parse errors").with_sub_diagnostics(err))
        }
    }
}
