//! Interface table (itable) generation for multi-inheritance polymorphism.
//!
//! While virtual tables handle single-inheritance dispatch for classes and function blocks,
//! interfaces introduce multi-inheritance which requires a separate mechanism — itables.
//! A POU implementing an interface (directly or via inheritance) gets a per-interface global
//! itable instance whose function pointers point to the POU's actual method implementations.
//!
//! The generation process is split into two steps:
//!
//! # 1. Itable Struct Definitions
//! One struct per interface containing a function pointer for each method (declared + inherited).
//! Function pointer types reference the original interface method POU directly (e.g. `IA.foo`),
//! which already exists in the index and avoids the need for separate forward declaration stubs.
//! For example:
//! ```text
//! TYPE __itable_IA:
//!     STRUCT
//!         foo: __FPOINTER TO IA.foo;
//!     END_STRUCT
//! END_TYPE
//! ```
//!
//! # 2. Global Itable Instances
//! One instance per (interface, POU) pair where the POU implements the interface directly or
//! via its inheritance chain. Each instance carries an initializer that points function pointers
//! to the POU's concrete method implementations:
//! ```text
//! VAR_GLOBAL
//!     __itable_IA_FbA_instance: __itable_IA := (foo := ADR(FbA.foo));
//! END_VAR
//! ```

use plc_ast::{
    ast::{
        AstFactory, AstNode, CompilationUnit, DataType, DataTypeDeclaration, LinkageType,
        UserTypeDeclaration, Variable, VariableBlock,
    },
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

use crate::index::{Index, InterfaceIndexEntry, PouIndexEntry};

/// Generates interface tables (itables) for all interfaces and their implementing POUs.
pub struct InterfaceTableGenerator {
    ids: IdProvider,
}

impl InterfaceTableGenerator {
    pub fn new(ids: IdProvider) -> Self {
        Self { ids }
    }

    /// Generates all itable-related AST artifacts and appends them to the compilation units.
    ///
    /// Itable struct definitions are placed in the compilation unit where the interface is
    /// defined, and global instances are placed in the unit where the implementing POU is
    /// defined. This ensures codegen treats each artifact as local to its respective unit.
    pub fn generate(&mut self, index: &Index, units: &mut [CompilationUnit]) {
        for unit in units.iter_mut() {
            let definitions = self.generate_itable_definitions(index, unit);
            let instances = self.generate_itable_instances(index, unit);

            unit.user_types.extend(definitions);
            if !instances.is_empty() {
                unit.global_vars.push(VariableBlock::global().with_variables(instances));
            }
        }
    }

    /// Creates one itable struct definition per interface defined in the given compilation unit.
    ///
    /// Each struct contains a function pointer member for every method in the interface
    /// (including inherited methods). The function pointer type references the original
    /// interface method POU directly (e.g. `IA.foo`), avoiding the need for separate forward
    /// declaration stubs. No initializers are set on the struct members.
    fn generate_itable_definitions(
        &mut self,
        index: &Index,
        unit: &CompilationUnit,
    ) -> Vec<UserTypeDeclaration> {
        let mut definitions = Vec::new();

        for ast_iface in &unit.interfaces {
            let Some(interface) = index.find_interface(&ast_iface.ident.name) else { continue };
            let location = SourceLocation::internal_in_unit(unit.file.get_name());
            let mut members = Vec::new();

            for method in helper::get_deduplicated_methods(interface, index) {
                let member = Variable {
                    name: method.get_call_name().to_string(),
                    data_type_declaration: DataTypeDeclaration::Definition {
                        data_type: Box::new(helper::create_function_pointer(
                            method.get_name().to_string(),
                            location.clone(),
                        )),
                        location: location.clone(),
                        scope: None,
                    },
                    initializer: None,
                    address: None,
                    location: location.clone(),
                };

                members.push(member);
            }

            let definition = UserTypeDeclaration {
                data_type: DataType::StructType {
                    name: Some(helper::get_itable_name(interface.get_name())),
                    variables: members,
                },
                initializer: None,
                location: location.clone(),
                scope: None,
                linkage: LinkageType::Internal,
            };

            definitions.push(definition);
        }

        definitions
    }

    /// Creates global itable instances for every (interface, POU) pair where the POU is
    /// defined in the given compilation unit.
    ///
    /// To find which interfaces a POU must provide itables for, we walk the POU's inheritance
    /// chain and collect all directly declared `IMPLEMENTS` interfaces plus their transitive
    /// ancestors via `get_derived_interfaces_recursive`.
    fn generate_itable_instances(&mut self, index: &Index, unit: &CompilationUnit) -> Vec<Variable> {
        let mut instances = Vec::new();
        let unit_file = unit.file.get_name();

        // Iterate over POUs defined in this unit that can implement interfaces
        let unit_pous = unit.pous.iter().filter(|p| p.kind.is_class() || p.kind.is_function_block());
        for ast_pou in unit_pous {
            let Some(pou) = index.find_pou(&ast_pou.name) else { continue };

            for iface_name in helper::collect_interfaces_for_pou(index, pou) {
                let Some(interface) = index.find_interface(iface_name) else { continue };
                let instance = self.generate_single_itable_instance(index, interface, pou, unit_file);
                instances.push(instance);
            }
        }

        instances.sort_by(|a, b| a.name.cmp(&b.name));
        instances
    }

    /// Creates a single global variable for an (interface, POU) pair with an initializer
    /// that maps each interface method to the POU's concrete implementation.
    fn generate_single_itable_instance(
        &mut self,
        index: &Index,
        interface: &InterfaceIndexEntry,
        pou: &PouIndexEntry,
        unit_file: Option<&'static str>,
    ) -> Variable {
        let location = SourceLocation::internal_in_unit(unit_file);
        let pou_name = pou.get_name();

        let mut assignments = Vec::new();
        for method in helper::get_deduplicated_methods(interface, index) {
            let method_call_name = method.get_call_name();

            // Find the concrete implementation by walking the POU's inheritance chain.
            // If no concrete method is found the POU has an incomplete implementation which
            // will be caught by validation; we simply skip the assignment here.
            let Some(concrete_method) = index.find_method(pou_name, method_call_name) else {
                continue;
            };

            // Build: <method_name> := ADR(<pou>.<method>)
            let left = AstFactory::create_member_reference(
                AstFactory::create_identifier(
                    method_call_name,
                    SourceLocation::internal(),
                    self.ids.next_id(),
                ),
                None,
                self.ids.next_id(),
            );

            let right = self.generate_adr_call(concrete_method.get_name());
            let assignment = AstFactory::create_assignment(left, right, self.ids.next_id());

            assignments.push(assignment);
        }

        // Build the initializer: (foo := ADR(...), bar := ADR(...))
        let initializer = AstFactory::create_paren_expression(
            AstFactory::create_expression_list(assignments, location.clone(), self.ids.next_id()),
            location.clone(),
            self.ids.next_id(),
        );

        Variable {
            name: helper::get_itable_instance_name(interface.get_name(), pou_name),
            data_type_declaration: DataTypeDeclaration::Reference {
                referenced_type: helper::get_itable_name(interface.get_name()),
                location: location.clone(),
            },
            initializer: Some(initializer),
            address: None,
            location,
        }
    }

    /// Creates a call statement of form `ADR(<qualified_name>)`, e.g. `ADR(FbA.foo)`.
    fn generate_adr_call(&mut self, qualified_name: &str) -> AstNode {
        let operator = AstFactory::create_member_reference(
            AstFactory::create_identifier("ADR", SourceLocation::internal(), self.ids.next_id()),
            None,
            self.ids.next_id(),
        );

        let names: Vec<&str> = qualified_name.split('.').collect();
        debug_assert!(!names.is_empty() && names.len() <= 2, "expected either <pou> or <pou>.<method>");

        let argument = match (names.first(), names.get(1)) {
            (Some(name_pou), None) => AstFactory::create_member_reference(
                AstFactory::create_identifier(*name_pou, SourceLocation::internal(), self.ids.next_id()),
                None,
                self.ids.next_id(),
            ),

            (Some(name_pou), Some(name_method)) => AstFactory::create_member_reference(
                AstFactory::create_identifier(*name_method, SourceLocation::internal(), self.ids.next_id()),
                Some(AstFactory::create_member_reference(
                    AstFactory::create_identifier(*name_pou, SourceLocation::internal(), self.ids.next_id()),
                    None,
                    self.ids.next_id(),
                )),
                self.ids.next_id(),
            ),

            _ => unreachable!(),
        };

        AstFactory::create_call_statement(
            operator,
            Some(argument),
            self.ids.next_id(),
            SourceLocation::internal(),
        )
    }
}

mod helper {
    use plc_ast::ast::{DataType, DataTypeDeclaration};
    use plc_source::source_location::SourceLocation;
    use std::hash::BuildHasherDefault;

    use indexmap::IndexMap;
    use rustc_hash::{FxHashSet, FxHasher};

    use crate::index::{Index, InterfaceIndexEntry, PouIndexEntry};

    /// Returns the methods of an interface deduplicated by call name in DFS order.
    ///
    /// Inherited methods (via `EXTENDS`) are visited depth-first before the interface's own
    /// declared methods, so parent methods appear first in the resulting itable layout.
    /// When an interface re-declares a method already present in a parent, the first
    /// occurrence (the ancestor's) wins to avoid duplicate struct members.
    pub fn get_deduplicated_methods<'idx>(
        interface: &'idx InterfaceIndexEntry,
        index: &'idx Index,
    ) -> Vec<&'idx PouIndexEntry> {
        let mut seen = IndexMap::<_, _, BuildHasherDefault<FxHasher>>::default();
        // DFS: inherited methods first, then own declarations
        for method in interface.get_derived_methods(index) {
            seen.entry(method.get_call_name()).or_insert(method);
        }
        for method in interface.get_declared_methods(index) {
            seen.entry(method.get_call_name()).or_insert(method);
        }
        seen.into_values().collect()
    }

    /// Returns the itable struct name for an interface, e.g. `__itable_IA`.
    pub fn get_itable_name(interface_name: &str) -> String {
        format!("__itable_{interface_name}")
    }

    /// Returns the itable instance name for an (interface, POU) pair, e.g.
    /// `__itable_IA_FbA_instance`.
    pub fn get_itable_instance_name(interface_name: &str, pou_name: &str) -> String {
        format!("__itable_{interface_name}_{pou_name}_instance")
    }

    /// Creates a function pointer data type referencing the given type name.
    pub fn create_function_pointer(referenced_type: String, location: SourceLocation) -> DataType {
        DataType::PointerType {
            name: None,
            referenced_type: Box::new(DataTypeDeclaration::Reference { referenced_type, location }),
            auto_deref: None,
            type_safe: false,
            is_function: true,
        }
    }

    /// Collects all interfaces a POU must implement, accounting for both its own `IMPLEMENTS`
    /// declarations and those inherited from parent POUs, transitively expanding interface
    /// hierarchies.
    pub fn collect_interfaces_for_pou<'idx>(
        index: &'idx Index,
        pou: &'idx PouIndexEntry,
    ) -> FxHashSet<&'idx str> {
        let mut result = FxHashSet::default();
        let mut visited = FxHashSet::default();
        let mut current = Some(pou);

        // Walk the POU inheritance chain (the POU itself, its parent, grandparent, etc.)
        // The visited set guards against inheritance cycles (e.g. A EXTENDS B, B EXTENDS A)
        // which are caught by validation but must not cause infinite loops here.
        while let Some(p) = current {
            if !visited.insert(p.get_name()) {
                break;
            }

            // Collect directly declared IMPLEMENTS interfaces at this level
            for iface_name in p.get_interfaces() {
                if let Some(iface) = index.find_interface(iface_name) {
                    // Expand to include all ancestor interfaces
                    for ancestor in iface.get_derived_interfaces_recursive(index) {
                        result.insert(ancestor.get_name());
                    }
                }
            }

            current = p.get_super_class().and_then(|sc| index.find_pou(sc));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::lowering::polymorphism::table::interface::tests::helper::lower_and_serialize;

    #[test]
    fn pou_without_interfaces_produces_no_itable() {
        // POUs that don't implement any interface should not produce any itable artifacts.
        let result = lower_and_serialize(
            r#"
            FUNCTION_BLOCK FbA
                METHOD foo END_METHOD
            END_FUNCTION_BLOCK
            "#,
        );

        insta::assert_snapshot!(result, @"");
    }

    #[test]
    fn interfaces_generate_itables() {
        // Given some interface, assert that the table(s) are generated
        let result = lower_and_serialize(
            r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE
            "#,
        );

        insta::assert_snapshot!(result, @r"
        // Structs
        __itable_IA {
            foo: __FPOINTER IA.foo;
        }
        ");
    }

    #[test]
    fn pous_implementing_interfaces_generate_instances() {
        // Given some single interface and single POU implementing it, we want to assert a table and instance
        // for them are generated
        // Give some interface and implementer, assert that the table(s) and instance(s) are generated
        let result = lower_and_serialize(
            r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK
            "#,
        );

        insta::assert_snapshot!(result, @r"
        // Structs
        __itable_IA {
            foo: __FPOINTER IA.foo;
        }
        // Globals
        __itable_IA_FbA_instance: __itable_IA := (foo := ADR(FbA.foo))
        ");
    }

    #[test]
    fn extended_interface_includes_inherited_methods() {
        // An interface extending another should produce a struct with both its own and inherited
        // methods (inherited first, per DFS order). The implementing POU gets instances for both
        // the parent and child interface.
        let result = lower_and_serialize(
            r#"
            INTERFACE IA
                METHOD foo END_METHOD
            END_INTERFACE

            INTERFACE IB EXTENDS IA
                METHOD bar END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IB
                METHOD foo END_METHOD
                METHOD bar END_METHOD
            END_FUNCTION_BLOCK
            "#,
        );

        insta::assert_snapshot!(result, @r"
        // Structs
        __itable_IA {
            foo: __FPOINTER IA.foo;
        }
        __itable_IB {
            foo: __FPOINTER IA.foo;
            bar: __FPOINTER IB.bar;
        }
        // Globals
        __itable_IA_FbA_instance: __itable_IA := (foo := ADR(FbA.foo))
        __itable_IB_FbA_instance: __itable_IB := (foo := ADR(FbA.foo), bar := ADR(FbA.bar))
        ");
    }

    #[test]
    fn overridden_method_points_to_child() {
        // When a child POU overrides a method, its itable instance should reference the child's
        // implementation. Non-overridden methods should still point to the ancestor.
        let result = lower_and_serialize(
            r#"
            INTERFACE IA
                METHOD foo END_METHOD
                METHOD bar END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo END_METHOD
                METHOD bar END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
                METHOD foo END_METHOD
            END_FUNCTION_BLOCK
            "#,
        );

        insta::assert_snapshot!(result, @r"
        // Structs
        __itable_IA {
            foo: __FPOINTER IA.foo;
            bar: __FPOINTER IA.bar;
        }
        // Globals
        __itable_IA_FbA_instance: __itable_IA := (foo := ADR(FbA.foo), bar := ADR(FbA.bar))
        __itable_IA_FbB_instance: __itable_IA := (foo := ADR(FbB.foo), bar := ADR(FbA.bar))
        ");
    }

    #[test]
    fn inherited_interface_obligation_through_pou_chain() {
        // A POU that doesn't declare IMPLEMENTS but extends one that does should still get
        // itable instances for all transitively inherited interfaces.
        let result = lower_and_serialize(
            r#"
            INTERFACE IA
                METHOD foo END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK
            "#,
        );

        insta::assert_snapshot!(result, @r"
        // Structs
        __itable_IA {
            foo: __FPOINTER IA.foo;
        }
        // Globals
        __itable_IA_FbA_instance: __itable_IA := (foo := ADR(FbA.foo))
        __itable_IA_FbB_instance: __itable_IA := (foo := ADR(FbA.foo))
        ");
    }

    #[test]
    fn inherited_interface_with_override_points_to_child() {
        // A POU that extends another (which implements an interface) and overrides one of its
        // methods should have the overridden entry point to the child, while non-overridden
        // methods still point to the ancestor's implementation.
        let result = lower_and_serialize(
            r#"
            INTERFACE IA
                METHOD foo END_METHOD
                METHOD bar END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo END_METHOD
                METHOD bar END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
                METHOD bar END_METHOD
            END_FUNCTION_BLOCK
            "#,
        );

        insta::assert_snapshot!(result, @r"
        // Structs
        __itable_IA {
            foo: __FPOINTER IA.foo;
            bar: __FPOINTER IA.bar;
        }
        // Globals
        __itable_IA_FbA_instance: __itable_IA := (foo := ADR(FbA.foo), bar := ADR(FbA.bar))
        __itable_IA_FbB_instance: __itable_IA := (foo := ADR(FbA.foo), bar := ADR(FbB.bar))
        ");
    }

    #[test]
    fn inherited_interface_obligation_through_pou_and_interface_chain() {
        // A POU that extends another which implements a derived interface should get itable
        // instances for the entire transitive interface hierarchy, even though it declares
        // nothing itself. Here FbB extends FbA which implements IB (which extends IA), so
        // FbB should get itables for both IA and IB.
        let result = lower_and_serialize(
            r#"
            INTERFACE IA
                METHOD foo END_METHOD
            END_INTERFACE

            INTERFACE IB EXTENDS IA
                METHOD bar END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IB
                METHOD foo END_METHOD
                METHOD bar END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK
            "#,
        );

        insta::assert_snapshot!(result, @r"
        // Structs
        __itable_IA {
            foo: __FPOINTER IA.foo;
        }
        __itable_IB {
            foo: __FPOINTER IA.foo;
            bar: __FPOINTER IB.bar;
        }
        // Globals
        __itable_IA_FbA_instance: __itable_IA := (foo := ADR(FbA.foo))
        __itable_IA_FbB_instance: __itable_IA := (foo := ADR(FbA.foo))
        __itable_IB_FbA_instance: __itable_IB := (foo := ADR(FbA.foo), bar := ADR(FbA.bar))
        __itable_IB_FbB_instance: __itable_IB := (foo := ADR(FbA.foo), bar := ADR(FbA.bar))
        ");
    }

    #[test]
    fn deep_pou_inheritance_chain() {
        // Three levels of POU inheritance with method overrides at each level. FbC extends FbB
        // extends FbA which implements IA. Each POU should get its own itable instance with
        // method pointers resolved to the most derived override in its chain.
        let result = lower_and_serialize(
            r#"
            INTERFACE IA
                METHOD foo END_METHOD
                METHOD bar END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo END_METHOD
                METHOD bar END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
                METHOD foo END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbC EXTENDS FbB
                METHOD bar END_METHOD
            END_FUNCTION_BLOCK
            "#,
        );

        insta::assert_snapshot!(result, @r"
        // Structs
        __itable_IA {
            foo: __FPOINTER IA.foo;
            bar: __FPOINTER IA.bar;
        }
        // Globals
        __itable_IA_FbA_instance: __itable_IA := (foo := ADR(FbA.foo), bar := ADR(FbA.bar))
        __itable_IA_FbB_instance: __itable_IA := (foo := ADR(FbB.foo), bar := ADR(FbA.bar))
        __itable_IA_FbC_instance: __itable_IA := (foo := ADR(FbB.foo), bar := ADR(FbC.bar))
        ");
    }

    #[test]
    fn deep_pou_chain_with_unique_interfaces_per_level() {
        // Each POU in the inheritance chain implements a different interface, and IB extends IA
        // to add interface-level inheritance. Descendants must accumulate all interface
        // obligations from their ancestors: FbA gets IA, FbB gets IA+IB, FbC gets IA+IB+IC.
        // FbB overrides `foo` (from IA) and FbC overrides `bar` (from IB), so the initializers
        // must resolve through the chain: FbC's IA itable should point foo to FbB (not FbA),
        // and FbC's IB itable should point bar to FbC (not FbB).
        let result = lower_and_serialize(
            r#"
            INTERFACE IA
                METHOD foo END_METHOD
            END_INTERFACE

            INTERFACE IB EXTENDS IA
                METHOD bar END_METHOD
            END_INTERFACE

            INTERFACE IC
                METHOD baz END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA IMPLEMENTS IB
                METHOD foo END_METHOD
                METHOD bar END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbC EXTENDS FbB IMPLEMENTS IC
                METHOD bar END_METHOD
                METHOD baz END_METHOD
            END_FUNCTION_BLOCK
            "#,
        );

        insta::assert_snapshot!(result, @r"
        // Structs
        __itable_IA {
            foo: __FPOINTER IA.foo;
        }
        __itable_IB {
            foo: __FPOINTER IA.foo;
            bar: __FPOINTER IB.bar;
        }
        __itable_IC {
            baz: __FPOINTER IC.baz;
        }
        // Globals
        __itable_IA_FbA_instance: __itable_IA := (foo := ADR(FbA.foo))
        __itable_IA_FbB_instance: __itable_IA := (foo := ADR(FbB.foo))
        __itable_IA_FbC_instance: __itable_IA := (foo := ADR(FbB.foo))
        __itable_IB_FbB_instance: __itable_IB := (foo := ADR(FbB.foo), bar := ADR(FbB.bar))
        __itable_IB_FbC_instance: __itable_IB := (foo := ADR(FbB.foo), bar := ADR(FbC.bar))
        __itable_IC_FbC_instance: __itable_IC := (baz := ADR(FbC.baz))
        ");
    }

    #[test]
    fn multiple_unrelated_interfaces() {
        // A POU implementing two unrelated interfaces should produce separate structs and
        // instances for each.
        let result = lower_and_serialize(
            r#"
            INTERFACE IA
                METHOD foo END_METHOD
            END_INTERFACE

            INTERFACE IB
                METHOD bar END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA, IB
                METHOD foo END_METHOD
                METHOD bar END_METHOD
            END_FUNCTION_BLOCK
            "#,
        );

        insta::assert_snapshot!(result, @r"
        // Structs
        __itable_IA {
            foo: __FPOINTER IA.foo;
        }
        __itable_IB {
            bar: __FPOINTER IB.bar;
        }
        // Globals
        __itable_IA_FbA_instance: __itable_IA := (foo := ADR(FbA.foo))
        __itable_IB_FbA_instance: __itable_IB := (bar := ADR(FbA.bar))
        ");
    }

    #[test]
    fn diamond_interface_hierarchy() {
        // Diamond: ID extends IB and IC, both of which extend IA. Each interface has a dedicated
        // POU implementor. Every POU should get exactly one itable instance per unique interface
        // in its transitive hierarchy — no duplicates for the shared ancestor IA.
        //     IA
        //   /    \
        //  IB    IC
        //   \    /
        //     ID
        let result = lower_and_serialize(
            r#"
            INTERFACE IA
                METHOD foo END_METHOD
            END_INTERFACE

            INTERFACE IB EXTENDS IA
                METHOD bar END_METHOD
            END_INTERFACE

            INTERFACE IC EXTENDS IA
                METHOD baz END_METHOD
            END_INTERFACE

            INTERFACE ID EXTENDS IB, IC
                METHOD qux END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB IMPLEMENTS IB
                METHOD foo END_METHOD
                METHOD bar END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbC IMPLEMENTS IC
                METHOD foo END_METHOD
                METHOD baz END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbD IMPLEMENTS ID
                METHOD foo END_METHOD
                METHOD bar END_METHOD
                METHOD baz END_METHOD
                METHOD qux END_METHOD
            END_FUNCTION_BLOCK
            "#,
        );

        insta::assert_snapshot!(result, @r"
        // Structs
        __itable_IA {
            foo: __FPOINTER IA.foo;
        }
        __itable_IB {
            foo: __FPOINTER IA.foo;
            bar: __FPOINTER IB.bar;
        }
        __itable_IC {
            foo: __FPOINTER IA.foo;
            baz: __FPOINTER IC.baz;
        }
        __itable_ID {
            bar: __FPOINTER IB.bar;
            foo: __FPOINTER IA.foo;
            baz: __FPOINTER IC.baz;
            qux: __FPOINTER ID.qux;
        }
        // Globals
        __itable_IA_FbA_instance: __itable_IA := (foo := ADR(FbA.foo))
        __itable_IA_FbB_instance: __itable_IA := (foo := ADR(FbB.foo))
        __itable_IA_FbC_instance: __itable_IA := (foo := ADR(FbC.foo))
        __itable_IA_FbD_instance: __itable_IA := (foo := ADR(FbD.foo))
        __itable_IB_FbB_instance: __itable_IB := (foo := ADR(FbB.foo), bar := ADR(FbB.bar))
        __itable_IB_FbD_instance: __itable_IB := (foo := ADR(FbD.foo), bar := ADR(FbD.bar))
        __itable_IC_FbC_instance: __itable_IC := (foo := ADR(FbC.foo), baz := ADR(FbC.baz))
        __itable_IC_FbD_instance: __itable_IC := (foo := ADR(FbD.foo), baz := ADR(FbD.baz))
        __itable_ID_FbD_instance: __itable_ID := (bar := ADR(FbD.bar), foo := ADR(FbD.foo), baz := ADR(FbD.baz), qux := ADR(FbD.qux))
        ");
    }

    #[test]
    fn multi_unit_artifacts_land_in_respective_units() {
        use helper::lower_and_serialize_multi;
        use plc_source::SourceCodeFactory;

        // Interface defined in one file, implementing POU in another. The itable struct
        // definition should appear in the interface's unit and the global instance should
        // appear in the POU's unit.
        let result = lower_and_serialize_multi(vec![
            r#"
            INTERFACE IA
                METHOD foo END_METHOD
            END_INTERFACE
            "#
            .create_source("iface.st"),
            r#"
            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo END_METHOD
            END_FUNCTION_BLOCK
            "#
            .create_source("pou.st"),
        ]);

        insta::assert_snapshot!(result, @r"
        // --- unit: iface.st ---
        // Structs
        __itable_IA {
            foo: __FPOINTER IA.foo;
        }
        // --- unit: pou.st ---
        // Globals
        __itable_IA_FbA_instance: __itable_IA := (foo := ADR(FbA.foo))
        ");
    }

    mod helper {
        use std::fmt::Write;

        use driver::parse_and_annotate;
        use plc_ast::{
            ast::{AstStatement, CallStatement, CompilationUnit, DataType, ReferenceAccess, ReferenceExpr},
            provider::IdProvider,
        };
        use plc_source::SourceCode;

        use crate::{index::Index, test_utils::tests::index_unit_with_id, typesystem::DataTypeInformation};

        // TODO: We should probably move this to `lowering/mod.rs` such that every lowering module can make use of
        // it. This is essentially a integration test because it runs all lowering steps, but that's important in
        // the lowering context given they run sequentially and we want to observe that behavior.
        pub fn lower_with_index(source: impl Into<SourceCode>) -> (CompilationUnit, Index) {
            let (_, mut project) = parse_and_annotate("unit-test", vec![source.into()]).unwrap();
            let unit: CompilationUnit = project.units.remove(0).into();
            // Re-index the lowered unit to get a crate-local Index with all generated types
            let index = index_unit_with_id(&unit, IdProvider::default());
            (unit, index)
        }

        /// Lowers multiple source files and returns all compilation units along with a
        /// combined index. Each source should be a distinct [`SourceCode`] with its own path
        /// so that each produces a separate compilation unit.
        pub fn lower_multi_with_index(sources: Vec<SourceCode>) -> (Vec<CompilationUnit>, Index) {
            let (_, project) = parse_and_annotate("unit-test", sources).unwrap();
            let units: Vec<_> = project.units.into_iter().map(CompilationUnit::from).collect();
            // Build a combined index over all units so type lookups work across files
            let mut index = Index::default();
            let builtins = crate::builtins::parse_built_ins(IdProvider::default());
            index.import(crate::index::indexer::index(&builtins));
            for data_type in crate::typesystem::get_builtin_types() {
                index.register_type(data_type);
            }
            for unit in &units {
                index.import(crate::index::indexer::index(unit));
            }
            (units, index)
        }

        /// Formats an itable initializer node: `(foo := ADR(FbA.foo), bar := ADR(FbA.bar))`
        fn format_initializer(node: &plc_ast::ast::AstNode) -> String {
            // ParenExpression → ExpressionList → [Assignment]
            let AstStatement::ParenExpression(inner) = &node.stmt else { return "???".into() };
            let AstStatement::ExpressionList(exprs) = &inner.stmt else { return "???".into() };

            let parts: Vec<String> = exprs
                .iter()
                .map(|expr| {
                    let AstStatement::Assignment(assign) = &expr.stmt else { return "???".into() };
                    let left = assign.left.get_flat_reference_name().unwrap_or("???");

                    // Right side: CallStatement { operator: ADR, parameters: member-chain }
                    let AstStatement::CallStatement(CallStatement { parameters: Some(param), .. }) =
                        &assign.right.stmt
                    else {
                        return format!("{left} := ???");
                    };

                    let qualified = format_member_chain(param);
                    format!("{left} := ADR({qualified})")
                })
                .collect();

            format!("({})", parts.join(", "))
        }

        /// Formats a `ReferenceExpr { Member(Identifier), base }` chain as `"Base.Member"`.
        fn format_member_chain(node: &plc_ast::ast::AstNode) -> String {
            let AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Member(member), base }) =
                &node.stmt
            else {
                return "???".into();
            };

            let name = member.get_flat_reference_name().unwrap_or("???");
            match base {
                Some(base_node) => format!("{}.{name}", format_member_chain(base_node)),
                None => name.to_string(),
            }
        }

        /// Serializes the `__itable_`-prefixed artifacts from a single compilation unit.
        /// Sections (`// Structs`, `// Globals`) are only emitted when they contain entries.
        fn serialize_unit(unit: &CompilationUnit, index: &Index) -> String {
            let mut out = String::new();

            // Collect itable struct definitions
            let mut structs_buf = String::new();
            for ut in &unit.user_types {
                let DataType::StructType { name: Some(name), variables } = &ut.data_type else { continue };
                if !name.starts_with("__itable_") {
                    continue;
                }

                writeln!(&mut structs_buf, "{name} {{").unwrap();
                for var in variables {
                    // The member's type is a reference to a named pointer type (e.g. "____itable_IA_foo").
                    // Look it up in the index to get the inner referenced type.
                    let ref_name = var.data_type_declaration.get_referenced_type().unwrap_or_default();
                    let resolved = index.find_type(&ref_name).and_then(|dt| match &dt.information {
                        DataTypeInformation::Pointer { inner_type_name, is_function: true, .. } => {
                            Some(inner_type_name.as_str())
                        }
                        _ => None,
                    });

                    let target = resolved.unwrap_or("???");
                    writeln!(&mut structs_buf, "    {}: __FPOINTER {target};", var.name).unwrap();
                }
                writeln!(&mut structs_buf, "}}").unwrap();
            }

            if !structs_buf.is_empty() {
                writeln!(&mut out, "// Structs").unwrap();
                out.push_str(&structs_buf);
            }

            // Collect itable global instances
            let mut globals_buf = String::new();
            for block in &unit.global_vars {
                for var in &block.variables {
                    if !var.name.starts_with("__itable_") {
                        continue;
                    }

                    let type_name = var.data_type_declaration.get_referenced_type().unwrap_or_default();
                    let init_str = var.initializer.as_ref().map(format_initializer).unwrap_or_default();

                    writeln!(&mut globals_buf, "{}: {type_name} := {init_str}", var.name).unwrap();
                }
            }

            if !globals_buf.is_empty() {
                writeln!(&mut out, "// Globals").unwrap();
                out.push_str(&globals_buf);
            }

            out
        }

        /// Produces a concise summary of all `__itable_`-prefixed artifacts in a single
        /// compilation unit.
        ///
        /// Uses the index to resolve indirect type references (e.g. `____itable_IA_foo` → `IA.foo`).
        /// Sections are only emitted when they contain entries. Output format:
        /// ```text
        /// // Structs
        /// __itable_IA {
        ///     foo: __FPOINTER IA.foo;
        /// }
        /// // Globals
        /// __itable_IA_FbA_instance: __itable_IA := (foo := ADR(FbA.foo))
        /// ```
        pub fn lower_and_serialize(source: impl Into<SourceCode>) -> String {
            let (unit, index) = lower_with_index(source);
            serialize_unit(&unit, &index)
        }

        /// Produces a per-unit summary of all `__itable_`-prefixed artifacts across multiple
        /// compilation units. Each unit section is prefixed with its file name.
        ///
        /// Output format:
        /// ```text
        /// // --- unit: iface.st ---
        /// // Structs
        /// ...
        /// // Globals
        /// ...
        ///
        /// // --- unit: pou.st ---
        /// // Structs
        /// ...
        /// // Globals
        /// ...
        /// ```
        pub fn lower_and_serialize_multi(sources: Vec<SourceCode>) -> String {
            let (units, index) = lower_multi_with_index(sources);
            let mut out = String::new();

            for unit in &units {
                let file_name = unit.file.get_name().unwrap_or("<unknown>");
                // Skip internal units generated by the pipeline (e.g. __initializers, __init_*)
                if file_name.starts_with("__") {
                    continue;
                }
                writeln!(&mut out, "// --- unit: {file_name} ---").unwrap();
                out.push_str(&serialize_unit(unit, &index));
            }

            out
        }
    }
}
