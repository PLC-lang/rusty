//! This module is responsible for lowering any
//! 1. [`CompilationUnit::properties`] into [`CompilationUnit::units`] and [`CompilationUnit::implementations`]
//! 2. [`AstStatement::ReferenceExpr`] into [`AstStatement::CallStatement`] to trigger the GET or SET methods
//!
//! The first step is triggered right after parsing the source code. For example assume some user wrote the
//! following code
//! ```iec61131st
//! FUNCTION_BLOCK fb
//!     PROPERTY foo : DINT
//!         GET
//!             // ...
//!             foo := <expr>;
//!             // ...
//!         END_GET
//!
//!         SET
//!             // ...
//!             <expr> := foo;
//!             // ...
//!         END_SET
//!     END_PROPERTY
//! END_FUNCTION_BLOCK
//! ```
//! internally these GET and SET blocks will be lowered into methods because semantically `<var> := fb.foo` is
//! equivalent to `<var> := fb.get_foo()` and `fb.foo := <expr>` is equivalent to `fb.set_foo(<expr>)`. Hence
//! the properties internal representation is as follows
//! ```iec61131st
//! FUNCTION_BLOCK fb
//!     // Compiler internal
//!     VAR_PROPERTY
//!         foo : DINT; // The name and datatype are derived from the property declaration
//!     END_VAR
//!
//!     METHOD __get_foo
//!         // ...
//!         foo := <expr>;
//!         // ...
//!         __get_foo := foo; // Compiler internal, will always be patched at the end of a GET block
//!     END_METHOD
//!
//!     METHOD __set_foo
//!         // Compiler internal
//!         VAR_INPUT
//!             __in : DINT;
//!         END_VAR
//!
//!         foo := __in; // Compiler internal, will always be patched at the beginning of a SET block
//!
//!         // ...
//!         <expr> := foo;
//!         // ...
//!     END_METHOD
//! END_FUNCTION_BLOCK
//! ```
//!
//! To then trigger these methods whenever a property is referenced in some statement, a second lowering stage
//! needs to be applied. That lowering stage happens right after we have successfully annotated all AST nodes.
//! Again, for example assume we have the following code
//! ```iec61131st
//! FUNCTION main
//!     VAR
//!         fbInstance : fb;
//!         localVariable : DINT;
//!     END_VAR
//!
//!     fbInstance.foo := 5;                // We want this to be `fbInstance.__set_foo(5);`
//!     localVariable := fbInstance.foo;    // ... and this to be `localVariable := fbInstance.__get_foo();`
//! END_FUNCTION
//! ```
//! Lowering these references is done by simply using the [`AstVisitorMut`] and iterating over all statements.
//! Any [`AstStatement::ReferenceExpr`] that is not the left hand side of an assignment will then be lowered
//! into a `__get_<property name>` call if we're dealing with a property reference. If the reference is the
//! left hand side of an assignment however, the node as a whole will instead be lowered into a
//! `__set_<property name>(<complete right hand side>)` call instead.

use std::collections::HashMap;

use helper::{create_internal_assignment, patch_prefix_to_name};
use plc_ast::{
    ast::{
        AccessModifier, ArgumentProperty, AstFactory, AstNode, AstStatement, CompilationUnit, Implementation,
        LinkageType, Pou, PouType, Property, PropertyKind, ReferenceAccess, ReferenceExpr, Variable,
        VariableBlock, VariableBlockType,
    },
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
    try_from_mut,
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
    pub fn lower_references_to_calls(&mut self, unit: &mut CompilationUnit) {
        self.visit_compilation_unit(unit);
    }

    /// Lowers [`CompilationUnit::properties`] into [`CompilationUnit::units`] and
    /// [`CompilationUnit::implementations`]
    pub fn lower_properties_to_pous(&mut self, unit: &mut CompilationUnit) {
        // Properties are internally represented as a variable, hence we keep track of all encountered
        // properties and their respective parent POUs to later patch these variables into a `VAR_PROPERTY`
        // block
        let mut parents: HashMap<String, Vec<Property>> = HashMap::new();

        for property in &mut unit.properties.drain(..) {
            match parents.get_mut(&property.parent_name) {
                Some(values) => values.push(property.clone()),
                None => {
                    parents.insert(property.parent_name.clone(), vec![property.clone()]);
                }
            }

            for property_impl in property.implementations {
                let name = format!(
                    "{parent}.__{kind}_{name}",
                    parent = property.parent_name,
                    kind = property_impl.kind,
                    name = property.name
                );

                let mut pou = Pou {
                    name,
                    kind: PouType::Method {
                        parent: property.parent_name.clone(),
                        property: Some(qualified_name(&property.parent_name, &property.name)),
                    },
                    variable_blocks: Vec::new(),
                    return_type: Some(property.datatype.clone()),
                    location: property.name_location.clone(),
                    name_location: property.name_location.clone(),
                    poly_mode: None,
                    generics: Vec::new(),
                    linkage: LinkageType::Internal,
                    super_class: None,
                    interfaces: Vec::new(),
                    is_const: false,
                    id: self.id_provider.next_id(),
                };

                let mut implementation = Implementation {
                    name: pou.name.clone(),
                    type_name: pou.name.clone(),
                    linkage: pou.linkage,
                    pou_type: pou.kind.clone(),
                    statements: property_impl.body,
                    location: pou.location.clone(),
                    name_location: pou.name_location.clone(),
                    overriding: false,
                    generic: false,
                    access: Some(AccessModifier::Public),
                };

                match property_impl.kind {
                    // We have to append a `<method_name> := <property_name>` assignment at the end of the
                    // list of statements when dealing with getters
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
                    // 1. Patch a variable block of type `VAR_INPUT` with a single variable named
                    //    `__in : <property_type>`
                    // 2. Prepend a `<property_name> := __in` assignment to the implementation
                    PropertyKind::Set => {
                        let parameter_name = "__in";

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
                        pou.return_type = None;

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

        // Iterate over all POUs, check if they have one or more properties defined and if so, add a
        // variable block of type `Property` consisting of all the properties.
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

impl AstVisitorMut for PropertyLowerer {
    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        for implementation in &mut unit.implementations {
            self.visit_implementation(implementation);
        }
    }

    fn visit_implementation(&mut self, implementation: &mut Implementation) {
        // We want to avoid calling `__get__foo` when inside a `GET` block of the `foo` property and similarly
        // avoid calling `__set__foo` when inside a `SET` block of the `foo` property. To do so we keep track
        // of the current context by storing the qualified name of the property we're currently inside of.
        if let PouType::Method { property: Some(qualified_name), .. } = &implementation.pou_type {
            self.context = Some(qualified_name.clone())
        }

        implementation.walk(self);

        // ...and reset the context once done (duh)
        self.context = None;
    }

    fn visit_assignment(&mut self, node: &mut AstNode) {
        let AstStatement::Assignment(data) = &mut node.stmt else {
            unreachable!();
        };

        self.visit(&mut data.right);

        match self.annotations.as_ref().and_then(|map| map.get(&data.left)) {
            // When dealing with an assignment where the left-hand side is a property reference, we have to
            // replace the reference with a method call to `__set_<property>(<right-hand-side>)`
            Some(annotation) if annotation.is_property() => {
                if self.context.as_deref() == annotation.qualified_name() {
                    return;
                }

                patch_prefix_to_name("__set_", &mut data.left);
                let call = AstFactory::create_call_statement(
                    data.left.as_ref().clone(),
                    Some(data.right.as_ref().clone()),
                    self.id_provider.next_id(),
                    node.location.clone(),
                );

                // In-place AST replacement of the assignment statements as a whole with the newly created call
                let _ = std::mem::replace(node, call);
            }

            _ => (),
        }
    }

    fn visit_reference_expr(&mut self, node: &mut AstNode) {
        let Some(ReferenceExpr { access, base }) = try_from_mut!(node, ReferenceExpr) else {
            return;
        };

        // First check if we're dealing with an array, as they'll need to be handled a bit differently, i.e.
        // we have to lower their base and index expression as well. Think of `fb.foo[fb.bar]` which needs to
        // become `fb.__get_foo()[fb.__get_bar()]`
        if let Some(base) = base {
            base.walk(self);
        }

        match access {
            ReferenceAccess::Member(node) | ReferenceAccess::Index(node) | ReferenceAccess::Cast(node) => {
                node.walk(self);
            }

            _ => (),
        };

        // ...and otherwise check the node as a whole
        if let Some(annotation) = self.annotations.as_ref().unwrap().get(node) {
            if !annotation.is_property() {
                return;
            }

            if self.context.as_deref() == annotation.qualified_name() {
                return;
            }

            // Any property reference that is not the left-hand side of an assignment needs to be replaced
            // with a function call to the respective getter property method.
            patch_prefix_to_name("__get_", node);
            let call = AstFactory::create_call_statement(
                node.clone(),
                None,
                self.id_provider.next_id(),
                node.location.clone(),
            );

            // In-place AST replacement of the reference-expr node with the newly created call
            let _ = std::mem::replace(node, call);
        }
    }
}

mod helper {
    use plc_ast::{
        ast::{AstFactory, AstNode, AstStatement, ReferenceAccess, ReferenceExpr},
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

    pub fn patch_prefix_to_name(prefix: &str, node: &mut AstNode) {
        let AstStatement::ReferenceExpr(ReferenceExpr { ref mut access, .. }) = &mut node.stmt else {
            return;
        };
        let ReferenceAccess::Member(member) = access else { return };
        let AstStatement::Identifier(name) = &mut member.stmt else { return };

        name.insert_str(0, prefix);
    }
}

#[cfg(test)]
mod tests {
    use plc_ast::{
        ast::{CompilationUnit, LinkageType},
        provider::IdProvider,
    };
    use plc_source::source_location::SourceLocationFactory;

    use crate::{
        lexer::lex_with_ids,
        lowering::property::PropertyLowerer,
        parser::parse,
        resolver::AstAnnotations,
        test_utils::tests::{annotate_with_ids, index_unit_with_id},
    };

    // Parse -> Lower -> Index -> Annotate -> Lower -> Snapshot
    fn lower(source: &str) -> CompilationUnit {
        let mut id_provider = IdProvider::default();
        let mut lowerer = PropertyLowerer::new(id_provider.clone());

        // Parse
        let (mut unit, diagnostics) = parse(
            lex_with_ids(source, id_provider.clone(), SourceLocationFactory::internal(source)),
            LinkageType::Internal,
            "test.st",
        );
        assert_eq!(diagnostics, Vec::new());

        // Lower
        lowerer.lower_properties_to_pous(&mut unit);

        // Index
        let mut index = index_unit_with_id(&unit, id_provider.clone());

        // Annotate
        let annotations = AstAnnotations::new(
            annotate_with_ids(&mut unit, &mut index, id_provider.clone()),
            id_provider.next_id(),
        );

        // Lower
        lowerer.annotations = Some(annotations);
        lowerer.lower_references_to_calls(&mut unit);

        unit
    }

    #[test]
    fn properties_are_used_within_each_other() {
        let source = r"
        FUNCTION_BLOCK fb
          VAR
            foo : DINT;
          END_VAR
          PROPERTY myProp: DINT
            GET
              myProp := foo;
            END_GET
            SET
              foo := myProp;
              myProp := another_prop;
            END_SET
          END_PROPERTY
          PROPERTY another_prop : DINT
            GET
              another_prop := myProp;
            END_GET
            SET
            END_SET
          END_PROPERTY
        END_FUNCTION_BLOCK
        ";

        let unit = lower(source);
        insta::assert_debug_snapshot!(unit.implementations, @r#"
        [
            Implementation {
                name: "fb",
                type_name: "fb",
                linkage: Internal,
                pou_type: FunctionBlock,
                statements: [],
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 21,
                            column: 8,
                            offset: 486,
                        }..TextLocation {
                            line: 20,
                            column: 22,
                            offset: 477,
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
                name: "fb.__get_myProp",
                type_name: "fb.__get_myProp",
                linkage: Internal,
                pou_type: Method {
                    parent: "fb",
                    property: Some(
                        "fb.myProp",
                    ),
                },
                statements: [
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "myProp",
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
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "__get_myProp",
                                },
                            ),
                            base: None,
                        },
                        right: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "myProp",
                                },
                            ),
                            base: None,
                        },
                    },
                ],
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 5,
                            column: 19,
                            offset: 102,
                        }..TextLocation {
                            line: 5,
                            column: 25,
                            offset: 108,
                        },
                    ),
                },
                name_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 5,
                            column: 19,
                            offset: 102,
                        }..TextLocation {
                            line: 5,
                            column: 25,
                            offset: 108,
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
                name: "fb.__set_myProp",
                type_name: "fb.__set_myProp",
                linkage: Internal,
                pou_type: Method {
                    parent: "fb",
                    property: Some(
                        "fb.myProp",
                    ),
                },
                statements: [
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "myProp",
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
                                    name: "foo",
                                },
                            ),
                            base: None,
                        },
                        right: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "myProp",
                                },
                            ),
                            base: None,
                        },
                    },
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "myProp",
                                },
                            ),
                            base: None,
                        },
                        right: CallStatement {
                            operator: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__get_another_prop",
                                    },
                                ),
                                base: None,
                            },
                            parameters: None,
                        },
                    },
                ],
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 5,
                            column: 19,
                            offset: 102,
                        }..TextLocation {
                            line: 5,
                            column: 25,
                            offset: 108,
                        },
                    ),
                },
                name_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 5,
                            column: 19,
                            offset: 102,
                        }..TextLocation {
                            line: 5,
                            column: 25,
                            offset: 108,
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
                name: "fb.__get_another_prop",
                type_name: "fb.__get_another_prop",
                linkage: Internal,
                pou_type: Method {
                    parent: "fb",
                    property: Some(
                        "fb.another_prop",
                    ),
                },
                statements: [
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "another_prop",
                                },
                            ),
                            base: None,
                        },
                        right: CallStatement {
                            operator: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__get_myProp",
                                    },
                                ),
                                base: None,
                            },
                            parameters: None,
                        },
                    },
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "__get_another_prop",
                                },
                            ),
                            base: None,
                        },
                        right: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "another_prop",
                                },
                            ),
                            base: None,
                        },
                    },
                ],
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 14,
                            column: 19,
                            offset: 325,
                        }..TextLocation {
                            line: 14,
                            column: 31,
                            offset: 337,
                        },
                    ),
                },
                name_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 14,
                            column: 19,
                            offset: 325,
                        }..TextLocation {
                            line: 14,
                            column: 31,
                            offset: 337,
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
                name: "fb.__set_another_prop",
                type_name: "fb.__set_another_prop",
                linkage: Internal,
                pou_type: Method {
                    parent: "fb",
                    property: Some(
                        "fb.another_prop",
                    ),
                },
                statements: [
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "another_prop",
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
                ],
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 14,
                            column: 19,
                            offset: 325,
                        }..TextLocation {
                            line: 14,
                            column: 31,
                            offset: 337,
                        },
                    ),
                },
                name_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 14,
                            column: 19,
                            offset: 325,
                        }..TextLocation {
                            line: 14,
                            column: 31,
                            offset: 337,
                        },
                    ),
                },
                overriding: false,
                generic: false,
                access: Some(
                    Public,
                ),
            },
        ]
        "#);
    }

    #[test]
    fn properties_are_patched_with_function_calls() {
        let source = r"
        FUNCTION_BLOCK fb
          VAR
            foo : DINT;
          END_VAR
          PROPERTY myProp: DINT
            GET
              myProp := foo;
            END_GET
            SET
              foo := myProp;
            END_SET
          END_PROPERTY
        printf('%d', myProp);
        END_FUNCTION_BLOCK
        ";

        let unit = lower(source);
        insta::assert_debug_snapshot!(unit.implementations, @r#"
        [
            Implementation {
                name: "fb",
                type_name: "fb",
                linkage: Internal,
                pou_type: FunctionBlock,
                statements: [
                    CallStatement {
                        operator: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "printf",
                                },
                            ),
                            base: None,
                        },
                        parameters: Some(
                            ExpressionList {
                                expressions: [
                                    LiteralString {
                                        value: "%d",
                                        is_wide: false,
                                    },
                                    CallStatement {
                                        operator: ReferenceExpr {
                                            kind: Member(
                                                Identifier {
                                                    name: "__get_myProp",
                                                },
                                            ),
                                            base: None,
                                        },
                                        parameters: None,
                                    },
                                ],
                            },
                        ),
                    },
                ],
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 13,
                            column: 8,
                            offset: 276,
                        }..TextLocation {
                            line: 13,
                            column: 29,
                            offset: 297,
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
                name: "fb.__get_myProp",
                type_name: "fb.__get_myProp",
                linkage: Internal,
                pou_type: Method {
                    parent: "fb",
                    property: Some(
                        "fb.myProp",
                    ),
                },
                statements: [
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "myProp",
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
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "__get_myProp",
                                },
                            ),
                            base: None,
                        },
                        right: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "myProp",
                                },
                            ),
                            base: None,
                        },
                    },
                ],
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 5,
                            column: 19,
                            offset: 102,
                        }..TextLocation {
                            line: 5,
                            column: 25,
                            offset: 108,
                        },
                    ),
                },
                name_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 5,
                            column: 19,
                            offset: 102,
                        }..TextLocation {
                            line: 5,
                            column: 25,
                            offset: 108,
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
                name: "fb.__set_myProp",
                type_name: "fb.__set_myProp",
                linkage: Internal,
                pou_type: Method {
                    parent: "fb",
                    property: Some(
                        "fb.myProp",
                    ),
                },
                statements: [
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "myProp",
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
                                    name: "foo",
                                },
                            ),
                            base: None,
                        },
                        right: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "myProp",
                                },
                            ),
                            base: None,
                        },
                    },
                ],
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 5,
                            column: 19,
                            offset: 102,
                        }..TextLocation {
                            line: 5,
                            column: 25,
                            offset: 108,
                        },
                    ),
                },
                name_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 5,
                            column: 19,
                            offset: 102,
                        }..TextLocation {
                            line: 5,
                            column: 25,
                            offset: 108,
                        },
                    ),
                },
                overriding: false,
                generic: false,
                access: Some(
                    Public,
                ),
            },
        ]
        "#);
    }

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

        let unit = lower(source);
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
            return_type: None,
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

    #[test]
    fn getter_with_array_of_strings() {
        let source = r"
        FUNCTION_BLOCK fb
            PROPERTY foo : ARRAY[1..5] OF STRING
                GET
                    foo := 5;
                END_GET

                SET
                    localPrivateVariable := foo;
                END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb2 EXTENDS fb
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                fbInstance : fb2;
            END_VAR

            fbInstance.foo[1];
            printf('%s$N', REF(fbInstance.foo[1]));
        END_FUNCTION
        ";

        // fbInstance.__get_foo()[1]
        let unit = lower(source);
        insta::assert_debug_snapshot!(unit.implementations[2].statements[0], @r###"
        ReferenceExpr {
            kind: Index(
                LiteralInteger {
                    value: 1,
                },
            ),
            base: Some(
                CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__get_foo",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "fbInstance",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                    parameters: None,
                },
            ),
        }
        "###);
        insta::assert_debug_snapshot!(unit.implementations[2].statements[1], @r###"
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "printf",
                    },
                ),
                base: None,
            },
            parameters: Some(
                ExpressionList {
                    expressions: [
                        LiteralString {
                            value: "%s\n",
                            is_wide: false,
                        },
                        CallStatement {
                            operator: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "REF",
                                    },
                                ),
                                base: None,
                            },
                            parameters: Some(
                                ReferenceExpr {
                                    kind: Index(
                                        LiteralInteger {
                                            value: 1,
                                        },
                                    ),
                                    base: Some(
                                        CallStatement {
                                            operator: ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__get_foo",
                                                    },
                                                ),
                                                base: Some(
                                                    ReferenceExpr {
                                                        kind: Member(
                                                            Identifier {
                                                                name: "fbInstance",
                                                            },
                                                        ),
                                                        base: None,
                                                    },
                                                ),
                                            },
                                            parameters: None,
                                        },
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
        }
        "###);
    }

    #[test]
    fn getter_used_in_array_index() {
        let source = r"
        FUNCTION_BLOCK fb
            PROPERTY foo : ARRAY[1..5] OF STRING
                GET
                    foo := ['a', 'b', 'c', 'd', 'e'];
                END_GET
            END_PROPERTY

            PROPERTY bar : DINT
                GET
                    bar := 5;
                END_GET
            END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instance : fb;
            END_VAR

            // We expect `instance.__get_foo()[instance.__get_bar()]`
            printf('%s$N', REF(instance.foo[instance.bar]));
        END_FUNCTION
        ";

        // fbInstance.__get_foo()[1]
        let unit = lower(source);
        insta::assert_debug_snapshot!(unit.implementations[1].statements[0], @r###"
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "printf",
                    },
                ),
                base: None,
            },
            parameters: Some(
                ExpressionList {
                    expressions: [
                        LiteralString {
                            value: "%s\n",
                            is_wide: false,
                        },
                        CallStatement {
                            operator: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "REF",
                                    },
                                ),
                                base: None,
                            },
                            parameters: Some(
                                ReferenceExpr {
                                    kind: Index(
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
                                                            name: "instance",
                                                        },
                                                    ),
                                                    base: None,
                                                },
                                            ),
                                        },
                                    ),
                                    base: Some(
                                        CallStatement {
                                            operator: ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__get_foo",
                                                    },
                                                ),
                                                base: Some(
                                                    ReferenceExpr {
                                                        kind: Member(
                                                            Identifier {
                                                                name: "instance",
                                                            },
                                                        ),
                                                        base: None,
                                                    },
                                                ),
                                            },
                                            parameters: None,
                                        },
                                    ),
                                },
                            ),
                        },
                    ],
                },
            ),
        }
        "###);
    }
}
