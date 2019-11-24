use super::lexer;
use logos::Lexer;

use super::lexer::Token::*;

macro_rules! expect {
    ( $token:expr, $lexer:expr) => {
        if ($lexer.token != $token) {
            return Err(format!("expected {:?}, but found {:?}", $token, $lexer.token).to_string());
        }
    };
}

type RustyLexer<'a> = Lexer<lexer::Token, &'a str>;

#[derive(Debug, PartialEq)]
pub struct Program {
    name: String,
    variable_blocks: Vec<VariableBlock>,
    statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct CompilationUnit {
    units: Vec<Program>,
}

#[derive(Debug, PartialEq)]
pub struct VariableBlock {
    variables: Vec<Variable>,
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    name: String,
    data_type: String,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    LiteralNumber {
        value: String,
    },
    Reference {
        name: String,
    },
    BinaryExpression {
        operator: Operator,
        left: Box<Statement>,
        right: Box<Statement>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
}

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
                lexer.advance();
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
    Ok(unit)
}

fn parse_program(lexer: &mut RustyLexer) -> Result<Program, String> {
    let mut result = create_program();

    expect!(Identifier, lexer);

    //Parse Identifier
    result.name = slice_and_advance(lexer);

    //Parse variable declarations
    while lexer.token == KeywordVar {
        lexer.advance();
        let block = parse_variable_block(lexer);
        match block {
            Ok(b) => result.variable_blocks.push(b),
            Err(msg) => return Err(msg),
        };
    }

    //Parse the statemetns
    while lexer.token != KeywordEndProgram && lexer.token != End && lexer.token != Error {
        let statement = (parse_statement(lexer))?;
        result.statements.push(statement);
    }
    expect!(KeywordEndProgram, lexer);

    Ok(result)
}

fn parse_statement(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let result = parse_binary_expression(lexer);
    expect!(KeywordSemicolon, lexer);
    lexer.advance();
    result
}

fn parse_binary_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let left = parse_unary_expression(lexer)?;
    let operator = match lexer.token {
        OperatorPlus => Operator::Plus,
        OperatorMinus => Operator::Minus,
        _ => return Ok(left),
    };
    lexer.advance();
    let right = parse_binary_expression(lexer)?;
    Ok(Statement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
    })
}

fn parse_unary_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    match lexer.token {
        Identifier => parse_reference(lexer),
        LiteralNumber => parse_literal_number(lexer),
        _ => Err(unexpected_token(lexer)),
    }
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

fn parse_variable_block(lexer: &mut RustyLexer) -> Result<VariableBlock, String> {
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

    expect!(KeywordSemicolon, lexer);
    lexer.advance();

    owner.variables.push(Variable { name, data_type });

    Ok(owner)
}

#[cfg(test)]
mod tests {
    use super::super::lexer;
    use super::Statement;

    #[test]
    fn empty_returns_empty_compilation_unit() {
        let result = super::parse(lexer::lex("")).unwrap();
        assert_eq!(result.units.len(), 0);
    }

    #[test]
    fn simple_foo_program_can_be_parsed() {
        let lexer = lexer::lex("PROGRAM foo END_PROGRAM");
        let result = super::parse(lexer).unwrap();

        let prg = &result.units[0];
        assert_eq!(prg.name, "foo");
    }

    #[test]
    fn two_programs_can_be_parsed() {
        let lexer = lexer::lex("PROGRAM foo END_PROGRAM  PROGRAM bar END_PROGRAM");
        let result = super::parse(lexer).unwrap();

        let prg = &result.units[0];
        assert_eq!(prg.name, "foo");
        let prg2 = &result.units[1];
        assert_eq!(prg2.name, "bar");
    }

    #[test]
    fn simple_program_with_varblock_can_be_parsed() {
        let lexer = lexer::lex("PROGRAM buz VAR END_VAR END_PROGRAM");
        let result = super::parse(lexer).unwrap();

        let prg = &result.units[0];

        assert_eq!(prg.variable_blocks.len(), 1);
    }

    #[test]
    fn simple_program_with_two_varblocks_can_be_parsed() {
        let lexer = lexer::lex("PROGRAM buz VAR END_VAR VAR END_VAR END_PROGRAM");
        let result = super::parse(lexer).unwrap();

        let prg = &result.units[0];

        assert_eq!(prg.variable_blocks.len(), 2);
    }

    #[test]
    fn a_program_needs_to_end_with_endProgram() {
        let lexer = lexer::lex("PROGRAM buz ");
        let result = super::parse(lexer);
        assert_eq!(
            result,
            Err("expected KeywordEndProgram, but found End".to_string())
        );
    }

    #[test]
    fn a_variable_declaration_block_needs_to_end_with_endVar() {
        let lexer = lexer::lex("PROGRAM buz VAR END_PROGRAM ");
        let result = super::parse(lexer);
        assert_eq!(
            result,
            Err("expected KeywordEndVar, but found KeywordEndProgram".to_string())
        );
    }

    #[test]
    fn simple_program_with_variable_can_be_parsed() {
        let lexer = lexer::lex("PROGRAM buz VAR x : INT; END_VAR END_PROGRAM");
        let result = super::parse(lexer).unwrap();

        let prg = &result.units[0];
        let variable = &prg.variable_blocks[0].variables[0];

        assert_eq!(variable.name, "x");
        assert_eq!(variable.data_type, "INT");
    }

    #[test]
    fn single_statement_parsed() {
        let lexer = lexer::lex("PROGRAM exp x; END_PROGRAM");
        let result = super::parse(lexer).unwrap();

        let prg = &result.units[0];
        let statement = &prg.statements[0];

        if let Statement::Reference { name } = statement {
            assert_eq!(name, "x");
        } else {
            panic!("Expected Reference but found {:?}", statement);
        }
    }

    #[test]
    fn literal_can_be_parsed() {
        let lexer = lexer::lex("PROGRAM exp 7; END_PROGRAM");
        let result = super::parse(lexer).unwrap();

        let prg = &result.units[0];
        let statement = &prg.statements[0];

        if let Statement::LiteralNumber { value } = statement {
            assert_eq!(value, "7");
        } else {
            panic!("Expected LiteralNumber but found {:?}", statement);
        }
    }

    #[test]
    fn additon_of_two_variables_parsed() {
        let lexer = lexer::lex("PROGRAM exp x+y; END_PROGRAM");
        let result = super::parse(lexer).unwrap();

        let prg = &result.units[0];
        let statement = &prg.statements[0];

        if let Statement::BinaryExpression {
            operator,
            left,  //Box<Reference> {name : left}),
            right, //Box<Reference> {name : right}),
        } = statement
        {
            if let Statement::Reference { name } = &**left {
                assert_eq!(name, "x");
            }
            if let Statement::Reference { name } = &**right {
                assert_eq!(name, "y");
            }
            assert_eq!(operator, &super::Operator::Plus);
        } else {
            panic!("Expected Reference but found {:?}", statement);
        }
    }

    #[test]
    fn additon_of_three_variables_parsed() {
        let lexer = lexer::lex("PROGRAM exp x+y-z; END_PROGRAM");
        let result = super::parse(lexer).unwrap();

        let prg = &result.units[0];
        let statement = &prg.statements[0];

        if let Statement::BinaryExpression {
            operator,
            left,  //Box<Reference> {name : left}),
            right, //Box<Reference> {name : right}),
        } = statement
        {
            if let Statement::Reference { name } = &**left {
                assert_eq!(name, "x");
            }
            if let Statement::BinaryExpression {
                operator,
                left,
                right,
            } = &**right
            {
                if let Statement::Reference { name } = &**left {
                    assert_eq!(name, "y");
                }
                if let Statement::Reference { name } = &**right {
                    assert_eq!(name, "z");
                }
                assert_eq!(operator, &super::Operator::Minus);
            } else {
                panic!("Expected Reference but found {:?}", statement);
            }
        } else {
            panic!("Expected Reference but found {:?}", statement);
        }
    }
}
