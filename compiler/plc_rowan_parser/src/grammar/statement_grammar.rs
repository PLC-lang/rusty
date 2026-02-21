use crate::grammar::name_ref;
use crate::parser::Parser;
use crate::SyntaxKind::*;
use crate::T;

/// Returns true if the current token can start an ExpressionStmt.
fn at_expression_start(p: &Parser) -> bool {
    matches!(
        p.current(),
        IF_KW
            | FOR_KW
            | WHILE_KW
            | INT_NUMBER
            | FLOAT_NUMBER
            | BOOL_LITERAL
            | STRING_LITERAL
            | CHAR
            | STRING
            | W_STRING
            | IDENT
    )
}

/// Parse a Body = ExpressionStmt*
pub fn body(p: &mut Parser) {
    let m = p.start();
    while at_expression_start(p) {
        expression_stmt(p, true);
    }
    m.complete(p, BODY);
}

/// Parse an ExpressionStmt = Expression ';'?
/// Wraps any expression in an EXPRESSION_STMT node.
pub fn expression_stmt(p: &mut Parser, eat_semicolon: bool) {
    let m = p.start();
    expression(p);
    if eat_semicolon {
        p.eat(T![;]);
    }
    m.complete(p, EXPRESSION_STMT);
}

/// Parse a single Expression (dispatches to the right sub-parser).
pub fn expression(p: &mut Parser) {
    match p.current() {
        IF_KW => if_statement(p),
        FOR_KW => for_statement(p),
        WHILE_KW => while_statement(p),
        // IDENT followed by ':=' is an assignment; otherwise it's a name reference (literal).
        IDENT if p.nth(1) == ASSIGN => assignment(p),
        _ if p.current().is_literal() => literal(p),
        _ if p.current() == IDENT => name_ref(p),
        _ => {
            p.error("expected expression");
        }
    }
}

// IfStatement =
//   'IF' condition:ConditionThenBlock ElseIfArm* ElseArm? 'END_IF'
pub fn if_statement(p: &mut Parser) {
    let m = p.start();
    p.bump(IF_KW);
    condition_then_block(p);
    while p.at(ELSIF_KW) {
        else_if_arm(p);
    }
    if p.at(ELSE_KW) {
        else_arm(p);
    }
    p.expect(END_IF_KW);
    m.complete(p, IF_STATEMENT);
}

// ConditionThenBlock = condition:ExpressionStmt 'THEN' then_branch:Body
fn condition_then_block(p: &mut Parser) {
    let m = p.start();
    expression_stmt(p, false);
    p.expect(THEN_KW);
    body(p);
    m.complete(p, CONDITION_THEN_BLOCK);
}

// ElseIfArm = 'ELSIF' condition:ConditionThenBlock
fn else_if_arm(p: &mut Parser) {
    let m = p.start();
    p.bump(ELSIF_KW);
    condition_then_block(p);
    m.complete(p, ELSE_IF_ARM);
}

// ElseArm = 'ELSE' ExpressionStmt
fn else_arm(p: &mut Parser) {
    let m = p.start();
    p.bump(ELSE_KW);
    // The ELSE body is a sequence of statements, represented as a Body in the
    // grammar — but the ungram models it as a single ExpressionStmt. We parse
    // all available statements here by looping.
    
    while !p.at(END_IF_KW){
        expression_stmt(p, true);
    }
    m.complete(p, ELSE_ARM);
}

// ForStatement =
//   'FOR' counter:Assignment 'TO' end:ExpressionStmt ('BY' step:ExpressionStmt)? 'DO' body:Body 'END_FOR'
pub fn for_statement(p: &mut Parser) {
    let m = p.start();
    p.bump(FOR_KW);
    // counter: IDENT ':=' value — no trailing ';' in the FOR header
    for_counter(p);
    p.expect(TO_KW);
    expression_stmt(p, false); // end value
    if p.eat(BY_KW) {
        expression_stmt(p, false); // step value
    }
    p.expect(DO_KW);
    body(p);
    p.expect(END_FOR_KW);
    m.complete(p, FOR_STATEMENT);
}

/// Counter init in a FOR loop: `IDENT ':=' value` (no trailing ';').
/// Emitted as an ASSIGNMENT node.
fn for_counter(p: &mut Parser) {
    let m = p.start();
    let name_m = p.start();
    p.expect(IDENT);
    name_m.complete(p, NAME);
    p.expect(T![:=]);
    // value — no semicolon, no EXPRESSION_STMT wrapper (it's inline in the FOR header)
    value_expression(p);
    m.complete(p, ASSIGNMENT);
}

// WhileStatement =
//   'WHILE' condition:ExpressionStmt 'DO' body:Body 'END_WHILE'
pub fn while_statement(p: &mut Parser) {
    let m = p.start();
    p.bump(WHILE_KW);
    expression_stmt(p, false); // condition
    p.expect(DO_KW);
    body(p);
    p.expect(END_WHILE_KW);
    m.complete(p, WHILE_STATEMENT);
}

// Assignment = target:Name ':=' value:ExpressionStmt ';'
// The ';' is consumed inside expression_stmt when this is called from body().
// When called standalone (e.g. as a body-level statement) the wrapping
// expression_stmt() already ate the ';', so assignment itself only emits
// NAME ':=' value — and the outer expression_stmt handles the semicolon.
pub fn assignment(p: &mut Parser) {
    let m = p.start();
    let name_m = p.start();
    p.expect(IDENT);
    name_m.complete(p, NAME);
    p.expect(T![:=]);
    // Parse the RHS — wrap it in EXPRESSION_STMT to match the grammar
    let val_m = p.start();
    value_expression(p);
    // No semicolon here: expression_stmt() called from body() will eat it
    // after assignment() returns and the EXPRESSION_STMT is completed.
    val_m.complete(p, EXPRESSION_STMT);
    m.complete(p, ASSIGNMENT);
}

/// Parse a value expression (RHS of assignment) — no `:=` recursion.
fn value_expression(p: &mut Parser) {
    match p.current() {
        IF_KW => if_statement(p),
        FOR_KW => for_statement(p),
        WHILE_KW => while_statement(p),
        _ if p.current().is_literal() || p.at(IDENT) => literal(p),
        _ => {
            p.error("expected expression");
        }
    }
}

// Literal = value:(int | float | string | bool | char | …)
fn literal(p: &mut Parser) {
    let m = p.start();
    p.bump_any();
    m.complete(p, LITERAL);
}

#[cfg(test)]
mod tests {
    use crate::{
        grammar::{statement_grammar::body, tests::{format_tree, parse_with}},
        lexed_str::LexedStr,
    };

    fn parse_body(input: &str) -> String {
        let lexed = LexedStr::new(input);
        let output = parse_with(&lexed, body);
        format_tree(&output, &lexed)
    }

    // -----------------------------------------------------------------------
    // Literal
    // -----------------------------------------------------------------------

    #[test]
    fn parse_literal_int() {
        insta::assert_snapshot!(parse_body("42;"));
    }

    // -----------------------------------------------------------------------
    // Assignment
    // -----------------------------------------------------------------------

    #[test]
    fn parse_assignment_snapshot() {
        insta::assert_snapshot!(parse_body("x := 42;"));
    }

    // -----------------------------------------------------------------------
    // WhileStatement
    // -----------------------------------------------------------------------

    #[test]
    fn parse_while_snapshot() {
        insta::assert_snapshot!(parse_body("WHILE x DO\n  y := 1;\nEND_WHILE"));
    }

    // -----------------------------------------------------------------------
    // IfStatement
    // -----------------------------------------------------------------------

    #[test]
    fn parse_if_snapshot() {
        insta::assert_snapshot!(parse_body("IF x THEN\n  y := 1;\nEND_IF"));
    }

    #[test]
    fn parse_if_elsif_else_snapshot() {
        insta::assert_snapshot!(parse_body(
            "IF a THEN\n  x := 1;\nELSIF b THEN\n  x := 2;\nELSE\n  x := 3;\nEND_IF"
        ));
    }

    // -----------------------------------------------------------------------
    // ForStatement
    // -----------------------------------------------------------------------

    #[test]
    fn parse_for_snapshot() {
        insta::assert_snapshot!(parse_body(
            "FOR nCounter := 1 TO 5 BY 1 DO\n  nVar1 := nVar1;\nEND_FOR"
        ));
    }

    #[test]
    fn parse_for_without_by_snapshot() {
        insta::assert_snapshot!(parse_body(
            "FOR i := 0 TO 10 DO\n  x := i;\nEND_FOR"
        ));
    }
}

