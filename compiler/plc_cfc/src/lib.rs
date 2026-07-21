//! CFC/FBD XML -> Structured Text AST transpiler.
//!
//! [`parse_file`] is the driver entry point for `.cfc` sources: it deserializes
//! the document, resolves the network into ordered assignments, validates them,
//! and transpiles the result into a [`CompilationUnit`].

pub mod model;
mod resolve;
mod transpile;
mod validation;

#[cfg(test)]
mod test_utils;

use plc_ast::ast::{CompilationUnit, LinkageType};
use plc_ast::provider::IdProvider;
use plc_diagnostics::diagnostician::Diagnostician;
use plc_diagnostics::diagnostics::{Diagnostic, Severity};
use plc_source::{SourceCode, SourceContainer};

use crate::model::Pou;

pub fn parse_file(
    source: &SourceCode,
    _: LinkageType,
    id_provider: IdProvider,
    diagnostician: &mut Diagnostician,
) -> Result<CompilationUnit, Diagnostic> {
    diagnostician.register_file(source.get_location_str().to_string(), source.source.clone());

    let pou = match Pou::parse(&source.source) {
        Ok(pou) => pou,
        Err(err) => {
            diagnostician.handle(&[Diagnostic::new(format!("Unable to parse CFC file: {err}"))]);
            return Err(Diagnostic::new("Compilation aborted due to CFC parse errors"));
        }
    };

    let resolved = resolve::resolve(pou.content().network());

    let mut diagnostics = validation::validate(&resolved, source, id_provider.clone());
    let (unit, transpile_diagnostics) = transpile::transpile(&pou, &resolved, source, id_provider);
    diagnostics.extend(transpile_diagnostics);

    if diagnostician.handle(&diagnostics) == Severity::Error {
        Err(Diagnostic::new("Compilation aborted due to CFC parse errors"))
    } else {
        Ok(unit)
    }
}
