use insta::assert_debug_snapshot;
use plc_ast::provider::IdProvider;

use crate::{
    index::const_expressions::UnresolvableKind,
    resolver::const_evaluator::evaluate_constants,
    test_utils::tests::{annotate_and_lower_with_ids, index, index_with_ids},
};

/// # Architecture Design Records: Lowering of complex initializers to initializer functions
///
/// When encountering an unresolvable initializer to a pointer during constant propagation,
/// rusty will mark this const-expression for a retry during later stages in the compilation pipeline.
#[test]
fn ref_initializer_is_marked_for_later_resolution() {
    let (_, index) = index(
        r#"
        FUNCTION_BLOCK foo
        VAR
            s : STRING;
            ps: REF_TO STRING := REF(s);
        END_VAR
        END_FUNCTION_BLOCK
       "#,
    );

    let (_, unresolvable) = evaluate_constants(index);
    assert_eq!(unresolvable.len(), 1);
    assert_eq!(unresolvable[0].get_reason(), Some(r#"Try to re-resolve during codegen"#));

    let Some(UnresolvableKind::Address(_)) = unresolvable[0].kind else { panic!() };
}

/// These unresolvables are collected and lowered during the `annotation`-stage.
/// Each POU containing such statements will have a corresponding init function registered
/// in the global `Index` and a new `POU` named `__init_<pou-name>` created.
#[test]
fn ref_call_in_initializer_is_lowered_to_init_function() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        FUNCTION_BLOCK foo
        VAR
            s : STRING;
            ps: REFERENCE TO STRING := REF(s);
        END_VAR
        END_FUNCTION_BLOCK
        ",
        id_provider.clone(),
    );

    let (_, index, annotated_units) = annotate_and_lower_with_ids(unit, index, id_provider);

    assert!(index.find_pou("__init_foo").is_some());

    let units = annotated_units.iter().map(|(units, _, _)| units).collect::<Vec<_>>();
    let init_foo_unit = &units[1].units[0];

    assert_debug_snapshot!(init_foo_unit, @r###"
    POU {
        name: "__init_foo",
        variable_blocks: [
            VariableBlock {
                variables: [
                    Variable {
                        name: "self",
                        data_type: DataTypeReference {
                            referenced_type: "foo",
                        },
                    },
                ],
                variable_block_type: InOut,
            },
        ],
        pou_type: Function,
        return_type: None,
    }
    "###);
}

/// The thusly created function takes a single argument, a pointer to an instance of the POU to be initialized.
/// In its body, new `AstStatements` will be created; either assigning the initializer value or, for types which
/// have complex initializers themselves, calling the corresponding init function with the member-instance.
#[test]
fn initializers_are_assigned_or_delegated_to_respective_init_functions() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        FUNCTION_BLOCK foo
        VAR
            s : STRING;
            ps: REF_TO STRING := REF(s);
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar
        VAR
            fb: foo;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM baz
        VAR
            d: DINT;
            pd AT d : DINT;
            fb: bar;
        END_VAR
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    let (_, _, annotated_units) = annotate_and_lower_with_ids(unit, index, id_provider);

    let units = annotated_units.iter().map(|(units, _, _)| units).collect::<Vec<_>>();
    // the init-function for `foo` is expected to have a single assignment statement in its function body
    let init_foo_impl = &units[1].implementations[0];
    assert_eq!(&init_foo_impl.name, "__init_foo");
    let statements = &init_foo_impl.statements;
    assert_eq!(statements.len(), 1);
    assert_debug_snapshot!(statements[0], @r###"
    Assignment {
        left: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "ps",
                },
            ),
            base: Some(
                ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "self",
                        },
                    ),
                    base: None,
                },
            ),
        },
        right: CallStatement {
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
                            name: "s",
                        },
                    ),
                    base: None,
                },
            ),
        },
    }
    "###);

    // the init-function for `bar` will have a `CallStatement` to `__init_foo` as its only statement, passing the member-instance `self.fb`
    let init_bar_impl = &units[1].implementations[2];
    assert_eq!(&init_bar_impl.name, "__init_bar");
    let statements = &init_bar_impl.statements;
    assert_eq!(statements.len(), 1);
    assert_debug_snapshot!(statements[0], @r###"
    CallStatement {
        operator: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "__init_foo",
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
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "self",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        ),
    }
    "###);

    // the init-function for `baz` will have an `Assignment`, assigning `REF(d)` to `self.pd` followed by
    // a `CallStatement` to `__init_bar`, passing the member-instance `self.fb`
    let init_baz_impl = &units[1].implementations[1];
    assert_eq!(&init_baz_impl.name, "__init_baz");
    let statements = &init_baz_impl.statements;
    assert_eq!(statements.len(), 2);
    assert_debug_snapshot!(statements[0], @r###"
    Assignment {
        left: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "pd",
                },
            ),
            base: Some(
                ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "self",
                        },
                    ),
                    base: None,
                },
            ),
        },
        right: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "d",
                },
            ),
            base: None,
        },
    }
    "###);

    assert_debug_snapshot!(statements[1], @r###"
    CallStatement {
        operator: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "__init_bar",
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
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "self",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        ),
    }
    "###);
}

/// Finally, after lowering each individual init function, all global initializers are
/// collected and wrapped in a single `__init` function. This function does not take any arguments,
/// since it only deals with global symbols.
/// Simple global variables with `REF` initializers have their respective addresses assigned,
/// PROGRAM instances will have call statements to their initialization functions generated,
/// passing the global instance as argument
/// TODO: support global struct/function block instances
#[test]
fn global_initializers_are_wrapped_in_single_init_function() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        VAR_GLOBAL
            s : STRING;
            gs : REFERENCE TO STRING := REF(s);
        END_VAR

        FUNCTION_BLOCK foo
        VAR
            ps: REF_TO STRING := REF(s);
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM bar
        VAR
            fb: foo;
        END_VAR
        END_PROGRAM

        PROGRAM baz
        VAR
            fb: foo;
        END_VAR
        END_PROGRAM

        PROGRAM qux
        VAR
            fb: foo;
        END_VAR
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    let (_, index, annotated_units) = annotate_and_lower_with_ids(unit, index, id_provider);
    assert!(index.find_pou("__init").is_some());

    let units = annotated_units.iter().map(|(units, _, _)| units).collect::<Vec<_>>();

    let init = &units[2].units[0];
    assert_debug_snapshot!(init, @r###"
    POU {
        name: "__init",
        variable_blocks: [],
        pou_type: Function,
        return_type: None,
    }
    "###);

    let init_impl = &units[2].implementations[0];
    assert_eq!(&init_impl.name, "__init");
    assert_eq!(init_impl.statements.len(), 4);
    // global variable blocks are initialized first, hence we expect the first statement in the `__init` body to be an
    // `Assignment`, assigning `REF(s)` to `gs`. This is followed by three `CallStatements`, one for each global `PROGRAM`
    // instance.
    assert_debug_snapshot!(&init_impl.statements[0], @r###"
    Assignment {
        left: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "gs",
                },
            ),
            base: None,
        },
        right: CallStatement {
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
                            name: "s",
                        },
                    ),
                    base: None,
                },
            ),
        },
    }
    "###);
    assert_debug_snapshot!(&init_impl.statements[1], @r###"
    CallStatement {
        operator: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "__init_bar",
                },
            ),
            base: None,
        },
        parameters: Some(
            ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "bar",
                    },
                ),
                base: None,
            },
        ),
    }
    "###);
    assert_debug_snapshot!(&init_impl.statements[2], @r###"
    CallStatement {
        operator: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "__init_baz",
                },
            ),
            base: None,
        },
        parameters: Some(
            ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "baz",
                    },
                ),
                base: None,
            },
        ),
    }
    "###);
    assert_debug_snapshot!(&init_impl.statements[3], @r###"
    CallStatement {
        operator: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "__init_qux",
                },
            ),
            base: None,
        },
        parameters: Some(
            ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "qux",
                    },
                ),
                base: None,
            },
        ),
    }
    "###);
}
