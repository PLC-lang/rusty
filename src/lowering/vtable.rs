//! Module responsible for creating virtual tables
//!
//! TODO: Documentation, in a nutshell:
//! * Virtual table struct definition
//! * Virtual table instance (global variable)
//! * Virtual table as member field in POU

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
                self.inject_vtable_member(pou);
                let definition = self.generate_vtable_struct_definition(index, pou);
                let instance = self.generate_global_vtable_instance(pou, &definition);

                definitions.push(definition);
                instances.push(instance);
            }

            unit.user_types.extend(definitions);
            unit.global_vars.push(VariableBlock::global().with_variables(instances));
        }
    }

    /// Injects a `__vtable` member variable into the POU when dealing with a class or function block that
    /// does NOT extend another. In terms of source code we would get
    /// ```norun
    /// FUNCTION_BLOCK MyFb
    ///     VAR
    ///         __vtable: POINTER TO __VOID;
    ///         myVariable: DINT;
    ///     END_VAR
    /// END_FUNCTION_BLOCK
    /// ```
    fn inject_vtable_member(&mut self, pou: &mut Pou) {
        let location = SourceLocation::internal_in_unit(pou.location.get_file_name());

        if pou.super_class.is_none() && matches!(pou.kind, PouType::Class | PouType::FunctionBlock) {
            pou.variable_blocks.insert(
                0,
                VariableBlock {
                    kind: VariableBlockType::Local,
                    variables: vec![Variable {
                        name: "__vtable".into(),
                        data_type_declaration: DataTypeDeclaration::Definition {
                            data_type: DataType::PointerType {
                                name: None,
                                referenced_type: Box::new(DataTypeDeclaration::Reference {
                                    referenced_type: VOID_INTERNAL_NAME.to_string(),
                                    location: location.clone(),
                                }),
                                auto_deref: None,
                                type_safe: false,
                                is_function: false,
                            },
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
    }

    /// Creates the virtual table struct definition of the given POU. In terms of source code something alike
    /// ```norun
    /// TYPE vtable:
    ///     STRUCT
    ///         // <method name>: POINTER TO <POU>.<method name> := ADR(<POU>.<method name>);
    ///         foo: POINTER TO MyFb.foo := ADR(MyFb.foo);
    ///         ...
    ///     END_STRUCT
    /// END_TYPE
    /// ```
    fn generate_vtable_struct_definition(&mut self, index: &Index, pou: &Pou) -> UserTypeDeclaration {
        debug_assert!(pou.kind.is_class() | pou.kind.is_function_block());

        let mut members = Vec::new();
        let location = SourceLocation::internal_in_unit(pou.location.get_file_name());

        // Function blocks need to be handled differently to classes because they're callable, e.g. `MyFb()`,
        // thus needing a `__body` function pointer.
        if pou.kind.is_function_block() {
            let member = Variable {
                name: String::from("__body"),
                data_type_declaration: DataTypeDeclaration::Definition {
                    data_type: helper::create_function_pointer(pou.name.clone(), pou.location.clone()),
                    location: location.clone(),
                    scope: None,
                },
                initializer: None, // TODO(vosa): Doesn't currently work
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
                    data_type: helper::create_function_pointer(
                        method.get_name().into(),
                        method.get_location().into(),
                    ),
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
        }
    }

    /// Creates a global virtual table instance for the given POU. These instances are used when creating a
    /// POU instance, such that the initializer of the POU will call `self.__vtable := ADR(<instance>)`
    fn generate_global_vtable_instance(&mut self, pou: &Pou, vtable: &UserTypeDeclaration) -> Variable {
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

    // XXX: fuck me, a `parse(<input>)` would make life so much easier here, the whole method would be a simple `parse("ADR(foo.bar)")` essentially
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
        debug_assert!(!names.is_empty() && names.len() <= 2); // It's either a `<pou>` or `<pou>.<method>`, where the latter case represents the body of a FB

        // XXX: Before merging check if the body methods of function blocks are renamed to `__body`, in which
        //      case `ADR(<POU>)` no longer needs to be supported simplfiying this code; see debug_assert asweel
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

            // In theory unreachable?
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

// TODO: Before merging try to make these tests more concise by not abusing snapshot testing here.
//       Specifically some tests here really don't need the whole snapshot data but rather only parts of it
//       such as the method names part of a virtual table struct. I think for the first few tests we want to
//       have snapshots such that the reader understands the structure of things and for the remaining we then
//       can switch to a more concise format.
#[cfg(test)]
mod tests {
    use plc_ast::provider::IdProvider;

    use crate::{lowering::vtable::VirtualTableGenerator, test_utils::tests::index_with_ids};

    #[test]
    fn vtable_member_variable_is_inject() {
        let ids = IdProvider::default();
        let (unit, index) = index_with_ids(
            r"
            FUNCTION_BLOCK FbA
                VAR
                    stateA: DINT;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
                VAR
                    stateB: DINT;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbC EXTENDS FbB
                VAR
                    stateC: DINT;
                END_VAR
            END_FUNCTION_BLOCK

            CLASS ClassA
                VAR
                    stateA: DINT;
                END_VAR
            END_CLASS

            CLASS ClassB EXTENDS ClassA
                VAR
                    stateB: DINT;
                END_VAR
            END_CLASS

            CLASS ClassC EXTENDS ClassB
                VAR
                    stateC: DINT;
                END_VAR
            END_CLASS
            ",
            ids.clone(),
        );

        let mut units = vec![unit];
        let mut generator = VirtualTableGenerator::new(ids);
        generator.generate(&index, &mut units);

        // XXX: Could probably be improved by finding a local variable block instead of creating a new one?
        // Only the "root" POUs should receive a `__vtable` member field
        insta::assert_debug_snapshot!(units[0].pous, @r#"
        [
            POU {
                name: "FbA",
                variable_blocks: [
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
                                name: "stateA",
                                data_type: DataTypeReference {
                                    referenced_type: "DINT",
                                },
                            },
                        ],
                        variable_block_type: Local,
                    },
                ],
                pou_type: FunctionBlock,
                return_type: None,
                interfaces: [],
                properties: [],
            },
            POU {
                name: "FbB",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "stateB",
                                data_type: DataTypeReference {
                                    referenced_type: "DINT",
                                },
                            },
                        ],
                        variable_block_type: Local,
                    },
                ],
                pou_type: FunctionBlock,
                return_type: None,
                interfaces: [],
                properties: [],
            },
            POU {
                name: "FbC",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "stateC",
                                data_type: DataTypeReference {
                                    referenced_type: "DINT",
                                },
                            },
                        ],
                        variable_block_type: Local,
                    },
                ],
                pou_type: FunctionBlock,
                return_type: None,
                interfaces: [],
                properties: [],
            },
            POU {
                name: "ClassA",
                variable_blocks: [
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
                                name: "stateA",
                                data_type: DataTypeReference {
                                    referenced_type: "DINT",
                                },
                            },
                        ],
                        variable_block_type: Local,
                    },
                ],
                pou_type: Class,
                return_type: None,
                interfaces: [],
                properties: [],
            },
            POU {
                name: "ClassB",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "stateB",
                                data_type: DataTypeReference {
                                    referenced_type: "DINT",
                                },
                            },
                        ],
                        variable_block_type: Local,
                    },
                ],
                pou_type: Class,
                return_type: None,
                interfaces: [],
                properties: [],
            },
            POU {
                name: "ClassC",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "stateC",
                                data_type: DataTypeReference {
                                    referenced_type: "DINT",
                                },
                            },
                        ],
                        variable_block_type: Local,
                    },
                ],
                pou_type: Class,
                return_type: None,
                interfaces: [],
                properties: [],
            },
        ]
        "#);
    }

    #[test]
    fn virtual_table_definition_and_instances_are_created_for_every_class_or_function_block() {
        let ids = IdProvider::default();
        let (unit, index) = index_with_ids(
            r"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK

            CLASS ClassA
            END_CLASS
            
            CLASS ClassB EXTENDS ClassA
            END_CLASS
            ",
            ids.clone(),
        );

        let mut units = vec![unit];
        let mut generator = VirtualTableGenerator::new(ids);
        generator.generate(&index, &mut units);

        // XXX: Could probably be improved by finding a global variable block instead of creating a new one?
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
                        name: "__vtable_ClassA_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_ClassA",
                        },
                    },
                    Variable {
                        name: "__vtable_ClassB_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_ClassB",
                        },
                    },
                ],
                variable_block_type: Global,
            },
        ]
        "#);

        // Because classes are not callable unlike function blocks, e.g. MyFb(), the virtual table will not
        // contain any function pointers if no methods are declared
        insta::assert_debug_snapshot!(units[0].user_types, @r#"
        [
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
                        },
                    ],
                },
                initializer: None,
                scope: None,
            },
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
                        },
                    ],
                },
                initializer: None,
                scope: None,
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_ClassA",
                    ),
                    variables: [],
                },
                initializer: None,
                scope: None,
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_ClassB",
                    ),
                    variables: [],
                },
                initializer: None,
                scope: None,
            },
        ]
        "#);
    }

    #[test]
    fn methods_are_part_of_virtual_tables_and_inherited_by_children() {
        let ids = IdProvider::default();
        let (unit, index) = index_with_ids(
            r"
            FUNCTION_BLOCK FbA
                METHOD methodOneInA
                END_METHOD

                METHOD methodTwoInA
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
                METHOD methodOneInB
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbC EXTENDS FbB
                METHOD methodOneInC
                END_METHOD
            END_FUNCTION_BLOCK

            CLASS ClassA
                METHOD methodOneInA
                END_METHOD

                METHOD methodTwoInA
                END_METHOD
            END_CLASS
            
            CLASS ClassB EXTENDS ClassA
                METHOD methodOneInB
                END_METHOD
            END_CLASS

            CLASS ClassC EXTENDS ClassB
                METHOD methodOneInC
                END_METHOD
            END_CLASS
            ",
            ids.clone(),
        );

        let mut units = vec![unit];
        let mut generator = VirtualTableGenerator::new(ids);
        generator.generate(&index, &mut units);

        // XXX: Could probably be improved by finding a global variable block instead of creating a new one?
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
                        name: "__vtable_FbC_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_FbC",
                        },
                    },
                    Variable {
                        name: "__vtable_ClassA_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_ClassA",
                        },
                    },
                    Variable {
                        name: "__vtable_ClassB_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_ClassB",
                        },
                    },
                    Variable {
                        name: "__vtable_ClassC_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_ClassC",
                        },
                    },
                ],
                variable_block_type: Global,
            },
        ]
        "#);

        insta::assert_debug_snapshot!(units[0].user_types, @r#"
        [
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
                        },
                        Variable {
                            name: "methodOneInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbA.methodOneInA",
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
                                                    name: "methodOneInA",
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
                            name: "methodTwoInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbA.methodTwoInA",
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
                                                    name: "methodTwoInA",
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
            },
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
                        },
                        Variable {
                            name: "methodOneInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbA.methodOneInA",
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
                                                    name: "methodOneInA",
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
                            name: "methodTwoInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbA.methodTwoInA",
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
                                                    name: "methodTwoInA",
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
                            name: "methodOneInB",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbB.methodOneInB",
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
                                                    name: "methodOneInB",
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
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_FbC",
                    ),
                    variables: [
                        Variable {
                            name: "__body",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbC",
                                    },
                                    auto_deref: None,
                                    type_safe: false,
                                    is_function: true,
                                },
                            },
                        },
                        Variable {
                            name: "methodOneInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbA.methodOneInA",
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
                                                    name: "methodOneInA",
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
                            name: "methodTwoInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbA.methodTwoInA",
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
                                                    name: "methodTwoInA",
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
                            name: "methodOneInB",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbB.methodOneInB",
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
                                                    name: "methodOneInB",
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
                        Variable {
                            name: "methodOneInC",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbC.methodOneInC",
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
                                                    name: "methodOneInC",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "FbC",
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
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_ClassA",
                    ),
                    variables: [
                        Variable {
                            name: "methodOneInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassA.methodOneInA",
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
                                                    name: "methodOneInA",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassA",
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
                            name: "methodTwoInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassA.methodTwoInA",
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
                                                    name: "methodTwoInA",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassA",
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
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_ClassB",
                    ),
                    variables: [
                        Variable {
                            name: "methodOneInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassA.methodOneInA",
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
                                                    name: "methodOneInA",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassA",
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
                            name: "methodTwoInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassA.methodTwoInA",
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
                                                    name: "methodTwoInA",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassA",
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
                            name: "methodOneInB",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassB.methodOneInB",
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
                                                    name: "methodOneInB",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassB",
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
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_ClassC",
                    ),
                    variables: [
                        Variable {
                            name: "methodOneInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassA.methodOneInA",
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
                                                    name: "methodOneInA",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassA",
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
                            name: "methodTwoInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassA.methodTwoInA",
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
                                                    name: "methodTwoInA",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassA",
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
                            name: "methodOneInB",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassB.methodOneInB",
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
                                                    name: "methodOneInB",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassB",
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
                            name: "methodOneInC",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassC.methodOneInC",
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
                                                    name: "methodOneInC",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassC",
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
            },
        ]
        "#);
    }

    #[test]
    fn overridden_methods_point_to_child_pou_rather_than_parent() {
        let ids = IdProvider::default();
        let (unit, index) = index_with_ids(
            r"
            FUNCTION_BLOCK FbA
                METHOD methodOneInA
                END_METHOD

                METHOD methodTwoInA
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
                // Overridden, should point to FbB.methodOneInA
                METHOD methodOneInA
                END_METHOD

                METHOD methodOneInB
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbC EXTENDS FbB
                // Overridden, should point to FbC.methodOneInB
                METHOD methodOneInB
                END_METHOD

                METHOD methodOneInC
                END_METHOD
            END_FUNCTION_BLOCK

            CLASS ClassA
                METHOD methodOneInA
                END_METHOD

                METHOD methodTwoInA
                END_METHOD
            END_CLASS

            CLASS ClassB EXTENDS ClassA
                // Overridden, should point to ClassB.methodOneInA
                METHOD methodOneInA
                END_METHOD

                METHOD methodOneInB
                END_METHOD
            END_CLASS

            CLASS ClassC EXTENDS ClassB
                // Overridden, should point to ClassC.methodOneInB
                METHOD methodOneInB
                END_METHOD

                METHOD methodOneInC
                END_METHOD
            END_CLASS
            ",
            ids.clone(),
        );

        let mut units = vec![unit];
        let mut generator = VirtualTableGenerator::new(ids);
        generator.generate(&index, &mut units);

        // XXX: Could probably be improved by finding a global variable block instead of creating a new one?
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
                        name: "__vtable_FbC_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_FbC",
                        },
                    },
                    Variable {
                        name: "__vtable_ClassA_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_ClassA",
                        },
                    },
                    Variable {
                        name: "__vtable_ClassB_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_ClassB",
                        },
                    },
                    Variable {
                        name: "__vtable_ClassC_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_ClassC",
                        },
                    },
                ],
                variable_block_type: Global,
            },
        ]
        "#);

        insta::assert_debug_snapshot!(units[0].user_types, @r#"
        [
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
                        },
                        Variable {
                            name: "methodOneInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbA.methodOneInA",
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
                                                    name: "methodOneInA",
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
                            name: "methodTwoInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbA.methodTwoInA",
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
                                                    name: "methodTwoInA",
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
            },
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
                        },
                        Variable {
                            name: "methodOneInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbB.methodOneInA",
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
                                                    name: "methodOneInA",
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
                        Variable {
                            name: "methodTwoInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbA.methodTwoInA",
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
                                                    name: "methodTwoInA",
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
                            name: "methodOneInB",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbB.methodOneInB",
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
                                                    name: "methodOneInB",
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
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_FbC",
                    ),
                    variables: [
                        Variable {
                            name: "__body",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbC",
                                    },
                                    auto_deref: None,
                                    type_safe: false,
                                    is_function: true,
                                },
                            },
                        },
                        Variable {
                            name: "methodOneInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbB.methodOneInA",
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
                                                    name: "methodOneInA",
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
                        Variable {
                            name: "methodTwoInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbA.methodTwoInA",
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
                                                    name: "methodTwoInA",
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
                            name: "methodOneInB",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbC.methodOneInB",
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
                                                    name: "methodOneInB",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "FbC",
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
                            name: "methodOneInC",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbC.methodOneInC",
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
                                                    name: "methodOneInC",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "FbC",
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
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_ClassA",
                    ),
                    variables: [
                        Variable {
                            name: "methodOneInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassA.methodOneInA",
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
                                                    name: "methodOneInA",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassA",
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
                            name: "methodTwoInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassA.methodTwoInA",
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
                                                    name: "methodTwoInA",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassA",
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
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_ClassB",
                    ),
                    variables: [
                        Variable {
                            name: "methodOneInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassB.methodOneInA",
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
                                                    name: "methodOneInA",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassB",
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
                            name: "methodTwoInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassA.methodTwoInA",
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
                                                    name: "methodTwoInA",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassA",
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
                            name: "methodOneInB",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassB.methodOneInB",
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
                                                    name: "methodOneInB",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassB",
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
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_ClassC",
                    ),
                    variables: [
                        Variable {
                            name: "methodOneInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassB.methodOneInA",
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
                                                    name: "methodOneInA",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassB",
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
                            name: "methodTwoInA",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassA.methodTwoInA",
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
                                                    name: "methodTwoInA",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassA",
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
                            name: "methodOneInB",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassC.methodOneInB",
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
                                                    name: "methodOneInB",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassC",
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
                            name: "methodOneInC",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassC.methodOneInC",
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
                                                    name: "methodOneInC",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassC",
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
            },
        ]
        "#);
    }

    #[test]
    fn virtual_table_method_order_is_untouched_in_extended_virtual_tables() {
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

        // XXX: Could probably be improved by finding a global variable block instead of creating a new one?
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
                        name: "__vtable_FbC_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_FbC",
                        },
                    },
                    Variable {
                        name: "__vtable_ClassA_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_ClassA",
                        },
                    },
                    Variable {
                        name: "__vtable_ClassB_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_ClassB",
                        },
                    },
                    Variable {
                        name: "__vtable_ClassC_instance",
                        data_type: DataTypeReference {
                            referenced_type: "__vtable_ClassC",
                        },
                    },
                ],
                variable_block_type: Global,
            },
        ]
        "#);

        insta::assert_debug_snapshot!(units[0].user_types, @r#"
        [
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
                        },
                        Variable {
                            name: "one",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbA.one",
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
                                                    name: "one",
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
                            name: "two",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbA.two",
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
                                                    name: "two",
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
                            name: "three",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbA.three",
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
                                                    name: "three",
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
            },
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
                        },
                        Variable {
                            name: "one",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbB.one",
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
                                                    name: "one",
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
                        Variable {
                            name: "two",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbB.two",
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
                                                    name: "two",
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
                        Variable {
                            name: "three",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbA.three",
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
                                                    name: "three",
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
                            name: "four",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbB.four",
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
                                                    name: "four",
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
                        Variable {
                            name: "five",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbB.five",
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
                                                    name: "five",
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
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_FbC",
                    ),
                    variables: [
                        Variable {
                            name: "__body",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbC",
                                    },
                                    auto_deref: None,
                                    type_safe: false,
                                    is_function: true,
                                },
                            },
                        },
                        Variable {
                            name: "one",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbB.one",
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
                                                    name: "one",
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
                        Variable {
                            name: "two",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbB.two",
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
                                                    name: "two",
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
                        Variable {
                            name: "three",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbA.three",
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
                                                    name: "three",
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
                            name: "four",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbB.four",
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
                                                    name: "four",
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
                        Variable {
                            name: "five",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbC.five",
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
                                                    name: "five",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "FbC",
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
                            name: "six",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "FbC.six",
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
                                                    name: "six",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "FbC",
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
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_ClassA",
                    ),
                    variables: [
                        Variable {
                            name: "one",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassA.one",
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
                                                    name: "one",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassA",
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
                            name: "two",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassA.two",
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
                                                    name: "two",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassA",
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
                            name: "three",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassA.three",
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
                                                    name: "three",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassA",
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
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_ClassB",
                    ),
                    variables: [
                        Variable {
                            name: "one",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassB.one",
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
                                                    name: "one",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassB",
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
                            name: "two",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassB.two",
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
                                                    name: "two",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassB",
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
                            name: "three",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassA.three",
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
                                                    name: "three",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassA",
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
                            name: "four",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassB.four",
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
                                                    name: "four",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassB",
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
                            name: "five",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassB.five",
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
                                                    name: "five",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassB",
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
            },
            UserTypeDeclaration {
                data_type: StructType {
                    name: Some(
                        "__vtable_ClassC",
                    ),
                    variables: [
                        Variable {
                            name: "one",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassB.one",
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
                                                    name: "one",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassB",
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
                            name: "two",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassB.two",
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
                                                    name: "two",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassB",
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
                            name: "three",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassA.three",
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
                                                    name: "three",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassA",
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
                            name: "four",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassB.four",
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
                                                    name: "four",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassB",
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
                            name: "five",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassC.five",
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
                                                    name: "five",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassC",
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
                            name: "six",
                            data_type: DataTypeDefinition {
                                data_type: PointerType {
                                    name: None,
                                    referenced_type: DataTypeReference {
                                        referenced_type: "ClassC.six",
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
                                                    name: "six",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "ClassC",
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
            },
        ]
        "#);
    }
}
