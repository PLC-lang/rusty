// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{
    ast::*,
    lexer::Token,
    parser::tests::{empty_stmt, ref_to},
    test_utils::tests::parse,
    Diagnostic,
};
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

    let (compilation_unit, diagnostics) = parse(src);
    //expected end of statement (e.g. ;), but found KeywordEndProgram at line: 1 offset: 14..25"
    //Expecting a missing semicolon message
    let expected = Diagnostic::unexpected_token_found("KeywordSemicolon", "'foo()'", (76..81).into());
    assert_eq!(diagnostics[0], expected);

    let pou = &compilation_unit.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements),
        r#"[
    CallStatement {
        operator: Reference {
            name: "buz",
        },
        parameters: None,
    },
]"#
    );
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

    let (compilation_unit, diagnostics) = parse(src);
    let expected = Diagnostic::unexpected_token_found("KeywordParensClose", "'c'", (58..59).into());
    assert_eq!(diagnostics, vec![expected]);

    let pou = &compilation_unit.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements),
        format!(
            "{:#?}",
            vec![AstStatement::CallStatement {
                location: SourceRange::undefined(),
                operator: Box::new(ref_to("buz")),
                parameters: Box::new(Some(AstStatement::ExpressionList {
                    expressions: vec![ref_to("a"), ref_to("b"),],
                    id: 0
                })),
                id: 0
            }]
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

    let (compilation_unit, diagnostics) = parse(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_token("[KeywordParensClose]", (57..58).into()),
            Diagnostic::unexpected_token_found("KeywordParensClose", "';'", (57..58).into()),
            Diagnostic::unexpected_token_found("KeywordSemicolon", "')'", (60..61).into())
        ]
    );

    let pou = &compilation_unit.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements),
        format!(
            "{:#?}",
            vec![
                AstStatement::CallStatement {
                    location: SourceRange::undefined(),
                    operator: Box::new(ref_to("buz")),
                    parameters: Box::new(Some(AstStatement::ExpressionList {
                        expressions: vec![ref_to("a"), ref_to("b")],
                        id: 0
                    })),
                    id: 0
                },
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

    let (cu, diagnostics) = parse(src);
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
    Reference {
        name: "x",
    },
]"#
    );

    assert_eq!(diagnostics[0], Diagnostic::unexpected_token_found("Literal", ";", (41..42).into()));
}

#[test]
fn incomplete_statement_in_parantheses_recovery_test() {
    let src = "
        PROGRAM exp 
            (1 + 2 - ) + 3;
            x;
        END_PROGRAM
        ";

    let (cu, diagnostics) = parse(src);
    let pou = &cu.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements),
        r#"[
    BinaryExpression {
        operator: Plus,
        left: BinaryExpression {
            operator: Minus,
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
        right: LiteralInteger {
            value: 3,
        },
    },
    Reference {
        name: "x",
    },
]"#
    );

    assert_eq!(diagnostics[0], Diagnostic::unexpected_token_found("Literal", ")", (43..44).into()));
}

#[test]
fn mismatched_parantheses_recovery_test() {
    let src = "
        PROGRAM exp 
            (1 + 2;
            x;
        END_PROGRAM
        ";

    let (cu, diagnostics) = parse(src);
    let pou = &cu.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements),
        r#"[
    BinaryExpression {
        operator: Plus,
        left: LiteralInteger {
            value: 1,
        },
        right: LiteralInteger {
            value: 2,
        },
    },
    Reference {
        name: "x",
    },
]"#
    );

    assert_eq!(diagnostics[0], Diagnostic::missing_token("[KeywordParensClose]", (40..41).into()));
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

    let (cu, diagnostics) = parse(src);
    let pou = &cu.units[0];
    assert_eq!(
        format!("{:#?}", pou.variable_blocks[0]),
        format!(
            "{:#?}",
            VariableBlock {
                constant: false,
                access: AccessModifier::Protected,
                retain: false,
                location: SourceRange::undefined(),
                variables: vec![Variable {
                    name: "c".into(),
                    data_type_declaration: DataTypeDeclaration::DataTypeReference {
                        referenced_type: "INT".into(),
                        location: SourceRange::undefined(),
                    },
                    initializer: None,
                    address: None,
                    location: SourceRange::undefined(),
                },],
                variable_block_type: VariableBlockType::Local,
                linkage: LinkageType::Internal,
            }
        )
    );

    assert_eq!(
        diagnostics[0],
        Diagnostic::unexpected_token_found(
            format!("{:?}", Token::KeywordEndVar).as_str(),
            "'4 : INT;'",
            (77..85).into()
        )
    );
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

    let (cu, diagnostics) = parse(src);
    let pou = &cu.units[0];
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

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_token("KeywordColon or KeywordComma", (53..54).into()),
            Diagnostic::unexpected_token_found("DataTypeDefinition", "KeywordSemicolon", (61..62).into()),
            Diagnostic::missing_token("KeywordColon", (108..109).into()),
            Diagnostic::unexpected_token_found("DataTypeDefinition", "KeywordComma", (108..109).into()),
            Diagnostic::unexpected_token_found("KeywordSemicolon", "', : INT'", (108..115).into()),
            Diagnostic::unexpected_token_found("DataTypeDefinition", "KeywordSemicolon", (143..144).into()),
        ]
    );
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
    let (_, diagnostics) = parse(src);

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_token("[KeywordSemicolon, KeywordColon]", (79..85).into()),
            Diagnostic::unexpected_token_found("KeywordSemicolon", "'END_IF'", (79..85).into())
        ]
    );
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
    let (unit, diagnostics) = parse(src);

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_token("[KeywordEndIf, KeywordElseIf, KeywordElse]", (145..156).into()),
            Diagnostic::unexpected_token_found("KeywordEndIf", "'END_PROGRAM'", (145..156).into()),
        ]
    );

    assert_eq!(
        format!("{:#?}", unit.implementations[0].statements),
        format!(
            "{:#?}",
            vec![AstStatement::IfStatement {
                blocks: vec![ConditionalBlock {
                    condition: Box::new(AstStatement::LiteralBool {
                        value: false,
                        location: SourceRange::undefined(),
                        id: 0
                    }),
                    body: vec![
                        AstStatement::IfStatement {
                            blocks: vec![ConditionalBlock {
                                condition: Box::new(AstStatement::LiteralBool {
                                    value: true,
                                    location: SourceRange::undefined(),
                                    id: 0
                                }),
                                body: vec![AstStatement::Assignment {
                                    left: Box::new(ref_to("x")),
                                    right: Box::new(ref_to("y")),
                                    id: 0
                                }],
                            }],
                            else_block: vec![],
                            location: SourceRange::undefined(),
                            id: 0,
                        },
                        AstStatement::Assignment {
                            left: Box::new(ref_to("y")),
                            right: Box::new(ref_to("x")),
                            id: 0
                        }
                    ]
                },],
                else_block: vec![],
                location: SourceRange::undefined(),
                id: 0,
            },]
        )
    );
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
    let (_, diagnostics) = parse(src);

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_token("[KeywordSemicolon, KeywordColon]", (81..88).into()),
            Diagnostic::unexpected_token_found("KeywordSemicolon", "'END_FOR'", (81..88).into())
        ]
    );
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
    let (unit, diagnostics) = parse(src);

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_token("[KeywordEndFor]", (159..170).into()),
            Diagnostic::unexpected_token_found("KeywordEndFor", "'END_PROGRAM'", (159..170).into()),
        ]
    );

    assert_eq!(
        format!("{:#?}", unit.implementations[0].statements),
        format!(
            "{:#?}",
            vec![AstStatement::ForLoopStatement {
                counter: Box::new(ref_to("x")),
                start: Box::new(AstStatement::LiteralInteger {
                    value: 1,
                    location: SourceRange::undefined(),
                    id: 0
                }),
                end: Box::new(AstStatement::LiteralInteger {
                    value: 2,
                    location: SourceRange::undefined(),
                    id: 0
                }),
                by_step: None,
                body: vec![
                    AstStatement::ForLoopStatement {
                        counter: Box::new(ref_to("x")),
                        start: Box::new(AstStatement::LiteralInteger {
                            value: 1,
                            location: SourceRange::undefined(),
                            id: 0
                        }),
                        end: Box::new(AstStatement::LiteralInteger {
                            value: 2,
                            location: SourceRange::undefined(),
                            id: 0
                        }),

                        by_step: None,
                        body: vec![AstStatement::Assignment {
                            left: Box::new(ref_to("y")),
                            right: Box::new(ref_to("x")),
                            id: 0
                        },],
                        location: SourceRange::undefined(),
                        id: 0
                    },
                    AstStatement::Assignment {
                        left: Box::new(ref_to("x")),
                        right: Box::new(ref_to("y")),
                        id: 0
                    }
                ],
                location: SourceRange::undefined(),
                id: 0
            },]
        )
    );
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
    let (unit, diagnostics) = parse(src);

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_token("[KeywordSemicolon, KeywordColon]", (69..74).into()),
            Diagnostic::unexpected_token_found("KeywordSemicolon", "'UNTIL'", (69..74).into()),
        ]
    );

    assert_eq!(
        format!("{:#?}", unit.implementations[0].statements),
        format!(
            "{:#?}",
            vec![
                AstStatement::RepeatLoopStatement {
                    body: vec![AstStatement::Assignment {
                        left: Box::new(ref_to("x")),
                        right: Box::new(AstStatement::LiteralInteger {
                            value: 3,
                            location: SourceRange::undefined(),
                            id: 0
                        }),
                        id: 0
                    }],
                    condition: Box::new(AstStatement::BinaryExpression {
                        left: Box::new(ref_to("x")),
                        right: Box::new(ref_to("y")),
                        operator: crate::ast::Operator::Equal,
                        id: 0
                    }),
                    location: SourceRange::undefined(),
                    id: 0
                },
                AstStatement::Assignment { left: Box::new(ref_to("y")), right: Box::new(ref_to("x")), id: 0 }
            ]
        )
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
    let (unit, diagnostics) = parse(src);

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_token("[KeywordUntil, KeywordEndRepeat]", (158..169).into()),
            Diagnostic::unexpected_token_found("KeywordUntil", "'END_PROGRAM'", (158..169).into()),
        ]
    );

    assert_eq!(
        format!("{:#?}", unit.implementations[0].statements),
        format!(
            "{:#?}",
            vec![AstStatement::RepeatLoopStatement {
                body: vec![
                    AstStatement::RepeatLoopStatement {
                        body: vec![empty_stmt()],
                        condition: Box::new(AstStatement::BinaryExpression {
                            left: Box::new(ref_to("x")),
                            right: Box::new(ref_to("y")),
                            operator: crate::ast::Operator::Equal,
                            id: 0
                        }),
                        location: SourceRange::undefined(),
                        id: 0
                    },
                    AstStatement::Assignment {
                        left: Box::new(ref_to("y")),
                        right: Box::new(ref_to("x")),
                        id: 0
                    }
                ],
                condition: Box::new(empty_stmt()),
                location: SourceRange::undefined(),
                id: 0
            },]
        )
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
    let (unit, diagnostics) = parse(src);

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::unexpected_token_found("Literal", "END_PROGRAM", (171..182).into()),
            Diagnostic::missing_token("[KeywordEndRepeat]", (171..182).into()),
            Diagnostic::unexpected_token_found("KeywordEndRepeat", "'END_PROGRAM'", (171..182).into()),
        ]
    );

    assert_eq!(
        format!("{:#?}", unit.implementations[0].statements),
        format!(
            "{:#?}",
            vec![AstStatement::RepeatLoopStatement {
                body: vec![
                    AstStatement::RepeatLoopStatement {
                        body: vec![empty_stmt()],
                        condition: Box::new(AstStatement::BinaryExpression {
                            left: Box::new(ref_to("x")),
                            right: Box::new(ref_to("y")),
                            operator: crate::ast::Operator::Equal,
                            id: 0
                        }),
                        location: SourceRange::undefined(),
                        id: 0
                    },
                    AstStatement::Assignment {
                        left: Box::new(ref_to("y")),
                        right: Box::new(ref_to("x")),
                        id: 0
                    }
                ],
                condition: Box::new(empty_stmt()),
                location: SourceRange::undefined(),
                id: 0
            },]
        )
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
    let (unit, diagnostics) = parse(src);

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_token("[KeywordEndRepeat]", (177..188).into()),
            Diagnostic::unexpected_token_found("KeywordEndRepeat", "'END_PROGRAM'", (177..188).into()),
        ]
    );

    assert_eq!(
        format!("{:#?}", unit.implementations[0].statements),
        format!(
            "{:#?}",
            vec![AstStatement::RepeatLoopStatement {
                body: vec![
                    AstStatement::RepeatLoopStatement {
                        body: vec![empty_stmt()],
                        condition: Box::new(AstStatement::BinaryExpression {
                            left: Box::new(ref_to("x")),
                            right: Box::new(ref_to("y")),
                            operator: crate::ast::Operator::Equal,
                            id: 0
                        }),
                        location: SourceRange::undefined(),
                        id: 0
                    },
                    AstStatement::Assignment {
                        left: Box::new(ref_to("y")),
                        right: Box::new(ref_to("x")),
                        id: 0
                    }
                ],
                condition: Box::new(AstStatement::BinaryExpression {
                    left: Box::new(ref_to("x")),
                    right: Box::new(ref_to("y")),
                    operator: crate::ast::Operator::Equal,
                    id: 0
                }),
                location: SourceRange::undefined(),
                id: 0
            },]
        )
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
    let (unit, diagnostics) = parse(src);

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_token("[KeywordSemicolon, KeywordColon]", (77..86).into()),
            Diagnostic::unexpected_token_found("KeywordSemicolon", "'END_WHILE'", (77..86).into()),
        ]
    );

    assert_eq!(
        format!("{:#?}", unit.implementations[0].statements),
        format!(
            "{:#?}",
            vec![
                AstStatement::WhileLoopStatement {
                    body: vec![AstStatement::Assignment {
                        left: Box::new(ref_to("x")),
                        right: Box::new(AstStatement::LiteralInteger {
                            value: 3,
                            location: SourceRange::undefined(),
                            id: 0
                        }),
                        id: 0
                    }],
                    condition: Box::new(AstStatement::BinaryExpression {
                        left: Box::new(ref_to("x")),
                        right: Box::new(ref_to("y")),
                        operator: crate::ast::Operator::Equal,
                        id: 0
                    }),
                    location: SourceRange::undefined(),
                    id: 0
                },
                AstStatement::Assignment { left: Box::new(ref_to("y")), right: Box::new(ref_to("x")), id: 0 }
            ]
        )
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
    let (unit, diagnostics) = parse(src);

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_token("[KeywordEndWhile]", (156..167).into()),
            Diagnostic::unexpected_token_found("KeywordEndWhile", "'END_PROGRAM'", (156..167).into()),
        ]
    );

    assert_eq!(
        format!("{:#?}", unit.implementations[0].statements),
        format!(
            "{:#?}",
            vec![AstStatement::WhileLoopStatement {
                body: vec![
                    AstStatement::WhileLoopStatement {
                        body: vec![empty_stmt()],
                        condition: Box::new(AstStatement::BinaryExpression {
                            left: Box::new(ref_to("x")),
                            right: Box::new(ref_to("y")),
                            operator: crate::ast::Operator::Equal,
                            id: 0
                        }),
                        location: SourceRange::undefined(),
                        id: 0
                    },
                    AstStatement::Assignment {
                        left: Box::new(ref_to("y")),
                        right: Box::new(ref_to("x")),
                        id: 0
                    }
                ],
                condition: Box::new(AstStatement::BinaryExpression {
                    left: Box::new(ref_to("x")),
                    right: Box::new(ref_to("y")),
                    operator: crate::ast::Operator::Equal,
                    id: 0
                }),
                location: SourceRange::undefined(),
                id: 0
            },]
        )
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
    let (unit, diagnostics) = parse(src);

    assert_eq!(diagnostics, vec![Diagnostic::missing_token("KeywordDo", (55..56).into()),]);

    assert_eq!(
        format!("{:#?}", unit.implementations[0].statements),
        format!(
            "{:#?}",
            vec![AstStatement::WhileLoopStatement {
                body: vec![AstStatement::Assignment {
                    left: Box::new(ref_to("y")),
                    right: Box::new(ref_to("x")),
                    id: 0
                }],
                condition: Box::new(AstStatement::BinaryExpression {
                    left: Box::new(ref_to("x")),
                    right: Box::new(ref_to("y")),
                    operator: crate::ast::Operator::Equal,
                    id: 0
                }),
                location: SourceRange::undefined(),
                id: 0
            }]
        )
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
    let (unit, diagnostics) = parse(src);

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_token("[KeywordSemicolon, KeywordColon]", (68..76).into()),
            Diagnostic::unexpected_token_found("KeywordSemicolon", "'END_CASE'", (68..76).into()),
        ]
    );

    assert_eq!(
        format!("{:#?}", unit.implementations[0].statements),
        format!(
            "{:#?}",
            vec![AstStatement::CaseStatement {
                selector: Box::new(ref_to("x")),
                case_blocks: vec![ConditionalBlock {
                    condition: Box::new(ref_to("y")),
                    body: vec![AstStatement::Assignment {
                        left: Box::new(ref_to("y")),
                        right: Box::new(ref_to("z")),
                        id: 0
                    }],
                },],
                else_block: vec![],
                location: SourceRange::undefined(),
                id: 0
            }]
        )
    );
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
    let (cu, diagnostics) = parse(src);

    assert_eq!(
        format!("{:#?}", cu.implementations[0].statements),
        r#"[
    CaseStatement {
        selector: Reference {
            name: "x",
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
                        left: Reference {
                            name: "x",
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

    assert_eq!(diagnostics, vec![Diagnostic::unexpected_token_found("Literal", ":", (85..86).into())]);
}

#[test]
fn pointer_type_without_to_test() {
    let src = r#"
        TYPE SamplePointer :
            POINTER INT;
        END_TYPE 
        "#;
    let (result, diagnostics) = parse(src);
    let pointer_type = &result.user_types[0];
    let expected = UserTypeDeclaration {
        data_type: DataType::PointerType {
            name: Some("SamplePointer".into()),
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "INT".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        location: SourceRange::undefined(),
        initializer: None,
        scope: None,
    };
    assert_eq!(format!("{expected:#?}"), format!("{pointer_type:#?}").as_str());

    assert_eq!(
        vec![
            Diagnostic::ImprovementSuggestion {
                message: "'POINTER TO' is not a standard keyword, use REF_TO instead".to_string(),
                range: vec![(42..49).into()]
            },
            Diagnostic::unexpected_token_found("KeywordTo", "INT", (50..53).into())
        ],
        diagnostics
    )
}

#[test]
fn pointer_type_with_wrong_keyword_to_test() {
    let src = r#"
        TYPE SamplePointer :
            POINTER tu INT;
        END_TYPE 
        "#;
    let (result, diagnostics) = parse(src);
    let pointer_type = &result.user_types[0];
    let expected = UserTypeDeclaration {
        data_type: DataType::PointerType {
            name: Some("SamplePointer".into()),
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "tu".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        location: SourceRange::undefined(),
        initializer: None,
        scope: None,
    };
    assert_eq!(format!("{expected:#?}"), format!("{pointer_type:#?}").as_str());
    assert_eq!(
        vec![
            Diagnostic::ImprovementSuggestion {
                message: "'POINTER TO' is not a standard keyword, use REF_TO instead".to_string(),
                range: vec![(42..49).into()]
            },
            Diagnostic::unexpected_token_found("KeywordTo", "tu", (50..52).into()),
            Diagnostic::unexpected_token_found("KeywordSemicolon", "'INT'", (53..56).into())
        ],
        diagnostics
    )
}

#[test]
fn bitwise_access_error_validation() {
    let src = "PROGRAM exp 
    a.1e5; 
    b.%f6;
    END_PROGRAM";
    let (ast, diagnostics) = parse(src);
    println!("{ast:?}");

    assert_eq!(2, diagnostics.len());
    let errs = vec![
        Diagnostic::unexpected_token_found("Integer", r#"Exponent value: 1e5"#, (19..22).into()),
        Diagnostic::unexpected_token_found("KeywordSemicolon", "'f6'", (32..34).into()),
    ];
    assert_eq!(errs, diagnostics);
}
