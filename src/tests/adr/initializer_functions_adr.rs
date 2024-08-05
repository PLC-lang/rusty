use insta::assert_debug_snapshot;
use plc_ast::provider::IdProvider;

use crate::{index::const_expressions::UnresolvableKind, resolver::const_evaluator::evaluate_constants, test_utils::tests::{annotate_and_lower_with_ids, index, index_with_ids}};

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
    assert_eq!(
        unresolvable[0].get_reason(),
        Some(r#"Try to re-resolve during codegen"#)
    );
    
    let Some(UnresolvableKind::Address(_)) = unresolvable[0].kind else {
        panic!()
    };
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
    dbg!(&units[1]);
    
    // the init-function for `foo` is expected to have a single assignment statement in its function body
    let init_foo_impl = &units[1].implementations[0];
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


    // the init-function for `bar` will have a `CallStatement` to `__init_foo` as its only statement, passing the member-instance `bar.foo`
    let init_bar_impl = &units[1].implementations[1];
    let statements = &init_bar_impl.statements;
    assert_eq!(statements.len(), 1);
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
}
