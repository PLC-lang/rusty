use insta::assert_debug_snapshot;

use crate::test_utils::tests::parse;

#[test]
fn function_can_be_a_variable_type() {
    let src = r"
{external}
FUNCTION prot : DINT
VAR_INPUT
    a : DINT;
END_VAR
END_FUNCTION

VAR_GLOBAL
    f : REF_TO prot;
END_VAR
";
    let result = parse(src).0;
    assert_debug_snapshot!(result, @r###"
    CompilationUnit {
        global_vars: [
            VariableBlock {
                variables: [
                    Variable {
                        name: "f",
                        data_type: DataTypeDefinition {
                            data_type: PointerType {
                                name: None,
                                referenced_type: DataTypeReference {
                                    referenced_type: "prot",
                                },
                                auto_deref: None,
                            },
                        },
                    },
                ],
                variable_block_type: Global,
            },
        ],
        var_config: [],
        pous: [
            POU {
                name: "prot",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "a",
                                data_type: DataTypeReference {
                                    referenced_type: "DINT",
                                },
                            },
                        ],
                        variable_block_type: Input(
                            ByVal,
                        ),
                    },
                ],
                pou_type: Function,
                return_type: Some(
                    DataTypeReference {
                        referenced_type: "DINT",
                    },
                ),
                interfaces: [],
                properties: [],
            },
        ],
        implementations: [
            Implementation {
                name: "prot",
                type_name: "prot",
                linkage: External,
                pou_type: Function,
                statements: [],
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 6,
                            column: 0,
                            offset: 65,
                        }..TextLocation {
                            line: 5,
                            column: 7,
                            offset: 64,
                        },
                    ),
                },
                name_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 2,
                            column: 9,
                            offset: 21,
                        }..TextLocation {
                            line: 2,
                            column: 13,
                            offset: 25,
                        },
                    ),
                },
                end_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 6,
                            column: 0,
                            offset: 65,
                        }..TextLocation {
                            line: 6,
                            column: 12,
                            offset: 77,
                        },
                    ),
                },
                overriding: false,
                generic: false,
                access: None,
            },
        ],
        interfaces: [],
        user_types: [],
        file: File(
            "test.st",
        ),
    }
    "###);
}

#[test]
fn function_variable_can_be_called() {
    let src = r"
{external}
FUNCTION prot : DINT
VAR_INPUT
    a : DINT;
END_VAR
END_FUNCTION

FUNCTION test : DINT
VAR
    f : REF_TO prot := REF(prot);
END_VAR
    f := REF(prot);
   f^(1);
END_FUNCTION
";
    let result = parse(src).0;
    assert_debug_snapshot!(result, @r###"
    CompilationUnit {
        global_vars: [],
        var_config: [],
        pous: [
            POU {
                name: "prot",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "a",
                                data_type: DataTypeReference {
                                    referenced_type: "DINT",
                                },
                            },
                        ],
                        variable_block_type: Input(
                            ByVal,
                        ),
                    },
                ],
                pou_type: Function,
                return_type: Some(
                    DataTypeReference {
                        referenced_type: "DINT",
                    },
                ),
                interfaces: [],
                properties: [],
            },
            POU {
                name: "test",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "f",
                                data_type: DataTypeDefinition {
                                    data_type: PointerType {
                                        name: None,
                                        referenced_type: DataTypeReference {
                                            referenced_type: "prot",
                                        },
                                        auto_deref: None,
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
                                                        name: "prot",
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
                ],
                pou_type: Function,
                return_type: Some(
                    DataTypeReference {
                        referenced_type: "DINT",
                    },
                ),
                interfaces: [],
                properties: [],
            },
        ],
        implementations: [
            Implementation {
                name: "prot",
                type_name: "prot",
                linkage: External,
                pou_type: Function,
                statements: [],
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 6,
                            column: 0,
                            offset: 65,
                        }..TextLocation {
                            line: 5,
                            column: 7,
                            offset: 64,
                        },
                    ),
                },
                name_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 2,
                            column: 9,
                            offset: 21,
                        }..TextLocation {
                            line: 2,
                            column: 13,
                            offset: 25,
                        },
                    ),
                },
                end_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 6,
                            column: 0,
                            offset: 65,
                        }..TextLocation {
                            line: 6,
                            column: 12,
                            offset: 77,
                        },
                    ),
                },
                overriding: false,
                generic: false,
                access: None,
            },
            Implementation {
                name: "test",
                type_name: "test",
                linkage: Internal,
                pou_type: Function,
                statements: [
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "f",
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
                                            name: "prot",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    },
                    CallStatement {
                        operator: ReferenceExpr {
                            kind: Deref,
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "f",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                        parameters: Some(
                            LiteralInteger {
                                value: 1,
                            },
                        ),
                    },
                ],
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 12,
                            column: 4,
                            offset: 150,
                        }..TextLocation {
                            line: 13,
                            column: 9,
                            offset: 175,
                        },
                    ),
                },
                name_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 8,
                            column: 9,
                            offset: 88,
                        }..TextLocation {
                            line: 8,
                            column: 13,
                            offset: 92,
                        },
                    ),
                },
                end_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 14,
                            column: 0,
                            offset: 176,
                        }..TextLocation {
                            line: 14,
                            column: 12,
                            offset: 188,
                        },
                    ),
                },
                overriding: false,
                generic: false,
                access: None,
            },
        ],
        interfaces: [],
        user_types: [],
        file: File(
            "test.st",
        ),
    }
    "###);
}

#[test]
fn void_variable_can_be_cast_to_function() {
    let src = r"
{external}
FUNCTION prot : DINT
VAR_INPUT
    a : DINT;
END_VAR
END_FUNCTION

FUNCTION test : DINT
VAR
    f : REF_TO __VOID := REF(prot);
END_VAR
    f := REF(prot);
    prot#f^(1);
END_FUNCTION
";
    let result = parse(src).0;
    assert_debug_snapshot!(result, @r###"
    CompilationUnit {
        global_vars: [],
        var_config: [],
        pous: [
            POU {
                name: "prot",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "a",
                                data_type: DataTypeReference {
                                    referenced_type: "DINT",
                                },
                            },
                        ],
                        variable_block_type: Input(
                            ByVal,
                        ),
                    },
                ],
                pou_type: Function,
                return_type: Some(
                    DataTypeReference {
                        referenced_type: "DINT",
                    },
                ),
                interfaces: [],
                properties: [],
            },
            POU {
                name: "test",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "f",
                                data_type: DataTypeDefinition {
                                    data_type: PointerType {
                                        name: None,
                                        referenced_type: DataTypeReference {
                                            referenced_type: "__VOID",
                                        },
                                        auto_deref: None,
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
                                                        name: "prot",
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
                ],
                pou_type: Function,
                return_type: Some(
                    DataTypeReference {
                        referenced_type: "DINT",
                    },
                ),
                interfaces: [],
                properties: [],
            },
        ],
        implementations: [
            Implementation {
                name: "prot",
                type_name: "prot",
                linkage: External,
                pou_type: Function,
                statements: [],
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 6,
                            column: 0,
                            offset: 65,
                        }..TextLocation {
                            line: 5,
                            column: 7,
                            offset: 64,
                        },
                    ),
                },
                name_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 2,
                            column: 9,
                            offset: 21,
                        }..TextLocation {
                            line: 2,
                            column: 13,
                            offset: 25,
                        },
                    ),
                },
                end_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 6,
                            column: 0,
                            offset: 65,
                        }..TextLocation {
                            line: 6,
                            column: 12,
                            offset: 77,
                        },
                    ),
                },
                overriding: false,
                generic: false,
                access: None,
            },
            Implementation {
                name: "test",
                type_name: "test",
                linkage: Internal,
                pou_type: Function,
                statements: [
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "f",
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
                                            name: "prot",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    },
                    CallStatement {
                        operator: ReferenceExpr {
                            kind: Deref,
                            base: Some(
                                ReferenceExpr {
                                    kind: Cast(
                                        Identifier {
                                            name: "f",
                                        },
                                    ),
                                    base: Some(
                                        ReferenceExpr {
                                            kind: Member(
                                                Identifier {
                                                    name: "prot",
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
                                value: 1,
                            },
                        ),
                    },
                ],
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 12,
                            column: 4,
                            offset: 152,
                        }..TextLocation {
                            line: 13,
                            column: 15,
                            offset: 183,
                        },
                    ),
                },
                name_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 8,
                            column: 9,
                            offset: 88,
                        }..TextLocation {
                            line: 8,
                            column: 13,
                            offset: 92,
                        },
                    ),
                },
                end_location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 14,
                            column: 0,
                            offset: 184,
                        }..TextLocation {
                            line: 14,
                            column: 12,
                            offset: 196,
                        },
                    ),
                },
                overriding: false,
                generic: false,
                access: None,
            },
        ],
        interfaces: [],
        user_types: [],
        file: File(
            "test.st",
        ),
    }
    "###);
}
