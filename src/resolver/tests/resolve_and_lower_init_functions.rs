use insta::assert_debug_snapshot;
use plc_ast::{ast::PouType, provider::IdProvider};

use crate::test_utils::tests::{annotate_and_lower_with_ids, index_with_ids};

#[test]
fn function_block_init_fn_created() {
    let id_provider = IdProvider::default();
    // GIVEN a function block with a ref initializer
    // WHEN lowered
    let (unit, index) = index_with_ids("
        FUNCTION_BLOCK foo
        VAR
            s : STRING;
            ps: REF_TO STRING := REF(s);
        END_VAR
        END_FUNCTION_BLOCK
        ",
        id_provider.clone(),
    );
    let (_, index, annotated_units) = annotate_and_lower_with_ids(unit, index, id_provider);

    // THEN we expect the index to now have a corresponding init function
    assert!(index.find_pou("__init_foo").is_some());
    // AND we expect a new function to be created for it
    let units = annotated_units.iter().map(|(units, _, _)| units).collect::<Vec<_>>();
    let init_foo = &units[1];
    let implementation = &init_foo.implementations[0];
    assert_eq!(implementation.name, "__init_foo");
    assert_eq!(implementation.pou_type, PouType::Init);

    // we expect this function to have a single parameter "self", being an instance of the initialized POU
    assert_debug_snapshot!(init_foo.units[0].variable_blocks[0].variables[0], @r###"
    Variable {
        name: "self",
        data_type: DataTypeReference {
            referenced_type: "foo",
        },
    }
    "###);

    // this init-function is expected to have a single assignment statement in its function body
    let statements = &implementation.statements;
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
}

#[test]
fn program_init_fn_created() {
    let id_provider = IdProvider::default();
    // GIVEN a program with a ref initializer
    // WHEN lowered
    let (unit, index) = index_with_ids("
        PROGRAM foo
        VAR
            s : STRING;
            ps: REF_TO STRING := REF(s);
        END_VAR
        END_PROGRAM
        ",
        id_provider.clone(),
    );
    let (_, index, annotated_units) = annotate_and_lower_with_ids(unit, index, id_provider);
    // THEN we expect the index to now have a corresponding init function
    assert!(index.find_pou("__init_foo").is_some());
    // AND we expect a new function to be created for it
    let units = annotated_units.iter().map(|(units, _, _)| units).collect::<Vec<_>>();
    let init_foo = &units[1];
    let implementation = &init_foo.implementations[0];
    assert_eq!(implementation.name, "__init_foo");
    assert_eq!(implementation.pou_type, PouType::Init);

    // we expect this function to have a single parameter "self", being an instance of the initialized POU
    assert_debug_snapshot!(init_foo.units[0].variable_blocks[0].variables[0], @r###"
    Variable {
        name: "self",
        data_type: DataTypeReference {
            referenced_type: "foo",
        },
    }
    "###);

    // this init-function is expected to have a single assignment statement in its function body
    let statements = &implementation.statements;
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
}

#[test]
fn init_wrapper_function_created() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids("
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
        id_provider.clone(),
    );
    let (_, index, annotated_units) = annotate_and_lower_with_ids(unit, index, id_provider);
    let units = annotated_units.iter().map(|(units, _, _)| units).collect::<Vec<_>>();

    // we expect there to be 3 `CompilationUnit`s, one for the original source, one with pou initializer functions, and finally
    // one for the `__init` wrapper
    assert_eq!(units.len(), 3);

    // we expect the index to now have an `__init` function for our `TestProject`
    assert!(index.find_pou("__init___testproject").is_some());

    // we expect a new function to be created for it
    let init = &units[2];
    let implementation = &init.implementations[0];
    assert_eq!(implementation.name, "__init___testproject");
    assert_eq!(implementation.pou_type, PouType::Init);

    // we expect this function to have no parameters
    assert!(init.units[0].variable_blocks.is_empty());

    // we expect to the body to have 2 statements
    let statements = &implementation.statements;
    assert_eq!(statements.len(), 2);

    // we expect the first statement in the function-body to assign `REF(s)` to `gs`, since
    // global variables are to be initialized first
    assert_debug_snapshot!(statements[0], @r###"
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

    // we expect the second statement to call `__init_foo`, passing its global instance
    assert_debug_snapshot!(statements[1], @r###"
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
    }
    "###);

    // since `foo` has a member-instance of `bar`, we expect its initializer to call/propagate to `__init_bar` with its local member
    let init_foo = &units[1].implementations[1];
    assert_debug_snapshot!(init_foo.statements[0], @r###"
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
