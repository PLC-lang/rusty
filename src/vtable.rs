use plc_ast::{
    ast::{
        AccessModifier, DataType, DataTypeDeclaration, LinkageType, UserTypeDeclaration, Variable,
        VariableBlock, VariableBlockType,
    },
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;
use plc_util::convention::{generate_vtable_name, generate_vtable_type_name};

use crate::{index::Index, typesystem::VOID_POINTER_INTERNAL_NAME};

pub struct VTableIndexer {
    pub id_provider: IdProvider,
}

impl VTableIndexer {
    pub fn new(id_provider: IdProvider) -> Self {
        Self { id_provider }
    }

    pub fn create_vtables_for_pous(index: &Index) -> Vec<UserTypeDeclaration> {
        let mut vtables = Vec::new();
        for pou in index.get_pous().values().filter(|pou| pou.is_function_block() || pou.is_class()) {
            let mut variables = Vec::new();

            if let Some(parent) = pou.get_super_class() {
                variables.push(VTableIndexer::create_vtable_reference(parent));
            }

            for interface in pou.get_interfaces() {
                variables.push(VTableIndexer::create_vtable_reference(interface));
            }

            if pou.is_function_block() {
                variables.push(VTableIndexer::create_void_pointer("__body"));
            }

            for method in index.get_methods_local(pou.get_name()) {
                variables.push(VTableIndexer::create_void_pointer(method.get_name()));
            }

            vtables.push(VTableIndexer::create_vtable(pou.get_name(), variables));
        }

        vtables
    }

    pub fn create_global_variables_for_vtable(index: &Index) -> (VariableBlock, VariableBlock) {
        let mut internals = Vec::new();
        let mut externals = Vec::new();
        for pou in index.get_pous().values().filter(|pou| pou.is_function_block() || pou.is_class()) {
            let name = generate_vtable_name(pou.get_name());
            let type_name = generate_vtable_type_name(pou.get_name());
            let variable = Variable {
                name: name.to_string(),
                data_type_declaration: DataTypeDeclaration::Reference {
                    referenced_type: type_name.to_string(),
                    location: SourceLocation::internal(),
                },
                initializer: None,
                address: None,
                location: {
                    // We have some logic in the codegen where global variables are generated as an external
                    // if they don't have the same file location as the current unit.
                    pou.get_location().clone().into_internal_with_file()
                },
            };

            if matches!(pou.get_linkage(), LinkageType::External) {
                externals.push(variable);
            } else {
                internals.push(variable);
            }
        }
        (
            VariableBlock {
                access: AccessModifier::Protected,
                constant: false,
                retain: false,
                variables: internals,
                kind: VariableBlockType::Global,
                linkage: LinkageType::Internal,
                location: SourceLocation::internal(),
            },
            VariableBlock {
                access: AccessModifier::Protected,
                constant: false,
                retain: false,
                variables: externals,
                kind: VariableBlockType::Global,
                linkage: LinkageType::External,
                location: SourceLocation::internal(),
            },
        )
    }

    pub fn create_vtables_for_interfaces(index: &Index) -> Vec<UserTypeDeclaration> {
        let mut vtables = Vec::new();
        for interface in index.get_interfaces().values() {
            let mut variables = Vec::new();
            for extension in &interface.extensions {
                variables.push(VTableIndexer::create_vtable_reference(&extension.name));
            }

            for method in interface.get_declared_methods(index) {
                variables.push(VTableIndexer::create_void_pointer(method.get_name()));
            }

            vtables.push(VTableIndexer::create_vtable(interface.get_name(), variables));
        }

        vtables
    }

    /// Creates a void pointer variable with the given name and location
    fn create_void_pointer(name: &str) -> Variable {
        Variable {
            name: name.to_string(),
            data_type_declaration: DataTypeDeclaration::Reference {
                referenced_type: VOID_POINTER_INTERNAL_NAME.into(),
                location: SourceLocation::internal(),
            },
            initializer: None,
            address: None,
            location: SourceLocation::internal(),
        }
    }

    fn create_vtable_reference(name: &str) -> Variable {
        Variable {
            name: generate_vtable_type_name(name),
            data_type_declaration: DataTypeDeclaration::Reference {
                referenced_type: generate_vtable_type_name(name),
                location: SourceLocation::internal(),
            },
            initializer: None,
            address: None,
            location: SourceLocation::internal(),
        }
    }

    /// Creates a vtable with the given member variables and a mangled name of the form `__vtable_<name>`
    fn create_vtable(name: &str, variables: Vec<Variable>) -> UserTypeDeclaration {
        UserTypeDeclaration {
            data_type: DataType::StructType { name: Some(generate_vtable_type_name(name)), variables },
            initializer: None,
            location: SourceLocation::internal(),
            scope: Some(name.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use plc_ast::ast::LinkageType;

    use crate::{test_utils::tests::index, vtable::VTableIndexer};
    #[test]
    fn function_block_gets_vtable() {
        let src = r#"
            FUNCTION_BLOCK Test
            METHOD TestMethod
            END_METHOD
            END_FUNCTION_BLOCK
            FUNCTION_BLOCK Test2
            END_FUNCTION_BLOCK
        "#;

        let (_unit, index) = index(src);
        let vtables = VTableIndexer::create_vtables_for_pous(&index);
        insta::assert_debug_snapshot!(vtables, @r#"
        [
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_Test_type",
                    ),
                    variables: [
                        Variable {
                            name: "__body",
                            data_type: DataTypeReference {
                                referenced_type: "__VOID_POINTER",
                            },
                        },
                        Variable {
                            name: "Test.TestMethod",
                            data_type: DataTypeReference {
                                referenced_type: "__VOID_POINTER",
                            },
                        },
                    ],
                },
                initializer: None,
                scope: Some(
                    "Test",
                ),
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_Test2_type",
                    ),
                    variables: [
                        Variable {
                            name: "__body",
                            data_type: DataTypeReference {
                                referenced_type: "__VOID_POINTER",
                            },
                        },
                    ],
                },
                initializer: None,
                scope: Some(
                    "Test2",
                ),
            },
        ]
        "#);
    }

    #[test]
    fn classes_gets_vtable() {
        let src = r#"
            CLASS Test
            METHOD TestMethod
            END_METHOD
            END_CLASS
            CLASS Test2
            END_CLASS
        "#;

        let (_unit, index) = index(src);
        let vtables = VTableIndexer::create_vtables_for_pous(&index);
        insta::assert_debug_snapshot!(vtables, @r#"
        [
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_Test_type",
                    ),
                    variables: [
                        Variable {
                            name: "Test.TestMethod",
                            data_type: DataTypeReference {
                                referenced_type: "__VOID_POINTER",
                            },
                        },
                    ],
                },
                initializer: None,
                scope: Some(
                    "Test",
                ),
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_Test2_type",
                    ),
                    variables: [],
                },
                initializer: None,
                scope: Some(
                    "Test2",
                ),
            },
        ]
        "#);
    }

    #[test]
    fn interface_gets_vtable() {
        let src = r#"
            INTERFACE Test
            METHOD TestMethod
            END_METHOD
            END_INTERFACE
            INTERFACE Test2
            END_INTERFACE
        "#;

        let (_unit, index) = index(src);
        let vtables = VTableIndexer::create_vtables_for_interfaces(&index);
        insta::assert_debug_snapshot!(vtables, @r#"
        [
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_Test_type",
                    ),
                    variables: [
                        Variable {
                            name: "Test.TestMethod",
                            data_type: DataTypeReference {
                                referenced_type: "__VOID_POINTER",
                            },
                        },
                    ],
                },
                initializer: None,
                scope: Some(
                    "Test",
                ),
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_Test2_type",
                    ),
                    variables: [],
                },
                initializer: None,
                scope: Some(
                    "Test2",
                ),
            },
        ]
        "#);
    }

    #[test]
    fn parent_fb_is_referenced_in_child_vtable() {
        let src = r#"
            FUNCTION_BLOCK Test
            METHOD TestMethod
            END_METHOD
            END_FUNCTION_BLOCK
            FUNCTION_BLOCK Test2 EXTENDS Test
            END_FUNCTION_BLOCK
        "#;

        let (_unit, index) = index(src);
        let vtables = VTableIndexer::create_vtables_for_pous(&index);
        insta::assert_debug_snapshot!(vtables, @r###"
        [
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_Test_type",
                    ),
                    variables: [
                        Variable {
                            name: "__body",
                            data_type: DataTypeReference {
                                referenced_type: "__VOID_POINTER",
                            },
                        },
                        Variable {
                            name: "Test.TestMethod",
                            data_type: DataTypeReference {
                                referenced_type: "__VOID_POINTER",
                            },
                        },
                    ],
                },
                initializer: None,
                scope: Some(
                    "Test",
                ),
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_Test2_type",
                    ),
                    variables: [
                        Variable {
                            name: "__vtable_Test_type",
                            data_type: DataTypeReference {
                                referenced_type: "__vtable_Test_type",
                            },
                        },
                        Variable {
                            name: "__body",
                            data_type: DataTypeReference {
                                referenced_type: "__VOID_POINTER",
                            },
                        },
                    ],
                },
                initializer: None,
                scope: Some(
                    "Test2",
                ),
            },
        ]
        "###);
    }

    #[test]
    fn top_level_function_block_type_has_vtable_pointer() {
        //TODO:
    }

    #[test]
    fn implemented_interfaces_are_referenced_in_vtable() {
        let src = r#"
            FUNCTION_BLOCK Test IMPLEMENTS TestInt
            METHOD TestMethod
            END_METHOD
            END_FUNCTION_BLOCK
            FUNCTION_BLOCK Test2 EXTENDS Test IMPLEMENTS TestInt2
            END_FUNCTION_BLOCK
            INTERFACE TestInt
            METHOD TestMethod
            END_METHOD
            END_INTERFACE
            INTERFACE TestInt2
            END_INTERFACE
        "#;

        let (_unit, index) = index(src);
        let mut vtables = VTableIndexer::create_vtables_for_pous(&index);
        vtables.extend(VTableIndexer::create_vtables_for_interfaces(&index));
        insta::assert_debug_snapshot!(vtables, @r###"
        [
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_Test_type",
                    ),
                    variables: [
                        Variable {
                            name: "__vtable_TestInt_type",
                            data_type: DataTypeReference {
                                referenced_type: "__vtable_TestInt_type",
                            },
                        },
                        Variable {
                            name: "__body",
                            data_type: DataTypeReference {
                                referenced_type: "__VOID_POINTER",
                            },
                        },
                        Variable {
                            name: "Test.TestMethod",
                            data_type: DataTypeReference {
                                referenced_type: "__VOID_POINTER",
                            },
                        },
                    ],
                },
                initializer: None,
                scope: Some(
                    "Test",
                ),
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_Test2_type",
                    ),
                    variables: [
                        Variable {
                            name: "__vtable_Test_type",
                            data_type: DataTypeReference {
                                referenced_type: "__vtable_Test_type",
                            },
                        },
                        Variable {
                            name: "__vtable_TestInt2_type",
                            data_type: DataTypeReference {
                                referenced_type: "__vtable_TestInt2_type",
                            },
                        },
                        Variable {
                            name: "__body",
                            data_type: DataTypeReference {
                                referenced_type: "__VOID_POINTER",
                            },
                        },
                    ],
                },
                initializer: None,
                scope: Some(
                    "Test2",
                ),
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_TestInt_type",
                    ),
                    variables: [
                        Variable {
                            name: "TestInt.TestMethod",
                            data_type: DataTypeReference {
                                referenced_type: "__VOID_POINTER",
                            },
                        },
                    ],
                },
                initializer: None,
                scope: Some(
                    "TestInt",
                ),
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_TestInt2_type",
                    ),
                    variables: [],
                },
                initializer: None,
                scope: Some(
                    "TestInt2",
                ),
            },
        ]
        "###);
    }

    #[test]
    fn methods_are_last_field_in_vtable() {
        //TODO: order
        let src = r#"
            FUNCTION_BLOCK Test IMPLEMENTS TestInt
            METHOD TestMethod
            END_METHOD
            END_FUNCTION_BLOCK
            FUNCTION_BLOCK Test2 EXTENDS Test IMPLEMENTS TestInt2
            METHOD TestMethod2
            END_METHOD
            END_FUNCTION_BLOCK
            INTERFACE TestInt
            METHOD TestMethod
            END_METHOD
            END_INTERFACE
            INTERFACE TestInt2
            END_INTERFACE
        "#;

        let (_unit, index) = index(src);
        let mut vtables = VTableIndexer::create_vtables_for_pous(&index);
        vtables.extend(VTableIndexer::create_vtables_for_interfaces(&index));
        insta::assert_debug_snapshot!(vtables, @r###"
        [
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_Test_type",
                    ),
                    variables: [
                        Variable {
                            name: "__vtable_TestInt_type",
                            data_type: DataTypeReference {
                                referenced_type: "__vtable_TestInt_type",
                            },
                        },
                        Variable {
                            name: "__body",
                            data_type: DataTypeReference {
                                referenced_type: "__VOID_POINTER",
                            },
                        },
                        Variable {
                            name: "Test.TestMethod",
                            data_type: DataTypeReference {
                                referenced_type: "__VOID_POINTER",
                            },
                        },
                    ],
                },
                initializer: None,
                scope: Some(
                    "Test",
                ),
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_Test2_type",
                    ),
                    variables: [
                        Variable {
                            name: "__vtable_Test_type",
                            data_type: DataTypeReference {
                                referenced_type: "__vtable_Test_type",
                            },
                        },
                        Variable {
                            name: "__vtable_TestInt2_type",
                            data_type: DataTypeReference {
                                referenced_type: "__vtable_TestInt2_type",
                            },
                        },
                        Variable {
                            name: "__body",
                            data_type: DataTypeReference {
                                referenced_type: "__VOID_POINTER",
                            },
                        },
                        Variable {
                            name: "Test2.TestMethod2",
                            data_type: DataTypeReference {
                                referenced_type: "__VOID_POINTER",
                            },
                        },
                    ],
                },
                initializer: None,
                scope: Some(
                    "Test2",
                ),
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_TestInt_type",
                    ),
                    variables: [
                        Variable {
                            name: "TestInt.TestMethod",
                            data_type: DataTypeReference {
                                referenced_type: "__VOID_POINTER",
                            },
                        },
                    ],
                },
                initializer: None,
                scope: Some(
                    "TestInt",
                ),
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_TestInt2_type",
                    ),
                    variables: [],
                },
                initializer: None,
                scope: Some(
                    "TestInt2",
                ),
            },
        ]
        "###);
    }

    #[test]
    fn functions_dont_get_vtable() {
        let src = r#"
            FUNCTION Test
            END_FUNCTION
            FUNCTION Test2
            END_FUNCTION
        "#;

        let (_unit, index) = index(src);
        let vtables = VTableIndexer::create_vtables_for_pous(&index);
        insta::assert_debug_snapshot!(vtables, @"[]");
    }

    #[test]
    fn programs_dont_get_vtable() {
        let src = r#"
            PROGRAM Test
            END_PROGRAM
            PROGRAM Test2
            END_PROGRAM"#;
        let (_unit, index) = index(src);
        let vtables = VTableIndexer::create_vtables_for_pous(&index);
        insta::assert_debug_snapshot!(vtables, @"[]");
    }

    //TODO:
    // overriden methods don't appear in vtable
    // interfaces already implemented by a parent class don't appear in vtable
    //

    #[test]
    fn function_block_vtables_get_a_global_variable() {
        let src = r#"
            FUNCTION_BLOCK Test
            METHOD TestMethod
            END_METHOD
            END_FUNCTION_BLOCK
            FUNCTION_BLOCK Test2 EXTENDS Test
            END_FUNCTION_BLOCK
            {external}
            FUNCTION_BLOCK Test3
            METHOD TestMethod
            END_METHOD
            END_FUNCTION_BLOCK
            FUNCTION_BLOCK Test4 EXTENDS Test3
            END_FUNCTION_BLOCK
        "#;

        let (_unit, index) = index(src);
        let (internals, externals) = VTableIndexer::create_global_variables_for_vtable(&index);

        assert_eq!(internals.linkage, LinkageType::Internal);
        assert_eq!(externals.linkage, LinkageType::External);

        insta::assert_debug_snapshot!(internals, @r#"
        VariableBlock {
            variables: [
                Variable {
                    name: "__vtable_Test",
                    data_type: DataTypeReference {
                        referenced_type: "__vtable_Test_type",
                    },
                },
                Variable {
                    name: "__vtable_Test2",
                    data_type: DataTypeReference {
                        referenced_type: "__vtable_Test2_type",
                    },
                },
                Variable {
                    name: "__vtable_Test4",
                    data_type: DataTypeReference {
                        referenced_type: "__vtable_Test4_type",
                    },
                },
            ],
            variable_block_type: Global,
        }
        "#);
        insta::assert_debug_snapshot!(externals, @r#"
        VariableBlock {
            variables: [
                Variable {
                    name: "__vtable_Test3",
                    data_type: DataTypeReference {
                        referenced_type: "__vtable_Test3_type",
                    },
                },
            ],
            variable_block_type: Global,
        }
        "#);
    }
}
