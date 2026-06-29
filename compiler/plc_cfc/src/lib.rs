mod model;
mod placeholder;
mod resolver;
mod transpiler;
mod validator;

pub use placeholder::resolve_temp_types;

use ast::ast::{CompilationUnit, LinkageType};
use ast::provider::IdProvider;
use plc_diagnostics::diagnostician::Diagnostician;
use plc_diagnostics::diagnostics::{Diagnostic, Severity};
use plc_source::source_location::SourceLocationFactory;
use plc_source::{SourceCode, SourceContainer};

use crate::resolver::Resolver;
use crate::transpiler::Transpiler;

pub fn parse_file(
    source: &SourceCode,
    _: LinkageType,
    id_provider: IdProvider,
    diagnostician: &mut Diagnostician,
) -> Result<CompilationUnit, Diagnostic> {
    let factory = SourceLocationFactory::for_source(source);

    // TODO: Shouldn't the caller be responsible for registering files for the diagnostician?
    diagnostician.register_file(source.get_location_str().to_string(), source.source.clone());

    let deserialized = model::from_str(&source.source).map_err(|error| {
        let message = format!("Invalid CFC format: {error}");
        Diagnostic::new(message).with_location(factory.create_file_only_location())
    })?;

    let resolver = Resolver::index(&deserialized);
    let diagnostics = validator::validate(&deserialized, &resolver, &factory);

    if diagnostician.handle(&diagnostics) == Severity::Error {
        return Err(Diagnostic::new("Compilation aborted due to invalid CFC content")
            .with_sub_diagnostics(diagnostics));
    }

    Transpiler::new(deserialized, id_provider, factory).transpile()
}
