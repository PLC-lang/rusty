use crate::test_utils;

#[test]
fn function_pointer_definition() {
    let source = r"
    TYPE VTable:
        STRUCT
            fooPtr : REF_TO foo := REF(foo);
        END_STRUCT
    END_TYPE

    FUNCTION foo : INT
        VAR_INPUT
            in : DINT;
        END_VAR
    END_FUNCTION
    ";

    let (unit, diagnostics) = test_utils::tests::parse(source);

    assert_eq!(diagnostics, vec![]);
    insta::assert_debug_snapshot!(unit.user_types[0], @r#"
    UserTypeDeclaration {
        data_type: StructType {
            name: Some(
                "VTable",
            ),
            variables: [
                Variable {
                    name: "fooPtr",
                    data_type: DataTypeDefinition {
                        data_type: PointerType {
                            name: None,
                            referenced_type: DataTypeReference {
                                referenced_type: "foo",
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
                                            name: "foo",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
            ],
        },
        initializer: None,
        scope: None,
    }
    "#);
}

#[test]
fn function_pointer_assignment() {
    let source = r"
    VAR_GLOBAL
        vtable_global : VTable := (fooPtr := REF(foo));
    END_VAR

    TYPE VTable:
        STRUCT
            fooPtr : REF_TO foo;
        END_STRUCT
    END_TYPE

    FUNCTION foo : INT
        VAR_INPUT
            in : DINT;
        END_VAR
    END_FUNCTION
    ";

    let (unit, diagnostics) = test_utils::tests::parse(source);

    assert_eq!(diagnostics, vec![]);
    insta::assert_debug_snapshot!(unit.global_vars[0], @r#"
    VariableBlock {
        variables: [
            Variable {
                name: "vtable_global",
                data_type: DataTypeReference {
                    referenced_type: "VTable",
                },
                initializer: Some(
                    ParenExpression {
                        expression: Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "fooPtr",
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
                                                name: "foo",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
                            },
                        },
                    },
                ),
            },
        ],
        variable_block_type: Global,
    }
    "#);
}

#[test]
fn function_pointer_call() {
    let source = r"
    VAR_GLOBAL
        vtable_global : VTable := (fooPtr := REF(foo));
    END_VAR

    TYPE VTable:
        STRUCT
            fooPtr : REF_TO foo;
        END_STRUCT
    END_TYPE

    FUNCTION foo : INT
        VAR_INPUT
            in : DINT;
        END_VAR
    END_FUNCTION

    FUNCTION main
        vtable_global.fooPtr^(420);
    END_FUNCTION
    ";

    let (unit, diagnostics) = test_utils::tests::parse(source);

    assert_eq!(diagnostics, vec![]);
    insta::assert_debug_snapshot!(unit.implementations[1].statements[0], @r#"
    CallStatement {
        operator: ReferenceExpr {
            kind: Deref,
            base: Some(
                ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "fooPtr",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "vtable_global",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
            ),
        },
        parameters: Some(
            LiteralInteger {
                value: 420,
            },
        ),
    }
    "#);
}
