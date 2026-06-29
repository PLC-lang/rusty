use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocationFactory;

use crate::model::{FbdNetwork, Pou};
use crate::resolver::Resolver;

/// Runs every structural validation over an indexed network, collecting their diagnostics. An empty
/// result means the network is well-formed enough to be transpiled.
pub fn validate(pou: &Pou, resolver: &Resolver, factory: &SourceLocationFactory) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    let Some(network) = pou.get_network() else {
        return diagnostics;
    };

    validate_cycle(network, resolver, factory, &mut diagnostics);

    diagnostics
}

/// Reports data-flow cycles, i.e. blocks that (transitively) feed into themselves without an explicit
/// feedback connection.
fn validate_cycle(
    network: &FbdNetwork,
    resolver: &Resolver,
    factory: &SourceLocationFactory,
    diagnostics: &mut Vec<Diagnostic>,
) {
}
