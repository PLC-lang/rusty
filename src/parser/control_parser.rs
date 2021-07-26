// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::ast::*;
use crate::lexer::Token::*;
use crate::parser::{parse_any_in_region, parse_body_in_region};
use crate::Diagnostic;

use super::ParseSession;
use super::{parse_primary_expression, parse_reference, parse_statement};

pub fn parse_control_statement(lexer: &mut ParseSession) -> Statement {
    match lexer.token {
        KeywordIf => parse_if_statement(lexer),
        KeywordFor => parse_for_statement(lexer),
        KeywordWhile => parse_while_statement(lexer),
        KeywordRepeat => parse_repeat_statement(lexer),
        KeywordCase => parse_case_statement(lexer),
        _ => parse_statement(lexer),
    }
}

fn parse_if_statement(lexer: &mut ParseSession) -> Statement {
    let start = lexer.range().start;
    lexer.advance(); //If
    let mut conditional_blocks = vec![];

    while lexer.last_token == KeywordElseIf || lexer.last_token == KeywordIf {
        let condition = parse_primary_expression(lexer);
        if !lexer.expect_token(KeywordThen) {
            return Statement::EmptyStatement {
                location: lexer.location(),
            };
        }
        lexer.advance();

        let condition_block = ConditionalBlock {
            condition: Box::new(condition),
            body: parse_body_in_region(lexer, vec![KeywordEndIf, KeywordElseIf, KeywordElse]),
        };

        conditional_blocks.push(condition_block);
    }

    let mut else_block = Vec::new();

    if lexer.last_token == KeywordElse {
        else_block.append(&mut parse_body_in_region(lexer, vec![KeywordEndIf]));
    }

    let end = lexer.last_range.end;

    Statement::IfStatement {
        blocks: conditional_blocks,
        else_block,
        location: SourceRange::new(start..end),
    }
}

fn parse_for_statement(lexer: &mut ParseSession) -> Statement {
    let start = lexer.range().start;
    lexer.advance(); // FOR

    let counter_expression = parse_reference(lexer);
    if !lexer.expect_token(KeywordAssignment) {
        return Statement::EmptyStatement {
            location: lexer.location(),
        };
    }
    lexer.advance();

    let start_expression = parse_primary_expression(lexer);
    if !lexer.expect_token(KeywordTo) {
        return Statement::EmptyStatement {
            location: lexer.location(),
        };
    }
    lexer.advance();
    let end_expression = parse_primary_expression(lexer);

    let step = if lexer.token == KeywordBy {
        lexer.advance(); // BY
        Some(Box::new(parse_primary_expression(lexer)))
    } else {
        None
    };

    lexer.consume_or_report(KeywordDo); // DO

    Statement::ForLoopStatement {
        counter: Box::new(counter_expression),
        start: Box::new(start_expression),
        end: Box::new(end_expression),
        by_step: step,
        body: parse_body_in_region(lexer, vec![KeywordEndFor]),
        location: SourceRange::new(start..lexer.last_range.end),
    }
}

fn parse_while_statement(lexer: &mut ParseSession) -> Statement {
    let start = lexer.range().start;
    lexer.advance(); //WHILE

    let condition = parse_primary_expression(lexer);
    lexer.consume_or_report(KeywordDo);

    Statement::WhileLoopStatement {
        condition: Box::new(condition),
        body: parse_body_in_region(lexer, vec![KeywordEndWhile]),
        location: SourceRange::new(start..lexer.last_range.end),
    }
}

fn parse_repeat_statement(lexer: &mut ParseSession) -> Statement {
    let start = lexer.range().start;
    lexer.advance(); //REPEAT

    let body = parse_body_in_region(lexer, vec![KeywordUntil, KeywordEndRepeat]); //UNTIL
    let condition = if lexer.last_token == KeywordUntil {
        parse_any_in_region(lexer, vec![KeywordEndRepeat], |lexer| {
            parse_primary_expression(lexer)
        })
    } else {
        Statement::EmptyStatement {
            location: lexer.location(),
        }
    };

    Statement::RepeatLoopStatement {
        condition: Box::new(condition),
        body,
        location: SourceRange::new(start..lexer.range().end),
    }
}

fn parse_case_statement(lexer: &mut ParseSession) -> Statement {
    let start = lexer.range().start;
    lexer.advance(); // CASE

    let selector = Box::new(parse_primary_expression(lexer));

    if !lexer.expect_token(KeywordOf) {
        return Statement::EmptyStatement {
            location: lexer.location(),
        };
    } // OF
    lexer.advance();

    let mut case_blocks = Vec::new();
    if lexer.token != KeywordEndCase && lexer.token != KeywordElse {
        let body = parse_body_in_region(lexer, vec![KeywordEndCase, KeywordElse]);

        let mut current_condition = None;
        let mut current_body = vec![];
        for statement in body {
            if let Statement::CaseCondition { condition } = statement {
                if let Some(condition) = current_condition {
                    let block = ConditionalBlock {
                        condition,
                        body: current_body,
                    };
                    case_blocks.push(block);
                    current_body = vec![];
                }
                current_condition = Some(condition);
            } else {
                //If no current condition is available, log a diagnostic and add an empty condition
                if current_condition.is_none() {
                    lexer.accept_diagnostic(Diagnostic::syntax_error(
                        "Missing Case-Condition".into(),
                        lexer.location(),
                    ));
                    current_condition = Some(Box::new(Statement::EmptyStatement {
                        location: lexer.location(),
                    }));
                }
                current_body.push(statement);
            }
        }
        if let Some(condition) = current_condition {
            let block = ConditionalBlock {
                condition,
                body: current_body,
            };
            case_blocks.push(block);
        }
    }

    let else_block = if lexer.last_token == KeywordElse {
        parse_body_in_region(lexer, vec![KeywordEndCase])
    } else {
        vec![]
    };

    let end = lexer.last_range.end;
    Statement::CaseStatement {
        selector,
        case_blocks,
        else_block,
        location: SourceRange::new(start..end),
    }
}
