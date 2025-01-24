use plc::{
    index::{Index, PouIndexEntry},
    resolver::{AnnotationMap, AstAnnotations, StatementAnnotation},
};
use plc_ast::{
    ast::{
        AstFactory, AstNode, CompilationUnit, DataTypeDeclaration, LinkageType, Pou, PouType,
        ReferenceAccess, ReferenceExpr, Variable, VariableBlock, VariableBlockType,
    },
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
    try_from_mut,
};
use plc_source::source_location::SourceLocation;

#[derive(Debug)]
struct Context {
    base_type_name: Option<String>,
    pou: Option<String>,
    id_provider: IdProvider,
}

impl Context {
    fn new(id_provider: IdProvider) -> Self {
        Self { base_type_name: None, pou: None, id_provider }
    }

    fn with_base(&self, base_type_name: impl Into<String>) -> Self {
        Self {
            base_type_name: Some(base_type_name.into()),
            pou: self.pou.clone(),
            id_provider: self.id_provider.clone(),
        }
    }

    fn with_pou(&self, pou: impl Into<String>) -> Self {
        Self {
            base_type_name: self.base_type_name.clone(),
            pou: Some(pou.into()),
            id_provider: self.id_provider.clone(),
        }
    }

    fn provider(&self) -> IdProvider {
        self.id_provider.clone()
    }
}

pub struct InheritanceLowerer {
    pub index: Option<Index>,
    pub annotations: Option<AstAnnotations>,
    ctx: Context,
}

impl InheritanceLowerer {
    pub fn new(id_provider: IdProvider) -> Self {
        Self { index: None, annotations: None, ctx: Context::new(id_provider) }
    }

    pub fn visit_unit(&mut self, unit: &mut CompilationUnit) {
        // XXX: not sure if we need to visit global vars and user-data-types when walking the compilation unit here
        // for pou in unit.units.iter_mut() {
        //     pou.walk(self);
        // }
        // for implementation in unit.implementations.iter_mut() {
        //     implementation.walk(self);
        // }
        self.visit_compilation_unit(unit);
    }

    fn walk_with_context<T: WalkerMut>(&mut self, t: &mut T, ctx: Context) {
        let old_ctx = std::mem::replace(&mut self.ctx, ctx);
        t.walk(self);
        self.ctx = old_ctx;
    }

    fn update_inheritance_chain(&self, mut node: AstNode) -> AstNode {
        let Some(index) = self.index.as_ref() else {
            // TODO: this will skip visiting initializer nodes
            return node;
        };
        let annotations = self.annotations.as_ref().expect("Annotations not set");
        // disect the qualified name
        // a.b.c -> am i a direct member of b?
        //      b => am I a direct member of a?
        //      a => am I a direct member of local container?

        let Some(ReferenceExpr { base, access }) = try_from_mut!(node, ReferenceExpr) else {
            return node;
        };

        let access = match access {
            ReferenceAccess::Member(ast_node)
            | ReferenceAccess::Index(ast_node)
            | ReferenceAccess::Cast(ast_node) => self.update_inheritance_chain(*std::mem::take(ast_node)),
            ReferenceAccess::Deref | ReferenceAccess::Address => {
                return node;
            }
        };

        let base = std::mem::take(base);
        let (base, ty) = if let Some(base) = base {
            let ty = annotations.get_type(&*base, index);

            (Some(Box::new(self.update_inheritance_chain(*base))), ty)
        } else {
            (base, self.ctx.pou.as_ref().and_then(|it| index.get_type(&it).ok()))
        };

        let qualified_name = annotations.get_qualified_name(&access).expect("QualifiedName"); // TODO: error handling/early exit

        let segment = qualified_name.split('.').next().expect("Must have a name");

        if ty.is_some_and(|it| it.get_name() == segment) {
            // reference was flat reference, just return access
            return AstFactory::create_member_reference(
                access,
                base.map(|it| *it),
                self.ctx.provider().next_id(),
            );
        }

        let inheritance_chain = index.find_ancestors(ty.map(|it| it.get_name()).unwrap_or_default(), segment);
        if inheritance_chain.len() <= 1 {
            return AstFactory::create_member_reference(
                access,
                base.map(|it| *it),
                self.ctx.provider().next_id(),
            );
        }

        // add a `__BASE` qualifier for each element in the inheritance chain, exluding `self`
        let base = inheritance_chain.iter().skip(1).fold(base, |base, _| {
            Some(Box::new(AstFactory::create_member_reference(
                AstFactory::create_identifier(
                    "__BASE",
                    SourceLocation::internal(),
                    self.ctx.provider().next_id(),
                ),
                base.map(|it| *it),
                self.ctx.provider().next_id(),
            )))
        });

        AstFactory::create_member_reference(access, base.map(|it| *it), self.ctx.provider().next_id())
    }
}

impl AstVisitorMut for InheritanceLowerer {
    fn visit_pou(&mut self, pou: &mut Pou) {
        if self.index.is_some() {
            return self.walk_with_context(pou, self.ctx.with_pou(&pou.name));
        }
        if !matches!(pou.kind, PouType::FunctionBlock | PouType::Class) {
            return;
        }

        let Some(base_name) = pou.super_class.as_ref() else {
            return;
        };

        let base_var = Variable {
            name: "__BASE".to_string(),
            data_type_declaration: DataTypeDeclaration::DataTypeReference {
                referenced_type: base_name.into(),
                location: SourceLocation::internal(),
            },
            location: SourceLocation::internal(),
            initializer: None,
            address: None,
        };

        let block = VariableBlock {
            variables: vec![base_var],
            variable_block_type: VariableBlockType::Base,
            linkage: LinkageType::Internal,
            location: SourceLocation::internal(),
            ..Default::default()
        };

        pou.variable_blocks.insert(0, block);
        self.walk_with_context(pou, self.ctx.with_pou(&pou.name));
    }

    fn visit_implementation(&mut self, implementation: &mut plc_ast::ast::Implementation) {
        //Only go through the implementation if we have the index and annotations
        if self.index.is_none() || self.annotations.is_none() {
            return;
        }

        let ctx = self.ctx.with_pou(&implementation.type_name);
        let ctx = if let Some(base) = self
            .index
            .as_ref()
            .and_then(|it| it.find_pou(&implementation.type_name).and_then(|it| it.get_super_class()))
        {
            ctx.with_base(base)
        } else {
            ctx
        };
        self.walk_with_context(implementation, ctx);
    }

    fn visit_reference_expr(&mut self, node: &mut AstNode) {
        // If the reference is to a member of the base class, we need to add a reference to the
        // base class
        let new_node = self.update_inheritance_chain(std::mem::take(node));
        *node = new_node;
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use plc_driver::parse_and_annotate;
    use plc_source::SourceCode;

    // FIXME: `cargo insta review` doesn't pick up these snapshots for review
    #[test]
    fn after_parsing_a_function_block_contains_ref_to_its_base() {
        let src: SourceCode = "
        FUNCTION_BLOCK foo
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar EXTENDS foo
        END_FUNCTION_BLOCK
        "
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().units[1];
        assert_debug_snapshot!(unit, @r#"
        POU {
            name: "bar",
            variable_blocks: [
                VariableBlock {
                    variables: [
                        Variable {
                            name: "__BASE",
                            data_type: DataTypeReference {
                                referenced_type: "foo",
                            },
                        },
                    ],
                    variable_block_type: Base,
                },
            ],
            pou_type: FunctionBlock,
            return_type: None,
            interfaces: [],
        }
        "#);
    }

    #[test]
    fn write_to_parent_variable_qualified_access() {
        let src: SourceCode = "
            FUNCTION_BLOCK fb
            VAR
                x : INT;
                y : INT;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK fb2 EXTENDS fb
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK foo
            VAR
                myFb : fb2;
                // x : DINT;
            END_VAR
                myFb.x := 1;
            END_FUNCTION_BLOCK
        "
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[2];
        assert_debug_snapshot!(unit, @r#"
        Implementation {
            name: "foo",
            type_name: "foo",
            linkage: Internal,
            pou_type: FunctionBlock,
            statements: [
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__BASE",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "myFb",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
                            },
                        ),
                    },
                    right: LiteralInteger {
                        value: 1,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 16,
                        column: 16,
                        offset: 363,
                    }..TextLocation {
                        line: 16,
                        column: 28,
                        offset: 375,
                    },
                ),
            },
            name_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 11,
                        column: 27,
                        offset: 250,
                    }..TextLocation {
                        line: 11,
                        column: 30,
                        offset: 253,
                    },
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "#);
    }

    #[test]
    fn write_to_parent_variable_in_instance() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK foo
            VAR
                s : STRING;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK bar EXTENDS foo
                s := 'world';
            END_FUNCTION_BLOCK
        "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[1];
        assert_debug_snapshot!(unit, @r#"
        Implementation {
            name: "bar",
            type_name: "bar",
            linkage: Internal,
            pou_type: FunctionBlock,
            statements: [
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "s",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__BASE",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                    right: LiteralString {
                        value: "world",
                        is_wide: false,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 8,
                        column: 16,
                        offset: 187,
                    }..TextLocation {
                        line: 8,
                        column: 29,
                        offset: 200,
                    },
                ),
            },
            name_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 7,
                        column: 27,
                        offset: 155,
                    }..TextLocation {
                        line: 7,
                        column: 30,
                        offset: 158,
                    },
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "#);
    }

    #[test]
    fn write_to_grandparent_variable_in_initializer() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK grandparent
            VAR
                z : INT;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK parent EXTENDS grandparent
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK child EXTENDS parent
                z := 42;
            END_FUNCTION_BLOCK
        "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[2];
        assert_debug_snapshot!(unit);
    }

    #[test]
    fn test_array_access_in_nested_function_blocks_with_base_references() {
        let src: SourceCode = r#"
                FUNCTION_BLOCK grandparent
                VAR
                    y : ARRAY[0..5] OF INT;
                    a : INT;
                END_VAR
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK parent extends grandparent
                    VAR
                        x : ARRAY[0..10] OF INT;
                        b : INT;
                    END_VAR
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK child EXTENDS parent
                    VAR
                        z : ARRAY[0..10] OF INT;
                    END_VAR
                    x[0] := 42; //__BASE.x[0] := 42;
                    y[2]:= 5; //__BASE.__BASE.y[2] := 5;
                    z[3] := x[1] + y[2]; //z[3] := __BASE.x[1] + __BASE.__BASE.y[2];
                    x[a] := 5; //__BASE.x[__BASE__.BASE__.a] := 5;
                    y[b] := 6; //__BASE.__BASE.y[__BASE.b] := 6;
                    z[a+b] := 10; //z[__BASE.__BASE.a + __BASE.b] := 10;
                END_FUNCTION_BLOCK
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[2];
        assert_debug_snapshot!(unit, @r#"
        Implementation {
            name: "child",
            type_name: "child",
            linkage: Internal,
            pou_type: FunctionBlock,
            statements: [
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            LiteralInteger {
                                value: 0,
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "x",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__BASE",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
                            },
                        ),
                    },
                    right: LiteralInteger {
                        value: 42,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            LiteralInteger {
                                value: 2,
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "y",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__BASE",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__BASE",
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
                    right: LiteralInteger {
                        value: 5,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            LiteralInteger {
                                value: 3,
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "z",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                    right: BinaryExpression {
                        operator: Plus,
                        left: ReferenceExpr {
                            kind: Member(
                                LiteralInteger {
                                    value: 1,
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "x",
                                        },
                                    ),
                                    base: Some(
                                        ReferenceExpr {
                                            kind: Member(
                                                Identifier {
                                                    name: "__BASE",
                                                },
                                            ),
                                            base: None,
                                        },
                                    ),
                                },
                            ),
                        },
                        right: ReferenceExpr {
                            kind: Member(
                                LiteralInteger {
                                    value: 2,
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "y",
                                        },
                                    ),
                                    base: Some(
                                        ReferenceExpr {
                                            kind: Member(
                                                Identifier {
                                                    name: "__BASE",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "__BASE",
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
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "a",
                                    },
                                ),
                                base: None,
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "x",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__BASE",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
                            },
                        ),
                    },
                    right: LiteralInteger {
                        value: 5,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "b",
                                    },
                                ),
                                base: None,
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "y",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__BASE",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__BASE",
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
                    right: LiteralInteger {
                        value: 6,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            BinaryExpression {
                                operator: Plus,
                                left: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "a",
                                        },
                                    ),
                                    base: None,
                                },
                                right: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "b",
                                        },
                                    ),
                                    base: None,
                                },
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "z",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                    right: LiteralInteger {
                        value: 10,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 19,
                        column: 20,
                        offset: 598,
                    }..TextLocation {
                        line: 24,
                        column: 33,
                        offset: 868,
                    },
                ),
            },
            name_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 15,
                        column: 31,
                        offset: 456,
                    }..TextLocation {
                        line: 15,
                        column: 36,
                        offset: 461,
                    },
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "#);
    }

    #[test]
    fn test_multi_level_reference_handling() {
        let src: SourceCode = "
            FUNCTION_BLOCK fb
            VAR
                x : INT;
                y : INT;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK fb2 EXTENDS fb
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK baz
            VAR
                myFb : fb2;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK foo EXTENDS baz
            VAR
                x : INT;
            END_VAR
                myFb.x := 1;
                // __BASE.myFb.__BASE.x
            END_FUNCTION_BLOCK
        "
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[3];
        assert_debug_snapshot!(unit, @r#"
        Implementation {
            name: "foo",
            type_name: "foo",
            linkage: Internal,
            pou_type: FunctionBlock,
            statements: [
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__BASE",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "myFb",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__BASE",
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
                    right: LiteralInteger {
                        value: 1,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 21,
                        column: 16,
                        offset: 470,
                    }..TextLocation {
                        line: 21,
                        column: 28,
                        offset: 482,
                    },
                ),
            },
            name_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 17,
                        column: 27,
                        offset: 377,
                    }..TextLocation {
                        line: 17,
                        column: 30,
                        offset: 380,
                    },
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "#);
    }
}
