//! AST lowering passes for polymorphic dispatch.
//!
//! Enables runtime polymorphism for classes, function blocks, and interfaces through two
//! sequential phases:
//!
//! 1. [Table generation][`table`]: creates vtable structs (single-inheritance POU hierarchies)
//! and itable structs (multi-inheritance interface dispatch), each containing function-pointer
//! members and accompanied by global instances — one per POU for vtables, one per
//! (POU, interface) pair for itables.
//!
//! 2. [Dispatch lowering][`dispatch`]: rewrites call sites to indirect calls through the
//! generated tables (e.g. `ref^.foo()` → `__vtable_A#(ref^.__vtable^).foo^(ref^)`) and
//! replaces interface-typed variables with fat pointers carrying a `data`/`table` pair.

pub mod dispatch;
pub mod table;

use plc_ast::{ast::CompilationUnit, provider::IdProvider};

use crate::{index::Index, resolver::AnnotationMapImpl};

use self::{dispatch::DispatchLowerer, table::TableGenerator};

/// Entry point for all polymorphism-related lowering passes.
pub struct PolymorphismLowerer {
    pub ids: IdProvider,
}

impl PolymorphismLowerer {
    pub fn new(ids: IdProvider) -> Self {
        Self { ids }
    }

    /// Generates vtable and itable struct definitions, `__vtable` member fields on root POUs,
    /// and global table instances. Must be called before [`dispatch`](Self::dispatch).
    pub fn table(&self, index: &Index, units: &mut Vec<CompilationUnit>) {
        TableGenerator::generate(self.ids.clone(), index, units);
    }

    /// Rewrites call sites and type declarations to route through the generated tables.
    ///
    /// 1. Interface dispatch: replaces interface-typed declarations with `__FATPOINTER`,
    /// expands assignments, and transforms calls through itables.
    /// 2. POU dispatch: transforms method calls into indirect calls through vtables.
    pub fn dispatch(&self, index: Index, annotations: AnnotationMapImpl, units: &mut [CompilationUnit]) {
        DispatchLowerer::lower(self.ids.clone(), index, annotations, units)
    }
}
