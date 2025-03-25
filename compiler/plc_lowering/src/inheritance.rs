//! This module is responsible for lowering inherited references.
//! Derived POUs can access members of their base class using the `__SUPER` qualifier,
//! which is an implicit reference to an instance of the base class which
//! is added as a member variable of the derived class in the 'pre_index' pass.
//!
//! For example, given the following code:
//! ```iec61131st
//! FUNCTION_BLOCK foo
//! END_FUNCTION_BLOCK
//!
//! FUNCTION_BLOCK bar EXTENDS foo
//! END_FUNCTION_BLOCK
//! ```
//! After the `pre-index` pass, the `bar`-function block will be transformed to:
//! ```iec61131st
//! FUNCTION_BLOCK bar
//! VAR
//!   __foo : foo;
//! END_VAR
//! END_FUNCTION_BLOCK
//! ```
//!
//! During the `post_annotate` pass, the inheritance hierarchy is resolved and the `__SUPER` references are added to
//! each `ReferenceExpression` that references a member of the base class.
//!
//! For example, given the following code:
//! ```iec61131st
//! FUNCTION_BLOCK foo
//! VAR
//!     x: INT;
//! END_VAR
//! END_FUNCTION_BLOCK
//!
//! FUNCTION_BLOCK bar EXTENDS foo
//! END_FUNCTION_BLOCK
//!
//! FUNCTION_BLOCK baz
//! VAR
//!    myFb : bar;
//! END_VAR
//!     myFb.x := 1;
//! END_FUNCTION_BLOCK
//! ```
//! After the `post_annotate` pass, the `baz`-function block will be transformed to:
//! ```iec61131st
//! FUNCTION_BLOCK baz
//! VAR
//!   myFb : bar;
//! END_VAR
//!    myFb.__foo.x := 1;
//! END_FUNCTION_BLOCK
//! ```

use plc::{index::Index, lowering::create_call_statement, resolver::AnnotationMap};
use plc_ast::{
    ast::{
        AstFactory, AstNode, AstStatement, CompilationUnit, DataTypeDeclaration, LinkageType, Pou, PouType,
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

    fn try_with_base(&self, implementation_name: &str, index: &Index) -> Option<Self> {
        index
            .find_pou(implementation_name)
            .and_then(|it| it.get_super_class())
            .map(|base| self.with_base(base))
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
    pub annotations: Option<Box<dyn AnnotationMap>>,
    ctx: Context,
}

impl InheritanceLowerer {
    pub fn new(id_provider: IdProvider) -> Self {
        Self { index: None, annotations: None, ctx: Context::new(id_provider) }
    }

    pub fn provider(&self) -> IdProvider {
        self.ctx.provider()
    }

    pub fn visit_unit(&mut self, unit: &mut CompilationUnit) {
        // before we start walking the unit, we need to lower the `super` keyword to the corresponding `ReferenceExpr`
        let mut super_lowerer =
            SuperKeywordLowerer::new(self.provider(), self.index.as_ref(), self.annotations.as_mut());
        super_lowerer.visit_compilation_unit(unit);
        self.visit_compilation_unit(unit);
    }

    fn walk_with_context<T: WalkerMut>(&mut self, t: &mut T, ctx: Context) {
        let old_ctx = std::mem::replace(&mut self.ctx, ctx);
        t.walk(self);
        self.ctx = old_ctx;
    }

    // Updates the base to reflect the inheritance chain of the node and returns it
    fn update_inheritance_chain(
        &self,
        node: &AstNode,
        base: Option<Box<AstNode>>,
        base_type: &str,
    ) -> Option<Box<AstNode>> {
        if self.index.is_none() || self.annotations.is_none() {
            return base;
        }

        let annotations = self.annotations.as_ref().expect("Annotations exist");
        let Some(qualified_name) = annotations.get_qualified_name(node) else {
            return base;
        };

        let segment = qualified_name.split('.').next().expect("Must have a name");

        if base_type == segment {
            return base;
        }

        let index = self.index.as_ref().expect("Index exists");
        let inheritance_chain = index.get_inheritance_chain(base_type, segment);
        if inheritance_chain.len() <= 1 {
            return base;
        }

        // add a `__SUPER` qualifier for each element in the inheritance chain, exluding `self`
        inheritance_chain.iter().rev().skip(1).fold(base, |base, pou| {
            Some(Box::new(AstFactory::create_member_reference(
                AstFactory::create_identifier(
                    format!("__{}", pou.get_name()),
                    SourceLocation::internal(),
                    self.provider().next_id(),
                ),
                base.map(|it: Box<AstNode>| *it),
                self.provider().next_id(),
            )))
        })
    }
}

impl AstVisitorMut for InheritanceLowerer {
    fn visit_pou(&mut self, pou: &mut Pou) {
        if self.index.is_some() {
            // methods need to be walked in the context of its container
            let pou_name = if let PouType::Method { parent, .. } = &pou.kind { parent } else { &pou.name };
            return self.walk_with_context(pou, self.ctx.with_pou(pou_name));
        }
        if !matches!(pou.kind, PouType::FunctionBlock | PouType::Class) {
            return;
        }

        let Some(base_name) = pou.super_class.as_ref() else {
            return;
        };

        let base_var = Variable {
            name: format!("__{}", &base_name.name),
            data_type_declaration: DataTypeDeclaration::Reference {
                referenced_type: base_name.name.clone(),
                location: SourceLocation::internal(),
            },
            location: SourceLocation::internal(),
            initializer: None,
            address: None,
        };

        let block = VariableBlock {
            variables: vec![base_var],
            kind: VariableBlockType::Local,
            linkage: LinkageType::Internal,
            location: SourceLocation::internal(),
            ..Default::default()
        };

        // To ensure that the `__SUPER` variable is consistently the first variable in the struct,
        // we insert it as the first variable block. This simplifies writing FFI bindings.
        pou.variable_blocks.insert(0, block);
        self.walk_with_context(pou, self.ctx.with_pou(&pou.name));
    }

    fn visit_implementation(&mut self, implementation: &mut plc_ast::ast::Implementation) {
        //Only go through the implementation if we have the index and annotations
        if self.index.is_none() || self.annotations.is_none() {
            return;
        }

        let type_name = if let PouType::Method { parent, .. } = &implementation.pou_type {
            parent
        } else {
            &implementation.type_name
        };
        let ctx = self.ctx.with_pou(type_name);
        let ctx = self
            .index
            .as_ref()
            .and_then(|it| ctx.try_with_base(&implementation.type_name, it))
            .unwrap_or(ctx);

        self.walk_with_context(implementation, ctx);
    }

    fn visit_reference_expr(&mut self, node: &mut AstNode) {
        if self.index.is_none() || self.annotations.is_none() {
            return;
        }
        // Find the base type of the expression before walking the node
        let base_type_name =
            if let AstStatement::ReferenceExpr(ReferenceExpr { base: Some(base), .. }) = node.get_stmt() {
                let index = self.index.as_ref().expect("Index exists");
                let annotations = self.annotations.as_ref().expect("Annotations exist");
                annotations.get_type(base, index).map(|it| it.get_name().to_string())
            } else {
                self.ctx.pou.clone()
            };
        // First walk the statement itself so we make sure any base is correctly added
        let stmt = try_from_mut!(node, ReferenceExpr).expect("ReferenceExpr");
        stmt.walk(self);
        // If the reference is to a member of the base class, we need to add a reference to the
        // base class
        if let ReferenceExpr { base, access: ReferenceAccess::Member(access) } = stmt {
            if let Some(base_type_name) = base_type_name {
                let mut owned_base =
                    self.update_inheritance_chain(access, std::mem::take(base), &base_type_name);
                std::mem::swap(base, &mut owned_base);
            }
        };
    }
}

struct SuperKeywordLowerer<'sup> {
    index: Option<&'sup Index>,
    annotations: Option<&'sup mut Box<dyn AnnotationMap>>,
    ctx: Context,
}

impl<'sup> SuperKeywordLowerer<'sup> {
    pub fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        if self.index.is_none() || self.annotations.is_none() {
            return;
        }
        unit.walk(self);
    }

    fn new(
        id_provider: IdProvider,
        index: Option<&'sup Index>,
        annotations: Option<&'sup mut Box<dyn AnnotationMap>>,
    ) -> Self {
        Self { index, ctx: Context::new(id_provider), annotations }
    }

    fn provider(&self) -> IdProvider {
        self.ctx.provider()
    }

    fn walk_with_context<T: WalkerMut>(&mut self, t: &mut T, ctx: Context) {
        let old_ctx = std::mem::replace(&mut self.ctx, ctx);
        t.walk(self);
        self.ctx = old_ctx;
    }
}

impl AstVisitorMut for SuperKeywordLowerer<'_> {
    fn visit_implementation(&mut self, implementation: &mut plc_ast::ast::Implementation) {
        //Only go through the implementation if we have the index and annotations
        if self.index.is_none() || self.annotations.is_none() {
            return;
        }

        let type_name = if let PouType::Method { parent, .. } = &implementation.pou_type {
            parent
        } else {
            &implementation.type_name
        };
        let ctx = self.ctx.with_pou(type_name);
        let ctx = self
            .index
            .as_ref()
            .and_then(|it| ctx.try_with_base(&implementation.type_name, it))
            .unwrap_or(ctx);

        self.walk_with_context(implementation, ctx);
    }

    fn visit_reference_expr(&mut self, node: &mut AstNode) {
        if self.index.is_none() || self.annotations.is_none() {
            return;
        }

        let ReferenceExpr { base, access } = try_from_mut!(node, ReferenceExpr).expect("ReferenceExpr");
        if let Some(base) = base {
            self.visit(base);
        }

        match access {
            ReferenceAccess::Member(t) | ReferenceAccess::Index(t) | ReferenceAccess::Cast(t) => {
                let is_super = t.is_super();
                self.visit(t);
                if is_super {
                    // `super` has been lowered to a new `ReferenceExpr` without base, so we need to add the original base
                    let ReferenceExpr { base: super_base, .. } =
                        try_from_mut!(t, ReferenceExpr).expect("ReferenceExpr");
                    std::mem::swap(super_base, base);
                }
            }
            _ => {}
        };
    }

    fn visit_super(&mut self, node: &mut AstNode) {
        let Some(base_type_name) = self
            .ctx
            .base_type_name
            .as_deref()
            .or_else(|| {
                self.ctx
                    .pou
                    .as_ref()
                    .and_then(|it| self.index.unwrap().find_pou(it))
                    .and_then(|it| it.get_super_class())
            })
            .map(|it| format!("__{it}"))
        else {
            return;
        };

        let old_node = std::mem::take(node);
        let location = old_node.get_location();
        let AstStatement::Super(deref_marker) = old_node.get_stmt() else {
            unreachable!("Must be a super statement")
        };

        let mut new_node = if deref_marker.is_some() {
            // If the super statement is dereferenced, we can just use the existing base-class instance
            AstFactory::create_member_reference(
                AstFactory::create_identifier(&base_type_name, location, self.provider().next_id())
                    .with_metadata(old_node.into()),
                None,
                self.provider().next_id(),
            )
        } else {
            // If the super statement is not dereferenced, we need to bitcast the base-class instance
            create_call_statement("REF", &base_type_name, None, self.provider().clone(), &location)
                .with_metadata(old_node.into())
        };

        std::mem::swap(node, &mut new_node);
        let resolver = super::LoweringResolver::new(self.index.unwrap(), self.provider())
            .with_pou(self.ctx.pou.as_deref().unwrap_or_default());
        self.annotations.as_mut().unwrap().import(resolver.resolve_statement(node));
    }
}

#[cfg(test)]
mod units_tests {
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
        let unit = &project.units[0].get_unit().pous[1];
        assert_debug_snapshot!(unit, @r###"
        POU {
            name: "bar",
            variable_blocks: [
                VariableBlock {
                    variables: [
                        Variable {
                            name: "__foo",
                            data_type: DataTypeReference {
                                referenced_type: "foo",
                            },
                        },
                    ],
                    variable_block_type: Local,
                },
            ],
            pou_type: FunctionBlock,
            return_type: None,
            interfaces: [],
        }
        "###);
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
                x : INT;
            END_VAR
                myFb.x := 1; //myFb.__SUPER.x := 1;
                x := 2; // this should not have any bases added
            END_FUNCTION_BLOCK
        "
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[2];
        assert_debug_snapshot!(unit, @r###"
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
                                        name: "__fb",
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
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: None,
                    },
                    right: LiteralInteger {
                        value: 2,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 16,
                        column: 16,
                        offset: 359,
                    }..TextLocation {
                        line: 17,
                        column: 23,
                        offset: 418,
                    },
                ),
                file: Some(
                    "<internal>",
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
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 18,
                        column: 12,
                        offset: 471,
                    }..TextLocation {
                        line: 18,
                        column: 30,
                        offset: 489,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "###);
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
        assert_debug_snapshot!(unit, @r###"
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
                                        name: "__foo",
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
                file: Some(
                    "<internal>",
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
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 9,
                        column: 12,
                        offset: 213,
                    }..TextLocation {
                        line: 9,
                        column: 30,
                        offset: 231,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "###);
    }

    #[test]
    fn write_to_grandparent_variable_in_initializer() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK grandparent
            VAR
                z : INT := 42;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK parent EXTENDS grandparent
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK child EXTENDS parent
                z := 420;
            END_FUNCTION_BLOCK
        "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        // let unit = &project.units[0].get_unit();
        let unit = &project.units[0].get_unit().implementations[2];
        assert_debug_snapshot!(unit, @r###"
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
                                        name: "__grandparent",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__parent",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
                            },
                        ),
                    },
                    right: LiteralInteger {
                        value: 420,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 11,
                        column: 16,
                        offset: 289,
                    }..TextLocation {
                        line: 11,
                        column: 25,
                        offset: 298,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            name_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 10,
                        column: 27,
                        offset: 252,
                    }..TextLocation {
                        line: 10,
                        column: 32,
                        offset: 257,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 12,
                        column: 12,
                        offset: 311,
                    }..TextLocation {
                        line: 12,
                        column: 30,
                        offset: 329,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "###);
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
                    x[0] := 42; //__SUPER.x[0] := 42;
                    y[2]:= 5; //__SUPER.__SUPER.y[2] := 5;
                    z[3] := x[1] + y[2]; //z[3] := __SUPER.x[1] + __SUPER.__SUPER.y[2];
                    x[a] := 5; //__SUPER.x[__SUPER__.BASE__.a] := 5;
                    y[b] := 6; //__SUPER.__SUPER.y[__SUPER.b] := 6;
                    z[a+b] := 10; //z[__SUPER.__SUPER.a + __SUPER.b] := 10;
                END_FUNCTION_BLOCK
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[2];
        assert_debug_snapshot!(unit, @r###"
        Implementation {
            name: "child",
            type_name: "child",
            linkage: Internal,
            pou_type: FunctionBlock,
            statements: [
                Assignment {
                    left: ReferenceExpr {
                        kind: Index(
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
                                                name: "__parent",
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
                        kind: Index(
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
                                                name: "__grandparent",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__parent",
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
                        kind: Index(
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
                            kind: Index(
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
                                                    name: "__parent",
                                                },
                                            ),
                                            base: None,
                                        },
                                    ),
                                },
                            ),
                        },
                        right: ReferenceExpr {
                            kind: Index(
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
                                                    name: "__grandparent",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "__parent",
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
                        kind: Index(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "a",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__grandparent",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__parent",
                                                    },
                                                ),
                                                base: None,
                                            },
                                        ),
                                    },
                                ),
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
                                                name: "__parent",
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
                        kind: Index(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "b",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__parent",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
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
                                                name: "__grandparent",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__parent",
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
                        kind: Index(
                            BinaryExpression {
                                operator: Plus,
                                left: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "a",
                                        },
                                    ),
                                    base: Some(
                                        ReferenceExpr {
                                            kind: Member(
                                                Identifier {
                                                    name: "__grandparent",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "__parent",
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
                                        Identifier {
                                            name: "b",
                                        },
                                    ),
                                    base: Some(
                                        ReferenceExpr {
                                            kind: Member(
                                                Identifier {
                                                    name: "__parent",
                                                },
                                            ),
                                            base: None,
                                        },
                                    ),
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
                        offset: 949,
                    },
                ),
                file: Some(
                    "<internal>",
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
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 25,
                        column: 16,
                        offset: 1008,
                    }..TextLocation {
                        line: 25,
                        column: 34,
                        offset: 1026,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "###);
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
                // __SUPER.myFb.__SUPER.x
            END_FUNCTION_BLOCK
        "
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[3];
        assert_debug_snapshot!(unit, @r###"
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
                                        name: "__fb",
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
                                                        name: "__baz",
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
                file: Some(
                    "<internal>",
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
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 23,
                        column: 12,
                        offset: 537,
                    }..TextLocation {
                        line: 23,
                        column: 30,
                        offset: 555,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "###);
    }

    #[test]
    fn test_array_of_objects() {
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
            END_FUNCTION_BLOCK

            FUNCTION main
            VAR
                arr: ARRAY[0..10] OF child;
            END_VAR
                arr[0].a := 10;
                arr[0].y[0] := 20;
                arr[1].b := 30;
                arr[1].x[1] := 40;
                arr[2].z[2] := 50;
            END_FUNCTION
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[3];
        assert_debug_snapshot!(unit, @r###"
        Implementation {
            name: "main",
            type_name: "main",
            linkage: Internal,
            pou_type: Function,
            statements: [
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "a",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__grandparent",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__parent",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Index(
                                                    LiteralInteger {
                                                        value: 0,
                                                    },
                                                ),
                                                base: Some(
                                                    ReferenceExpr {
                                                        kind: Member(
                                                            Identifier {
                                                                name: "arr",
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
                        ),
                    },
                    right: LiteralInteger {
                        value: 10,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Index(
                            LiteralInteger {
                                value: 0,
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
                                                name: "__grandparent",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__parent",
                                                    },
                                                ),
                                                base: Some(
                                                    ReferenceExpr {
                                                        kind: Index(
                                                            LiteralInteger {
                                                                value: 0,
                                                            },
                                                        ),
                                                        base: Some(
                                                            ReferenceExpr {
                                                                kind: Member(
                                                                    Identifier {
                                                                        name: "arr",
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
                                ),
                            },
                        ),
                    },
                    right: LiteralInteger {
                        value: 20,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "b",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__parent",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Index(
                                            LiteralInteger {
                                                value: 1,
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "arr",
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
                        value: 30,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Index(
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
                                                name: "__parent",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Index(
                                                    LiteralInteger {
                                                        value: 1,
                                                    },
                                                ),
                                                base: Some(
                                                    ReferenceExpr {
                                                        kind: Member(
                                                            Identifier {
                                                                name: "arr",
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
                        ),
                    },
                    right: LiteralInteger {
                        value: 40,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Index(
                            LiteralInteger {
                                value: 2,
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "z",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Index(
                                            LiteralInteger {
                                                value: 2,
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "arr",
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
                        value: 50,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 25,
                        column: 16,
                        offset: 668,
                    }..TextLocation {
                        line: 29,
                        column: 34,
                        offset: 820,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            name_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 21,
                        column: 21,
                        offset: 567,
                    }..TextLocation {
                        line: 21,
                        column: 25,
                        offset: 571,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 30,
                        column: 12,
                        offset: 833,
                    }..TextLocation {
                        line: 30,
                        column: 24,
                        offset: 845,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "###)
    }

    #[test]
    fn test_complex_array_access() {
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
                y[b + z[b*2] - a] := 20;
            END_FUNCTION_BLOCK
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[2];
        assert_debug_snapshot!(unit, @r###"
        Implementation {
            name: "child",
            type_name: "child",
            linkage: Internal,
            pou_type: FunctionBlock,
            statements: [
                Assignment {
                    left: ReferenceExpr {
                        kind: Index(
                            BinaryExpression {
                                operator: Minus,
                                left: BinaryExpression {
                                    operator: Plus,
                                    left: ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "b",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__parent",
                                                    },
                                                ),
                                                base: None,
                                            },
                                        ),
                                    },
                                    right: ReferenceExpr {
                                        kind: Index(
                                            BinaryExpression {
                                                operator: Multiplication,
                                                left: ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "b",
                                                        },
                                                    ),
                                                    base: Some(
                                                        ReferenceExpr {
                                                            kind: Member(
                                                                Identifier {
                                                                    name: "__parent",
                                                                },
                                                            ),
                                                            base: None,
                                                        },
                                                    ),
                                                },
                                                right: LiteralInteger {
                                                    value: 2,
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
                                },
                                right: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "a",
                                        },
                                    ),
                                    base: Some(
                                        ReferenceExpr {
                                            kind: Member(
                                                Identifier {
                                                    name: "__grandparent",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "__parent",
                                                        },
                                                    ),
                                                    base: None,
                                                },
                                            ),
                                        },
                                    ),
                                },
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
                                                name: "__grandparent",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__parent",
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
                        value: 20,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 19,
                        column: 16,
                        offset: 530,
                    }..TextLocation {
                        line: 19,
                        column: 40,
                        offset: 554,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            name_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 15,
                        column: 27,
                        offset: 404,
                    }..TextLocation {
                        line: 15,
                        column: 32,
                        offset: 409,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 20,
                        column: 12,
                        offset: 567,
                    }..TextLocation {
                        line: 20,
                        column: 30,
                        offset: 585,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "###);
    }

    #[test]
    fn pointer_deref_in_grandparent() {
        let src: SourceCode = r#"
                FUNCTION_BLOCK grandparent
                VAR
                    a : REF_TO INT;
                END_VAR
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK parent extends grandparent
                VAR
                    b : REF_TO INT;
                END_VAR
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK child EXTENDS parent
                VAR
                    c : REF_TO INT;
                END_VAR
                END_FUNCTION_BLOCK

                FUNCTION main
                VAR
                    fb: child;
                END_VAR
                    fb.c^ := 10;
                    fb.b^ := 20;
                    fb.a^ := 30;
                END_FUNCTION
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[3];
        assert_debug_snapshot!(unit, @r###"
        Implementation {
            name: "main",
            type_name: "main",
            linkage: Internal,
            pou_type: Function,
            statements: [
                CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__init_child",
                            },
                        ),
                        base: None,
                    },
                    parameters: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "fb",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Deref,
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "c",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "fb",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
                            },
                        ),
                    },
                    right: LiteralInteger {
                        value: 10,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Deref,
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "b",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__parent",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "fb",
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
                        value: 20,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Deref,
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "a",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__grandparent",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__parent",
                                                    },
                                                ),
                                                base: Some(
                                                    ReferenceExpr {
                                                        kind: Member(
                                                            Identifier {
                                                                name: "fb",
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
                        ),
                    },
                    right: LiteralInteger {
                        value: 30,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 23,
                        column: 20,
                        offset: 627,
                    }..TextLocation {
                        line: 25,
                        column: 32,
                        offset: 705,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            name_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 19,
                        column: 25,
                        offset: 527,
                    }..TextLocation {
                        line: 19,
                        column: 29,
                        offset: 531,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 26,
                        column: 16,
                        offset: 722,
                    }..TextLocation {
                        line: 26,
                        column: 28,
                        offset: 734,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "###)
    }

    #[test]
    fn base_type_in_initializer() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK grandparent
            VAR CONSTANT
                a : DINT := 3;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK parent extends grandparent
            VAR
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK child EXTENDS parent
            VAR
                b : DINT := a;
            END_VAR
            END_FUNCTION_BLOCK
        "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().pous[2];
        assert_debug_snapshot!(unit, @r###"
        POU {
            name: "child",
            variable_blocks: [
                VariableBlock {
                    variables: [
                        Variable {
                            name: "__parent",
                            data_type: DataTypeReference {
                                referenced_type: "parent",
                            },
                        },
                    ],
                    variable_block_type: Local,
                },
                VariableBlock {
                    variables: [
                        Variable {
                            name: "b",
                            data_type: DataTypeReference {
                                referenced_type: "DINT",
                            },
                            initializer: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "a",
                                        },
                                    ),
                                    base: Some(
                                        ReferenceExpr {
                                            kind: Member(
                                                Identifier {
                                                    name: "__grandparent",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "__parent",
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
                    ],
                    variable_block_type: Local,
                },
            ],
            pou_type: FunctionBlock,
            return_type: None,
            interfaces: [],
        }
        "###);
    }

    #[test]
    fn base_type_in_method_var_initializer() {
        let src: SourceCode = r#"
    FUNCTION_BLOCK grandparent
    VAR CONSTANT
        a : DINT := 3;
    END_VAR
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK parent extends grandparent
    VAR
    END_VAR
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK child EXTENDS parent
        METHOD foo
        VAR
            b : DINT := a;
        END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
"#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().pous[3];
        assert_debug_snapshot!(unit, @r#"
        POU {
            name: "child.foo",
            variable_blocks: [
                VariableBlock {
                    variables: [
                        Variable {
                            name: "b",
                            data_type: DataTypeReference {
                                referenced_type: "DINT",
                            },
                            initializer: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "a",
                                        },
                                    ),
                                    base: Some(
                                        ReferenceExpr {
                                            kind: Member(
                                                Identifier {
                                                    name: "__grandparent",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "__parent",
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
                    ],
                    variable_block_type: Local,
                },
            ],
            pou_type: Method {
                parent: "child",
                property: None,
                declaration_kind: Concrete,
            },
            return_type: None,
            interfaces: [],
        }
        "#);
    }

    #[test]
    fn assigning_to_base_type_in_method() {
        let src: SourceCode = r#"
        FUNCTION_BLOCK foo
        VAR
            x : DINT := 50;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar EXTENDS foo
        METHOD set0 // TODO(volsa): https://github.com/PLC-lang/rusty/issues/1408
            x := 25;
        END_METHOD
        END_FUNCTION_BLOCK
        "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[1];
        assert_debug_snapshot!(unit, @r###"
        Implementation {
            name: "bar.set0",
            type_name: "bar.set0",
            linkage: Internal,
            pou_type: Method {
                parent: "bar",
                property: None,
                declaration_kind: Concrete,
            },
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
                                        name: "__foo",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                    right: LiteralInteger {
                        value: 25,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 9,
                        column: 12,
                        offset: 245,
                    }..TextLocation {
                        line: 9,
                        column: 20,
                        offset: 253,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            name_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 8,
                        column: 15,
                        offset: 166,
                    }..TextLocation {
                        line: 8,
                        column: 19,
                        offset: 170,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 10,
                        column: 8,
                        offset: 262,
                    }..TextLocation {
                        line: 10,
                        column: 18,
                        offset: 272,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: Some(
                Protected,
            ),
        }
        "###);
    }
}

#[cfg(test)]
mod resolve_bases_tests {
    use std::ops::Deref;

    use insta::assert_debug_snapshot;
    use plc::resolver::AnnotationMap;
    use plc_ast::{
        ast::{Assignment, ReferenceExpr},
        try_from,
    };
    use plc_driver::{parse_and_annotate, pipelines::AnnotatedProject};
    use plc_source::SourceCode;

    #[test]
    fn base_types_resolved() {
        let src: SourceCode = r#"
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
            END_FUNCTION_BLOCK
            "#
        .into();

        let (_, AnnotatedProject { units, index: _index, annotations }) =
            parse_and_annotate("test", vec![src]).unwrap();
        let unit = &units[0].get_unit().implementations[3];
        let statement = &unit.statements[0];
        let Some(Assignment { left, .. }) = try_from!(statement, Assignment) else { unreachable!() };
        assert_debug_snapshot!(annotations.get(left), @r#"
        Some(
            Variable {
                resulting_type: "INT",
                qualified_name: "fb.x",
                constant: false,
                argument_type: ByVal(
                    Local,
                ),
                auto_deref: None,
            },
        )
        "#);

        let Some(ReferenceExpr { base, .. }) = try_from!(left, ReferenceExpr) else { unreachable!() };
        let base1 = base.as_ref().unwrap().deref();
        assert_debug_snapshot!(annotations.get(base1).unwrap(), @r###"
        Variable {
            resulting_type: "fb",
            qualified_name: "fb2.__fb",
            constant: false,
            argument_type: ByVal(
                Local,
            ),
            auto_deref: None,
        }
        "###);

        let Some(ReferenceExpr { base, .. }) = try_from!(base1, ReferenceExpr) else { unreachable!() };
        let base2 = base.as_ref().unwrap().deref();
        assert_debug_snapshot!(annotations.get(base2).unwrap(), @r#"
        Variable {
            resulting_type: "fb2",
            qualified_name: "baz.myFb",
            constant: false,
            argument_type: ByVal(
                Local,
            ),
            auto_deref: None,
        }
        "#);

        let Some(ReferenceExpr { base, .. }) = try_from!(base2, ReferenceExpr) else { unreachable!() };
        let base3 = base.as_ref().unwrap().deref();
        assert_debug_snapshot!(annotations.get(base3).unwrap(), @r###"
        Variable {
            resulting_type: "baz",
            qualified_name: "foo.__baz",
            constant: false,
            argument_type: ByVal(
                Local,
            ),
            auto_deref: None,
        }
        "###);
    }
}

#[cfg(test)]
mod inherited_properties {
    use insta::assert_debug_snapshot;
    use plc_driver::{parse_and_annotate, pipelines::AnnotatedProject};
    use plc_source::SourceCode;

    #[test]
    fn reference_to_property_declared_in_parent_is_called_correctly() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK fb
            PROPERTY foo : INT
                GET END_GET
                SET END_SET
            END_PROPERTY
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK fb2 EXTENDS fb
                foo;
            END_FUNCTION_BLOCK
        "#
        .into();

        let (_, AnnotatedProject { units, .. }) = parse_and_annotate("test", vec![src]).unwrap();
        let implementation = &units[0].get_unit().implementations[1];
        let stmt = &implementation.statements[0];
        assert_debug_snapshot!(stmt, @r###"
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
                                name: "__fb",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            parameters: None,
        }
        "###);
    }

    #[test]
    fn reference_to_property_declared_in_grandparent_is_called_correctly() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK fb
            PROPERTY foo : INT
                GET END_GET
                SET END_SET
            END_PROPERTY
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK fb2 EXTENDS fb
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK fb3 EXTENDS fb2
                foo;
            END_FUNCTION_BLOCK
        "#
        .into();

        let (_, AnnotatedProject { units, .. }) = parse_and_annotate("test", vec![src]).unwrap();
        let implementation = &units[0].get_unit().implementations[2];
        let stmt = &implementation.statements[0];
        assert_debug_snapshot!(stmt, @r###"
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
                                name: "__fb",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__fb2",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
            },
            parameters: None,
        }
        "###);
    }

    #[test]
    fn extended_prop() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK fb
            PROPERTY foo : INT
                GET END_GET
            END_PROPERTY
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK fb2 EXTENDS fb
            PROPERTY FOO : INT
                SET END_SET
            END_PROPERTY
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK fb3 EXTENDS fb2
                // we expect the RHS to call the getter defined in the grandparent and
                // pass the result to the setter call in the grandparent
                foo := foo;
            END_FUNCTION_BLOCK
        "#
        .into();

        let (_, AnnotatedProject { units, .. }) = parse_and_annotate("test", vec![src]).unwrap();
        let implementation = &units[0].get_unit().implementations[2];
        let stmt = &implementation.statements[0];
        assert_debug_snapshot!(stmt, @r###"
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "__set_foo",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__fb2",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            parameters: Some(
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
                                        name: "__fb",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__fb2",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
                            },
                        ),
                    },
                    parameters: None,
                },
            ),
        }
        "###);
    }
}

#[cfg(test)]
mod super_tests {
    use insta::assert_debug_snapshot;
    use plc_driver::parse_and_annotate;
    use plc_source::SourceCode;

    #[test]
    fn super_qualified_reference_resolves_to_same_parent_var() {
        let src: SourceCode = "
        FUNCTION_BLOCK foo
            VAR
                x: DINT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar EXTENDS foo
            x := 3;
            super^.x := 3;
        END_FUNCTION_BLOCK
        "
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let statements = &project.units[0].get_unit().implementations[1].statements;
        assert_debug_snapshot!(statements, @r#"
        [
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
                                    name: "__foo",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 3,
                },
            },
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
                                    name: "__foo",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 3,
                },
            },
        ]
        "#);
    }

    #[test]
    fn super_without_deref_lowered_to_ref_call_to_parent_instance() {
        let src: SourceCode = "
        FUNCTION_BLOCK foo
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar EXTENDS foo
            super;
        END_FUNCTION_BLOCK
        "
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let statements = &project.units[0].get_unit().implementations[1].statements;
        assert_debug_snapshot!(statements, @r#"
        [
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
                        kind: Member(
                            Identifier {
                                name: "__foo",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        ]
        "#);
    }

    #[test]
    fn super_expression_as_function_argument() {
        let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            // Use SUPER expression for function calls
            foo(SUPER, SUPER^);
        END_FUNCTION_BLOCK

        FUNCTION foo : INT
        VAR_INPUT
            input_ref : REF_TO parent;
            input : parent;
        END_VAR
        END_FUNCTION
    "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let statements = &project.units[0].get_unit().implementations[1].statements;
        assert_debug_snapshot!(statements, @r#"
    [
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "foo",
                    },
                ),
                base: None,
            },
            parameters: Some(
                ExpressionList {
                    expressions: [
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
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "__parent",
                                },
                            ),
                            base: None,
                        },
                    ],
                },
            ),
        },
    ]
    "#);
    }

    #[test]
    fn access_grandparent_through_super() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK grandparent
            VAR
                x : INT := 10;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK parent EXTENDS grandparent
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK child EXTENDS parent
                // Access grandparent member through SUPER^
                SUPER^.x := 200;
            END_FUNCTION_BLOCK
        "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let statements = &project.units[0].get_unit().implementations[2].statements;
        assert_debug_snapshot!(statements, @r#"
        [
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
                                    name: "__grandparent",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 200,
                },
            },
        ]
        "#);
    }

    #[test]
    #[ignore = "stack overflow"]
    fn access_great_grandparent_through_super() {
        let src: SourceCode = r#"
        FUNCTION_BLOCK great_grandparent
        VAR
            x : INT := 10;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK grandparent EXTENDS great_grandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent EXTENDS grandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            // Access great_grandparent member through SUPER^
            SUPER^.x := 100;
        END_FUNCTION_BLOCK
    "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let statements = &project.units[0].get_unit().implementations[3].statements;
        assert_debug_snapshot!(statements, @r#""#);
    }

    #[test]
    fn super_keyword_in_method_call() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK parent
            VAR
                x : INT := 10;
            END_VAR
                METHOD base_method : INT
                    base_method := x;
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK child EXTENDS parent
                METHOD test
                    // Call method on parent through SUPER^
                    SUPER^.base_method();
                END_METHOD
            END_FUNCTION_BLOCK
        "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let statements = &project.units[0].get_unit().implementations[2].statements;
        assert_debug_snapshot!(statements, @r#"
        [
            CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "base_method",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "__parent",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                parameters: None,
            },
        ]
        "#);
    }

    #[test]
    fn chained_super_keywords() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK grandparent
            VAR
                x : INT := 10;
                y : INT := 20;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK parent EXTENDS grandparent
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK child EXTENDS parent
                // Chained SUPER access (technically invalid but we should handle it gracefully)
                SUPER^.SUPER^.x := SUPER^.SUPER^.y;
            END_FUNCTION_BLOCK
        "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let statements = &project.units[0].get_unit().implementations[2].statements;
        assert_debug_snapshot!(statements, @r#"
        [
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
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: Some(
                                        ReferenceExpr {
                                            kind: Member(
                                                Identifier {
                                                    name: "__parent",
                                                },
                                            ),
                                            base: None,
                                        },
                                    ),
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                right: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "y",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: Some(
                                        ReferenceExpr {
                                            kind: Member(
                                                Identifier {
                                                    name: "__parent",
                                                },
                                            ),
                                            base: None,
                                        },
                                    ),
                                },
                            ),
                            base: None,
                        },
                    ),
                },
            },
        ]
        "#);
    }

    #[test]
    fn super_in_array_access_edge_cases() {
        let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        VAR
            arr : ARRAY[0..5] OF INT := [1,2,3,4,5,6];
            idx : INT := 2;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        VAR
            arr : ARRAY[0..5] OF INT := [10,20,30,40,50,60];
        END_VAR
            // Access parent array with SUPER^ using parent's index
            SUPER^.arr[SUPER^.idx] := 100;
            
            // Access parent array with SUPER^ using child's array element
            SUPER^.arr[arr[0]] := 200;
            
            // Access child array using parent's index
            arr[SUPER^.idx] := 300;
        END_FUNCTION_BLOCK
    "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let statements = &project.units[0].get_unit().implementations[1].statements;
        assert_debug_snapshot!(statements, @r#"
        [
            Assignment {
                left: ReferenceExpr {
                    kind: Index(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "idx",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "arr",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 100,
                },
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Index(
                        ReferenceExpr {
                            kind: Index(
                                LiteralInteger {
                                    value: 0,
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "arr",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "arr",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 200,
                },
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Index(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "idx",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "arr",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 300,
                },
            },
        ]
        "#);
    }

    #[test]
    fn super_in_complex_expressions() {
        let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        VAR
            x : INT := 10;
            y : INT := 20;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        VAR
            z : INT := 30;
        END_VAR
            // Use SUPER^ in complex arithmetic expressions
            z := SUPER^.x + SUPER^.y * 2;
            
            // Use SUPER^ in condition
            IF SUPER^.x > SUPER^.y THEN
                z := 100;
            END_IF;
            
            // Use SUPER^ in mixed expression with child variables
            SUPER^.x := z - SUPER^.y;
        END_FUNCTION_BLOCK
    "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let statements = &project.units[0].get_unit().implementations[1].statements;
        assert_debug_snapshot!(statements, @r#"
        [
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "z",
                        },
                    ),
                    base: None,
                },
                right: BinaryExpression {
                    operator: Plus,
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
                                        name: "__parent",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                    right: BinaryExpression {
                        operator: Multiplication,
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "y",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                        right: LiteralInteger {
                            value: 2,
                        },
                    },
                },
            },
            IfStatement {
                blocks: [
                    ConditionalBlock {
                        condition: BinaryExpression {
                            operator: Greater,
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
                                                name: "__parent",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
                            },
                            right: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "y",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__parent",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
                            },
                        },
                        body: [
                            Assignment {
                                left: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "z",
                                        },
                                    ),
                                    base: None,
                                },
                                right: LiteralInteger {
                                    value: 100,
                                },
                            },
                        ],
                    },
                ],
                else_block: [],
            },
            EmptyStatement,
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
                                    name: "__parent",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                right: BinaryExpression {
                    operator: Minus,
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "z",
                            },
                        ),
                        base: None,
                    },
                    right: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "y",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__parent",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                },
            },
        ]
        "#);
    }

    #[test]
    fn super_with_mixed_deref_patterns() {
        let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        VAR
            x : REF_TO INT;
            y : INT := 20;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            // Mix of SUPER^ with deref operator ^
            SUPER^.x^ := 100;
            
            // Multiple ^ operators in sequence
            SUPER^.x^ := SUPER^.y;
        END_FUNCTION_BLOCK
    "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let statements = &project.units[0].get_unit().implementations[1].statements;
        assert_debug_snapshot!(statements, @r#"
        [
            Assignment {
                left: ReferenceExpr {
                    kind: Deref,
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
                                            name: "__parent",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 100,
                },
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Deref,
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
                                            name: "__parent",
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
                        Identifier {
                            name: "y",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "__parent",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
            },
        ]
        "#);
    }

    #[test]
    fn super_to_access_overridden_methods() {
        let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        VAR
            x : INT := 10;
        END_VAR
            METHOD calculate : INT
                calculate := x * 2;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        VAR
            y : INT := 20;
        END_VAR
            METHOD calculate : INT // Override parent's method
                calculate := x + y;
            END_METHOD

            METHOD test : INT
                // Call parent's version of the overridden method
                test := SUPER^.calculate();
            END_METHOD
        END_FUNCTION_BLOCK
    "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let statements = &project.units[0].get_unit().implementations[3].statements;
        assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "test",
                    },
                ),
                base: None,
            },
            right: CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "calculate",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "__parent",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                parameters: None,
            },
        },
    ]
    "#);
    }

    #[test]
    fn super_in_complex_expressions_with_method_calls() {
        let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        VAR
            x : INT := 10;
            y : INT := 20;
        END_VAR
            METHOD get_x : INT
                get_x := x;
            END_METHOD
            
            METHOD get_y : INT
                get_y := y;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        VAR
            z : INT := 30;
        END_VAR
            METHOD get_x : INT // Override parent's method
                get_x := x * 2;
            END_METHOD
            
            METHOD test : INT
                // Use parent's methods in an expression
                test := SUPER^.get_x() + SUPER^.get_y();
            END_METHOD
        END_FUNCTION_BLOCK
    "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let statements = &dbg!(&project.units[0].get_unit().implementations)[4].statements;
        assert_debug_snapshot!(statements, @r#"
        [
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "test",
                        },
                    ),
                    base: None,
                },
                right: BinaryExpression {
                    operator: Plus,
                    left: CallStatement {
                        operator: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "get_x",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                        parameters: None,
                    },
                    right: CallStatement {
                        operator: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "get_y",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                        parameters: None,
                    },
                },
            },
        ]
        "#);
    }

    #[test]
    fn super_access_with_interface_methods() {
        let src: SourceCode = r#"
        INTERFACE ICounter
            METHOD increment : INT END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK parent IMPLEMENTS ICounter
        VAR
            count : INT := 0;
        END_VAR
            METHOD increment : INT
                count := count + 1;
                increment := count;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD increment : INT // Override the interface method
                count := count + 10;
                increment := count;
            END_METHOD
            
            METHOD double_increment : INT
                // Call parent's implementation of the interface method
                SUPER^.increment();
                // Call our own implementation
                increment();
                double_increment := count;
            END_METHOD
        END_FUNCTION_BLOCK
    "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let statements = &project.units[0].get_unit().implementations[3].statements;
        assert_debug_snapshot!(statements, @r#"
        [
            CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "increment",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "__parent",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                parameters: None,
            },
            CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "increment",
                        },
                    ),
                    base: None,
                },
                parameters: None,
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "double_increment",
                        },
                    ),
                    base: None,
                },
                right: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "count",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "__parent",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
            },
        ]
        "#);
    }

    #[test]
    fn super_with_multiple_overridden_methods_in_hierarchy() {
        let src: SourceCode = r#"
        FUNCTION_BLOCK grandparent
            METHOD process : INT
                process := 1;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent EXTENDS grandparent
            METHOD process : INT // Override grandparent method
                process := 2;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD process : INT // Override parent method
                process := 3;
            END_METHOD
            
            METHOD test : INT
                // Call parent's version of the method
                test := SUPER^.process();
                
                // We cannot access grandparent's version directly with SUPER^.SUPER^.process()
                // as chaining SUPER is not allowed, we'll still check if we handle it gracefully
                test := SUPER^.SUPER^.process();
            END_METHOD
        END_FUNCTION_BLOCK
    "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let statements = &project.units[0].get_unit().implementations[5].statements;
        assert_debug_snapshot!(statements, @r#"
        [
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "test",
                        },
                    ),
                    base: None,
                },
                right: CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "process",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__parent",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                    parameters: None,
                },
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "test",
                        },
                    ),
                    base: None,
                },
                right: CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "process",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__parent",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__parent",
                                                    },
                                                ),
                                                base: None,
                                            },
                                        ),
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                    parameters: None,
                },
            },
        ]
        "#);
    }

    #[test]
    fn super_in_constructor() {
        let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        VAR
            x : INT := 10;
        END_VAR
            METHOD init
                x := 100;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        VAR
            y : INT := 20;
        END_VAR
            METHOD init
                // Call parent's init method
                SUPER^.init();
                y := 200;
            END_METHOD
        END_FUNCTION_BLOCK
    "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let statements = &project.units[0].get_unit().implementations[2].statements;
        assert_debug_snapshot!(statements, @r#"
        [
            CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "init",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "__parent",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                parameters: None,
            },
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "y",
                        },
                    ),
                    base: None,
                },
                right: LiteralInteger {
                    value: 200,
                },
            },
        ]
        "#);
    }
}
