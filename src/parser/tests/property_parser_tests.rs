use crate::test_utils::tests::{parse, parse_buffered};

#[test]
fn properties_can_be_parsed() {
    let source = r"
        FUNCTION_BLOCK foo
            PROPERTY_GET bar: INT
                VAR
                    getLocalVariable : DINT;
                END_VAR

                bar := 5;
            END_PROPERTY
            PROPERTY_SET bar: INT
                VAR
                    setLocalVariable : DINT;
                END_VAR

                localNonExistingVariable := bar;
            END_PROPERTY
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse(source);

    assert_eq!(diagnostics, vec![]);

    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.pous[0].name, "foo");

    let properties = &unit.pous[0].properties;
    assert_eq!(properties.len(), 1);
    assert_eq!(properties[0].ident.name, "bar");
    assert_eq!(properties[0].implementations.len(), 2);

    insta::assert_debug_snapshot!(properties, @r#"
    [
        PropertyBlock {
            ident: Identifier {
                name: "bar",
                location: SourceLocation {
                    span: Range(2:25 - 2:28),
                },
            },
            implementations: [
                PropertyImplementation {
                    kind: Get,
                    datatype: DataTypeReference {
                        referenced_type: "INT",
                    },
                    location: SourceLocation {
                        span: Range(2:12 - 2:24),
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
                    end_location: SourceLocation {
                        span: Range(8:12 - 8:24),
                    },
                },
                PropertyImplementation {
                    kind: Set,
                    datatype: DataTypeReference {
                        referenced_type: "INT",
                    },
                    location: SourceLocation {
                        span: Range(9:12 - 9:24),
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
                    end_location: SourceLocation {
                        span: Range(15:12 - 15:24),
                    },
                },
            ],
        },
    ]
    "#);
}

#[test]
fn property_with_missing_name() {
    let source = r"
        FUNCTION_BLOCK foo
            PROPERTY_GET : INT  // <- Missing name
            END_PROPERTY
        END_FUNCTION_BLOCK
    ";

    let (_, diagnostics) = parse_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E001]: Property definition is missing a name
      ┌─ <internal>:3:26
      │
    3 │             PROPERTY_GET : INT  // <- Missing name
      │                          ^ Property definition is missing a name
    ");
}

#[test]
fn property_with_missing_colon() {
    let source = r"
        FUNCTION_BLOCK foo
            PROPERTY_GET bar    // <- Missing colon and datatype
            END_PROPERTY
        END_FUNCTION_BLOCK
    ";

    let (_, diagnostics) = parse_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E001]: Property definition is missing ':'
      ┌─ <internal>:3:26
      │
    3 │             PROPERTY_GET bar    // <- Missing colon and datatype
      │                          ^^^ Property definition is missing ':'
    ");
}

#[test]
fn property_with_missing_datatype() {
    let source = r"
        FUNCTION_BLOCK foo
            PROPERTY_GET bar:    // <- Missing datatype
            END_PROPERTY
        END_FUNCTION_BLOCK
    ";

    let (_, diagnostics) = parse_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E001]: Property definition is missing a datatype
      ┌─ <internal>:3:29
      │
    3 │             PROPERTY_GET bar:    // <- Missing datatype
      │                             ^ Property definition is missing a datatype
    ");
}
