use driver::{parse_and_annotate, pipelines::AnnotatedProject};
use insta::assert_debug_snapshot;
use plc_ast::ast::PouType;
use plc_source::SourceCode;

#[test]
fn function_block_init_fn_created() {
    // GIVEN a function block with a ref initializer
    // WHEN lowered
    let (_, annotated_project) = parse_and_annotate(
        "Test",
        vec![SourceCode::from(
            "
           FUNCTION_BLOCK foo
        VAR
            s : STRING;
            ps: REF_TO STRING := REF(s);
        END_VAR
        END_FUNCTION_BLOCK
            ",
        )],
    )
    .unwrap();
    let AnnotatedProject { units, index, .. } = annotated_project;
    let units = units.iter().map(|unit| unit.get_unit()).collect::<Vec<_>>();
    // THEN we expect the index to now have a corresponding init function
    assert!(index.find_pou("__init_foo").is_some());
    // AND we expect a new function to be created for it
    let init_foo = &units[1];
    let implementation = &init_foo.implementations[1];
    assert_eq!(implementation.name, "__init_foo");
    assert_eq!(implementation.pou_type, PouType::Init);

    // we expect this function to have a single parameter "self", being an instance of the initialized POU
    assert_debug_snapshot!(init_foo.pous[1].variable_blocks[0].variables[0], @r#"
    Variable {
        name: "self",
        data_type: DataTypeReference {
            referenced_type: "foo",
        },
    }
    "#);

    let statements = &implementation.statements;
    assert_eq!(statements.len(), 2);
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "__vtable",
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
                            name: "ADR",
                        },
                    ),
                    base: None,
                },
                parameters: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__vtable_foo_instance",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        },
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
            },
        },
    ]
    "#);
}

#[test]
fn program_init_fn_created() {
    // GIVEN a program with a ref initializer
    // WHEN lowered
    let (_, annotated_project) = parse_and_annotate(
        "Test",
        vec![SourceCode::from(
            "
   PROGRAM foo
        VAR
            s : STRING;
            ps: REF_TO STRING := REF(s);
        END_VAR
        END_PROGRAM
            ",
        )],
    )
    .unwrap();
    let AnnotatedProject { units, index, .. } = annotated_project;
    let units = units.iter().map(|unit| unit.get_unit()).collect::<Vec<_>>();
    // THEN we expect the index to now have a corresponding init function
    assert!(index.find_pou("__init_foo").is_some());
    // AND we expect a new function to be created for it
    let init_foo = &units[1];
    let implementation = &init_foo.implementations[0];
    assert_eq!(implementation.name, "__init_foo");
    assert_eq!(implementation.pou_type, PouType::Init);

    // we expect this function to have a single parameter "self", being an instance of the initialized POU
    assert_debug_snapshot!(init_foo.pous[0].variable_blocks[0].variables[0], @r#"
    Variable {
        name: "self",
        data_type: DataTypeReference {
            referenced_type: "foo",
        },
    }
    "#);

    // this init-function is expected to have a single assignment statement in its function body
    let statements = &implementation.statements;
    assert_eq!(statements.len(), 1);
    assert_debug_snapshot!(statements[0], @r#"
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
        },
    }
    "#);
}

#[test]
fn init_wrapper_function_created() {
    let (_, annotated_project) = parse_and_annotate(
        "Test",
        vec![SourceCode::from(
            "
        VAR_GLOBAL
            s : STRING;
            gs : REFERENCE TO STRING := REF(s);
        END_VAR

        FUNCTION_BLOCK bar
        VAR
            ps AT s : STRING;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM foo
        VAR
            fb: bar;
        END_VAR
        END_PROGRAM
            ",
        )],
    )
    .unwrap();
    let AnnotatedProject { units, index, .. } = annotated_project;
    let units = units.iter().map(|unit| unit.get_unit()).collect::<Vec<_>>();

    // we expect there to be 3 `CompilationUnit`s, one for the original source, one with pou initializer functions, and finally
    // one for the `__init` wrapper
    assert_eq!(units.len(), 3);

    // we expect the index to now have an `__init` function for our `TestProject`
    assert!(index.find_pou("__init___Test").is_some());

    // we expect a new function to be created for it
    let init = &units[2];
    let implementation = &init.implementations[0];
    assert_eq!(implementation.name, "__init___Test");
    assert_eq!(implementation.pou_type, PouType::ProjectInit);

    // we expect this function to have no parameters
    assert!(init.pous[0].variable_blocks.is_empty());

    // we expect to the body to have 3 statements
    let statements = &implementation.statements;

    assert_debug_snapshot!(statements, @r#"
    [
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
                            name: "foo",
                        },
                    ),
                    base: None,
                },
            ),
        },
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "__init___vtable_bar",
                    },
                ),
                base: None,
            },
            parameters: Some(
                ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "__vtable_bar_instance",
                        },
                    ),
                    base: None,
                },
            ),
        },
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
        },
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "__user_init_foo",
                    },
                ),
                base: None,
            },
            parameters: Some(
                ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "foo",
                        },
                    ),
                    base: None,
                },
            ),
        },
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "__user_init___vtable_bar",
                    },
                ),
                base: None,
            },
            parameters: Some(
                ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "__vtable_bar_instance",
                        },
                    ),
                    base: None,
                },
            ),
        },
    ]
    "#);

    // since `foo` has a member-instance of `bar`, we expect its initializer to call/propagate to `__init_bar` with its local member
    let init_foo = &units[1].implementations[2];
    assert_debug_snapshot!(init_foo.statements[0], @r#"
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
    "#);
}
