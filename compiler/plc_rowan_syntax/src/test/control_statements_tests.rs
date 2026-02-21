use indoc::indoc;
use plc_rowan_parser::{grammar, SyntaxKind};

use crate::{
    ast::{AstNode, CompilationUnit, Expression, ExpressionStmt},
    test::test_util::parse_generic,
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
    (if_stmt.IF_token().unwrap().to_string());
    assert!(if_stmt.END_IF_token().is_some());
    // if_condition is the ConditionThenBlock holding the condition + THEN branch
    let if_cond = if_stmt.if_condition().expect("expected if_condition");
    assert!(if_cond.THEN_token().is_some());

    //TODO: wtf?
    let cond_text = if_cond
        .syntax()
        .children()
        .find(|n| n.kind() == SyntaxKind::EXPRESSION_STMT)
        .expect("expected ExpressionStmt condition child")
        .text()
        .to_string();
    assert!(cond_text.contains("cond"));
    // ELSE arm is present, no ELSIF arms
    assert!(if_stmt.else_arm().is_some());
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
