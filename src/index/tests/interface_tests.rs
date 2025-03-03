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
    }
    "###);

    insta::assert_debug_snapshot!(index.find_pou("myInterface.foo").unwrap(), @r###"
    Method {
        name: "myInterface.foo",
        parent_pou_name: "myInterface",
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
            parent_pou_name: "myInterface",
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
            parent_pou_name: "myInterface",
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
            parent_pou_name: "myInterface",
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
