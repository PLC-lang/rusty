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

#[derive(Debug, PartialEq)]
pub struct Program {
    name: String,
    variable_blocks: Vec<VariableBlock>,
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

///
/// returns an error for an uidientified token
///  
fn unidentified_token(lexer: &Lexer<lexer::Token, &str>) -> String {
    format!(
        "unidentified token: {t:?} at {location:?}",
        t = lexer.slice(),
        location = lexer.range()
    )
}

///
/// returns an error for an unexpected token
///  
fn unexpected_token(lexer: &Lexer<lexer::Token, &str>) -> String {
    format!(
        "unexpected token: {t:?} at {location:?}",
        t = lexer.token,
        location = lexer.range()
    )
}

fn slice_and_advance(lexer: &mut Lexer<lexer::Token, &str>) -> String {
    let slice = lexer.slice().to_string();
    lexer.advance();
    slice
}

pub fn parse(mut lexer: Lexer<lexer::Token, &str>) -> Result<CompilationUnit, String> {
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

fn parse_program(lexer: &mut Lexer<lexer::Token, &str>) -> Result<Program, String> {
    let mut result = Program {
        name: "".to_string(),
        variable_blocks: Vec::new(),
    };

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

    expect!(KeywordEndProgram, lexer);

    Ok(result)
}

fn parse_variable_block(lexer: &mut Lexer<lexer::Token, &str>) -> Result<VariableBlock, String> {
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
    lexer: &mut Lexer<lexer::Token, &str>,
    mut owner: VariableBlock,
) -> Result<VariableBlock, String> {
    let name = slice_and_advance(lexer);

    expect!(KeywordColon, lexer);
    lexer.advance();

    expect!(Identifier, lexer);
    let data_type = slice_and_advance(lexer);

    expect!(KeywordSemiColon, lexer);
    lexer.advance();

    owner.variables.push(Variable { name, data_type });

    Ok(owner)
}

#[cfg(test)]
mod tests {
    use super::super::lexer;

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

}
