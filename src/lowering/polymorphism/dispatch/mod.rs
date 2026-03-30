//! Rewrites call sites and declarations to route through generated vtable and itable structures.
//!
//! This is the second phase of the polymorphism lowering pipeline (the first being
//! [table generation][`super::table`]). It runs two sub-passes in order:
//!
//! 1. [`interface`] – rewrites interface-typed variables to fat pointers, expands assignments,
//!    and lowers method calls through itables.
//! 2. [`pou`] – rewrites method calls on classes/function blocks into indirect calls through
//!    vtables.
//!
//! Interface dispatch must run first; see the sub-module docs for details.

pub mod interface;
pub mod pou;
pub mod validation;

use plc_ast::{ast::CompilationUnit, provider::IdProvider};
use plc_diagnostics::diagnostics::Diagnostic;

use crate::{index::Index, resolver::AnnotationMapImpl};

use self::{interface::InterfaceDispatchLowerer, pou::PolymorphicCallLowerer};

/// Entry point for dispatch lowering, called by [`super::PolymorphismLowerer::dispatch`]
/// during the `post_annotate` pipeline stage.
pub struct DispatchLowerer;

impl DispatchLowerer {
    /// Lowers direct calls to indirect calls for polymorphic variables.
    /// Returns any diagnostics produced during interface validation.
    pub fn lower(
        ids: IdProvider,
        index: Index,
        annotations: AnnotationMapImpl,
        units: &mut [CompilationUnit],
    ) -> Vec<Diagnostic> {
        let mut lowerer = InterfaceDispatchLowerer::new(ids.clone(), &index, &annotations);
        let diagnostics = lowerer.lower(units);

        let mut lowerer = PolymorphicCallLowerer::new(ids, &index, &annotations);
        lowerer.lower(units);

        diagnostics
    }
}
