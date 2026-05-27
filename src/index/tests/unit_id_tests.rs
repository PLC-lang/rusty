//! Tests for the per-unit symbol-ownership plumbing: `Index::import_with_unit`
//! records contributions, and `Index::remove_unit` drops exactly those
//! contributions even when multiple units share a `SymbolMap` key.

use plc_ast::ast::LinkageType;
use plc_source::source_location::SourceLocation;

use crate::index::{ImplementationType, Index, PouIndexEntry, UnitId};
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

fn unit_with_auto_gen_function(name: &str) -> Index {
    let mut idx = Index::default();
    idx.register_pou(PouIndexEntry::create_generated_function_entry(
        name,
        "DINT",
        &[],
        LinkageType::Internal,
        false,
        SourceLocation::internal(),
        false,
    ));
    idx
}

fn unit_with_implementation(call_name: &str, type_name: &str) -> Index {
    let mut idx = Index::default();
    idx.register_implementation(
        call_name,
        type_name,
        None,
        ImplementationType::Function,
        false,
        SourceLocation::internal(),
    );
    idx
}

#[test]
fn remove_unit_preserves_auto_generated_pou_shared_with_other_unit() {
    // Two source units that both register the same auto-generated POU. Dropping
    // the first must not remove the entry while the second still owns it; the
    // entry only vanishes once the last owner is removed.
    let u1 = unit_with_auto_gen_function("__init_foo");
    let u2 = unit_with_auto_gen_function("__init_foo");

    let mut combined = Index::default();
    combined.import_with_unit(u1, UnitId::source(0));
    combined.import_with_unit(u2, UnitId::source(1));

    assert!(combined.find_pou("__init_foo").is_some(), "shared auto-gen must be present after both imports");

    combined.remove_unit(UnitId::source(0));
    assert!(
        combined.find_pou("__init_foo").is_some(),
        "auto-gen must survive removal of one owner while another still claims it"
    );

    combined.remove_unit(UnitId::source(1));
    assert!(combined.find_pou("__init_foo").is_none(), "auto-gen is gone once the last owner is removed");
}

#[test]
fn reindex_unit_with_shared_auto_gen_pou_is_idempotent() {
    // Re-importing a unit that contributes an auto-gen POU shared with another
    // unit must leave the index byte-equivalent (in observable shape) to the
    // pre-removal state.
    let u1 = unit_with_auto_gen_function("__init_bar");
    let u2 = unit_with_auto_gen_function("__init_bar");

    let mut combined = Index::default();
    combined.import_with_unit(u1, UnitId::source(0));
    combined.import_with_unit(u2, UnitId::source(1));

    combined.remove_unit(UnitId::source(0));
    combined.import_with_unit(unit_with_auto_gen_function("__init_bar"), UnitId::source(0));

    assert!(
        combined.find_pou("__init_bar").is_some(),
        "auto-gen must still be present after reindexing one owner"
    );

    // Now drop both owners; the entry should vanish.
    combined.remove_unit(UnitId::source(0));
    combined.remove_unit(UnitId::source(1));
    assert!(combined.find_pou("__init_bar").is_none());
}

#[test]
fn remove_unit_preserves_implementation_shared_with_other_unit() {
    // Two units that register implementations under the same call name. The
    // import path overwrites the IndexMap entry, but both owners are recorded;
    // dropping one must leave the entry standing for the other.
    let u1 = unit_with_implementation("shared_impl", "shared_impl");
    let u2 = unit_with_implementation("shared_impl", "shared_impl");

    let mut combined = Index::default();
    combined.import_with_unit(u1, UnitId::source(0));
    combined.import_with_unit(u2, UnitId::source(1));

    assert!(combined.find_implementation_by_name("shared_impl").is_some());

    combined.remove_unit(UnitId::source(0));
    assert!(
        combined.find_implementation_by_name("shared_impl").is_some(),
        "implementation must survive removal of one owner while another still claims it"
    );

    combined.remove_unit(UnitId::source(1));
    assert!(combined.find_implementation_by_name("shared_impl").is_none());
}
