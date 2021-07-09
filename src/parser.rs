/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::ast::*;
use crate::lexer;
use crate::lexer::{Token::*,RustyLexer};

use self::{control_parser::parse_control_statement, expressions_parser::parse_primary_expression};


mod expressions_parser;
mod control_parser;

#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! expect {
    ( $token:expr, $lexer:expr) => {
        if $lexer.token != $token {
            return Err(format!("expected {:?}, but found '{:}' [{:?}] at {:}", $token, $lexer.slice(), $lexer.token, $lexer.get_location_information()).to_string());
        }
    };
}

/// consumes an optional token and returns true if it was consumed.
pub fn allow(token: lexer::Token, lexer: &mut RustyLexer) -> bool {
    if lexer.token == token {
        lexer.advance();
        true
    } else {
        false
    }
}


fn create_pou(pou_type: PouType, linkage: LinkageType) -> POU {
    POU {
        pou_type,
        name: "".to_string(),
        variable_blocks: Vec::new(),
        statements: Vec::new(),
        return_type: None,
        linkage,
        location: 0..0,
    }
}

///
/// returns an error for an unidientified token
///  
fn unidentified_token(lexer: &RustyLexer) -> String {
    format!(
        "unidentified token: {t:?} at {location:?}",
        t = lexer.slice(),
        location = lexer.get_location_information()
    )
}

///
/// returns an error for an unexpected token
///  
fn unexpected_token(lexer: &RustyLexer) -> String {
    format!(
        "unexpected token: '{slice:}' [{t:?}] at {location:}",
        t = lexer.token,
        slice = lexer.slice(),
        location = lexer.get_location_information(),
    )
}

fn slice_and_advance(lexer: &mut RustyLexer) -> String {
    let slice = lexer.slice().to_string();
    lexer.advance();
    slice
}

pub fn parse(mut lexer: RustyLexer ) -> Result<(CompilationUnit, NewLines), String> {
    let mut unit = CompilationUnit { global_vars : Vec::new(), units: Vec::new(), types: Vec::new()};

    let mut linkage = LinkageType::Internal;
    loop {
        match lexer.token {
            PropertyExternal => {
                linkage = LinkageType::External; 
                lexer.advance();
                //Don't reset linkage
                continue
            }
            KeywordVarGlobal => 
                unit.global_vars.push(parse_variable_block(&mut lexer)?),
            KeywordProgram => 
                unit.units.push(parse_pou(&mut lexer, PouType::Program, linkage, KeywordEndProgram)?),
            KeywordFunction => 
                unit.units.push(parse_pou(&mut lexer, PouType::Function, linkage, KeywordEndFunction)?),
            KeywordFunctionBlock =>
                unit.units.push(parse_pou(&mut lexer, PouType::FunctionBlock, linkage, KeywordEndFunctionBlock)?),
            KeywordType =>
                unit.types.push(parse_type(&mut lexer)?),
            End => return Ok((unit, lexer.get_new_lines().clone())),
            Error => return Err(unidentified_token(&lexer)),
            _ => return Err(unexpected_token(&lexer)),
        };
        linkage = LinkageType::Internal;

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
fn parse_pou(lexer: &mut RustyLexer, pou_type: PouType, linkage: LinkageType, expected_end_token: lexer::Token) -> Result<POU, String> {
    let line_nr = lexer.get_current_line_nr();
    lexer.advance(); //Consume ProgramKeyword
    let mut result = create_pou(pou_type, linkage);
 
    //Parse Identifier
    expect!(Identifier, lexer);
    result.name = slice_and_advance(lexer);

    //optional return type
    if allow(KeywordColon, lexer) {
        let referenced_type = slice_and_advance(lexer);
        result.return_type = Some(DataTypeDeclaration::DataTypeReference {referenced_type});
    }

    //Parse variable declarations
    while lexer.token == KeywordVar || lexer.token == KeywordVarInput || lexer.token == KeywordVarOutput {
        let block = parse_variable_block(lexer);
        match block {
            Ok(b) => result.variable_blocks.push(b),
            Err(msg) => return Err(msg),
        };
    }

    //Parse the statemetns
    let mut body = parse_body(lexer, line_nr, &|it| *it == expected_end_token)?;
    result.statements.append(&mut body);

    expect!(expected_end_token, lexer);
    lexer.advance();
    Ok(result)
}

// TYPE ... END_TYPE
fn parse_type(lexer: &mut RustyLexer) -> Result<UserTypeDeclaration, String> {
    lexer.advance(); // consume the TYPE
    let name = slice_and_advance(lexer);
    expect!(KeywordColon, lexer);
    lexer.advance();

    let result = parse_data_type_definition(lexer, Some(name));
    
    if let Ok((DataTypeDeclaration::DataTypeDefinition {data_type}, initializer)) = result {
        expect!(KeywordEndType, lexer);
        lexer.advance();
        Ok(UserTypeDeclaration { data_type, initializer } )
    } else {
        Err(format!("expected struct, enum, or subrange found '{:}' [{:?}] at {:}", lexer.slice(), lexer.token, lexer.get_location_information()))
    }
}

type DataTypeWithInitializer = (DataTypeDeclaration, Option<Statement>);
// TYPE xxx : 'STRUCT' | '(' | IDENTIFIER
fn parse_data_type_definition(lexer: &mut RustyLexer, name: Option<String>) -> Result<DataTypeWithInitializer, String> {
    if allow(KeywordStruct, lexer) { //STRUCT
        let mut variables = Vec::new(); 
        while lexer.token == Identifier {
            variables.push(parse_variable(lexer)?);
        }
        expect!(KeywordEndStruct, lexer);
        lexer.advance();
        Ok((DataTypeDeclaration::DataTypeDefinition { data_type : DataType::StructType{ name, variables }},
            None))
    
    } else if allow(KeywordArray, lexer) { //ARRAY
        //expect open square
        expect!(KeywordSquareParensOpen,lexer);
        lexer.advance();
        //parse range
        let range = parse_primary_expression(lexer).unwrap();
        //expect close range
        expect!(KeywordSquareParensClose,lexer);
        lexer.advance();
        expect!(KeywordOf,lexer);
        lexer.advance();
        //expect type reference
        let (reference, initializer) = parse_data_type_definition(lexer,None).unwrap();
        Ok(
            (DataTypeDeclaration::DataTypeDefinition { data_type :DataType::ArrayType {name, bounds: range, referenced_type : Box::new(reference)}},
            initializer))
    } else if allow(KeywordParensOpen, lexer) { //ENUM
        let mut elements = Vec::new();

        //we expect at least one element
        expect!(Identifier, lexer);
        elements.push(slice_and_advance(lexer));
        //parse additional elements separated by ,
        while allow(KeywordComma, lexer) {
            expect!(Identifier, lexer);
            elements.push(slice_and_advance(lexer));
        }
        expect!(KeywordParensClose, lexer);
        lexer.advance();

        expect!(KeywordSemicolon, lexer);
        lexer.advance();

        Ok((DataTypeDeclaration::DataTypeDefinition { data_type : DataType::EnumType{ name, elements }}, None))

    } else if lexer.token == Identifier && lexer.slice().eq_ignore_ascii_case("STRING") {   //STRING TODO: should STRING be a keyword?
        lexer.advance();
        let size = if allow(KeywordSquareParensOpen, lexer) {
            lexer.advance();
            let size_statement = parse_expression(lexer)?;
            expect!(KeywordSquareParensClose, lexer);
            Some(size_statement)
        } else {
            None
        };

        let initializer = None;
        expect!(KeywordSemicolon, lexer);
        lexer.advance();

        Ok((DataTypeDeclaration::DataTypeDefinition { data_type: DataType::StringType {name, is_wide: false, size }}, initializer))
    
    } else if lexer.token == Identifier {   //Subrange
        let referenced_type = slice_and_advance(lexer);

        if name.is_some() {
            let initial_value = if allow(KeywordAssignment, lexer) {
                Some(parse_expression(lexer)?)
            } else {
                None
            };
            let data_type = DataTypeDeclaration::DataTypeDefinition { 
                data_type : DataType::SubRangeType { 
                    name, 
                    referenced_type }};
            expect!(KeywordSemicolon, lexer);
            lexer.advance();
            Ok((data_type, initial_value))
        } else {
             let initial_value = if allow(KeywordAssignment, lexer) {
                Some(parse_expression(lexer)?)
            } else {
                None
            };

            expect!(KeywordSemicolon, lexer);
            lexer.advance();
            Ok((DataTypeDeclaration::DataTypeReference { referenced_type: referenced_type }, initial_value))
        }
    } else {
        return Err(format!("expected datatype, struct or enum, found {:?}", lexer.token));
    }
}

fn is_end_of_stream(token: &lexer::Token) -> bool {
    *token == End || *token == Error 
}

fn parse_body(lexer: &mut RustyLexer, open_line_nr: usize, until: &dyn Fn(&lexer::Token) -> bool) -> Result<Vec<Statement>, String> {
    let mut statements = Vec::new();
    consume_all(lexer, KeywordSemicolon);
    while !until(&lexer.token) && !is_end_of_stream(&lexer.token) {
        let statement = parse_control(lexer)?; 
        consume_all(lexer, KeywordSemicolon);
        statements.push(statement);
    }
    if !until(&lexer.token) {
        return Err(format!("unexpected termination of body by '{:}' [{:?}], a block at line {:} was not closed", lexer.slice(), lexer.token, open_line_nr));
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
        return Err(format!("expected end of statement (e.g. ;), but found {:?} at {:}", lexer.token, lexer.get_location_information()));
    }
    result
}

fn parse_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    parse_primary_expression(lexer)
}

fn parse_reference(lexer: &mut RustyLexer) -> Result<Statement, String> {
    expressions_parser::parse_qualified_reference(lexer)
}

fn parse_control(lexer : &mut RustyLexer) -> Result<Statement, String> {
    parse_control_statement(lexer)
}

fn parse_variable_block_type(lexer: &mut RustyLexer) -> Result<VariableBlockType, String> {
    let block_type = &lexer.token;
    let result = match block_type {
        KeywordVar =>  Ok(VariableBlockType::Local),
        KeywordVarInput => Ok(VariableBlockType::Input),
        KeywordVarOutput => Ok(VariableBlockType::Output),
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
        result.variables.push(parse_variable(lexer)?);
    }

    expect!(KeywordEndVar, lexer);

    lexer.advance();
    Ok(result)
}

fn parse_variable(
    lexer: &mut RustyLexer) -> Result<Variable, String> {
    let variable_range = lexer.range();
    let name = slice_and_advance(lexer);

    expect!(KeywordColon, lexer);
    lexer.advance();

    let (data_type, initializer) = parse_data_type_definition(lexer, None)?;

    //Convert to real datatype
    Ok(Variable {
        name, 
        data_type, 
        location: variable_range,
        initializer: initializer,
    })
}
