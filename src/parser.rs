// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::ast::*;
use crate::lexer;
use crate::lexer::Token;
use crate::lexer::{ParseSession, Token::*};
use crate::{ast::Implementation, Diagnostic};

use self::{control_parser::parse_control_statement, expressions_parser::parse_primary_expression};

mod control_parser;
mod expressions_parser;

#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! expect {
    ( $token:expr, $lexer:expr) => {
        if $lexer.token != $token {
            return Err(Diagnostic::unexpected_token_found(
                format!("{:?}", $token),
                $lexer.slice().to_string(),
                $lexer.location(),
            ));
        }
    };
}

/// consumes an optional token and returns true if it was consumed.
pub fn allow(token: lexer::Token, lexer: &mut ParseSession) -> bool {
    if lexer.token == token {
        lexer.advance();
        true
    } else {
        false
    }
}

///
/// returns an error for an unidientified token
///  
fn unidentified_token(lexer: &ParseSession) -> Diagnostic {
    Diagnostic::syntax_error(
        format!("Unidentified token: {t:?}", t = lexer.slice()),
        lexer.location(),
    )
}

///
/// returns an error for an unexpected token
///  
fn unexpected_token(lexer: &ParseSession) -> Diagnostic {
    Diagnostic::syntax_error(
        format!("Unexpected token: '{slice:}'", slice = lexer.slice()),
        lexer.location(),
    )
}

fn slice_and_advance(lexer: &mut ParseSession) -> String {
    let slice = lexer.slice().to_string();
    lexer.advance();
    slice
}

pub type PResult<T> = Result<T, Diagnostic>;
pub type ParsedAst = (CompilationUnit, NewLines, Vec<Diagnostic>);

pub fn parse(mut lexer: ParseSession) -> PResult<ParsedAst> {
    let mut unit = CompilationUnit::default();

    let mut linkage = LinkageType::Internal;
    loop {
        match lexer.token {
            PropertyExternal => {
                linkage = LinkageType::External;
                lexer.advance();
                //Don't reset linkage
                continue;
            }
            KeywordVarGlobal => {
                if let Some(block) = parse_variable_block(&mut lexer) {
                    unit.global_vars.push(block);
                }
            }
            KeywordProgram => {
                let (pou, implementation) =
                    parse_pou(&mut lexer, PouType::Program, linkage, KeywordEndProgram)?;
                unit.units.push(pou);
                unit.implementations.push(implementation);
            }
            KeywordFunction => {
                let (pou, implementation) =
                    parse_pou(&mut lexer, PouType::Function, linkage, KeywordEndFunction)?;
                unit.units.push(pou);
                unit.implementations.push(implementation);
            }
            KeywordFunctionBlock => {
                let (pou, implementation) = parse_pou(
                    &mut lexer,
                    PouType::FunctionBlock,
                    linkage,
                    KeywordEndFunctionBlock,
                )?;
                unit.units.push(pou);
                unit.implementations.push(implementation);
            }
            KeywordAction => {
                let implementation = parse_action(&mut lexer, linkage, None)?;
                unit.implementations.push(implementation);
            }
            KeywordActions => {
                let mut actions = parse_actions(&mut lexer, linkage)?;
                unit.implementations.append(&mut actions);
            }
            KeywordType => unit.types.push(parse_type(&mut lexer)?),
            KeywordEndActions | End => {
                return Ok((
                    unit,
                    lexer.get_new_lines().clone(),
                    lexer.diagnostics.clone(),
                ))
            }
            Error => return Err(unidentified_token(&lexer)),
            _ => return Err(unexpected_token(&lexer)),
        };
        linkage = LinkageType::Internal;
    }
    //the match in the loop will always return
}

fn parse_actions(
    mut lexer: &mut ParseSession,
    linkage: LinkageType,
) -> Result<Vec<Implementation>, Diagnostic> {
    lexer.advance(); //Consume ACTIONS
    expect!(Identifier, lexer);
    let container = slice_and_advance(lexer);
    let mut result = vec![];

    //Go through each action
    while lexer.token != KeywordEndActions && !is_end_of_stream(&lexer.token) {
        match lexer.token {
            KeywordAction => result.push(parse_action(&mut lexer, linkage, Some(&container))?),
            Error => return Err(unidentified_token(&lexer)),
            _ => return Err(unexpected_token(&lexer)),
        }
    }
    lexer.advance(); //Consume end actions

    Ok(result)
}

///
/// parse a pou
/// # Arguments
///
/// * `lexer`       - the lexer
/// * `pou_type`    - the type of the pou currently parsed
/// * `expected_end_token` - the token that ends this pou
///
fn parse_pou(
    lexer: &mut ParseSession,
    pou_type: PouType,
    linkage: LinkageType,
    expected_end_token: lexer::Token,
) -> Result<(Pou, Implementation), Diagnostic> {
    let start = lexer.range().start;
    lexer.advance(); //Consume ProgramKeyword

    //Parse pou name
    let name = if lexer.token == Identifier {
        slice_and_advance(lexer)
    } else {
        //missing pou name
        lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
            "Identifier".to_string(),
            lexer.slice().to_string(),
            SourceRange::new(lexer.get_file_path(), lexer.range()),
        ));
        "".to_string()
    };

    //optional return type
    let start_return_type = lexer.range().start;
    let return_type = if allow(KeywordColon, lexer) {
        if lexer.token == Identifier || lexer.token == KeywordString {
            if pou_type != PouType::Function {
                lexer.accept_diagnostic(Diagnostic::return_type_not_supported(
                    &pou_type,
                    SourceRange::new(lexer.get_file_path(), start_return_type..lexer.range().end),
                ));
            }
            let referenced_type = slice_and_advance(lexer);
            Some(DataTypeDeclaration::DataTypeReference { referenced_type })
        } else {
            //missing return type
            lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
                "Datatype".to_string(),
                lexer.slice().to_string(),
                SourceRange::new(lexer.get_file_path(), lexer.range()),
            ));
            None
        }
    } else {
        None
    };

    lexer.enter_region(vec![
        KeywordEndProgram,
        KeywordEndFunction,
        KeywordEndFunctionBlock,
        KeywordEndAction,
    ]);

    //Parse variable declarations
    let mut variable_blocks = vec![];
    while lexer.token == KeywordVar
        || lexer.token == KeywordVarInput
        || lexer.token == KeywordVarOutput
        || lexer.token == KeywordVarInOut
    {
        if let Some(block) = parse_variable_block(lexer) {
            variable_blocks.push(block);
        }
    }

    let implementation =
        parse_implementation(lexer, linkage, pou_type, expected_end_token, &name, &name);

    lexer.recover_until_close();
    lexer.close_region();

    let pou = Pou {
        name,
        pou_type,
        variable_blocks,
        return_type,
        location: SourceRange::new(lexer.get_file_path(), start..lexer.range().end),
    };

    Ok((pou, implementation))
}

fn parse_implementation(
    lexer: &mut ParseSession,
    linkage: LinkageType,
    pou_type: PouType,
    expected_end_token: lexer::Token,
    call_name: &str,
    type_name: &str,
) -> Implementation {
    let start = lexer.range().start;
    let statements =
        parse_body(lexer, &|it: &ParseSession| it.token.ends_implementation()).unwrap_or_default();
    //lets see if we ended on the right END_ keyword
    if lexer.token != expected_end_token {
        lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
            format!("{:?}", expected_end_token),
            lexer.slice().into(),
            lexer.location(),
        ))
    }
    Implementation {
        name: call_name.into(),
        type_name: type_name.into(),
        linkage,
        pou_type,
        statements,
        location: SourceRange::new(lexer.get_file_path(), start..lexer.range().end),
    }
}

fn parse_action(
    lexer: &mut ParseSession,
    linkage: LinkageType,
    container: Option<&str>,
) -> Result<Implementation, Diagnostic> {
    lexer.advance(); //Consume the Action keyword
    lexer.enter_region(vec![
        KeywordEndAction,
        KeywordEndProgram,
        KeywordEndFunction,
        KeywordEndFunctionBlock,
    ]);

    let name_or_container = slice_and_advance(lexer);
    let (container, name) = if let Some(container) = container {
        (container.into(), name_or_container)
    } else {
        expect!(KeywordDot, lexer);
        lexer.advance();
        expect!(Identifier, lexer);
        let name = slice_and_advance(lexer);
        (name_or_container, name)
    };
    let call_name = format!("{}.{}", &container, &name);

    let implementation = parse_implementation(
        lexer,
        linkage,
        PouType::Action,
        lexer::Token::KeywordEndAction,
        &call_name,
        &container,
    );
    lexer.recover_until_close();
    lexer.close_region();

    Ok(implementation)
}

// TYPE ... END_TYPE
fn parse_type(lexer: &mut ParseSession) -> Result<UserTypeDeclaration, Diagnostic> {
    lexer.advance(); // consume the TYPE
    let name = slice_and_advance(lexer);
    expect!(KeywordColon, lexer);
    lexer.advance();

    let result = parse_data_type_definition(lexer, Some(name));

    if let Ok((DataTypeDeclaration::DataTypeDefinition { data_type }, initializer)) = result {
        expect!(KeywordEndType, lexer);
        lexer.advance();
        Ok(UserTypeDeclaration {
            data_type,
            initializer,
        })
    } else {
        Err(Diagnostic::unexpected_token_found(
            "struct, enum or subrange".into(),
            lexer.slice().into(),
            lexer.location(),
        ))
    }
}

type DataTypeWithInitializer = (DataTypeDeclaration, Option<Statement>);
// TYPE xxx : 'STRUCT' | '(' | IDENTIFIER
fn parse_data_type_definition(
    lexer: &mut ParseSession,
    name: Option<String>,
) -> Result<DataTypeWithInitializer, Diagnostic> {
    if allow(KeywordStruct, lexer) {
        //STRUCT
        let mut variables = Vec::new();
        while lexer.token == Identifier {
            variables.push(parse_variable(lexer)?);
        }
        expect!(KeywordEndStruct, lexer);
        lexer.advance();
        Ok((
            DataTypeDeclaration::DataTypeDefinition {
                data_type: DataType::StructType { name, variables },
            },
            None,
        ))
    } else if allow(KeywordArray, lexer) {
        //ARRAY
        //expect open square
        expect!(KeywordSquareParensOpen, lexer);
        lexer.advance();
        //parse range
        let range = parse_primary_expression(lexer)?;
        //expect close range
        expect!(KeywordSquareParensClose, lexer);
        lexer.advance();
        expect!(KeywordOf, lexer);
        lexer.advance();
        //expect type reference
        let (reference, initializer) = parse_data_type_definition(lexer, None)?;
        Ok((
            DataTypeDeclaration::DataTypeDefinition {
                data_type: DataType::ArrayType {
                    name,
                    bounds: range,
                    referenced_type: Box::new(reference),
                },
            },
            initializer,
        ))
    } else if allow(KeywordParensOpen, lexer) {
        //ENUM
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

        Ok((
            DataTypeDeclaration::DataTypeDefinition {
                data_type: DataType::EnumType { name, elements },
            },
            None,
        ))
    } else if lexer.token == KeywordString || lexer.token == KeywordWideString {
        let is_wide = lexer.token == KeywordWideString;
        lexer.advance();
        let size = if allow(KeywordSquareParensOpen, lexer) {
            let size_statement = parse_expression(lexer)?;
            expect!(KeywordSquareParensClose, lexer);
            lexer.advance();
            Some(size_statement)
        } else {
            None
        };

        let initializer = if allow(KeywordAssignment, lexer) {
            Some(parse_expression(lexer)?)
        } else {
            None
        };
        expect!(KeywordSemicolon, lexer);
        lexer.advance();

        Ok((
            DataTypeDeclaration::DataTypeDefinition {
                data_type: DataType::StringType {
                    name,
                    is_wide,
                    size,
                },
            },
            initializer,
        ))
    } else if lexer.token == Identifier {
        //Subrange
        let referenced_type = slice_and_advance(lexer);

        if allow(KeywordParensOpen, lexer) {
            let bounds = parse_expression(lexer)?;
            expect!(KeywordParensClose, lexer);
            lexer.advance();

            let initial_value = if allow(KeywordAssignment, lexer) {
                Some(parse_expression(lexer)?)
            } else {
                None
            };

            expect!(KeywordSemicolon, lexer);
            lexer.advance();

            return Ok((
                DataTypeDeclaration::DataTypeDefinition {
                    data_type: DataType::SubRangeType {
                        name,
                        bounds: Some(bounds),
                        referenced_type,
                    },
                },
                initial_value,
            ));
        }

        let initial_value = if allow(KeywordAssignment, lexer) {
            Some(parse_expression(lexer)?)
        } else {
            None
        };
        if name.is_some() {
            let data_type = DataTypeDeclaration::DataTypeDefinition {
                data_type: DataType::SubRangeType {
                    name,
                    referenced_type,
                    bounds: None,
                },
            };
            expect!(KeywordSemicolon, lexer);
            lexer.advance();
            Ok((data_type, initial_value))
        } else {
            expect!(KeywordSemicolon, lexer);
            lexer.advance();
            Ok((
                DataTypeDeclaration::DataTypeReference { referenced_type },
                initial_value,
            ))
        }
    } else {
        Err(Diagnostic::unexpected_token_found(
            "Datatype, Struct or Enum".into(),
            lexer.slice().into(),
            lexer.location(),
        ))
    }
}

fn is_end_of_stream(token: &lexer::Token) -> bool {
    *token == End || *token == Error
}

fn parse_body(
    lexer: &mut ParseSession,
    until: &dyn Fn(&ParseSession) -> bool,
) -> Result<Vec<Statement>, Diagnostic> {
    let mut statements = Vec::new();
    consume_all(lexer, KeywordSemicolon);
    while !until(&lexer) && lexer.token != lexer::Token::End {
        // !is_end_of_stream(&lexer.token) {
        //if my token is an error, Recover from errors --> Read until the next non error token
        if lexer.token == lexer::Token::Error {
            lexer.accept_diagnostic(Diagnostic::illegal_token(lexer.slice(), lexer.location()));
            lexer.advance();
            //Consume all semicolons as empty statments
            consume_all(lexer, KeywordSemicolon);
            continue;
        }
        let statement = parse_control(lexer)?;
        consume_all(lexer, KeywordSemicolon);
        statements.push(statement);
    }

    Ok(statements)
}

fn consume_all(lexer: &mut ParseSession, token: lexer::Token) {
    while lexer.token == token {
        lexer.advance();
    }
}

/**
 * parses a statement ending with a ;
 */
fn parse_statement(lexer: &mut ParseSession) -> Result<Statement, Diagnostic> {
    let result = parse_statement_in_region(lexer, vec![KeywordSemicolon, KeywordColon], |lexer| {
        parse_expression(lexer)
    });
    Ok(result)
}

pub fn parse_statement_in_region<F: FnOnce(&mut ParseSession) -> PResult<Statement>>(
    lexer: &mut ParseSession,
    closing_tokens: Vec<Token>,
    parse_fn: F,
) -> Statement {
    lexer.enter_region(closing_tokens);
    let start = lexer.range().start;
    let result = parse_fn(lexer).unwrap_or_else(|diagnostic| {
        lexer.accept_diagnostic(diagnostic);
        let end = lexer.range().end;
        let location = SourceRange::new(lexer.get_file_path(), start..end);
        //drop the originally parsed statement and replace with an empty-statement
        Statement::EmptyStatement { location }
    });
    //try to recover by eating everything until we believe the parser is able to continue
    lexer.recover_until_close();
    lexer.close_region();
    //Report a diagnostic
    result
}

pub fn parse_any_in_region<T, F: FnOnce(&mut ParseSession) -> PResult<T>>(
    lexer: &mut ParseSession,
    closing_tokens: Vec<Token>,
    parse_fn: F,
) -> Option<T> {
    lexer.enter_region(closing_tokens);
    let result = parse_fn(lexer).map(Some).unwrap_or_else(|diagnostic| {
        lexer.accept_diagnostic(diagnostic);
        None
    });
    //try to recover by eating everything until we believe the parser is able to continue
    lexer.recover_until_close();
    lexer.close_region();
    //Report a diagnostic
    result
}

fn parse_expression(lexer: &mut ParseSession) -> Result<Statement, Diagnostic> {
    parse_primary_expression(lexer)
}

fn parse_reference(lexer: &mut ParseSession) -> Result<Statement, Diagnostic> {
    expressions_parser::parse_qualified_reference(lexer)
}

fn parse_control(lexer: &mut ParseSession) -> Result<Statement, Diagnostic> {
    parse_control_statement(lexer)
}

fn parse_variable_block_type(lexer: &mut ParseSession) -> Result<VariableBlockType, Diagnostic> {
    let block_type = &lexer.token;
    let result = match block_type {
        KeywordVar => Ok(VariableBlockType::Local),
        KeywordVarInput => Ok(VariableBlockType::Input),
        KeywordVarOutput => Ok(VariableBlockType::Output),
        KeywordVarGlobal => Ok(VariableBlockType::Global),
        KeywordVarInOut => Ok(VariableBlockType::InOut),
        _ => Err(unexpected_token(lexer)),
    };
    lexer.advance();
    result
}

fn parse_variable_block(lexer: &mut ParseSession) -> Option<VariableBlock> {
    parse_any_in_region(lexer, vec![KeywordEndVar], |lexer| {
        let mut result = VariableBlock {
            variables: Vec::new(),
            variable_block_type: parse_variable_block_type(lexer)?,
        };

        while lexer.token == Identifier {
            let variable = parse_variable(lexer);
            if let Ok(variable) = variable {
                result.variables.push(variable);
            }
        }
        Ok(result)
    })
}

fn parse_variable(lexer: &mut ParseSession) -> Result<Variable, Diagnostic> {
    //create an optional recovery point
    lexer.enter_region(vec![KeywordSemicolon, KeywordEndType]);

    let variable_location = lexer.location();
    let name = slice_and_advance(lexer);

    let maybe_data_type = if allow(KeywordColon, lexer) {
        parse_data_type_definition(lexer, None)
    } else {
        Err(Diagnostic::unexpected_token_found(
            format!("{:?}", KeywordColon),
            lexer.slice().into(),
            lexer.location(),
        ))
    };
    match maybe_data_type {
        Ok((data_type, initializer)) => {
            //everything ok, so we drop the recovery-point
            lexer.closing_keywords.pop();
            Ok(Variable {
                name,
                data_type,
                location: variable_location,
                initializer,
            })
        }
        Err(diagnostic) => {
            lexer.recover_until_close();
            lexer.close_region();
            Err(diagnostic)
        }
    }
}
