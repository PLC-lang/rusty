use crate::test_utils;

#[test]
fn function_pointer_definition() {
    let source = r"
    FUNCTION pointAtMe : DINT
        VAR_INPUT
            x : DINT;
            y : DINT;
        END_VAR
    END_FUNCTION

    FUNCTION main
        VAR
            myFunctionPointer : REF_TO pointAtMe;
        END_VAR
    END_FUNCTION
    ";

    let (unit, diagnostics) = test_utils::tests::parse(source);

    assert_eq!(diagnostics, vec![]);
    insta::assert_debug_snapshot!(unit.pous[1].variable_blocks, @r#"
    [
        VariableBlock {
            variables: [
                Variable {
                    name: "myFunctionPointer",
                    data_type: DataTypeDefinition {
                        data_type: PointerType {
                            name: None,
                            referenced_type: DataTypeReference {
                                referenced_type: "pointAtMe",
                            },
                            auto_deref: None,
                            type_safe: true,
                        },
                    },
                },
            ],
            variable_block_type: Local,
        },
    ]
    "#);
}

#[test]
fn function_pointer_initialization() {
    let source = r"
    FUNCTION pointAtMe : DINT
        VAR_INPUT
            x : DINT;
            y : DINT;
        END_VAR
    END_FUNCTION

    FUNCTION main
        VAR
            myFunctionPointer : REF_TO pointAtMe := REF(pointAtMe);
        END_VAR
    END_FUNCTION
    ";

    let (unit, diagnostics) = test_utils::tests::parse(source);

    assert_eq!(diagnostics, vec![]);
    insta::assert_debug_snapshot!(unit.pous[1].variable_blocks, @r#"
    [
        VariableBlock {
            variables: [
                Variable {
                    name: "myFunctionPointer",
                    data_type: DataTypeDefinition {
                        data_type: PointerType {
                            name: None,
                            referenced_type: DataTypeReference {
                                referenced_type: "pointAtMe",
                            },
                            auto_deref: None,
                            type_safe: true,
                        },
                    },
                    initializer: Some(
                        CallStatement {
                            operator: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "REF",
                                    },
                                ),
                                base: None,
                            },
                            parameters: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "pointAtMe",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
            ],
            variable_block_type: Local,
        },
    ]
    "#);
}

#[test]
fn function_pointer_assignment() {
    let source = r"
    FUNCTION pointAtMe : DINT
        VAR_INPUT
            x : DINT;
            y : DINT;
        END_VAR
    END_FUNCTION

    FUNCTION main
        VAR
            myFunctionPointer : REF_TO pointAtMe;
        END_VAR

        myFunctionPointer := REF(pointAtMe);
    END_FUNCTION
    ";

    let (unit, diagnostics) = test_utils::tests::parse(source);

    assert_eq!(diagnostics, vec![]);
    insta::assert_debug_snapshot!(unit.implementations[1], @r#"
    Implementation {
        name: "main",
        type_name: "main",
        linkage: Internal,
        pou_type: Function,
        statements: [
            Assignment {
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "myFunctionPointer",
                        },
                    ),
                    base: None,
                },
                right: CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "REF",
                            },
                        ),
                        base: None,
                    },
                    parameters: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "pointAtMe",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
            },
        ],
        location: SourceLocation {
            span: Range(13:8 - 13:44),
        },
        name_location: SourceLocation {
            span: Range(8:13 - 8:17),
        },
        end_location: SourceLocation {
            span: Range(14:4 - 14:16),
        },
        overriding: false,
        generic: false,
        access: None,
    }
    "#);
}
