pub mod dispatch;
pub mod table;

use plc_ast::{ast::CompilationUnit, provider::IdProvider};

use crate::{index::Index, resolver::AnnotationMapImpl};

use self::{dispatch::DispatchLowerer, table::TableGenerator};

/// Entry point for all polymorphism-related lowering, including table generation and
/// call dispatch transformations. Delegates to [`TableGenerator`] and [`DispatchLowerer`]
/// which in turn coordinate their respective specialized lowerers for POUs (classes / function
/// blocks) and, in the future, interfaces.
pub struct PolymorphismLowerer {
    pub ids: IdProvider,
}

impl PolymorphismLowerer {
    pub fn new(ids: IdProvider) -> Self {
        Self { ids }
    }

    pub fn table(&self, index: &Index, units: &mut Vec<CompilationUnit>) {
        TableGenerator::generate(self.ids.clone(), index, units);
    }

    pub fn dispatch(
        &self,
        index: Index,
        annotations: AnnotationMapImpl,
        units: &mut [CompilationUnit],
    ) -> Index {
        DispatchLowerer::lower(self.ids.clone(), index, annotations, units)
    }
}
