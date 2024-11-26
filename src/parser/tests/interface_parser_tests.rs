use crate::test_utils::tests::parse;

#[test]
fn empty_interface() {
    let source = r"
    INTERFACE myInterface
    END_INTERFACE
    ";

    let (unit, diagnostics) = parse(source);

    assert_eq!(diagnostics.len(), 0, "Expected no diagnostics but got {:#?}", diagnostics);
    insta::assert_debug_snapshot!(unit.interfaces, @r###"
    [
        Interface {
            name: "myInterface",
            methods: [],
            location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 1,
                        column: 4,
                        offset: 5,
                    }..TextLocation {
                        line: 3,
                        column: 4,
                        offset: 49,
                    },
                ),
            },
            location_name: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 1,
                        column: 14,
                        offset: 15,
                    }..TextLocation {
                        line: 1,
                        column: 25,
                        offset: 26,
                    },
                ),
            },
        },
    ]
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

    let (unit, diagnostics) = parse(source);

    assert_eq!(diagnostics.len(), 0, "Expected no diagnostics but got {:#?}", diagnostics);
    insta::assert_debug_snapshot!(unit.interfaces, @r###"
    [
        Interface {
            name: "myInterface",
            methods: [
                POU {
                    name: "myInterface.foo",
                    variable_blocks: [
                        VariableBlock {
                            variables: [
                                Variable {
                                    name: "a",
                                    data_type: DataTypeReference {
                                        referenced_type: "INT",
                                    },
                                },
                                Variable {
                                    name: "b",
                                    data_type: DataTypeReference {
                                        referenced_type: "INT",
                                    },
                                },
                            ],
                            variable_block_type: Input(
                                ByVal,
                            ),
                        },
                    ],
                    pou_type: Method {
                        parent: "myInterface",
                    },
                    return_type: Some(
                        DataTypeReference {
                            referenced_type: "INT",
                        },
                    ),
                    interfaces: [],
                },
            ],
            location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 1,
                        column: 4,
                        offset: 5,
                    }..TextLocation {
                        line: 9,
                        column: 4,
                        offset: 185,
                    },
                ),
            },
            location_name: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 1,
                        column: 14,
                        offset: 15,
                    }..TextLocation {
                        line: 1,
                        column: 25,
                        offset: 26,
                    },
                ),
            },
        },
    ]
    "###);
}

#[test]
fn interface_with_multiple_methods() {
    let source = r"
    INTERFACE myInterface
        METHOD foo : INT
            VAR_INPUT
                a : INT;
                b : INT;
            END_VAR
        END_METHOD

        METHOD bar : INT
            VAR_INPUT
                c : INT;
            END_VAR

            VAR_IN_OUT
                d : INT;
            END_VAR
        END_METHOD
    END_INTERFACE
    ";

    let (unit, diagnostics) = parse(source);

    assert_eq!(diagnostics.len(), 0, "Expected no diagnostics but got {:#?}", diagnostics);
    insta::assert_debug_snapshot!(unit.interfaces, @r###"
    [
        Interface {
            name: "myInterface",
            methods: [
                POU {
                    name: "myInterface.foo",
                    variable_blocks: [
                        VariableBlock {
                            variables: [
                                Variable {
                                    name: "a",
                                    data_type: DataTypeReference {
                                        referenced_type: "INT",
                                    },
                                },
                                Variable {
                                    name: "b",
                                    data_type: DataTypeReference {
                                        referenced_type: "INT",
                                    },
                                },
                            ],
                            variable_block_type: Input(
                                ByVal,
                            ),
                        },
                    ],
                    pou_type: Method {
                        parent: "myInterface",
                    },
                    return_type: Some(
                        DataTypeReference {
                            referenced_type: "INT",
                        },
                    ),
                    interfaces: [],
                },
                POU {
                    name: "myInterface.bar",
                    variable_blocks: [
                        VariableBlock {
                            variables: [
                                Variable {
                                    name: "c",
                                    data_type: DataTypeReference {
                                        referenced_type: "INT",
                                    },
                                },
                            ],
                            variable_block_type: Input(
                                ByVal,
                            ),
                        },
                        VariableBlock {
                            variables: [
                                Variable {
                                    name: "d",
                                    data_type: DataTypeReference {
                                        referenced_type: "INT",
                                    },
                                },
                            ],
                            variable_block_type: InOut,
                        },
                    ],
                    pou_type: Method {
                        parent: "myInterface",
                    },
                    return_type: Some(
                        DataTypeReference {
                            referenced_type: "INT",
                        },
                    ),
                    interfaces: [],
                },
            ],
            location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 1,
                        column: 4,
                        offset: 5,
                    }..TextLocation {
                        line: 19,
                        column: 4,
                        offset: 366,
                    },
                ),
            },
            location_name: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 1,
                        column: 14,
                        offset: 15,
                    }..TextLocation {
                        line: 1,
                        column: 25,
                        offset: 26,
                    },
                ),
            },
        },
    ]
    "###);
}

#[test]
fn pou_implementing_single_interface() {
    let source = r#"
    FUNCTION_BLOCK foo IMPLEMENTS myInterface END_FUNCTION_BLOCK
    "#;

    let (unit, diagnostics) = parse(source);

    assert_eq!(diagnostics.len(), 0, "Expected no diagnostics but got {:#?}", diagnostics);
    insta::assert_debug_snapshot!(unit.units[0], @r###"
    POU {
        name: "foo",
        variable_blocks: [],
        pou_type: FunctionBlock,
        return_type: None,
        interfaces: [
            InterfaceDeclaration {
                name: "myInterface",
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 1,
                            column: 34,
                            offset: 35,
                        }..TextLocation {
                            line: 1,
                            column: 45,
                            offset: 46,
                        },
                    ),
                },
            },
        ],
    }
    "###);
}

#[test]
fn pou_implementing_multiple_interfaces() {
    let source = r#"
    FUNCTION_BLOCK foo IMPLEMENTS InterfaceA, InterfaceB, InterfaceC END_FUNCTION_BLOCK
    "#;

    let (unit, diagnostics) = parse(source);

    assert_eq!(diagnostics.len(), 0, "Expected no diagnostics but got {:#?}", diagnostics);
    insta::assert_debug_snapshot!(unit.units[0], @r###"
    POU {
        name: "foo",
        variable_blocks: [],
        pou_type: FunctionBlock,
        return_type: None,
        interfaces: [
            InterfaceDeclaration {
                name: "InterfaceA",
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 1,
                            column: 34,
                            offset: 35,
                        }..TextLocation {
                            line: 1,
                            column: 44,
                            offset: 45,
                        },
                    ),
                },
            },
            InterfaceDeclaration {
                name: "InterfaceB",
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 1,
                            column: 46,
                            offset: 47,
                        }..TextLocation {
                            line: 1,
                            column: 56,
                            offset: 57,
                        },
                    ),
                },
            },
            InterfaceDeclaration {
                name: "InterfaceC",
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 1,
                            column: 58,
                            offset: 59,
                        }..TextLocation {
                            line: 1,
                            column: 68,
                            offset: 69,
                        },
                    ),
                },
            },
        ],
    }
    "###);
}
