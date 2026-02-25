//! Contains tests related to lowering, i.e. (for now) anything that uses the `ReplacementAst`.

use insta::assert_debug_snapshot;
use plc_ast::provider::IdProvider;

use crate::{
    resolver::AnnotationMap,
    test_utils::tests::{annotate_with_ids, index_with_ids},
};

#[test]
fn initializer_with_ref_call_annotated_as_pointer() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        FUNCTION main
            VAR
                foo : DINT;
                bar : REFERENCE TO DINT := REF(foo);
            END_VAR
        END_FUNCTION
        ",
        id_provider.clone(),
    );

    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let initializer_bar = unit.pous[0].variable_blocks[0].variables[1].initializer.as_ref().unwrap();
    let initializer_bar_annotation = annotations.get(initializer_bar).unwrap();

    assert_debug_snapshot!((initializer_bar, initializer_bar_annotation), @r#"
    (
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
                            name: "foo",
                        },
                    ),
                    base: None,
                },
            ),
        },
        Value {
            resulting_type: "__POINTER_TO_DINT",
        },
    )
    "#);
}

#[test]
fn initializer_with_refassignment_annotated_with_replacementast() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        "
        FUNCTION main
            VAR
                foo : DINT;
                bar : REFERENCE TO DINT REF= foo;
            END_VAR
        END_FUNCTION
        ",
        id_provider.clone(),
    );

    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let initializer_bar = unit.pous[0].variable_blocks[0].variables[1].initializer.as_ref().unwrap();
    let initializer_bar_annotation = annotations.get(initializer_bar).unwrap();

    assert_debug_snapshot!((initializer_bar, initializer_bar_annotation), @r#"
    (
        ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "foo",
                },
            ),
            base: None,
        },
        Variable {
            resulting_type: "DINT",
            qualified_name: "main.foo",
            constant: false,
            argument_type: ByVal(
                Local,
            ),
            auto_deref: None,
        },
    )
    "#);
}

#[test]
fn initializer_of_alias_annotated_with_replacementast() {
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
    let initializer_bar = unit.pous[0].variable_blocks[0].variables[1].initializer.as_ref().unwrap();
    let initializer_bar_annotation = annotations.get(initializer_bar).unwrap();

    assert_debug_snapshot!((initializer_bar, initializer_bar_annotation), @r#"
    (
        ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "foo",
                },
            ),
            base: None,
        },
        Variable {
            resulting_type: "DINT",
            qualified_name: "main.foo",
            constant: false,
            argument_type: ByVal(
                Local,
            ),
            auto_deref: None,
        },
    )
    "#);
}

#[test]
fn initializer_of_alias_annotated_with_replacementast_array() {
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
    let initializer_bar = unit.pous[0].variable_blocks[0].variables[1].initializer.as_ref().unwrap();
    let initializer_bar_annotation = annotations.get(initializer_bar).unwrap();

    assert_debug_snapshot!((initializer_bar, initializer_bar_annotation), @r#"
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
        Value {
            resulting_type: "DINT",
        },
    )
    "#);
}
