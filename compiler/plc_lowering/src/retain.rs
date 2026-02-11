//! This module handles indirection for retain variables in PROGRAMS
//! It also moves retain variables that are declared in non retain blocks into retain blocks.

use plc_ast::{
    ast::{AccessModifier, AstFactory, CompilationUnit, Variable},
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

pub struct RetainParticipant {
    pub ids: IdProvider,
}

impl RetainParticipant {
    pub fn new(ids: IdProvider) -> Self {
        Self { ids }
    }

    pub fn lower_retain(&mut self, units: &mut [CompilationUnit], index: plc::index::Index) {
        let mut lowerer = RetainLowerer { ids: self.ids.clone(), index, context: Context::default() };
        for unit in units {
            lowerer.visit_compilation_unit(unit);
        }
    }
}

struct RetainLowerer {
    ids: IdProvider,
    index: plc::index::Index,
    context: Context,
}

#[derive(Debug, Default)]
struct Context {
    container_name: Option<String>,
    in_program: bool,
    retain_variables: Vec<Variable>,
}

impl AstVisitorMut for RetainLowerer {
    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        unit.walk(self);
        // After visiting the compilation unit, add all retain variables to the global vars
        if !self.context.retain_variables.is_empty() {
            // Find an existing retain global variable block or create a new one if it doesn't exist
            unit.global_vars
                .iter_mut()
                .find(|block| block.retain)
                .map(|block| block.variables.append(&mut self.context.retain_variables))
                .unwrap_or_else(|| {
                    let retain_block = plc_ast::ast::VariableBlock {
                        variables: self.context.retain_variables.drain(..).collect(),
                        kind: plc_ast::ast::VariableBlockType::Global,
                        constant: false,
                        retain: true,
                        linkage: plc_ast::ast::LinkageType::Internal,
                        location: SourceLocation::internal(),
                        access: AccessModifier::Public,
                    };
                    unit.global_vars.push(retain_block);
                });
        }
    }
    fn visit_pou(&mut self, pou: &mut plc_ast::ast::Pou) {
        self.context.in_program = matches!(pou.kind, plc_ast::ast::PouType::Program);
        self.context.container_name = Some(pou.name.clone());
        pou.walk(self);
        self.context.in_program = false;
        self.context.container_name = None;
    }

    fn visit_variable_block(&mut self, block: &mut plc_ast::ast::VariableBlock) {
        let variables = std::mem::take(&mut block.variables);
        for mut variable in variables {
            if let Some(variable_index) =
                self.index.find_variable(self.context.container_name.as_deref(), &[variable.get_name()])
            {
                // If the variable should be retained
                if variable_index.should_retain(&self.index) {
                    if self.context.in_program {
                        let new_name = format!(
                            "__{}_{}",
                            self.context.container_name.as_deref().unwrap_or_default(),
                            variable.get_name()
                        );
                        // Create a global variable called __<pou_name>_<var_name> and move the initializer and datatype to the global variable
                        let new_var = Variable {
                            name: new_name.clone(),
                            data_type_declaration: variable.data_type_declaration.clone(),
                            initializer: variable.initializer.take(),
                            location: variable.location.clone(),
                            address: None,
                        };
                        // Move the variable to a global retain variable and replace the original variable with an auto reference to the global variable
                        self.context.retain_variables.push(new_var);
                        variable.address = Some(AstFactory::create_identifier(
                            new_name,
                            variable.location.clone(),
                            self.ids.next_id(),
                        ));
                        block.variables.push(variable);
                    } else if matches!(block.kind, plc_ast::ast::VariableBlockType::Global) && !block.retain {
                        self.context.retain_variables.push(variable);
                    } else {
                        block.variables.push(variable);
                    }
                } else {
                    block.variables.push(variable);
                }
            } else {
                // If the variable is not found in the index, just keep it as is (this should not happen since the index should contain all variables, but we want to be safe)
                block.variables.push(variable);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use plc_driver::parse_and_annotate;
    use plc_source::SourceCode;

    #[test]
    fn test_retain_lowering_on_program() {
        let source: SourceCode = r#"
        PROGRAM Test
        VAR RETAIN
            x: INT := 5;
        END_VAR
        END_PROGRAM
        "#
        .into();

        let (_, project) =
            parse_and_annotate("test", vec![source]).expect("Failed to parse compilation unit");

        let unit = &project.units[0];
        // Expect the unit to have a global retain variable and the original variable to be replaced with a auto reference to the retain variable
        assert_debug_snapshot!(unit, @r#"
        AnnotatedUnit {
            unit: CompilationUnit {
                global_vars: [
                    VariableBlock {
                        variables: [],
                        variable_block_type: Global,
                    },
                    VariableBlock {
                        variables: [],
                        variable_block_type: Global,
                    },
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "__Test_x",
                                data_type: DataTypeReference {
                                    referenced_type: "INT",
                                },
                                initializer: Some(
                                    LiteralInteger {
                                        value: 5,
                                    },
                                ),
                            },
                        ],
                        variable_block_type: Global,
                        retain: true,
                    },
                ],
                var_config: [],
                pous: [
                    POU {
                        name: "Test",
                        variable_blocks: [
                            VariableBlock {
                                variables: [
                                    Variable {
                                        name: "x",
                                        data_type: DataTypeReference {
                                            referenced_type: "INT",
                                        },
                                        address: Some(
                                            Identifier {
                                                name: "__Test_x",
                                            },
                                        ),
                                    },
                                ],
                                variable_block_type: Local,
                                retain: true,
                            },
                        ],
                        pou_type: Program,
                        return_type: None,
                        interfaces: [],
                        properties: [],
                    },
                ],
                implementations: [
                    Implementation {
                        name: "Test",
                        type_name: "Test",
                        linkage: Internal,
                        pou_type: Program,
                        statements: [],
                        location: SourceLocation {
                            span: Range(5:8 - 4:15),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(1:16 - 1:20),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(5:8 - 5:19),
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
                user_types: [],
                file: File(
                    "<internal>",
                ),
            },
            dependencies: {
                Variable(
                    "__Test_x",
                ),
                Datatype(
                    "INT",
                ),
                Datatype(
                    "DINT",
                ),
                Datatype(
                    "Test",
                ),
            },
            literals: StringLiterals {
                utf08: {},
                utf16: {},
            },
        }
        "#);
    }

    #[test]
    fn test_retain_lowering_on_program_nested() {
        let source: SourceCode = r#"
        FUNCTION_BLOCK FB
        VAR RETAIN
            a: INT := 5;
        END_VAR
        END_FUNCTION_BLOCK
        PROGRAM Test
        VAR
            x: FB;
        END_VAR
        END_PROGRAM
        "#
        .into();

        let (_, project) =
            parse_and_annotate("test", vec![source]).expect("Failed to parse compilation unit");

        let unit = &project.units[0];
        // Expect the unit to have a global retain variable and the original variable to be replaced with a auto reference to the retain variable
        assert_debug_snapshot!(unit, @r#"
        AnnotatedUnit {
            unit: CompilationUnit {
                global_vars: [
                    VariableBlock {
                        variables: [],
                        variable_block_type: Global,
                    },
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "__vtable_FB_instance",
                                data_type: DataTypeReference {
                                    referenced_type: "__vtable_FB",
                                },
                            },
                        ],
                        variable_block_type: Global,
                    },
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "__Test_x",
                                data_type: DataTypeReference {
                                    referenced_type: "FB",
                                },
                            },
                        ],
                        variable_block_type: Global,
                        retain: true,
                    },
                ],
                var_config: [],
                pous: [
                    POU {
                        name: "FB",
                        variable_blocks: [
                            VariableBlock {
                                variables: [
                                    Variable {
                                        name: "__vtable",
                                        data_type: DataTypeReference {
                                            referenced_type: "__FB___vtable",
                                        },
                                    },
                                ],
                                variable_block_type: Local,
                            },
                            VariableBlock {
                                variables: [
                                    Variable {
                                        name: "a",
                                        data_type: DataTypeReference {
                                            referenced_type: "INT",
                                        },
                                        initializer: Some(
                                            LiteralInteger {
                                                value: 5,
                                            },
                                        ),
                                    },
                                ],
                                variable_block_type: Local,
                                retain: true,
                            },
                        ],
                        pou_type: FunctionBlock,
                        return_type: None,
                        interfaces: [],
                        properties: [],
                    },
                    POU {
                        name: "Test",
                        variable_blocks: [
                            VariableBlock {
                                variables: [
                                    Variable {
                                        name: "x",
                                        data_type: DataTypeReference {
                                            referenced_type: "FB",
                                        },
                                        address: Some(
                                            Identifier {
                                                name: "__Test_x",
                                            },
                                        ),
                                    },
                                ],
                                variable_block_type: Local,
                            },
                        ],
                        pou_type: Program,
                        return_type: None,
                        interfaces: [],
                        properties: [],
                    },
                ],
                implementations: [
                    Implementation {
                        name: "FB",
                        type_name: "FB",
                        linkage: Internal,
                        pou_type: FunctionBlock,
                        statements: [],
                        location: SourceLocation {
                            span: Range(5:8 - 4:15),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(1:23 - 1:25),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(5:8 - 5:26),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        overriding: false,
                        generic: false,
                        access: None,
                    },
                    Implementation {
                        name: "Test",
                        type_name: "Test",
                        linkage: Internal,
                        pou_type: Program,
                        statements: [],
                        location: SourceLocation {
                            span: Range(10:8 - 9:15),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(6:16 - 6:20),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(10:8 - 10:19),
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
                                "__vtable_FB",
                            ),
                            variables: [
                                Variable {
                                    name: "__body",
                                    data_type: DataTypeReference {
                                        referenced_type: "____vtable_FB___body",
                                    },
                                },
                            ],
                        },
                        initializer: None,
                        scope: None,
                    },
                    UserTypeDeclaration {
                        data_type: PointerType {
                            name: Some(
                                "__FB___vtable",
                            ),
                            referenced_type: DataTypeReference {
                                referenced_type: "__VOID",
                            },
                            auto_deref: None,
                            type_safe: false,
                            is_function: false,
                        },
                        initializer: None,
                        scope: None,
                    },
                    UserTypeDeclaration {
                        data_type: PointerType {
                            name: Some(
                                "____vtable_FB___body",
                            ),
                            referenced_type: DataTypeReference {
                                referenced_type: "FB",
                            },
                            auto_deref: None,
                            type_safe: false,
                            is_function: true,
                        },
                        initializer: None,
                        scope: None,
                    },
                ],
                file: File(
                    "<internal>",
                ),
            },
            dependencies: {
                Variable(
                    "__vtable_FB_instance",
                ),
                Datatype(
                    "__vtable_FB",
                ),
                Datatype(
                    "____vtable_FB___body",
                ),
                Datatype(
                    "FB",
                ),
                Datatype(
                    "__FB___vtable",
                ),
                Datatype(
                    "__VOID",
                ),
                Datatype(
                    "INT",
                ),
                Variable(
                    "__Test_x",
                ),
                Datatype(
                    "DINT",
                ),
                Datatype(
                    "Test",
                ),
            },
            literals: StringLiterals {
                utf08: {},
                utf16: {},
            },
        }
        "#);
    }

    #[test]
    fn test_retain_in_global_nested_should_move_to_retain_block() {
        let source: SourceCode = r#"
        FUNCTION_BLOCK FB
        VAR RETAIN
            a: INT := 5;
        END_VAR
        END_FUNCTION_BLOCK
        VAR_GLOBAL RETAIN
            explicit_retain: FB;
            y : INT;
        END_VAR
        VAR_GLOBAL
            implicit_retain: FB;
            x : INT;
        END_VAR
        "#
        .into();

        let (_, project) =
            parse_and_annotate("test", vec![source]).expect("Failed to parse compilation unit");

        let unit = &project.units[0];
        // Expect the unit to have a global retain variable and the original variable to be replaced with an auto reference to the retain variable
        assert_debug_snapshot!(unit, @r#"
        AnnotatedUnit {
            unit: CompilationUnit {
                global_vars: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "explicit_retain",
                                data_type: DataTypeReference {
                                    referenced_type: "FB",
                                },
                            },
                            Variable {
                                name: "y",
                                data_type: DataTypeReference {
                                    referenced_type: "INT",
                                },
                            },
                            Variable {
                                name: "implicit_retain",
                                data_type: DataTypeReference {
                                    referenced_type: "FB",
                                },
                            },
                        ],
                        variable_block_type: Global,
                        retain: true,
                    },
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "x",
                                data_type: DataTypeReference {
                                    referenced_type: "INT",
                                },
                            },
                        ],
                        variable_block_type: Global,
                    },
                    VariableBlock {
                        variables: [],
                        variable_block_type: Global,
                    },
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "__vtable_FB_instance",
                                data_type: DataTypeReference {
                                    referenced_type: "__vtable_FB",
                                },
                            },
                        ],
                        variable_block_type: Global,
                    },
                ],
                var_config: [],
                pous: [
                    POU {
                        name: "FB",
                        variable_blocks: [
                            VariableBlock {
                                variables: [
                                    Variable {
                                        name: "__vtable",
                                        data_type: DataTypeReference {
                                            referenced_type: "__FB___vtable",
                                        },
                                    },
                                ],
                                variable_block_type: Local,
                            },
                            VariableBlock {
                                variables: [
                                    Variable {
                                        name: "a",
                                        data_type: DataTypeReference {
                                            referenced_type: "INT",
                                        },
                                        initializer: Some(
                                            LiteralInteger {
                                                value: 5,
                                            },
                                        ),
                                    },
                                ],
                                variable_block_type: Local,
                                retain: true,
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
                        name: "FB",
                        type_name: "FB",
                        linkage: Internal,
                        pou_type: FunctionBlock,
                        statements: [],
                        location: SourceLocation {
                            span: Range(5:8 - 4:15),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        name_location: SourceLocation {
                            span: Range(1:23 - 1:25),
                            file: Some(
                                "<internal>",
                            ),
                        },
                        end_location: SourceLocation {
                            span: Range(5:8 - 5:26),
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
                                "__vtable_FB",
                            ),
                            variables: [
                                Variable {
                                    name: "__body",
                                    data_type: DataTypeReference {
                                        referenced_type: "____vtable_FB___body",
                                    },
                                },
                            ],
                        },
                        initializer: None,
                        scope: None,
                    },
                    UserTypeDeclaration {
                        data_type: PointerType {
                            name: Some(
                                "__FB___vtable",
                            ),
                            referenced_type: DataTypeReference {
                                referenced_type: "__VOID",
                            },
                            auto_deref: None,
                            type_safe: false,
                            is_function: false,
                        },
                        initializer: None,
                        scope: None,
                    },
                    UserTypeDeclaration {
                        data_type: PointerType {
                            name: Some(
                                "____vtable_FB___body",
                            ),
                            referenced_type: DataTypeReference {
                                referenced_type: "FB",
                            },
                            auto_deref: None,
                            type_safe: false,
                            is_function: true,
                        },
                        initializer: None,
                        scope: None,
                    },
                ],
                file: File(
                    "<internal>",
                ),
            },
            dependencies: {
                Variable(
                    "explicit_retain",
                ),
                Datatype(
                    "FB",
                ),
                Datatype(
                    "__FB___vtable",
                ),
                Datatype(
                    "__VOID",
                ),
                Datatype(
                    "INT",
                ),
                Datatype(
                    "__vtable_FB",
                ),
                Datatype(
                    "____vtable_FB___body",
                ),
                Variable(
                    "y",
                ),
                Variable(
                    "implicit_retain",
                ),
                Variable(
                    "x",
                ),
                Variable(
                    "__vtable_FB_instance",
                ),
                Datatype(
                    "DINT",
                ),
            },
            literals: StringLiterals {
                utf08: {},
                utf16: {},
            },
        }
        "#);
    }
}
