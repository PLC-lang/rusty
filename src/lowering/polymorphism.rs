//! Module responsible for desugaring function calls that are polymorphic in nature.
//!
//! TODO: Documentation, in a nutshell any variable of type `POINTER TO <Function Block | Class>` must be
//! desugared to call its methods using the virtual table.

use plc_ast::{
    ast::{
        AstFactory, AstNode, AstStatement, CallStatement, CompilationUnit, PouType, ReferenceAccess,
        ReferenceExpr,
    },
    mut_visitor::AstVisitorMut,
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

use crate::{
    index::Index,
    resolver::{AnnotationMap, AnnotationMapImpl},
    typesystem::{DataTypeInformation, StructSource},
};

pub struct PolymorphicCallDesugarer {
    pub ids: IdProvider,
    pub index: Option<Index>,
    pub annotations: Option<AnnotationMapImpl>,
}

impl PolymorphicCallDesugarer {
    pub fn new(ids: IdProvider) -> PolymorphicCallDesugarer {
        PolymorphicCallDesugarer { ids, index: None, annotations: None }
    }

    pub fn desugar_unit(&mut self, unit: &mut CompilationUnit) {
        self.visit_compilation_unit(unit);
    }

    // XXX: Tests for this method?
    fn is_polymorphic_candidate(&self, node: &AstNode) -> bool {
        let AstStatement::ReferenceExpr(ReferenceExpr { access, base }) = &node.stmt else {
            return false;
        };

        let index = self.index.as_ref().unwrap();
        let annotations = self.annotations.as_ref().unwrap();

        match (access, base) {
            // Probably dealing with `MyFbRef^.foo()`
            (ReferenceAccess::Member(_), Some(base)) if base.is_deref() => {
                self.is_polymorphic_candidate(base)
            }

            // Probably dealing with `MyFbRef^()`
            (ReferenceAccess::Deref, Some(base)) => {
                let info_ptr = annotations.get_type_or_void(base, index).get_type_information();
                let DataTypeInformation::Pointer { inner_type_name, .. } = info_ptr else {
                    return false;
                };

                let info_inner = index.get_type_information_or_void(inner_type_name);
                info_inner.is_class() || info_inner.is_function_block()
            }

            _ => false,
        }
    }

    fn patch_instance_argument(&mut self, operator: &mut AstNode, parameters: &mut Option<Box<AstNode>>) {
        // foo.bar()
        // ^^^ base
        let base = operator.get_base().unwrap(); // XXX: I think this might fail on `MyBlockRef^()`

        match parameters {
            None => {
                parameters.replace(Box::new(base.clone()));
            }

            Some(ref mut expr) => match &mut expr.stmt {
                AstStatement::ExpressionList(expressions) => {
                    expressions.insert(0, base.clone());
                }

                _ => {
                    let mut expressions = Box::new(AstFactory::create_expression_list(
                        vec![base.clone(), std::mem::take(expr)],
                        SourceLocation::internal(),
                        self.ids.next_id(),
                    ));

                    std::mem::swap(expr, &mut expressions);
                }
            },
        }
    }

    /// Patches a `__vtable` member access into the given node, e.g. `ref^.foo()` becomes `ref^.__vtable^.foo()`
    fn patch_vtable_access(&mut self, node: &mut AstNode) {
        let base_old = node.get_base_mut().unwrap(); // `ref^` in `ref^.foo()`

        let base_new = AstFactory::create_member_reference(
            AstFactory::create_identifier("__vtable", SourceLocation::internal(), self.ids.next_id()),
            Some(std::mem::take(base_old)),
            self.ids.next_id(),
        );

        let mut base_new_deref =
            AstFactory::create_deref_reference(base_new, self.ids.next_id(), SourceLocation::internal());

        std::mem::swap(base_old, &mut base_new_deref);
    }

    /// ref^.__vtable^.foo()` -> `__vtable_{POU_NAME}#(ref^.__vtable^).foo()
    fn patch_vtable_cast(&mut self, node: &mut AstNode, pou_type_name: &str) {
        let base_old = node.get_base_mut().unwrap(); // `ref^.__vtable^` in `ref^.__vtable^.foo()`
        let base_old_paren = AstFactory::create_paren_expression(
            std::mem::take(base_old),
            SourceLocation::internal(),
            self.ids.next_id(),
        );

        let mut base_new = AstFactory::create_cast_statement(
            AstFactory::create_member_reference(
                AstFactory::create_identifier(
                    format!("__vtable_{pou_type_name}"),
                    SourceLocation::internal(),
                    self.ids.next_id(),
                ),
                None,
                self.ids.next_id(),
            ),
            base_old_paren,
            &SourceLocation::internal(),
            self.ids.next_id(),
        );

        std::mem::swap(base_old, &mut base_new);
    }

    // `__vtable_{POU_NAME}#(ref^.__vtable^).foo()` -> `__vtable_{POU_NAME}#(ref^.__vtable^).foo^()`
    fn patch_method_call_deref(&mut self, node: &mut AstNode) {
        let mut base_new = AstFactory::create_deref_reference(
            std::mem::take(node),
            self.ids.next_id(),
            SourceLocation::internal(),
        );

        std::mem::swap(node, &mut base_new);
    }
}

impl AstVisitorMut for PolymorphicCallDesugarer {
    // TODO: Debugging
    fn visit_call_statement(&mut self, node: &mut AstNode) {
        // When dealing with a function call such as `ref^.foo()` we have to perform several steps to desugar
        // it into a form that can be executed by the codegen without any intervention from our side, namely:
        // 1. We must add the expression (excluding the method name) as the first argument to the call
        //    -> ref^.foo(ref^)
        // 2. We must access the virtual table of the instance, a VOID pointer
        //    -> ref^.__vtable^.foo(ref^)
        // 3. We must cast the virtual table access into a concrete type
        //    -> __vtable_XXX#(ref^.__vtable^).foo(ref^)
        // 4. We must dereference the method call, which is a function pointer
        //    -> __vtable_XXX#(ref^.__vtable^).foo^(ref^)
        //
        // The final result transforms ref^.foo() into __vtable_XXX#(ref^.__vtable^).foo^(ref^)
        let AstStatement::CallStatement(CallStatement { ref mut operator, parameters }) = &mut node.stmt
        else {
            unreachable!();
        };

        // Check if we're dealing with a polymorphic call, i.e. a variable declared as `POINTER TO <FB, Class>`
        if !self.is_polymorphic_candidate(operator) {
            return;
        }

        log::debug!("Desugaring polymorphic call: {operator:#?}");

        let index = self.index.as_ref().unwrap();
        let annotations = self.annotations.as_ref().unwrap();

        let dt = annotations.get_type(operator.get_base().unwrap(), index).unwrap().clone();
        debug_assert!(matches!(
            dt.get_type_information(),
            DataTypeInformation::Struct { source: StructSource::Pou(PouType::FunctionBlock), .. }
        ));

        // Step 1: Patch the instance argument into the argument list
        self.patch_instance_argument(operator, parameters);

        // Step 2: Patch a dereferenced virtual table access into the operator
        self.patch_vtable_access(operator);

        // Step 3: Patch the virtual table cast into the operator
        self.patch_vtable_cast(operator, dt.get_name());

        // Step 4: Patch the method call to a dereferenced call
        self.patch_method_call_deref(operator);
    }
}

#[cfg(test)]
mod tests {
    use plc_ast::{ast::CompilationUnit, provider::IdProvider};

    use crate::{
        lowering::polymorphism::PolymorphicCallDesugarer,
        test_utils::tests::{annotate_with_ids, index_with_ids},
    };

    fn init(source: &str) -> (CompilationUnit, PolymorphicCallDesugarer) {
        let id_provider = IdProvider::default();
        let (unit, mut index) = index_with_ids(source, id_provider.clone());
        let annotations = annotate_with_ids(&unit, &mut index, id_provider.clone());

        let mut lowerer = PolymorphicCallDesugarer::new(id_provider);
        lowerer.index = Some(index);
        lowerer.annotations = Some(annotations);

        (unit, lowerer)
    }

    // TODO: refA^();
    // TODO: Tests params
    #[test]
    fn simple_method_call_is_desugared() {
        let source = r"
            FUNCTION_BLOCK A
                VAR
                    inner: POINTER TO A;
                END_VAR

                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instanceA: A;
                    refInstanceA: POINTER TO A;
                END_VAR

                refInstanceA^.foo();
            END_FUNCTION
        ";

        let (mut unit, mut desugarer) = init(source);
        desugarer.desugar_unit(&mut unit);

        let statements = &unit.implementations.iter().find(|it| it.name == "main").unwrap().statements;
        insta::assert_debug_snapshot!(statements[0], @r#"
        CallStatement {
            operator: ReferenceExpr {
                kind: Deref,
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "foo",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Cast(
                                    ParenExpression {
                                        expression: ReferenceExpr {
                                            kind: Deref,
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "__vtable",
                                                        },
                                                    ),
                                                    base: Some(
                                                        ReferenceExpr {
                                                            kind: Deref,
                                                            base: Some(
                                                                ReferenceExpr {
                                                                    kind: Member(
                                                                        Identifier {
                                                                            name: "refInstanceA",
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
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__vtable_A",
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
            parameters: Some(
                ReferenceExpr {
                    kind: Deref,
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "refInstanceA",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
            ),
        }
        "#);
    }

    #[test]
    fn demo() {
        let source = r"
            FUNCTION_BLOCK A
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK B EXTENDS A
                METHOD foo
                    printf('B::foo$N');
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK C EXTENDS B
                    METHOD foo
                    printf('C::foo$N');
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refArr: ARRAY[1..2] OF POINTER TO B;
                END_VAR

                refArr[1]^.foo();
            END_FUNCTION
        ";

        let (mut unit, mut desugarer) = init(source);
        desugarer.desugar_unit(&mut unit);

        let statements = &unit.implementations.iter().find(|it| it.name == "main").unwrap().statements;
        insta::assert_debug_snapshot!(statements[0], @r#"
        CallStatement {
            operator: ReferenceExpr {
                kind: Deref,
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "foo",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Cast(
                                    ParenExpression {
                                        expression: ReferenceExpr {
                                            kind: Deref,
                                            base: Some(
                                                ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "__vtable",
                                                        },
                                                    ),
                                                    base: Some(
                                                        ReferenceExpr {
                                                            kind: Deref,
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
                                                                                    name: "refArr",
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
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__vtable_B",
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
            parameters: Some(
                ReferenceExpr {
                    kind: Deref,
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
                                            name: "refArr",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
            ),
        }
        "#);
    }
}
