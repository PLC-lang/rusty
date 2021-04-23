// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::ast::{Operator, Statement};
use crate::parser::parse;
use pretty_assertions::*;

#[test]
fn single_statement_parsed() {
    let lexer = super::lex("PROGRAM exp x; END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let Statement::Reference { name, .. } = statement {
        assert_eq!(name, "x");
    } else {
        panic!("Expected Reference but found {:?}", statement);
    }
}

#[test]
fn qualified_reference_statement_parsed() {
    let lexer = super::lex("PROGRAM exp a.x; END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let Statement::QualifiedReference { elements, .. } = statement {
        assert_eq!(
            elements,
            &[
                Statement::Reference {
                    name: "a".to_string(),
                    location: (12..13).into()
                },
                Statement::Reference {
                    name: "x".to_string(),
                    location: (14..15).into()
                },
            ]
        );
    } else {
        panic!("Expected Reference but found {:?}", statement);
    }
}

#[test]
fn literal_can_be_parsed() {
    let lexer = super::lex("PROGRAM exp 7; END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let Statement::LiteralInteger { value, location: _ } = statement {
        assert_eq!(value, "7");
    } else {
        panic!("Expected LiteralInteger but found {:?}", statement);
    }
}

#[test]
fn additon_of_two_variables_parsed() {
    let lexer = super::lex("PROGRAM exp x+y; END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let Statement::BinaryExpression {
        operator,
        left,  //Box<Reference> {name : left}),
        right, //Box<Reference> {name : right}),
    } = statement
    {
        if let Statement::Reference { name, .. } = &**left {
            assert_eq!(name, "x");
        }
        if let Statement::Reference { name, .. } = &**right {
            assert_eq!(name, "y");
        }
        assert_eq!(operator, &Operator::Plus);
    } else {
        panic!("Expected Reference but found {:?}", statement);
    }
}

#[test]
fn additon_of_three_variables_parsed() {
    let lexer = super::lex("PROGRAM exp x+y-z; END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let Statement::BinaryExpression {
        operator,
        left,  //Box<Reference> {name : left}),
        right, //Box<Reference> {name : right}),
    } = statement
    {
        assert_eq!(operator, &Operator::Plus);
        if let Statement::Reference { name, .. } = &**left {
            assert_eq!(name, "x");
        }
        if let Statement::BinaryExpression {
            operator,
            left,
            right,
        } = &**right
        {
            if let Statement::Reference { name, .. } = &**left {
                assert_eq!(name, "y");
            }
            if let Statement::Reference { name, .. } = &**right {
                assert_eq!(name, "z");
            }
            assert_eq!(operator, &Operator::Minus);
        } else {
            panic!("Expected Reference but found {:?}", statement);
        }
    } else {
        panic!("Expected Reference but found {:?}", statement);
    }
}

#[test]
fn parenthesis_expressions_should_not_change_the_ast() {
    let lexer = super::lex("PROGRAM exp (x+y); END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let Statement::BinaryExpression {
        operator,
        left,
        right,
    } = statement
    {
        if let Statement::Reference { name, .. } = &**left {
            assert_eq!(name, "x");
        }
        if let Statement::Reference { name, .. } = &**right {
            assert_eq!(name, "y");
        }
        assert_eq!(operator, &Operator::Plus);
    } else {
        panic!("Expected Reference but found {:?}", statement);
    }
}

#[test]
fn multiplication_expressions_parse() {
    let lexer = super::lex("PROGRAM exp 1*2/7; END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Multiplication,
    left: LiteralInteger {
        value: "1",
    },
    right: BinaryExpression {
        operator: Division,
        left: LiteralInteger {
            value: "2",
        },
        right: LiteralInteger {
            value: "7",
        },
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn addition_ast_test() {
    let lexer = super::lex("PROGRAM exp 1+2; END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Plus,
    left: LiteralInteger {
        value: "1",
    },
    right: LiteralInteger {
        value: "2",
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn multiplication_ast_test() {
    let lexer = super::lex("PROGRAM exp 1+2*3; END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Plus,
    left: LiteralInteger {
        value: "1",
    },
    right: BinaryExpression {
        operator: Multiplication,
        left: LiteralInteger {
            value: "2",
        },
        right: LiteralInteger {
            value: "3",
        },
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn term_ast_test() {
    let lexer = super::lex("PROGRAM exp 1+2*3+4; END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Plus,
    left: LiteralInteger {
        value: "1",
    },
    right: BinaryExpression {
        operator: Plus,
        left: BinaryExpression {
            operator: Multiplication,
            left: LiteralInteger {
                value: "2",
            },
            right: LiteralInteger {
                value: "3",
            },
        },
        right: LiteralInteger {
            value: "4",
        },
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn module_expression_test() {
    let lexer = super::lex("PROGRAM exp 5 MOD 2; END_PROGRAM");

    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Modulo,
    left: LiteralInteger {
        value: "5",
    },
    right: LiteralInteger {
        value: "2",
    },
}"#;

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn parenthesized_term_ast_test() {
    let lexer = super::lex("PROGRAM exp (1+2)*(3+4); END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Multiplication,
    left: BinaryExpression {
        operator: Plus,
        left: LiteralInteger {
            value: "1",
        },
        right: LiteralInteger {
            value: "2",
        },
    },
    right: BinaryExpression {
        operator: Plus,
        left: LiteralInteger {
            value: "3",
        },
        right: LiteralInteger {
            value: "4",
        },
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn boolean_literals_can_be_parsed() {
    let lexer = super::lex("PROGRAM exp TRUE OR FALSE; END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
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
    let lexer = super::lex("PROGRAM exp x := 3; x := 1 + 2; END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    {
        let statement = &prg.statements[0];
        let ast_string = format!("{:#?}", statement);
        let expected_ast = r#"Assignment {
    left: Reference {
        name: "x",
    },
    right: LiteralInteger {
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
        left: LiteralInteger {
            value: "1",
        },
        right: LiteralInteger {
            value: "2",
        },
    },
}"#;
        assert_eq!(ast_string, expected_ast);
    }
}

#[test]
fn equality_expression_test() {
    let lexer = super::lex("PROGRAM exp x = 3; x - 0 <> 1 + 2; END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    {
        let statement = &prg.statements[0];
        let ast_string = format!("{:#?}", statement);
        let expected_ast = r#"BinaryExpression {
    operator: Equal,
    left: Reference {
        name: "x",
    },
    right: LiteralInteger {
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
        right: LiteralInteger {
            value: "0",
        },
    },
    right: BinaryExpression {
        operator: Plus,
        left: LiteralInteger {
            value: "1",
        },
        right: LiteralInteger {
            value: "2",
        },
    },
}"#;
        assert_eq!(ast_string, expected_ast);
    }
}
#[test]
fn comparison_expression_test() {
    let lexer = super::lex(
        "PROGRAM exp 
                                    a < 3; 
                                    b > 0;
                                    c <= 7;
                                    d >= 4;
                                    e := 2 + 1 > 3 + 1;
                                    END_PROGRAM",
    );
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    {
        let statement = &prg.statements[0];
        let expected_ast = r#"BinaryExpression {
    operator: Less,
    left: Reference {
        name: "a",
    },
    right: LiteralInteger {
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
    right: LiteralInteger {
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
    right: LiteralInteger {
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
    right: LiteralInteger {
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
            left: LiteralInteger {
                value: "2",
            },
            right: LiteralInteger {
                value: "1",
            },
        },
        right: BinaryExpression {
            operator: Plus,
            left: LiteralInteger {
                value: "3",
            },
            right: LiteralInteger {
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
    let lexer = super::lex("PROGRAM exp a AND NOT b OR c XOR d; END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Or,
    left: BinaryExpression {
        operator: And,
        left: Reference {
            name: "a",
        },
        right: UnaryExpression {
            operator: Not,
            value: Reference {
                name: "b",
            },
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
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn boolean_expression_param_ast_test() {
    let lexer = super::lex("PROGRAM exp a AND (NOT (b OR c) XOR d); END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
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
    let lexer = super::lex(
        "
        PROGRAM exp 
        -1;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"UnaryExpression {
    operator: Minus,
    value: LiteralInteger {
        value: "1",
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn literal_date_test() {
    let lexer = super::lex(
        "
        PROGRAM exp 
            DATE#1984-10-01; 
            D#2021-04-20; 
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap().0;
    let ast_string = format!("{:#?}", &result.implementations[0].statements);
    let expected_ast = r#"[
    LiteralDate {
        year: 1984,
        month: 10,
        day: 1,
    },
    LiteralDate {
        year: 2021,
        month: 4,
        day: 20,
    },
]"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn literal_real_test() {
    let lexer = super::lex(
        "
        PROGRAM exp 
        1.1;
        1.2e3;
        1.2e-4;
        -1.5;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements;

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"[
    LiteralReal {
        value: "1.1",
    },
    LiteralReal {
        value: "1.2e3",
    },
    LiteralReal {
        value: "1.2e-4",
    },
    UnaryExpression {
        operator: Minus,
        value: LiteralReal {
            value: "1.5",
        },
    },
]"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn signed_literal_expression_test() {
    let lexer = super::lex(
        "
        PROGRAM exp 
        2 +-x;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Plus,
    left: LiteralInteger {
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
    let lexer = super::lex(
        "
        PROGRAM exp 
        -4 + 5;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Plus,
    left: UnaryExpression {
        operator: Minus,
        value: LiteralInteger {
            value: "4",
        },
    },
    right: LiteralInteger {
        value: "5",
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn or_compare_expressions_priority_test() {
    let lexer = super::lex(
        "
        PROGRAM exp 
        x > 1 OR b1;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Or,
    left: BinaryExpression {
        operator: Greater,
        left: Reference {
            name: "x",
        },
        right: LiteralInteger {
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
    let lexer = super::lex(
        "
        PROGRAM exp 
        x + 1 > 2 OR b1;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Or,
    left: BinaryExpression {
        operator: Greater,
        left: BinaryExpression {
            operator: Plus,
            left: Reference {
                name: "x",
            },
            right: LiteralInteger {
                value: "1",
            },
        },
        right: LiteralInteger {
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
    let lexer = super::lex(
        "
        PROGRAM exp 
        a AND b XOR c OR d;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Or,
    left: BinaryExpression {
        operator: Xor,
        left: BinaryExpression {
            operator: And,
            left: Reference {
                name: "a",
            },
            right: Reference {
                name: "b",
            },
        },
        right: Reference {
            name: "c",
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
    let lexer = super::lex(
        "
        PROGRAM exp 
        x < 7 = y > 6;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Equal,
    left: BinaryExpression {
        operator: Less,
        left: Reference {
            name: "x",
        },
        right: LiteralInteger {
            value: "7",
        },
    },
    right: BinaryExpression {
        operator: Greater,
        left: Reference {
            name: "y",
        },
        right: LiteralInteger {
            value: "6",
        },
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn expression_list() {
    //technically this is an illegal state, the parser will accept it though
    let lexer = super::lex(
        "
        PROGRAM exp 
        1,2,3;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"ExpressionList {
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
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn expression_list_assignments() {
    //technically this is an illegal state, the parser will accept it though
    let lexer = super::lex(
        "
        PROGRAM exp 
        x := 1, y := 2, z:= 3;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"ExpressionList {
    expressions: [
        Assignment {
            left: Reference {
                name: "x",
            },
            right: LiteralInteger {
                value: "1",
            },
        },
        Assignment {
            left: Reference {
                name: "y",
            },
            right: LiteralInteger {
                value: "2",
            },
        },
        Assignment {
            left: Reference {
                name: "z",
            },
            right: LiteralInteger {
                value: "3",
            },
        },
    ],
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn range_expression() {
    let lexer = super::lex(
        "
        PROGRAM exp 
        a..b;
        1..2;
        a..2;
        2..a;
        -2..-1;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"RangeStatement {
    start: Reference {
        name: "a",
    },
    end: Reference {
        name: "b",
    },
}"#;
    assert_eq!(ast_string, expected_ast);

    let statement = &prg.statements[1];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"RangeStatement {
    start: LiteralInteger {
        value: "1",
    },
    end: LiteralInteger {
        value: "2",
    },
}"#;
    assert_eq!(ast_string, expected_ast);

    let statement = &prg.statements[2];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"RangeStatement {
    start: Reference {
        name: "a",
    },
    end: LiteralInteger {
        value: "2",
    },
}"#;
    assert_eq!(ast_string, expected_ast);

    let statement = &prg.statements[3];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"RangeStatement {
    start: LiteralInteger {
        value: "2",
    },
    end: Reference {
        name: "a",
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn negative_range_expression() {
    let lexer = super::lex(
        "
        PROGRAM exp 
        -2..-1;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"RangeStatement {
    start: UnaryExpression {
        operator: Minus,
        value: LiteralInteger {
            value: "2",
        },
    },
    end: UnaryExpression {
        operator: Minus,
        value: LiteralInteger {
            value: "1",
        },
    },
}"#;

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn negative_range_expression_space() {
    let lexer = super::lex(
        "
        PROGRAM exp 
        -2 ..-1;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"RangeStatement {
    start: UnaryExpression {
        operator: Minus,
        value: LiteralInteger {
            value: "2",
        },
    },
    end: UnaryExpression {
        operator: Minus,
        value: LiteralInteger {
            value: "1",
        },
    },
}"#;

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn range_expression2() {
    let lexer = super::lex(
        "
        PROGRAM exp 
        1 .. 2;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"RangeStatement {
    start: LiteralInteger {
        value: "1",
    },
    end: LiteralInteger {
        value: "2",
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn function_call_no_params() {
    let lexer = super::lex(
        "
    PROGRAM prg
    fn();
    END_PROGRAM
    ",
    );
    let parse_result = parse(lexer).unwrap().0;

    let statement = &parse_result.implementations[0].statements[0];

    let ast_string = format!("{:#?}", statement);

    let expected_ast = r#"CallStatement {
    operator: Reference {
        name: "fn",
    },
    parameters: None,
}"#;

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn function_call_params() {
    let lexer = super::lex(
        "
    PROGRAM prg
    fn(1,2,3);
    END_PROGRAM
    ",
    );
    let parse_result = parse(lexer).unwrap().0;

    let statement = &parse_result.implementations[0].statements[0];

    let ast_string = format!("{:#?}", statement);

    let expected_ast = r#"CallStatement {
    operator: Reference {
        name: "fn",
    },
    parameters: Some(
        ExpressionList {
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
    ),
}"#;

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn string_can_be_parsed() {
    let lexer = super::lex(
        "PROGRAM buz VAR x : STRING; END_VAR x := 'Hello, World!'; x := ''; END_PROGRAM",
    );
    let result = parse(lexer).unwrap().0;

    let unit = &result.units[0];
    let prg = &result.implementations[0];
    let variable_block = &unit.variable_blocks[0];
    let ast_string = format!("{:#?}", variable_block);
    let expected_ast = r#"VariableBlock {
    variables: [
        Variable {
            name: "x",
            data_type: DataTypeDefinition {
                data_type: StringType {
                    name: None,
                    is_wide: false,
                    size: None,
                },
            },
        },
    ],
    variable_block_type: Local,
}"#;
    assert_eq!(ast_string, expected_ast);

    let statements = &prg.statements;
    let ast_string = format!("{:#?}", statements[0]);
    let expected_ast = r#"Assignment {
    left: Reference {
        name: "x",
    },
    right: LiteralString {
        value: "Hello, World!",
    },
}"#;
    assert_eq!(ast_string, expected_ast);

    let ast_string = format!("{:#?}", statements[1]);
    let expected_ast = r#"Assignment {
    left: Reference {
        name: "x",
    },
    right: LiteralString {
        value: "",
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn arrays_can_be_parsed() {
    let lexer = super::lex(
        "PROGRAM buz VAR x : ARRAY[0..9] OF STRING; END_VAR x[0] := 'Hello, World!'; x[y] := ''; END_PROGRAM",
    );
    let result = parse(lexer).unwrap().0;

    let unit = &result.units[0];
    let prg = &result.implementations[0];
    let variable_block = &unit.variable_blocks[0];
    let ast_string = format!("{:#?}", variable_block);
    let expected_ast = r#"VariableBlock {
    variables: [
        Variable {
            name: "x",
            data_type: DataTypeDefinition {
                data_type: ArrayType {
                    name: None,
                    bounds: RangeStatement {
                        start: LiteralInteger {
                            value: "0",
                        },
                        end: LiteralInteger {
                            value: "9",
                        },
                    },
                    referenced_type: DataTypeDefinition {
                        data_type: StringType {
                            name: None,
                            is_wide: false,
                            size: None,
                        },
                    },
                },
            },
        },
    ],
    variable_block_type: Local,
}"#;
    assert_eq!(ast_string, expected_ast);

    let statements = &prg.statements;
    let ast_string = format!("{:#?}", statements[0]);
    let expected_ast = r#"Assignment {
    left: ArrayAccess {
        reference: Reference {
            name: "x",
        },
        access: LiteralInteger {
            value: "0",
        },
    },
    right: LiteralString {
        value: "Hello, World!",
    },
}"#;
    assert_eq!(ast_string, expected_ast);

    let ast_string = format!("{:#?}", statements[1]);
    let expected_ast = r#"Assignment {
    left: ArrayAccess {
        reference: Reference {
            name: "x",
        },
        access: Reference {
            name: "y",
        },
    },
    right: LiteralString {
        value: "",
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn nested_arrays_can_be_parsed() {
    let lexer = super::lex(
        "PROGRAM buz VAR x : ARRAY[0..9] OF ARRAY[0..9] OF STRING; END_VAR x[0][1] := 'Hello, World!'; x[y][1] := ''; END_PROGRAM",
    );
    let result = parse(lexer).unwrap().0;

    let unit = &result.units[0];
    let prg = &result.implementations[0];
    let variable_block = &unit.variable_blocks[0];
    let ast_string = format!("{:#?}", variable_block);
    let expected_ast = r#"VariableBlock {
    variables: [
        Variable {
            name: "x",
            data_type: DataTypeDefinition {
                data_type: ArrayType {
                    name: None,
                    bounds: RangeStatement {
                        start: LiteralInteger {
                            value: "0",
                        },
                        end: LiteralInteger {
                            value: "9",
                        },
                    },
                    referenced_type: DataTypeDefinition {
                        data_type: ArrayType {
                            name: None,
                            bounds: RangeStatement {
                                start: LiteralInteger {
                                    value: "0",
                                },
                                end: LiteralInteger {
                                    value: "9",
                                },
                            },
                            referenced_type: DataTypeDefinition {
                                data_type: StringType {
                                    name: None,
                                    is_wide: false,
                                    size: None,
                                },
                            },
                        },
                    },
                },
            },
        },
    ],
    variable_block_type: Local,
}"#;
    assert_eq!(ast_string, expected_ast);

    let statements = &prg.statements;
    let ast_string = format!("{:#?}", statements[0]);
    let expected_ast = r#"Assignment {
    left: ArrayAccess {
        reference: ArrayAccess {
            reference: Reference {
                name: "x",
            },
            access: LiteralInteger {
                value: "0",
            },
        },
        access: LiteralInteger {
            value: "1",
        },
    },
    right: LiteralString {
        value: "Hello, World!",
    },
}"#;
    assert_eq!(ast_string, expected_ast);

    let ast_string = format!("{:#?}", statements[1]);
    let expected_ast = r#"Assignment {
    left: ArrayAccess {
        reference: ArrayAccess {
            reference: Reference {
                name: "x",
            },
            access: Reference {
                name: "y",
            },
        },
        access: LiteralInteger {
            value: "1",
        },
    },
    right: LiteralString {
        value: "",
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn multidim_arrays_can_be_parsed() {
    let lexer = super::lex(
        "PROGRAM buz VAR x : ARRAY[0..9,1..2] OF STRING; END_VAR x[0,1] := 'Hello, World!'; x[y,1] := ''; END_PROGRAM",
    );
    let result = parse(lexer).unwrap().0;

    let unit = &result.units[0];
    let prg = &result.implementations[0];
    let variable_block = &unit.variable_blocks[0];
    let ast_string = format!("{:#?}", variable_block);
    let expected_ast = r#"VariableBlock {
    variables: [
        Variable {
            name: "x",
            data_type: DataTypeDefinition {
                data_type: ArrayType {
                    name: None,
                    bounds: ExpressionList {
                        expressions: [
                            RangeStatement {
                                start: LiteralInteger {
                                    value: "0",
                                },
                                end: LiteralInteger {
                                    value: "9",
                                },
                            },
                            RangeStatement {
                                start: LiteralInteger {
                                    value: "1",
                                },
                                end: LiteralInteger {
                                    value: "2",
                                },
                            },
                        ],
                    },
                    referenced_type: DataTypeDefinition {
                        data_type: StringType {
                            name: None,
                            is_wide: false,
                            size: None,
                        },
                    },
                },
            },
        },
    ],
    variable_block_type: Local,
}"#;
    assert_eq!(ast_string, expected_ast);

    let statements = &prg.statements;
    let ast_string = format!("{:#?}", statements[0]);
    let expected_ast = r#"Assignment {
    left: ArrayAccess {
        reference: Reference {
            name: "x",
        },
        access: ExpressionList {
            expressions: [
                LiteralInteger {
                    value: "0",
                },
                LiteralInteger {
                    value: "1",
                },
            ],
        },
    },
    right: LiteralString {
        value: "Hello, World!",
    },
}"#;
    assert_eq!(ast_string, expected_ast);

    let ast_string = format!("{:#?}", statements[1]);
    let expected_ast = r#"Assignment {
    left: ArrayAccess {
        reference: Reference {
            name: "x",
        },
        access: ExpressionList {
            expressions: [
                Reference {
                    name: "y",
                },
                LiteralInteger {
                    value: "1",
                },
            ],
        },
    },
    right: LiteralString {
        value: "",
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn arrays_in_structs_can_be_parsed() {
    let lexer = super::lex(
        "
        PROGRAM buz VAR x : MyStructWithArray; END_VAR x.y[7]; END_PROGRAM",
    );
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"QualifiedReference {
    elements: [
        Reference {
            name: "x",
        },
        ArrayAccess {
            reference: Reference {
                name: "y",
            },
            access: LiteralInteger {
                value: "7",
            },
        },
    ],
}"#;

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn arrays_of_structs_can_be_parsed() {
    let lexer = super::lex(
        "
        PROGRAM buz VAR x : ARRAY[0..1] OF MyStruct; END_VAR x[1].y; END_PROGRAM",
    );
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"QualifiedReference {
    elements: [
        ArrayAccess {
            reference: Reference {
                name: "x",
            },
            access: LiteralInteger {
                value: "1",
            },
        },
        Reference {
            name: "y",
        },
    ],
}"#;

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn function_call_formal_params() {
    let lexer = super::lex(
        "
    PROGRAM prg
    fn(x := 1,y := 2,z => a);
    END_PROGRAM
    ",
    );
    let parse_result = parse(lexer).unwrap().0;

    let statement = &parse_result.implementations[0].statements[0];

    let ast_string = format!("{:#?}", statement);

    let expected_ast = r#"CallStatement {
    operator: Reference {
        name: "fn",
    },
    parameters: Some(
        ExpressionList {
            expressions: [
                Assignment {
                    left: Reference {
                        name: "x",
                    },
                    right: LiteralInteger {
                        value: "1",
                    },
                },
                Assignment {
                    left: Reference {
                        name: "y",
                    },
                    right: LiteralInteger {
                        value: "2",
                    },
                },
                OutputAssignment {
                    left: Reference {
                        name: "z",
                    },
                    right: Reference {
                        name: "a",
                    },
                },
            ],
        },
    ),
}"#;

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn function_call_return_params() {
    let lexer = super::lex(
        "
    PROGRAM prg
    x := fn(1,2,3);
    END_PROGRAM
    ",
    );
    let parse_result = parse(lexer).unwrap().0;

    let statement = &parse_result.implementations[0].statements[0];

    let ast_string = format!("{:#?}", statement);

    let expected_ast = r#"Assignment {
    left: Reference {
        name: "x",
    },
    right: CallStatement {
        operator: Reference {
            name: "fn",
        },
        parameters: Some(
            ExpressionList {
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
        ),
    },
}"#;

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn literals_location_test() {
    let source = "PROGRAM prg 7; 'hello'; TRUE; 3.1415; END_PROGRAM";
    let lexer = super::lex(source);
    let parse_result = parse(lexer).unwrap().0;

    let unit = &parse_result.implementations[0];

    // 1
    let location = &unit.statements[0].get_location();
    assert_eq!(location, &(12..13).into());
    assert_eq!(
        source[location.get_start()..location.get_end()].to_string(),
        "7"
    );

    // 'hello'
    let location = &unit.statements[1].get_location();
    assert_eq!(location, &(15..22).into());
    assert_eq!(
        source[location.get_start()..location.get_end()].to_string(),
        "'hello'"
    );

    // true
    let location = &unit.statements[2].get_location();
    assert_eq!(location, &(24..28).into());
    assert_eq!(
        source[location.get_start()..location.get_end()].to_string(),
        "TRUE"
    );

    //3.1415
    let location = &unit.statements[3].get_location();
    assert_eq!(location, &(30..36).into());
    assert_eq!(
        source[location.get_start()..location.get_end()].to_string(),
        "3.1415"
    )
}

#[test]
fn reference_location_test() {
    let source = "PROGRAM prg a;bb;ccc; END_PROGRAM";
    let lexer = super::lex(source);
    let parse_result = parse(lexer).unwrap().0;

    let unit = &parse_result.implementations[0];

    let location = &unit.statements[0].get_location();
    assert_eq!(
        source[location.get_start()..location.get_end()].to_string(),
        "a"
    );

    let location = &unit.statements[1].get_location();
    assert_eq!(
        source[location.get_start()..location.get_end()].to_string(),
        "bb"
    );

    let location = &unit.statements[2].get_location();
    assert_eq!(
        source[location.get_start()..location.get_end()].to_string(),
        "ccc"
    );
}

#[test]
fn expressions_location_test() {
    let source = "
    PROGRAM prg 
        a + b;
        x + z - y + u - v;
        -x;
        1..3;
        a := a + 4;
    END_PROGRAM";
    let lexer = super::lex(source);
    let parse_result = parse(lexer).unwrap().0;

    let unit = &parse_result.implementations[0];

    let location = &unit.statements[0].get_location();
    assert_eq!(
        source[location.get_start()..location.get_end()].to_string(),
        "a + b"
    );

    let location = &unit.statements[1].get_location();
    assert_eq!(
        source[location.get_start()..location.get_end()].to_string(),
        "x + z - y + u - v"
    );

    let location = &unit.statements[2].get_location();
    assert_eq!(
        source[location.get_start()..location.get_end()].to_string(),
        "-x"
    );

    let location = &unit.statements[3].get_location();
    assert_eq!(
        source[location.get_start()..location.get_end()].to_string(),
        "1..3"
    );

    let location = &unit.statements[4].get_location();
    assert_eq!(
        source[location.get_start()..location.get_end()].to_string(),
        "a := a + 4"
    );
}
