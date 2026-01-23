use insta::assert_debug_snapshot;
use plc_ast::provider::IdProvider;

use crate::{
    resolver::AnnotationMap,
    test_utils::tests::{annotate_and_lower_with_ids, index_and_lower},
};

#[test]
fn overriden_method_is_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index, _) = index_and_lower(
        r#"
        FUNCTION_BLOCK base
            METHOD foo : BOOL
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK derived EXTENDS base
            METHOD foo : BOOL
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
        id_provider.clone(),
    );

    let (annotations, _, unit) = annotate_and_lower_with_ids(unit, index, id_provider);

    let unit = &unit.0.pous[3];
    assert_debug_snapshot!(annotations.get_with_id(unit.id), @r#"
    Some(
        Override {
            definitions: [
                Concrete(
                    "base.foo",
                ),
            ],
        },
    )
    "#);
}

#[test]
fn overriden_method_from_multiple_interfaces_is_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index, _) = index_and_lower(
        r#"
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
    "#,
        id_provider.clone(),
    );

    let (annotations, _, units) = annotate_and_lower_with_ids(unit, index, id_provider);

    let unit = &units.0.pous[1];
    assert_debug_snapshot!(annotations.get_with_id(unit.id), @r#"
    Some(
        Override {
            definitions: [
                Abstract(
                    "base.foo",
                ),
                Abstract(
                    "base2.foo",
                ),
            ],
        },
    )
    "#);
    let unit = &units.0.pous[2];
    assert_debug_snapshot!(annotations.get_with_id(unit.id), @r#"
    Some(
        Override {
            definitions: [
                Abstract(
                    "base2.bar",
                ),
            ],
        },
    )
    "#);
}

#[test]
fn overriden_method_from_interface_is_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index, _) = index_and_lower(
        r#"
        INTERFACE base
            METHOD foo : BOOL
            END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK derived IMPLEMENTS base
            METHOD foo : BOOL
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
        id_provider.clone(),
    );

    let (annotations, _, units) = annotate_and_lower_with_ids(unit, index, id_provider);

    let unit = &units.0.pous[1];
    assert_debug_snapshot!(annotations.get_with_id(unit.id), @r#"
    Some(
        Override {
            definitions: [
                Abstract(
                    "base.foo",
                ),
            ],
        },
    )
    "#);
}

#[test]
fn overriden_method_from_interface_and_base_is_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index, _) = index_and_lower(
        r#"
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
    "#,
        id_provider.clone(),
    );

    let (annotations, _, units) = annotate_and_lower_with_ids(unit, index, id_provider);

    let unit = &units.0.pous[3];
    assert_debug_snapshot!(annotations.get_with_id(unit.id), @r#"
    Some(
        Override {
            definitions: [
                Concrete(
                    "base.foo",
                ),
                Abstract(
                    "base2.foo",
                ),
            ],
        },
    )
    "#);
    let unit = &units.0.pous[4];
    assert_debug_snapshot!(annotations.get_with_id(unit.id), @r#"
    Some(
        Override {
            definitions: [
                Abstract(
                    "base2.bar",
                ),
            ],
        },
    )
    "#);
}

#[test]
fn all_available_methods_of_container_are_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index, _) = index_and_lower(
        r#"
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
    "#,
        id_provider.clone(),
    );

    let (annotations, _, units) = annotate_and_lower_with_ids(unit, index, id_provider);

    let unit = &units.0.pous[2];
    assert_debug_snapshot!(annotations.get_with_id(unit.id), @r#"
    Some(
        MethodDeclarations {
            declarations: {
                "bar": [
                    Concrete(
                        "derived.bar",
                    ),
                    Abstract(
                        "base2.bar",
                    ),
                ],
                "foo": [
                    Concrete(
                        "derived.foo",
                    ),
                    Abstract(
                        "base2.foo",
                    ),
                ],
            },
        },
    )
    "#);
}

#[test]
fn all_available_methods_of_interface_are_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index, _) = index_and_lower(
        r#"
        INTERFACE foo
            METHOD bar
            END_METHOD
        END_INTERFACE

        INTERFACE baz EXTENDS foo
            METHOD qux
            END_METHOD
        END_INTERFACE

        INTERFACE quux
            METHOD corge
            END_METHOD
        END_INTERFACE

        INTERFACE quuz EXTENDS quux
            METHOD grault
            END_METHOD

            METHOD garply
            END_METHOD
        END_INTERFACE

        INTERFACE quxat
            METHOD waldo
            END_METHOD
        END_INTERFACE

        INTERFACE quxar EXTENDS quuz, baz, quxat
            METHOD fred
            END_METHOD
        END_INTERFACE
    "#,
        id_provider.clone(),
    );

    let (annotations, _, units) = annotate_and_lower_with_ids(unit, index, id_provider);

    let intf = &units.0.interfaces.last().unwrap();
    assert_debug_snapshot!(annotations.get_with_id(intf.id), @r#"
    Some(
        MethodDeclarations {
            declarations: {
                "fred": [
                    Abstract(
                        "quxar.fred",
                    ),
                ],
                "garply": [
                    Abstract(
                        "quuz.garply",
                    ),
                ],
                "corge": [
                    Abstract(
                        "quux.corge",
                    ),
                ],
                "waldo": [
                    Abstract(
                        "quxat.waldo",
                    ),
                ],
                "grault": [
                    Abstract(
                        "quuz.grault",
                    ),
                ],
                "bar": [
                    Abstract(
                        "foo.bar",
                    ),
                ],
                "qux": [
                    Abstract(
                        "baz.qux",
                    ),
                ],
            },
        },
    )
    "#);
}

#[test]
fn extended_interface_has_overridden_method_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index, _) = index_and_lower(
        r#"
        INTERFACE foo
            METHOD bar
            END_METHOD
        END_INTERFACE

        INTERFACE baz EXTENDS foo
            METHOD bar
            END_METHOD
            METHOD qux
            END_METHOD
        END_INTERFACE

        INTERFACE quux
            METHOD qux
            END_METHOD
            METHOD corge
            END_METHOD
        END_INTERFACE

        INTERFACE quuz EXTENDS quux
            METHOD grault
            END_METHOD
            METHOD corge
            END_METHOD
        END_INTERFACE

        INTERFACE grauply EXTENDS quuz
            METHOD qux 
            END_METHOD
            METHOD corge 
            END_METHOD
            METHOD grault
            END_METHOD
        END_INTERFACE
    "#,
        id_provider.clone(),
    );

    let (annotations, _, units) = annotate_and_lower_with_ids(unit, index, id_provider);
    let intf_baz = &units.0.interfaces[1];
    let method = &intf_baz.methods[0];
    // for baz we only expect one override, `bar`
    assert_debug_snapshot!(annotations.get_with_id(method.id), @r#"
    Some(
        Override {
            definitions: [
                Abstract(
                    "foo.bar",
                ),
            ],
        },
    )
    "#);
    // for grauply, we expect all 3 methods to be overrides at different inheritance levels.
    let intf_grauply = &units.0.interfaces[4];
    let method = &intf_grauply.methods[0];
    assert_debug_snapshot!(annotations.get_with_id(method.id), @r#"
    Some(
        Override {
            definitions: [
                Abstract(
                    "quux.qux",
                ),
            ],
        },
    )
    "#);
    // The method `corge` is defined in each interface in the inheritance chain, so we expect 2 overrides
    let method = &intf_grauply.methods[1];
    assert_debug_snapshot!(annotations.get_with_id(method.id), @r#"
    Some(
        Override {
            definitions: [
                Abstract(
                    "quuz.corge",
                ),
                Abstract(
                    "quux.corge",
                ),
            ],
        },
    )
    "#);
    let method = &intf_grauply.methods[2];
    assert_debug_snapshot!(annotations.get_with_id(method.id), @r#"
    Some(
        Override {
            definitions: [
                Abstract(
                    "quuz.grault",
                ),
            ],
        },
    )
    "#);
}

#[test]
fn function_block_has_both_abstract_and_concrete_annotation_from_extended_intf() {
    let id_provider = IdProvider::default();
    let (unit, index, _) = index_and_lower(
        r#"
        INTERFACE foo
            METHOD bar
            END_METHOD
        END_INTERFACE

        INTERFACE baz EXTENDS foo
            METHOD qux
            END_METHOD
        END_INTERFACE

        INTERFACE quux
            METHOD corge
            END_METHOD
        END_INTERFACE

        INTERFACE quuz EXTENDS quux
            METHOD grault
            END_METHOD

            METHOD garply
            END_METHOD
        END_INTERFACE

        INTERFACE quxat
            METHOD waldo
            END_METHOD
        END_INTERFACE

        INTERFACE quxar EXTENDS quuz, baz, quxat
            METHOD fred
            END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK fb IMPLEMENTS quxar
            METHOD bar
            END_METHOD
            METHOD qux
            END_METHOD
            METHOD corge
            END_METHOD
            METHOD grault
            END_METHOD
            METHOD garply
            END_METHOD
            METHOD waldo
            END_METHOD
            METHOD fred
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
        id_provider.clone(),
    );

    let (annotations, _, units) = annotate_and_lower_with_ids(unit, index, id_provider);
    let units = &units.0.pous;
    let fb = &units[0];
    assert_debug_snapshot!(annotations.get_with_id(fb.id), @r#"
    Some(
        MethodDeclarations {
            declarations: {
                "corge": [
                    Concrete(
                        "fb.corge",
                    ),
                    Abstract(
                        "quux.corge",
                    ),
                ],
                "garply": [
                    Concrete(
                        "fb.garply",
                    ),
                    Abstract(
                        "quuz.garply",
                    ),
                ],
                "waldo": [
                    Concrete(
                        "fb.waldo",
                    ),
                    Abstract(
                        "quxat.waldo",
                    ),
                ],
                "fred": [
                    Concrete(
                        "fb.fred",
                    ),
                    Abstract(
                        "quxar.fred",
                    ),
                ],
                "grault": [
                    Concrete(
                        "fb.grault",
                    ),
                    Abstract(
                        "quuz.grault",
                    ),
                ],
                "bar": [
                    Concrete(
                        "fb.bar",
                    ),
                    Abstract(
                        "foo.bar",
                    ),
                ],
                "qux": [
                    Concrete(
                        "fb.qux",
                    ),
                    Abstract(
                        "baz.qux",
                    ),
                ],
            },
        },
    )
    "#);
}

#[test]
fn function_block_methods_have_overrides_from_extended_interface_annotated() {
    let id_provider = IdProvider::default();
    let (unit, index, _) = index_and_lower(
        r#"
        INTERFACE foo
            METHOD bar
            END_METHOD
        END_INTERFACE

        INTERFACE baz EXTENDS foo
            METHOD qux
            END_METHOD
        END_INTERFACE

        INTERFACE quux
            METHOD corge
            END_METHOD
        END_INTERFACE

        INTERFACE quuz EXTENDS quux
            METHOD grault
            END_METHOD

            METHOD garply
            END_METHOD
        END_INTERFACE

        INTERFACE quxat
            METHOD waldo
            END_METHOD
        END_INTERFACE

        INTERFACE quxar EXTENDS quuz, baz, quxat
            METHOD fred
            END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK fb IMPLEMENTS quxar
            METHOD bar
            END_METHOD
            METHOD qux
            END_METHOD
            METHOD corge
            END_METHOD
            METHOD grault
            END_METHOD
            METHOD garply
            END_METHOD
            METHOD waldo
            END_METHOD
            METHOD fred
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
        id_provider.clone(),
    );

    let (annotations, _, unit) = annotate_and_lower_with_ids(unit, index, id_provider);
    let units = &unit.0.pous;
    let method = &units[1];
    assert_debug_snapshot!(annotations.get_with_id(method.id), @r#"
    Some(
        Override {
            definitions: [
                Abstract(
                    "foo.bar",
                ),
            ],
        },
    )
    "#);
    let method = &units[2];
    assert_debug_snapshot!(annotations.get_with_id(method.id), @r#"
    Some(
        Override {
            definitions: [
                Abstract(
                    "baz.qux",
                ),
            ],
        },
    )
    "#);
    let method = &units[3];
    assert_debug_snapshot!(annotations.get_with_id(method.id), @r#"
    Some(
        Override {
            definitions: [
                Abstract(
                    "quux.corge",
                ),
            ],
        },
    )
    "#);
    let method = &units[4];
    assert_debug_snapshot!(annotations.get_with_id(method.id), @r#"
    Some(
        Override {
            definitions: [
                Abstract(
                    "quuz.grault",
                ),
            ],
        },
    )
    "#);
    let method = &units[5];
    assert_debug_snapshot!(annotations.get_with_id(method.id), @r#"
    Some(
        Override {
            definitions: [
                Abstract(
                    "quuz.garply",
                ),
            ],
        },
    )
    "#);
    let method = &units[6];
    assert_debug_snapshot!(annotations.get_with_id(method.id), @r#"
    Some(
        Override {
            definitions: [
                Abstract(
                    "quxat.waldo",
                ),
            ],
        },
    )
    "#);
    let method = &units[7];
    assert_debug_snapshot!(annotations.get_with_id(method.id), @r#"
    Some(
        Override {
            definitions: [
                Abstract(
                    "quxar.fred",
                ),
            ],
        },
    )
    "#);
}
