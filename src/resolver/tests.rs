mod const_resolver_tests;
mod lowering;
mod resolve_control_statments;
mod resolve_expressions_tests;
mod resolve_generic_calls;
mod resolve_literals_tests;
mod resolver_dependency_resolution;

mod helper {
    use plc_ast::{ast::CompilationUnit, provider::IdProvider};

    use crate::{
        index::{FxIndexSet, Index},
        resolver::{AnnotationMapImpl, Dependency, InitializerFunctions, StringLiterals, TypeAnnotator},
    };

    pub(super) fn visit_unit(
        index: &Index,
        unit: &CompilationUnit,
        id_provider: IdProvider,
    ) -> (AnnotationMapImpl, FxIndexSet<Dependency>, StringLiterals) {
        TypeAnnotator::visit_unit(index, unit, id_provider, InitializerFunctions::default())
    }
}
