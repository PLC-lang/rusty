mod interface;
mod pou;

use plc_ast::{ast::CompilationUnit, provider::IdProvider};

use crate::{index::Index, lowering::polymorphism::table::interface::InterfaceTableGenerator};

use self::pou::VirtualTableGenerator;

/// Generates all polymorphism-related tables (virtual tables for POUs and, in the future,
/// interface tables) by delegating to specialized generators.
pub struct TableGenerator;

impl TableGenerator {
    pub fn generate(ids: IdProvider, index: &Index, units: &mut Vec<CompilationUnit>) {
        let mut vtable_gen = VirtualTableGenerator::new(ids.clone());
        vtable_gen.generate(index, units);

        let mut itable_gen = InterfaceTableGenerator::new(ids);
        itable_gen.generate(index, units);
    }
}
