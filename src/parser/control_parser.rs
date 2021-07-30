// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::ast::*;
use crate::lexer::Token::*;
use crate::parser::parse_body_in_region;
use crate::parser::parse_statement_in_region;
use crate::Diagnostic;

use super::ParseSession;
use super::{parse_expression, parse_reference, parse_statement};

pub fn parse_control_statement(lexer: &mut ParseSession) -> Result<Statement, Diagnostic> {
    match lexer.token {
        KeywordIf => parse_if_statement(lexer),
        KeywordFor => parse_for_statement(lexer),
        KeywordWhile => parse_while_statement(lexer),
        KeywordRepeat => parse_repeat_statement(lexer),
        KeywordCase => parse_case_statement(lexer),
        _ => parse_statement(lexer),
    }
}

fn parse_if_statement(lexer: &mut ParseSession) -> Result<Statement, Diagnostic> {
    let start = lexer.range().start;
    lexer.advance(); //If
    let mut conditional_blocks = vec![];

    while lexer.last_token == KeywordElseIf || lexer.last_token == KeywordIf {
        let condition = parse_expression(lexer);
        lexer.expect(KeywordThen)?;
        lexer.advance();
        let body = parse_body_in_region(lexer, vec![KeywordEndIf, KeywordElseIf, KeywordElse]);

        let condition_block = ConditionalBlock {
            condition: Box::new(condition?),
            body: body?,
        };

        conditional_blocks.push(condition_block);
    }

    let mut else_block = Vec::new();

    if lexer.last_token == KeywordElse {
        else_block.append(&mut parse_body_in_region(lexer, vec![KeywordEndIf])?);
    }

    let end = lexer.last_range.end;

    Ok(Statement::IfStatement {
        blocks: conditional_blocks,
        else_block,
        location: SourceRange::new(start..end),
        id: lexer.next_id(),
    })
}

fn parse_for_statement(lexer: &mut ParseSession) -> Result<Statement, Diagnostic> {
    let start = lexer.range().start;
    lexer.advance(); // FOR

    let counter_expression = parse_reference(lexer)?;
    lexer.expect(KeywordAssignment)?; // :=
    lexer.advance();

    let start_expression = parse_expression(lexer)?;

    lexer.expect(KeywordTo)?; // TO
    lexer.advance();
    let end_expression = parse_expression(lexer)?;

    let step = if lexer.token == KeywordBy {
        lexer.advance(); // BY
        Some(Box::new(parse_expression(lexer)?))
    } else {
        None
    };

    lexer.consume_or_report(KeywordDo); // DO

    let body = parse_body_in_region(lexer, vec![KeywordEndFor]);
    Ok(Statement::ForLoopStatement {
        counter: Box::new(counter_expression),
        start: Box::new(start_expression),
        end: Box::new(end_expression),
        by_step: step,
        body: body?,
        location: SourceRange::new(start..lexer.last_range.end),
        id: lexer.next_id(),
    })
}

fn parse_while_statement(lexer: &mut ParseSession) -> Result<Statement, Diagnostic> {
    let start = lexer.range().start;
    lexer.advance(); //WHILE

    let start_condition = lexer.range().start;
    let condition = match parse_expression(lexer) {
        Ok(condition) => condition,
        Err(diagnostic) => {
            lexer.accept_diagnostic(diagnostic);
            Statement::EmptyStatement {
                location: (start_condition..lexer.range().end).into(),
                id: lexer.next_id(),
            }
        }
    };
    lexer.consume_or_report(KeywordDo);

    let body = parse_body_in_region(lexer, vec![KeywordEndWhile])?;
    Ok(Statement::WhileLoopStatement {
        condition: Box::new(condition),
        body,
        location: SourceRange::new(start..lexer.last_range.end),
        id: lexer.next_id(),
    })
}

fn parse_repeat_statement(lexer: &mut ParseSession) -> Result<Statement, Diagnostic> {
    let start = lexer.range().start;
    lexer.advance(); //REPEAT

    let body = parse_body_in_region(lexer, vec![KeywordUntil, KeywordEndRepeat])?; //UNTIL
    let condition = if lexer.last_token == KeywordUntil {
        parse_statement_in_region(lexer, vec![KeywordEndRepeat], |lexer| {
            parse_expression(lexer)
        })
    } else {
        Statement::EmptyStatement {
            location: lexer.location(),
            id: lexer.next_id(),
        }
    };

    Ok(Statement::RepeatLoopStatement {
        condition: Box::new(condition),
        body,
        location: SourceRange::new(start..lexer.range().end),
        id: lexer.next_id(),
    })
}

fn parse_case_statement(lexer: &mut ParseSession) -> Result<Statement, Diagnostic> {
    let start = lexer.range().start;
    lexer.advance(); // CASE

    let selector = Box::new(parse_expression(lexer)?);

    lexer.expect(KeywordOf)?; // OF
    lexer.advance();

    let mut case_blocks = Vec::new();
    if lexer.token != KeywordEndCase && lexer.token != KeywordElse {
        let body = parse_body_in_region(lexer, vec![KeywordEndCase, KeywordElse])?;

        let mut current_condition = None;
        let mut current_body = vec![];
        for statement in body {
            if let Statement::CaseCondition { condition, .. } = statement {
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
                        id: lexer.next_id(),
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
        parse_body_in_region(lexer, vec![KeywordEndCase])?
    } else {
        vec![]
    };

    let end = lexer.last_range.end;
    Ok(Statement::CaseStatement {
        selector,
        case_blocks,
        else_block,
        location: SourceRange::new(start..end),
        id: lexer.next_id(),
    })
}
