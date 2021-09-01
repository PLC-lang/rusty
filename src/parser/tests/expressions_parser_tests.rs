// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::ast::{
    AstStatement, DataType, DataTypeDeclaration, DirectAccess, Operator, Pou, SourceRange,
};
use crate::parser::parse;
use crate::parser::tests::{literal_int, ref_to};
use pretty_assertions::*;

#[test]
fn single_statement_parsed() {
    let lexer = super::lex("PROGRAM exp x; END_PROGRAM");
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let AstStatement::Reference { name, .. } = statement {
        assert_eq!(name, "x");
    } else {
        panic!("Expected Reference but found {:?}", statement);
    }
}

#[test]
fn qualified_reference_statement_parsed() {
    let lexer = super::lex("PROGRAM exp a.x; END_PROGRAM");
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let AstStatement::QualifiedReference { elements, .. } = statement {
        assert_eq!(
            format!("{:?}", elements),
            format!(
                "{:?}",
                &[
                    AstStatement::Reference {
                        name: "a".to_string(),
                        location: (12..13).into(),
                        id: 0
                    },
                    AstStatement::Reference {
                        name: "x".to_string(),
                        location: (14..15).into(),
                        id: 0
                    },
                ]
            )
        );
    } else {
        panic!("Expected Reference but found {:?}", statement);
    }
}
#[test]
fn bitwise_access_parsed() {
    let lexer = super::lex(
        "PROGRAM exp 
    a.0; 
    a.%X1; 
    a.%B1; 
    a[0].%W1; 
    a.b.%D1; 
    END_PROGRAM",
    );
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements;
    let expected = vec![
        AstStatement::QualifiedReference {
            elements: vec![
                ref_to("a"),
                AstStatement::DirectAccess {
                    access: DirectAccess::Bit,
                    index: 0,
                    location: SourceRange::undefined(),
                    id: 0,
                },
            ],
            id: 0,
        },
        AstStatement::QualifiedReference {
            elements: vec![
                ref_to("a"),
                AstStatement::DirectAccess {
                    access: DirectAccess::Bit,
                    index: 1,
                    location: SourceRange::undefined(),
                    id: 0,
                },
            ],
            id: 0,
        },
        AstStatement::QualifiedReference {
            elements: vec![
                ref_to("a"),
                AstStatement::DirectAccess {
                    access: DirectAccess::Byte,
                    index: 1,
                    location: SourceRange::undefined(),
                    id: 0,
                },
            ],
            id: 0,
        },
        AstStatement::QualifiedReference {
            elements: vec![
                AstStatement::ArrayAccess {
                    access: Box::new(literal_int(0)),
                    reference: Box::new(ref_to("a")),
                    id: 0,
                },
                AstStatement::DirectAccess {
                    access: DirectAccess::Word,
                    index: 1,
                    location: SourceRange::undefined(),
                    id: 0,
                },
            ],
            id: 0,
        },
        AstStatement::QualifiedReference {
            elements: vec![
                ref_to("a"),
                ref_to("b"),
                AstStatement::DirectAccess {
                    access: DirectAccess::DWord,
                    index: 1,
                    location: SourceRange::undefined(),
                    id: 0,
                },
            ],
            id: 0,
        },
    ];

    assert_eq!(format!("{:?}", expected), format!("{:?}", statement));
}

#[test]
fn literal_can_be_parsed() {
    let lexer = super::lex("PROGRAM exp 7; END_PROGRAM");
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let AstStatement::LiteralInteger { value, .. } = statement {
        assert_eq!(value, &7_i64);
    } else {
        panic!("Expected LiteralInteger but found {:?}", statement);
    }
}

#[test]
fn literal_binary_with_underscore_number_can_be_parsed() {
    let lexer = super::lex("PROGRAM exp 2#101_101; END_PROGRAM");
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let AstStatement::LiteralInteger { value, .. } = statement {
        assert_eq!(value, &45_i64);
    } else {
        panic!("Expected LiteralInteger but found {:?}", statement);
    }
}

#[test]
fn literal_hex_number_with_underscores_can_be_parsed() {
    let lexer = super::lex("PROGRAM exp 16#DE_AD_be_ef; END_PROGRAM");
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let AstStatement::LiteralInteger { value, .. } = statement {
        assert_eq!(value, &3735928559_i64);
    } else {
        panic!("Expected LiteralInteger but found {:?}", statement);
    }
}

#[test]
fn literal_hex_number_can_be_parsed() {
    let lexer = super::lex("PROGRAM exp 16#DEADbeef; END_PROGRAM");
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let AstStatement::LiteralInteger { value, .. } = statement {
        assert_eq!(value, &3735928559_i64);
    } else {
        panic!("Expected LiteralInteger but found {:?}", statement);
    }
}

#[test]
fn literal_oct_number_with_underscores_can_be_parsed() {
    let lexer = super::lex("PROGRAM exp 8#7_7; END_PROGRAM");
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let AstStatement::LiteralInteger { value, .. } = statement {
        assert_eq!(value, &63_i64);
    } else {
        panic!("Expected LiteralInteger but found {:?}", statement);
    }
}

#[test]
fn literal_dec_number_with_underscores_can_be_parsed() {
    let lexer = super::lex("PROGRAM exp 43_000; END_PROGRAM");
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let AstStatement::LiteralInteger { value, .. } = statement {
        assert_eq!(value, &43000_i64);
    } else {
        panic!("Expected LiteralInteger but found {:?}", statement);
    }
}

#[test]
fn literal_oct_number_with_underscore_can_be_parsed() {
    let lexer = super::lex("PROGRAM exp 8#7_7; END_PROGRAM");
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let AstStatement::LiteralInteger { value, .. } = statement {
        assert_eq!(value, &63_i64);
    } else {
        panic!("Expected LiteralInteger but found {:?}", statement);
    }
}

#[test]
fn additon_of_two_variables_parsed() {
    let lexer = super::lex("PROGRAM exp x+y; END_PROGRAM");
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let AstStatement::BinaryExpression {
        operator,
        left,  //Box<Reference> {name : left}),
        right, //Box<Reference> {name : right}),
        ..
    } = statement
    {
        if let AstStatement::Reference { name, .. } = &**left {
            assert_eq!(name, "x");
        }
        if let AstStatement::Reference { name, .. } = &**right {
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
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let AstStatement::BinaryExpression {
        operator,
        left,  //Box<Reference> {name : left}),
        right, //Box<Reference> {name : right}),
        ..
    } = statement
    {
        assert_eq!(operator, &Operator::Plus);
        if let AstStatement::Reference { name, .. } = &**left {
            assert_eq!(name, "x");
        }
        if let AstStatement::BinaryExpression {
            operator,
            left,
            right,
            ..
        } = &**right
        {
            if let AstStatement::Reference { name, .. } = &**left {
                assert_eq!(name, "y");
            }
            if let AstStatement::Reference { name, .. } = &**right {
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
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    if let AstStatement::BinaryExpression {
        operator,
        left,
        right,
        ..
    } = statement
    {
        if let AstStatement::Reference { name, .. } = &**left {
            assert_eq!(name, "x");
        }
        if let AstStatement::Reference { name, .. } = &**right {
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
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Multiplication,
    left: LiteralInteger {
        value: 1,
    },
    right: BinaryExpression {
        operator: Division,
        left: LiteralInteger {
            value: 2,
        },
        right: LiteralInteger {
            value: 7,
        },
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn addition_ast_test() {
    let lexer = super::lex("PROGRAM exp 1+2; END_PROGRAM");
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Plus,
    left: LiteralInteger {
        value: 1,
    },
    right: LiteralInteger {
        value: 2,
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn multiplication_ast_test() {
    let lexer = super::lex("PROGRAM exp 1+2*3; END_PROGRAM");
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Plus,
    left: LiteralInteger {
        value: 1,
    },
    right: BinaryExpression {
        operator: Multiplication,
        left: LiteralInteger {
            value: 2,
        },
        right: LiteralInteger {
            value: 3,
        },
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn term_ast_test() {
    let lexer = super::lex("PROGRAM exp 1+2*3+4; END_PROGRAM");
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Plus,
    left: LiteralInteger {
        value: 1,
    },
    right: BinaryExpression {
        operator: Plus,
        left: BinaryExpression {
            operator: Multiplication,
            left: LiteralInteger {
                value: 2,
            },
            right: LiteralInteger {
                value: 3,
            },
        },
        right: LiteralInteger {
            value: 4,
        },
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn module_expression_test() {
    let lexer = super::lex("PROGRAM exp 5 MOD 2; END_PROGRAM");

    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Modulo,
    left: LiteralInteger {
        value: 5,
    },
    right: LiteralInteger {
        value: 2,
    },
}"#;

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn parenthesized_term_ast_test() {
    let lexer = super::lex("PROGRAM exp (1+2)*(3+4); END_PROGRAM");
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Multiplication,
    left: BinaryExpression {
        operator: Plus,
        left: LiteralInteger {
            value: 1,
        },
        right: LiteralInteger {
            value: 2,
        },
    },
    right: BinaryExpression {
        operator: Plus,
        left: LiteralInteger {
            value: 3,
        },
        right: LiteralInteger {
            value: 4,
        },
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn boolean_literals_can_be_parsed() {
    let lexer = super::lex("PROGRAM exp TRUE OR FALSE; END_PROGRAM");
    let result = parse(lexer).0;

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
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    {
        let statement = &prg.statements[0];
        let ast_string = format!("{:#?}", statement);
        let expected_ast = r#"Assignment {
    left: Reference {
        name: "x",
    },
    right: LiteralInteger {
        value: 3,
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
            value: 1,
        },
        right: LiteralInteger {
            value: 2,
        },
    },
}"#;
        assert_eq!(ast_string, expected_ast);
    }
}

#[test]
fn equality_expression_test() {
    let lexer = super::lex("PROGRAM exp x = 3; x - 0 <> 1 + 2; END_PROGRAM");
    let result = parse(lexer).0;

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
        value: 3,
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
            value: 0,
        },
    },
    right: BinaryExpression {
        operator: Plus,
        left: LiteralInteger {
            value: 1,
        },
        right: LiteralInteger {
            value: 2,
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
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    {
        let statement = &prg.statements[0];
        let expected_ast = r#"BinaryExpression {
    operator: Less,
    left: Reference {
        name: "a",
    },
    right: LiteralInteger {
        value: 3,
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
        value: 0,
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
        value: 7,
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
        value: 4,
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
                value: 2,
            },
            right: LiteralInteger {
                value: 1,
            },
        },
        right: BinaryExpression {
            operator: Plus,
            left: LiteralInteger {
                value: 3,
            },
            right: LiteralInteger {
                value: 1,
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
    let result = parse(lexer).0;

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
    let result = parse(lexer).0;

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
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"LiteralInteger {
    value: -1,
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
    let result = parse(lexer).0;
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
fn literal_time_test() {
    let lexer = super::lex(
        "
        PROGRAM exp 
            T#12d;
            T#12.4d;
            TIME#-12m;
            TIME#12s;
            T#12ms;
            T#12d10ms;
            T#-12h10.6m;
            TIME#12m4s;
            TIME#4d6h8m7s12ms4us8ns;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).0;
    let ast_string = format!("{:#?}", &result.implementations[0].statements);
    let expected_ast = r#"[
    LiteralTime {
        day: 12.0,
        hour: 0.0,
        min: 0.0,
        sec: 0.0,
        milli: 0.0,
        micro: 0.0,
        nano: 0,
        negative: false,
    },
    LiteralTime {
        day: 12.4,
        hour: 0.0,
        min: 0.0,
        sec: 0.0,
        milli: 0.0,
        micro: 0.0,
        nano: 0,
        negative: false,
    },
    LiteralTime {
        day: 0.0,
        hour: 0.0,
        min: 12.0,
        sec: 0.0,
        milli: 0.0,
        micro: 0.0,
        nano: 0,
        negative: true,
    },
    LiteralTime {
        day: 0.0,
        hour: 0.0,
        min: 0.0,
        sec: 12.0,
        milli: 0.0,
        micro: 0.0,
        nano: 0,
        negative: false,
    },
    LiteralTime {
        day: 0.0,
        hour: 0.0,
        min: 0.0,
        sec: 0.0,
        milli: 12.0,
        micro: 0.0,
        nano: 0,
        negative: false,
    },
    LiteralTime {
        day: 12.0,
        hour: 0.0,
        min: 0.0,
        sec: 0.0,
        milli: 10.0,
        micro: 0.0,
        nano: 0,
        negative: false,
    },
    LiteralTime {
        day: 0.0,
        hour: 12.0,
        min: 10.6,
        sec: 0.0,
        milli: 0.0,
        micro: 0.0,
        nano: 0,
        negative: true,
    },
    LiteralTime {
        day: 0.0,
        hour: 0.0,
        min: 12.0,
        sec: 4.0,
        milli: 0.0,
        micro: 0.0,
        nano: 0,
        negative: false,
    },
    LiteralTime {
        day: 4.0,
        hour: 6.0,
        min: 8.0,
        sec: 7.0,
        milli: 12.0,
        micro: 4.0,
        nano: 8,
        negative: false,
    },
]"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn literal_time_of_day_test() {
    let lexer = super::lex(
        "
        PROGRAM exp 
            TOD#12:00:00;
            TOD#00:12:00;
            TOD#00:00:12;
            TIME_OF_DAY#04:16:22;
            TIME_OF_DAY#04:16:22.1;
            TIME_OF_DAY#04:16:22.001;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).0;
    let ast_string = format!("{:#?}", &result.implementations[0].statements);
    let expected_ast = r#"[
    LiteralTimeOfDay {
        hour: 12,
        min: 0,
        sec: 0,
        milli: 0,
    },
    LiteralTimeOfDay {
        hour: 0,
        min: 12,
        sec: 0,
        milli: 0,
    },
    LiteralTimeOfDay {
        hour: 0,
        min: 0,
        sec: 12,
        milli: 0,
    },
    LiteralTimeOfDay {
        hour: 4,
        min: 16,
        sec: 22,
        milli: 0,
    },
    LiteralTimeOfDay {
        hour: 4,
        min: 16,
        sec: 22,
        milli: 100,
    },
    LiteralTimeOfDay {
        hour: 4,
        min: 16,
        sec: 22,
        milli: 1,
    },
]"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn literal_date_and_time_test() {
    let lexer = super::lex(
        "
        PROGRAM exp 
            DATE_AND_TIME#1984-10-01-16:40:22; 
            DT#2021-04-20-22:33:14; 
            DT#2021-04-20-22:33:14.999; 
        END_PROGRAM
        ",
    );
    let result = parse(lexer).0;
    let ast_string = format!("{:#?}", &result.implementations[0].statements);
    let expected_ast = r#"[
    LiteralDateAndTime {
        year: 1984,
        month: 10,
        day: 1,
        hour: 16,
        min: 40,
        sec: 22,
        milli: 0,
    },
    LiteralDateAndTime {
        year: 2021,
        month: 4,
        day: 20,
        hour: 22,
        min: 33,
        sec: 14,
        milli: 0,
    },
    LiteralDateAndTime {
        year: 2021,
        month: 4,
        day: 20,
        hour: 22,
        min: 33,
        sec: 14,
        milli: 999,
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
    let result = parse(lexer).0;

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

fn literal_int_cast(data_type: &str, value: i64) -> AstStatement {
    AstStatement::CastStatement {
        id: 0,
        location: SourceRange::undefined(),
        target: Box::new(AstStatement::LiteralInteger {
            id: 0,
            location: (0..0).into(),
            value,
        }),
        type_name: data_type.to_string(),
    }
}

#[test]
fn literal_enum_parse_test() {
    let lexer = super::lex(
        r#"
        PROGRAM exp 
            MyEnum#Val1;
            MyEnum#Val2;
            MyEnum#Val3;
        END_PROGRAM
        "#,
    );
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements;

    let ast_string = format!("{:#?}", statement);
    assert_eq!(
        ast_string,
        format!(
            "{:#?}",
            vec![
                AstStatement::CastStatement {
                    id: 0,
                    location: (0..0).into(),
                    type_name: "MyEnum".into(),
                    target: Box::new(ref_to("Val1"))
                },
                AstStatement::CastStatement {
                    id: 0,
                    location: (0..0).into(),
                    type_name: "MyEnum".into(),
                    target: Box::new(ref_to("Val2"))
                },
                AstStatement::CastStatement {
                    id: 0,
                    location: (0..0).into(),
                    type_name: "MyEnum".into(),
                    target: Box::new(ref_to("Val3"))
                }
            ]
        )
    );
}

#[test]
fn literal_cast_parse_test() {
    let lexer = super::lex(
        r#"
        PROGRAM exp 
            SINT#100;
            DINT#16#AFFE;
            BYTE#8#77;
            WORD#2#1010;
            INT#100;
            DINT#-100;
            REAL#-3.1415;
            BOOL#1;
            BOOL#FALSE;
            STRING#"abc";
            WSTRING#'xyz';
        END_PROGRAM
        "#,
    );
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements;

    let ast_string = format!("{:#?}", statement);
    assert_eq!(
        ast_string,
        format!(
            "{:#?}",
            vec![
                literal_int_cast("SINT", 100),
                literal_int_cast("DINT", 45054),
                literal_int_cast("BYTE", 63),
                literal_int_cast("WORD", 10),
                literal_int_cast("INT", 100),
                literal_int_cast("DINT", -100),
                AstStatement::CastStatement {
                    id: 0,
                    location: (0..0).into(),
                    type_name: "REAL".into(),
                    target: Box::new(AstStatement::LiteralReal {
                        id: 0,
                        location: (0..0).into(),
                        value: "-3.1415".to_string()
                    })
                },
                AstStatement::CastStatement {
                    id: 0,
                    location: (0..0).into(),
                    type_name: "BOOL".into(),
                    target: Box::new(AstStatement::LiteralInteger {
                        id: 0,
                        location: (0..0).into(),
                        value: 1,
                    })
                },
                AstStatement::CastStatement {
                    id: 0,
                    location: (0..0).into(),
                    type_name: "BOOL".into(),
                    target: Box::new(AstStatement::LiteralBool {
                        id: 0,
                        location: (0..0).into(),
                        value: false
                    })
                },
                AstStatement::CastStatement {
                    id: 0,
                    location: (0..0).into(),
                    type_name: "STRING".into(),
                    target: Box::new(AstStatement::LiteralString {
                        id: 0,
                        location: (0..0).into(),
                        value: "abc".to_string(),
                        is_wide: true,
                    })
                },
                AstStatement::CastStatement {
                    id: 0,
                    location: (0..0).into(),
                    type_name: "WSTRING".into(),
                    target: Box::new(AstStatement::LiteralString {
                        id: 0,
                        location: (0..0).into(),
                        value: "xyz".to_string(),
                        is_wide: false,
                    })
                },
            ]
        )
    );
}

#[test]
fn literal_exponents_test() {
    let lexer = super::lex(
        "
        PROGRAM exp 
        1_2e3;
        12e3;
        12.0e3;
        12e-4;
        1_2e-4;
        12.0e-4;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements;

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"[
    LiteralReal {
        value: "12e3",
    },
    LiteralReal {
        value: "12e3",
    },
    LiteralReal {
        value: "12.0e3",
    },
    LiteralReal {
        value: "12e-4",
    },
    LiteralReal {
        value: "12e-4",
    },
    LiteralReal {
        value: "12.0e-4",
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
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Plus,
    left: LiteralInteger {
        value: 2,
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
fn assignment_to_null() {
    let lexer = super::lex(
        "
        PROGRAM exp 
        x := NULL;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"Assignment {
    left: Reference {
        name: "x",
    },
    right: LiteralNull,
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn pointer_address_test() {
    let lexer = super::lex(
        "
        PROGRAM exp 
        &x;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"UnaryExpression {
    operator: Address,
    value: Reference {
        name: "x",
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn pointer_dereference_test() {
    let lexer = super::lex(
        "
        PROGRAM exp 
        x^;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"PointerAccess {
    reference: Reference {
        name: "x",
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn pointer_dereference_test_nested() {
    let lexer = super::lex(
        "
        PROGRAM exp 
        x^^[0][1]^[2]^^;
        END_PROGRAM
        ",
    );
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"PointerAccess {
    reference: PointerAccess {
        reference: ArrayAccess {
            reference: PointerAccess {
                reference: ArrayAccess {
                    reference: ArrayAccess {
                        reference: PointerAccess {
                            reference: PointerAccess {
                                reference: Reference {
                                    name: "x",
                                },
                            },
                        },
                        access: LiteralInteger {
                            value: 0,
                        },
                    },
                    access: LiteralInteger {
                        value: 1,
                    },
                },
            },
            access: LiteralInteger {
                value: 2,
            },
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
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"BinaryExpression {
    operator: Plus,
    left: LiteralInteger {
        value: -4,
    },
    right: LiteralInteger {
        value: 5,
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
    let result = parse(lexer).0;

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
            value: 1,
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
    let result = parse(lexer).0;

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
                value: 1,
            },
        },
        right: LiteralInteger {
            value: 2,
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
    let result = parse(lexer).0;

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
    let result = parse(lexer).0;

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
            value: 7,
        },
    },
    right: BinaryExpression {
        operator: Greater,
        left: Reference {
            name: "y",
        },
        right: LiteralInteger {
            value: 6,
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
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"ExpressionList {
    expressions: [
        LiteralInteger {
            value: 1,
        },
        LiteralInteger {
            value: 2,
        },
        LiteralInteger {
            value: 3,
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
    let result = parse(lexer).0;

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
                value: 1,
            },
        },
        Assignment {
            left: Reference {
                name: "y",
            },
            right: LiteralInteger {
                value: 2,
            },
        },
        Assignment {
            left: Reference {
                name: "z",
            },
            right: LiteralInteger {
                value: 3,
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
    let result = parse(lexer).0;

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
        value: 1,
    },
    end: LiteralInteger {
        value: 2,
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
        value: 2,
    },
}"#;
    assert_eq!(ast_string, expected_ast);

    let statement = &prg.statements[3];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"RangeStatement {
    start: LiteralInteger {
        value: 2,
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
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"RangeStatement {
    start: LiteralInteger {
        value: -2,
    },
    end: LiteralInteger {
        value: -1,
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
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"RangeStatement {
    start: LiteralInteger {
        value: -2,
    },
    end: LiteralInteger {
        value: -1,
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
    let result = parse(lexer).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"RangeStatement {
    start: LiteralInteger {
        value: 1,
    },
    end: LiteralInteger {
        value: 2,
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
    let parse_result = parse(lexer).0;

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
    let parse_result = parse(lexer).0;

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
                    value: 1,
                },
                LiteralInteger {
                    value: 2,
                },
                LiteralInteger {
                    value: 3,
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
    let result = parse(lexer).0;

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
        is_wide: false,
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
        is_wide: false,
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn wide_string_can_be_parsed() {
    let lexer = super::lex(
        "PROGRAM buz VAR x : WSTRING; END_VAR x := \"Hello, World!\"; x := \"\"; END_PROGRAM",
    );
    let result = parse(lexer).0;

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
                    is_wide: true,
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
        is_wide: true,
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
        is_wide: true,
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn arrays_can_be_parsed() {
    let lexer = super::lex(
        "PROGRAM buz VAR x : ARRAY[0..9] OF STRING; END_VAR x[0] := 'Hello, World!'; x[y] := ''; END_PROGRAM",
    );
    let result = parse(lexer).0;

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
                            value: 0,
                        },
                        end: LiteralInteger {
                            value: 9,
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
            value: 0,
        },
    },
    right: LiteralString {
        value: "Hello, World!",
        is_wide: false,
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
        is_wide: false,
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn nested_arrays_can_be_parsed() {
    let lexer = super::lex(
        "PROGRAM buz VAR x : ARRAY[0..9] OF ARRAY[0..9] OF STRING; END_VAR x[0][1] := 'Hello, World!'; x[y][1] := ''; END_PROGRAM",
    );
    let result = parse(lexer).0;

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
                            value: 0,
                        },
                        end: LiteralInteger {
                            value: 9,
                        },
                    },
                    referenced_type: DataTypeDefinition {
                        data_type: ArrayType {
                            name: None,
                            bounds: RangeStatement {
                                start: LiteralInteger {
                                    value: 0,
                                },
                                end: LiteralInteger {
                                    value: 9,
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
                value: 0,
            },
        },
        access: LiteralInteger {
            value: 1,
        },
    },
    right: LiteralString {
        value: "Hello, World!",
        is_wide: false,
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
            value: 1,
        },
    },
    right: LiteralString {
        value: "",
        is_wide: false,
    },
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn multidim_arrays_can_be_parsed() {
    let lexer = super::lex(
        "PROGRAM buz VAR x : ARRAY[0..9,1..2] OF STRING; END_VAR x[0,1] := 'Hello, World!'; x[y,1] := ''; END_PROGRAM",
    );
    let result = parse(lexer).0;

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
                                    value: 0,
                                },
                                end: LiteralInteger {
                                    value: 9,
                                },
                            },
                            RangeStatement {
                                start: LiteralInteger {
                                    value: 1,
                                },
                                end: LiteralInteger {
                                    value: 2,
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
                    value: 0,
                },
                LiteralInteger {
                    value: 1,
                },
            ],
        },
    },
    right: LiteralString {
        value: "Hello, World!",
        is_wide: false,
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
                    value: 1,
                },
            ],
        },
    },
    right: LiteralString {
        value: "",
        is_wide: false,
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
    let result = parse(lexer).0;

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
                value: 7,
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
    let result = parse(lexer).0;

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
                value: 1,
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
    let parse_result = parse(lexer).0;

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
                        value: 1,
                    },
                },
                Assignment {
                    left: Reference {
                        name: "y",
                    },
                    right: LiteralInteger {
                        value: 2,
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
    let parse_result = parse(lexer).0;

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
                        value: 1,
                    },
                    LiteralInteger {
                        value: 2,
                    },
                    LiteralInteger {
                        value: 3,
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
    let parse_result = parse(lexer).0;

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
    let parse_result = parse(lexer).0;

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
    let parse_result = parse(lexer).0;

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

#[test]
fn sized_string_as_function_return() {
    let (ast, diagnostics) = parse(super::lex(
        r"
    FUNCTION foo : STRING[10]
    END_FUNCTION
    ",
    ));

    let expected = Pou {
        name: "foo".into(),
        poly_mode: None,
        pou_type: crate::ast::PouType::Function,
        return_type: Some(DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::StringType {
                name: None,
                is_wide: false,
                size: Some(AstStatement::LiteralInteger {
                    value: 10,
                    location: SourceRange::undefined(),
                    id: 0,
                }),
            },
            location: SourceRange::undefined(),
        }),
        variable_blocks: vec![],
        location: SourceRange::undefined(),
    };

    assert_eq!(format!("{:?}", ast.units[0]), format!("{:?}", expected));
    assert_eq!(diagnostics.is_empty(), true);
}

#[test]
fn array_type_as_function_return() {
    let (ast, diagnostics) = parse(super::lex(
        r"
    FUNCTION foo : ARRAY[0..10] OF INT
    END_FUNCTION
    ",
    ));

    let expected = Pou {
        name: "foo".into(),
        poly_mode: None,
        pou_type: crate::ast::PouType::Function,
        return_type: Some(DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::ArrayType {
                referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                    referenced_type: "INT".into(),
                    location: SourceRange::undefined(),
                }),
                bounds: AstStatement::RangeStatement {
                    start: Box::new(AstStatement::LiteralInteger {
                        id: 0,
                        location: SourceRange::undefined(),
                        value: 0,
                    }),
                    end: Box::new(AstStatement::LiteralInteger {
                        id: 0,
                        location: SourceRange::undefined(),
                        value: 10,
                    }),
                    id: 0,
                },
                name: None,
            },
            location: SourceRange::undefined(),
        }),
        variable_blocks: vec![],
        location: SourceRange::undefined(),
    };

    assert_eq!(format!("{:?}", ast.units[0]), format!("{:?}", expected));
    assert_eq!(diagnostics.is_empty(), true);
}
