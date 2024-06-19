//! Contains tests related to lowering, i.e. (for now) anything that uses the `ReplacementAst`.

use insta::assert_debug_snapshot;
use plc_ast::provider::IdProvider;

use crate::{
    resolver::AnnotationMap,
    test_utils::tests::{annotate_with_ids, index_with_ids},
};

#[test]
fn temp() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        FUNCTION main
            VAR
                foo : DINT;
                bar AT foo : DINT;
            END_VAR
        END_FUNCTION
        ",
        id_provider.clone(),
    );

    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let initializer_bar = unit.units[0].variable_blocks[0].variables[1].initializer.as_ref().unwrap();
    let initializer_bar_annotation = annotations.get(initializer_bar).unwrap();

    assert_debug_snapshot!((initializer_bar, initializer_bar_annotation), @r###"
    (
        ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "foo",
                },
            ),
            base: None,
        },
        ReplacementAst {
            statement: CallStatement {
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
                                name: "foo",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        },
    )
    "###);
}

#[test]
fn temp3() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        FUNCTION main
            VAR
                foo : ARRAY[1..5] OF DINT;
                bar AT foo[1] : DINT;
            END_VAR
        END_FUNCTION
        ",
        id_provider.clone(),
    );

    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let initializer_bar = unit.units[0].variable_blocks[0].variables[1].initializer.as_ref().unwrap();
    let initializer_bar_annotation = annotations.get(initializer_bar).unwrap();

    assert_debug_snapshot!((initializer_bar, initializer_bar_annotation), @r###"
    (
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
                            name: "foo",
                        },
                    ),
                    base: None,
                },
            ),
        },
        ReplacementAst {
            statement: CallStatement {
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
                        kind: Index(
                            LiteralInteger {
                                value: 1,
                            },
                        ),
                        base: Some(
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
                ),
            },
        },
    )
    "###);
}
