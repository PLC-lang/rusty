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
//!   __SUPER : foo;
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
//!    myFb.__SUPER.x := 1;
//! END_FUNCTION_BLOCK
//! ```

use plc::{index::Index, resolver::AnnotationMap};
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
const BASE_TYPE_NAME: &str = "__SUPER";

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
        inheritance_chain.iter().skip(1).fold(base, |base, _| {
            Some(Box::new(AstFactory::create_member_reference(
                AstFactory::create_identifier(
                    BASE_TYPE_NAME,
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
            let pou_name = if let PouType::Method { parent } = &pou.kind { parent } else { &pou.name };
            return self.walk_with_context(pou, self.ctx.with_pou(pou_name));
        }
        if !matches!(pou.kind, PouType::FunctionBlock | PouType::Class) {
            return;
        }

        let Some(base_name) = pou.super_class.as_ref() else {
            return;
        };

        let base_var = Variable {
            name: BASE_TYPE_NAME.into(),
            data_type_declaration: DataTypeDeclaration::Reference {
                referenced_type: base_name.into(),
                location: SourceLocation::internal(),
            },
            location: SourceLocation::internal(),
            initializer: None,
            address: None,
        };

        let block = VariableBlock {
            variables: vec![base_var],
            variable_block_type: VariableBlockType::Local,
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

        let type_name = if let PouType::Method { parent } = &implementation.pou_type {
            parent
        } else {
            &implementation.type_name
        };

        let ctx = self.ctx.with_pou(type_name);
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
        let unit = &project.units[0].get_unit().units[1];
        assert_debug_snapshot!(unit, @r#"
        POU {
            name: "bar",
            variable_blocks: [
                VariableBlock {
                    variables: [
                        Variable {
                            name: "__SUPER",
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
                x : INT;
            END_VAR
                myFb.x := 1; //myFb.__SUPER.x := 1;
                x := 2; // this should not have any bases added
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
                                        name: "__SUPER",
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
                                        name: "__SUPER",
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
                                        name: "__SUPER",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__SUPER",
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
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "#);
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
        assert_debug_snapshot!(unit, @r#"
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
                                                name: "__SUPER",
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
                                                name: "__SUPER",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__SUPER",
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
                                                    name: "__SUPER",
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
                                                    name: "__SUPER",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "__SUPER",
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
                                                name: "__SUPER",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__SUPER",
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
                                                name: "__SUPER",
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
                                                name: "__SUPER",
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
                                                name: "__SUPER",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__SUPER",
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
                                                    name: "__SUPER",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "__SUPER",
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
                                                    name: "__SUPER",
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
                // __SUPER.myFb.__SUPER.x
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
                                        name: "__SUPER",
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
                                                        name: "__SUPER",
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
        assert_debug_snapshot!(unit, @r#"
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
                                        name: "__SUPER",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__SUPER",
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
                                                name: "__SUPER",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__SUPER",
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
                                        name: "__SUPER",
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
                                                name: "__SUPER",
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
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "#)
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
        assert_debug_snapshot!(unit, @r#"
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
                                                        name: "__SUPER",
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
                                                                    name: "__SUPER",
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
                                                    name: "__SUPER",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "__SUPER",
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
                                                name: "__SUPER",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__SUPER",
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
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "#);
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
        assert_debug_snapshot!(unit, @r#"
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
                                                name: "__SUPER",
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
                                                name: "__SUPER",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__SUPER",
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
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "#)
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
        let unit = &project.units[0].get_unit().units[2];
        assert_debug_snapshot!(unit, @r#"
        POU {
            name: "child",
            variable_blocks: [
                VariableBlock {
                    variables: [
                        Variable {
                            name: "__SUPER",
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
                                                    name: "__SUPER",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "__SUPER",
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
        "#);
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
        let unit = &project.units[0].get_unit().units[3];
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
                                                    name: "__SUPER",
                                                },
                                            ),
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "__SUPER",
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
        METHOD set
            x := 25;
        END_METHOD
        END_FUNCTION_BLOCK
        "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[1];
        assert_debug_snapshot!(unit, @r#"
        Implementation {
            name: "bar.set",
            type_name: "bar.set",
            linkage: Internal,
            pou_type: Method {
                parent: "bar",
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
                                        name: "__SUPER",
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
                        offset: 182,
                    }..TextLocation {
                        line: 9,
                        column: 20,
                        offset: 190,
                    },
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
                        column: 18,
                        offset: 169,
                    },
                ),
            },
            overriding: false,
            generic: false,
            access: Some(
                Protected,
            ),
        }
        "#);
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
        assert_debug_snapshot!(annotations.get(base1).unwrap(), @r#"
        Variable {
            resulting_type: "fb",
            qualified_name: "fb2.__SUPER",
            constant: false,
            argument_type: ByVal(
                Local,
            ),
            auto_deref: None,
        }
        "#);

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
        assert_debug_snapshot!(annotations.get(base3).unwrap(), @r#"
        Variable {
            resulting_type: "baz",
            qualified_name: "foo.__SUPER",
            constant: false,
            argument_type: ByVal(
                Local,
            ),
            auto_deref: None,
        }
        "#);
    }
}
