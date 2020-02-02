use crate::lexer;
use crate::lexer::Token::*;
use crate::expect;
use crate::ast::*;

use super::RustyLexer;
use super::{parse_statement_or_case_label, parse_body, parse_expression, parse_reference };

#[cfg(test)]
mod tests;

pub fn parse_control_statement(lexer: &mut RustyLexer) -> Result<Statement, String> {
    return match lexer.token {
        KeywordIf => parse_if_statement(lexer),
        KeywordFor => parse_for_statement(lexer),
        KeywordWhile => parse_while_statement(lexer),
        KeywordRepeat => parse_repeat_statement(lexer),
        KeywordCase => parse_case_statement(lexer),
        _ => parse_statement_or_case_label(lexer),
    }
}

fn parse_if_statement(lexer: &mut RustyLexer) -> Result<Statement, String> {
    
    let end_of_body = | it : &lexer::Token | 
                                *it == KeywordElseIf
                            || *it == KeywordElse
                            || *it == KeywordEndIf;

    
    let mut conditional_blocks = vec![];

    while lexer.token == KeywordElseIf || lexer.token == KeywordIf{
        lexer.advance();//If//ElseIf
        let condition = parse_expression(lexer);
        expect!(KeywordThen, lexer);
        lexer.advance();
        let body = parse_body(lexer, &end_of_body);

        let condition_block = ConditionalBlock {
            condition: Box::new(condition?),
            body: body?,
        };

        conditional_blocks.push(condition_block);
    }
    
    let mut else_block = Vec::new();

    if lexer.token == KeywordElse {
        lexer.advance(); // else
        else_block.append(&mut parse_body(lexer, &|it| *it == KeywordEndIf)?)
    }
    lexer.advance();
    
    Ok(Statement::IfStatement{blocks: conditional_blocks, else_block: else_block})   
}

fn parse_for_statement(lexer: &mut RustyLexer) -> Result<Statement, String> {
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
    lexer.advance();

    let body = parse_body(
                    lexer, 
                    &|t: &lexer::Token| *t == KeywordEndFor);
    lexer.advance();

    Ok(Statement::ForLoopStatement{counter: Box::new(counter_expression), start: Box::new(start_expression), end: Box::new(end_expression), by_step: step, body: body?})
}

fn parse_while_statement(lexer: &mut RustyLexer) -> Result<Statement, String> {
    lexer.advance(); //WHILE

    let condition = Box::new(parse_expression(lexer)?);

    expect!(KeywordDo, lexer); // DO
    lexer.advance();
    
    let body = parse_body(
                    lexer,
                    &|t: &lexer::Token| *t == KeywordEndWhile)?;
    lexer.advance();

    Ok(Statement::WhileLoopStatement{ condition, body })
}

fn parse_repeat_statement(lexer: &mut RustyLexer) -> Result<Statement, String> {
    lexer.advance(); //REPEAT
    
    let body = parse_body(
        lexer,
        &|t: &lexer::Token| *t == KeywordUntil)?; //UNTIL
    lexer.advance();

    let condition = Box::new(parse_expression(lexer)?);

    expect!(KeywordEndRepeat, lexer); // END_REPEAT
    lexer.advance();
    
    Ok(Statement::RepeatLoopStatement{ condition, body })
}

fn parse_case_statement(lexer: &mut RustyLexer) -> Result<Statement, String> {
    lexer.advance(); // CASE

    let selector = Box::new(parse_expression(lexer)?);

    expect!(KeywordOf, lexer); // OF
    lexer.advance();

    //TODO case-stmt without a body will crash
    //parse the first condition (parse ahead)
    let mut condition = Some(parse_expression(lexer)?);
    expect!(KeywordColon, lexer); // :
    lexer.advance();
    
    let mut case_blocks = Vec::new();
    while lexer.token != KeywordEndCase && lexer.token != KeywordElse && condition.is_some() {
        let (body, next_condition) = parse_case_body_with_label( condition.unwrap(), lexer)?;
        condition = next_condition;
        case_blocks.push(body);
    }
    lexer.advance();
    let else_block = Vec::new();

    Ok(Statement::CaseStatement{ selector, case_blocks, else_block })
}

/**
 * returns a case-body (limited by either END_CASE, ELSE or another case-label <xxx> : ) combined in a tuple 
 * with an optional following case-label (the condition of the next case-body)
 */
fn parse_case_body_with_label(condition: Statement, lexer: &mut RustyLexer) -> Result<(ConditionalBlock, Option<Statement>), String> {
    let mut body = parse_body(lexer, &|t: &lexer::Token| *t == KeywordEndCase || *t == KeywordColon || *t == KeywordElse )?;
    if lexer.token == KeywordColon {
        lexer.advance();
        // the block was ended with a new case-label (e.g. '2:')
        // so we add the block and return the next block's condition
        // because we already parsed it 
        if body.is_empty() {
            return Err("Unexpected ':' - no case-label could be found.".to_string());
        }

        //
        let next_condition = body.remove(body.len() - 1);
        let block = ConditionalBlock { condition: Box::new(condition), body };
        let block_and_next_condition = (block, Some(next_condition));
        return Ok(block_and_next_condition);
    } 

    //this is either END_CASE or ELSE
    let block = ConditionalBlock { condition: Box::new(condition), body };
    let block_without_next_condition = (block, None);
    return Ok( block_without_next_condition );
}
