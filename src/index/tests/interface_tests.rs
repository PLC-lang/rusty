use itertools::Itertools;

use crate::test_utils::tests::index;

#[test]
fn empty_interface() {
    let source = r"
    INTERFACE myInterface
    END_INTERFACE
    ";

    let (_, index) = index(source);

    insta::assert_debug_snapshot!(index.find_interface("myInterface").unwrap(), @r#"
    InterfaceIndexEntry {
        name: "myInterface",
        methods: [],
        extensions: [],
    }
    "#);
}

#[test]
fn interface_with_single_method() {
    let source = r"
    INTERFACE myInterface
        METHOD foo : INT
            VAR_INPUT
                a : INT;
                b : INT;
            END_VAR
        END_METHOD
    END_INTERFACE
    ";

    let (_, index) = index(source);

    insta::assert_debug_snapshot!(index.find_interface("myInterface").unwrap(), @r#"
    InterfaceIndexEntry {
        name: "myInterface",
        methods: [
            "myInterface.foo",
        ],
        extensions: [],
    }
    "#);

    insta::assert_debug_snapshot!(index.find_pou("myInterface.foo").unwrap(), @r#"
    Method {
        name: "myInterface.foo",
        parent_name: "myInterface",
        property: None,
        declaration_kind: Abstract,
        return_type: "INT",
        instance_struct_name: "myInterface.foo",
        linkage: Internal,
        location: SourceLocation {
            span: Range(2:15 - 2:18),
            file: Some(
                "<internal>",
            ),
        },
    }
    "#);

    insta::assert_debug_snapshot!(index.get_pou_members("myInterface.foo"), @r#"
    [
        VariableIndexEntry {
            name: "a",
            qualified_name: "myInterface.foo.a",
            initial_value: None,
            argument_type: ByVal(
                Input,
            ),
            is_constant: false,
            is_var_external: false,
            data_type_name: "INT",
            location_in_parent: 0,
            linkage: Internal,
            binding: None,
            source_location: SourceLocation {
                span: Range(4:16 - 4:17),
                file: Some(
                    "<internal>",
                ),
            },
            varargs: None,
        },
        VariableIndexEntry {
            name: "b",
            qualified_name: "myInterface.foo.b",
            initial_value: None,
            argument_type: ByVal(
                Input,
            ),
            is_constant: false,
            is_var_external: false,
            data_type_name: "INT",
            location_in_parent: 1,
            linkage: Internal,
            binding: None,
            source_location: SourceLocation {
                span: Range(5:16 - 5:17),
                file: Some(
                    "<internal>",
                ),
            },
            varargs: None,
        },
        VariableIndexEntry {
            name: "foo",
            qualified_name: "myInterface.foo.foo",
            initial_value: None,
            argument_type: ByVal(
                Return,
            ),
            is_constant: false,
            is_var_external: false,
            data_type_name: "INT",
            location_in_parent: 2,
            linkage: Internal,
            binding: None,
            source_location: SourceLocation {
                span: Range(2:15 - 2:18),
                file: Some(
                    "<internal>",
                ),
            },
            varargs: None,
        },
    ]
    "#);
}

#[test]
fn get_interface_methods() {
    let source = r"
    INTERFACE myInterface
        METHOD foo : SINT
            VAR_INPUT
                a : SINT;
            END_VAR
        END_METHOD

        METHOD bar : INT
            VAR_INPUT
                b : INT;
            END_VAR
        END_METHOD

        METHOD baz : DINT
            VAR_INPUT
                c : DINT;
            END_VAR
        END_METHOD
    ";

    let (_, index) = index(source);
    let entry = index.find_interface("myInterface").unwrap();

    insta::assert_debug_snapshot!(entry.get_methods(&index), @r#"
    [
        Method {
            name: "myInterface.foo",
            parent_name: "myInterface",
            property: None,
            declaration_kind: Abstract,
            return_type: "SINT",
            instance_struct_name: "myInterface.foo",
            linkage: Internal,
            location: SourceLocation {
                span: Range(2:15 - 2:18),
                file: Some(
                    "<internal>",
                ),
            },
        },
        Method {
            name: "myInterface.bar",
            parent_name: "myInterface",
            property: None,
            declaration_kind: Abstract,
            return_type: "INT",
            instance_struct_name: "myInterface.bar",
            linkage: Internal,
            location: SourceLocation {
                span: Range(8:15 - 8:18),
                file: Some(
                    "<internal>",
                ),
            },
        },
        Method {
            name: "myInterface.baz",
            parent_name: "myInterface",
            property: None,
            declaration_kind: Abstract,
            return_type: "DINT",
            instance_struct_name: "myInterface.baz",
            linkage: Internal,
            location: SourceLocation {
                span: Range(14:15 - 14:18),
                file: Some(
                    "<internal>",
                ),
            },
        },
    ]
    "#);
}

#[test]
fn extended_interfaces() {
    let source = r"
        INTERFACE foo
        METHOD m_foo
        END_METHOD
        END_INTERFACE

        INTERFACE bar EXTENDS foo
        END_INTERFACE

        INTERFACE baz
        METHOD m_baz
        END_METHOD
        END_INTERFACE

        INTERFACE qux EXTENDS foo, baz
        END_INTERFACE
    ";

    let (_, index) = index(source);
    let entry = index.find_interface("bar").unwrap();
    insta::assert_debug_snapshot!(entry.get_derived_interfaces(&index), @r#"
    [
        Ok(
            InterfaceIndexEntry {
                name: "foo",
                methods: [
                    "foo.m_foo",
                ],
                extensions: [],
            },
        ),
    ]
    "#);

    let entry = index.find_interface("qux").unwrap();
    insta::assert_debug_snapshot!(entry.get_derived_interfaces(&index), @r#"
    [
        Ok(
            InterfaceIndexEntry {
                name: "foo",
                methods: [
                    "foo.m_foo",
                ],
                extensions: [],
            },
        ),
        Ok(
            InterfaceIndexEntry {
                name: "baz",
                methods: [
                    "baz.m_baz",
                ],
                extensions: [],
            },
        ),
    ]
    "#);
}

#[test]
fn nested_extended_interfaces() {
    let source = r"
        INTERFACE foo
        METHOD m_foo
        END_METHOD
        END_INTERFACE

        INTERFACE bar EXTENDS foo
        METHOD m_bar
        END_METHOD
        END_INTERFACE

        INTERFACE baz EXTENDS bar
        METHOD m_baz
        END_METHOD
        END_INTERFACE

        INTERFACE qux EXTENDS baz
        END_INTERFACE
    ";

    let (_, index) = index(source);
    let entry = index.find_interface("bar").unwrap();
    insta::assert_debug_snapshot!(entry.get_derived_interfaces(&index), @r#"
    [
        Ok(
            InterfaceIndexEntry {
                name: "foo",
                methods: [
                    "foo.m_foo",
                ],
                extensions: [],
            },
        ),
    ]
    "#);

    let entry = index.find_interface("baz").unwrap();
    insta::assert_debug_snapshot!(entry.get_derived_interfaces(&index), @r#"
    [
        Ok(
            InterfaceIndexEntry {
                name: "bar",
                methods: [
                    "bar.m_bar",
                ],
                extensions: [
                    Identifier {
                        name: "foo",
                        location: SourceLocation {
                            span: Range(6:30 - 6:33),
                            file: Some(
                                "<internal>",
                            ),
                        },
                    },
                ],
            },
        ),
    ]
    "#);

    let entry = index.find_interface("qux").unwrap();
    insta::assert_debug_snapshot!(entry.get_derived_interfaces(&index), @r#"
    [
        Ok(
            InterfaceIndexEntry {
                name: "baz",
                methods: [
                    "baz.m_baz",
                ],
                extensions: [
                    Identifier {
                        name: "bar",
                        location: SourceLocation {
                            span: Range(11:30 - 11:33),
                            file: Some(
                                "<internal>",
                            ),
                        },
                    },
                ],
            },
        ),
    ]
    "#);
}

#[test]
fn deriving_from_undeclared_interface() {
    let source = r"
        INTERFACE foo EXTENDS bar
        END_INTERFACE

        INTERFACE baz EXTENDS foo, bar
        END_INTERFACE
    ";

    let (_, index) = index(source);
    let entry = index.find_interface("foo").unwrap();
    insta::assert_debug_snapshot!(entry.get_derived_interfaces(&index), @r#"
    [
        Err(
            Identifier {
                name: "bar",
                location: SourceLocation {
                    span: Range(1:30 - 1:33),
                    file: Some(
                        "<internal>",
                    ),
                },
            },
        ),
    ]
    "#);

    let entry = index.find_interface("baz").unwrap();
    insta::assert_debug_snapshot!(entry.get_derived_interfaces(&index), @r#"
    [
        Ok(
            InterfaceIndexEntry {
                name: "foo",
                methods: [],
                extensions: [
                    Identifier {
                        name: "bar",
                        location: SourceLocation {
                            span: Range(1:30 - 1:33),
                            file: Some(
                                "<internal>",
                            ),
                        },
                    },
                ],
            },
        ),
        Err(
            Identifier {
                name: "bar",
                location: SourceLocation {
                    span: Range(4:35 - 4:38),
                    file: Some(
                        "<internal>",
                    ),
                },
            },
        ),
    ]
    "#);
}

#[test]
fn recursive_interfaces_do_not_overflow_the_stack_when_getting_all_methods() {
    let source = r"
        INTERFACE foo EXTENDS bar
        METHOD bar
        END_METHOD
        END_INTERFACE

        INTERFACE bar EXTENDS foo
        METHOD foo
        END_METHOD
        END_INTERFACE
    ";

    let (_, index) = index(source);

    let entry = index.find_interface("bar").unwrap();
    insta::assert_debug_snapshot!(entry.get_methods(&index), @r#"
    [
        Method {
            name: "bar.foo",
            parent_name: "bar",
            property: None,
            declaration_kind: Abstract,
            return_type: "VOID",
            instance_struct_name: "bar.foo",
            linkage: Internal,
            location: SourceLocation {
                span: Range(7:15 - 7:18),
                file: Some(
                    "<internal>",
                ),
            },
        },
        Method {
            name: "foo.bar",
            parent_name: "foo",
            property: None,
            declaration_kind: Abstract,
            return_type: "VOID",
            instance_struct_name: "foo.bar",
            linkage: Internal,
            location: SourceLocation {
                span: Range(2:15 - 2:18),
                file: Some(
                    "<internal>",
                ),
            },
        },
    ]
    "#);
}

#[test]
fn find_all_derived_interfaces_directly_or_indirectly() {
    /*
    The relationships are as follows:

    H -> E, F, G -> D -> B -> A -> C
                    ▲              |
                    |              |
                    +--------------+

    Lonely, we, are, so, lonelyy
     */
    let source = r"
    INTERFACE a EXTENDS c
    END_INTERFACE

    INTERfACE b EXTENDS a
    END_INTERFACE
    
    INTERFACE c EXTENDS d
    END_INTERFACE

    INTERFACE d EXTENDS b
    END_INTERFACE

    INTERFACE e EXTENDS d
    END_INTERFACE

    INTERFACE f EXTENDS d
    END_INTERFACE

    INTERFACE g EXTENDS d
    END_INTERFACE

    INTERFACE h EXTENDS e, f, g
    END_INTERFACE

    // These should not be included in the result :(
    INTERFACE lonely     END_INTERFACE
    INTERFACE we         END_INTERFACE
    INTERFACE are        END_INTERFACE
    INTERFACE so         END_INTERFACE
    INTERFACE lonelyy    END_INTERFACE
    ";

    let (_, index) = index(source);
    let entry = index.find_interface("h").unwrap();

    // We expect no failure, even though the relationship is cyclic
    let mut derived =
        entry.get_derived_interfaces_recursive(&index).iter().map(|it| &it.ident.name).collect_vec();

    derived.sort();
    assert_eq!(derived, vec!["a", "b", "c", "d", "e", "f", "g", "h"]);
}
