// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::parser::tests::ref_to;
use crate::test_utils::tests::parse;
use insta::{assert_debug_snapshot, assert_snapshot};
use plc_ast::ast::{Assignment, AstFactory, AstNode, AstStatement, Operator};
use plc_ast::literals::AstLiteral;
use plc_source::source_location::SourceLocation;
use pretty_assertions::*;

#[test]
fn single_statement_parsed() {
    let src = "PROGRAM exp x; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    assert_debug_snapshot!(&prg.statements[0]);
}

#[test]
fn qualified_reference_statement_parsed() {
    let src = "PROGRAM exp a.x; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    assert_debug_snapshot!(&prg.statements[0]);
}

#[test]
fn bitwise_access_parsed() {
    let src = "PROGRAM exp
    a.0;
    a.%X1;
    a.%B1;
    a.%Bb;
    a[0].%W1;
    a.b.%D1;
    a.%B1.%X1;
    END_PROGRAM";
    let (result, diagnostics) = parse(src);

    let prg = &result.implementations[0];
    assert_debug_snapshot!(&prg.statements);
    assert_eq!(true, diagnostics.is_empty());
}

#[test]
fn literal_can_be_parsed() {
    let src = "PROGRAM exp 7; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    assert_debug_snapshot!(&prg.statements[0]);
}

#[test]
fn literal_binary_with_underscore_number_can_be_parsed() {
    let src = "PROGRAM exp 2#101_101; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn literal_hex_number_with_underscores_can_be_parsed() {
    let src = "PROGRAM exp 16#DE_AD_be_ef; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn literal_hex_number_can_be_parsed() {
    let src = "PROGRAM exp 16#DEADbeef; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn literal_oct_number_with_underscores_can_be_parsed() {
    let src = "PROGRAM exp 8#7_7; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn literal_dec_number_with_underscores_can_be_parsed() {
    let src = "PROGRAM exp 43_000; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn literal_oct_number_with_underscore_can_be_parsed() {
    let src = "PROGRAM exp 8#7_7; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn binary_stmts_of_two_variables_parsed() {
    let src = "PROGRAM exp
        x+y;
        x.y = y.z;
        x.y - y.z;
    END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements;

    assert_debug_snapshot!(statement);
}

#[test]
fn additon_of_three_variables_parsed() {
    let src = "PROGRAM exp x+y-z; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn parenthesis_expressions_should_change_the_ast() {
    let src = "PROGRAM exp (x+y); END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn multiplication_expressions_parse() {
    let src = "PROGRAM exp 1*2/7; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn exponent_expressions_parse() {
    let src = "PROGRAM exp 1**2; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    insta::assert_debug_snapshot!(statement);
}

#[test]
fn addition_ast_test() {
    let src = "PROGRAM exp 1+2; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn multiplication_ast_test() {
    let src = "PROGRAM exp 1+2*3; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn term_ast_test() {
    let src = "PROGRAM exp 1+2*3+4; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn module_expression_test() {
    let src = "PROGRAM exp 5 MOD 2; END_PROGRAM";

    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn parenthesized_term_ast_test() {
    let src = "PROGRAM exp (1+2)*(3+4); END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn boolean_literals_can_be_parsed() {
    let src = "PROGRAM exp TRUE OR FALSE; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn assignment_test() {
    let src = "PROGRAM exp x := 3; x := 1 + 2; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0].statements;
    assert_debug_snapshot!(prg);
}

#[test]
fn equality_expression_test() {
    let src = "PROGRAM exp x = 3; x - 0 <> 1 + 2; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0].statements;
    assert_debug_snapshot!(prg);
}

#[test]
fn comparison_expression_test() {
    let src = "PROGRAM exp
                                    a < 3;
                                    b > 0;
                                    c <= 7;
                                    d >= 4;
                                    e := 2 + 1 > 3 + 1;
                                    END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0].statements;
    assert_debug_snapshot!(prg);
}

#[test]
fn boolean_expression_ast_test() {
    let src = "PROGRAM exp a AND NOT b OR c XOR d; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn boolean_expression_param_ast_test() {
    let src = "PROGRAM exp a AND (NOT (b OR c) XOR d); END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn signed_literal_minus_test() {
    let src = "
        PROGRAM exp
        -1;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn literal_date_test() {
    let src = "
        PROGRAM exp
            DATE#1984-10-01;
            D#2021-04-20;
        END_PROGRAM
        ";
    let result = parse(src).0;
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
fn literal_long_date_test() {
    let src = "
        PROGRAM exp
            LDATE#1984-10-01;
        END_PROGRAM
        ";
    let result = parse(src).0;
    let ast_string = format!("{:#?}", &result.implementations[0].statements);
    let expected_ast = r#"[
    LiteralDate {
        year: 1984,
        month: 10,
        day: 1,
    },
]"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn literal_time_test() {
    let src = "
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
        ";
    let result = parse(src).0;
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
fn literal_long_time_test() {
    let src = "
        PROGRAM exp
            LTIME#12d;
            LTIME#12.4d;
        END_PROGRAM
        ";
    let result = parse(src).0;
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
]"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn literal_time_of_day_test() {
    let src = "
        PROGRAM exp
            TOD#12:00:00;
            TOD#00:12:00;
            TOD#00:00:12;
            TIME_OF_DAY#04:16:22;
            TIME_OF_DAY#04:16:22.1;
            TIME_OF_DAY#04:16:22.001002003;
            TIME_OF_DAY#04:16;
        END_PROGRAM
        ";
    let result = parse(src).0;
    let ast_string = format!("{:#?}", &result.implementations[0].statements);
    let expected_ast = r#"[
    LiteralTimeOfDay {
        hour: 12,
        min: 0,
        sec: 0,
        nano: 0,
    },
    LiteralTimeOfDay {
        hour: 0,
        min: 12,
        sec: 0,
        nano: 0,
    },
    LiteralTimeOfDay {
        hour: 0,
        min: 0,
        sec: 12,
        nano: 0,
    },
    LiteralTimeOfDay {
        hour: 4,
        min: 16,
        sec: 22,
        nano: 0,
    },
    LiteralTimeOfDay {
        hour: 4,
        min: 16,
        sec: 22,
        nano: 100000000,
    },
    LiteralTimeOfDay {
        hour: 4,
        min: 16,
        sec: 22,
        nano: 1002003,
    },
    LiteralTimeOfDay {
        hour: 4,
        min: 16,
        sec: 0,
        nano: 0,
    },
]"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn literal_long_time_of_day_test() {
    let src = "
        PROGRAM exp
            LTOD#12:00:00.123456789;
            LTOD#00:12:00.99;
            LTOD#00:00:12;
        END_PROGRAM
        ";
    let result = parse(src).0;
    let ast_string = format!("{:#?}", &result.implementations[0].statements);
    let expected_ast = r#"[
    LiteralTimeOfDay {
        hour: 12,
        min: 0,
        sec: 0,
        nano: 123456789,
    },
    LiteralTimeOfDay {
        hour: 0,
        min: 12,
        sec: 0,
        nano: 990000000,
    },
    LiteralTimeOfDay {
        hour: 0,
        min: 0,
        sec: 12,
        nano: 0,
    },
]"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn literal_date_and_time_test() {
    let src = "
        PROGRAM exp
            DATE_AND_TIME#1984-10-01-16:40:22;
            DT#2021-04-20-22:33:14;
            DT#2021-04-20-22:33:14.999999999;
            DATE_AND_TIME#2000-01-01-20:15;
        END_PROGRAM
        ";
    let result = parse(src).0;
    let ast_string = format!("{:#?}", &result.implementations[0].statements);
    let expected_ast = r#"[
    LiteralDateAndTime {
        year: 1984,
        month: 10,
        day: 1,
        hour: 16,
        min: 40,
        sec: 22,
        nano: 0,
    },
    LiteralDateAndTime {
        year: 2021,
        month: 4,
        day: 20,
        hour: 22,
        min: 33,
        sec: 14,
        nano: 0,
    },
    LiteralDateAndTime {
        year: 2021,
        month: 4,
        day: 20,
        hour: 22,
        min: 33,
        sec: 14,
        nano: 999999999,
    },
    LiteralDateAndTime {
        year: 2000,
        month: 1,
        day: 1,
        hour: 20,
        min: 15,
        sec: 0,
        nano: 0,
    },
]"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn literal_long_date_and_time_test() {
    let src = "
        PROGRAM exp
            LDT#1984-10-01-16:40:22.123456789;
            LDT#2021-04-20-22:33:14;
        END_PROGRAM
        ";
    let result = parse(src).0;
    let ast_string = format!("{:#?}", &result.implementations[0].statements);
    let expected_ast = r#"[
    LiteralDateAndTime {
        year: 1984,
        month: 10,
        day: 1,
        hour: 16,
        min: 40,
        sec: 22,
        nano: 123456789,
    },
    LiteralDateAndTime {
        year: 2021,
        month: 4,
        day: 20,
        hour: 22,
        min: 33,
        sec: 14,
        nano: 0,
    },
]"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn literal_real_test() {
    let src = "
        PROGRAM exp
        1.1;
        1.2e3;
        1.2e-4;
        -1.5;
        -1.5e3;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements;

    let ast_string = format!("{statement:#?}");
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
    UnaryExpression {
        operator: Minus,
        value: LiteralReal {
            value: "1.5e3",
        },
    },
]"#;
    assert_eq!(ast_string, expected_ast);
}

fn cast(data_type: &str, value: AstNode) -> AstNode {
    AstFactory::create_cast_statement(
        AstFactory::create_member_reference(
            AstFactory::create_identifier(data_type, SourceLocation::internal(), 0),
            None,
            0,
        ),
        value,
        &SourceLocation::internal(),
        0,
    )
}

#[test]
fn literal_enum_parse_test() {
    let src = r#"
        PROGRAM exp
            MyEnum#Val7;
            MyEnum#Val2;
            MyEnum#Val3;
        END_PROGRAM
        "#;
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements;
    assert_debug_snapshot!(statement);
}

#[test]
fn literal_cast_parse_test() {
    let src = r#"
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
            CHAR#"A";
            WCHAR#'B';
        END_PROGRAM
        "#;
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements;

    let ast_string = format!("{statement:#?}");
    fn literal(value: AstLiteral) -> AstNode {
        AstFactory::create_literal(value, SourceLocation::internal(), 0)
    }

    assert_eq!(
        ast_string,
        format!(
            "{:#?}",
            vec![
                cast("SINT", literal(AstLiteral::new_integer(100))),
                cast("DINT", literal(AstLiteral::new_integer(45054))),
                cast("BYTE", literal(AstLiteral::new_integer(63))),
                cast("WORD", literal(AstLiteral::new_integer(10))),
                cast("INT", literal(AstLiteral::new_integer(100))),
                cast("DINT", literal(AstLiteral::new_integer(-100))),
                cast("REAL", literal(AstLiteral::new_real("-3.1415".into()))),
                cast("BOOL", literal(AstLiteral::new_integer(1))),
                cast("BOOL", literal(AstLiteral::new_bool(false))),
                cast("STRING", literal(AstLiteral::new_string("abc".into(), true))),
                cast("WSTRING", literal(AstLiteral::new_string("xyz".into(), false))),
                cast("CHAR", literal(AstLiteral::new_string("A".into(), true))),
                cast("WCHAR", literal(AstLiteral::new_string("B".to_string(), false))),
            ]
        )
    );
}

#[test]
fn literal_exponents_test() {
    let src = "
        PROGRAM exp
        1_2e3;
        12e3;
        12.0e3;
        12e-4;
        1_2e-4;
        12.0e-4;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements;

    let ast_string = format!("{statement:#?}");
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
    let src = "
        PROGRAM exp
        2 +-x;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn assignment_to_null() {
    let src = "
        PROGRAM exp
        x := NULL;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn assignment_to_number_with_implicit_and_explicit_plus_sign() {
    let src = "
        PROGRAM exp
            VAR
                x : INT;
            END_VAR
            x := 1;
            x := +1;
        END_PROGRAM
    ";

    let result = parse(src).0;
    let statements = &result.implementations[0].statements;

    assert_debug_snapshot!(statements);
}

#[test]
fn assignment_to_number_reference_with_explicit_plus_sign() {
    let src = "
        PROGRAM exp
            VAR
                x : INT;
            END_VAR
            x := 1;
            x := +x;
        END_PROGRAM
    ";

    let result = parse(src).0;
    let statements = &result.implementations[0].statements;
    assert_debug_snapshot!(statements);
}

#[test]
fn pointer_dereference_test() {
    let src = "
        PROGRAM exp
        x^;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn pointer_dereference_test_nested() {
    let src = "
        PROGRAM exp
        x^^[0][1]^[2]^^;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn signed_literal_expression_reversed_test() {
    let src = "
        PROGRAM exp
        -4 + 5;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn or_compare_expressions_priority_test() {
    let src = "
        PROGRAM exp
        x > 1 OR b1;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn addition_compare_or_priority_test() {
    let src = "
        PROGRAM exp
        x + 1 > 2 OR b1;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn and_test() {
    let src = "
        PROGRAM amp
        b AND c;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn amp_as_and_test() {
    let src = "
        PROGRAM amp
        b & c;
        END_PROGRAM
        ";
    let result = parse(src).0;
    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn boolean_priority_test() {
    let src = "
        PROGRAM exp
        a AND b XOR c OR d;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn comparison_priority_test() {
    let src = "
        PROGRAM exp
        x < 7 = y > 6;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn expression_list() {
    //technically this is an illegal state, the parser will accept it though
    let src = "
        PROGRAM exp
        1,2,3;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn expression_list_assignments() {
    //technically this is an illegal state, the parser will accept it though
    let src = "
        PROGRAM exp
        x := 1, y := 2, z:= 3;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn range_expression() {
    let src = "
        PROGRAM exp
        a..b;
        1..2;
        a..2;
        2..a;
        -2..-1;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statements = &prg.statements;
    assert_debug_snapshot!(statements)
}

#[test]
fn negative_range_expression() {
    let src = "
        PROGRAM exp
        -2..-1;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{statement:#?}");
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
    let src = "
        PROGRAM exp
        -2 ..-1;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{statement:#?}");
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
    let src = "
        PROGRAM exp
        1 .. 2;
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{statement:#?}");
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
    let src = "
    PROGRAM prg
    fn();
    END_PROGRAM
    ";
    let parse_result = parse(src).0;

    let statement = &parse_result.implementations[0].statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn function_call_params() {
    let src = "
    PROGRAM prg
    fn(1,2,3);
    END_PROGRAM
    ";
    let parse_result = parse(src).0;

    let statement = &parse_result.implementations[0].statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn function_call_params_with_trailling_comma() {
    let src = "
    PROGRAM prg
    fn(1,2,3,);
    END_PROGRAM
    ";
    let (parse_result, diagnostics) = parse(src);

    assert_eq!(diagnostics, vec![]);

    let statement = &parse_result.implementations[0].statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn string_can_be_parsed() {
    let src = "PROGRAM buz VAR x : STRING; END_VAR x := 'Hello, World!'; x := ''; END_PROGRAM";
    let result = parse(src).0;

    let unit = &result.pous[0];
    let prg = &result.implementations[0];
    let variable_block = &unit.variable_blocks[0];
    let ast_string = format!("{variable_block:#?}");
    assert_snapshot!(ast_string, @r#"
    VariableBlock {
        variables: [
            Variable {
                name: "x",
                data_type: DataTypeReference {
                    referenced_type: "STRING",
                },
            },
        ],
        variable_block_type: Local,
    }
    "#);

    let statements = &prg.statements;
    assert_debug_snapshot!(statements);
}

#[test]
fn wide_string_can_be_parsed() {
    let src = "PROGRAM buz VAR x : WSTRING; END_VAR x := \"Hello, World!\"; x := \"\"; END_PROGRAM";
    let result = parse(src).0;

    let unit = &result.pous[0];
    let prg = &result.implementations[0];
    let variable_block = &unit.variable_blocks[0];
    let ast_string = format!("{variable_block:#?}");
    assert_snapshot!(ast_string, @r#"
    VariableBlock {
        variables: [
            Variable {
                name: "x",
                data_type: DataTypeReference {
                    referenced_type: "WSTRING",
                },
            },
        ],
        variable_block_type: Local,
    }
    "#);

    let statements = &prg.statements;
    assert_debug_snapshot!(statements);
}

#[test]
fn arrays_can_be_parsed() {
    let src =
        "PROGRAM buz VAR x : ARRAY[0..9] OF STRING; END_VAR x[0] := 'Hello, World!'; x[y] := ''; END_PROGRAM";
    let result = parse(src).0;

    let unit = &result.pous[0];
    let prg = &result.implementations[0];
    let variable_block = &unit.variable_blocks[0];
    let ast_string = format!("{variable_block:#?}");

    assert_snapshot!(ast_string, @r#"
    VariableBlock {
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
                        referenced_type: DataTypeReference {
                            referenced_type: "STRING",
                        },
                        is_variable_length: false,
                    },
                },
            },
        ],
        variable_block_type: Local,
    }
    "#);

    let statements = &prg.statements;
    assert_debug_snapshot!(statements);
}

#[test]
fn nested_arrays_can_be_parsed() {
    let src = "PROGRAM buz VAR x : ARRAY[0..9] OF ARRAY[0..9] OF STRING; END_VAR x[0][1] := 'Hello, World!'; x[y][1] := ''; END_PROGRAM";
    let result = parse(src).0;

    let unit = &result.pous[0];
    let prg = &result.implementations[0];
    let variable_block = &unit.variable_blocks[0];
    let ast_string = format!("{variable_block:#?}");
    assert_snapshot!(ast_string, @r#"
    VariableBlock {
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
                                referenced_type: DataTypeReference {
                                    referenced_type: "STRING",
                                },
                                is_variable_length: false,
                            },
                        },
                        is_variable_length: false,
                    },
                },
            },
        ],
        variable_block_type: Local,
    }
    "#);

    let statements = &prg.statements;
    assert_debug_snapshot!(statements);
}

#[test]
fn multidim_arrays_can_be_parsed() {
    let src = "PROGRAM buz VAR x : ARRAY[0..9,1..2] OF STRING; END_VAR x[0,1] := 'Hello, World!'; x[y,1] := ''; END_PROGRAM";
    let result = parse(src).0;

    let unit = &result.pous[0];
    let prg = &result.implementations[0];
    let variable_block = &unit.variable_blocks[0];
    let ast_string = format!("{variable_block:#?}");
    assert_snapshot!(ast_string, @r#"
    VariableBlock {
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
                        referenced_type: DataTypeReference {
                            referenced_type: "STRING",
                        },
                        is_variable_length: false,
                    },
                },
            },
        ],
        variable_block_type: Local,
    }
    "#);

    let statements = &prg.statements;
    assert_debug_snapshot!(statements);
}

#[test]
fn arrays_in_structs_can_be_parsed() {
    let src = "
        PROGRAM buz VAR x : MyStructWithArray; END_VAR x.y[7]; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn arrays_of_structs_can_be_parsed() {
    let src = "
        PROGRAM buz VAR x : ARRAY[0..1] OF MyStruct; END_VAR x[1].y; END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn function_call_formal_params() {
    let src = "
    PROGRAM prg
    fn(x := 1,y := 2,z => a);
    END_PROGRAM
    ";
    let parse_result = parse(src).0;

    let statement = &parse_result.implementations[0].statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn function_call_return_params() {
    let src = "
    PROGRAM prg
    x := fn(1,2,3);
    END_PROGRAM
    ";
    let parse_result = parse(src).0;

    let statement = &parse_result.implementations[0].statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn literals_location_test() {
    let source = "PROGRAM prg 7; 'hello'; TRUE; 3.1415; END_PROGRAM";
    let parse_result = parse(source).0;

    let unit = &parse_result.implementations[0];

    // 1
    let location = &unit.statements[0].get_location();
    assert_eq!(location.to_range().unwrap(), (12..13));
    assert_eq!(source[location.to_range().unwrap()].to_string(), "7");

    // 'hello'
    let location = &unit.statements[1].get_location();
    assert_eq!(location.to_range().unwrap(), (15..22));
    assert_eq!(source[location.to_range().unwrap()].to_string(), "'hello'");

    // true
    let location = &unit.statements[2].get_location();
    assert_eq!(location.to_range().unwrap(), (24..28));
    assert_eq!(source[location.to_range().unwrap()].to_string(), "TRUE");

    //3.1415
    let location = &unit.statements[3].get_location();
    assert_eq!(location.to_range().unwrap(), (30..36));
    assert_eq!(source[location.to_range().unwrap()].to_string(), "3.1415")
}

#[test]
fn reference_location_test() {
    let source = "PROGRAM prg a;bb;ccc; END_PROGRAM";
    let parse_result = parse(source).0;

    let unit = &parse_result.implementations[0];

    let location = &unit.statements[0].get_location();
    assert_eq!(source[location.to_range().unwrap()].to_string(), "a");

    let location = &unit.statements[1].get_location();
    assert_eq!(source[location.to_range().unwrap()].to_string(), "bb");

    let location = &unit.statements[2].get_location();
    assert_eq!(source[location.to_range().unwrap()].to_string(), "ccc");
}

#[test]
fn qualified_reference_location_test() {
    let source = "PROGRAM prg a.b.c;aa.bb.cc[2];aaa.bbb.ccc^;&aaa.bbb.ccc; END_PROGRAM";
    let parse_result = parse(source).0;

    let unit = &parse_result.implementations[0];

    let location = &unit.statements[0].get_location();
    assert_eq!(source[location.to_range().unwrap()].to_string(), "a.b.c");

    let location = &unit.statements[1].get_location();
    assert_eq!(source[location.to_range().unwrap()].to_string(), "aa.bb.cc[2]");

    let location = &unit.statements[2].get_location();
    assert_eq!(source[location.to_range().unwrap()].to_string(), "aaa.bbb.ccc^");

    let location = &unit.statements[3].get_location();
    assert_eq!(source[location.to_range().unwrap()].to_string(), "&aaa.bbb.ccc");
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
    let parse_result = parse(source).0;

    let unit = &parse_result.implementations[0];

    let location = &unit.statements[0].get_location();
    assert_eq!(source[location.to_range().unwrap()].to_string(), "a + b");

    let location = &unit.statements[1].get_location();
    assert_eq!(source[location.to_range().unwrap()].to_string(), "x + z - y + u - v");

    let location = &unit.statements[2].get_location();
    assert_eq!(source[location.to_range().unwrap()].to_string(), "-x");

    let location = &unit.statements[3].get_location();
    assert_eq!(source[location.to_range().unwrap()].to_string(), "1..3");

    let location = &unit.statements[4].get_location();
    assert_eq!(source[location.to_range().unwrap()].to_string(), "a := a + 4");
}

#[test]
fn sized_string_as_function_return() {
    let (ast, diagnostics) = parse(
        r"
    FUNCTION foo : STRING[10]
    END_FUNCTION
    ",
    );
    assert_debug_snapshot!(ast.pous[0], @r#"
    POU {
        name: "foo",
        variable_blocks: [],
        pou_type: Function,
        return_type: Some(
            DataTypeDefinition {
                data_type: StringType {
                    name: None,
                    is_wide: false,
                    size: Some(
                        LiteralInteger {
                            value: 10,
                        },
                    ),
                },
            },
        ),
        interfaces: [],
        properties: [],
    }
    "#);
    assert_eq!(diagnostics.is_empty(), true);
}

#[test]
fn array_type_as_function_return() {
    let (ast, diagnostics) = parse(
        r"
    FUNCTION foo : ARRAY[0..10] OF INT
    END_FUNCTION
    ",
    );

    assert_debug_snapshot!(ast.pous[0], @r#"
    POU {
        name: "foo",
        variable_blocks: [],
        pou_type: Function,
        return_type: Some(
            DataTypeDefinition {
                data_type: ArrayType {
                    name: None,
                    bounds: RangeStatement {
                        start: LiteralInteger {
                            value: 0,
                        },
                        end: LiteralInteger {
                            value: 10,
                        },
                    },
                    referenced_type: DataTypeReference {
                        referenced_type: "INT",
                    },
                    is_variable_length: false,
                },
            },
        ),
        interfaces: [],
        properties: [],
    }
    "#);
    assert_eq!(diagnostics.is_empty(), true);
}

#[test]
fn exp_mul_priority_test() {
    let (ast, diagnostics) = parse(
        r"
    FUNCTION foo : INT
        a * b ** c;
    END_FUNCTION
    ",
    );

    insta::assert_debug_snapshot!(ast);

    assert_eq!(diagnostics.is_empty(), true);
}

#[test]
/// regress #286
fn plus_minus_parse_tree_priority_test() {
    let (ast, diagnostics) = parse(
        r"
    FUNCTION foo : INT
        a - b + c;
    END_FUNCTION
    ",
    );
    assert_eq!(
        format!("{:#?}", ast.implementations[0].statements[0]),
        format!(
            "{:#?}",
            AstFactory::create_binary_expression(
                AstFactory::create_binary_expression(ref_to("a"), Operator::Minus, ref_to("b"), 0),
                Operator::Plus,
                ref_to("c"),
                0
            )
        )
    );
    assert_eq!(diagnostics.is_empty(), true);
}

#[test]
/// regress #286
fn mul_div_mod_parse_tree_priority_test() {
    let (ast, diagnostics) = parse(
        r"
    FUNCTION foo : INT
        a * b / c MOD d;
    END_FUNCTION
    ",
    );

    assert_eq!(
        format!("{:#?}", ast.implementations[0].statements[0]),
        format!(
            "{:#?}",
            AstFactory::create_binary_expression(
                AstFactory::create_binary_expression(
                    AstFactory::create_binary_expression(
                        ref_to("a"),
                        Operator::Multiplication,
                        ref_to("b"),
                        0
                    ),
                    Operator::Division,
                    ref_to("c"),
                    0
                ),
                Operator::Modulo,
                ref_to("d"),
                0
            )
        )
    );
    assert_eq!(diagnostics.is_empty(), true);
}

#[test]
fn direct_access_as_expression_parsed() {
    // GIVEN a program with several types of direct access
    let src = "
    PROGRAM prg
        x := 6 + %IX2.1;
        y := %MB200;
        z := %GD5 * 2;
    END_PROGRAM
    ";

    // WHEN The program is parsed
    let (result, _) = parse(src);

    //THEN the AST contains direct address nodes at the access location
    assert_debug_snapshot!(result);
}

#[test]
fn parenthesized_expression_span() {
    let src = "PROGRAM prg [(1 + 2)] END_PROGRAM";

    let (result, _) = parse(src);
    let AstStatement::Literal(AstLiteral::Array(array)) = result.implementations[0].statements[0].get_stmt()
    else {
        panic!()
    };
    let range = array.elements().unwrap().get_location().get_span().to_range().unwrap();
    assert_eq!(&src[range.start..range.end], "(1 + 2)");
}

#[test]
fn function_call_array_index() {
    let src = "
    PROGRAM prg
        foo()[1];
        foo()[1 + 2]
        foo()[one];
        foo()[one + two];
        foo()[bar()];
    END_PROGRAM
    ";

    let parse_result = parse(src).0;
    assert_debug_snapshot!(parse_result.implementations[0].statements);
}

#[test]
fn global_namespace_operator() {
    let src = r#"
    FUNCTION main
        .foo;
        .foo := 1;
        .foo := .foo + 1;

        foo := 1 + .foo + foo();
        foo := 1 + .foo + 2 + .foo;

        someFunc(.foo);
        printf("%d$N", .foo);
    END_FUNCTION
    "#;

    let result = parse(src).0;

    assert_eq!(&src[result.implementations[0].statements[0].get_location().to_range().unwrap()], ".foo");
    assert_debug_snapshot!(result.implementations[0].statements);
}

#[test]
fn super_keyword_can_be_parsed_in_expressions() {
    let src = "
    FUNCTION_BLOCK fb
        super.x;
        super^.y;
        super;
        super^.foo(super.x + super^.y);
        super();
        super := REF(fb2);
    END_FUNCTION_BLOCK
        ";

    let parse_result = parse(src).0;
    assert_debug_snapshot!(parse_result.implementations[0].statements, @r#"
    [
        ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "x",
                },
            ),
            base: Some(
                Super,
            ),
        },
        ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "y",
                },
            ),
            base: Some(
                Super(derefed),
            ),
        },
        Super,
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "foo",
                    },
                ),
                base: Some(
                    Super(derefed),
                ),
            },
            parameters: Some(
                BinaryExpression {
                    operator: Plus,
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: Some(
                            Super,
                        ),
                    },
                    right: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "y",
                            },
                        ),
                        base: Some(
                            Super(derefed),
                        ),
                    },
                },
            ),
        },
        CallStatement {
            operator: Super,
            parameters: None,
        },
        Assignment {
            left: Super,
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
                                name: "fb2",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        },
    ]
    "#);
}

#[test]
fn this_keyword_can_be_parsed_in_expressions() {
    let src = "
    FUNCTION_BLOCK fb
        this.x;
        this^.y;
        this;
        this^.foo(this.x + this^.y);
        this();
        this := REF(fb2);
    END_FUNCTION_BLOCK
        ";

    let parse_result = parse(src).0;
    assert_debug_snapshot!(parse_result.implementations[0].statements, @r#"
    [
        ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "x",
                },
            ),
            base: Some(
                This,
            ),
        },
        ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "y",
                },
            ),
            base: Some(
                ReferenceExpr {
                    kind: Deref,
                    base: Some(
                        This,
                    ),
                },
            ),
        },
        This,
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "foo",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Deref,
                        base: Some(
                            This,
                        ),
                    },
                ),
            },
            parameters: Some(
                BinaryExpression {
                    operator: Plus,
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: Some(
                            This,
                        ),
                    },
                    right: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "y",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Deref,
                                base: Some(
                                    This,
                                ),
                            },
                        ),
                    },
                },
            ),
        },
        CallStatement {
            operator: This,
            parameters: None,
        },
        Assignment {
            left: This,
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
                                name: "fb2",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        },
    ]
    "#);
}

#[test]
fn this_keyword_can_be_mixed_with_super() {
    let src = "
    FUNCTION_BLOCK fb
        this^.super^.foo(this^.x + this^.y);
    END_FUNCTION_BLOCK
        ";

    let parse_result = parse(src).0;
    assert_debug_snapshot!(parse_result.implementations[0].statements, @r#"
    [
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "foo",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Super(derefed),
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Deref,
                                base: Some(
                                    This,
                                ),
                            },
                        ),
                    },
                ),
            },
            parameters: Some(
                BinaryExpression {
                    operator: Plus,
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Deref,
                                base: Some(
                                    This,
                                ),
                            },
                        ),
                    },
                    right: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "y",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Deref,
                                base: Some(
                                    This,
                                ),
                            },
                        ),
                    },
                },
            ),
        },
    ]
    "#);
}

#[test]
fn this_keyword_can_be_parsed_in_method() {
    let src = "
    FUNCTION_BLOCK fb
        METHOD doSomething : INT
            doSomething := this^.y;
        END_METHOD
    END_FUNCTION_BLOCK
somePtr := this;
        ";

    let parse_result = parse(src).0;
    assert_debug_snapshot!(parse_result.implementations[0].statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "doSomething",
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
                base: Some(
                    ReferenceExpr {
                        kind: Deref,
                        base: Some(
                            This,
                        ),
                    },
                ),
            },
        },
    ]
    "#);
}

#[test]
fn unary_plus_expression_test() {
    let src = "
    PROGRAM exp
    VAR
        x : INT;
    END_VAR
        +x;
        x := +x + 4;
        x := +-4 + 5;
        +-x;
        x := +foo(+x);
    END_PROGRAM

    FUNCTION foo : INT
    VAR_INPUT
        x : INT;
    END_VAR
    END_FUNCTION
    ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    assert_debug_snapshot!(statement, @r#"
    UnaryExpression {
        operator: Plus,
        value: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "x",
                },
            ),
            base: None,
        },
    }
    "#);

    let AstStatement::Assignment(Assignment { right: expr, .. }) = &prg.statements[1].get_stmt() else {
        panic!()
    };
    assert_debug_snapshot!(expr, @r#"
    BinaryExpression {
        operator: Plus,
        left: UnaryExpression {
            operator: Plus,
            value: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: None,
            },
        },
        right: LiteralInteger {
            value: 4,
        },
    }
    "#);

    let AstStatement::Assignment(Assignment { right: expr, .. }) = &prg.statements[2].get_stmt() else {
        panic!()
    };
    assert_debug_snapshot!(expr, @r"
    BinaryExpression {
        operator: Plus,
        left: LiteralInteger {
            value: -4,
        },
        right: LiteralInteger {
            value: 5,
        },
    }
    ");

    let statement = &prg.statements[3];
    assert_debug_snapshot!(statement, @r#"
    UnaryExpression {
        operator: Plus,
        value: UnaryExpression {
            operator: Minus,
            value: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: None,
            },
        },
    }
    "#);

    let AstStatement::Assignment(Assignment { right: expr, .. }) = &prg.statements[4].get_stmt() else {
        panic!()
    };
    assert_debug_snapshot!(expr, @r#"
    UnaryExpression {
        operator: Plus,
        value: CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "foo",
                    },
                ),
                base: None,
            },
            parameters: Some(
                UnaryExpression {
                    operator: Plus,
                    value: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: None,
                    },
                },
            ),
        },
    }
    "#);
}
