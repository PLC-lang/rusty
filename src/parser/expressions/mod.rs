use crate::lexer::Token::*;
use crate::expect;
use crate::ast::*;

use super::RustyLexer;
use super::{unexpected_token,slice_and_advance};

#[cfg(test)]
mod tests;

pub fn parse_primary_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    parse_equality_expression(lexer)
}

fn parse_equality_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let left = parse_compare_expression(lexer)?;
    let operator = match lexer.token {
        OperatorEqual => Operator::Equal,
        OperatorNotEqual => Operator::NotEqual,
        _ => return Ok(left),
    };
    lexer.advance();
    let right = parse_equality_expression(lexer)?;
    Ok(Statement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
    })
}

fn parse_compare_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let left = parse_additive_expression(lexer)?;
    let operator = match lexer.token {
        OperatorLess => Operator::Less,
        OperatorGreater => Operator::Greater,
        OperatorLessOrEqual => Operator::LessOrEqual,
        OperatorGreaterOrEqual => Operator::GreaterOrEqual,
        _ => return Ok(left),
    };
    lexer.advance();
    let right = parse_compare_expression(lexer)?;
    Ok(Statement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
    })
}

fn parse_additive_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let left = parse_multiplication_expression(lexer)?;
    let operator = match lexer.token {
        OperatorPlus => Operator::Plus,
        OperatorMinus => Operator::Minus,
        _ => return Ok(left),
    };
    lexer.advance();
    let right = parse_additive_expression(lexer)?;
    Ok(Statement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
    })
}

fn parse_multiplication_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let left = parse_boolean_expression(lexer)?;
    let operator = match lexer.token {
        OperatorMultiplication => Operator::Multiplication,
        OperatorDivision => Operator::Division,
        OperatorModulo => Operator::Modulo,
        _ => return Ok(left),
    };
    lexer.advance();
    let right = parse_multiplication_expression(lexer)?;
    Ok(Statement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
    })
}

fn parse_boolean_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let current = parse_parenthesized_expression(lexer);
    let operator = match lexer.token {
        OperatorAnd => Some(Operator::And),
        OperatorOr => Some(Operator::Or),
        OperatorXor => Some(Operator::Xor),
        _ => None,
    };

    if let Some(operator) = operator {
        lexer.advance();
        return Ok(Statement::BinaryExpression {
            operator,
            left: Box::new(current?),
            right: Box::new(parse_primary_expression(lexer)?),
        });
    }
    current
}

fn parse_parenthesized_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    match lexer.token {
        KeywordParensOpen => {
            lexer.advance();
            let result = parse_primary_expression(lexer);
            expect!(KeywordParensClose, lexer);
            lexer.advance();
            result
        }
        _ => parse_unary_expression(lexer),
    }
}

fn parse_unary_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let operator = match lexer.token {
        OperatorNot => Some(Operator::Not),
        OperatorMinus => Some(Operator::Minus),
        _ => None,
    };
    if let Some(operator) = operator {
        lexer.advance();
        Ok(Statement::UnaryExpression {
            operator: operator,
            value: Box::new(parse_parenthesized_expression(lexer)?),
        })
    } else {
        parse_leaf_expression(lexer)
    }
}

fn parse_leaf_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let current = match lexer.token {
        Identifier => parse_reference(lexer),
        LiteralNumber => parse_literal_number(lexer),
        LiteralTrue => parse_bool_literal(lexer, true),
        LiteralFalse => parse_bool_literal(lexer, false),
        _ => Err(unexpected_token(lexer)),
    };

    if current.is_ok() && lexer.token == KeywordAssignment {
        lexer.advance();
        return Ok(Statement::Assignment {
            left: Box::new(current?),
            right: Box::new(parse_primary_expression(lexer)?),
        });
    };
    current
}

fn parse_bool_literal(lexer: &mut RustyLexer, value: bool) -> Result<Statement, String> {
    lexer.advance();
    Ok(Statement::LiteralBool { value })
}

fn parse_reference(lexer: &mut RustyLexer) -> Result<Statement, String> {
    Ok(Statement::Reference {
        name: slice_and_advance(lexer).to_string(),
    })
}

fn parse_literal_number(lexer: &mut RustyLexer) -> Result<Statement, String> {
    Ok(Statement::LiteralNumber {
        value: slice_and_advance(lexer),
    })
}

