//! Module generating VTable type-definitions and their global instances.
//!
//! TODO: Documentation

use plc_ast::{
    ast::{
        AstFactory, AstNode, CompilationUnit, DataType, DataTypeDeclaration, Pou, UserTypeDeclaration,
        Variable, VariableBlock,
    },
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

use crate::index::Index;

pub struct VTableGenerator {
    pub id_provider: IdProvider,
}

impl VTableGenerator {
    pub fn new(id_provider: IdProvider) -> VTableGenerator {
        VTableGenerator { id_provider }
    }

    pub fn generate_vtables(&mut self, index: &Index, units: &mut Vec<CompilationUnit>) {
        for unit in units {
            let mut definitions = Vec::new();
            let mut instances = Vec::new();

            for pou in unit.pous.iter().filter(|pou| pou.kind.is_class() | pou.kind.is_function_block()) {
                let definition = self.generate_vtable_definitions(index, pou);
                let instance = self.generate_vtable_instance(pou, &definition);

                definitions.push(definition);
                instances.push(instance);
            }

            unit.user_types.extend(definitions);
            unit.global_vars.push(VariableBlock::global().with_variables(instances));
        }
    }

    /// Creates and returns a struct type definition representing the VTable of the given POU, containing all methods as
    /// function pointers.
    fn generate_vtable_definitions(&mut self, index: &Index, pou: &Pou) -> UserTypeDeclaration {
        debug_assert!(pou.kind.is_class() | pou.kind.is_function_block());

        let mut variables = Vec::new();
        let file_location = pou.location.get_file_name();

        if pou.kind.is_function_block() {
            let variable = Variable {
                name: String::from("__body"),
                data_type_declaration: DataTypeDeclaration::Definition {
                    data_type: DataType::PointerType {
                        name: None,
                        referenced_type: Box::new(DataTypeDeclaration::Reference {
                            referenced_type: pou.name.to_string(),
                            location: pou.location.clone(),
                        }),
                        auto_deref: None, // XXX(vosa): Could be autoderef, simplifiying the lowering code?
                        type_safe: false, // XXX(vosa): Explain why not type safe, mostly because of overridden methods
                    },
                    location: SourceLocation::internal(),
                    scope: None,
                },
                initializer: None, //Some(self.generate_initalizer(&pou.name)),
                address: None,
                location: SourceLocation::internal(),
            };

            variables.push(variable);
        }

        for method in index.get_methods(&pou.name) {
            let pointer = DataType::PointerType {
                name: None,
                referenced_type: Box::new(DataTypeDeclaration::reference(
                    method.get_name(),
                    method.get_location(),
                )),
                auto_deref: None, // XXX(vosa): Could be autoderef, simplyifing the lowering code?
                type_safe: false,
            };

            let variable = Variable {
                name: method.get_flat_reference_name().to_string(),
                data_type_declaration: DataTypeDeclaration::Definition {
                    data_type: pointer,
                    location: SourceLocation::internal_in_unit(file_location),
                    scope: None,
                },
                initializer: Some(self.generate_initalizer(method.get_name())),
                address: None,
                location: SourceLocation::internal_in_unit(file_location),
            };

            variables.push(variable);
        }

        UserTypeDeclaration {
            data_type: DataType::StructType { name: Some(format!("__vtable_{}", &pou.name)), variables },
            initializer: None,
            location: SourceLocation::internal_in_unit(file_location),
            scope: None,
        }
    }

    fn generate_vtable_instance(&mut self, pou: &Pou, vtable: &UserTypeDeclaration) -> Variable {
        Variable {
            name: format!("{}_instance", vtable.data_type.get_name().unwrap()),
            data_type_declaration: DataTypeDeclaration::reference(
                vtable.data_type.get_name().unwrap(),
                &vtable.location,
            ),
            initializer: None,
            address: None,
            location: SourceLocation::internal_in_unit(pou.location.get_file_name()),
        }
    }

    // XXX: fuck me, a `parse(<input>)` would make life so much easier here, the whole method would be a simple `parse("ADR(foo.bar)")` essentially
    /// Creates a call statement of form `ADR(<qualified name of method>)`
    fn generate_initalizer(&mut self, qualified_name: &str) -> AstNode {
        // ADR(<pou>.<method>)
        // ^^^
        let operator = AstFactory::create_member_reference(
            AstFactory::create_identifier("ADR", SourceLocation::internal(), self.id_provider.next_id()),
            None,
            self.id_provider.next_id(),
        );

        // ADR(<pou>.<method>)
        //     ^^^^^^^^^^^^^^
        let names = qualified_name.split('.').collect::<Vec<_>>();
        debug_assert!(!names.is_empty() && names.len() <= 2); // It's either a `<pou>` or `<pou>.<method>`, where the latter case represents the body of a FB

        let argument = match (names.first(), names.get(1)) {
            // ADR(<pou>)
            (Some(name_pou), None) => AstFactory::create_member_reference(
                AstFactory::create_identifier(
                    *name_pou,
                    SourceLocation::internal(),
                    self.id_provider.next_id(),
                ),
                None,
                self.id_provider.next_id(),
            ),

            // ADR(<pou>.<method>)
            (Some(name_pou), Some(name_method)) => AstFactory::create_member_reference(
                AstFactory::create_identifier(
                    *name_method,
                    SourceLocation::internal(),
                    self.id_provider.next_id(),
                ),
                Some(AstFactory::create_member_reference(
                    AstFactory::create_identifier(
                        *name_pou,
                        SourceLocation::internal(),
                        self.id_provider.next_id(),
                    ),
                    None,
                    self.id_provider.next_id(),
                )),
                self.id_provider.next_id(),
            ),

            // In theory unreachable?
            _ => unreachable!(),
        };

        // ADR(<pou>.<method>)
        // ^^^^^^^^^^^^^^^^^^^
        AstFactory::create_call_statement(
            operator,
            Some(argument),
            self.id_provider.next_id(),
            SourceLocation::internal(),
        )
    }
}

#[cfg(test)]
mod tests {
    use plc_ast::provider::IdProvider;

    use crate::{test_utils::tests::index_with_ids, vtable::VTableGenerator};

    #[test]
    fn empty_pou_has_body_function_pointer() {
        let ids = IdProvider::default();
        let (unit, index) = index_with_ids(
            r"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK
            ",
            ids.clone(),
        );

        let mut units = vec![unit];
        let mut generator = VTableGenerator::new(ids);
        generator.generate_vtables(&index, &mut units);

        insta::assert_debug_snapshot!(units, @r#"
        [
            CompilationUnit {
                global_vars: [
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
                        ],
                        variable_block_type: Global,
                    },
                ],
                var_config: [],
                pous: [
                    POU {
                        name: "FbA",
                        variable_blocks: [
                            VariableBlock {
                                variables: [
                                    Variable {
                                        name: "__vtable",
                                        data_type: DataTypeReference {
                                            referenced_type: "__VOID_POINTER",
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
                ],
                implementations: [
                    Implementation {
                        name: "FbA",
                        type_name: "FbA",
                        linkage: Internal,
                        pou_type: FunctionBlock,
                        statements: [],
                        location: SourceLocation {
                            span: Range(2:12 - 1:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(1:27 - 1:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(2:12 - 2:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: None,
                    },
                ],
                interfaces: [],
                user_types: [
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
                                        },
                                    },
                                },
                            ],
                        },
                        initializer: None,
                        scope: None,
                    },
                ],
                file: File(
                    "<internal>",
                ),
            },
        ]
        "#);
    }

    #[test]
    fn simple() {
        let ids = IdProvider::default();
        let (unit, index) = index_with_ids(
            r"
            FUNCTION_BLOCK FbA
                METHOD foo
                END_METHOD
                
                METHOD bar: DINT
                    VAR_INPUT
                        in: STRING;
                    END_VAR
                END_METHOD
            END_FUNCTION_BLOCK
            ",
            ids.clone(),
        );

        let mut units = vec![unit];
        let mut generator = VTableGenerator::new(ids);
        generator.generate_vtables(&index, &mut units);

        insta::assert_debug_snapshot!(units, @r#"
        [
            CompilationUnit {
                global_vars: [
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
                        ],
                        variable_block_type: Global,
                    },
                ],
                var_config: [],
                pous: [
                    POU {
                        name: "FbA",
                        variable_blocks: [
                            VariableBlock {
                                variables: [
                                    Variable {
                                        name: "__vtable",
                                        data_type: DataTypeReference {
                                            referenced_type: "__VOID_POINTER",
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
                        name: "FbA.foo",
                        variable_blocks: [],
                        pou_type: Method {
                            parent: "FbA",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        return_type: None,
                        interfaces: [],
                        properties: [],
                    },
                    POU {
                        name: "FbA.bar",
                        variable_blocks: [
                            VariableBlock {
                                variables: [
                                    Variable {
                                        name: "in",
                                        data_type: DataTypeReference {
                                            referenced_type: "STRING",
                                        },
                                    },
                                ],
                                variable_block_type: Input(
                                    ByVal,
                                ),
                            },
                        ],
                        pou_type: Method {
                            parent: "FbA",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        return_type: Some(
                            DataTypeReference {
                                referenced_type: "DINT",
                            },
                        ),
                        interfaces: [],
                        properties: [],
                    },
                ],
                implementations: [
                    Implementation {
                        name: "FbA.foo",
                        type_name: "FbA.foo",
                        linkage: Internal,
                        pou_type: Method {
                            parent: "FbA",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        statements: [],
                        location: SourceLocation {
                            span: Range(3:16 - 2:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(2:23 - 2:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(3:16 - 3:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: Some(
                            Protected,
                        ),
                    },
                    Implementation {
                        name: "FbA.bar",
                        type_name: "FbA.bar",
                        linkage: Internal,
                        pou_type: Method {
                            parent: "FbA",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        statements: [],
                        location: SourceLocation {
                            span: Range(9:16 - 8:27),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(5:23 - 5:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(9:16 - 9:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: Some(
                            Protected,
                        ),
                    },
                    Implementation {
                        name: "FbA",
                        type_name: "FbA",
                        linkage: Internal,
                        pou_type: FunctionBlock,
                        statements: [],
                        location: SourceLocation {
                            span: Range(10:12 - 9:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(1:27 - 1:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(10:12 - 10:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: None,
                    },
                ],
                interfaces: [],
                user_types: [
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
                                        },
                                    },
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
                                                referenced_type: "FbA.bar",
                                            },
                                            auto_deref: None,
                                            type_safe: false,
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
                ],
                file: File(
                    "<internal>",
                ),
            },
        ]
        "#);
    }

    #[test]
    fn inheritance() {
        let ids = IdProvider::default();
        let (unit, index) = index_with_ids(
            r"
            FUNCTION_BLOCK FbA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
                METHOD bar
                END_METHOD
            END_FUNCTION_BLOCK
            ",
            ids.clone(),
        );

        let mut units = vec![unit];
        let mut generator = VTableGenerator::new(ids);
        generator.generate_vtables(&index, &mut units);

        insta::assert_debug_snapshot!(units, @r#"
        [
            CompilationUnit {
                global_vars: [
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
                        ],
                        variable_block_type: Global,
                    },
                ],
                var_config: [],
                pous: [
                    POU {
                        name: "FbA",
                        variable_blocks: [
                            VariableBlock {
                                variables: [
                                    Variable {
                                        name: "__vtable",
                                        data_type: DataTypeReference {
                                            referenced_type: "__VOID_POINTER",
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
                        name: "FbA.foo",
                        variable_blocks: [],
                        pou_type: Method {
                            parent: "FbA",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        return_type: None,
                        interfaces: [],
                        properties: [],
                    },
                    POU {
                        name: "FbB",
                        variable_blocks: [],
                        pou_type: FunctionBlock,
                        return_type: None,
                        interfaces: [],
                        properties: [],
                    },
                    POU {
                        name: "FbB.bar",
                        variable_blocks: [],
                        pou_type: Method {
                            parent: "FbB",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        return_type: None,
                        interfaces: [],
                        properties: [],
                    },
                ],
                implementations: [
                    Implementation {
                        name: "FbA.foo",
                        type_name: "FbA.foo",
                        linkage: Internal,
                        pou_type: Method {
                            parent: "FbA",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        statements: [],
                        location: SourceLocation {
                            span: Range(3:16 - 2:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(2:23 - 2:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(3:16 - 3:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: Some(
                            Protected,
                        ),
                    },
                    Implementation {
                        name: "FbA",
                        type_name: "FbA",
                        linkage: Internal,
                        pou_type: FunctionBlock,
                        statements: [],
                        location: SourceLocation {
                            span: Range(4:12 - 3:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(1:27 - 1:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(4:12 - 4:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: None,
                    },
                    Implementation {
                        name: "FbB.bar",
                        type_name: "FbB.bar",
                        linkage: Internal,
                        pou_type: Method {
                            parent: "FbB",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        statements: [],
                        location: SourceLocation {
                            span: Range(8:16 - 7:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(7:23 - 7:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(8:16 - 8:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: Some(
                            Protected,
                        ),
                    },
                    Implementation {
                        name: "FbB",
                        type_name: "FbB",
                        linkage: Internal,
                        pou_type: FunctionBlock,
                        statements: [],
                        location: SourceLocation {
                            span: Range(9:12 - 8:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(6:27 - 6:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(9:12 - 9:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: None,
                    },
                ],
                interfaces: [],
                user_types: [
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
                                        },
                                    },
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
                                        },
                                    },
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
                    },
                ],
                file: File(
                    "<internal>",
                ),
            },
        ]
        "#);
    }

    #[test]
    fn inheritance_chain() {
        let ids = IdProvider::default();
        let (unit, index) = index_with_ids(
            r"
            FUNCTION_BLOCK FbA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
                METHOD bar
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbC EXTENDS FbB
                METHOD baz
                END_METHOD
            END_FUNCTION_BLOCK
            ",
            ids.clone(),
        );

        let mut units = vec![unit];
        let mut generator = VTableGenerator::new(ids);
        generator.generate_vtables(&index, &mut units);

        insta::assert_debug_snapshot!(units, @r#"
        [
            CompilationUnit {
                global_vars: [
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
                        ],
                        variable_block_type: Global,
                    },
                ],
                var_config: [],
                pous: [
                    POU {
                        name: "FbA",
                        variable_blocks: [
                            VariableBlock {
                                variables: [
                                    Variable {
                                        name: "__vtable",
                                        data_type: DataTypeReference {
                                            referenced_type: "__VOID_POINTER",
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
                        name: "FbA.foo",
                        variable_blocks: [],
                        pou_type: Method {
                            parent: "FbA",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        return_type: None,
                        interfaces: [],
                        properties: [],
                    },
                    POU {
                        name: "FbB",
                        variable_blocks: [],
                        pou_type: FunctionBlock,
                        return_type: None,
                        interfaces: [],
                        properties: [],
                    },
                    POU {
                        name: "FbB.bar",
                        variable_blocks: [],
                        pou_type: Method {
                            parent: "FbB",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        return_type: None,
                        interfaces: [],
                        properties: [],
                    },
                    POU {
                        name: "FbC",
                        variable_blocks: [],
                        pou_type: FunctionBlock,
                        return_type: None,
                        interfaces: [],
                        properties: [],
                    },
                    POU {
                        name: "FbC.baz",
                        variable_blocks: [],
                        pou_type: Method {
                            parent: "FbC",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        return_type: None,
                        interfaces: [],
                        properties: [],
                    },
                ],
                implementations: [
                    Implementation {
                        name: "FbA.foo",
                        type_name: "FbA.foo",
                        linkage: Internal,
                        pou_type: Method {
                            parent: "FbA",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        statements: [],
                        location: SourceLocation {
                            span: Range(3:16 - 2:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(2:23 - 2:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(3:16 - 3:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: Some(
                            Protected,
                        ),
                    },
                    Implementation {
                        name: "FbA",
                        type_name: "FbA",
                        linkage: Internal,
                        pou_type: FunctionBlock,
                        statements: [],
                        location: SourceLocation {
                            span: Range(4:12 - 3:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(1:27 - 1:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(4:12 - 4:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: None,
                    },
                    Implementation {
                        name: "FbB.bar",
                        type_name: "FbB.bar",
                        linkage: Internal,
                        pou_type: Method {
                            parent: "FbB",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        statements: [],
                        location: SourceLocation {
                            span: Range(8:16 - 7:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(7:23 - 7:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(8:16 - 8:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: Some(
                            Protected,
                        ),
                    },
                    Implementation {
                        name: "FbB",
                        type_name: "FbB",
                        linkage: Internal,
                        pou_type: FunctionBlock,
                        statements: [],
                        location: SourceLocation {
                            span: Range(9:12 - 8:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(6:27 - 6:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(9:12 - 9:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: None,
                    },
                    Implementation {
                        name: "FbC.baz",
                        type_name: "FbC.baz",
                        linkage: Internal,
                        pou_type: Method {
                            parent: "FbC",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        statements: [],
                        location: SourceLocation {
                            span: Range(13:16 - 12:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(12:23 - 12:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(13:16 - 13:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: Some(
                            Protected,
                        ),
                    },
                    Implementation {
                        name: "FbC",
                        type_name: "FbC",
                        linkage: Internal,
                        pou_type: FunctionBlock,
                        statements: [],
                        location: SourceLocation {
                            span: Range(14:12 - 13:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(11:27 - 11:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(14:12 - 14:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: None,
                    },
                ],
                interfaces: [],
                user_types: [
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
                                        },
                                    },
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
                                        },
                                    },
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
                                        },
                                    },
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
                                Variable {
                                    name: "baz",
                                    data_type: DataTypeDefinition {
                                        data_type: PointerType {
                                            name: None,
                                            referenced_type: DataTypeReference {
                                                referenced_type: "FbC.baz",
                                            },
                                            auto_deref: None,
                                            type_safe: false,
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
                                                            name: "baz",
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
                ],
                file: File(
                    "<internal>",
                ),
            },
        ]
        "#);
    }

    #[test]
    fn inheritance_overridden() {
        let ids = IdProvider::default();
        let (unit, index) = index_with_ids(
            r"
            FUNCTION_BLOCK FbA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
                METHOD foo
                    // Overridden by FbB, the VTable must point at this method rather than FbA::foo
                END_METHOD
            END_FUNCTION_BLOCK
            ",
            ids.clone(),
        );

        let mut units = vec![unit];
        let mut generator = VTableGenerator::new(ids);
        generator.generate_vtables(&index, &mut units);

        insta::assert_debug_snapshot!(units, @r#"
        [
            CompilationUnit {
                global_vars: [
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
                        ],
                        variable_block_type: Global,
                    },
                ],
                var_config: [],
                pous: [
                    POU {
                        name: "FbA",
                        variable_blocks: [
                            VariableBlock {
                                variables: [
                                    Variable {
                                        name: "__vtable",
                                        data_type: DataTypeReference {
                                            referenced_type: "__VOID_POINTER",
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
                        name: "FbA.foo",
                        variable_blocks: [],
                        pou_type: Method {
                            parent: "FbA",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        return_type: None,
                        interfaces: [],
                        properties: [],
                    },
                    POU {
                        name: "FbB",
                        variable_blocks: [],
                        pou_type: FunctionBlock,
                        return_type: None,
                        interfaces: [],
                        properties: [],
                    },
                    POU {
                        name: "FbB.foo",
                        variable_blocks: [],
                        pou_type: Method {
                            parent: "FbB",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        return_type: None,
                        interfaces: [],
                        properties: [],
                    },
                ],
                implementations: [
                    Implementation {
                        name: "FbA.foo",
                        type_name: "FbA.foo",
                        linkage: Internal,
                        pou_type: Method {
                            parent: "FbA",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        statements: [],
                        location: SourceLocation {
                            span: Range(3:16 - 2:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(2:23 - 2:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(3:16 - 3:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: Some(
                            Protected,
                        ),
                    },
                    Implementation {
                        name: "FbA",
                        type_name: "FbA",
                        linkage: Internal,
                        pou_type: FunctionBlock,
                        statements: [],
                        location: SourceLocation {
                            span: Range(4:12 - 3:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(1:27 - 1:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(4:12 - 4:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: None,
                    },
                    Implementation {
                        name: "FbB.foo",
                        type_name: "FbB.foo",
                        linkage: Internal,
                        pou_type: Method {
                            parent: "FbB",
                            property: None,
                            declaration_kind: Concrete,
                        },
                        statements: [],
                        location: SourceLocation {
                            span: Range(9:16 - 7:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(7:23 - 7:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(9:16 - 9:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: Some(
                            Protected,
                        ),
                    },
                    Implementation {
                        name: "FbB",
                        type_name: "FbB",
                        linkage: Internal,
                        pou_type: FunctionBlock,
                        statements: [],
                        location: SourceLocation {
                            span: Range(10:12 - 9:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(6:27 - 6:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(10:12 - 10:30),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: None,
                    },
                ],
                interfaces: [],
                user_types: [
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
                                        },
                                    },
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
                                        },
                                    },
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
                    },
                ],
                file: File(
                    "<internal>",
                ),
            },
        ]
        "#);
    }
}
