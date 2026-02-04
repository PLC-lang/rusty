//! Interface table (itable) generation for interface-based polymorphism.
//!
//! This module is responsible for creating interface tables that enable dynamic dispatch for
//! interface-typed variables. Unlike virtual tables (see [`crate::lowering::vtable`]) which work
//! with single-inheritance hierarchies via prefix-compatible struct layouts, interface tables solve
//! the problem of dispatching across unrelated type hierarchies that happen to implement the same
//! interface.
//!
//! The process of creating these interface tables can be broken down into three tasks:
//!
//! # 1. Interface Table Struct Definitions
//! One itable struct is generated per interface, with function pointer fields for each method
//! defined (or inherited) by the interface. For example, given interface `B` with methods `foo`
//! and `bar`:
//! ```text
//! TYPE __itable_B:
//!     STRUCT
//!         foo: __FPOINTER TO fwd_B_foo;
//!         bar: __FPOINTER TO fwd_B_bar;
//!     END_STRUCT
//! END_TYPE
//! ```
//!
//! # 2. Forward Declarations
//! Temporary forward-declaration functions are generated for each interface method, providing the
//! type reference needed by the function pointer syntax. These are a workaround for the current
//! function pointer syntax and may be removed in the future.
//!
//! # 3. Global Itable Instances
//! For each (interface, implementing POU) pair — including transitive implementations — a global
//! itable instance is generated with function pointers initialized to the POU's method addresses.
//!
//! Note: The `__FATPOINTER` struct used to represent interface-typed variables is generated
//! on-demand by [`crate::lowering::itable_calls::InterfaceCallLowerer`] when interface-typed
//! variables are encountered.

use plc_ast::{
    ast::{
        ArgumentProperty, AstFactory, AstNode, CompilationUnit, DataType, DataTypeDeclaration,
        Implementation, LinkageType, Pou, PouType, UserTypeDeclaration, Variable, VariableBlock,
        VariableBlockType,
    },
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;
use rustc_hash::FxHashSet;

use crate::index::{Index, InterfaceIndexEntry, VariableType};

pub struct InterfaceTableGenerator {
    pub ids: IdProvider,
}

impl InterfaceTableGenerator {
    pub fn new(ids: IdProvider) -> InterfaceTableGenerator {
        InterfaceTableGenerator { ids }
    }

    pub fn generate(&mut self, index: &Index, units: &mut Vec<CompilationUnit>) {
        for unit in units.iter_mut() {
            // 2. Generate itable struct definitions per interface, in each unit that declares them
            // 3. Generate forward declarations for interface methods
            let mut definitions = Vec::new();
            let mut forward_pous = Vec::new();
            let mut forward_impls = Vec::new();
            for interface in &unit.interfaces {
                let iface = index.find_interface(&interface.ident.name).unwrap();
                definitions.push(self.generate_itable_definition(index, iface));
                self.generate_forward_declarations(index, iface, &mut forward_pous, &mut forward_impls);
            }
            unit.user_types.extend(definitions);
            unit.pous.extend(forward_pous);
            unit.implementations.extend(forward_impls);

            // 4. Generate global itable instances per (interface, POU) pair
            let mut instances = Vec::new();
            for pou in unit.pous.iter().filter(|p| p.kind.is_class() || p.kind.is_function_block()) {
                instances.extend(self.generate_itable_instances(index, &pou.name, &pou.location));
            }
            unit.global_vars.push(VariableBlock::global().with_variables(instances));
        }
    }

    /// Generates an itable struct definition for a single interface.
    ///
    /// The struct contains one function pointer field per method (inherited + own),
    /// each pointing to a forward declaration function `__fwd_{interface}_{method}`.
    fn generate_itable_definition(
        &self,
        index: &Index,
        interface: &InterfaceIndexEntry,
    ) -> UserTypeDeclaration {
        let location = SourceLocation::internal_in_unit(interface.ident.location.get_file_name());
        let interface_name = interface.get_name();
        let methods = helper::dedupe_methods_by_call_name(interface.get_methods(index));

        // TODO: Depth-First order for variables, probably needs a `interface.get_methods_depth_first`? Address this later
        let variables = methods
            .iter()
            .map(|method| {
                let call_name = method.get_call_name();
                let fwd_name = helper::get_forward_declaration_name(interface_name, call_name);
                Variable {
                    name: call_name.to_string(),
                    data_type_declaration: DataTypeDeclaration::Definition {
                        data_type: Box::new(helper::create_function_pointer(fwd_name, location.clone())),
                        location: location.clone(),
                        scope: None,
                    },
                    initializer: None,
                    address: None,
                    location: location.clone(),
                }
            })
            .collect();

        UserTypeDeclaration {
            data_type: DataType::StructType {
                name: Some(helper::get_itable_name(interface_name)),
                variables,
            },
            initializer: None,
            location,
            scope: None,
        }
    }

    /// Generates forward declaration POUs for each method of an interface.
    ///
    /// These are temporary stub functions (e.g. `__fwd_B_foo`) that exist solely to provide
    /// a concrete function name for the `__FPOINTER TO __fwd_B_foo` syntax used in itable
    /// struct fields. Each forward declaration mirrors the original method's signature
    /// (parameters, return type) and prepends a `self: POINTER TO __VOID` parameter.
    ///
    /// TODO: Remove once function pointers support C-style signatures.
    fn generate_forward_declarations(
        &mut self,
        index: &Index,
        interface: &InterfaceIndexEntry,
        pous: &mut Vec<Pou>,
        implementations: &mut Vec<Implementation>,
    ) {
        let location = SourceLocation::internal_in_unit(interface.ident.location.get_file_name());
        let interface_name = interface.get_name();
        let methods = helper::dedupe_methods_by_call_name(interface.get_methods(index));

        for method in &methods {
            let call_name = method.get_call_name();
            let fwd_name = helper::get_forward_declaration_name(interface_name, call_name);

            // Build variable blocks: start with `self: POINTER TO __VOID`, then add the
            // method's own parameters by looking them up in the index.
            let self_block = VariableBlock::default()
                .with_block_type(VariableBlockType::Input(ArgumentProperty::ByVal))
                .with_variables(vec![helper::create_void_pointer_variable("self", &location)]);

            let mut variable_blocks = vec![self_block];
            let members = index.get_pou_members(method.get_name());
            for member in members {
                let variable_type = member.get_variable_type();
                let block_type = match variable_type {
                    VariableType::Input => VariableBlockType::Input(ArgumentProperty::ByVal),
                    VariableType::Output => VariableBlockType::Output,
                    VariableType::InOut => VariableBlockType::InOut,
                    _ => continue,
                };

                // TODO: The index wraps OUTPUT/INOUT types with `__auto_pointer_to_` for
                // by-reference semantics; strip it to get the declared type name. This
                // might need a more robust approach later.
                let type_name = member.get_type_name();
                let type_name = type_name.strip_prefix("__auto_pointer_to_").unwrap_or(type_name).to_string();

                variable_blocks.push(VariableBlock::default().with_block_type(block_type).with_variables(
                    vec![Variable {
                        name: member.get_name().to_string(),
                        data_type_declaration: DataTypeDeclaration::Reference {
                            referenced_type: type_name,
                            location: location.clone(),
                        },
                        initializer: None,
                        address: None,
                        location: location.clone(),
                    }],
                ));
            }

            let return_type =
                index.find_return_type(method.get_name()).map(|_| DataTypeDeclaration::Reference {
                    referenced_type: index
                        .find_return_variable(method.get_name())
                        .map(|v| v.get_type_name().to_string())
                        .unwrap_or_default(),
                    location: location.clone(),
                });

            pous.push(Pou {
                name: fwd_name.clone(),
                id: self.ids.next_id(),
                kind: PouType::Function,
                variable_blocks,
                return_type,
                location: location.clone(),
                name_location: location.clone(),
                poly_mode: None,
                generics: vec![],
                linkage: LinkageType::Internal,
                super_class: None,
                interfaces: vec![],
                properties: vec![],
                is_const: false,
            });

            implementations.push(Implementation {
                name: fwd_name.clone(),
                type_name: fwd_name,
                linkage: LinkageType::Internal,
                pou_type: PouType::Function,
                statements: vec![],
                location: location.clone(),
                name_location: location.clone(),
                end_location: location.clone(),
                overriding: false,
                generic: false,
                access: None,
            });
        }
    }

    /// Generates global itable instances for all interfaces that the given POU implements,
    /// including transitive implementations through the superclass chain and interface
    /// inheritance hierarchy.
    ///
    /// For each (interface, POU) pair, a global variable is created with the itable struct
    /// type and initialized with `ADR(...)` calls pointing to the POU's concrete method
    /// implementations (resolving overrides vs. inherited methods).
    fn generate_itable_instances(
        &mut self,
        index: &Index,
        pou_name: &str,
        pou_location: &SourceLocation,
    ) -> Vec<Variable> {
        let location = SourceLocation::internal_in_unit(pou_location.get_file_name());
        let all_interfaces = self.collect_all_interfaces(index, pou_name);

        all_interfaces
            .iter()
            .map(|interface| {
                let methods = interface.get_methods(index);
                let initializer = self.generate_itable_initializer(index, &methods, pou_name);

                Variable {
                    name: helper::get_itable_instance_name(interface.get_name(), pou_name),
                    data_type_declaration: DataTypeDeclaration::Reference {
                        referenced_type: helper::get_itable_name(interface.get_name()),
                        location: location.clone(),
                    },
                    initializer,
                    address: None,
                    location: location.clone(),
                }
            })
            .collect()
    }

    /// Collects all interfaces that a POU implements — directly, via superclasses, and via
    /// interface inheritance. For example, if `FbStream EXTENDS FbReader IMPLEMENTS IWriter`,
    /// and `FbReader IMPLEMENTS IReader`, and `IWriter EXTENDS IBase`, then FbStream
    /// implements `{IWriter, IBase, IReader}`.
    fn collect_all_interfaces<'a>(&self, index: &'a Index, pou_name: &str) -> Vec<&'a InterfaceIndexEntry> {
        let mut seen_interfaces = FxHashSet::default();
        let mut seen_pous = FxHashSet::default();
        let mut result = Vec::new();

        // Walk up the POU inheritance chain (FbStream -> FbReader -> ...)
        let mut current_pou = index.find_pou(pou_name);
        while let Some(pou) = current_pou {
            // Detect inheritance cycles to avoid infinite loops
            if !seen_pous.insert(pou.get_name().to_string()) {
                break;
            }

            // Collect directly declared interfaces on this POU
            for iface_name in pou.get_interfaces() {
                if let Some(iface) = index.find_interface(iface_name) {
                    // Add the interface itself
                    if seen_interfaces.insert(iface.get_name().to_string()) {
                        result.push(iface);
                    }

                    // Add all transitively inherited interfaces (e.g. IReadWriter -> IReader, IWriter)
                    for parent_iface in iface.get_derived_interfaces_recursive(index) {
                        if seen_interfaces.insert(parent_iface.get_name().to_string()) {
                            result.push(parent_iface);
                        }
                    }
                }
            }

            current_pou = pou.get_super_class().and_then(|name| index.find_pou(name));
        }

        result
    }

    /// Creates an initializer expression for an itable instance variable.
    ///
    /// Produces an expression list of the form `(foo := ADR(Pou.foo), bar := ADR(Pou.bar))`,
    /// where each method is resolved to the concrete implementation the POU provides (which
    /// may be inherited from a superclass).
    fn generate_itable_initializer(
        &mut self,
        index: &Index,
        methods: &[&crate::index::PouIndexEntry],
        pou_name: &str,
    ) -> Option<AstNode> {
        let assignments: Vec<AstNode> = methods
            .iter()
            .filter_map(|method| {
                let call_name = method.get_call_name();
                let resolved = index.find_method(pou_name, call_name)?;

                // field_name := ADR(ConcretePou.method)
                let left = AstFactory::create_member_reference(
                    AstFactory::create_identifier(call_name, SourceLocation::internal(), self.ids.next_id()),
                    None,
                    self.ids.next_id(),
                );
                let right = self.generate_initializer(resolved.get_name());
                Some(AstFactory::create_assignment(left, right, self.ids.next_id()))
            })
            .collect();

        if assignments.is_empty() {
            None
        } else {
            Some(AstFactory::create_expression_list(
                assignments,
                SourceLocation::internal(),
                self.ids.next_id(),
            ))
        }
    }

    /// Creates a call statement of the form `ADR(<qualified name>)`, e.g. `ADR(FbStream.read)`.
    fn generate_initializer(&mut self, qualified_name: &str) -> AstNode {
        let operator = AstFactory::create_member_reference(
            AstFactory::create_identifier("ADR", SourceLocation::internal(), self.ids.next_id()),
            None,
            self.ids.next_id(),
        );

        let names: Vec<_> = qualified_name.split('.').collect();
        debug_assert!(!names.is_empty() && names.len() <= 2, "expected either <pou> or <pou>.<method>");

        let argument = match (names.first(), names.get(1)) {
            (Some(&name_pou), None) => AstFactory::create_member_reference(
                AstFactory::create_identifier(name_pou, SourceLocation::internal(), self.ids.next_id()),
                None,
                self.ids.next_id(),
            ),
            (Some(&name_pou), Some(&name_method)) => AstFactory::create_member_reference(
                AstFactory::create_identifier(name_method, SourceLocation::internal(), self.ids.next_id()),
                Some(AstFactory::create_member_reference(
                    AstFactory::create_identifier(name_pou, SourceLocation::internal(), self.ids.next_id()),
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

pub(crate) mod helper {
    use plc_ast::ast::{DataType, DataTypeDeclaration, Variable};
    use plc_source::source_location::SourceLocation;
    use rustc_hash::FxHashSet;

    use crate::index::PouIndexEntry;
    use crate::typesystem::VOID_INTERNAL_NAME;

    /// Deduplicates methods by call name, keeping the first occurrence.
    /// Since `get_methods` returns declared methods first (from the child interface),
    /// this ensures we prefer the child's version over inherited ones.
    pub fn dedupe_methods_by_call_name<'a>(methods: Vec<&'a PouIndexEntry>) -> Vec<&'a PouIndexEntry> {
        let mut seen = FxHashSet::default();
        methods.into_iter().filter(|m| seen.insert(m.get_call_name())).collect()
    }

    pub fn get_itable_name(name: &str) -> String {
        format!("__itable_{name}")
    }

    pub fn get_itable_instance_name(interface: &str, pou: &str) -> String {
        format!("__itable_{interface}_{pou}_instance")
    }

    pub fn get_forward_declaration_name(interface: &str, method: &str) -> String {
        format!("__fwd_{interface}_{method}")
    }

    pub fn create_function_pointer(referenced_type: String, location: SourceLocation) -> DataType {
        DataType::PointerType {
            name: None,
            referenced_type: Box::new(DataTypeDeclaration::Reference { referenced_type, location }),
            auto_deref: None,
            type_safe: false,
            is_function: true,
        }
    }

    pub fn create_void_pointer_variable(name: &str, location: &SourceLocation) -> Variable {
        Variable {
            name: name.to_string(),
            data_type_declaration: DataTypeDeclaration::Definition {
                data_type: Box::new(DataType::PointerType {
                    name: None,
                    referenced_type: Box::new(DataTypeDeclaration::Reference {
                        referenced_type: VOID_INTERNAL_NAME.to_string(),
                        location: location.clone(),
                    }),
                    auto_deref: None,
                    type_safe: false,
                    is_function: false,
                }),
                location: location.clone(),
                scope: None,
            },
            initializer: None,
            address: None,
            location: location.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lowering::itable::tests::helper::generate_itable_artifacts;

    #[test]
    fn diamond_hierarchy() {
        //    A
        //  /   \
        // B     C
        //  \   /
        //    D
        let source = r#"
        INTERFACE A
            METHOD foo
            END_METHOD
        END_INTERFACE

        INTERFACE B EXTENDS A
            METHOD bar
            END_METHOD
        END_INTERFACE

        INTERFACE C EXTENDS A
            METHOD baz
            END_METHOD
        END_INTERFACE

        INTERFACE DD EXTENDS B, C
            METHOD qux
            END_METHOD
        END_INTERFACE
        "#;

        let (itables, instances, fwd_declarations) = generate_itable_artifacts(source);

        assert_eq!(itables.len(), 4, "one itable per interface: A, B, C and D");
        insta::assert_snapshot!(itables.join("\n\n"), @r"
        __itable_A {
            foo: __FPOINTER __fwd_A_foo;
        }

        __itable_B {
            bar: __FPOINTER __fwd_B_bar;
            foo: __FPOINTER __fwd_B_foo;
        }

        __itable_C {
            baz: __FPOINTER __fwd_C_baz;
            foo: __FPOINTER __fwd_C_foo;
        }

        __itable_DD {
            qux: __FPOINTER __fwd_DD_qux;
            bar: __FPOINTER __fwd_DD_bar;
            foo: __FPOINTER __fwd_DD_foo;
            baz: __FPOINTER __fwd_DD_baz;
        }
        ");

        assert_eq!(fwd_declarations.len(), 9, "A = foo; B = foo, bar; C = foo, baz; D = foo, bar, baz, qux");
        insta::assert_debug_snapshot!(fwd_declarations, @r#"
        [
            "__fwd_A_foo(self: POINTER TO __VOID)",
            "__fwd_B_bar(self: POINTER TO __VOID)",
            "__fwd_B_foo(self: POINTER TO __VOID)",
            "__fwd_C_baz(self: POINTER TO __VOID)",
            "__fwd_C_foo(self: POINTER TO __VOID)",
            "__fwd_DD_bar(self: POINTER TO __VOID)",
            "__fwd_DD_baz(self: POINTER TO __VOID)",
            "__fwd_DD_foo(self: POINTER TO __VOID)",
            "__fwd_DD_qux(self: POINTER TO __VOID)",
        ]
        "#);

        assert_eq!(instances.len(), 0, "no POUs implementing the interfaces, hence no instances");
        insta::assert_debug_snapshot!(instances, @"[]");
    }

    #[test]
    fn diamond_hierarchy_with_implementation() {
        //    A
        //  /   \
        // B     C
        //  \   /
        //    D
        // Each interface has one function block implementing it

        let source = r#"
        INTERFACE A
            METHOD foo
            END_METHOD
        END_INTERFACE

        INTERFACE B EXTENDS A
            METHOD bar
            END_METHOD
        END_INTERFACE

        INTERFACE C EXTENDS A
            METHOD baz
            END_METHOD
        END_INTERFACE

        INTERFACE DD EXTENDS B, C
            METHOD qux
            END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK FbA IMPLEMENTS A
            METHOD foo
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK FbB IMPLEMENTS B
            METHOD foo
            END_METHOD

            METHOD bar
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK FbC IMPLEMENTS C
            METHOD foo
            END_METHOD

            METHOD baz
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK FbD IMPLEMENTS DD
            METHOD foo
            END_METHOD

            METHOD bar
            END_METHOD

            METHOD baz
            END_METHOD

            METHOD qux
            END_METHOD
        END_FUNCTION_BLOCK
        "#;

        let (itables, instances, fwd_declarations) = generate_itable_artifacts(source);

        assert_eq!(itables.len(), 4, "one itable per interface: A, B, C and D");
        insta::assert_snapshot!(itables.join("\n\n"), @r"
        __itable_A {
            foo: __FPOINTER __fwd_A_foo;
        }

        __itable_B {
            bar: __FPOINTER __fwd_B_bar;
            foo: __FPOINTER __fwd_B_foo;
        }

        __itable_C {
            baz: __FPOINTER __fwd_C_baz;
            foo: __FPOINTER __fwd_C_foo;
        }

        __itable_DD {
            qux: __FPOINTER __fwd_DD_qux;
            bar: __FPOINTER __fwd_DD_bar;
            foo: __FPOINTER __fwd_DD_foo;
            baz: __FPOINTER __fwd_DD_baz;
        }
        ");

        assert_eq!(fwd_declarations.len(), 9, "A = foo; B = foo, bar; C = foo, baz; D = foo, bar, baz, qux");
        insta::assert_debug_snapshot!(fwd_declarations, @r#"
        [
            "__fwd_A_foo(self: POINTER TO __VOID)",
            "__fwd_B_bar(self: POINTER TO __VOID)",
            "__fwd_B_foo(self: POINTER TO __VOID)",
            "__fwd_C_baz(self: POINTER TO __VOID)",
            "__fwd_C_foo(self: POINTER TO __VOID)",
            "__fwd_DD_bar(self: POINTER TO __VOID)",
            "__fwd_DD_baz(self: POINTER TO __VOID)",
            "__fwd_DD_foo(self: POINTER TO __VOID)",
            "__fwd_DD_qux(self: POINTER TO __VOID)",
        ]
        "#);

        assert_eq!(instances.len(), 9, "one instance per direct or indirect interface implementation");
        insta::assert_debug_snapshot!(instances, @r#"
        [
            "__itable_A_FbA_instance: __itable_A := (foo := ADR(FbA.foo))",
            "__itable_A_FbB_instance: __itable_A := (foo := ADR(FbB.foo))",
            "__itable_A_FbC_instance: __itable_A := (foo := ADR(FbC.foo))",
            "__itable_A_FbD_instance: __itable_A := (foo := ADR(FbD.foo))",
            "__itable_B_FbB_instance: __itable_B := (bar := ADR(FbB.bar), foo := ADR(FbB.foo))",
            "__itable_B_FbD_instance: __itable_B := (bar := ADR(FbD.bar), foo := ADR(FbD.foo))",
            "__itable_C_FbC_instance: __itable_C := (baz := ADR(FbC.baz), foo := ADR(FbC.foo))",
            "__itable_C_FbD_instance: __itable_C := (baz := ADR(FbD.baz), foo := ADR(FbD.foo))",
            "__itable_DD_FbD_instance: __itable_DD := (qux := ADR(FbD.qux), bar := ADR(FbD.bar), foo := ADR(FbD.foo), baz := ADR(FbD.baz))",
        ]
        "#);
    }

    #[test]
    fn pou_with_implicit_interface_implementation_by_inheritance() {
        // Scenario: FbA implements interface A, FbB extends FbA and implicitly inherits interface A. As such
        // FbB can be assigned to interface A

        let source = r#"
        INTERFACE A
            METHOD foo
            END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK FbA IMPLEMENTS A
            METHOD foo
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK FbB EXTENDS FbA
        END_FUNCTION_BLOCK
        "#;

        let (itables, instances, fwd_declarations) = generate_itable_artifacts(source);

        assert_eq!(itables.len(), 1, "one itable per interface: A");
        insta::assert_snapshot!(itables.join("\n\n"), @r"
        __itable_A {
            foo: __FPOINTER __fwd_A_foo;
        }
        ");

        assert_eq!(fwd_declarations.len(), 1, "A = foo");
        insta::assert_debug_snapshot!(fwd_declarations, @r#"
        [
            "__fwd_A_foo(self: POINTER TO __VOID)",
        ]
        "#);

        assert_eq!(instances.len(), 2, "one instance per direct or indirect interface implementation");
        insta::assert_debug_snapshot!(instances, @r#"
        [
            "__itable_A_FbA_instance: __itable_A := (foo := ADR(FbA.foo))",
            "__itable_A_FbB_instance: __itable_A := (foo := ADR(FbA.foo))",
        ]
        "#);
    }

    #[test]
    fn pou_with_implicit_interface_implementation_by_inheritance_with_overridden_method() {
        // Scenario: FbA implements interface A, FbB extends FbA and implicitly inherits interface A. As such
        // FbB can be assigned to interface A. However, FbB overriddes the method from its base class.

        let source = r#"
        INTERFACE A
            METHOD foo
            END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK FbA IMPLEMENTS A
            METHOD foo
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK FbB EXTENDS FbA
            METHOD foo
            END_METHOD
        END_FUNCTION_BLOCK
        "#;

        let (itables, instances, fwd_declarations) = generate_itable_artifacts(source);

        assert_eq!(itables.len(), 1, "one itable per interface: A");
        insta::assert_snapshot!(itables.join("\n\n"), @r"
        __itable_A {
            foo: __FPOINTER __fwd_A_foo;
        }
        ");

        assert_eq!(fwd_declarations.len(), 1, "A = foo");
        insta::assert_debug_snapshot!(fwd_declarations, @r#"
        [
            "__fwd_A_foo(self: POINTER TO __VOID)",
        ]
        "#);

        // Here we expect `ADR(FbB.foo)` because of the overriden method
        assert_eq!(instances.len(), 2, "one instance per direct or indirect interface implementation");
        insta::assert_debug_snapshot!(instances, @r#"
        [
            "__itable_A_FbA_instance: __itable_A := (foo := ADR(FbA.foo))",
            "__itable_A_FbB_instance: __itable_A := (foo := ADR(FbB.foo))",
        ]
        "#);
    }

    #[test]
    fn itable_generation_with_inheritance_and_method_override() {
        // Simple inheritance scenario:
        // - `base` implements `someInterface` with method `foo`
        // - `extended` extends `base` and overrides `foo`
        //
        // Expected itable instances:
        // - (someInterface, base)     → foo: base.foo
        // - (someInterface, extended) → foo: extended.foo (override, not inherited)

        let source = r#"
            INTERFACE someInterface
                METHOD foo : STRING
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK base IMPLEMENTS someInterface
                VAR_OUTPUT
                    calledBody : STRING;
                END_VAR

                METHOD foo : STRING
                    foo := 'base.foo';
                END_METHOD

                // body
                calledBody := 'base';
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK extended EXTENDS base
                METHOD foo : STRING
                    foo := 'extended.foo';
                END_METHOD

                // body
                calledBody := 'extended';
            END_FUNCTION_BLOCK
        "#;

        let (itable_types, itable_instances, _) = generate_itable_artifacts(source);

        // Verify itable struct definition
        insta::assert_snapshot!(itable_types.join("\n\n"), @r"
        __itable_someInterface {
            foo: __FPOINTER __fwd_someInterface_foo;
        }
        ");

        // Verify global itable instances
        insta::assert_debug_snapshot!(itable_instances, @r#"
        [
            "__itable_someInterface_base_instance: __itable_someInterface := (foo := ADR(base.foo))",
            "__itable_someInterface_extended_instance: __itable_someInterface := (foo := ADR(extended.foo))",
        ]
        "#);
    }

    mod helper {
        use plc_ast::ast::{DataType, DataTypeDeclaration, Pou, UserTypeDeclaration};
        use plc_ast::provider::IdProvider;

        use crate::{lowering::itable::InterfaceTableGenerator, test_utils::tests::index_with_ids};

        /// Lowers source code and extracts serialized itable artifacts for testing.
        ///
        /// TODO: We test serialized representations of AST nodes rather than their internal
        /// structure. Fields like `is_function`, `auto_deref`, `type_safe`, etc. are not
        /// explicitly verified. We may want to add more granular tests for these in the future.
        pub fn generate_itable_artifacts(source: &str) -> (Vec<String>, Vec<String>, Vec<String>) {
            let ids = IdProvider::default();
            let (unit, index) = index_with_ids(source, ids.clone());

            let mut units = vec![unit];
            InterfaceTableGenerator::new(ids).generate(&index, &mut units);

            let unit = &units[0];

            let itables = unit
                .user_types
                .iter()
                .filter(|ty| ty.data_type.get_name().is_some_and(|n| n.starts_with("__itable_")))
                .map(serialize_type_declaration)
                .collect();

            let mut instances: Vec<_> = unit
                .global_vars
                .iter()
                .flat_map(|block| &block.variables)
                .filter(|v| v.name.starts_with("__itable_"))
                .map(|v| {
                    let DataTypeDeclaration::Reference { referenced_type, .. } = &v.data_type_declaration
                    else {
                        panic!("Expected Reference, got {:?}", v.data_type_declaration)
                    };
                    let init = v.initializer.as_ref().map(|i| i.as_string()).unwrap_or_default();
                    format!("{}: {referenced_type} := ({init})", v.name)
                })
                .collect();
            instances.sort();

            let mut forward_decls: Vec<_> = unit
                .pous
                .iter()
                .filter(|p| p.name.starts_with("__fwd_"))
                .map(serialize_forward_declaration)
                .collect();
            forward_decls.sort();

            (itables, instances, forward_decls)
        }

        /// Serializes a DataTypeDeclaration to a compact string (e.g., `POINTER TO __VOID`).
        fn serialize_data_type_decl(decl: &DataTypeDeclaration) -> String {
            match decl {
                DataTypeDeclaration::Reference { referenced_type, .. } => referenced_type.clone(),
                DataTypeDeclaration::Definition { data_type, .. } => match data_type.as_ref() {
                    DataType::PointerType { referenced_type, is_function: true, .. } => {
                        format!("__FPOINTER {}", serialize_data_type_decl(referenced_type))
                    }
                    DataType::PointerType { referenced_type, .. } => {
                        format!("POINTER TO {}", serialize_data_type_decl(referenced_type))
                    }
                    other => panic!("Unexpected data type: {:?}", other),
                },
                other => panic!("Expected Reference or Definition, got {:?}", other),
            }
        }

        /// Serializes an itable struct (e.g., `__itable_A { foo: __FPOINTER __fwd_A_foo; }`).
        fn serialize_type_declaration(decl: &UserTypeDeclaration) -> String {
            let DataType::StructType { name, variables } = &decl.data_type else {
                return format!("{:?}", decl.data_type);
            };

            let name = name.as_deref().unwrap_or("<anonymous>");
            let fields: Vec<_> = variables
                .iter()
                .map(|var| {
                    format!("    {}: {};", var.name, serialize_data_type_decl(&var.data_type_declaration))
                })
                .collect();

            format!("{name} {{\n{}\n}}", fields.join("\n"))
        }

        /// Serializes a forward declaration (e.g., `__fwd_A_foo(self: POINTER TO __VOID): DINT`).
        fn serialize_forward_declaration(pou: &Pou) -> String {
            let params: Vec<_> = pou
                .variable_blocks
                .iter()
                .flat_map(|block| &block.variables)
                .map(|var| format!("{}: {}", var.name, serialize_data_type_decl(&var.data_type_declaration)))
                .collect();

            let ret = pou
                .return_type
                .as_ref()
                .map(|rt| format!(": {}", serialize_data_type_decl(rt)))
                .unwrap_or_default();

            format!("{}({}){ret}", pou.name, params.join(", "))
        }
    }
}
