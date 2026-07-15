//! CFC/FBD XML -> Structured Text AST transpiler.
//!
//! [`parse_file`] is the driver entry point for `.cfc` sources: it deserializes
//! the document, resolves the network into ordered assignments, validates them,
//! and transpiles the result into a [`CompilationUnit`].

pub mod model;
mod resolve;
mod transpile;
mod validation;

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
    // Register the source so diagnostics can render snippets from it.
    diagnostician.register_file(source.get_location_str().to_string(), source.source.clone());

    // Deserialize the document; a malformed file aborts before any analysis.
    let pou = match Pou::parse(&source.source) {
        Ok(pou) => pou,
        Err(err) => {
            diagnostician.handle(&[Diagnostic::new(format!("Unable to parse CFC file: {err}"))]);
            return Err(Diagnostic::new("Compilation aborted due to CFC parse errors"));
        }
    };

    // Resolve the network, then validate and lower it, gathering every diagnostic.
    let resolved = resolve::resolve(pou.content().network());
    let mut diagnostics = validation::validate(&resolved, source, id_provider.clone());
    let (unit, transpile_diagnostics) = transpile::transpile(&pou, &resolved, source, id_provider);
    diagnostics.extend(transpile_diagnostics);

    // An error aborts; warnings still yield a usable unit.
    if diagnostician.handle(&diagnostics) == Severity::Error {
        Err(Diagnostic::new("Compilation aborted due to CFC parse errors"))
    } else {
        Ok(unit)
    }
}

#[cfg(test)]
mod test_utils {
    //! Shared helpers for the transpiler/resolver/validation tests.

    use plc_ast::ast::LinkageType;
    use plc_ast::provider::IdProvider;
    use plc_ast::ser::AstSerializer;
    use plc_diagnostics::diagnostician::Diagnostician;
    use plc_diagnostics::reporter::DiagnosticReporter;
    use plc_source::SourceCode;

    /// Load the `.cfc` of a fixture addressed relative to `fixtures/`, e.g.
    /// `"variables/valid/simple_assignment"`.
    pub(crate) fn fixture_source(fixture: &str) -> SourceCode {
        let name = fixture.rsplit('/').next().unwrap();
        let disk = format!("{}/fixtures/{fixture}/{name}.cfc", env!("CARGO_MANIFEST_DIR"));
        // Label the source with a stable, machine-independent name so diagnostic
        // snapshots don't embed an absolute path.
        SourceCode {
            source: std::fs::read_to_string(&disk).unwrap(),
            path: Some(format!("{name}.cfc").into()),
        }
    }

    /// Run the CFC pipeline over a fixture. `Ok` is the transpiled POU serialized
    /// back to Structured Text; `Err` is the diagnostics rendered as the console
    /// shows them (returned when compilation aborts).
    pub(crate) fn transpile_project(fixture: &str) -> Result<String, String> {
        let source = fixture_source(fixture);
        let mut diagnostician = Diagnostician::buffered();
        match crate::parse_file(&source, LinkageType::Internal, IdProvider::default(), &mut diagnostician) {
            Ok(unit) => Ok(AstSerializer::from_unit(&unit)),
            Err(_) => Err(diagnostician.buffer().unwrap_or_default()),
        }
    }

    /// Render every diagnostic a fixture produces, whether or not it aborts
    /// compilation. Used to snapshot warnings on otherwise-valid fixtures.
    pub(crate) fn diagnostics(fixture: &str) -> String {
        let source = fixture_source(fixture);
        let mut diagnostician = Diagnostician::buffered();
        let _ = crate::parse_file(&source, LinkageType::Internal, IdProvider::default(), &mut diagnostician);
        diagnostician.buffer().unwrap_or_default()
    }
}
