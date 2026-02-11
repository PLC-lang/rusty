//! Virtual table generation of classes and function blocks.
//!
//! This module is responsible for creating virtual tables for classes and function blocks, enabling
//! dynamic dispatch for polymorphic use cases. In short, every method call needs to be invoked indirectly
//! through a virtual table at runtime, which essentially is a collection of function pointers pointing to
//! the method implementations of POUs. The process of creating these virtual tables can be broken down into
//! three tasks:
//!
//! # 1. Virtual Table POU Member Field
//! Every root, i.e. non-extended, class or function block will receive a `__vtable: POINTER TO __VOID` member
//! field. For example a function block such as
//! ```text
//! FUNCTION_BLOCK A
//!     VAR
//!         one: DINT;
//!     END_VAR
//! END_FUNCTION_BLOCK
//! ```
//! will internally expand to
//! ```text
//! FUNCTION_BLOCK A
//!     VAR
//!         __vtable: POINTER TO __VOID;
//!         one: DINT;
//!     END_VAR
//! END_FUNCTION_BLOCK
//! ```
//! Note that we use a `VOID` type here because the actual type assigned to the `__vtable` member differs from
//! POU to POU.
//!
//! # 2. Virtual Table Struct Definition
//! As mentioned, virtual tables are essentially just a collection of function pointers reflecting a POU.
//! Consider a function block A with methods `foo` and `bar` as well as a function block B extending A with
//! methods `foo` (inherited) and `bar` (overridden) as well as `baz` (new). For these two function blocks we
//! would generate the following virtual table structures:
//!
//! ```text
//! TYPE __vtable_A:
//!     STRUCT
//!         foo: __FPOINTER TO A.foo := ADR(A.foo);
//!         bar: __FPOINTER TO A.bar := ADR(A.bar);
//!     END_STRUCT
//! END_TYPE
//!
//! TYPE __vtable_B:
//!     STRUCT
//!         foo: __FPOINTER TO A.foo := ADR(A.foo); // inherited
//!         bar: __FPOINTER TO B.bar := ADR(B.bar); // overridden
//!         baz: __FPOINTER TO B.baz := ADR(B.baz); // new
//!     END_STRUCT
//! END_TYPE
//! ```
//!
//! # 3. Global Virtual Table Instances
//! The newly created `__vtable` member fields need to point to some instance of the corresponding virtual
//! table. For that we will create one global variable for each virtual table. Deriving from the previous
//! example we would generate the following two variables
//! ```text
//! VAR_GLOBAL
//!     __vtable_instance_A: __vtable_A;
//!     __vtable_instance_B: __vtable_B;
//! END_VAR
//! ```
//! These global variables will later be assigned in the [`crate::lowering::initializers`] module.
//!
//! Note that the actual lowering of method calls to make use of these virtual tables will happen in the
//! [`crate::lowering::polymorphism`] module.

use plc_ast::{
    ast::{
        AccessModifier, AstFactory, AstNode, CompilationUnit, DataType, DataTypeDeclaration, LinkageType,
        Pou, PouType, UserTypeDeclaration, Variable, VariableBlock, VariableBlockType,
    },
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

use crate::{index::Index, typesystem::VOID_INTERNAL_NAME};

pub struct VirtualTableGenerator {
    pub ids: IdProvider,
}

impl VirtualTableGenerator {
    pub fn new(ids: IdProvider) -> VirtualTableGenerator {
        VirtualTableGenerator { ids }
    }

    pub fn generate(&mut self, index: &Index, units: &mut Vec<CompilationUnit>) {
        for unit in units {
            let mut definitions = Vec::new();
            let mut instances = Vec::new();

            for pou in unit.pous.iter_mut().filter(|pou| pou.kind.is_class() | pou.kind.is_function_block()) {
                self.patch_vtable_member(pou);
                let definition = self.generate_vtable_definition(index, pou);
                let instance = self.generate_vtable_instance(pou, &definition);

                definitions.push(definition);
                instances.push(instance);
            }

            unit.user_types.extend(definitions);
            unit.global_vars.push(VariableBlock::global().with_variables(instances));
        }
    }

    /// Patches a `__vtable: POINTER TO __VOID` member variable into the given POU
    fn patch_vtable_member(&mut self, pou: &mut Pou) {
        debug_assert!(matches!(pou.kind, PouType::Class | PouType::FunctionBlock));

        if pou.super_class.is_some() {
            return;
        }

        let location = SourceLocation::internal_in_unit(pou.location.get_file_name());
        pou.variable_blocks.insert(
            0,
            VariableBlock {
                kind: VariableBlockType::Local,
                variables: vec![Variable {
                    name: "__vtable".into(),
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
                }],
                linkage: LinkageType::Internal,
                access: AccessModifier::Protected,
                constant: false,
                retain: false,
                location: location.clone(),
            },
        );
    }

    /// Creates the virtual table struct definition for the given POU.
    fn generate_vtable_definition(&mut self, index: &Index, pou: &Pou) -> UserTypeDeclaration {
        debug_assert!(pou.kind.is_class() | pou.kind.is_function_block());

        let mut members = Vec::new();
        let location = SourceLocation::internal_in_unit(pou.location.get_file_name());

        // Function blocks need to be handled differently to classes because they're callable, e.g. `MyFb()`,
        // thus needing a `__body` function pointer.
        if pou.kind.is_function_block() {
            let member = Variable {
                name: String::from("__body"),
                data_type_declaration: DataTypeDeclaration::Definition {
                    data_type: Box::new(helper::create_function_pointer(
                        pou.name.clone(),
                        pou.location.clone(),
                    )),
                    location: location.clone(),
                    scope: None,
                },
                initializer: Some(self.generate_initalizer(pou.name.as_str())),
                address: None,
                location: location.clone(),
            };

            members.push(member);
        }

        // Iterate over all methods and create function pointer member fields for them
        for method in index.get_methods_in_fixed_order(&pou.name) {
            let member = Variable {
                name: method.get_call_name().to_string(),
                data_type_declaration: DataTypeDeclaration::Definition {
                    data_type: Box::new(helper::create_function_pointer(
                        method.get_name().into(),
                        method.get_location().into(),
                    )),
                    location: location.clone(),
                    scope: None,
                },
                initializer: Some(self.generate_initalizer(method.get_name())),
                address: None,
                location: location.clone(),
            };

            members.push(member);
        }

        UserTypeDeclaration {
            data_type: DataType::StructType { name: Some(helper::get_vtable_name(pou)), variables: members },
            initializer: None,
            location: location.clone(),
            scope: None,
            linkage: pou.linkage,
        }
    }

    /// Creates a global virtual table instance for the given POU.
    fn generate_vtable_instance(&mut self, pou: &Pou, vtable: &UserTypeDeclaration) -> Variable {
        Variable {
            name: helper::get_vtable_instance_name(pou),
            data_type_declaration: DataTypeDeclaration::Reference {
                referenced_type: vtable.data_type.get_name().unwrap().to_string(),
                location: vtable.location.clone(),
            },
            initializer: None,
            address: None,
            location: SourceLocation::internal_in_unit(pou.location.get_file_name()),
        }
    }

    /// Creates a call statement of form `ADR(<qualified name of method>)`, e.g. `ADR(MyFb.foo)`
    fn generate_initalizer(&mut self, qualified_name: &str) -> AstNode {
        // ADR(<pou>.<method>)
        // ^^^
        let operator = AstFactory::create_member_reference(
            AstFactory::create_identifier("ADR", SourceLocation::internal(), self.ids.next_id()),
            None,
            self.ids.next_id(),
        );

        // ADR(<pou>.<method>)
        //     ^^^^^^^^^^^^^^
        let names = qualified_name.split('.').collect::<Vec<_>>();
        debug_assert!(!names.is_empty() && names.len() <= 2, "expected either <pou> or <pou>.<method>");

        let argument = match (names.first(), names.get(1)) {
            // ADR(<pou>)
            (Some(name_pou), None) => AstFactory::create_member_reference(
                AstFactory::create_identifier(*name_pou, SourceLocation::internal(), self.ids.next_id()),
                None,
                self.ids.next_id(),
            ),

            // ADR(<pou>.<method>)
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

        // ADR(<pou>.<method>)
        // ^^^^^^^^^^^^^^^^^^^
        AstFactory::create_call_statement(
            operator,
            Some(argument),
            self.ids.next_id(),
            SourceLocation::internal(),
        )
    }
}

mod helper {
    use plc_ast::ast::{DataType, DataTypeDeclaration, Pou};
    use plc_source::source_location::SourceLocation;

    pub fn get_vtable_name(pou: &Pou) -> String {
        format!("__vtable_{}", pou.name)
    }

    pub fn get_vtable_instance_name(pou: &Pou) -> String {
        format!("__vtable_{}_instance", pou.name)
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
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use plc_ast::{ast::DataType, provider::IdProvider};

    use crate::{lowering::vtable::VirtualTableGenerator, test_utils::tests::index_with_ids};

    #[test]
    fn root_pou_has_vtable_member_field() {
        let ids = IdProvider::default();
        let (unit, index) = index_with_ids(
            r#"
            FUNCTION_BLOCK FbA
                VAR
                    localVar: DINT;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK

            CLASS ClA
                VAR
                    localVar: DINT;
                END_VAR
            END_CLASS

            CLASS ClB EXTENDS ClA
            END_CLASS

            "#,
            ids.clone(),
        );

        let mut units = vec![unit];
        let mut generator = VirtualTableGenerator::new(ids);
        generator.generate(&index, &mut units);

        let fb_a = units[0].pous.iter().find(|pou| pou.name == "FbA").unwrap();
        let fb_b = units[0].pous.iter().find(|pou| pou.name == "FbB").unwrap();

        assert_eq!(fb_b.variable_blocks.len(), 0);
        insta::assert_debug_snapshot!(fb_a.variable_blocks, @r#"
        [
            VariableBlock {
                variables: [
                    Variable {
                        name: "__vtable",
                        data_type: DataTypeDefinition {
                            data_type: PointerType {
                                name: None,
                                referenced_type: DataTypeReference {
                                    referenced_type: "__VOID",
                                },
                                auto_deref: None,
                                type_safe: false,
                                is_function: false,
                            },
                        },
                    },
                ],
                variable_block_type: Local,
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "localVar",
                        data_type: DataTypeReference {
                            referenced_type: "DINT",
                        },
                    },
                ],
                variable_block_type: Local,
            },
        ]
        "#);

        let cl_a = units[0].pous.iter().find(|pou| pou.name == "ClA").unwrap();
        let cl_b = units[0].pous.iter().find(|pou| pou.name == "ClB").unwrap();

        assert_eq!(cl_b.variable_blocks.len(), 0);
        insta::assert_debug_snapshot!(cl_a.variable_blocks, @r#"
        [
            VariableBlock {
                variables: [
                    Variable {
                        name: "__vtable",
                        data_type: DataTypeDefinition {
                            data_type: PointerType {
                                name: None,
                                referenced_type: DataTypeReference {
                                    referenced_type: "__VOID",
                                },
                                auto_deref: None,
                                type_safe: false,
                                is_function: false,
                            },
                        },
                    },
                ],
                variable_block_type: Local,
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "localVar",
                        data_type: DataTypeReference {
                            referenced_type: "DINT",
                        },
                    },
                ],
                variable_block_type: Local,
            },
        ]
        "#);
    }

    #[test]
    fn virtual_table_has_body_entry_for_function_blocks() {
        let ids = IdProvider::default();
        let (unit, index) = index_with_ids(
            r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            CLASS ClA
            END_CLASS
            "#,
            ids.clone(),
        );

        let mut units = vec![unit];
        let mut generator = VirtualTableGenerator::new(ids);
        generator.generate(&index, &mut units);

        let fb_a = units[0].user_types.iter().find(|ut| ut.data_type.get_name().unwrap() == "__vtable_FbA");
        insta::assert_debug_snapshot!(fb_a.unwrap(), @r#"
        UserTypeDeclaration {
            data_type: StructType {
                name: Some(
                    "__vtable_FbA",
                ),
                variables: [
                    Variable {
                        name: "__body",
                        data_type: DataTypeDefinition {
                            data_type: PointerType {
                                name: None,
                                referenced_type: DataTypeReference {
                                    referenced_type: "FbA",
                                },
                                auto_deref: None,
                                type_safe: false,
                                is_function: true,
                            },
                        },
                        initializer: Some(
                            CallStatement {
                                operator: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "ADR",
                                        },
                                    ),
                                    base: None,
                                },
                                parameters: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "FbA",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
                            },
                        ),
                    },
                ],
            },
            initializer: None,
            scope: None,
        }
        "#);

        // Classes are not callable, hence no `__body` entry
        let cl_a = units[0].user_types.iter().find(|ut| ut.data_type.get_name().unwrap() == "__vtable_ClA");
        insta::assert_debug_snapshot!(cl_a.unwrap(), @r#"
        UserTypeDeclaration {
            data_type: StructType {
                name: Some(
                    "__vtable_ClA",
                ),
                variables: [],
            },
            initializer: None,
            scope: None,
        }
        "#);
    }

    #[test]
    fn virtual_table_struct_definitions_are_generated() {
        let ids = IdProvider::default();
        let (unit, index) = index_with_ids(
            r#"
            FUNCTION_BLOCK FbA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            CLASS ClA
                METHOD foo
                END_METHOD
            END_CLASS
            "#,
            ids.clone(),
        );

        let mut units = vec![unit];
        let mut generator = VirtualTableGenerator::new(ids);
        generator.generate(&index, &mut units);

        let fb_a = units[0].user_types.iter().find(|ut| ut.data_type.get_name().unwrap() == "__vtable_FbA");
        insta::assert_debug_snapshot!(fb_a.unwrap(), @r#"
        UserTypeDeclaration {
            data_type: StructType {
                name: Some(
                    "__vtable_FbA",
                ),
                variables: [
                    Variable {
                        name: "__body",
                        data_type: DataTypeDefinition {
                            data_type: PointerType {
                                name: None,
                                referenced_type: DataTypeReference {
                                    referenced_type: "FbA",
                                },
                                auto_deref: None,
                                type_safe: false,
                                is_function: true,
                            },
                        },
                        initializer: Some(
                            CallStatement {
                                operator: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "ADR",
                                        },
                                    ),
                                    base: None,
                                },
                                parameters: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "FbA",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
                            },
                        ),
                    },
                    Variable {
                        name: "foo",
                        data_type: DataTypeDefinition {
                            data_type: PointerType {
                                name: None,
                                referenced_type: DataTypeReference {
                                    referenced_type: "FbA.foo",
                                },
                                auto_deref: None,
                                type_safe: false,
                                is_function: true,
                            },
                        },
                        initializer: Some(
                            CallStatement {
                                operator: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "ADR",
                                        },
                                    ),
                                    base: None,
                                },
                                parameters: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "foo",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "FbA",
                                                    },
                                                ),
                                                base: None,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                ],
            },
            initializer: None,
            scope: None,
        }
        "#);

        let cl_a = units[0].user_types.iter().find(|ut| ut.data_type.get_name().unwrap() == "__vtable_ClA");
        insta::assert_debug_snapshot!(cl_a.unwrap(), @r#"
        UserTypeDeclaration {
            data_type: StructType {
                name: Some(
                    "__vtable_ClA",
                ),
                variables: [
                    Variable {
                        name: "foo",
                        data_type: DataTypeDefinition {
                            data_type: PointerType {
                                name: None,
                                referenced_type: DataTypeReference {
                                    referenced_type: "ClA.foo",
                                },
                                auto_deref: None,
                                type_safe: false,
                                is_function: true,
                            },
                        },
                        initializer: Some(
                            CallStatement {
                                operator: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "ADR",
                                        },
                                    ),
                                    base: None,
                                },
                                parameters: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "foo",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "ClA",
                                                    },
                                                ),
                                                base: None,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                ],
            },
            initializer: None,
            scope: None,
        }
        "#);
    }

    #[test]
    fn virtual_table_struct_definitions_are_generated_for_overridden_methods() {
        let ids = IdProvider::default();
        let (unit, index) = index_with_ids(
            r#"
            FUNCTION_BLOCK FbA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            CLASS ClA
                METHOD foo
                END_METHOD
            END_CLASS

            CLASS ClB EXTENDS ClA
                METHOD foo
                END_METHOD
            END_CLASS
            "#,
            ids.clone(),
        );

        let mut units = vec![unit];
        let mut generator = VirtualTableGenerator::new(ids);
        generator.generate(&index, &mut units);

        let fb_b = units[0].user_types.iter().find(|ut| ut.data_type.get_name().unwrap() == "__vtable_FbB");
        insta::assert_debug_snapshot!(fb_b.unwrap(), @r#"
        UserTypeDeclaration {
            data_type: StructType {
                name: Some(
                    "__vtable_FbB",
                ),
                variables: [
                    Variable {
                        name: "__body",
                        data_type: DataTypeDefinition {
                            data_type: PointerType {
                                name: None,
                                referenced_type: DataTypeReference {
                                    referenced_type: "FbB",
                                },
                                auto_deref: None,
                                type_safe: false,
                                is_function: true,
                            },
                        },
                        initializer: Some(
                            CallStatement {
                                operator: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "ADR",
                                        },
                                    ),
                                    base: None,
                                },
                                parameters: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "FbB",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
                            },
                        ),
                    },
                    Variable {
                        name: "foo",
                        data_type: DataTypeDefinition {
                            data_type: PointerType {
                                name: None,
                                referenced_type: DataTypeReference {
                                    referenced_type: "FbB.foo",
                                },
                                auto_deref: None,
                                type_safe: false,
                                is_function: true,
                            },
                        },
                        initializer: Some(
                            CallStatement {
                                operator: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "ADR",
                                        },
                                    ),
                                    base: None,
                                },
                                parameters: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "foo",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "FbB",
                                                    },
                                                ),
                                                base: None,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                ],
            },
            initializer: None,
            scope: None,
        }
        "#);

        let cl_b = units[0].user_types.iter().find(|ut| ut.data_type.get_name().unwrap() == "__vtable_ClB");
        insta::assert_debug_snapshot!(cl_b.unwrap(), @r#"
        UserTypeDeclaration {
            data_type: StructType {
                name: Some(
                    "__vtable_ClB",
                ),
                variables: [
                    Variable {
                        name: "foo",
                        data_type: DataTypeDefinition {
                            data_type: PointerType {
                                name: None,
                                referenced_type: DataTypeReference {
                                    referenced_type: "ClB.foo",
                                },
                                auto_deref: None,
                                type_safe: false,
                                is_function: true,
                            },
                        },
                        initializer: Some(
                            CallStatement {
                                operator: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "ADR",
                                        },
                                    ),
                                    base: None,
                                },
                                parameters: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "foo",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "ClB",
                                                    },
                                                ),
                                                base: None,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                ],
            },
            initializer: None,
            scope: None,
        }
        "#);
    }

    #[test]
    fn virtual_table_struct_definitions_are_generated_for_new_methods() {
        let ids = IdProvider::default();
        let (unit, index) = index_with_ids(
            r#"
            FUNCTION_BLOCK FbA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
                METHOD bar
                END_METHOD
            END_FUNCTION_BLOCK

            CLASS ClA
                METHOD foo
                END_METHOD
            END_CLASS

            CLASS ClB EXTENDS ClA
                METHOD bar
                END_METHOD
            END_CLASS
            "#,
            ids.clone(),
        );

        let mut units = vec![unit];
        let mut generator = VirtualTableGenerator::new(ids);
        generator.generate(&index, &mut units);

        let fb_b = units[0].user_types.iter().find(|ut| ut.data_type.get_name().unwrap() == "__vtable_FbB");
        insta::assert_debug_snapshot!(fb_b.unwrap(), @r#"
        UserTypeDeclaration {
            data_type: StructType {
                name: Some(
                    "__vtable_FbB",
                ),
                variables: [
                    Variable {
                        name: "__body",
                        data_type: DataTypeDefinition {
                            data_type: PointerType {
                                name: None,
                                referenced_type: DataTypeReference {
                                    referenced_type: "FbB",
                                },
                                auto_deref: None,
                                type_safe: false,
                                is_function: true,
                            },
                        },
                        initializer: Some(
                            CallStatement {
                                operator: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "ADR",
                                        },
                                    ),
                                    base: None,
                                },
                                parameters: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "FbB",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
                            },
                        ),
                    },
                    Variable {
                        name: "foo",
                        data_type: DataTypeDefinition {
                            data_type: PointerType {
                                name: None,
                                referenced_type: DataTypeReference {
                                    referenced_type: "FbA.foo",
                                },
                                auto_deref: None,
                                type_safe: false,
                                is_function: true,
                            },
                        },
                        initializer: Some(
                            CallStatement {
                                operator: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "ADR",
                                        },
                                    ),
                                    base: None,
                                },
                                parameters: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "foo",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "FbA",
                                                    },
                                                ),
                                                base: None,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Variable {
                        name: "bar",
                        data_type: DataTypeDefinition {
                            data_type: PointerType {
                                name: None,
                                referenced_type: DataTypeReference {
                                    referenced_type: "FbB.bar",
                                },
                                auto_deref: None,
                                type_safe: false,
                                is_function: true,
                            },
                        },
                        initializer: Some(
                            CallStatement {
                                operator: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "ADR",
                                        },
                                    ),
                                    base: None,
                                },
                                parameters: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "bar",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "FbB",
                                                    },
                                                ),
                                                base: None,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                ],
            },
            initializer: None,
            scope: None,
        }
        "#);

        let cl_b = units[0].user_types.iter().find(|ut| ut.data_type.get_name().unwrap() == "__vtable_ClB");
        insta::assert_debug_snapshot!(cl_b.unwrap(), @r#"
        UserTypeDeclaration {
            data_type: StructType {
                name: Some(
                    "__vtable_ClB",
                ),
                variables: [
                    Variable {
                        name: "foo",
                        data_type: DataTypeDefinition {
                            data_type: PointerType {
                                name: None,
                                referenced_type: DataTypeReference {
                                    referenced_type: "ClA.foo",
                                },
                                auto_deref: None,
                                type_safe: false,
                                is_function: true,
                            },
                        },
                        initializer: Some(
                            CallStatement {
                                operator: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "ADR",
                                        },
                                    ),
                                    base: None,
                                },
                                parameters: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "foo",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "ClA",
                                                    },
                                                ),
                                                base: None,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                    Variable {
                        name: "bar",
                        data_type: DataTypeDefinition {
                            data_type: PointerType {
                                name: None,
                                referenced_type: DataTypeReference {
                                    referenced_type: "ClB.bar",
                                },
                                auto_deref: None,
                                type_safe: false,
                                is_function: true,
                            },
                        },
                        initializer: Some(
                            CallStatement {
                                operator: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "ADR",
                                        },
                                    ),
                                    base: None,
                                },
                                parameters: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "bar",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "ClB",
                                                    },
                                                ),
                                                base: None,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                ],
            },
            initializer: None,
            scope: None,
        }
        "#);
    }

    #[test]
    fn global_variable_instances_are_generated() {
        let ids = IdProvider::default();
        let (unit, index) = index_with_ids(
            r#"
            FUNCTION_BLOCK FbA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
                METHOD bar
                END_METHOD
            END_FUNCTION_BLOCK

            CLASS ClA
                METHOD foo
                END_METHOD
            END_CLASS

            CLASS ClB EXTENDS ClA
                METHOD bar
                END_METHOD
            END_CLASS
            "#,
            ids.clone(),
        );

        let mut units = vec![unit];
        let mut generator = VirtualTableGenerator::new(ids);
        generator.generate(&index, &mut units);

        insta::assert_debug_snapshot!(units[0].global_vars, @r#"
        [
            VariableBlock {
                variables: [],
                variable_block_type: Global,
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "__vtable_FbA_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_FbA",
                        },
                    },
                    Variable {
                        name: "__vtable_FbB_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_FbB",
                        },
                    },
                    Variable {
                        name: "__vtable_ClA_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_ClA",
                        },
                    },
                    Variable {
                        name: "__vtable_ClB_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_ClB",
                        },
                    },
                ],
                variable_block_type: Global,
            },
        ]
        "#);
    }

    #[test]
    fn order_of_methods_in_virtual_table_is_constant() {
        let ids = IdProvider::default();
        let (unit, index) = index_with_ids(
            r"
            FUNCTION_BLOCK FbA
                METHOD one
                END_METHOD

                METHOD two
                END_METHOD

                METHOD three
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
                METHOD four
                END_METHOD

                METHOD one
                END_METHOD

                METHOD five
                END_METHOD

                METHOD two
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbC EXTENDS FbB
                METHOD six
                END_METHOD

                METHOD five
                END_METHOD
            END_FUNCTION_BLOCK

            CLASS ClassA
                METHOD one
                END_METHOD

                METHOD two
                END_METHOD

                METHOD three
                END_METHOD
            END_CLASS

            CLASS ClassB EXTENDS ClassA
                METHOD four
                END_METHOD

                METHOD one
                END_METHOD

                METHOD five
                END_METHOD

                METHOD two
                END_METHOD
            END_CLASS

            CLASS ClassC EXTENDS ClassB
                METHOD six
                END_METHOD

                METHOD five
                END_METHOD
            END_CLASS
            ",
            ids.clone(),
        );

        let mut units = vec![unit];
        let mut generator = VirtualTableGenerator::new(ids);
        generator.generate(&index, &mut units);

        let mut types = Vec::new();
        for ty in &units[0].user_types {
            let DataType::StructType { name: Some(name), variables } = &ty.data_type else { unreachable!() };
            let var_names = variables.iter().map(|var| &var.name).join(", ");

            types.push(format!("{name}: {var_names}"));
        }

        // Despite the methods being declared "chaotically" (not in order) in the child POUs, we expect them
        // to appear in order in the actual struct definition. This is required to safely upcast from a child
        // POU to a parent POU in the context of polymorphic calls.
        insta::assert_debug_snapshot!(types, @r#"
        [
            "__vtable_FbA: __body, one, two, three",
            "__vtable_FbB: __body, one, two, three, four, five",
            "__vtable_FbC: __body, one, two, three, four, five, six",
            "__vtable_ClassA: one, two, three",
            "__vtable_ClassB: one, two, three, four, five",
            "__vtable_ClassC: one, two, three, four, five, six",
        ]
        "#);
    }
}
