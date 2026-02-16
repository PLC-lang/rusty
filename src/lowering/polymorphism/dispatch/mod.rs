pub mod pou;

use plc_ast::{ast::CompilationUnit, provider::IdProvider};

use crate::{index::Index, resolver::AnnotationMapImpl};

use self::pou::PolymorphicCallLowerer;

/// Lowers all polymorphism-related calls (POU method dispatch and, in the future, interface
/// dispatch) by delegating to specialized lowerers.
pub struct DispatchLowerer;

impl DispatchLowerer {
    pub fn lower(
        ids: IdProvider,
        index: Index,
        annotations: AnnotationMapImpl,
        units: &mut [CompilationUnit],
    ) -> Index {
        let mut lowerer = PolymorphicCallLowerer::new(ids);
        lowerer.index = Some(index);
        lowerer.annotations = Some(annotations);

        for unit in units.iter_mut() {
            lowerer.lower_unit(unit);
        }

        lowerer.index.take().expect("Index")
    }
}
