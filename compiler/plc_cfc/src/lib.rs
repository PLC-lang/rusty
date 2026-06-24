//! CFC (Continuous Function Chart) frontend, loosely based on the
//! IEC 61131-10 XML exchange format. See `CLAUDE.md` for the conventions
//! this crate follows and how it deviates from the strict standard.

mod model;
mod placeholder;
mod resolver;
mod transpiler;

pub use placeholder::resolve_temp_types;

use ast::ast::{CompilationUnit, LinkageType};
use ast::provider::IdProvider;
use plc_diagnostics::diagnostician::Diagnostician;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocationFactory;
use plc_source::{SourceCode, SourceContainer};

use crate::transpiler::Transpiler;

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
    // XXX: The API is a bit fucked here, no? Why do I as a parse_file method have to register the
    // source location? That should be the job of the caller? Maybe fix this later?
    diagnostician.register_file(source.get_location_str().to_string(), source.source.clone());

    let factory = SourceLocationFactory::for_source(source);

    // First deserialize the CFC (XML) content
    let deserialized = model::from_str(&source.source).map_err(|err| {
        Diagnostic::new(format!("Invalid CFC format: {err}"))
            .with_location(factory.create_file_only_location())
    })?;

    // Then transpile it into an AST
    Transpiler::new(deserialized, id_provider, factory).transpile()
}
