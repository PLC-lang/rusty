use crate::test_utils::tests::index;

#[test]
fn empty_interface() {
    let source = r"
    INTERFACE myInterface
    END_INTERFACE
    ";

    let (_, index) = index(source);

    insta::assert_debug_snapshot!(index.find_interface("myInterface").unwrap(), @r###"
    InterfaceIndexEntry {
        name: "myInterface",
        methods: [],
        extensions: [],
    }
    "###);
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

    insta::assert_debug_snapshot!(index.find_interface("myInterface").unwrap(), @r###"
    InterfaceIndexEntry {
        name: "myInterface",
        methods: [
            "myInterface.foo",
        ],
        extensions: [],
    }
    "###);

    insta::assert_debug_snapshot!(index.find_pou("myInterface.foo").unwrap(), @r###"
    Method {
        name: "myInterface.foo",
        parent_name: "myInterface",
        property: None,
        declaration_kind: Abstract,
        return_type: "INT",
        instance_struct_name: "myInterface.foo",
        linkage: Internal,
        location: SourceLocation {
            span: Range(
                TextLocation {
                    line: 2,
                    column: 15,
                    offset: 42,
                }..TextLocation {
                    line: 2,
                    column: 18,
                    offset: 45,
                },
            ),
            file: Some(
                "<internal>",
            ),
        },
    }
    "###);

    insta::assert_debug_snapshot!(index.get_pou_members("myInterface.foo"), @r###"
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
                span: Range(
                    TextLocation {
                        line: 4,
                        column: 16,
                        offset: 90,
                    }..TextLocation {
                        line: 4,
                        column: 17,
                        offset: 91,
                    },
                ),
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
                span: Range(
                    TextLocation {
                        line: 5,
                        column: 16,
                        offset: 115,
                    }..TextLocation {
                        line: 5,
                        column: 17,
                        offset: 116,
                    },
                ),
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
                span: Range(
                    TextLocation {
                        line: 2,
                        column: 15,
                        offset: 42,
                    }..TextLocation {
                        line: 2,
                        column: 18,
                        offset: 45,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
            varargs: None,
        },
    ]
    "###);
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

    insta::assert_debug_snapshot!(entry.get_methods(&index), @r###"
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
                span: Range(
                    TextLocation {
                        line: 2,
                        column: 15,
                        offset: 42,
                    }..TextLocation {
                        line: 2,
                        column: 18,
                        offset: 45,
                    },
                ),
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
                span: Range(
                    TextLocation {
                        line: 8,
                        column: 15,
                        offset: 156,
                    }..TextLocation {
                        line: 8,
                        column: 18,
                        offset: 159,
                    },
                ),
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
                span: Range(
                    TextLocation {
                        line: 14,
                        column: 15,
                        offset: 268,
                    }..TextLocation {
                        line: 14,
                        column: 18,
                        offset: 271,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
        },
    ]
    "###);
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
    insta::assert_debug_snapshot!(entry.get_derived_interfaces(&index), @r###"
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
    "###);

    let entry = index.find_interface("qux").unwrap();
    insta::assert_debug_snapshot!(entry.get_derived_interfaces(&index), @r###"
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
    "###);
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
    insta::assert_debug_snapshot!(entry.get_derived_interfaces(&index), @r###"
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
    "###);

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
                            span: Range(
                                TextLocation {
                                    line: 6,
                                    column: 30,
                                    offset: 116,
                                }..TextLocation {
                                    line: 6,
                                    column: 33,
                                    offset: 119,
                                },
                            ),
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
                            span: Range(
                                TextLocation {
                                    line: 11,
                                    column: 30,
                                    offset: 213,
                                }..TextLocation {
                                    line: 11,
                                    column: 33,
                                    offset: 216,
                                },
                            ),
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
                    span: Range(
                        TextLocation {
                            line: 1,
                            column: 30,
                            offset: 31,
                        }..TextLocation {
                            line: 1,
                            column: 33,
                            offset: 34,
                        },
                    ),
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
                            span: Range(
                                TextLocation {
                                    line: 1,
                                    column: 30,
                                    offset: 31,
                                }..TextLocation {
                                    line: 1,
                                    column: 33,
                                    offset: 34,
                                },
                            ),
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
                    span: Range(
                        TextLocation {
                            line: 4,
                            column: 35,
                            offset: 93,
                        }..TextLocation {
                            line: 4,
                            column: 38,
                            offset: 96,
                        },
                    ),
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
    insta::assert_debug_snapshot!(entry.get_methods(&index), @r###"
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
                span: Range(
                    TextLocation {
                        line: 7,
                        column: 15,
                        offset: 145,
                    }..TextLocation {
                        line: 7,
                        column: 18,
                        offset: 148,
                    },
                ),
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
                span: Range(
                    TextLocation {
                        line: 2,
                        column: 15,
                        offset: 50,
                    }..TextLocation {
                        line: 2,
                        column: 18,
                        offset: 53,
                    },
                ),
                file: Some(
                    "<internal>",
                ),
            },
        },
    ]
    "###);
}
