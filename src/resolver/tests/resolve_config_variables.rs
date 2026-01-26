use insta::assert_debug_snapshot;
use plc_ast::provider::IdProvider;

use crate::{
    resolver::{AnnotationMap, TypeAnnotator},
    test_utils::tests::index_with_ids,
};

#[test]
fn var_config_references_get_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        FUNCTION_BLOCK fb
        VAR
            b AT %I* : BOOL;
        END_VAR
        END_FUNCTION_BLOCK
        VAR_GLOBAL 
            bar: fb;
        END_VAR
        VAR_CONFIG
            bar.b AT %IX1.0 : BOOL;
        END_VAR
        ",
        id_provider.clone(),
    );
    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let reference = &unit.var_config[0].reference;

    assert_debug_snapshot!(annotations.get(reference), @r#"
    Some(
        Variable {
            resulting_type: "BOOL",
            qualified_name: "fb.b",
            constant: false,
            argument_type: ByVal(
                Local,
            ),
            auto_deref: Some(
                Alias(
                    "__fb_b",
                ),
            ),
        },
    )
    "#);
}

#[test]
fn var_config_with_multiple_qualifiers_gets_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        FUNCTION_BLOCK fb
        VAR
            b AT %I* : BOOL;
        END_VAR
        END_FUNCTION_BLOCK
        FUNCTION_BLOCK base
        VAR
            bar: fb;
        END_VAR
        END_FUNCTION_BLOCK
        PROGRAM prog
        VAR
            foo: base;
        END_VAR
        END_PROGRAM
        VAR_CONFIG
            prog.foo.bar.b AT %IX1.0 : BOOL;
        END_VAR
        ",
        id_provider.clone(),
    );
    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let reference = &unit.var_config[0].reference;

    assert_debug_snapshot!(annotations.get(reference), @r#"
    Some(
        Variable {
            resulting_type: "BOOL",
            qualified_name: "fb.b",
            constant: false,
            argument_type: ByVal(
                Local,
            ),
            auto_deref: Some(
                Alias(
                    "__fb_b",
                ),
            ),
        },
    )
    "#);
}

#[test]
fn var_config_array_access_gets_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        FUNCTION_BLOCK fb
        VAR
            b AT %I* : BOOL;
        END_VAR
        END_FUNCTION_BLOCK
        FUNCTION_BLOCK base
        VAR
            bar: fb;
        END_VAR
        END_FUNCTION_BLOCK
        PROGRAM prog
        VAR
            foo: ARRAY[0..10] OF base;
        END_VAR
        END_PROGRAM
        VAR_CONFIG
            prog.foo[0].bar.b AT %IX1.0 : BOOL;
        END_VAR
        ",
        id_provider.clone(),
    );
    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let reference = &unit.var_config[0].reference;

    assert_debug_snapshot!(annotations.get(reference), @r#"
    Some(
        Variable {
            resulting_type: "BOOL",
            qualified_name: "fb.b",
            constant: false,
            argument_type: ByVal(
                Local,
            ),
            auto_deref: Some(
                Alias(
                    "__fb_b",
                ),
            ),
        },
    )
    "#);
}

#[test]
fn var_config_array_access_with_const_expr_gets_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        VAR_GLOBAL CONSTANT
            START: DINT := 2;
            END: DINT := 10;
        END_VAR
        FUNCTION_BLOCK fb
        VAR
            b AT %I* : LWORD;
        END_VAR
        END_FUNCTION_BLOCK
        FUNCTION_BLOCK base
        VAR
            bar: fb;
        END_VAR
        END_FUNCTION_BLOCK
        PROGRAM prog
        VAR
            foo: ARRAY[START..END] OF base;
        END_VAR
        END_PROGRAM
        VAR_CONFIG
            prog.foo[END - START + ((END-START) / 2)].bar.b AT %IX1.0 : LWORD;
            prog.foo[1].bar.b AT %IX1.0 : LWORD;
        END_VAR
        ",
        id_provider.clone(),
    );
    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let reference = &unit.var_config[0].reference;

    assert_debug_snapshot!(annotations.get(reference), @r#"
    Some(
        Variable {
            resulting_type: "LWORD",
            qualified_name: "fb.b",
            constant: false,
            argument_type: ByVal(
                Local,
            ),
            auto_deref: Some(
                Alias(
                    "__fb_b",
                ),
            ),
        },
    )
    "#);
}

#[test]
fn var_config_flat_global_reference_is_resolved() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        VAR_GLOBAL 
            bar AT %I* : BOOL;
        END_VAR
        VAR_CONFIG
            bar AT %IX1.0 : BOOL;
        END_VAR
        ",
        id_provider.clone(),
    );
    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    let reference = &unit.var_config[0].reference;

    assert_debug_snapshot!(annotations.get(reference), @r#"
    Some(
        Variable {
            resulting_type: "BOOL",
            qualified_name: "bar",
            constant: false,
            argument_type: ByVal(
                Global,
            ),
            auto_deref: Some(
                Alias(
                    "__global_bar",
                ),
            ),
        },
    )
    "#);
}
