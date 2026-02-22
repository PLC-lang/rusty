use indoc::indoc;
use plc_rowan_parser::{grammar, SyntaxKind};

use crate::{
    ast::{AstNode, HasName, Expression, ExpressionStmt},
    test::test_util::parse_generic,
    expect_all,
};

#[test]
fn while_accessors() {
    let src = indoc! {"
                WHILE running DO
                    x := 1;
                END_WHILE
        "};

    let stmt: ExpressionStmt =
        parse_generic(src, |p| grammar::statement_grammar::expression_stmt(p, true)).ok().unwrap();

    let Expression::WhileStatement(while_stmt) = stmt.expression().unwrap() else {
        panic!("expected WhileStatement");
    };
    assert!(while_stmt.WHILE_token().is_some());
    assert!(while_stmt.DO_token().is_some());
    assert!(while_stmt.END_WHILE_token().is_some());
}

// -----------------------------------------------------------------------
// IfStatement accessors
// -----------------------------------------------------------------------

#[test]
fn if_accessors() {
    let src = indoc! {"
                IF cond THEN
                    x := 1;
                ELSE
                    x := 2;
                END_IF
        "};
    let stmt: ExpressionStmt =
        parse_generic(src, |p| grammar::statement_grammar::expression_stmt(p, true)).ok().unwrap();
    let Expression::IfStatement(if_stmt) = stmt.expression().unwrap() else {
        panic!("expected IfStatement");
    };
    expect_all!(
        if_stmt.IF_token(),
        if_stmt.if_condition().unwrap().THEN_token(),
        if_stmt.else_arm().unwrap().ELSE_token(),
        if_stmt.END_IF_token()
    );

    // if_condition is the ConditionThenBlock holding the condition + THEN branch
    let if_cond = if_stmt.if_condition().unwrap();
    assert_eq!(if_cond.condition_expr().unwrap().text(), "cond".to_string());
    assert_eq!(if_cond.body().unwrap().expression_stmts().next().unwrap().text(), "x:=1;".to_string());

    // ELSE arm is present, no ELSIF arms
    let else_arm = if_stmt.else_arm().unwrap();
    assert_eq!(else_arm.body().unwrap().expression_stmts().next().unwrap().text(), "x:=2;".to_string());

    // no elsif arms
    assert_eq!(if_stmt.else_if_arms().count(), 0);
}

#[test]
fn if_elsif_accessors() {
    let src = indoc! {"
                IF a THEN
                    x := 1;
                ELSIF b THEN
                    x := 2;
                END_IF
        "};
    let stmt: ExpressionStmt =
        parse_generic(src, |p| grammar::statement_grammar::expression_stmt(p, true)).ok().unwrap();

    let Expression::IfStatement(if_stmt) = stmt.expression().unwrap() else {
        panic!("expected IfStatement");
    };
    // if_condition covers the primary IF branch
    assert!(if_stmt.if_condition().is_some());
    assert_eq!(if_stmt.else_if_arms().count(), 1);
    assert!(if_stmt.else_arm().is_none());
}

#[test]
fn if_condition_accessors() {
    // Verify that if_condition() returns the ConditionThenBlock for the IF branch,
    // and that it has the correct condition text and a body with statements.
    let src = indoc! {"
                IF ready THEN
                    done := 1;
                END_IF
        "};
    let stmt: ExpressionStmt =
        parse_generic(src, |p| grammar::statement_grammar::expression_stmt(p, true)).ok().unwrap();
    let Expression::IfStatement(if_stmt) = stmt.expression().unwrap() else {
        panic!("expected IfStatement");
    };

    let if_cond = if_stmt.if_condition().expect("if_condition should be present");

    // The THEN keyword is part of ConditionThenBlock
    assert!(if_cond.THEN_token().is_some());

    // The condition expression (first ExpressionStmt child) holds the identifier
    let Expression::NameRef(name_ref) =
        if_cond.condition_expr().as_ref().and_then(ExpressionStmt::expression).unwrap()
    else {
        panic!();
    };
    assert_eq!(name_ref.ident_token().unwrap().text(), "ready");

    // The then_branch (Body child) contains the assignment
    assert_eq!(if_cond.body().unwrap().expression_stmts().count(), 1);
}

// -----------------------------------------------------------------------
// ForStatement accessors
// -----------------------------------------------------------------------

#[test]
fn for_without_by_accessors() {
    let src = indoc! {"
                FOR i := 0 TO 10 DO
                    x := 1;
                END_FOR
        "};
    let stmt: ExpressionStmt =
        parse_generic(src, |p| grammar::statement_grammar::expression_stmt(p, true)).ok().unwrap();
    let Expression::ForStatement(for_stmt) = stmt.expression().unwrap() else {
        panic!("expected ForStatement");
    };
    // No BY keyword — use BY_token() to detect the optional step
    assert!(for_stmt.BY_token().is_none());
    // counter target is "i"
    let counter = for_stmt.counter().unwrap();
    assert_eq!(counter.target().unwrap().ident_token().unwrap().text(), "i");
}

// -----------------------------------------------------------------------
// CaseStatement accessors
// -----------------------------------------------------------------------

#[test]
fn case_accessors() {
    let src = indoc! {"
        CASE myCondition OF
            1:
                a := 1;
        END_CASE
    "};
    let stmt: ExpressionStmt =
        parse_generic(src, |p| grammar::statement_grammar::expression_stmt(p, true)).ok().unwrap();
    let Expression::CaseStatement(case_stmt) = stmt.expression().unwrap() else {
        panic!("expected CaseStatement");
    };

    // All keyword tokens should be present
    expect_all!(
        case_stmt.CASE_token(),
        case_stmt.OF_token(),
        case_stmt.END_CASE_token(),
    );

    // case_expr should hold the selector expression
    let case_expr = case_stmt.case_expr().expect("case_expr should be present");
    let Expression::NameRef(name_ref) = case_expr.expression().unwrap() else {
        panic!("expected NameRef as case expression");
    };
    assert_eq!(name_ref.ident_token().unwrap().text(), "myCondition");
}

#[test]
fn case_arms_with_multiple_values() {
    let src = indoc! {"
        CASE sel OF
            1, 5:
                x := 10;
            2:
                x := 20;
        END_CASE
    "};
    let stmt: ExpressionStmt =
        parse_generic(src, |p| grammar::statement_grammar::expression_stmt(p, true)).ok().unwrap();
    let Expression::CaseStatement(case_stmt) = stmt.expression().unwrap() else {
        panic!("expected CaseStatement");
    };

    let arms: Vec<_> = case_stmt.case_arms().collect();
    assert_eq!(arms.len(), 2, "expected two case arms");

    // First arm: values 1, 5
    let first_arm = &arms[0];
    assert!(first_arm.colon_token().is_some());
    let first_values = first_arm.case_values().expect("first arm should have case_values");
    let exprs: Vec<_> = first_values.expressions().collect();
    assert_eq!(exprs.len(), 2, "first arm should have two values");

    // First arm body should contain one statement
    let first_body = first_arm.body().expect("first arm should have a body");
    assert_eq!(first_body.expression_stmts().count(), 1);

    // Second arm: value 2
    let second_arm = &arms[1];
    assert!(second_arm.colon_token().is_some());
    let second_values = second_arm.case_values().expect("second arm should have case_values");
    assert_eq!(second_values.expressions().count(), 1);
}

#[test]
fn case_with_else_arm() {
    let src = indoc! {"
        CASE x OF
            1:
                a := 1;
            ELSE
                a := 0;
        END_CASE
    "};
    let stmt: ExpressionStmt =
        parse_generic(src, |p| grammar::statement_grammar::expression_stmt(p, true)).ok().unwrap();
    let Expression::CaseStatement(case_stmt) = stmt.expression().unwrap() else {
        panic!("expected CaseStatement");
    };

    // There should be one case arm and the ELSE arm should exist as a child node
    let arms: Vec<_> = case_stmt.case_arms().collect();
    assert_eq!(arms.len(), 1, "expected one case arm before ELSE");

    // The ELSE arm is emitted as an ELSE_ARM node inside the CaseStatement.
    // Verify it exists by looking for a child with kind ELSE_ARM.
    let else_arm = case_stmt
        .syntax()
        .children()
        .find(|n| n.kind() == SyntaxKind::ELSE_ARM)
        .expect("expected an ELSE_ARM child node");
    // The ELSE_ARM should start with the ELSE keyword token
    let has_else_kw = else_arm
        .children_with_tokens()
        .any(|it| it.as_token().is_some_and(|t| t.kind() == SyntaxKind::ELSE_KW));
    assert!(has_else_kw, "ELSE_ARM should contain the ELSE keyword");
}

#[test]
fn case_arm_body_has_statements() {
    let src = indoc! {"
        CASE n OF
            1:
                a := 10;
                b := 20;
        END_CASE
    "};
    let stmt: ExpressionStmt =
        parse_generic(src, |p| grammar::statement_grammar::expression_stmt(p, true)).ok().unwrap();
    let Expression::CaseStatement(case_stmt) = stmt.expression().unwrap() else {
        panic!("expected CaseStatement");
    };

    let arm = case_stmt.case_arms().next().expect("expected at least one case arm");
    let body = arm.body().expect("case arm should have a body");
    assert_eq!(body.expression_stmts().count(), 2, "case arm body should contain two statements");
}

// -----------------------------------------------------------------------
// CallStatement accessors
// -----------------------------------------------------------------------

#[test]
fn call_statement_positional_args() {
    let src = "foo(4, 5);";
    let stmt: ExpressionStmt =
        parse_generic(src, |p| grammar::statement_grammar::expression_stmt(p, true)).ok().unwrap();
    let Expression::CallStatement(call) = stmt.expression().unwrap() else {
        panic!("expected CallStatement");
    };

    // Tokens present
    expect_all!(
        call.l_paren_token(),
        call.r_paren_token(),
    );

    // Callee
    let callee = call.callee().expect("callee should be present");
    assert_eq!(callee.ident_token().unwrap().text(), "foo");

    // Argument list with two positional args (no name)
    let arg_list = call.argument_list().expect("argument_list should be present");
    let args: Vec<_> = arg_list.arguments().collect();
    assert_eq!(args.len(), 2);

    // Positional args have no name
    assert!(args[0].name().is_none());
    assert!(args[1].name().is_none());
}

#[test]
fn call_statement_named_args() {
    let src = "TMR(IN := 5, PT := x);";
    let stmt: ExpressionStmt =
        parse_generic(src, |p| grammar::statement_grammar::expression_stmt(p, true)).ok().unwrap();
    let Expression::CallStatement(call) = stmt.expression().unwrap() else {
        panic!("expected CallStatement");
    };

    let callee = call.callee().expect("callee should be present");
    assert_eq!(callee.ident_token().unwrap().text(), "TMR");

    let arg_list = call.argument_list().expect("argument_list should be present");
    let args: Vec<_> = arg_list.arguments().collect();
    assert_eq!(args.len(), 2);

    // Named args have names
    assert_eq!(args[0].name().unwrap().ident_token().unwrap().text(), "IN");
    assert_eq!(args[1].name().unwrap().ident_token().unwrap().text(), "PT");
}

#[test]
fn call_statement_no_args() {
    let src = "reset();";
    let stmt: ExpressionStmt =
        parse_generic(src, |p| grammar::statement_grammar::expression_stmt(p, true)).ok().unwrap();
    let Expression::CallStatement(call) = stmt.expression().unwrap() else {
        panic!("expected CallStatement");
    };

    let callee = call.callee().expect("callee should be present");
    assert_eq!(callee.ident_token().unwrap().text(), "reset");

    // No argument list when there are no arguments
    assert!(call.argument_list().is_none());
}
