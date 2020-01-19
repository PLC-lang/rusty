use crate::lexer;
use crate::lexer::Token::*;
use crate::expect;
use crate::ast::*;

use super::RustyLexer;
use super::{parse_statement, parse_body, parse_expression};

#[cfg(test)]
mod tests;

pub fn parse_control_statement(lexer: &mut RustyLexer) -> Result<Statement, String> {
    if lexer.token == KeywordIf {
        return parse_if_statement(lexer);
    }
    parse_statement(lexer)
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