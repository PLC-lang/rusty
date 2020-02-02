use crate::lexer;
use logos::Lexer;

use crate::ast::CompilationUnit;
use crate::ast::PrimitiveType;
use crate::ast::Program;
use crate::ast::Statement;
use crate::ast::Type;
use crate::ast::Variable;
use crate::ast::VariableBlock;
use crate::lexer::Token::*;

use expressions::parse_primary_expression;
use control::parse_control_statement;

mod expressions;
mod control;

#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! expect {
    ( $token:expr, $lexer:expr) => {
        if $lexer.token != $token {
            return Err(format!("expected {:?}, but found {:?}", $token, $lexer.token).to_string());
        }
    };
}

type RustyLexer<'a> = Lexer<lexer::Token, &'a str>;

fn create_program() -> Program {
    Program {
        name: "".to_string(),
        variable_blocks: Vec::new(),
        statements: Vec::new(),
    }
}

///
/// returns an error for an uidientified token
///  
fn unidentified_token(lexer: &RustyLexer) -> String {
    format!(
        "unidentified token: {t:?} at {location:?}",
        t = lexer.slice(),
        location = lexer.range()
    )
}

///
/// returns an error for an unexpected token
///  
fn unexpected_token(lexer: &RustyLexer) -> String {
    format!(
        "unexpected token: {t:?} at {location:?}",
        t = lexer.token,
        location = lexer.range()
    )
}

fn slice_and_advance(lexer: &mut RustyLexer) -> String {
    let slice = lexer.slice().to_string();
    lexer.advance();
    slice
}

pub fn parse(mut lexer: RustyLexer) -> Result<CompilationUnit, String> {
    let mut unit = CompilationUnit { units: Vec::new() };

    loop {
        match lexer.token {
            KeywordProgram => {
                let program = parse_program(&mut lexer);
                match program {
                    Ok(p) => unit.units.push(p),

                    Err(msg) => return Err(msg),
                };
            }
            End => return Ok(unit),
            Error => return Err(unidentified_token(&lexer)),
            _ => return Err(unexpected_token(&lexer)),
        };

        lexer.advance();
    }
    //the match in the loop will always return
}

fn parse_program(lexer: &mut RustyLexer) -> Result<Program, String> {
    lexer.advance(); //Consume ProgramKeyword
    let mut result = create_program();
    expect!(Identifier, lexer);

    //Parse Identifier
    result.name = slice_and_advance(lexer);

    //Parse variable declarations
    while lexer.token == KeywordVar {
        let block = parse_variable_block(lexer);
        match block {
            Ok(b) => result.variable_blocks.push(b),
            Err(msg) => return Err(msg),
        };
    }

    //Parse the statemetns
    let mut body = parse_body(lexer, &|it| *it == KeywordEndProgram)?;
    result.statements.append(&mut body);

    Ok(result)
}

fn is_end_of_stream(token: &lexer::Token) -> bool {
    *token == End || *token == Error 
}

fn parse_body(lexer: &mut RustyLexer, until: &dyn Fn(&lexer::Token) -> bool) -> Result<Vec<Statement>, String> {
    let mut statements = Vec::new();
    while !until(&lexer.token) && !is_end_of_stream(&lexer.token) {
        let statement = parse_control(lexer)?;
        statements.push(statement);
    }
    if !until(&lexer.token) {
        return Err(format!("unexpected end of body {:?}", lexer.token).to_string());
    }
    Ok(statements)
}

/**
 * parses either an expression (ended with ';' or a case-label ended with ':')
 * a case-label will not consume the ":"!!! (weird but necessary?)
 */
fn parse_statement_or_case_label(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let result = parse_expression(lexer);
 
    if lexer.token == KeywordColon {
        return result;
    }

    expect!(KeywordSemicolon, lexer);
    lexer.advance();
    result
}

fn parse_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    parse_primary_expression(lexer)
}

fn parse_reference(lexer: &mut RustyLexer) -> Result<Statement, String> {
    expressions::parse_reference(lexer)
}

fn parse_control(lexer : &mut RustyLexer) -> Result<Statement, String> {
    parse_control_statement(lexer)
}

fn parse_variable_block(lexer: &mut RustyLexer) -> Result<VariableBlock, String> {
    lexer.advance(); //Consume VarBlock
    let mut result = VariableBlock {
        variables: Vec::new(),
    };

    while lexer.token == Identifier {
        result = parse_variable(lexer, result)?;
    }

    expect!(KeywordEndVar, lexer);

    lexer.advance();
    Ok(result)
}

fn parse_variable(
    lexer: &mut RustyLexer,
    mut owner: VariableBlock,
) -> Result<VariableBlock, String> {
    let name = slice_and_advance(lexer);

    expect!(KeywordColon, lexer);
    lexer.advance();

    expect!(Identifier, lexer);
    let data_type = slice_and_advance(lexer);
    //Convert to real datatype

    expect!(KeywordSemicolon, lexer);
    lexer.advance();

    owner.variables.push(Variable {
        name,
        data_type: get_data_type(data_type),
    });
    Ok(owner)
}

fn get_data_type(name: String) -> Type {
    let prim_type = match name.to_lowercase().as_str() {
        "int" => Some(PrimitiveType::Int),
        "bool" => Some(PrimitiveType::Bool),
        _ => None,
    };

    if let Some(prim_type) = prim_type {
        Type::Primitive(prim_type)
    } else {
        Type::Custom
    }
}


