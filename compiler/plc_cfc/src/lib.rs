//! CFC to AST transpiler
//!
//! A `.cfc` document is deserialized into the raw `model` types, then resolved
//! into a `network::Network`: the `Resolver` surveys the diagram, traces the
//! wiring, orders and validates the statements — every decision and diagnostic
//! in one stage — so the `Transpiler` renders the final `CompilationUnit`
//! without making any decisions of its own.
//!
//! The driver enters twice: [`parse_file`] during the parse step, yielding an
//! interface-only unit so the POU's signature reaches the index, and
//! [`transpile_file`] post-index, lowering the network with that index in hand.

// Public as the shape of the `.cfc` format itself.
pub mod model;

mod network;
mod resolver;
mod st;
mod transpiler;

use plc::index::Index;
use plc_ast::ast::{CompilationUnit, LinkageType};
use plc_ast::provider::IdProvider;
use plc_diagnostics::diagnostician::Diagnostician;
use plc_diagnostics::diagnostics::{Diagnostic, Severity};
use plc_source::{SourceCode, SourceContainer};

use crate::model::Pou;
use crate::resolver::Resolver;
use crate::transpiler::Transpiler;

pub fn parse_file(
    source: &SourceCode,
    _: LinkageType,
    ids: IdProvider,
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

    let (unit, diagnostics) = st::parse_interface(&pou, source, ids);

    // An error aborts; warnings still yield a usable unit.
    if diagnostician.handle(&diagnostics) == Severity::Error {
        Err(Diagnostic::new("Compilation aborted due to CFC parse errors"))
    } else {
        Ok(unit)
    }
}

pub fn transpile_file(
    source: &SourceCode,
    index: &Index,
    ids: IdProvider,
) -> Result<(CompilationUnit, Vec<Diagnostic>), Diagnostic> {
    let pou = Pou::parse(&source.source)
        .map_err(|err| Diagnostic::new(format!("Unable to parse CFC file: {err}")))?;

    // The parse step already reported the interface's diagnostics.
    let (unit, _) = st::parse_interface(&pou, source, ids.clone());

    let (network, diagnostics) = Resolver::new(ids.clone(), source, index).resolve(pou.content().network());
    let unit = Transpiler::new(ids).transpile(unit, network);

    Ok((unit, diagnostics))
}

#[cfg(test)]
mod test_utils {
    use plc_ast::ast::LinkageType;
    use plc_ast::provider::IdProvider;
    use plc_ast::ser::AstSerializer;
    use plc_diagnostics::diagnostician::Diagnostician;
    use plc_diagnostics::diagnostics::Severity;
    use plc_diagnostics::reporter::DiagnosticReporter;
    use plc_source::SourceCode;

    // Indexes every POU a fixture involves: the interface, any companion `.st`
    // beside the `.cfc` declaring the fixture's callees, and the builtin types.
    pub(crate) fn fixture_index(
        fixture: &str,
        interface: &plc_ast::ast::CompilationUnit,
    ) -> plc::index::Index {
        let mut index = plc::index::indexer::index(interface);
        for data_type in plc::typesystem::get_builtin_types() {
            index.register_type(data_type);
        }

        let dir = format!("{}/fixtures/{fixture}", env!("CARGO_MANIFEST_DIR"));
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|extension| extension != "st") {
                continue;
            }

            let source = std::fs::read_to_string(&path).unwrap();
            let factory = plc_source::source_location::SourceLocationFactory::internal(&source);
            let session = plc::lexer::lex_with_ids(&source, IdProvider::default(), factory);
            let (unit, _) = plc::parser::parse(session, LinkageType::Internal, "<callees>");
            index.import(plc::index::indexer::index(&unit));
        }

        index
    }

    pub(crate) fn fixture_source(fixture: &str) -> SourceCode {
        let name = fixture.rsplit('/').next().unwrap();
        let disk = format!("{}/fixtures/{fixture}/{name}.cfc", env!("CARGO_MANIFEST_DIR"));
        // A stable name, so snapshots don't embed absolute paths.
        SourceCode {
            source: std::fs::read_to_string(&disk).unwrap(),
            path: Some(format!("{name}.cfc").into()),
        }
    }

    pub(crate) fn transpile_project(fixture: &str) -> Result<String, String> {
        let source = fixture_source(fixture);
        let ids = IdProvider::default();
        let mut diagnostician = Diagnostician::buffered();
        let abort = |diagnostician: &mut Diagnostician| diagnostician.buffer().unwrap_or_default();

        // Mimic the driver: the parse step's unit seeds the index.
        let interface = crate::parse_file(&source, LinkageType::Internal, ids.clone(), &mut diagnostician)
            .map_err(|_| abort(&mut diagnostician))?;
        let index = fixture_index(fixture, &interface);

        let (unit, diagnostics) =
            crate::transpile_file(&source, &index, ids).map_err(|err| err.get_message().to_string())?;
        match diagnostician.handle(&diagnostics) {
            Severity::Error => Err(abort(&mut diagnostician)),
            _ => Ok(AstSerializer::from_unit(&unit)),
        }
    }

    // Renders every diagnostic a fixture produces, whether or not it aborts.
    pub(crate) fn diagnostics(fixture: &str) -> String {
        let source = fixture_source(fixture);
        let ids = IdProvider::default();
        let mut diagnostician = Diagnostician::buffered();

        let Ok(interface) =
            crate::parse_file(&source, LinkageType::Internal, ids.clone(), &mut diagnostician)
        else {
            return diagnostician.buffer().unwrap_or_default();
        };
        let index = fixture_index(fixture, &interface);

        if let Ok((_, diagnostics)) = crate::transpile_file(&source, &index, ids) {
            diagnostician.handle(&diagnostics);
        }
        diagnostician.buffer().unwrap_or_default()
    }
}
