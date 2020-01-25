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
                    name: "x",
                },
            ],
        },
    ],
    else_block: [
        Reference {
            name: "y",
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
                    name: "x",
                },
            ],
        },
        ConditionalBlock {
            condition: Reference {
                name: "y",
            },
            body: [
                Reference {
                    name: "z",
                },
            ],
        },
        ConditionalBlock {
            condition: Reference {
                name: "w",
            },
            body: [
                Reference {
                    name: "v",
                },
            ],
        },
    ],
    else_block: [
        Reference {
            name: "u",
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
        FOR x TO 10 DO
        END_FOR
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"ForLoopStatement {
    start: Reference {
        name: "x",
    },
    end: LiteralNumber {
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
        FOR x TO 10 BY 7 DO 
        END_FOR
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"ForLoopStatement {
    start: Reference {
        name: "x",
    },
    end: LiteralNumber {
        value: "10",
    },
    by_step: Some(
        LiteralNumber {
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
        FOR x TO y DO
        END_FOR
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"ForLoopStatement {
    start: Reference {
        name: "x",
    },
    end: Reference {
        name: "y",
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
        FOR x TO y DO
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
    start: Reference {
        name: "x",
    },
    end: Reference {
        name: "y",
    },
    by_step: None,
    body: [
        Reference {
            name: "x",
        },
        Reference {
            name: "y",
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
            name: "x",
        },
        right: LiteralNumber {
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
            name: "x",
        },
        Reference {
            name: "y",
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
            name: "x",
        },
        right: LiteralNumber {
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
            name: "x",
        },
        Reference {
            name: "y",
        },
    ],
}"#;

    assert_eq!(ast_string, expected_ast);
}
