use crate::test_utils::tests::parse;

#[test]
fn empty_interface() {
    let source = r"
    INTERFACE myInterface
    END_INTERFACE
    ";

    let (unit, diagnostics) = parse(source);

    assert_eq!(diagnostics.len(), 0, "Expected no diagnostics but got {:#?}", diagnostics);
    insta::assert_debug_snapshot!(unit.interfaces, @r#"
    [
        Interface {
            id: 1,
            ident: Identifier {
                name: "myInterface",
                location: SourceLocation {
                    span: Range(1:14 - 1:25),
                },
            },
            location: SourceLocation {
                span: Range(1:4 - 3:4),
            },
            methods: [],
            extensions: [],
            properties: [],
        },
    ]
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

    let (unit, diagnostics) = parse(source);

    assert_eq!(diagnostics.len(), 0, "Expected no diagnostics but got {:#?}", diagnostics);
    insta::assert_debug_snapshot!(unit.interfaces, @r#"
    [
        Interface {
            id: 2,
            ident: Identifier {
                name: "myInterface",
                location: SourceLocation {
                    span: Range(1:14 - 1:25),
                },
            },
            location: SourceLocation {
                span: Range(1:4 - 9:4),
            },
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
                        property: None,
                        declaration_kind: Abstract,
                    },
                    return_type: Some(
                        DataTypeReference {
                            referenced_type: "INT",
                        },
                    ),
                    interfaces: [],
                    properties: [],
                },
            ],
            extensions: [],
            properties: [],
        },
    ]
    "#);
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
    insta::assert_debug_snapshot!(unit.interfaces, @r#"
    [
        Interface {
            id: 3,
            ident: Identifier {
                name: "myInterface",
                location: SourceLocation {
                    span: Range(1:14 - 1:25),
                },
            },
            location: SourceLocation {
                span: Range(1:4 - 19:4),
            },
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
                        property: None,
                        declaration_kind: Abstract,
                    },
                    return_type: Some(
                        DataTypeReference {
                            referenced_type: "INT",
                        },
                    ),
                    interfaces: [],
                    properties: [],
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
                        property: None,
                        declaration_kind: Abstract,
                    },
                    return_type: Some(
                        DataTypeReference {
                            referenced_type: "INT",
                        },
                    ),
                    interfaces: [],
                    properties: [],
                },
            ],
            extensions: [],
            properties: [],
        },
    ]
    "#);
}

#[test]
fn pou_implementing_single_interface() {
    let source = r#"
    FUNCTION_BLOCK foo IMPLEMENTS myInterface END_FUNCTION_BLOCK
    "#;

    let (unit, diagnostics) = parse(source);

    assert_eq!(diagnostics.len(), 0, "Expected no diagnostics but got {:#?}", diagnostics);
    insta::assert_debug_snapshot!(unit.pous[0], @r#"
    POU {
        name: "foo",
        variable_blocks: [],
        pou_type: FunctionBlock,
        return_type: None,
        interfaces: [
            Identifier {
                name: "myInterface",
                location: SourceLocation {
                    span: Range(1:34 - 1:45),
                },
            },
        ],
        properties: [],
    }
    "#);
}

#[test]
fn pou_implementing_multiple_interfaces() {
    let source = r#"
    FUNCTION_BLOCK foo IMPLEMENTS InterfaceA, InterfaceB, InterfaceC END_FUNCTION_BLOCK
    "#;

    let (unit, diagnostics) = parse(source);

    assert_eq!(diagnostics.len(), 0, "Expected no diagnostics but got {:#?}", diagnostics);
    insta::assert_debug_snapshot!(unit.pous[0], @r#"
    POU {
        name: "foo",
        variable_blocks: [],
        pou_type: FunctionBlock,
        return_type: None,
        interfaces: [
            Identifier {
                name: "InterfaceA",
                location: SourceLocation {
                    span: Range(1:34 - 1:44),
                },
            },
            Identifier {
                name: "InterfaceB",
                location: SourceLocation {
                    span: Range(1:46 - 1:56),
                },
            },
            Identifier {
                name: "InterfaceC",
                location: SourceLocation {
                    span: Range(1:58 - 1:68),
                },
            },
        ],
        properties: [],
    }
    "#);
}

#[test]
fn interface_deriving_from_other_interface() {
    let source = r#"
        INTERFACE foo
        METHOD baz
        END_METHOD
        END_INTERFACE

        INTERFACE bar EXTENDS foo
        METHOD qux
        END_METHOD
        END_INTERFACE
    "#;

    let (unit, diagnostics) = parse(source);

    assert_eq!(diagnostics.len(), 0, "Expected no diagnostics but got {:#?}", diagnostics);
    insta::assert_debug_snapshot!(unit.interfaces[1], @r#"
    Interface {
        id: 4,
        ident: Identifier {
            name: "bar",
            location: SourceLocation {
                span: Range(6:18 - 6:21),
            },
        },
        location: SourceLocation {
            span: Range(6:8 - 10:4),
        },
        methods: [
            POU {
                name: "bar.qux",
                variable_blocks: [],
                pou_type: Method {
                    parent: "bar",
                    property: None,
                    declaration_kind: Abstract,
                },
                return_type: None,
                interfaces: [],
                properties: [],
            },
        ],
        extensions: [
            Identifier {
                name: "foo",
                location: SourceLocation {
                    span: Range(6:30 - 6:33),
                },
            },
        ],
        properties: [],
    }
    "#);
}

#[test]
fn interface_deriving_from_multiple_interfaces() {
    let source = r#"
    INTERFACE foo
    METHOD baz
    END_METHOD
    END_INTERFACE

    INTERFACE bar
    METHOD qux
    END_METHOD
    END_INTERFACE

    INTERFACE quux EXTENDS foo, bar
    END_INTERFACE
    "#;

    let (unit, diagnostics) = parse(source);

    assert_eq!(diagnostics.len(), 0, "Expected no diagnostics but got {:#?}", diagnostics);
    insta::assert_debug_snapshot!(unit.interfaces[2], @r#"
    Interface {
        id: 5,
        ident: Identifier {
            name: "quux",
            location: SourceLocation {
                span: Range(11:14 - 11:18),
            },
        },
        location: SourceLocation {
            span: Range(11:4 - 13:4),
        },
        methods: [],
        extensions: [
            Identifier {
                name: "foo",
                location: SourceLocation {
                    span: Range(11:27 - 11:30),
                },
            },
            Identifier {
                name: "bar",
                location: SourceLocation {
                    span: Range(11:32 - 11:35),
                },
            },
        ],
        properties: [],
    }
    "#);
}

#[test]
fn interface_with_property() {
    let source = r"
    INTERFACE myInterface
        PROPERTY foo : INT
            GET END_GET
            SET END_SET
        END_PROPERTY
    END_INTERFACE
    ";

    let (unit, diagnostics) = parse(source);

    assert_eq!(diagnostics.len(), 0, "Expected no diagnostics but got {:#?}", diagnostics);

    assert_eq!(unit.interfaces.len(), 1);
    assert_eq!(unit.interfaces[0].ident.name, "myInterface");

    insta::assert_debug_snapshot!(unit.interfaces[0].properties, @r#"
    [
        PropertyBlock {
            ident: Identifier {
                name: "foo",
                location: SourceLocation {
                    span: Range(2:17 - 2:20),
                },
            },
            datatype: DataTypeReference {
                referenced_type: "INT",
            },
            implementations: [
                PropertyImplementation {
                    kind: Get,
                    location: SourceLocation {
                        span: Range(3:12 - 3:15),
                    },
                    variable_blocks: [],
                    body: [],
                    end_location: SourceLocation {
                        span: Range(3:16 - 3:23),
                    },
                },
                PropertyImplementation {
                    kind: Set,
                    location: SourceLocation {
                        span: Range(4:12 - 4:15),
                    },
                    variable_blocks: [],
                    body: [],
                    end_location: SourceLocation {
                        span: Range(4:16 - 4:23),
                    },
                },
            ],
        },
    ]
    "#);
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

            PROPERTY propA : INT
                GET
                    1 > 2;
                END_GET

                SET
                    1 > 2;
                END_SET
            END_PROPERTY
        END_INTERFACE
        ";

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E113]: Interfaces can not have a default implementation
          ┌─ <internal>:4:17
          │
        4 │                 1 > 2;
          │                 ^^^^^ Interfaces can not have a default implementation

        error[E113]: Interfaces can not have a default implementation
           ┌─ <internal>:10:21
           │
        10 │                     1 > 2;
           │                     ^^^^^ Interfaces can not have a default implementation

        error[E113]: Interfaces can not have a default implementation
           ┌─ <internal>:14:21
           │
        14 │                     1 > 2;
           │                     ^^^^^ Interfaces can not have a default implementation
        ");
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
        insta::assert_snapshot!(diagnostics, @r"
        error[E006]: Expected a name for the interface definition but got nothing
          ┌─ <internal>:2:9
          │
        2 │         INTERFACE
          │         ^^^^^^^^^ Expected a name for the interface definition but got nothing
        ");
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
        insta::assert_snapshot!(diagnostics, @r"
        error[E006]: Expected a comma separated list of identifiers after `IMPLEMENTS` but got nothing
          ┌─ <internal>:2:28
          │
        2 │         FUNCTION_BLOCK foo IMPLEMENTS
          │                            ^^^^^^^^^^ Expected a comma separated list of identifiers after `IMPLEMENTS` but got nothing
        ");
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
