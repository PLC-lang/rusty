pub(crate) mod interface;
mod pou;

use plc_ast::{
    ast::{CompilationUnit, LinkageType},
    provider::IdProvider,
};

use crate::{index::Index, lowering::polymorphism::table::interface::InterfaceTableGenerator};

use self::pou::VirtualTableGenerator;

/// Generates all polymorphism-related tables (virtual tables for POUs and, in the future,
/// interface tables) by delegating to specialized generators.
pub struct TableGenerator;

impl TableGenerator {
    /// Returns `true` if either the vtable or itable generation produced any
    /// new definitions or instances. When `false`, no compilation unit was
    /// modified and the caller can skip the downstream re-index.
    pub fn generate(
        ids: IdProvider,
        generate_external_constructors: bool,
        index: &Index,
        units: &mut Vec<CompilationUnit>,
    ) -> bool {
        let mut vtable_gen = VirtualTableGenerator::new(ids.clone(), generate_external_constructors);
        let vtable_changed = vtable_gen.generate(index, units);

        let mut itable_gen = InterfaceTableGenerator::new(ids, generate_external_constructors);
        let itable_changed = itable_gen.generate(index, units);

        vtable_changed || itable_changed
    }
}

/// Returns whether a POU with the given linkage should have its table instances
/// defined in this compilation unit. External/Include POUs rely on the library
/// to provide the instances, unless `--generate-external-constructors` is set
/// for External POUs.
pub fn is_internal_instance(linkage: LinkageType, generate_external_constructors: bool) -> bool {
    match linkage {
        LinkageType::External if generate_external_constructors => true,
        LinkageType::External | LinkageType::Include => false,
        _ => true,
    }
}
