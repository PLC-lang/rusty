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
            InterfaceIdentifier {
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
            InterfaceIdentifier {
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
            InterfaceIdentifier {
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
            InterfaceIdentifier {
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

mod error_handling {
    use crate::test_utils::tests::{parse, parse_and_validate_buffered};

    #[test]
    fn default_method_impl() {
        let source = r"
        INTERFACE interfaceA
            METHOD methodA : INT
                1 > 2;
                methodA := 5;
            END_METHOD
        END_INTERFACE
        ";

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r###"
        warning[E113]: Interfaces can not have a default implementations
          ┌─ <internal>:4:17
          │  
        4 │ ╭                 1 > 2;
        5 │ │                 methodA := 5;
          │ ╰─────────────────────────────^ Interfaces can not have a default implementations

        "###);
    }

    #[test]
    fn error_recovery_empty_interface_name() {
        let source = r"
        INTERFACE
            METHOD foo
                VAR_INPUT
                    a : DINT;
                END_VAR
            END_METHOD
        END_INTERFACE
        ";

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r###"
        error[E006]: Expected a name for the interface definition but got nothing
          ┌─ <internal>:2:9
          │
        2 │         INTERFACE
          │         ^^^^^^^^^ Expected a name for the interface definition but got nothing

        "###);
    }

    #[test]
    fn error_implements_without_declarations() {
        let source = r"
        FUNCTION_BLOCK foo IMPLEMENTS
            METHOD bar
                VAR_INPUT
                    a : DINT;
                END_VAR
            END_METHOD
        END_FUNCTION_BLOCK
        ";

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r###"
        error[E006]: Expected a comma separated list of identifiers after `IMPLEMENTS` but got nothing
          ┌─ <internal>:2:28
          │
        2 │         FUNCTION_BLOCK foo IMPLEMENTS
          │                            ^^^^^^^^^^ Expected a comma separated list of identifiers after `IMPLEMENTS` but got nothing

        "###);
    }

    #[test]
    fn trailing_comma_in_implements_are_ignored() {
        let source = r"
        INTERFACE a /* ... */ END_INTERFACE
        INTERFACE b /* ... */ END_INTERFACE

        FUNCTION_BLOCK foo IMPLEMENTS a,    /* ... */ END_FUNCTION_BLOCK
        FUNCTION_BLOCK bar IMPLEMENTS a, b, /* ... */ END_FUNCTION_BLOCK
        ";

        let (_, diagnostics) = parse(source);
        assert_eq!(diagnostics.len(), 0, "Expected no diagnostics but got {:#?}", diagnostics);
    }
}
