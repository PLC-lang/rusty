use crate::lexer;
use crate::lexer::Token::*;
use crate::expect;
use crate::ast::*;

use super::RustyLexer;
use super::{parse_statement, parse_body, parse_expression };

#[cfg(test)]
mod tests;

pub fn parse_control_statement(lexer: &mut RustyLexer) -> Result<Statement, String> {
    return match lexer.token {
        KeywordIf => parse_if_statement(lexer),
        KeywordFor => parse_for_statement(lexer),
        _ => parse_statement(lexer),
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

    Ok(Statement::ForLoopStatement{ start: Box::new(start_expression), end: Box::new(end_expression), by_step: step, body: body?})
}