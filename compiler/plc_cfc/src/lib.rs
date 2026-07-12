#![allow(rustdoc::private_intra_doc_links)]
//! Frontend for CFC, the graphical dialect of IEC 61131-3.
//!
//! CFC (Continuous Function Chart) programs are not written as text but drawn as a network of boxes and
//! wires. This crate turns such a network into the same AST the textual Structured Text frontend produces, so
//! the rest of the compiler can treat a graphical POU like any other.
//!
//! ## Inside the crate
//!
//! A CFC file is processed in five steps. [`parse_file`] runs the first four end to end; the fifth is
//! deferred:
//!
//! 1. [Deserialize](crate::model): parse the `.cfc` XML into a typed object graph.
//! 2. [Resolve](crate::resolver): scan that graph once and index how its objects are wired together.
//! 3. [Validate](crate::validator): report any problems as diagnostics, aborting on an error.
//! 4. [Transpile](crate::transpiler): lower the graph into an AST. This is where the real work happens; see
//!    that module's documentation for how each kind of object is translated.
//! 5. [Type temporaries](crate::placeholder): the temporaries the transpiler emits for block results carry
//!    placeholder type names, since their real types are not known during parsing; [`resolve_temp_types`]
//!    fills them in afterwards, at `post_annotate`.
//!
//! ## In the compiler pipeline
//!
//! The driver selects a frontend per source file by type: XML (`.cfc`) sources are routed to [`parse_file`],
//! which yields a `CompilationUnit` indistinguishable from one parsed from Structured Text. Step 5 above is
//! wired in separately as a `post_annotate` participant (the driver's `CfcTempLowerer`), which rewrites the
//! placeholder types and re-indexes/re-annotates once the index and annotations are available.

mod model;
mod placeholder;
mod resolver;
mod transpiler;
mod validator;

pub use placeholder::{has_placeholder_types, resolve_temp_types};

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
    // CFC files are always [`LinkageType::Internal`], hence the linkage is ignored here
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

    let resolver = Resolver::resolve(&deserialized);
    let diagnostics = validator::validate(&deserialized, &factory, &resolver);

    if diagnostician.handle(&diagnostics) == Severity::Error {
        return Err(Diagnostic::new("Compilation aborted due to invalid CFC content")
            .with_sub_diagnostics(diagnostics));
    }

    Transpiler::new(deserialized, id_provider, factory).transpile()
}
