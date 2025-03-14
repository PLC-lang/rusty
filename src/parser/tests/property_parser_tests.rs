use crate::test_utils::tests::{parse, parse_buffered};

#[test]
fn properties_can_be_parsed() {
    let source = r"
        FUNCTION_BLOCK foo
            PROPERTY bar : INT
                GET
                    VAR
                        getLocalVariable : DINT;
                    END_VAR

                    bar := 5;
                END_GET
                SET
                    VAR
                        setLocalVariable : DINT;
                    END_VAR

                    localNonExistingVariable := bar;
                END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse(source);

    assert_eq!(diagnostics, vec![]);

    assert_eq!(unit.units.len(), 1);
    assert_eq!(unit.units[0].name, "foo");

    let properties = &unit.units[0].properties;
    assert_eq!(properties.len(), 1);
    assert_eq!(properties[0].name.name, "bar");
    assert_eq!(properties[0].implementations.len(), 2);

    insta::assert_debug_snapshot!(properties, @r###"
    [
        PropertyBlock {
            name: Identifier {
                name: "bar",
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 2,
                            column: 21,
                            offset: 49,
                        }..TextLocation {
                            line: 2,
                            column: 24,
                            offset: 52,
                        },
                    ),
                },
            },
            return_type: DataTypeReference {
                referenced_type: "INT",
            },
            implementations: [
                PropertyImplementation {
                    kind: Get,
                    location: SourceLocation {
                        span: Range(
                            TextLocation {
                                line: 3,
                                column: 16,
                                offset: 75,
                            }..TextLocation {
                                line: 3,
                                column: 19,
                                offset: 78,
                            },
                        ),
                    },
                    variable_blocks: [
                        VariableBlock {
                            variables: [
                                Variable {
                                    name: "getLocalVariable",
                                    data_type: DataTypeReference {
                                        referenced_type: "DINT",
                                    },
                                },
                            ],
                            variable_block_type: Local,
                        },
                    ],
                    body: [
                        Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "bar",
                                    },
                                ),
                                base: None,
                            },
                            right: LiteralInteger {
                                value: 5,
                            },
                        },
                    ],
                },
                PropertyImplementation {
                    kind: Set,
                    location: SourceLocation {
                        span: Range(
                            TextLocation {
                                line: 10,
                                column: 16,
                                offset: 251,
                            }..TextLocation {
                                line: 10,
                                column: 19,
                                offset: 254,
                            },
                        ),
                    },
                    variable_blocks: [
                        VariableBlock {
                            variables: [
                                Variable {
                                    name: "setLocalVariable",
                                    data_type: DataTypeReference {
                                        referenced_type: "DINT",
                                    },
                                },
                            ],
                            variable_block_type: Local,
                        },
                    ],
                    body: [
                        Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "localNonExistingVariable",
                                    },
                                ),
                                base: None,
                            },
                            right: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "bar",
                                    },
                                ),
                                base: None,
                            },
                        },
                    ],
                },
            ],
        },
    ]
    "###);
}

#[test]
fn property_with_missing_name() {
    let source = r"
        FUNCTION_BLOCK foo
            PROPERTY : INT  // <- Missing name
            END_PROPERTY
        END_FUNCTION_BLOCK
    ";

    let (_, diagnostics) = parse_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E007]: Unexpected token: expected Identifier but found :
      ┌─ <internal>:3:22
      │
    3 │             PROPERTY : INT  // <- Missing name
      │                      ^ Unexpected token: expected Identifier but found :

    error[E001]: Property definition is missing a name
      ┌─ <internal>:3:22
      │
    3 │             PROPERTY : INT  // <- Missing name
      │                      ^ Property definition is missing a name
    ");
}

#[test]
fn property_with_missing_datatype() {
    let source = r"
        FUNCTION_BLOCK foo
            PROPERTY bar    // <- Missing datatype
            END_PROPERTY
        END_FUNCTION_BLOCK
    ";

    let (_, diagnostics) = parse_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E001]: Property definition is missing a datatype
      ┌─ <internal>:3:22
      │
    3 │             PROPERTY bar    // <- Missing datatype
      │                      ^^^ Property definition is missing a datatype
    ");
}

#[test]
fn property_with_variable_block() {
    let source = r"
        FUNCTION_BLOCK foo
            PROPERTY bar : DINT
                VAR
                    // Invalid variable block, should be in a getter or setter
                END_VAR

                GET
                    // ...
                END_GET
            END_PROPERTY
        END_FUNCTION_BLOCK
    ";

    let (_, diagnostics) = parse_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E007]: Variable blocks may only be defined within a GET or SET block in the context of properties
      ┌─ <internal>:4:17
      │
    4 │                 VAR
      │                 ^^^ Variable blocks may only be defined within a GET or SET block in the context of properties
    ");
}
