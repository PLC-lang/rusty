//! Tests for the per-unit symbol-ownership plumbing: `Index::import_with_unit`
//! records contributions, and `Index::remove_unit` drops exactly those
//! contributions even when multiple units share a `SymbolMap` key.

use crate::index::{Index, UnitId};
use crate::test_utils::tests::index;

#[test]
fn remove_unit_drops_source_globals() {
    let source = r#"
        VAR_GLOBAL
            a : INT;
            b : DINT;
        END_VAR
    "#;
    let (_, mut global) = index(source);

    assert!(global.find_global_variable("a").is_some());
    assert!(global.find_global_variable("b").is_some());

    global.remove_unit(UnitId::source(0));

    assert!(global.find_global_variable("a").is_none());
    assert!(global.find_global_variable("b").is_none());
    // Built-in types are untouched.
    assert!(global.find_effective_type_by_name("INT").is_some());
}

#[test]
fn remove_unit_drops_source_pous() {
    let (_, mut global) = index(
        r#"
            FUNCTION foo : INT
            END_FUNCTION
            PROGRAM Main
            VAR
                x : INT;
            END_VAR
            END_PROGRAM
        "#,
    );

    assert!(global.find_pou("foo").is_some());
    assert!(global.find_pou("Main").is_some());

    global.remove_unit(UnitId::source(0));

    assert!(global.find_pou("foo").is_none());
    assert!(global.find_pou("Main").is_none());
}

#[test]
fn remove_unit_preserves_other_units_enum_variant_with_same_short_name() {
    // Two enums in different units that both expose a variant named `RED`.
    // The variants land in `enum_global_variables` under the lowercase short
    // name `red`, so the map key is shared but the qualified names differ.
    let e1 = build_enum_only_index("E1");
    let e2 = build_enum_only_index("E2");

    let mut combined = Index::default();
    combined.import_with_unit(e1, UnitId::source(0));
    combined.import_with_unit(e2, UnitId::source(1));

    // Both qualified variants are reachable before removal.
    assert!(combined.find_enum_variant("E1", "RED").is_some());
    assert!(combined.find_enum_variant("E2", "RED").is_some());

    // Drop only the first unit's contributions.
    combined.remove_unit(UnitId::source(0));

    assert!(combined.find_enum_variant("E1", "RED").is_none(), "E1.RED must be gone");
    assert!(combined.find_enum_variant("E2", "RED").is_some(), "E2.RED must survive");
}

fn build_enum_only_index(enum_name: &str) -> Index {
    let source = format!(
        r#"
            TYPE {enum_name} : (RED, GREEN, BLUE); END_TYPE
        "#
    );
    let (_, index) = index(source.as_str());
    index
}
