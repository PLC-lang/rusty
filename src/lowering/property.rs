//! TODO: Description of this module

use std::collections::HashMap;

use helper::create_internal_assignment;
use plc_ast::{
    ast::{
        AccessModifier, ArgumentProperty, CompilationUnit, Implementation, LinkageType, Pou, PouType,
        Property, PropertyKind, Variable, VariableBlock, VariableBlockType,
    },
    mut_visitor::AstVisitorMut,
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

pub struct PropertyLowerer {
    id_provider: IdProvider,
}

impl PropertyLowerer {
    pub fn new(id_provider: IdProvider) -> PropertyLowerer {
        PropertyLowerer { id_provider }
    }

    pub fn lower_properties_to_methods(&mut self, unit: &mut CompilationUnit) {
        self.visit_compilation_unit(unit);
    }
}

// TODO: There are a lot of clone calls here, see if we can reduce them?
impl AstVisitorMut for PropertyLowerer {
    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        let mut parents: HashMap<String, Vec<Property>> = HashMap::new();

        for property in &mut unit.properties.drain(..) {
            // Keep track of the parent POUs and all their defined properties
            match parents.get_mut(&property.name_parent) {
                Some(values) => values.push(property.clone()),
                None => {
                    parents.insert(property.name_parent.clone(), vec![property.clone()]);
                }
            }

            for property_impl in property.implementations {
                let name = format!(
                    "{parent}.__{kind}_{name}",
                    parent = property.name_parent,
                    kind = property_impl.kind,
                    name = property.name
                );

                let mut pou = Pou {
                    name,
                    kind: PouType::Method { parent: property.name_parent.clone() },
                    variable_blocks: Vec::new(),
                    return_type: Some(property.datatype.clone()),
                    location: SourceLocation::undefined(), // TODO: Update me
                    name_location: property.name_location.clone(),
                    poly_mode: None,
                    generics: Vec::new(),
                    linkage: LinkageType::Internal,
                    super_class: None,
                    interfaces: Vec::new(),
                    is_const: false,
                };

                let mut implementation = Implementation {
                    name: pou.name.clone(),
                    type_name: pou.name.clone(),
                    linkage: pou.linkage.clone(),
                    pou_type: pou.kind.clone(),
                    statements: property_impl.statements,
                    location: pou.location.clone(),
                    name_location: pou.name_location.clone(),
                    overriding: false,
                    generic: false,
                    access: Some(AccessModifier::Public),
                };

                match property_impl.kind {
                    // We have to append a `<method_name> := <property_name>` assignment when dealing with getters
                    PropertyKind::Get => {
                        let name_lhs = format!("__{}_{}", property_impl.kind, property.name);
                        let name_rhs = &property.name;

                        implementation.statements.push(create_internal_assignment(
                            &mut self.id_provider,
                            name_lhs,
                            name_rhs,
                        ));
                    }

                    // We have to do two things when dealing with setters:
                    // 1. Patch a variable block of type `VAR_INPUT` with a single variable named `__in : <property_type>`
                    // 2. Prepend a `<property_name> := __in` assignment to the implementation
                    PropertyKind::Set => {
                        let parameter_name = "__in";

                        // TODO: The return type of a setter should be VOID?
                        pou.variable_blocks.push(VariableBlock {
                            access: AccessModifier::Public,
                            constant: false,
                            retain: false,
                            variables: vec![Variable {
                                name: parameter_name.to_string(),
                                data_type_declaration: property.datatype.clone(),
                                initializer: None,
                                address: None,
                                location: SourceLocation::internal(),
                            }],
                            variable_block_type: VariableBlockType::Input(ArgumentProperty::ByVal),
                            linkage: LinkageType::Internal,
                            location: SourceLocation::internal(),
                        });

                        let name_lhs = &property.name;
                        let name_rhs = parameter_name;

                        implementation
                            .statements
                            .insert(0, create_internal_assignment(&mut self.id_provider, name_lhs, name_rhs));
                    }
                };

                unit.units.push(pou);
                unit.implementations.push(implementation);
            }
        }

        // Iterate over all POUs, check if they have one or more properties defined and if so, add a variable block
        // of type `Property` consisting of all the properties.
        for pou in &mut unit.units {
            if let Some(properties) = parents.get(&pou.name) {
                let mut variables = Vec::new();
                for property in properties {
                    variables.push(Variable {
                        name: property.name.clone(),
                        data_type_declaration: property.datatype.clone(),
                        initializer: None,
                        address: None,
                        location: property.name_location.clone(),
                    });
                }

                pou.variable_blocks.push(VariableBlock::property(variables));
            }
        }
    }
}

mod helper {
    use plc_ast::{
        ast::{AstFactory, AstNode},
        provider::IdProvider,
    };
    use plc_source::source_location::SourceLocation;

    /// Creates an assignment node with the given left-hand and right-hand side names as member references.
    /// The member references therby have a base of [`None`] and an internal source location.
    pub fn create_internal_assignment<T, U>(id_provider: &mut IdProvider, name_lhs: T, name_rhs: U) -> AstNode
    where
        T: Into<String>,
        U: Into<String>,
    {
        AstFactory::create_assignment(
            AstFactory::create_member_reference(
                AstFactory::create_identifier(
                    name_lhs.into(),
                    SourceLocation::internal(),
                    id_provider.next_id(),
                ),
                None,
                id_provider.next_id(),
            ),
            AstFactory::create_member_reference(
                AstFactory::create_identifier(
                    name_rhs.into(),
                    SourceLocation::internal(),
                    id_provider.next_id(),
                ),
                None,
                id_provider.next_id(),
            ),
            id_provider.next_id(),
        )
    }
}

#[cfg(test)]
mod tests {
    use plc_ast::{mut_visitor::AstVisitorMut, provider::IdProvider};

    use crate::{lowering::property::PropertyLowerer, test_utils::tests::parse};

    #[test]
    fn temp() {
        let source = r"
        FUNCTION_BLOCK fb
            VAR
                localPrivateVariable : DINT;
            END_VAR

            PROPERTY foo : DINT
                GET
                    foo := 5;
                END_GET

                SET
                    localPrivateVariable := foo;
                END_SET
            END_PROPERTY

            PROPERTY bar : DINT
                GET
                    bar := 5;
                END_GET

                SET
                    localPrivateVariable := bar;
                END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
    ";

        let (mut unit, diagnostics) = parse(source);
        assert_eq!(diagnostics, Vec::new());

        let mut lowerer = PropertyLowerer::new(IdProvider::default());
        lowerer.visit_compilation_unit(&mut unit);

        insta::assert_debug_snapshot!(unit, @r#"
        CompilationUnit {
            global_vars: [],
            var_config: [],
            units: [
                POU {
                    name: "fb",
                    variable_blocks: [
                        VariableBlock {
                            variables: [
                                Variable {
                                    name: "localPrivateVariable",
                                    data_type: DataTypeReference {
                                        referenced_type: "DINT",
                                    },
                                },
                            ],
                            variable_block_type: Local,
                        },
                        VariableBlock {
                            variables: [
                                Variable {
                                    name: "foo",
                                    data_type: DataTypeReference {
                                        referenced_type: "DINT",
                                    },
                                },
                                Variable {
                                    name: "bar",
                                    data_type: DataTypeReference {
                                        referenced_type: "DINT",
                                    },
                                },
                            ],
                            variable_block_type: Property,
                        },
                    ],
                    pou_type: FunctionBlock,
                    return_type: None,
                    interfaces: [],
                },
                POU {
                    name: "fb.__get_foo",
                    variable_blocks: [],
                    pou_type: Method {
                        parent: "fb",
                    },
                    return_type: Some(
                        DataTypeReference {
                            referenced_type: "DINT",
                        },
                    ),
                    interfaces: [],
                },
                POU {
                    name: "fb.__set_foo",
                    variable_blocks: [
                        VariableBlock {
                            variables: [
                                Variable {
                                    name: "__in",
                                    data_type: DataTypeReference {
                                        referenced_type: "DINT",
                                    },
                                },
                            ],
                            variable_block_type: Input(
                                ByVal,
                            ),
                        },
                    ],
                    pou_type: Method {
                        parent: "fb",
                    },
                    return_type: Some(
                        DataTypeReference {
                            referenced_type: "DINT",
                        },
                    ),
                    interfaces: [],
                },
                POU {
                    name: "fb.__get_bar",
                    variable_blocks: [],
                    pou_type: Method {
                        parent: "fb",
                    },
                    return_type: Some(
                        DataTypeReference {
                            referenced_type: "DINT",
                        },
                    ),
                    interfaces: [],
                },
                POU {
                    name: "fb.__set_bar",
                    variable_blocks: [
                        VariableBlock {
                            variables: [
                                Variable {
                                    name: "__in",
                                    data_type: DataTypeReference {
                                        referenced_type: "DINT",
                                    },
                                },
                            ],
                            variable_block_type: Input(
                                ByVal,
                            ),
                        },
                    ],
                    pou_type: Method {
                        parent: "fb",
                    },
                    return_type: Some(
                        DataTypeReference {
                            referenced_type: "DINT",
                        },
                    ),
                    interfaces: [],
                },
            ],
            implementations: [
                Implementation {
                    name: "fb",
                    type_name: "fb",
                    linkage: Internal,
                    pou_type: FunctionBlock,
                    statements: [],
                    location: SourceLocation {
                        span: Range(
                            TextLocation {
                                line: 25,
                                column: 8,
                                offset: 568,
                            }..TextLocation {
                                line: 24,
                                column: 24,
                                offset: 559,
                            },
                        ),
                    },
                    name_location: SourceLocation {
                        span: Range(
                            TextLocation {
                                line: 1,
                                column: 23,
                                offset: 24,
                            }..TextLocation {
                                line: 1,
                                column: 25,
                                offset: 26,
                            },
                        ),
                    },
                    overriding: false,
                    generic: false,
                    access: None,
                },
                Implementation {
                    name: "fb.__get_foo",
                    type_name: "fb.__get_foo",
                    linkage: Internal,
                    pou_type: Method {
                        parent: "fb",
                    },
                    statements: [
                        Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "foo",
                                    },
                                ),
                                base: None,
                            },
                            right: LiteralInteger {
                                value: 5,
                            },
                        },
                        Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__get_foo",
                                    },
                                ),
                                base: None,
                            },
                            right: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "foo",
                                    },
                                ),
                                base: None,
                            },
                        },
                    ],
                    location: SourceLocation {
                        span: None,
                    },
                    name_location: SourceLocation {
                        span: Range(
                            TextLocation {
                                line: 6,
                                column: 21,
                                offset: 130,
                            }..TextLocation {
                                line: 6,
                                column: 24,
                                offset: 133,
                            },
                        ),
                    },
                    overriding: false,
                    generic: false,
                    access: Some(
                        Public,
                    ),
                },
                Implementation {
                    name: "fb.__set_foo",
                    type_name: "fb.__set_foo",
                    linkage: Internal,
                    pou_type: Method {
                        parent: "fb",
                    },
                    statements: [
                        Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "foo",
                                    },
                                ),
                                base: None,
                            },
                            right: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__in",
                                    },
                                ),
                                base: None,
                            },
                        },
                        Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "localPrivateVariable",
                                    },
                                ),
                                base: None,
                            },
                            right: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "foo",
                                    },
                                ),
                                base: None,
                            },
                        },
                    ],
                    location: SourceLocation {
                        span: None,
                    },
                    name_location: SourceLocation {
                        span: Range(
                            TextLocation {
                                line: 6,
                                column: 21,
                                offset: 130,
                            }..TextLocation {
                                line: 6,
                                column: 24,
                                offset: 133,
                            },
                        ),
                    },
                    overriding: false,
                    generic: false,
                    access: Some(
                        Public,
                    ),
                },
                Implementation {
                    name: "fb.__get_bar",
                    type_name: "fb.__get_bar",
                    linkage: Internal,
                    pou_type: Method {
                        parent: "fb",
                    },
                    statements: [
                        Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "bar",
                                    },
                                ),
                                base: None,
                            },
                            right: LiteralInteger {
                                value: 5,
                            },
                        },
                        Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__get_bar",
                                    },
                                ),
                                base: None,
                            },
                            right: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "bar",
                                    },
                                ),
                                base: None,
                            },
                        },
                    ],
                    location: SourceLocation {
                        span: None,
                    },
                    name_location: SourceLocation {
                        span: Range(
                            TextLocation {
                                line: 16,
                                column: 21,
                                offset: 356,
                            }..TextLocation {
                                line: 16,
                                column: 24,
                                offset: 359,
                            },
                        ),
                    },
                    overriding: false,
                    generic: false,
                    access: Some(
                        Public,
                    ),
                },
                Implementation {
                    name: "fb.__set_bar",
                    type_name: "fb.__set_bar",
                    linkage: Internal,
                    pou_type: Method {
                        parent: "fb",
                    },
                    statements: [
                        Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "bar",
                                    },
                                ),
                                base: None,
                            },
                            right: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__in",
                                    },
                                ),
                                base: None,
                            },
                        },
                        Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "localPrivateVariable",
                                    },
                                ),
                                base: None,
                            },
                            right: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "bar",
                                    },
                                ),
                                base: None,
                            },
                        },
                    ],
                    location: SourceLocation {
                        span: None,
                    },
                    name_location: SourceLocation {
                        span: Range(
                            TextLocation {
                                line: 16,
                                column: 21,
                                offset: 356,
                            }..TextLocation {
                                line: 16,
                                column: 24,
                                offset: 359,
                            },
                        ),
                    },
                    overriding: false,
                    generic: false,
                    access: Some(
                        Public,
                    ),
                },
            ],
            interfaces: [],
            user_types: [],
            file_name: "test.st",
            properties: [],
        }
    "#);
    }
}
