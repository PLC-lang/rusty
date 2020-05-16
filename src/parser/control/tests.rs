use crate::lexer;
use crate::parser::parse;
use pretty_assertions::*;


#[test]
fn if_statement() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        IF TRUE THEN
        END_IF
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"IfStatement {
    blocks: [
        ConditionalBlock {
            condition: LiteralBool {
                value: true,
            },
            body: [],
        },
    ],
    else_block: [],
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn if_else_statement_with_expressions() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        IF TRUE THEN
            x;
        ELSE
            y;
        END_IF
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"IfStatement {
    blocks: [
        ConditionalBlock {
            condition: LiteralBool {
                value: true,
            },
            body: [
                Reference {
                    elements: [
                        "x",
                    ],
                },
            ],
        },
    ],
    else_block: [
        Reference {
            elements: [
                "y",
            ],
        },
    ],
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn if_elsif_elsif_else_statement_with_expressions() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        IF TRUE THEN
            x;
        ELSIF y THEN
            z;
        ELSIF w THEN
            v;
        ELSE
            u;
        END_IF
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"IfStatement {
    blocks: [
        ConditionalBlock {
            condition: LiteralBool {
                value: true,
            },
            body: [
                Reference {
                    elements: [
                        "x",
                    ],
                },
            ],
        },
        ConditionalBlock {
            condition: Reference {
                elements: [
                    "y",
                ],
            },
            body: [
                Reference {
                    elements: [
                        "z",
                    ],
                },
            ],
        },
        ConditionalBlock {
            condition: Reference {
                elements: [
                    "w",
                ],
            },
            body: [
                Reference {
                    elements: [
                        "v",
                    ],
                },
            ],
        },
    ],
    else_block: [
        Reference {
            elements: [
                "u",
            ],
        },
    ],
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn for_with_literals_statement() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        FOR y := x TO 10 DO
        END_FOR
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"ForLoopStatement {
    counter: Reference {
        elements: [
            "y",
        ],
    },
    start: Reference {
        elements: [
            "x",
        ],
    },
    end: LiteralInteger {
        value: "10",
    },
    by_step: None,
    body: [],
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn for_with_step_statement() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        FOR x := 1 TO 10 BY 7 DO 
        END_FOR
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"ForLoopStatement {
    counter: Reference {
        elements: [
            "x",
        ],
    },
    start: LiteralInteger {
        value: "1",
    },
    end: LiteralInteger {
        value: "10",
    },
    by_step: Some(
        LiteralInteger {
            value: "7",
        },
    ),
    body: [],
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn for_with_reference_statement() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        FOR z := x TO y DO
        END_FOR
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"ForLoopStatement {
    counter: Reference {
        elements: [
            "z",
        ],
    },
    start: Reference {
        elements: [
            "x",
        ],
    },
    end: Reference {
        elements: [
            "y",
        ],
    },
    by_step: None,
    body: [],
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn for_with_body_statement() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        FOR z := x TO y DO
            x;
            y;
        END_FOR
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"ForLoopStatement {
    counter: Reference {
        elements: [
            "z",
        ],
    },
    start: Reference {
        elements: [
            "x",
        ],
    },
    end: Reference {
        elements: [
            "y",
        ],
    },
    by_step: None,
    body: [
        Reference {
            elements: [
                "x",
            ],
        },
        Reference {
            elements: [
                "y",
            ],
        },
    ],
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn while_with_literal() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        WHILE TRUE DO
        END_WHILE
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"WhileLoopStatement {
    condition: LiteralBool {
        value: true,
    },
    body: [],
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn while_with_expression() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        WHILE x < 7 DO 
        END_WHILE
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"WhileLoopStatement {
    condition: BinaryExpression {
        operator: Less,
        left: Reference {
            elements: [
                "x",
            ],
        },
        right: LiteralInteger {
            value: "7",
        },
    },
    body: [],
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn while_with_body_statement() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        WHILE TRUE DO
            x;
            y;
        END_WHILE
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"WhileLoopStatement {
    condition: LiteralBool {
        value: true,
    },
    body: [
        Reference {
            elements: [
                "x",
            ],
        },
        Reference {
            elements: [
                "y",
            ],
        },
    ],
}"#;

    assert_eq!(ast_string, expected_ast);
}


#[test]
fn repeat_with_literal() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        REPEAT
        UNTIL TRUE
        END_REPEAT
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"RepeatLoopStatement {
    condition: LiteralBool {
        value: true,
    },
    body: [],
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn repeat_with_expression() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        REPEAT
        UNTIL x > 7
        END_REPEAT
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"RepeatLoopStatement {
    condition: BinaryExpression {
        operator: Greater,
        left: Reference {
            elements: [
                "x",
            ],
        },
        right: LiteralInteger {
            value: "7",
        },
    },
    body: [],
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn repeat_with_body_statement() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        REPEAT
            x;
            y;
        UNTIL TRUE
        END_REPEAT
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"RepeatLoopStatement {
    condition: LiteralBool {
        value: true,
    },
    body: [
        Reference {
            elements: [
                "x",
            ],
        },
        Reference {
            elements: [
                "y",
            ],
        },
    ],
}"#;

    assert_eq!(ast_string, expected_ast);
}


#[test]
fn case_statement_with_one_condition() {
     let lexer = lexer::lex(
        "
        PROGRAM exp 
        CASE StateMachine OF
        1: x;
        END_CASE
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"CaseStatement {
    selector: Reference {
        elements: [
            "StateMachine",
        ],
    },
    case_blocks: [
        ConditionalBlock {
            condition: LiteralInteger {
                value: "1",
            },
            body: [
                Reference {
                    elements: [
                        "x",
                    ],
                },
            ],
        },
    ],
    else_block: [],
}"#;

    assert_eq!(ast_string, expected_ast);   
}

#[test]
fn case_statement_with_else_and_no_condition() {
     let lexer = lexer::lex(
        "
        PROGRAM exp 
        CASE StateMachine OF
        ELSE
        END_CASE
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"CaseStatement {
    selector: Reference {
        elements: [
            "StateMachine",
        ],
    },
    case_blocks: [],
    else_block: [],
}"#;

    assert_eq!(ast_string, expected_ast);   
}


#[test]
fn case_statement_with_no_conditions() {
     let lexer = lexer::lex(
        "
        PROGRAM exp 
        CASE StateMachine OF
        END_CASE
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"CaseStatement {
    selector: Reference {
        elements: [
            "StateMachine",
        ],
    },
    case_blocks: [],
    else_block: [],
}"#;

    assert_eq!(ast_string, expected_ast);   
}

#[test]
fn case_statement_with_one_condition_and_an_else() {
     let lexer = lexer::lex(
        "
        PROGRAM exp 
        CASE StateMachine OF
        1: x;
        ELSE
            y;
        END_CASE
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"CaseStatement {
    selector: Reference {
        elements: [
            "StateMachine",
        ],
    },
    case_blocks: [
        ConditionalBlock {
            condition: LiteralInteger {
                value: "1",
            },
            body: [
                Reference {
                    elements: [
                        "x",
                    ],
                },
            ],
        },
    ],
    else_block: [
        Reference {
            elements: [
                "y",
            ],
        },
    ],
}"#;

    assert_eq!(ast_string, expected_ast);   
}

#[test]
fn case_statement_with_one_empty_condition_and_an_else() {
     let lexer = lexer::lex(
        "
        PROGRAM exp 
        CASE StateMachine OF
        1:
        ELSE
            y;
        END_CASE
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"CaseStatement {
    selector: Reference {
        elements: [
            "StateMachine",
        ],
    },
    case_blocks: [
        ConditionalBlock {
            condition: LiteralInteger {
                value: "1",
            },
            body: [],
        },
    ],
    else_block: [
        Reference {
            elements: [
                "y",
            ],
        },
    ],
}"#;

    assert_eq!(ast_string, expected_ast);   
}

#[test]
fn case_statement_with_multiple_conditions() {
     let lexer = lexer::lex(
        "
        PROGRAM exp 
        CASE StateMachine OF
            1: x;
            2: y; yy; yyy;
            3: z;
        END_CASE
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"CaseStatement {
    selector: Reference {
        elements: [
            "StateMachine",
        ],
    },
    case_blocks: [
        ConditionalBlock {
            condition: LiteralInteger {
                value: "1",
            },
            body: [
                Reference {
                    elements: [
                        "x",
                    ],
                },
            ],
        },
        ConditionalBlock {
            condition: LiteralInteger {
                value: "2",
            },
            body: [
                Reference {
                    elements: [
                        "y",
                    ],
                },
                Reference {
                    elements: [
                        "yy",
                    ],
                },
                Reference {
                    elements: [
                        "yyy",
                    ],
                },
            ],
        },
        ConditionalBlock {
            condition: LiteralInteger {
                value: "3",
            },
            body: [
                Reference {
                    elements: [
                        "z",
                    ],
                },
            ],
        },
    ],
    else_block: [],
}"#;

    assert_eq!(ast_string, expected_ast);   
}

#[test]
fn case_statement_with_multiple_expressions_per_condition() {
     let lexer = lexer::lex(
        "
        PROGRAM exp 
        CASE StateMachine OF
            1,2,3: x;
            4..5: y;
        END_CASE
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"CaseStatement {
    selector: Reference {
        elements: [
            "StateMachine",
        ],
    },
    case_blocks: [
        ConditionalBlock {
            condition: ExpressionList {
                expressions: [
                    LiteralInteger {
                        value: "1",
                    },
                    LiteralInteger {
                        value: "2",
                    },
                    LiteralInteger {
                        value: "3",
                    },
                ],
            },
            body: [
                Reference {
                    elements: [
                        "x",
                    ],
                },
            ],
        },
        ConditionalBlock {
            condition: RangeStatement {
                start: LiteralInteger {
                    value: "4",
                },
                end: LiteralInteger {
                    value: "5",
                },
            },
            body: [
                Reference {
                    elements: [
                        "y",
                    ],
                },
            ],
        },
    ],
    else_block: [],
}"#;

    assert_eq!(ast_string, expected_ast);
}
