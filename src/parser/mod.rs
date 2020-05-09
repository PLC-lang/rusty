use crate::lexer;
use logos::Lexer;

use crate::ast::*;
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

/// consumes an optional token and returns true if it was consumed.
pub fn allow(token: lexer::Token, lexer: &mut RustyLexer) -> bool {
    if lexer.token == token {
        lexer.advance();
        true
    } else {
        false
    }
}


fn create_pou(pou_type: PouType) -> POU {
    POU {
        pou_type: pou_type,
        name: "".to_string(),
        variable_blocks: Vec::new(),
        statements: Vec::new(),
        return_type: None,
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
    let mut unit = CompilationUnit { global_vars : Vec::new(), units: Vec::new() };

    loop {
        match lexer.token {
            KeywordVarGlobal => 
                unit.global_vars.push(parse_variable_block(&mut lexer)?),
            KeywordProgram => 
                unit.units.push(parse_pou(&mut lexer, PouType::Program, KeywordEndProgram)?),
            KeywordFunction => 
                unit.units.push(parse_pou(&mut lexer, PouType::Function, KeywordEndFunction)?),
            KeywordFunctionBlock =>
                unit.units.push(parse_pou(&mut lexer, PouType::FunctionBlock, KeywordEndFunctionBlock)?),
            End => return Ok(unit),
            Error => return Err(unidentified_token(&lexer)),
            _ => return Err(unexpected_token(&lexer)),
        };

    }
    //the match in the loop will always return
}

///
/// parse a pou
/// # Arguments
/// 
/// * `lexer`       - the lexer
/// * `pou_type`    - the type of the pou currently parsed
/// * `expected_end_token` - the token that ends this pou
/// 
fn parse_pou(lexer: &mut RustyLexer, pou_type: PouType, expected_end_token: lexer::Token) -> Result<POU, String> {
    lexer.advance(); //Consume ProgramKeyword
    let mut result = create_pou(pou_type);
 
    //Parse Identifier
    expect!(Identifier, lexer);
    result.name = slice_and_advance(lexer);

    //optional return type
    if allow(KeywordColon, lexer) {
        result.return_type = Some(parse_data_type(lexer)?);
    }


    //Parse variable declarations
    while lexer.token == KeywordVar || lexer.token == KeywordVarInput {
        let block = parse_variable_block(lexer);
        match block {
            Ok(b) => result.variable_blocks.push(b),
            Err(msg) => return Err(msg),
        };
    }

    //Parse the statemetns
    let mut body = parse_body(lexer, &|it| *it == expected_end_token)?;
    result.statements.append(&mut body);

    expect!(expected_end_token, lexer);
    lexer.advance();
    Ok(result)
}

fn is_end_of_stream(token: &lexer::Token) -> bool {
    *token == End || *token == Error 
}

fn parse_body(lexer: &mut RustyLexer, until: &dyn Fn(&lexer::Token) -> bool) -> Result<Vec<Statement>, String> {
    let mut statements = Vec::new();
    consume_all(lexer, KeywordSemicolon);
    while !until(&lexer.token) && !is_end_of_stream(&lexer.token) {
        let statement = parse_control(lexer)?; 
        consume_all(lexer, KeywordSemicolon);
        statements.push(statement);
    }
    if !until(&lexer.token) {
        return Err(format!("unexpected end of body {:?}, statements : {:?}", lexer.token, statements).to_string());
    }
    Ok(statements)
}

fn consume_all(lexer: &mut RustyLexer, token: lexer::Token) {
    while lexer.token == token {
        lexer.advance();
    }
}

/**
 * parses either an expression (ended with ';' or a case-condition ended with ':')
 * does not consume the terminating token
 */
fn parse_statement(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let result = parse_expression(lexer);
 
    if !(lexer.token == KeywordColon || lexer.token == KeywordSemicolon) {
        return Err(format!("expected End Statement, but found {:?}", lexer.token).to_string());
    }
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

fn parse_variable_block_type(lexer: &mut RustyLexer) -> Result<VariableBlockType, String> {
    let block_type = &lexer.token;
    let result = match block_type {
        KeywordVar =>  Ok(VariableBlockType::Local),
        KeywordVarInput => Ok(VariableBlockType::Input),
        KeywordVarGlobal => Ok(VariableBlockType::Global),
        _ => Err(unexpected_token(lexer)),
    };
    lexer.advance();
    result
}

fn parse_variable_block(lexer: &mut RustyLexer) -> Result<VariableBlock, String> {
    //Consume the var block

    let mut result = VariableBlock {
        variables: Vec::new(),
        variable_block_type: parse_variable_block_type(lexer).unwrap(),
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

    let data_type = parse_data_type(lexer)?;
    //Convert to real datatype

    expect!(KeywordSemicolon, lexer);
    lexer.advance();

    owner.variables.push(Variable {name, data_type });
    Ok(owner)
}

fn parse_data_type(lexer: &mut RustyLexer) -> Result<Type, String> {
    expect!(Identifier, lexer);
    let name = slice_and_advance(lexer);
 
    let prim_type = match name.to_lowercase().as_str() {
        "int" => Some(PrimitiveType::Int),
        "bool" => Some(PrimitiveType::Bool),
        _ => None,
    };

    if let Some(prim_type) = prim_type {
        Ok(Type::Primitive(prim_type))
    } else {
        Ok(Type::Custom)
    }
}


