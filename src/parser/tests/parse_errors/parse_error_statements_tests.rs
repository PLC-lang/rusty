// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{parser::tests::ref_to, test_utils::tests::parse_buffered};
use insta::{assert_debug_snapshot, assert_snapshot};
use plc_ast::ast::{
    AccessModifier, AstFactory, DataType, DataTypeDeclaration, LinkageType, UserTypeDeclaration, Variable,
    VariableBlock, VariableBlockType,
};
use plc_source::source_location::SourceLocation;
use pretty_assertions::*;

/*
 * These tests deal with parsing-behavior in the expressions: ()  expressions: ()  presence of errors.
 * following scenarios will be tested:
 *  - missing semico id: ()  id: () lons at different locations
 *  - incomplete statements
 *  - incomplete statement-blocks (brackets)
 */

#[test]
fn missing_semicolon_after_call() {
    /*
     * missing ';' after buz will be reported, both calls should be
     * parsed correctly
     */
    let src = r"
                PROGRAM foo
                    buz()
                    foo();
                END_PROGRAM
    ";

    let (compilation_unit, diagnostics) = parse_buffered(src);
    //expected end of statement (e.g. ;), but found KeywordEndProgram at line: 1 offset: 14..25"
    //Expecting a missing semicolon message
    assert_snapshot!(diagnostics);

    let pou = &compilation_unit.implementations[0];
    assert_debug_snapshot!(pou.statements);
}

#[test]
fn missing_comma_in_call_parameters() {
    /*
     * the missing comma after b will end the expression-list so we expect a ')'
     * c will not be parsed as an expression
     */
    let src = r"
                PROGRAM foo
                    buz(a,b c);
                END_PROGRAM
    ";

    let (compilation_unit, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics);

    let pou = &compilation_unit.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements),
        format!(
            "{:#?}",
            vec![AstFactory::create_call_statement(
                ref_to("buz"),
                Some(AstFactory::create_expression_list(
                    vec![ref_to("a"), ref_to("b")],
                    SourceLocation::internal(),
                    0
                )),
                0,
                SourceLocation::internal()
            )]
        )
    );
}

#[test]
fn illegal_semicolon_in_call_parameters() {
    /*
     * _ the semicolon after b will close the call-statement
     * _ c will be its own reference with an illegal token ')'
     */
    let src = r"
                PROGRAM foo
                    buz(a,b; c);
                END_PROGRAM
    ";

    let (compilation_unit, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics);

    let pou = &compilation_unit.implementations[0];

    assert_eq!(
        format!("{:#?}", pou.statements),
        format!(
            "{:#?}",
            vec![
                AstFactory::create_call_statement(
                    ref_to("buz"),
                    Some(AstFactory::create_expression_list(
                        vec![ref_to("a"), ref_to("b")],
                        SourceLocation::internal(),
                        0
                    )),
                    0,
                    SourceLocation::internal()
                ),
                ref_to("c")
            ]
        )
    );
}

#[test]
fn incomplete_statement_test() {
    let src = "
        PROGRAM exp
            1 + 2 +;
            x;
        END_PROGRAM
        ";

    let (cu, diagnostics) = parse_buffered(src);
    let pou = &cu.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements),
        r#"[
    BinaryExpression {
        operator: Plus,
        left: BinaryExpression {
            operator: Plus,
            left: LiteralInteger {
                value: 1,
            },
            right: LiteralInteger {
                value: 2,
            },
        },
        right: EmptyStatement,
    },
    ReferenceExpr {
        kind: Member(
            Identifier {
                name: "x",
            },
        ),
        base: None,
    },
]"#
    );
    assert_snapshot!(diagnostics);
}

#[test]
fn incomplete_statement_in_parantheses_recovery_test() {
    let src = "
        PROGRAM exp
            (1 + 2 - ) + 3;
            x;
        END_PROGRAM
        ";

    let (cu, diagnostics) = parse_buffered(src);
    let pou = &cu.implementations[0];
    assert_snapshot!(diagnostics);
    assert_debug_snapshot!(pou.statements);
}

#[test]
fn mismatched_parantheses_recovery_test() {
    let src = "
        PROGRAM exp
            (1 + 2;
            x;
        END_PROGRAM
        ";

    let (cu, diagnostics) = parse_buffered(src);
    let pou = &cu.implementations[0];

    assert_snapshot!(diagnostics);
    assert_debug_snapshot!(pou.statements);
}

#[test]
fn invalid_variable_name_error_recovery() {
    let src = "
        PROGRAM p
            VAR
                c : INT;
                4 : INT;
            END_VAR
        END_PROGRAM
        ";

    let (cu, diagnostics) = parse_buffered(src);
    let pou = &cu.pous[0];
    assert_eq!(
        format!("{:#?}", pou.variable_blocks[0]),
        format!(
            "{:#?}",
            VariableBlock {
                constant: false,
                access: AccessModifier::Protected,
                retain: false,
                location: SourceLocation::internal(),
                variables: vec![Variable {
                    name: "c".into(),
                    data_type_declaration: DataTypeDeclaration::Reference {
                        referenced_type: "INT".into(),
                        location: SourceLocation::internal(),
                    },
                    initializer: None,
                    address: None,
                    location: SourceLocation::internal(),
                },],
                kind: VariableBlockType::Local,
                linkage: LinkageType::Internal,
            }
        )
    );
    assert_snapshot!(diagnostics);
}

#[test]
fn invalid_variable_data_type_error_recovery() {
    let src = "
        PROGRAM p
            VAR
                a DINT : ;
                c : INT;
                h , , : INT;
                f , INT : ;
            END_VAR
        END_PROGRAM
        ";

    let (cu, diagnostics) = parse_buffered(src);
    let pou = &cu.pous[0];
    assert_eq!(
        format!("{:#?}", pou.variable_blocks[0]),
        r#"VariableBlock {
    variables: [
        Variable {
            name: "c",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
    ],
    variable_block_type: Local,
}"#
    );
    assert_snapshot!(diagnostics);
}

#[test]
fn test_if_with_missing_semicolon_in_body() {
    //regress, this used to end in an endless loop
    let src = "PROGRAM My_PRG
            IF TRUE THEN
                x := x + 1
            END_IF
        END_PROGRAM
    ";
    let (_, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics);
}

#[test]
fn test_nested_if_with_missing_end_if() {
    //regress, this used to end in an endless loop
    let src = "PROGRAM My_PRG
            IF FALSE THEN
                IF TRUE THEN
                    x := y;
            END_IF
            y := x;
        END_PROGRAM
    ";
    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics);

    insta::assert_snapshot!(format!("{:#?}", unit.implementations[0].statements), @r#"
    [
        IfStatement {
            blocks: [
                ConditionalBlock {
                    condition: LiteralBool {
                        value: false,
                    },
                    body: [
                        IfStatement {
                            blocks: [
                                ConditionalBlock {
                                    condition: LiteralBool {
                                        value: true,
                                    },
                                    body: [
                                        Assignment {
                                            left: ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "x",
                                                    },
                                                ),
                                                base: None,
                                            },
                                            right: ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "y",
                                                    },
                                                ),
                                                base: None,
                                            },
                                        },
                                    ],
                                },
                            ],
                            else_block: [],
                        },
                        Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "y",
                                    },
                                ),
                                base: None,
                            },
                            right: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "x",
                                    },
                                ),
                                base: None,
                            },
                        },
                    ],
                },
            ],
            else_block: [],
        },
    ]
    "#);
}

#[test]
fn test_for_with_missing_semicolon_in_body() {
    //regress, this used to end in an endless loop
    let src = "PROGRAM My_PRG
            FOR x := 1 TO 2 DO
                y := x
            END_FOR
        END_PROGRAM
    ";
    let (_, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics);
}

#[test]
fn test_nested_for_with_missing_end_for() {
    //regress, this used to end in an endless loop
    let src = "PROGRAM My_PRG
            FOR x := 1 TO 2 DO
                FOR x := 1 TO 2 DO
                    y := x;
            END_FOR
            x := y;
        END_PROGRAM
    ";
    let (unit, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics);

    insta::assert_snapshot!(
        format!("{:#?}", unit.implementations[0].statements),
        @r#"
    [
        ForLoopStatement {
            counter: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: None,
            },
            start: LiteralInteger {
                value: 1,
            },
            end: LiteralInteger {
                value: 2,
            },
            by_step: None,
            body: [
                ForLoopStatement {
                    counter: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: None,
                    },
                    start: LiteralInteger {
                        value: 1,
                    },
                    end: LiteralInteger {
                        value: 2,
                    },
                    by_step: None,
                    body: [
                        Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "y",
                                    },
                                ),
                                base: None,
                            },
                            right: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "x",
                                    },
                                ),
                                base: None,
                            },
                        },
                    ],
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: None,
                    },
                    right: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "y",
                            },
                        ),
                        base: None,
                    },
                },
            ],
        },
    ]
    "#);
}

#[test]
fn test_repeat_with_missing_semicolon_in_body() {
    //regress, this used to end in an endless loop
    let src = "PROGRAM My_PRG
            REPEAT
                x := 3
            UNTIL x = y END_REPEAT
            y := x;
           END_PROGRAM
    ";
    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics);

    insta::assert_snapshot!(
        format!("{:#?}", unit.implementations[0].statements),
        @r#"
    [
        RepeatLoopStatement {
            condition: BinaryExpression {
                operator: Equal,
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "x",
                        },
                    ),
                    base: None,
                },
                right: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "y",
                        },
                    ),
                    base: None,
                },
            },
            body: [
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: None,
                    },
                    right: LiteralInteger {
                        value: 3,
                    },
                },
            ],
        },
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "y",
                    },
                ),
                base: None,
            },
            right: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: None,
            },
        },
    ]
    "#
    );
}

#[test]
fn test_nested_repeat_with_missing_until_end_repeat() {
    //regress, this used to end in an endless loop
    let src = "PROGRAM My_PRG
            REPEAT
                REPEAT
                    ;
                UNTIL x = y END_REPEAT
                y := x;
           END_PROGRAM
    ";
    let (unit, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics);
    insta::assert_snapshot!(
        format!("{:#?}", unit.implementations[0].statements),
        @r#"
    [
        RepeatLoopStatement {
            condition: EmptyStatement,
            body: [
                RepeatLoopStatement {
                    condition: BinaryExpression {
                        operator: Equal,
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "x",
                                },
                            ),
                            base: None,
                        },
                        right: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "y",
                                },
                            ),
                            base: None,
                        },
                    },
                    body: [
                        EmptyStatement,
                    ],
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "y",
                            },
                        ),
                        base: None,
                    },
                    right: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: None,
                    },
                },
            ],
        },
    ]
    "#
    );
}

#[test]
fn test_nested_repeat_with_missing_condition_and_end_repeat() {
    //regress, this used to end in an endless loop
    let src = "PROGRAM My_PRG
            REPEAT
                REPEAT
                    ;
                UNTIL x = y END_REPEAT
                y := x;
            UNTIL
           END_PROGRAM
    ";
    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics);

    insta::assert_snapshot!(
        format!("{:#?}", unit.implementations[0].statements),
        @r#"
    [
        RepeatLoopStatement {
            condition: EmptyStatement,
            body: [
                RepeatLoopStatement {
                    condition: BinaryExpression {
                        operator: Equal,
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "x",
                                },
                            ),
                            base: None,
                        },
                        right: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "y",
                                },
                            ),
                            base: None,
                        },
                    },
                    body: [
                        EmptyStatement,
                    ],
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "y",
                            },
                        ),
                        base: None,
                    },
                    right: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: None,
                    },
                },
            ],
        },
    ]
    "#
    );
}

#[test]
fn test_nested_repeat_with_missing_end_repeat() {
    //regress, this used to end in an endless loop
    let src = "PROGRAM My_PRG
            REPEAT
                REPEAT
                    ;
                UNTIL x = y END_REPEAT
                y := x;
            UNTIL x = y
           END_PROGRAM
    ";
    let (unit, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics);

    insta::assert_snapshot!(
        format!("{:#?}", unit.implementations[0].statements),
        @r#"
    [
        RepeatLoopStatement {
            condition: BinaryExpression {
                operator: Equal,
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "x",
                        },
                    ),
                    base: None,
                },
                right: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "y",
                        },
                    ),
                    base: None,
                },
            },
            body: [
                RepeatLoopStatement {
                    condition: BinaryExpression {
                        operator: Equal,
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "x",
                                },
                            ),
                            base: None,
                        },
                        right: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "y",
                                },
                            ),
                            base: None,
                        },
                    },
                    body: [
                        EmptyStatement,
                    ],
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "y",
                            },
                        ),
                        base: None,
                    },
                    right: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: None,
                    },
                },
            ],
        },
    ]
    "#
    );
}

#[test]
fn test_while_with_missing_semicolon_in_body() {
    //regress, this used to end in an endless loop
    let src = "PROGRAM My_PRG
            WHILE x = y DO
                x := 3
            END_WHILE
            y := x;
           END_PROGRAM
    ";
    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics);

    insta::assert_snapshot!(
        format!("{:#?}", unit.implementations[0].statements),
        @r#"
    [
        WhileLoopStatement {
            condition: BinaryExpression {
                operator: Equal,
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "x",
                        },
                    ),
                    base: None,
                },
                right: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "y",
                        },
                    ),
                    base: None,
                },
            },
            body: [
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: None,
                    },
                    right: LiteralInteger {
                        value: 3,
                    },
                },
            ],
        },
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "y",
                    },
                ),
                base: None,
            },
            right: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: None,
            },
        },
    ]
    "#
    );
}

#[test]
fn test_nested_while_with_missing_end_while() {
    //regress, this used to end in an endless loop
    let src = "PROGRAM My_PRG
            WHILE x = y DO
                WHILE x = y DO
                    ;
                END_WHILE
                y := x;
           END_PROGRAM
    ";
    let (unit, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics);

    insta::assert_snapshot!(
        format!("{:#?}", unit.implementations[0].statements),
        @r#"
    [
        WhileLoopStatement {
            condition: BinaryExpression {
                operator: Equal,
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "x",
                        },
                    ),
                    base: None,
                },
                right: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "y",
                        },
                    ),
                    base: None,
                },
            },
            body: [
                WhileLoopStatement {
                    condition: BinaryExpression {
                        operator: Equal,
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "x",
                                },
                            ),
                            base: None,
                        },
                        right: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "y",
                                },
                            ),
                            base: None,
                        },
                    },
                    body: [
                        EmptyStatement,
                    ],
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "y",
                            },
                        ),
                        base: None,
                    },
                    right: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: None,
                    },
                },
            ],
        },
    ]
    "#
    );
}

#[test]
fn test_while_with_missing_do() {
    //regress, this used to end in an endless loop
    let src = "PROGRAM My_PRG
            WHILE x = y
                y := x;
            END_WHILE
           END_PROGRAM
    ";
    let (unit, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics);

    insta::assert_snapshot!(
        format!("{:#?}", unit.implementations[0].statements),
        @r#"
    [
        WhileLoopStatement {
            condition: BinaryExpression {
                operator: Equal,
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "x",
                        },
                    ),
                    base: None,
                },
                right: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "y",
                        },
                    ),
                    base: None,
                },
            },
            body: [
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "y",
                            },
                        ),
                        base: None,
                    },
                    right: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: None,
                    },
                },
            ],
        },
    ]
    "#
    );
}

#[test]
fn test_case_body_with_missing_semicolon() {
    //regress, this used to end in an endless loop
    let src = "PROGRAM My_PRG
           CASE x OF
           y: y := z
           END_CASE
           END_PROGRAM
    ";
    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics);

    insta::assert_snapshot!(
        format!("{:#?}", unit.implementations[0].statements),
        @r#"
    [
        CaseStatement {
            selector: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: None,
            },
            case_blocks: [
                ConditionalBlock {
                    condition: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "y",
                            },
                        ),
                        base: None,
                    },
                    body: [
                        Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "y",
                                    },
                                ),
                                base: None,
                            },
                            right: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "z",
                                    },
                                ),
                                base: None,
                            },
                        },
                    ],
                },
            ],
            else_block: [],
        },
    ]
    "#);
}

#[test]
fn test_case_without_condition() {
    let src = "PROGRAM My_PRG
                CASE x OF
                    1:
                    : x := 3;
                END_CASE
            END_PROGRAM

    ";
    let (cu, diagnostics) = parse_buffered(src);

    assert_eq!(
        format!("{:#?}", cu.implementations[0].statements),
        r#"[
    CaseStatement {
        selector: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "x",
                },
            ),
            base: None,
        },
        case_blocks: [
            ConditionalBlock {
                condition: LiteralInteger {
                    value: 1,
                },
                body: [],
            },
            ConditionalBlock {
                condition: EmptyStatement,
                body: [
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "x",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 3,
                        },
                    },
                ],
            },
        ],
        else_block: [],
    },
]"#
    );
    assert_snapshot!(diagnostics);
}

#[test]
fn pointer_type_without_to_test() {
    let src = r#"
        TYPE SamplePointer :
            POINTER INT;
        END_TYPE
        "#;
    let (result, diagnostics) = parse_buffered(src);
    let pointer_type = &result.user_types[0];
    let expected = UserTypeDeclaration {
        data_type: DataType::PointerType {
            name: Some("SamplePointer".into()),
            referenced_type: Box::new(DataTypeDeclaration::Reference {
                referenced_type: "INT".to_string(),
                location: SourceLocation::internal(),
            }),
            auto_deref: None,
            type_safe: false,
            is_function: false,
        },
        location: SourceLocation::internal(),
        initializer: None,
        scope: None,
    };
    assert_eq!(format!("{expected:#?}"), format!("{pointer_type:#?}").as_str());

    assert_snapshot!(diagnostics);
}

#[test]
fn pointer_type_with_wrong_keyword_to_test() {
    let src = r#"
        TYPE SamplePointer :
            POINTER tu INT;
        END_TYPE
        "#;
    let (result, diagnostics) = parse_buffered(src);
    let pointer_type = &result.user_types[0];
    let expected = UserTypeDeclaration {
        data_type: DataType::PointerType {
            name: Some("SamplePointer".into()),
            referenced_type: Box::new(DataTypeDeclaration::Reference {
                referenced_type: "tu".to_string(),
                location: SourceLocation::internal(),
            }),
            auto_deref: None,
            type_safe: false,
            is_function: false,
        },
        location: SourceLocation::internal(),
        initializer: None,
        scope: None,
    };
    assert_eq!(format!("{expected:#?}"), format!("{pointer_type:#?}").as_str());
    assert_snapshot!(diagnostics);
}

#[test]
fn bitwise_access_error_validation() {
    let src = "PROGRAM exp
    a.1e5;   // exponent illegal
    b.%f6;   // f is no valid direct access modifier
    END_PROGRAM";
    let (_, diagnostics) = parse_buffered(src);
    assert_snapshot!(diagnostics);
}
