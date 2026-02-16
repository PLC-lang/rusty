use plc_ast::{
    ast::{Assignment, AstFactory, AstStatement, CallStatement, CompilationUnit, DataTypeDeclaration},
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

use crate::{
    index::Index,
    resolver::{AnnotationMap, AnnotationMapImpl},
    typesystem::DataType,
};

const FATPOINTER_TYPE_NAME: &str = "__FATPOINTER";
const FATPOINTER_DATA_FIELD_NAME: &str = "data";
const FATPOINTER_TABLE_FIELD_NAME: &str = "table";

pub struct InterfaceDispatchLowerer<'a> {
    ids: IdProvider,
    index: &'a Index,
    annotations: &'a AnnotationMapImpl,

    /// Do we need to generate the `__FATPOINTER` struct definition?
    needs_fatpointer_definition: bool,

    /// Are we in a call statement?
    call_guard: bool,
}

impl<'a> InterfaceDispatchLowerer<'a> {
    pub fn new(ids: IdProvider, index: &'a Index, annotations: &'a AnnotationMapImpl) -> Self {
        Self { ids, index, annotations, needs_fatpointer_definition: false, call_guard: false }
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

    /// Lowers concrete POU → interface assignments into fat pointer field assignments.
    ///
    /// `reference := instance` becomes:
    /// - `reference.data  := ADR(instance)`
    /// - `reference.table := ADR(__itable_<interface>_<POU>_instance)`
    ///
    /// Both are packed into a single `ExpressionList` node so we can expand one statement into
    /// two without needing access to the surrounding statement list.
    fn visit_assignment(&mut self, node: &mut plc_ast::ast::AstNode) {
        // Named call arguments are also represented as assignments — skip them here,
        // they are handled separately in call argument lowering.
        if self.call_guard {
            return;
        }

        let AstStatement::Assignment(Assignment { left, right }) = &node.stmt else { unreachable!() };

        // Only transform when the LHS is an interface type (per pre-lowering annotations)
        let Some(lhs_type) = self.annotations.get_type(left, self.index) else { return };
        if !lhs_type.is_interface() {
            return;
        }

        // Interface → interface assignment is out of scope
        let Some(rhs_type) = self.annotations.get_type(right, self.index) else { return };
        if rhs_type.is_interface() {
            return;
        }

        let interface_name = lhs_type.get_name();
        let pou_name = rhs_type.get_name();
        let itable_instance_name = format!("__itable_{interface_name}_{pou_name}_instance");

        // Clone left/right before replacing the node
        let left = left.as_ref().clone();
        let right = right.as_ref().clone();

        // reference.data := ADR(instance)
        let assign_data = helper::create_fat_pointer_field_assignment(
            &mut self.ids,
            &left,
            FATPOINTER_DATA_FIELD_NAME,
            &right,
        );

        // reference.table := ADR(__itable_<interface>_<POU>_instance)
        let itable_ref = AstFactory::create_member_reference(
            AstFactory::create_identifier(
                itable_instance_name,
                SourceLocation::internal(),
                self.ids.next_id(),
            ),
            None,
            self.ids.next_id(),
        );
        let assign_table = helper::create_fat_pointer_field_assignment(
            &mut self.ids,
            &left,
            FATPOINTER_TABLE_FIELD_NAME,
            &itable_ref,
        );

        // TODO(vosa): Mention why an expression list
        node.stmt = AstStatement::ExpressionList(vec![assign_data, assign_table]);
    }

    fn visit_call_statement(&mut self, node: &mut plc_ast::ast::AstNode) {
        self.call_guard = true;

        let AstStatement::CallStatement(CallStatement { ref mut operator, ref mut parameters }) =
            &mut node.stmt
        else {
            unreachable!();
        };

        operator.walk(self);
        if let Some(ref mut parameters) = parameters {
            parameters.walk(self);
        }

        self.call_guard = false;
    }
}

mod helper {
    use plc_ast::{
        ast::{
            AstFactory, AstNode, DataType, DataTypeDeclaration, LinkageType, UserTypeDeclaration, Variable,
        },
        provider::IdProvider,
    };
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

    /// Creates `<base>.<field> := ADR(<target>)`.
    pub fn create_fat_pointer_field_assignment(
        ids: &mut IdProvider,
        base: &AstNode,
        field: &str,
        target: &AstNode,
    ) -> AstNode {
        let location = SourceLocation::internal();

        // LHS: <base>.<field>
        let lhs = AstFactory::create_member_reference(
            AstFactory::create_identifier(field, &location, ids.next_id()),
            Some(base.clone()),
            ids.next_id(),
        );

        // RHS: ADR(<target>)
        let rhs = AstFactory::create_call_statement(
            AstFactory::create_member_reference(
                AstFactory::create_identifier("ADR", &location, ids.next_id()),
                None,
                ids.next_id(),
            ),
            Some(target.clone()),
            ids.next_id(),
            &location,
        );

        AstFactory::create_assignment(lhs, rhs, ids.next_id())
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
    use crate::lowering::polymorphism::dispatch::interface::{
        tests::helper::lower_and_serialize_statements, FATPOINTER_TYPE_NAME,
    };

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

    #[test]
    fn assignments_expand() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                VAR_INPUT
                    in: IA;
                    instance: FbA;
                END_VAR

                in := instance;
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance: FbA;
                    reference: IA;
                END_VAR

                reference := instance;
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main", "FbA"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "reference.data := ADR(instance), reference.table := ADR(__itable_IA_FbA_instance)",
            "// Statements in FbA",
            "in.data := ADR(instance), in.table := ADR(__itable_IA_FbA_instance)",
        ]
        "#)
    }

    // TODO: Expand with a named-argument variant once named arg lowering is implemented.
    #[test]
    fn call_args_single_interface_param() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    ref: IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instance: FbA;
                END_VAR
                consumer(instance);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "consumer(instance)",
        ]
        "#)
    }

    // TODO: Expand with a named-argument variant once named arg lowering is implemented.
    #[test]
    fn call_args_mixed_non_interface_and_interface() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    value: DINT;
                    ref: IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instance: FbA;
                END_VAR
                consumer(5, instance);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "consumer(5, instance)",
        ]
        "#)
    }

    // TODO: Expand with a named-argument variant once named arg lowering is implemented.
    #[test]
    fn call_args_interface_passed_directly() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    ref: IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instance: FbA;
                    reference: IA;
                END_VAR
                reference := instance;
                consumer(reference);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(instance)",
            "__user_init_FbA(instance)",
            "reference.data := ADR(instance), reference.table := ADR(__itable_IA_FbA_instance)",
            "consumer(reference)",
        ]
        "#)
    }

    // TODO: Expand with a named-argument variant once named arg lowering is implemented.
    #[test]
    fn call_args_multiple_interface_params() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION consumer2
                VAR_INPUT
                    a: IA;
                    b: IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    i1: FbA;
                    i2: FbA;
                END_VAR
                consumer2(i1, i2);
            END_FUNCTION
        "#;

        insta::assert_debug_snapshot!(lower_and_serialize_statements(source, &["main"]), @r#"
        [
            "// Statements in main",
            "__init_fba(i1)",
            "__init_fba(i2)",
            "__user_init_FbA(i1)",
            "__user_init_FbA(i2)",
            "consumer2(i1, i2)",
        ]
        "#)
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

        pub fn lower_and_serialize_statements(source: impl Into<SourceCode>, pous: &[&str]) -> Vec<String> {
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
