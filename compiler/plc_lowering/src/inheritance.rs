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

    fn old_update_inheritance_chain(&mut self, mut node: AstNode) -> AstNode {
        let Some(index) = self.index.as_ref() else {
            // TODO: this will skip visiting initializer nodes
            return node;
        };
        let annotations = self.annotations.as_ref().expect("Annotations not set");
        let ident = node.get_flat_reference_name().expect("Identifier").to_string();
        let container = self.ctx.pou.as_ref().expect("Implementation must be in POU context").to_string();

        if index.find_local_member(&container, &ident).is_some() {
            // this is a local reference to the current container
            return node;
        }

        let expr = try_from_mut!(node, ReferenceExpr).expect("ReferenceExpr");
        // TODO: member-access only? do we need to consider other cases (array-access, ...)?
        let ReferenceExpr { ref mut base, access: ReferenceAccess::Member(access) } = expr else {
            return node;
        };

        // if the member is not found in the current POU, we need to check the qualifier for a base class
        let Some(super_) = base
            .as_ref()
            .and_then(|it| {
                let qualifier = annotations.get_type_or_void(&*it, index).get_name();
                index.find_pou(&qualifier).and_then(PouIndexEntry::get_super_class)
            })
            .or_else(||
            // if we don't have a qualifier, check the current container for a base class
            index.find_pou(&container).and_then(PouIndexEntry::get_super_class))
        else {
            // the reference is neither in our current container's family-tree, nor is it part of another family.
            // nothing to do here
            return node;
        };

        let access = std::mem::take(access);
        let base: Option<Box<AstNode>> = std::mem::take(base);
        // update the base of the visited `ReferenceExpr`
        let base = AstFactory::create_member_reference(
            AstFactory::create_identifier(
                "__BASE",
                SourceLocation::internal(),
                self.ctx.provider().next_id(),
            ),
            base.map(|it| *it),
            self.ctx.provider().next_id(),
        );
        let node = AstFactory::create_member_reference(*access, Some(base), self.ctx.provider().next_id());
        // traverse and update the reference's lineage recursively
        self.ctx = self.ctx.with_pou(super_);
        let node = self.old_update_inheritance_chain(node);
        // reset the context to the original one
        self.ctx = self.ctx.with_pou(container);
        node
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

        // TODO: member-access only? do we need to consider other cases (array-access, ...)?
        let expr = try_from_mut!(node, ReferenceExpr).expect("ReferenceExpr");
        let ReferenceExpr { ref mut base, access: ReferenceAccess::Member(access) } = expr else {
            return node;
        };

        let base = std::mem::take(base);
        let (base, ty) = if let Some(base) = base {
            let ty = annotations.get_type(&*base, index);

            (Some(Box::new(self.update_inheritance_chain(*base))), ty)
        } else {
            (base, self.ctx.pou.as_ref().and_then(|it| index.get_type(&it).ok()))
        };

        let access = *std::mem::take(access);

        let qualified_name = annotations.get_qualified_name(&access).expect("QualifiedName"); // TODO: error handling/early exit

        let segment = qualified_name.split('.').next().expect("Must have a name");

        if ty.is_some_and(|it| it.get_name() == segment) {
            // reference was flat reference, just return access
            dbg!("returning early: segment == qualified name");
            dbg!(ty, segment);
            return dbg!(access);
        }

        let inheritance_chain = index.find_ancestors(ty.map(|it| it.get_name()).unwrap_or_default(), segment);
        if inheritance_chain.len() <= 1 {
            dbg!("returning early: inheritance_chain <= 1");
            dbg!(&inheritance_chain, segment, ty);
            return dbg!(access);
        }

        // add a `__BASE` qualifier for each element in the inheritance chain, exluding `self`
        let base = inheritance_chain.iter().skip(1).fold(base, |base, _| {
            // update the base of the visited `ReferenceExpr`
            // let Some(base) = base else {
            //     return Some(Box::new(AstFactory::create_identifier(
            //         "__BASE",
            //         SourceLocation::internal(),
            //         self.ctx.provider().next_id(),
            //     )));
            // };

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
                        line: 15,
                        column: 16,
                        offset: 347,
                    }..TextLocation {
                        line: 15,
                        column: 28,
                        offset: 359,
                    },
                ),
            },
            name_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 11,
                        column: 27,
                        offset: 262,
                    }..TextLocation {
                        line: 11,
                        column: 30,
                        offset: 265,
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
                        offset: 189,
                    }..TextLocation {
                        line: 8,
                        column: 29,
                        offset: 202,
                    },
                ),
            },
            name_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 7,
                        column: 27,
                        offset: 156,
                    }..TextLocation {
                        line: 7,
                        column: 30,
                        offset: 159,
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
    fn foo() {
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
                // BASE__.myFb.__BASE.x
            END_FUNCTION_BLOCK
        "
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[3];
        assert_debug_snapshot!(unit, @r#""#);
    }
}
