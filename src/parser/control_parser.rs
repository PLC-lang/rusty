use crate::ast::*;
use crate::expect;
/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::lexer;
use crate::lexer::Token::*;

use super::RustyLexer;
use super::{parse_body, parse_expression, parse_reference, parse_statement};

pub fn parse_control_statement(lexer: &mut RustyLexer) -> Result<Statement, String> {
    return match lexer.token {
        KeywordIf => parse_if_statement(lexer),
        KeywordFor => parse_for_statement(lexer),
        KeywordWhile => parse_while_statement(lexer),
        KeywordRepeat => parse_repeat_statement(lexer),
        KeywordCase => parse_case_statement(lexer),
        _ => parse_statement(lexer),
    };
}

fn parse_if_statement(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let start = lexer.range().start;
    let end_of_body =
        |it: &lexer::Token| *it == KeywordElseIf || *it == KeywordElse || *it == KeywordEndIf;

    let mut conditional_blocks = vec![];

    while lexer.token == KeywordElseIf || lexer.token == KeywordIf {
        let line_nr = lexer.get_current_line_nr();
        lexer.advance(); //If//ElseIf
        let condition = parse_expression(lexer);
        expect!(KeywordThen, lexer);
        lexer.advance();
        let body = parse_body(lexer, line_nr, &end_of_body);

        let condition_block = ConditionalBlock {
            condition: Box::new(condition?),
            body: body?,
        };

        conditional_blocks.push(condition_block);
    }

    let mut else_block = Vec::new();

    if lexer.token == KeywordElse {
        let line_nr = lexer.get_current_line_nr();
        lexer.advance(); // else
        else_block.append(&mut parse_body(lexer, line_nr, &|it| *it == KeywordEndIf)?)
    }

    let end = lexer.range().end;
    lexer.advance();

    Ok(Statement::IfStatement {
        blocks: conditional_blocks,
        else_block: else_block,
        location: start..end,
    })
}

fn parse_for_statement(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let start = lexer.range().start;
    lexer.advance(); // FOR

    let counter_expression = parse_reference(lexer).unwrap();
    expect!(KeywordAssignment, lexer); // :=
    lexer.advance();

    let start_expression = parse_expression(lexer).unwrap();

    expect!(KeywordTo, lexer); // TO
    lexer.advance();
    let end_expression = parse_expression(lexer).unwrap();

    let step = if lexer.token == KeywordBy {
        lexer.advance(); // BY
        Some(Box::new(parse_expression(lexer).unwrap()))
    } else {
        None
    };

    expect!(KeywordDo, lexer); // DO
    let line_nr = lexer.get_current_line_nr();
    lexer.advance();

    let body = parse_body(lexer, line_nr, &|t: &lexer::Token| *t == KeywordEndFor);

    let end = lexer.range().end;
    lexer.advance();

    Ok(Statement::ForLoopStatement {
        counter: Box::new(counter_expression),
        start: Box::new(start_expression),
        end: Box::new(end_expression),
        by_step: step,
        body: body?,
        location: start..end,
    })
}

fn parse_while_statement(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let start = lexer.range().start;
    let line_nr = lexer.get_current_line_nr();
    lexer.advance(); //WHILE

    let condition = Box::new(parse_expression(lexer)?);

    expect!(KeywordDo, lexer); // DO
    lexer.advance();

    let body = parse_body(lexer, line_nr, &|t: &lexer::Token| *t == KeywordEndWhile)?;

    let end = lexer.range().end;
    lexer.advance();

    Ok(Statement::WhileLoopStatement {
        condition,
        body,
        location: start..end,
    })
}

fn parse_repeat_statement(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let start = lexer.range().start;
    let line_nr = lexer.get_current_line_nr();
    lexer.advance(); //REPEAT

    let body = parse_body(lexer, line_nr, &|t: &lexer::Token| *t == KeywordUntil)?; //UNTIL
    lexer.advance();

    let condition = Box::new(parse_expression(lexer)?);

    expect!(KeywordEndRepeat, lexer); // END_REPEAT
    let end = lexer.range().end;
    lexer.advance();

    Ok(Statement::RepeatLoopStatement {
        condition,
        body,
        location: start..end,
    })
}

fn parse_case_statement(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let start = lexer.range().start;
    lexer.advance(); // CASE

    let selector = Box::new(parse_expression(lexer)?);

    expect!(KeywordOf, lexer); // OF
    lexer.advance();

    let mut case_blocks = Vec::new();
    if lexer.token != KeywordEndCase && lexer.token != KeywordElse {
        let mut condition = Some(parse_expression(lexer)?);
        let case_line_nr = lexer.get_current_line_nr();
        expect!(KeywordColon, lexer); // :
        lexer.advance();

        loop {
            let (body, next_condition) =
                parse_case_body_with_condition(condition.unwrap(), lexer, case_line_nr)?;
            condition = next_condition;
            case_blocks.push(body);

            if !(lexer.token != KeywordEndCase && lexer.token != KeywordElse && condition.is_some())
            {
                break;
            }
        }
    }

    let mut else_block = Vec::new();

    if lexer.token == KeywordElse {
        let line_nr = lexer.get_current_line_nr();
        lexer.advance(); // else
        else_block.append(&mut parse_body(lexer, line_nr, &|it| {
            *it == KeywordEndCase
        })?)
    }
    let end = lexer.range().end;
    lexer.advance();

    Ok(Statement::CaseStatement {
        selector,
        case_blocks,
        else_block,
        location: start..end,
    })
}

/**
 * returns a case-body (limited by either END_CASE, ELSE or another case-condition <xxx> : ) combined in a tuple
 * with an optional following case-condition (the condition of the next case-body)
 */
fn parse_case_body_with_condition(
    condition: Statement,
    lexer: &mut RustyLexer,
    start_of_case: usize,
) -> Result<(ConditionalBlock, Option<Statement>), String> {
    let mut body = parse_body(lexer, start_of_case, &|t: &lexer::Token| {
        *t == KeywordEndCase || *t == KeywordColon || *t == KeywordElse
    })?;
    if lexer.token == KeywordColon {
        let colon_line_nr = lexer.get_current_line_nr();
        lexer.advance();
        // the block was ended with a new case-condition (e.g. '2:')
        // so we add the block and return the next block's condition
        // because we already parsed it
        if body.is_empty() {
            return Err(format!(
                "unexpected ':' at line {:} - no case-condition could be found",
                colon_line_nr
            ));
        }

        //
        let next_condition = body.remove(body.len() - 1);
        let block = ConditionalBlock {
            condition: Box::new(condition),
            body,
        };
        let block_and_next_condition = (block, Some(next_condition));
        return Ok(block_and_next_condition);
    }

    //this is either END_CASE or ELSE
    let block = ConditionalBlock {
        condition: Box::new(condition),
        body,
    };
    let block_without_next_condition = (block, None);
    return Ok(block_without_next_condition);
}
