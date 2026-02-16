pub mod interface;
pub mod pou;

use plc_ast::{ast::CompilationUnit, provider::IdProvider};

use crate::{index::Index, resolver::AnnotationMapImpl};

use self::{interface::InterfaceDispatchLowerer, pou::PolymorphicCallLowerer};

/// Lowers all polymorphism-related calls (POU method dispatch and interface dispatch) by
/// delegating to specialized lowerers.
pub struct DispatchLowerer;

impl DispatchLowerer {
    pub fn lower(
        ids: IdProvider,
        index: Index,
        annotations: AnnotationMapImpl,
        units: &mut [CompilationUnit],
    ) {
        // 1. Lower interface type declarations to __FATPOINTER
        let mut lowerer = InterfaceDispatchLowerer::new(ids.clone(), &index, &annotations);
        lowerer.lower(units);

        // 2. Lower POU polymorphic calls (vtable dispatch)
        let mut lowerer = PolymorphicCallLowerer::new(ids, &index, &annotations);

        for unit in units.iter_mut() {
            lowerer.lower_unit(unit);
        }
    }
}
