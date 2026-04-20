//! This module is responsible for lowering any
//! 1. [`CompilationUnit::properties`] into [`CompilationUnit::units`] and [`CompilationUnit::implementations`]
//! 2. [`AstStatement::ReferenceExpr`] into [`AstStatement::CallStatement`] to call the PROPERTY_GET or PROPERTY_SET methods
//!
//! The first step is triggered right after parsing the source code. For example assume some user wrote the
//! following code
//! ```iec61131st
//! FUNCTION_BLOCK fb
//!     PROPERTY_GET
//!         // ...
//!         foo := <expr>;
//!         // ...
//!     END_PROPERTY
//!
//!     PROPERTY_SET
//!         // ...
//!         <expr> := foo;
//!         // ...
//!     END_PROPERTY
//! END_FUNCTION_BLOCK
//! ```
//! internally these PROPERTY_GET and PROPERTY_SET blocks will be lowered into methods because semantically `<var> := fb.foo` is
//! equivalent to `<var> := fb.get_foo()` and `fb.foo := <expr>` is equivalent to `fb.set_foo(<expr>)`. Hence
//! the properties internal representation is as follows
//! ```iec61131st
//! FUNCTION_BLOCK fb
//!     METHOD __get_foo
//!         VAR
//!             foo : DINT; // Patched in by the lowerer
//!         END_VAR
//!
//!         // ...
//!         foo := <expr>;
//!         // ...
//!         __get_foo := foo; // Patched in by the lowerer
//!     END_METHOD
//!
//!     METHOD __set_foo
//!         VAR_INPUT
//!             foo : DINT; // Patched in by the lowerer
//!         END_VAR
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
//! Lowering these references is done by simply using the [`AstVisitorMut`], iterating over all statements and
//! identifying if any reference has a property annotation. If so, we distinguish between two cases:
//! 1. An assignment where the left hand side is a property reference, in which case the whole right hand side
//!    needs to be wrapped in a function call as `__set_<property name>(<right hand side>)`
//! 2. A reference, that is not the left hand side of an assignment, in which case the reference itself needs
//!    to be replaced with a function call as `__get_<property name>()`

use crate::lowering::property::helper::create_internal_assignment;
use crate::resolver::{AnnotationMap, AstAnnotations};
use helper::patch_prefix_to_name;
use plc_ast::{
    ast::{
        AccessModifier, ArgumentProperty, AstFactory, AstNode, AstStatement, CompilationUnit,
        DeclarationKind, Identifier, Implementation, LinkageType, Pou, PouType, PropertyBlock, PropertyKind,
        ReferenceAccess, ReferenceExpr, Variable, VariableBlock, VariableBlockType,
    },
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
    try_from_mut,
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;

pub struct PropertyLowerer {
    pub id_provider: IdProvider,
    pub annotations: Option<AstAnnotations>,
    pub diagnostics: Vec<Diagnostic>,
}

impl PropertyLowerer {
    pub fn new(id_provider: IdProvider) -> PropertyLowerer {
        PropertyLowerer { id_provider, annotations: None, diagnostics: Vec::new() }
    }
}

impl PropertyLowerer {
    /// Lowers [`CompilationUnit::properties`] into [`CompilationUnit::units`] and
    /// [`CompilationUnit::implementations`]
    pub fn properties_to_pous(&mut self, unit: &mut CompilationUnit) {
        let mut local_pous = Vec::new();
        let mut local_impls = Vec::new();

        // Lower properties defined in a POU (e.g. a FUNCTION_BLOCK).
        for pou in &mut unit.pous {
            for property in &mut pou.properties {
                let (pous, impls) = lower_to_pou(self.id_provider.clone(), &pou.name, property);

                local_pous.extend(pous);
                local_impls.extend(impls);
            }
        }

        // Lower properties defined in an interface.
        for interface in &mut unit.interfaces {
            for property in &mut interface.properties {
                let (pous, _) = lower_to_pou(self.id_provider.clone(), &interface.ident.name, property);

                interface.methods.extend(pous);
            }
        }

        unit.pous.extend(local_pous);
        unit.implementations.extend(local_impls);
    }

    /// Lowers any property references into method calls.
    pub fn properties_to_fncalls(&mut self, unit: &mut CompilationUnit) {
        self.visit_compilation_unit(unit);
    }

    fn visit_assignment_target(&mut self, node: &mut AstNode) {
        let Some(ReferenceExpr { access, base }) = try_from_mut!(node, ReferenceExpr) else {
            return;
        };

        // Lower expressions nested inside the assignment target, such as array indices.
        match access {
            ReferenceAccess::Index(node) | ReferenceAccess::Cast(node) => node.walk(self),

            // Member/global accesses are part of the assignment target chain itself. Do not walk them here
            ReferenceAccess::Global(_) | ReferenceAccess::Member(_) => (),
            ReferenceAccess::Deref | ReferenceAccess::Address => (),
        }

        // Continue along the target chain without lowering the target references themselves.
        if let Some(base) = base {
            self.visit_assignment_target(base);
        }
    }
}

impl AstVisitorMut for PropertyLowerer {
    fn visit_assignment(&mut self, node: &mut AstNode) {
        let AstStatement::Assignment(data) = &mut node.stmt else {
            unreachable!();
        };

        // Lower reads inside the assignment target, such as property gets used in index expressions.
        self.visit_assignment_target(&mut data.left);

        // Lower the right-hand side normally.
        self.visit(&mut data.right);

        let Some(annotations) = self.annotations.as_ref() else {
            return;
        };

        // Reject assignments that access through a property instead of assigning the property directly.
        if let Some(diagnostic) = validation::validate_nested_property_set_target(annotations, &data.left) {
            self.diagnostics.push(diagnostic);
            return;
        }

        match annotations.get(&data.left) {
            // When dealing with an assignment where the left-hand side is a property reference, we have to
            // replace the reference with a method call to `__set_<property>(<right-hand-side>)`
            Some(annotation) if annotation.is_property() => {
                patch_prefix_to_name("__set_", &mut data.left);
                let call = AstFactory::create_call_statement(
                    data.left.as_ref().clone(),
                    Some(data.right.as_ref().clone()),
                    self.id_provider.next_id(),
                    node.location.clone(),
                );

                // In-place AST replacement of the assignment statement with the newly created call.
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

/// The actual logic for lowering properties into methods and their implementations counterpart
pub fn lower_to_pou(
    mut provider: IdProvider,
    parent: &str,
    property: &mut PropertyBlock,
) -> (Vec<Pou>, Vec<Implementation>) {
    let mut pous = Vec::new();
    let mut impls = Vec::new();

    for property_impl in property.implementations.clone() {
        let Identifier { name, location } = &property.ident;

        let mangled_name = format!("{parent}.__{kind}_{name}", kind = property_impl.kind);

        // First transform the property into a method (__get... or __set...)
        let mut pou = Pou {
            name: mangled_name,
            kind: PouType::Method {
                parent: parent.to_string(),
                property: Some((name.to_string(), property_impl.kind)),
                declaration_kind: DeclarationKind::Concrete,
            },
            variable_blocks: property_impl.variable_blocks,
            return_type: None,
            location: location.clone(),
            name_location: location.clone(),
            poly_mode: None,
            generics: Vec::new(),
            linkage: LinkageType::Internal,
            super_class: None,
            interfaces: Vec::new(),
            is_const: false,
            id: provider.next_id(),
            properties: Vec::new(),
        };

        // ...then transform any statement inside the property into an implementation
        let mut implementation = Implementation {
            name: pou.name.clone(),
            type_name: pou.name.clone(),
            linkage: pou.linkage,
            pou_type: pou.kind.clone(),
            statements: property_impl.body,
            location: location.clone(),
            name_location: location.clone(),
            end_location: property_impl.end_location.clone(),
            overriding: false,
            generic: false,
            access: Some(AccessModifier::Public),
        };

        // ...then patch in local variables (and additionally some extra statements) for the implementation
        match property_impl.kind {
            PropertyKind::Get => {
                pou.variable_blocks.push(VariableBlock {
                    access: AccessModifier::Public,
                    constant: false,
                    retain: false,
                    variables: vec![Variable {
                        name: name.to_string(),
                        data_type_declaration: property.datatype.clone(),
                        initializer: None,
                        address: None,
                        location: SourceLocation::internal(),
                    }],
                    kind: VariableBlockType::Local,
                    linkage: LinkageType::Internal,
                    location: SourceLocation::internal(),
                });
                pou.return_type = Some(property.datatype.clone());

                let name_lhs = format!("__{}_{}", property_impl.kind, name);

                implementation.statements.push(create_internal_assignment(&mut provider, name_lhs, name));
            }

            PropertyKind::Set => {
                pou.variable_blocks.push(VariableBlock {
                    access: AccessModifier::Public,
                    constant: false,
                    retain: false,
                    variables: vec![Variable {
                        name: name.to_string(),
                        data_type_declaration: property.datatype.clone(),
                        initializer: None,
                        address: None,
                        location: SourceLocation::internal(),
                    }],
                    kind: VariableBlockType::Input(ArgumentProperty::ByVal),
                    linkage: LinkageType::Internal,
                    location: SourceLocation::internal(),
                });
            }
        };

        // ...aaaaand done
        pous.push(pou);
        impls.push(implementation);
    }

    (pous, impls)
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

mod validation {
    use plc_ast::ast::{AstNode, AstStatement, ReferenceAccess, ReferenceExpr};
    use plc_diagnostics::diagnostics::Diagnostic;
    use plc_source::source_location::SourceLocation;

    use crate::resolver::{AnnotationMap, AstAnnotations};

    pub fn validate_nested_property_set_target(
        annotations: &AstAnnotations,
        left: &AstNode,
    ) -> Option<Diagnostic> {
        let mut current = left;

        loop {
            let base = get_base(current)?;

            // Reject `foo.bar := value` when `foo` is a property. Setters operate on the whole property
            // and cannot be followed by further member or index accesses.
            if annotations.get(base).is_some_and(|annotation| annotation.is_property()) {
                return Some(invalid_property_set_target(left));
            }

            current = base;
        }
    }

    fn get_base(node: &AstNode) -> Option<&AstNode> {
        let AstStatement::ReferenceExpr(ReferenceExpr { base, .. }) = &node.stmt else {
            return None;
        };

        base.as_deref()
    }

    fn invalid_property_set_target(target: &AstNode) -> Diagnostic {
        Diagnostic::new("Properties can only be assigned as a whole, not through member or index access")
            .with_error_code("E128")
            .with_location(get_terminal_leaf_location(target))
    }

    fn get_terminal_leaf_location(node: &AstNode) -> SourceLocation {
        let AstStatement::ReferenceExpr(ReferenceExpr { access, base }) = &node.stmt else {
            return node.location.clone();
        };

        match access {
            ReferenceAccess::Global(node) | ReferenceAccess::Member(node) => node.location.clone(),

            // Highlight the trailing indexed access, e.g. `values[expr]`, rather than only the inner
            // index expression `expr`.
            ReferenceAccess::Index(_) => base
                .as_deref()
                .map(get_terminal_leaf_location)
                .map(|location| location.span(&node.location))
                .unwrap_or_else(|| node.location.clone()),

            ReferenceAccess::Cast(_) | ReferenceAccess::Deref | ReferenceAccess::Address => {
                node.location.clone()
            }
        }
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
        resolver::{AnnotationMapImpl, AstAnnotations},
        test_utils::tests::{annotate_with_ids, index_unit_with_id},
    };

    fn lower_properties_to_pous(source: &str) -> (CompilationUnit, AnnotationMapImpl) {
        lower_properties_to_pous_with_provider(source, IdProvider::default())
    }

    fn lower_properties_to_pous_with_provider(
        source: &str,
        id_provider: IdProvider,
    ) -> (CompilationUnit, AnnotationMapImpl) {
        let mut lowerer = PropertyLowerer::new(id_provider.clone());
        // Parse
        let (mut unit, diagnostics) = parse(
            lex_with_ids(source, id_provider.clone(), SourceLocationFactory::internal(source)),
            LinkageType::Internal,
            "test.st",
        );
        assert_eq!(diagnostics, Vec::new());

        // Lower
        lowerer.properties_to_pous(&mut unit);

        // Index
        let mut index = index_unit_with_id(&unit, id_provider.clone());

        // Annotate
        let annotations = annotate_with_ids(&unit, &mut index, id_provider.clone());

        (unit, annotations)
    }

    // Parse -> Lower -> Index -> Annotate -> Lower -> Snapshot
    fn lower(source: &str) -> CompilationUnit {
        let mut id_provider = IdProvider::default();
        let mut lowerer = PropertyLowerer::new(id_provider.clone());
        let (mut unit, annotations) = lower_properties_to_pous_with_provider(source, id_provider.clone());
        // Lower
        let annotations = AstAnnotations::new(annotations, id_provider.next_id());
        lowerer.annotations = Some(annotations);
        lowerer.properties_to_fncalls(&mut unit);

        unit
    }

    mod ast {
        use crate::lowering::property::tests::{lower, lower_properties_to_pous};

        #[test]
        fn get_is_lowered_to_method_with_local_variable_and_tail_return_statement() {
            let source = r"
            FUNCTION_BLOCK fb
                PROPERTY_GET foo: DINT
                END_PROPERTY
            END_FUNCTION_BLOCK
            ";

            let (unit, _) = lower_properties_to_pous(source);
            insta::assert_debug_snapshot!(unit.pous[1], @r#"
            POU {
                name: "fb.__get_foo",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "foo",
                                data_type: DataTypeReference {
                                    referenced_type: "DINT",
                                },
                            },
                        ],
                        variable_block_type: Local,
                    },
                ],
                pou_type: Method {
                    parent: "fb",
                    property: Some(
                        (
                            "foo",
                            Get,
                        ),
                    ),
                    declaration_kind: Concrete,
                },
                return_type: Some(
                    DataTypeReference {
                        referenced_type: "DINT",
                    },
                ),
                interfaces: [],
                properties: [],
            }
            "#);

            let return_statement = &unit.implementations[1].statements[0];
            insta::assert_debug_snapshot!(return_statement, @r#"
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
            }
            "#);
        }

        #[test]
        fn set_is_lowered_to_method_with_local_variable_of_type_input() {
            let source = r"
            FUNCTION_BLOCK fb
                PROPERTY_SET foo: DINT
                END_PROPERTY
            END_FUNCTION_BLOCK
            ";

            let (unit, _) = lower_properties_to_pous(source);
            insta::assert_debug_snapshot!(unit.pous[1], @r#"
            POU {
                name: "fb.__set_foo",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "foo",
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
                        (
                            "foo",
                            Set,
                        ),
                    ),
                    declaration_kind: Concrete,
                },
                return_type: None,
                interfaces: [],
                properties: [],
            }
            "#);
        }

        #[test]
        fn get_and_set_retains_original_statements_in_body() {
            let source = r"
            FUNCTION_BLOCK fb
                PROPERTY_GET foo: DINT
                    foo := 1;
                    foo := 2;
                    foo := 3;
                    foo := 4;
                    foo := 5;
                END_PROPERTY

                PROPERTY_SET foo: DINT
                    foo := 1;
                    foo := 2;
                    foo := 3;
                    foo := 4;
                    foo := 5;
                END_PROPERTY
            END_FUNCTION_BLOCK
            ";

            let unit = lower(source);
            assert_eq!(unit.implementations.len(), 3);

            assert_eq!(unit.implementations[1].statements.len(), 6); // 5 assignments + 1 internal assignment (return statement)
            assert_eq!(unit.implementations[2].statements.len(), 5); // 5 assignments
        }

        #[test]
        fn get_and_set_retains_original_variable_blocks() {
            let source = r"
            FUNCTION_BLOCK fb
                PROPERTY_GET foo: DINT
                    VAR
                        a, b, c : DINT;
                    END_VAR
                END_PROPERTY

                PROPERTY_SET foo: DINT
                    VAR
                        d, e, f : DINT;
                    END_VAR
                END_PROPERTY
            END_FUNCTION_BLOCK
            ";

            let unit = lower(source);
            assert_eq!(unit.pous[1].name, "fb.__get_foo");
            insta::assert_debug_snapshot!(unit.pous[1].variable_blocks, @r#"
            [
                VariableBlock {
                    variables: [
                        Variable {
                            name: "a",
                            data_type: DataTypeReference {
                                referenced_type: "DINT",
                            },
                        },
                        Variable {
                            name: "b",
                            data_type: DataTypeReference {
                                referenced_type: "DINT",
                            },
                        },
                        Variable {
                            name: "c",
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
                    ],
                    variable_block_type: Local,
                },
            ]
            "#);

            assert_eq!(unit.pous[2].name, "fb.__set_foo");
            insta::assert_debug_snapshot!(unit.pous[2].variable_blocks, @r#"
            [
                VariableBlock {
                    variables: [
                        Variable {
                            name: "d",
                            data_type: DataTypeReference {
                                referenced_type: "DINT",
                            },
                        },
                        Variable {
                            name: "e",
                            data_type: DataTypeReference {
                                referenced_type: "DINT",
                            },
                        },
                        Variable {
                            name: "f",
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
                    ],
                    variable_block_type: Input(
                        ByVal,
                    ),
                },
            ]
            "#);
        }

        #[test]
        fn multiple_properties_defined_in_pou_are_lowered_to_methods() {
            let source = r"
            FUNCTION_BLOCK fb
                VAR
                    localPrivateVariable : DINT;
                END_VAR

                PROPERTY_GET foo: DINT END_PROPERTY
                PROPERTY_SET foo: DINT END_PROPERTY

                PROPERTY_GET bar: DINT END_PROPERTY
                PROPERTY_SET bar: DINT END_PROPERTY

                PROPERTY_GET baz: DINT END_PROPERTY
                PROPERTY_SET baz: DINT END_PROPERTY

                PROPERTY_GET qux: DINT END_PROPERTY
                PROPERTY_SET qux: DINT END_PROPERTY
            END_FUNCTION_BLOCK
            ";

            let unit = lower(source);

            // No need to snapshot test here, we did plenty before this one
            assert_eq!(unit.pous.len(), 9);
            assert_eq!(unit.pous[0].name, "fb");
            assert_eq!(unit.pous[1].name, "fb.__get_foo");
            assert_eq!(unit.pous[2].name, "fb.__set_foo");
            assert_eq!(unit.pous[3].name, "fb.__get_bar");
            assert_eq!(unit.pous[4].name, "fb.__set_bar");
            assert_eq!(unit.pous[5].name, "fb.__get_baz");
            assert_eq!(unit.pous[6].name, "fb.__set_baz");
            assert_eq!(unit.pous[7].name, "fb.__get_qux");
            assert_eq!(unit.pous[8].name, "fb.__set_qux");
        }

        #[test]
        fn property_self_assignment() {
            let source = r"
            FUNCTION_BLOCK fb
                PROPERTY_GET foo: DINT END_PROPERTY
                PROPERTY_SET foo: DINT END_PROPERTY

                foo := foo;
            END_FUNCTION_BLOCK
            ";

            let unit = super::lower(source);
            insta::assert_debug_snapshot!(unit.implementations[0].statements, @r#"
            [
                CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__set_foo",
                            },
                        ),
                        base: None,
                    },
                    parameters: Some(
                        CallStatement {
                            operator: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__get_foo",
                                    },
                                ),
                                base: None,
                            },
                            parameters: None,
                        },
                    ),
                },
            ]
            "#);
        }

        #[test]
        fn properties_in_interfaces_are_lowered() {
            let source = r"
            INTERFACE foo
                PROPERTY_GET bar: DINT END_PROPERTY
                PROPERTY_SET bar: DINT END_PROPERTY
            END_INTERFACE
            ";

            let (unit, _) = super::lower_properties_to_pous(source);

            // We retain the properties
            assert_eq!(unit.interfaces[0].properties.len(), 1);

            // ...but at the same time lower them into methods hosted by the interface
            assert_eq!(unit.interfaces[0].methods.len(), 2);
            insta::assert_debug_snapshot!(unit.interfaces[0].methods, @r#"
            [
                POU {
                    name: "foo.__get_bar",
                    variable_blocks: [
                        VariableBlock {
                            variables: [
                                Variable {
                                    name: "bar",
                                    data_type: DataTypeReference {
                                        referenced_type: "DINT",
                                    },
                                },
                            ],
                            variable_block_type: Local,
                        },
                    ],
                    pou_type: Method {
                        parent: "foo",
                        property: Some(
                            (
                                "bar",
                                Get,
                            ),
                        ),
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
                POU {
                    name: "foo.__set_bar",
                    variable_blocks: [
                        VariableBlock {
                            variables: [
                                Variable {
                                    name: "bar",
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
                        parent: "foo",
                        property: Some(
                            (
                                "bar",
                                Set,
                            ),
                        ),
                        declaration_kind: Concrete,
                    },
                    return_type: None,
                    interfaces: [],
                    properties: [],
                },
            ]
            "#);
        }
    }

    mod resolver {
        use plc_ast::{
            ast::{Assignment, AstNode, BinaryExpression, CallStatement, ReferenceAccess, ReferenceExpr},
            try_from,
        };

        use crate::resolver::{AnnotationMap, StatementAnnotation};

        #[test]
        fn properties_in_assignments_are_annotated() {
            let source = r"
            FUNCTION_BLOCK fb
                VAR
                    foo : DINT;
                END_VAR

                PROPERTY_GET myProp: DINT END_PROPERTY
                PROPERTY_SET myProp: DINT END_PROPERTY
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : fb;
                    tmp : DINT;
                END_VAR

                instance.myProp := 5;
                tmp := instance.myProp;
            END_FUNCTION
            ";

            let (unit, annotations) = super::lower_properties_to_pous(source);

            let implementation = &unit.implementations[1];
            let setter = &implementation.statements[0];
            let getter = &implementation.statements[1];

            let Assignment { left, .. } = try_from!(setter, Assignment).unwrap();
            assert_eq!(
                annotations.get(left).unwrap(),
                &StatementAnnotation::Property { name: "__set_myProp".to_string() }
            );

            let Assignment { right, .. } = try_from!(getter, Assignment).unwrap();
            assert_eq!(
                annotations.get(right).unwrap(),
                &StatementAnnotation::Property { name: "__get_myProp".to_string() }
            );
        }

        #[test]
        fn lone_reference_is_annotated() {
            let source = r"
            FUNCTION_BLOCK fb
                PROPERTY_GET foo: DINT
                END_PROPERTY
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : fb;
                END_VAR

                instance.foo;
            END_FUNCTION
            ";

            let (unit, annotations) = super::lower_properties_to_pous(source);
            assert_eq!(
                annotations.get(&unit.implementations[1].statements[0]),
                Some(&StatementAnnotation::Property { name: "__get_foo".to_string() })
            );
        }

        #[test]
        fn lone_reference_inside_declaring_container_is_annotated() {
            let source = r"
            FUNCTION_BLOCK A
                PROPERTY_GET sayCheese: DINT
                    printf('Cheese');
                END_PROPERTY

                sayCheese;
            END_FUNCTION_BLOCK
            ";

            let (unit, annotations) = super::lower_properties_to_pous(source);
            assert_eq!(
                annotations.get(&unit.implementations[0].statements[0]).unwrap(),
                &StatementAnnotation::Property { name: "__get_sayCheese".to_string() }
            );
        }

        #[test]
        fn reference_as_argument_is_annoated() {
            let source = r"
            FUNCTION func : DINT
                VAR_INPUT
                    a : STRING;
                END_VAR

                VAR_OUTPUT
                    b : STRING;
                END_VAR

                VAR_IN_OUT
                    c : STRING;
                END_VAR
            END_FUNCTION

            FUNCTION_BLOCK fb
                PROPERTY_GET foo: STRING END_PROPERTY
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : fb;
                END_VAR

                func(instance.foo, instance.foo, instance.foo);
                func(a := instance.foo, b => instance.foo, c := instance.foo);
            END_FUNCTION
            ";

            let (unit, annotations) = super::lower_properties_to_pous(source);
            let implementation = &unit.implementations[2];

            let reference = &implementation.statements[0];
            let CallStatement { parameters, .. } = try_from!(reference, CallStatement).unwrap();
            let parameters = try_from!(parameters.as_ref().unwrap(), Vec<AstNode>).unwrap();

            for param in parameters.iter().take(3) {
                assert_eq!(
                    annotations.get(param).unwrap(),
                    &StatementAnnotation::Property { name: "__get_foo".to_string() }
                );
            }

            let reference = &implementation.statements[1];
            let CallStatement { parameters, .. } = try_from!(reference, CallStatement).unwrap();
            let parameters = try_from!(parameters.as_ref().unwrap(), Vec<AstNode>).unwrap();

            for param in parameters.iter().take(3) {
                let Assignment { right, .. } = try_from!(param, Assignment).unwrap();
                assert_eq!(
                    annotations.get(right).unwrap(),
                    &StatementAnnotation::Property { name: "__get_foo".to_string() }
                );
            }
        }

        #[test]
        fn reference_as_vararg_argument_is_annotated() {
            let source = r"
            FUNCTION printf : DINT
                VAR_INPUT {ref}
                    format : STRING;
                END_VAR

                VAR_INPUT
                    args : ...;
                END_VAR
            END_FUNCTION

            FUNCTION_BLOCK fb
                PROPERTY_GET foo: DINT END_PROPERTY
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : fb;
                END_VAR

                printf('%d$N', instance.foo);
            END_FUNCTION
            ";

            let (unit, annotations) = super::lower_properties_to_pous(source);
            let implementation = &unit.implementations[2];
            let reference = &implementation.statements[0];

            let CallStatement { parameters, .. } = try_from!(reference, CallStatement).unwrap();
            let parameters = try_from!(parameters.as_ref().unwrap(), Vec<AstNode>).unwrap();

            assert_eq!(
                annotations.get(&parameters[1]).unwrap(),
                &StatementAnnotation::Property { name: "__get_foo".to_string() }
            );
        }

        #[test]
        fn reference_as_array_index_is_annotated() {
            let source = r"
            FUNCTION_BLOCK fb
                PROPERTY_GET foo: DINT END_PROPERTY
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : fb;
                    arr : ARRAY[1..5] OF DINT;
                END_VAR

                arr[instance.foo];
                arr[instance.foo + 1] := arr[instance.foo];
            END_FUNCTION
            ";

            let (unit, annotations) = super::lower_properties_to_pous(source);
            let implementation = &unit.implementations[1];
            let reference = &implementation.statements[0];

            // arr[instance.foo]
            //     ^^^^^^^^^^^^
            let ReferenceExpr { access, .. } = try_from!(reference, ReferenceExpr).unwrap();
            let ReferenceAccess::Index(index) = access else { unreachable!() };
            let ReferenceExpr { access, .. } = try_from!(index, ReferenceExpr).unwrap();
            let ReferenceAccess::Member(ident) = access else { unreachable!() };

            assert_eq!(
                annotations.get(ident).unwrap(),
                &StatementAnnotation::Property { name: "__get_foo".to_string() }
            );

            // arr[instance.foo + 1] := arr[instance.foo]
            //     ^^^^^^^^^^^^
            let Assignment { left, right } = try_from!(implementation.statements[1], Assignment).unwrap();
            let ReferenceExpr { access, .. } = try_from!(left, ReferenceExpr).unwrap();
            let ReferenceAccess::Index(index) = access else { unreachable!() };
            let BinaryExpression { left, .. } = try_from!(index, BinaryExpression).unwrap();
            let ReferenceExpr { access, .. } = try_from!(left, ReferenceExpr).unwrap();
            let ReferenceAccess::Member(ident) = access else { unreachable!() };

            assert_eq!(
                annotations.get(ident).unwrap(),
                &StatementAnnotation::Property { name: "__get_foo".to_string() }
            );

            // arr[instance.foo + 1] := arr[instance.foo]
            //                              ^^^^^^^^^^^^
            let ReferenceExpr { access, .. } = try_from!(right, ReferenceExpr).unwrap();
            let ReferenceAccess::Index(index) = access else { unreachable!() };
            let ReferenceExpr { access, .. } = try_from!(index, ReferenceExpr).unwrap();
            let ReferenceAccess::Member(ident) = access else { unreachable!() };

            assert_eq!(
                annotations.get(ident).unwrap(),
                &StatementAnnotation::Property { name: "__get_foo".to_string() }
            );
        }

        #[test]
        fn reference_as_argument_as_array_index_is_annotated() {
            let source = r"
            FUNCTION_BLOCK fb
                PROPERTY_GET foo: ARRAY[1..5] OF STRING
                    foo := ['a', 'b', 'c', 'd', 'e'];
                END_PROPERTY

                PROPERTY_GET bar: DINT
                    bar := 5;
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

            let (unit, annotations) = super::lower_properties_to_pous(source);
            let printf = &unit.implementations[1].statements[0];

            // printf(...)
            let CallStatement { parameters, .. } = try_from!(printf, CallStatement).unwrap();
            let arguments = try_from!(parameters.as_ref().unwrap(), Vec<AstNode>).unwrap();

            // REF(instance.foo[instance.bar])
            //     ^^^^^^^^^^^^^^^^^^^^^^^^^^
            let CallStatement { parameters, .. } = try_from!(arguments[1], CallStatement).unwrap();
            let ReferenceExpr { access, base } =
                try_from!(parameters.as_ref().unwrap(), ReferenceExpr).unwrap();

            // instance.foo[instance.bar]
            // ^^^^^^^^^^^^
            let ReferenceExpr { access: access_lhs, .. } =
                try_from!(base.as_ref().unwrap(), ReferenceExpr).unwrap();
            let ReferenceAccess::Member(ident) = access_lhs else { unreachable!() };
            assert_eq!(
                annotations.get(ident).unwrap(),
                &StatementAnnotation::Property { name: "__get_foo".to_string() }
            );

            // instance.foo[instance.bar]
            //              ^^^^^^^^^^^^
            let ReferenceAccess::Index(index) = access else { unreachable!() };
            let ReferenceExpr { access, .. } = try_from!(index, ReferenceExpr).unwrap();
            let ReferenceAccess::Member(ident) = access else { unreachable!() };
            assert_eq!(
                annotations.get(ident).unwrap(),
                &StatementAnnotation::Property { name: "__get_bar".to_string() }
            );
        }

        #[test]
        fn property_variable_is_not_lowered_inside_own_block() {
            let source = r"
            FUNCTION_BLOCK fb
                PROPERTY_GET foo: DINT
                    // This should not be expanded into `__get_foo()`
                    foo;

                    // Similarly this should not be expanded into `__set_foo(5)`
                    foo := 5;
                END_PROPERTY

                PROPERTY_SET foo: DINT
                    // Same as above
                    foo;
                    foo := 5;
                END_PROPERTY
            END_FUNCTION_BLOCK
            ";

            let (unit, _) = super::lower_properties_to_pous(source);

            let get = &unit.implementations[1];
            insta::assert_debug_snapshot!(get.statements[0..2], @r#"
            [
                ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "foo",
                        },
                    ),
                    base: None,
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
                    right: LiteralInteger {
                        value: 5,
                    },
                },
            ]
            "#);

            let set = &unit.implementations[2];
            insta::assert_debug_snapshot!(set.statements, @r#"
            [
                ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "foo",
                        },
                    ),
                    base: None,
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
                    right: LiteralInteger {
                        value: 5,
                    },
                },
            ]
            "#);
        }

        #[test]
        fn property_cross_referencing() {
            let source = r"
            FUNCTION_BLOCK fb
                PROPERTY_GET foo: DINT
                END_PROPERTY

                PROPERTY_GET bar: DINT
                    foo;
                END_PROPERTY
            END_FUNCTION_BLOCK
            ";

            let (unit, annotations) = super::lower_properties_to_pous(source);
            let reference = &unit.implementations[2].statements[0];
            assert_eq!(
                annotations.get(reference).unwrap(),
                &StatementAnnotation::Property { name: "__get_foo".to_string() }
            );
        }

        #[test]
        fn property_in_action() {
            let source = r"
            FUNCTION_BLOCK fb
                PROPERTY_GET foo: DINT END_PROPERTY
                PROPERTY_SET foo: DINT END_PROPERTY
            END_FUNCTION_BLOCK

            ACTION fb.act
                foo := foo;
            END_ACTION

            ";

            let (unit, annotations) = super::lower_properties_to_pous(source);
            let statement = &unit.implementations[1].statements[0];
            let Assignment { left, right } = try_from!(statement, Assignment).unwrap();
            assert_eq!(
                annotations.get(left).unwrap(),
                &StatementAnnotation::Property { name: "__set_foo".to_string() }
            );

            assert_eq!(
                annotations.get(right).unwrap(),
                &StatementAnnotation::Property { name: "__get_foo".to_string() }
            )
        }

        #[test]
        fn inherited_property_through_child_interface_is_annotated() {
            let source = r"
            INTERFACE IA
                PROPERTY_GET value: DINT END_PROPERTY
                PROPERTY_SET value: DINT END_PROPERTY
            END_INTERFACE

            INTERFACE IB EXTENDS IA
                METHOD describe END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IB
                VAR
                    _val : DINT := 50;
                END_VAR

                PROPERTY_GET value: DINT
                    value := _val;
                END_PROPERTY

                PROPERTY_SET value: DINT
                    _val := value;
                END_PROPERTY

                METHOD describe END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : FbA;
                    refA : IA;
                    refB : IB;
                END_VAR

                refA := instance;
                refA.value;

                refB := instance;
                refB.value;
            END_FUNCTION
            ";

            let (unit, annotations) = super::lower_properties_to_pous(source);
            let statements = &unit.implementations.iter().find(|imp| imp.name == "main").unwrap().statements;

            // refA.value — property on the declaring interface
            assert_eq!(
                annotations.get(&statements[1]).unwrap(),
                &StatementAnnotation::Property { name: "__get_value".to_string() }
            );

            // refB.value — property inherited through child interface
            assert_eq!(
                annotations.get(&statements[3]).unwrap(),
                &StatementAnnotation::Property { name: "__get_value".to_string() }
            );
        }
    }
}
