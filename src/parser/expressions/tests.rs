use super::Statement;
use crate::parser::parse;
use crate::lexer;
use pretty_assertions::*;

#[test]
fn single_statement_parsed() {
    let lexer = lexer::lex("PROGRAM exp x; END_PROGRAM");
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    if let Statement::Reference { name } = statement {
        assert_eq!(name, "x");
    } else {
        panic!("Expected Reference but found {:?}", statement);
    }
}

#[test]
fn literal_can_be_parsed() {
    let lexer = lexer::lex("PROGRAM exp 7; END_PROGRAM");
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    if let Statement::LiteralNumber { value } = statement {
        assert_eq!(value, "7");
    } else {
        panic!("Expected LiteralNumber but found {:?}", statement);
    }
}

#[test]
fn additon_of_two_variables_parsed() {
    let lexer = lexer::lex("PROGRAM exp x+y; END_PROGRAM");
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    if let Statement::BinaryExpression {
        operator,
        left,  //Box<Reference> {name : left}),
        right, //Box<Reference> {name : right}),
    } = statement
    {
        if let Statement::Reference { name } = &**left {
            assert_eq!(name, "x");
        }
        if let Statement::Reference { name } = &**right {
            assert_eq!(name, "y");
        }
        assert_eq!(operator, &super::Operator::Plus);
    } else {
        panic!("Expected Reference but found {:?}", statement);
    }
}

#[test]
fn additon_of_three_variables_parsed() {
    let lexer = lexer::lex("PROGRAM exp x+y-z; END_PROGRAM");
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    if let Statement::BinaryExpression {
        operator,
        left,  //Box<Reference> {name : left}),
        right, //Box<Reference> {name : right}),
    } = statement
    {
        assert_eq!(operator, &super::Operator::Plus);
        if let Statement::Reference { name } = &**left {
            assert_eq!(name, "x");
        }
        if let Statement::BinaryExpression {
            operator,
            left,
            right,
        } = &**right
        {
            if let Statement::Reference { name } = &**left {
                assert_eq!(name, "y");
            }
            if let Statement::Reference { name } = &**right {
                assert_eq!(name, "z");
            }
            assert_eq!(operator, &super::Operator::Minus);
        } else {
            panic!("Expected Reference but found {:?}", statement);
        }
    } else {
        panic!("Expected Reference but found {:?}", statement);
    }
}

#[test]
fn parenthesis_expressions_should_not_change_the_ast() {
    let lexer = lexer::lex("PROGRAM exp (x+y); END_PROGRAM");
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    if let Statement::BinaryExpression {
        operator,
        left,
        right,
    } = statement
    {
        if let Statement::Reference { name } = &**left {
            assert_eq!(name, "x");
        }
        if let Statement::Reference { name } = &**right {
            assert_eq!(name, "y");
        }
        assert_eq!(operator, &super::Operator::Plus);
    } else {
        panic!("Expected Reference but found {:?}", statement);
    }
}

#[test]
fn multiplication_expressions_parse() {
    let lexer = lexer::lex("PROGRAM exp 1*2/7; END_PROGRAM");
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Multiplication,
    left: LiteralNumber {
        value: "1",
    },
    right: BinaryExpression {
        operator: Division,
        left: LiteralNumber {
            value: "2",
        },
        right: LiteralNumber {
            value: "7",
        },
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn addition_ast_test() {
    let lexer = lexer::lex("PROGRAM exp 1+2; END_PROGRAM");
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Plus,
    left: LiteralNumber {
        value: "1",
    },
    right: LiteralNumber {
        value: "2",
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn multiplication_ast_test() {
    let lexer = lexer::lex("PROGRAM exp 1+2*3; END_PROGRAM");
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Plus,
    left: LiteralNumber {
        value: "1",
    },
    right: BinaryExpression {
        operator: Multiplication,
        left: LiteralNumber {
            value: "2",
        },
        right: LiteralNumber {
            value: "3",
        },
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn term_ast_test() {
    let lexer = lexer::lex("PROGRAM exp 1+2*3+4; END_PROGRAM");
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Plus,
    left: LiteralNumber {
        value: "1",
    },
    right: BinaryExpression {
        operator: Plus,
        left: BinaryExpression {
            operator: Multiplication,
            left: LiteralNumber {
                value: "2",
            },
            right: LiteralNumber {
                value: "3",
            },
        },
        right: LiteralNumber {
            value: "4",
        },
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn module_expression_test() {
    let lexer = lexer::lex("PROGRAM exp 5 MOD 2; END_PROGRAM");

    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Modulo,
    left: LiteralNumber {
        value: "5",
    },
    right: LiteralNumber {
        value: "2",
    },
}"#;

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn parenthesized_term_ast_test() {
    let lexer = lexer::lex("PROGRAM exp (1+2)*(3+4); END_PROGRAM");
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Multiplication,
    left: BinaryExpression {
        operator: Plus,
        left: LiteralNumber {
            value: "1",
        },
        right: LiteralNumber {
            value: "2",
        },
    },
    right: BinaryExpression {
        operator: Plus,
        left: LiteralNumber {
            value: "3",
        },
        right: LiteralNumber {
            value: "4",
        },
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn boolean_literals_can_be_parsed() {
    let lexer = lexer::lex("PROGRAM exp TRUE OR FALSE; END_PROGRAM");
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Or,
    left: LiteralBool {
        value: true,
    },
    right: LiteralBool {
        value: false,
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn assignment_test() {
    let lexer = lexer::lex("PROGRAM exp x := 3; x := 1 + 2; END_PROGRAM");
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    {
        let statement = &prg.statements[0];
        let ast_string = format!("{:#?}", statement);
        let expected_ast = r#"Assignment {
    left: Reference {
        name: "x",
    },
    right: LiteralNumber {
        value: "3",
    },
}"#;
        assert_eq!(ast_string, expected_ast);
    }

    {
        let statement = &prg.statements[1];
        let ast_string = format!("{:#?}", statement);
        let expected_ast = r#"Assignment {
    left: Reference {
        name: "x",
    },
    right: BinaryExpression {
        operator: Plus,
        left: LiteralNumber {
            value: "1",
        },
        right: LiteralNumber {
            value: "2",
        },
    },
}"#;
        assert_eq!(ast_string, expected_ast);
    }
}

#[test]
fn equality_expression_test() {
    let lexer = lexer::lex("PROGRAM exp x = 3; x - 0 <> 1 + 2; END_PROGRAM");
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    {
        let statement = &prg.statements[0];
        let ast_string = format!("{:#?}", statement);
        let expected_ast = r#"BinaryExpression {
    operator: Equal,
    left: Reference {
        name: "x",
    },
    right: LiteralNumber {
        value: "3",
    },
}"#;
        assert_eq!(ast_string, expected_ast);
    }

    {
        let statement = &prg.statements[1];
        let ast_string = format!("{:#?}", statement);
        let expected_ast = r#"BinaryExpression {
    operator: NotEqual,
    left: BinaryExpression {
        operator: Minus,
        left: Reference {
            name: "x",
        },
        right: LiteralNumber {
            value: "0",
        },
    },
    right: BinaryExpression {
        operator: Plus,
        left: LiteralNumber {
            value: "1",
        },
        right: LiteralNumber {
            value: "2",
        },
    },
}"#;
        assert_eq!(ast_string, expected_ast);
    }
}
#[test]
fn comparison_expression_test() {
    let lexer = lexer::lex(
        "PROGRAM exp 
                                    a < 3; 
                                    b > 0;
                                    c <= 7;
                                    d >= 4;
                                    e := 2 + 1 > 3 + 1;
                                    END_PROGRAM",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    {
        let statement = &prg.statements[0];
        let expected_ast = r#"BinaryExpression {
    operator: Less,
    left: Reference {
        name: "a",
    },
    right: LiteralNumber {
        value: "3",
    },
}"#;
        assert_eq!(format!("{:#?}", statement), expected_ast);
    }
    {
        let statement = &prg.statements[1]; // b > 0
        let expected_ast = r#"BinaryExpression {
    operator: Greater,
    left: Reference {
        name: "b",
    },
    right: LiteralNumber {
        value: "0",
    },
}"#;
        assert_eq!(format!("{:#?}", statement), expected_ast);
    }
    {
        let statement = &prg.statements[2]; // c <= 7
        let expected_ast = r#"BinaryExpression {
    operator: LessOrEqual,
    left: Reference {
        name: "c",
    },
    right: LiteralNumber {
        value: "7",
    },
}"#;
        assert_eq!(format!("{:#?}", statement), expected_ast);
    }
    {
        let statement = &prg.statements[3]; // d >= 4
        let expected_ast = r#"BinaryExpression {
    operator: GreaterOrEqual,
    left: Reference {
        name: "d",
    },
    right: LiteralNumber {
        value: "4",
    },
}"#;
        assert_eq!(format!("{:#?}", statement), expected_ast);
    }
    {
        //e := 2 + 1 > 3 + 1;
        let statement = &prg.statements[4];
        let expected_ast = r#"Assignment {
    left: Reference {
        name: "e",
    },
    right: BinaryExpression {
        operator: Greater,
        left: BinaryExpression {
            operator: Plus,
            left: LiteralNumber {
                value: "2",
            },
            right: LiteralNumber {
                value: "1",
            },
        },
        right: BinaryExpression {
            operator: Plus,
            left: LiteralNumber {
                value: "3",
            },
            right: LiteralNumber {
                value: "1",
            },
        },
    },
}"#;
        assert_eq!(format!("{:#?}", statement), expected_ast);
    }
}

#[test]
fn boolean_expression_ast_test() {
    let lexer = lexer::lex("PROGRAM exp a AND NOT b OR c XOR d; END_PROGRAM");
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: And,
    left: Reference {
        name: "a",
    },
    right: BinaryExpression {
        operator: Or,
        left: UnaryExpression {
            operator: Not,
            value: Reference {
                name: "b",
            },
        },
        right: BinaryExpression {
            operator: Xor,
            left: Reference {
                name: "c",
            },
            right: Reference {
                name: "d",
            },
        },
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn boolean_expression_paran_ast_test() {
    let lexer = lexer::lex("PROGRAM exp a AND (NOT (b OR c) XOR d); END_PROGRAM");
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: And,
    left: Reference {
        name: "a",
    },
    right: BinaryExpression {
        operator: Xor,
        left: UnaryExpression {
            operator: Not,
            value: BinaryExpression {
                operator: Or,
                left: Reference {
                    name: "b",
                },
                right: Reference {
                    name: "c",
                },
            },
        },
        right: Reference {
            name: "d",
        },
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn signed_literal_minus_test() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        -1;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"UnaryExpression {
    operator: Minus,
    value: LiteralNumber {
        value: "1",
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn signed_literal_expression_test() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        2 +-x;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Plus,
    left: LiteralNumber {
        value: "2",
    },
    right: UnaryExpression {
        operator: Minus,
        value: Reference {
            name: "x",
        },
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn signed_literal_expression_reversed_test() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        -4 + 5;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Plus,
    left: UnaryExpression {
        operator: Minus,
        value: LiteralNumber {
            value: "4",
        },
    },
    right: LiteralNumber {
        value: "5",
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn or_compare_expressions_priority_test() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        x > 1 OR b1;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Or,
    left: BinaryExpression {
        operator: Greater,
        left: Reference {
            name: "x",
        },
        right: LiteralNumber {
            value: "1",
        },
    },
    right: Reference {
        name: "b1",
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}



#[test]
fn addition_compare_or_priority_test() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        x + 1 > 2 OR b1;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Or,
    left: BinaryExpression {
        operator: Greater,
        left: BinaryExpression {
            operator: Plus
            left: Reference {
                name: "x",
            },
            right: LiteralNumber {
                value: "1",
            },
        },
        right: LiteralNumber {
            value: "2",
        },
    },
    right: Reference {
        name: "b1",
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}


#[test]
fn boolean_priority_test() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        a AND b XOR c OR d;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Or,
    left: BinaryExpression {
        operator: Xor,
        left: BinaryExpression {
            operator: And
            left: Reference {
                name: "a",
            },
            right: Reference {
                name: "b",
            },
        },
        right: Reference {
            value: "c",
        },
    },
    right: Reference {
        name: "d",
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn comparison_priority_test() {
    let lexer = lexer::lex(
        "
        PROGRAM exp 
        x < 7 = y > 6;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap();

    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = 
r#"BinaryExpression {
    operator: Equal,
    left: BinaryExpression {
        operator: Less,
        left: Reference {
            name: "x",
        },
        right: LiteralNumber {
            value: "7",
        },
    },
    right: BinaryExpression {
        operator: Greater,
        left: Reference {
            name: "y",
        },
        right: LiteralNumber {
            value: "6",
        },
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}