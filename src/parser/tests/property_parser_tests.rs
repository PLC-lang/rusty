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
                    span: Range(2:21 - 2:24),
                },
            },
            datatype: DataTypeReference {
                referenced_type: "INT",
            },
            implementations: [
                PropertyImplementation {
                    kind: Get,
                    location: SourceLocation {
                        span: Range(3:16 - 3:19),
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
                        span: Range(9:16 - 9:23),
                    },
                },
                PropertyImplementation {
                    kind: Set,
                    location: SourceLocation {
                        span: Range(10:16 - 10:19),
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
                        span: Range(16:16 - 16:23),
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
