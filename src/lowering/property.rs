//! TODO: Description of this module

use std::collections::HashMap;

use helper::create_internal_assignment;
use plc_ast::{
    ast::{
        AccessModifier, ArgumentProperty, AstFactory, AstNode, AstStatement, CompilationUnit, Implementation,
        LinkageType, Pou, PouType, Property, PropertyKind, ReferenceAccess, ReferenceExpr, Variable,
        VariableBlock, VariableBlockType,
    },
    mut_visitor::AstVisitorMut,
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;
use plc_util::convention::qualified_name;

use crate::resolver::{AnnotationMap, AstAnnotations};

pub struct PropertyLowerer {
    pub id_provider: IdProvider,
    pub annotations: Option<AstAnnotations>,
    context: Option<String>,
}

impl PropertyLowerer {
    pub fn new(id_provider: IdProvider) -> PropertyLowerer {
        PropertyLowerer { id_provider, annotations: None, context: None }
    }
}
impl PropertyLowerer {
    pub fn lower_identifiers_to_calls(&mut self, unit: &mut CompilationUnit) {
        self.visit_compilation_unit(unit);
    }
}

impl AstVisitorMut for PropertyLowerer {
    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        for implementation in &mut unit.implementations {
            self.visit_implementation(implementation);
        }
    }

    fn visit_implementation(&mut self, implementation: &mut Implementation) {
        if let PouType::Method { property: Some(qualified_name), .. } = &implementation.pou_type {
            // TODO: Two things, first let's maybe introduce a `enter_method` and `exit_method` method and secondly
            //       I'm not entirely happy with this solution but it seemed to be the easiest way to solve for now
            self.context = Some(qualified_name.clone())
        }

        for statement in &mut implementation.statements {
            self.visit(statement);
        }

        self.context = None;
    }

    fn visit_assignment(&mut self, node: &mut AstNode) {
        let AstStatement::Assignment(data) = &mut node.stmt else {
            unreachable!();
        };

        match self.annotations.as_ref().and_then(|map| map.get(&data.left)) {
            Some(annotation) if annotation.is_property() => {
                if self.context.as_deref() == annotation.get_qualified_name() {
                    return;
                }

                patch_prefix_to_name("__set_", &mut data.left);
                let call = AstFactory::create_call_statement(
                    data.left.as_ref().clone(),
                    Some(data.right.as_ref().clone()),
                    self.id_provider.next_id(),
                    node.location.clone(),
                );

                let _ = std::mem::replace(node, call);
            }

            _ => {
                self.visit(&mut data.right);
            }
        }
    }

    fn visit_reference_expr(&mut self, node: &mut AstNode) {
        if let Some(annotation) = self.annotations.as_ref().unwrap().get(node) {
            if !annotation.is_property() {
                return;
            }

            if self.context.as_deref() == annotation.get_qualified_name() {
                return;
            }

            patch_prefix_to_name("__get_", node);
            let call = AstFactory::create_call_statement(
                node.clone(),
                None,
                self.id_provider.next_id(),
                node.location.clone(),
            );

            let _ = std::mem::replace(node, call);
        }
    }
}

fn patch_prefix_to_name(prefix: &str, node: &mut AstNode) {
    let AstStatement::ReferenceExpr(ReferenceExpr { ref mut access, .. }) = &mut node.stmt else { return };
    let ReferenceAccess::Member(member) = access else { return };
    let AstStatement::Identifier(name) = &mut member.stmt else { return };

    name.insert_str(0, prefix);
}

// TODO: There are a lot of clone calls here, see if we can reduce them?
impl PropertyLowerer {
    pub fn lower_to_methods(&mut self, unit: &mut CompilationUnit) {
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
                    kind: PouType::Method {
                        parent: property.name_parent.clone(),
                        property: Some(qualified_name(&property.name_parent, &property.name)),
                    },
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
                    linkage: pou.linkage,
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
    use plc_ast::provider::IdProvider;

    use crate::{lowering::property::PropertyLowerer, test_utils::tests::parse};

    #[test]
    fn properties_are_lowered_into_methods() {
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

        // Pre-Lowering
        assert_eq!(unit.units.len(), 1);
        assert_eq!(unit.units[0].name, "fb");

        let mut lowerer = PropertyLowerer::new(IdProvider::default());
        lowerer.lower_to_methods(&mut unit);

        // Post-Lowering
        assert_eq!(unit.units.len(), 5);
        assert_eq!(unit.units[0].name, "fb");
        assert_eq!(unit.units[1].name, "fb.__get_foo");
        assert_eq!(unit.units[2].name, "fb.__set_foo");
        assert_eq!(unit.units[3].name, "fb.__get_bar");
        assert_eq!(unit.units[4].name, "fb.__set_bar");

        insta::assert_debug_snapshot!(unit.units[1], @r#"
        POU {
            name: "fb.__get_foo",
            variable_blocks: [],
            pou_type: Method {
                parent: "fb",
                property: Some(
                    "fb.foo",
                ),
            },
            return_type: Some(
                DataTypeReference {
                    referenced_type: "DINT",
                },
            ),
            interfaces: [],
        }
        "#);

        insta::assert_debug_snapshot!(unit.units[2], @r#"
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
                property: Some(
                    "fb.foo",
                ),
            },
            return_type: Some(
                DataTypeReference {
                    referenced_type: "DINT",
                },
            ),
            interfaces: [],
        }
        "#);

        assert_eq!(unit.implementations.len(), 5);
        insta::assert_debug_snapshot!(unit.implementations[1], @r#"
        Implementation {
            name: "fb.__get_foo",
            type_name: "fb.__get_foo",
            linkage: Internal,
            pou_type: Method {
                parent: "fb",
                property: Some(
                    "fb.foo",
                ),
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
        }
        "#);

        insta::assert_debug_snapshot!(unit.implementations[2], @r#"
        Implementation {
            name: "fb.__set_foo",
            type_name: "fb.__set_foo",
            linkage: Internal,
            pou_type: Method {
                parent: "fb",
                property: Some(
                    "fb.foo",
                ),
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
        }
        "#);
    }
}
