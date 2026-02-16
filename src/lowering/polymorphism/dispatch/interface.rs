use plc_ast::{
    ast::{CompilationUnit, DataTypeDeclaration},
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
};

use crate::{index::Index, typesystem::DataType};

const FATPOINTER_TYPE_NAME: &str = "__FATPOINTER";
const FATPOINTER_DATA_FIELD_NAME: &str = "data";
const FATPOINTER_TABLE_FIELD_NAME: &str = "table";

pub struct InterfaceDispatchLowerer<'a> {
    ids: IdProvider,
    index: &'a Index,

    /// Do we need to generate the `__FATPOINTER` struct definition?
    needs_fatpointer_definition: bool,
}

impl<'a> InterfaceDispatchLowerer<'a> {
    pub fn new(ids: IdProvider, index: &'a Index) -> Self {
        Self { ids, index, needs_fatpointer_definition: false }
    }

    pub fn lower(&mut self, units: &mut [CompilationUnit]) {
        for unit in &mut *units {
            self.visit_compilation_unit(unit);
        }

        if self.needs_fatpointer_definition {
            units[0].user_types.push(helper::create_fat_pointer_struct());
        }
    }
}

impl<'a> AstVisitorMut for InterfaceDispatchLowerer<'a> {
    /// Replace any datatype declaration that resolves to an interface type with a `__FATPOINTER`
    fn visit_data_type_declaration(&mut self, data_type_declaration: &mut DataTypeDeclaration) {
        if let DataTypeDeclaration::Reference { referenced_type, .. } = data_type_declaration {
            if self.index.find_effective_type_by_name(referenced_type).is_some_and(DataType::is_interface) {
                helper::replace_datatype_with_fatpointer(data_type_declaration);
                self.needs_fatpointer_definition = true;
            }
        }

        data_type_declaration.walk(self);
    }
}

mod helper {
    use plc_ast::ast::{DataType, DataTypeDeclaration, LinkageType, UserTypeDeclaration, Variable};
    use plc_source::source_location::SourceLocation;

    use crate::{
        lowering::polymorphism::dispatch::interface::{
            FATPOINTER_DATA_FIELD_NAME, FATPOINTER_TABLE_FIELD_NAME, FATPOINTER_TYPE_NAME,
        },
        typesystem::VOID_INTERNAL_NAME,
    };

    pub fn replace_datatype_with_fatpointer(type_decl: &mut DataTypeDeclaration) {
        *type_decl = DataTypeDeclaration::Reference {
            referenced_type: String::from(FATPOINTER_TYPE_NAME),
            location: type_decl.get_location(),
        };
    }

    pub fn create_fat_pointer_struct() -> UserTypeDeclaration {
        /// Creates a struct member of type `POINTER TO __VOID`.
        fn create_void_pointer_member(name: &str, location: &SourceLocation) -> Variable {
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

        let location = SourceLocation::internal();

        UserTypeDeclaration {
            data_type: DataType::StructType {
                name: Some(String::from(FATPOINTER_TYPE_NAME)),
                variables: vec![
                    create_void_pointer_member(FATPOINTER_DATA_FIELD_NAME, &location),
                    create_void_pointer_member(FATPOINTER_TABLE_FIELD_NAME, &location),
                ],
            },
            initializer: None,
            location,
            scope: None,
            linkage: LinkageType::Internal,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lowering::polymorphism::dispatch::interface::FATPOINTER_TYPE_NAME;

    #[test]
    fn fatpointer_is_generated_on_demand() {
        // Initially, no POU makes use of a interface as a variable
        {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA
                END_FUNCTION_BLOCK
            "#;

            let annotated_unit = helper::lower(source);
            let mut user_types = annotated_unit.get_unit().user_types.iter();
            assert!(!user_types.any(|ty| ty.data_type.get_name().unwrap() == FATPOINTER_TYPE_NAME));
        }

        // However, if the interface is used as a variable, a fat-pointer MUST be generated
        {
            let source = r#"
                INTERFACE IA
                END_INTERFACE

                FUNCTION_BLOCK FbA
                    VAR
                        localVariable: IA;
                    END_VAR
                END_FUNCTION_BLOCK
            "#;

            let annotated_unit = helper::lower(source);
            let mut user_types = annotated_unit.get_unit().user_types.iter();
            assert!(user_types.any(|ty| ty.data_type.get_name().unwrap() == FATPOINTER_TYPE_NAME));
        }
    }

    #[test]
    fn fatpointer_replaces_function_return_type() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION foo: IA
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(helper::lower(source).get_unit().pous, @r#"
        [
            POU {
                name: "foo",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "foo",
                                data_type: DataTypeReference {
                                    referenced_type: "__FATPOINTER",
                                },
                            },
                        ],
                        variable_block_type: InOut,
                    },
                ],
                pou_type: Function,
                return_type: Some(
                    Aggregate {
                        referenced_type: "__FATPOINTER",
                    },
                ),
                interfaces: [],
                properties: [],
            },
        ]
        "#);
    }

    #[test]
    fn fatpointer_replaces_interface_variable_type() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION main
                VAR
                    localVariable: IA;
                END_VAR

                VAR_INPUT
                    inVariable: IA;
                END_VAR

                VAR_OUTPUT
                    outVariable: IA;
                END_VAR

                VAR_IN_OUT
                    inOutVariable: IA;
                END_VAR
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(helper::lower(source).get_unit().pous[0].variable_blocks, @r#"
        [
            VariableBlock {
                variables: [
                    Variable {
                        name: "localVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Local,
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "inVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Input(
                    ByVal,
                ),
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "outVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Output,
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "inOutVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: InOut,
            },
        ]
        "#);
    }

    #[test]
    fn fatpointer_replaces_interface_variable_aliased_type() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            TYPE
                AliasedIA: IA;
            END_TYPE

            FUNCTION main
                VAR
                    localVariable: AliasedIA;
                END_VAR

                VAR_INPUT
                    inVariable: AliasedIA;
                END_VAR

                VAR_OUTPUT
                    outVariable: AliasedIA;
                END_VAR

                VAR_IN_OUT
                    inOutVariable: AliasedIA;
                END_VAR
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(helper::lower(source).get_unit().pous[0].variable_blocks, @r#"
        [
            VariableBlock {
                variables: [
                    Variable {
                        name: "localVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Local,
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "inVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Input(
                    ByVal,
                ),
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "outVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: Output,
            },
            VariableBlock {
                variables: [
                    Variable {
                        name: "inOutVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__FATPOINTER",
                        },
                    },
                ],
                variable_block_type: InOut,
            },
        ]
        "#);
    }

    #[test]
    fn fatpointer_replaces_interface_variable_array_type() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION main
                VAR
                    localVariable: ARRAY[1..2] OF IA;
                    localVariableNested: ARRAY[1..2] OF ARRAY[3..4] OF IA;
                    localVariableNestedNested: ARRAY[1..2] OF ARRAY[3..4] OF ARRAY[5..6] OF IA;
                END_VAR
            END_FUNCTION
        "#;

        // TODO: snapshot needs to resolve to inner-most type, I want to see __FATPOINTER here
        // Put differently, the replacement of these datatypes works, but the snapshot doesn't reflect that
        // because it does currently not resolve these internal `__main_...` types.
        insta::assert_debug_snapshot!(helper::lower(source).get_unit().pous[0].variable_blocks, @r#"
        [
            VariableBlock {
                variables: [
                    Variable {
                        name: "localVariable",
                        data_type: DataTypeReference {
                            referenced_type: "__main_localVariable",
                        },
                    },
                    Variable {
                        name: "localVariableNested",
                        data_type: DataTypeReference {
                            referenced_type: "__main_localVariableNested",
                        },
                    },
                    Variable {
                        name: "localVariableNestedNested",
                        data_type: DataTypeReference {
                            referenced_type: "__main_localVariableNestedNested",
                        },
                    },
                ],
                variable_block_type: Local,
            },
        ]
        "#);
    }

    mod helper {
        use driver::{parse_and_annotate, pipelines::AnnotatedProject, pipelines::AnnotatedUnit};
        use plc_source::SourceCode;

        pub fn lower(source: impl Into<SourceCode>) -> AnnotatedUnit {
            let (_, mut project): (_, AnnotatedProject) =
                parse_and_annotate("unit-test", vec![source.into()]).unwrap();

            // (project.index, project.units.remove(0))
            project.units.remove(0)
        }

        fn lower_and_serialize_statements(source: impl Into<SourceCode>, pous: &[&str]) -> Vec<String> {
            let (_, project) = parse_and_annotate("unit-test", vec![source.into()]).unwrap();
            let unit = project.units[0].get_unit();

            let mut result = Vec::new();
            for pou in pous {
                result.push(format!("// Statements in {pou}"));
                let statements = &unit.implementations.iter().find(|it| &it.name == pou).unwrap().statements;
                let statements_str =
                    statements.iter().map(|statement| statement.as_string()).collect::<Vec<_>>();

                result.extend(statements_str);
            }

            result
        }
    }
}
