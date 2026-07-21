//! Shared helpers for the transpiler/resolver/validation tests.

use plc_ast::ast::LinkageType;
use plc_ast::provider::IdProvider;
use plc_ast::ser::AstSerializer;
use plc_diagnostics::diagnostician::Diagnostician;
use plc_diagnostics::reporter::DiagnosticReporter;
use plc_source::SourceCode;

/// Load the `.cfc` of a fixture addressed relative to `fixtures/`, e.g.
/// `"valid/simple_assignment"` or `"invalid/call_expression"`.
pub(crate) fn fixture_source(fixture: &str) -> SourceCode {
    let name = fixture.rsplit('/').next().unwrap();
    let disk = format!("{}/fixtures/{fixture}/{name}.cfc", env!("CARGO_MANIFEST_DIR"));
    // Label the source with a stable, machine-independent name so diagnostic
    // snapshots don't embed an absolute path.
    SourceCode { source: std::fs::read_to_string(&disk).unwrap(), path: Some(format!("{name}.cfc").into()) }
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
