use plc::{index::Index, resolver::{AnnotationMap, AstAnnotations}};
use plc_ast::{
    ast::{
        AstFactory, AstNode, CompilationUnit, DataTypeDeclaration, LinkageType, Pou, PouType, ReferenceAccess, ReferenceExpr, Variable, VariableBlock, VariableBlockType
    }, mut_visitor::{AstVisitorMut, WalkerMut}, provider::IdProvider, try_from_mut
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
        Self { base_type_name: Some(base_type_name.into()), pou: self.pou.clone(), id_provider: self.id_provider.clone() }
    }

    fn with_pou(&self, pou: impl Into<String>) -> Self {
        Self { base_type_name: self.base_type_name.clone(), pou: Some(pou.into()), id_provider: self.id_provider.clone() }
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
        Self {
            index: None,
            annotations: None,
            ctx: Context::new(id_provider),
        }
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
     
    fn check_heritage(&self, mut node: AstNode) -> AstNode {
        let Some(index) = self.index.as_ref() else {
            // TODO: this will skip visiting initializer nodes
            return node;
        };
        let annotations = self.annotations.as_ref().expect("Annotations not set");
        let ident = node.get_flat_reference_name().expect("Identifier").to_string();
        let container = self.ctx.pou.as_ref().expect("Implementation must be in POU context").as_str();

        let is_local = index.find_local_member(container, &ident).is_some();
        if is_local {
            return node;
        }

        let expr = try_from_mut!(node, ReferenceExpr).expect("ReferenceExpr");
        // TODO: member-access only? do we need to consider arrays etc?
        let ReferenceExpr { ref mut base, .. } = expr;
        let mut qualifier = base.as_ref().map(|it| annotations.get_type_or_void(&*it, index).get_name()).unwrap_or_default();
        // should we do this recursively instead? nobody is going to derive so many children that we risk overflowing the stack, right? ..right?
        while let Some(parent) = index.find_local_member(qualifier,  "__BASE") {
            // update the base of the visited `ReferenceExpr`
            let original_base = std::mem::take(base);
    
            let new_base = AstFactory::create_member_reference(
                AstFactory::create_identifier(
                    "__BASE", 
                    SourceLocation::internal(), 
                    self.ctx.provider().next_id()
                ), 
                original_base.map(|it| *it), 
                self.ctx.provider().next_id()
            );     
            *base = Some(Box::new(new_base));
            
            // check if our variable is a member of the currently checked generation. if so, we are done
            qualifier = parent.get_type_name();
            if index.find_local_member(qualifier, &ident).is_some() {
                break;
            }
        }
        
        // AstFactory::create_member_reference(std::mem::take(member), base, self.ctx.provider().next_id())
        dbg!(node)
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
        let ctx = if let Some(base) = self.index.as_ref().and_then(|it| it.find_pou(&implementation.type_name).and_then(|it| it.get_super_class())) {
            ctx.with_base(base)
        } else {
            ctx
        };
        self.walk_with_context(implementation, ctx);
    }

    fn visit_reference_expr(&mut self, node: &mut AstNode) {
        // If the reference is to a member of the base class, we need to add a reference to the
        // base class
        dbg!("checking heritage");
        let new_node = self.check_heritage(std::mem::take(node));
        *node = new_node;
        dbg!(&node);
        let expr = try_from_mut!(node, ReferenceExpr).expect("ReferenceExpr");
        expr.walk(self);
    }   
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use plc_driver::parse_and_annotate;
    use plc_source::SourceCode;

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
            END_VAR
                myFb.x := 1;
                // myFb.__BASE.x
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
                            Identifier {
                                name: "z",
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
                                    ),
                                ),
                            },
                        ),
                    },
                    right: LiteralInteger {
                        value: 42,
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
}
