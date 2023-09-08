// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::test_utils::tests::parse;
use insta::assert_debug_snapshot;
use plc_ast::{
    ast::AstStatement,
    control_statements::{AstControlStatement, ForLoopStatement, IfStatement},
};
use pretty_assertions::*;

#[test]
fn if_statement() {
    let src = "
        PROGRAM exp 
        IF TRUE THEN
        END_IF
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{statement:#?}");
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
fn test_return_statement() {
    let src = "PROGRAM ret RETURN END_PROGRAM";
    let result = parse(src).0;
    let prg = &result.implementations[0];
    let stmt = &prg.statements[0];

    assert_eq!(format!("{stmt:?}"), "ReturnStatement { condition: None }");
}

#[test]
fn test_continue_statement() {
    let src = "PROGRAM ret CONTINUE END_PROGRAM";
    let result = parse(src).0;
    let prg = &result.implementations[0];
    let stmt = &prg.statements[0];

    assert_eq!(format!("{stmt:?}"), "ContinueStatement");
}

#[test]
fn test_exit_statement() {
    let src = "PROGRAM ret EXIT END_PROGRAM";
    let result = parse(src).0;
    let prg = &result.implementations[0];
    let stmt = &prg.statements[0];

    assert_eq!(format!("{stmt:?}"), "ExitStatement");
}

#[test]
fn if_else_statement_with_expressions() {
    let src = "
        PROGRAM exp 
        IF TRUE THEN
            x;
        ELSE
            y;
        END_IF
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn if_elsif_elsif_else_statement_with_expressions() {
    let src = "
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
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn for_with_literals_statement() {
    let src = "
        PROGRAM exp 
        FOR y := x TO 10 DO
        END_FOR
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn for_with_step_statement() {
    let src = "
        PROGRAM exp 
        FOR x := 1 TO 10 BY 7 DO 
        END_FOR
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn for_with_reference_statement() {
    let src = "
        PROGRAM exp 
        FOR z := x TO y DO
        END_FOR
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn for_with_body_statement() {
    let src = "
        PROGRAM exp 
        FOR z := x TO y DO
            x;
            y;
        END_FOR
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn while_with_literal() {
    let src = "
        PROGRAM exp 
        WHILE TRUE DO
        END_WHILE
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{statement:#?}");
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
    let src = "
        PROGRAM exp 
        WHILE x < 7 DO 
        END_WHILE
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn while_with_body_statement() {
    let src = "
        PROGRAM exp 
        WHILE TRUE DO
            x;
            y;
        END_WHILE
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn repeat_with_literal() {
    let src = "
        PROGRAM exp 
        REPEAT
        UNTIL TRUE
        END_REPEAT
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{statement:#?}");
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
    let src = "
        PROGRAM exp 
        REPEAT
        UNTIL x > 7
        END_REPEAT
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn repeat_with_body_statement() {
    let src = "
        PROGRAM exp 
        REPEAT
            x;
            y;
        UNTIL TRUE
        END_REPEAT
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn case_statement_with_one_condition() {
    let src = "
        PROGRAM exp 
        CASE StateMachine OF
        1: x;
        END_CASE
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn case_statement_with_one_condition_with_trailling_comma() {
    let src = "
        PROGRAM exp 
        CASE StateMachine OF
        1,: x;
        END_CASE
        END_PROGRAM
        ";
    let (result, diagnostics) = parse(src);

    assert_eq!(diagnostics, vec![]);

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn case_statement_with_else_and_no_condition() {
    let src = "
        PROGRAM exp 
        CASE StateMachine OF
        ELSE
        END_CASE
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn case_statement_with_no_conditions() {
    let src = "
        PROGRAM exp 
        CASE StateMachine OF
        END_CASE
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn case_statement_with_one_condition_and_an_else() {
    let src = "
        PROGRAM exp 
        CASE StateMachine OF
        1: x;
        ELSE
            y;
        END_CASE
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn case_statement_with_one_empty_condition_and_an_else() {
    let src = "
        PROGRAM exp 
        CASE StateMachine OF
        1:
        ELSE
            y;
        END_CASE
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn case_statement_with_multiple_conditions() {
    let src = "
        PROGRAM exp 
        CASE StateMachine OF
            1: x;
            2: y; yy; yyy;
            3: z;
        END_CASE
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    assert_debug_snapshot!(statement);
}

#[test]
fn case_statement_with_multiple_expressions_per_condition() {
    let src = "
        PROGRAM exp 
        CASE StateMachine OF
            1,2,3: x;
            4..5, 6: y;
        END_CASE
        END_PROGRAM
        ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];
    assert_debug_snapshot!(statement);
}

#[test]
fn if_stmnt_location_test() {
    let source = "
    PROGRAM prg 
    IF a > 4 THEN
        a + b;
    ELSIF x < 2 THEN
        b + c;
    END_IF
    END_PROGRAM";

    let parse_result = parse(source).0;

    let unit = &parse_result.implementations[0];

    let location = &unit.statements[0].get_location();
    assert_eq!(
        source[location.to_range().unwrap()].to_string(),
        "IF a > 4 THEN
        a + b;
    ELSIF x < 2 THEN
        b + c;
    END_IF"
    );

    if let AstStatement::ControlStatement {
        kind: AstControlStatement::If(IfStatement { blocks, .. }), ..
    } = &unit.statements[0]
    {
        let if_location = blocks[0].condition.as_ref().get_location();
        assert_eq!(source[if_location.to_range().unwrap()].to_string(), "a > 4");

        let elsif_location = blocks[1].condition.as_ref().get_location();
        assert_eq!(source[elsif_location.to_range().unwrap()].to_string(), "x < 2");
    }
}

#[test]
fn for_stmnt_location_test() {
    let source = "
    PROGRAM prg 
    FOR x := 3 TO 9 BY 2 DO
        a + b;
    END_FOR
    END_PROGRAM";

    let parse_result = parse(source).0;

    let unit = &parse_result.implementations[0];

    let location = &unit.statements[0].get_location();
    assert_eq!(
        source[location.to_range().unwrap()].to_string(),
        "FOR x := 3 TO 9 BY 2 DO
        a + b;
    END_FOR"
    );

    if let AstStatement::ControlStatement {
        kind: AstControlStatement::ForLoop(ForLoopStatement { counter, start, end, by_step, .. }),
        ..
    } = &unit.statements[0]
    {
        let counter_location = counter.as_ref().get_location();
        assert_eq!(source[counter_location.to_range().unwrap()].to_string(), "x");

        let start_location = start.as_ref().get_location();
        assert_eq!(source[start_location.to_range().unwrap()].to_string(), "3");

        let end_location = end.as_ref().get_location();
        assert_eq!(source[end_location.to_range().unwrap()].to_string(), "9");

        let by_location = by_step.as_ref().map(|it| it.as_ref().get_location()).unwrap();
        assert_eq!(source[by_location.to_range().unwrap()].to_string(), "2");
    } else {
        panic!("expected ForLoopStatement")
    }
}

#[test]
fn while_stmnt_location_test() {
    let source = "
    PROGRAM prg 
    WHILE a < 2 DO
        a := a - 1;
    END_WHILE
    END_PROGRAM";

    let parse_result = parse(source).0;

    let unit = &parse_result.implementations[0];

    let location = &unit.statements[0].get_location();
    assert_eq!(
        source[location.to_range().unwrap()].to_string(),
        "WHILE a < 2 DO
        a := a - 1;
    END_WHILE"
    );
}

#[test]
fn case_stmnt_location_test() {
    let source = "
    PROGRAM prg 
    CASE a OF
    1:
        a := a - 1;
    2:
        a := a - 1;
    END_CASE
    END_PROGRAM";

    let parse_result = parse(source).0;

    let unit = &parse_result.implementations[0];

    let location = &unit.statements[0].get_location();
    assert_eq!(
        source[location.to_range().unwrap()].to_string(),
        "CASE a OF
    1:
        a := a - 1;
    2:
        a := a - 1;
    END_CASE"
    );
}

#[test]
fn call_stmnt_location_test() {
    let source = "
    PROGRAM prg 
    foo(a:=3, b:=4);
    END_PROGRAM";

    let parse_result = parse(source).0;

    let unit = &parse_result.implementations[0];

    let location = &unit.statements[0].get_location();
    assert_eq!(source[location.to_range().unwrap()].to_string(), "foo(a:=3, b:=4)");

    if let AstStatement::CallStatement { operator, parameters, .. } = &unit.statements[0] {
        let operator_location = operator.as_ref().get_location();
        assert_eq!(source[operator_location.to_range().unwrap()].to_string(), "foo");

        let parameters_statement = parameters.as_ref().as_ref();
        let parameters_location = parameters_statement.map(|it| it.get_location()).unwrap();
        assert_eq!(source[parameters_location.to_range().unwrap()].to_string(), "a:=3, b:=4");
    }
}
