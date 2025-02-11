use insta::assert_debug_snapshot;
use plc_ast::provider::IdProvider;

use crate::{resolver::AnnotationMap, test_utils::tests::{annotate_and_lower_with_ids, index_and_lower}};


#[test]
fn overriden_method_is_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index, _) = index_and_lower(r#"
        FUNCTION_BLOCK base
            METHOD foo : BOOL
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK derived EXTENDS base
            METHOD foo : BOOL
            END_METHOD
        END_FUNCTION_BLOCK
    "#, id_provider.clone());

    let (annotations, _, units) = annotate_and_lower_with_ids(unit, index, id_provider);

    let unit = &units[0].0.units[3];
    assert_debug_snapshot!(annotations.get_with_id(unit.id), @r###""###);

}

#[test]
fn overriden_method_from_multiple_interfaces_is_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index, _) = index_and_lower(r#"
        INTERFACE base
            METHOD foo : BOOL
            END_METHOD
        END_INTERFACE

        INTERFACE base2
            METHOD foo : BOOL
            END_METHOD
            METHOD bar : BOOL
            END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK derived IMPLEMENTS base,base2
            METHOD foo : BOOL
            END_METHOD
            METHOD bar : BOOL
            END_METHOD
        END_FUNCTION_BLOCK
    "#, id_provider.clone());

    let (annotations, _, units) = annotate_and_lower_with_ids(unit, index, id_provider);

    let unit = &units[0].0.units[1];
    assert_debug_snapshot!(annotations.get_with_id(unit.id), @r###""###);
    let unit = &units[0].0.units[2];
    assert_debug_snapshot!(annotations.get_with_id(unit.id), @r###""###);

}

#[test]
fn overriden_method_from_interface_is_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index, _) = index_and_lower(r#"
        INTERFACE base
            METHOD foo : BOOL
            END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK derived IMPLEMENTS base
            METHOD foo : BOOL
            END_METHOD
        END_FUNCTION_BLOCK
    "#, id_provider.clone());

    let (annotations, _, units) = annotate_and_lower_with_ids(unit, index, id_provider);

    let unit = &units[0].0.units[1];
    assert_debug_snapshot!(annotations.get_with_id(unit.id), @r###""###);

}

#[test]
fn overriden_method_from_interface_and_base_is_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index, _) = index_and_lower(r#"
        FUNCTION_BLOCK base
            METHOD foo : BOOL
            END_METHOD
        END_FUNCTION_BLOCK

        INTERFACE base2
            METHOD foo : BOOL
            END_METHOD
            METHOD bar : BOOL
            END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK derived EXTENDS base IMPLEMENTS base2
            METHOD foo : BOOL
            END_METHOD
            METHOD bar : BOOL
            END_METHOD
        END_FUNCTION_BLOCK
    "#, id_provider.clone());

    let (annotations, _, units) = annotate_and_lower_with_ids(unit, index, id_provider);

    let unit = &units[0].0.units[3];
    assert_debug_snapshot!(annotations.get_with_id(unit.id), @r###""###);
    let unit = &units[0].0.units[4];
    assert_debug_snapshot!(annotations.get_with_id(unit.id), @r###""###);

}