use crate::ast::PrimitiveType;
use crate::ast::Type;
use crate::lexer;
use pretty_assertions::*;

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
fn a_program_needs_to_end_with_end_program() {
    let lexer = lexer::lex("PROGRAM buz ");
    let result = super::parse(lexer);
    assert_eq!(result, Err("unexpected end of body End".to_string()));
}

#[test]
fn a_variable_declaration_block_needs_to_end_with_endvar() {
    let lexer = lexer::lex("PROGRAM buz VAR END_PROGRAM ");
    let result = super::parse(lexer);
    assert_eq!(
        result,
        Err("expected KeywordEndVar, but found KeywordEndProgram".to_string())
    );
}


#[test]
fn a_statement_without_a_semicolon_fails() {
    let lexer = lexer::lex("PROGRAM buz x END_PROGRAM ");
    let result = super::parse(lexer);
    assert_eq!(
        result,
        Err("expected End Statement, but found KeywordEndProgram".to_string())
    );
}

#[test]
fn empty_statements_are_ignored() {
    let lexer = lexer::lex("PROGRAM buz ;;;; END_PROGRAM ");
    let result = super::parse(lexer).unwrap();
    
    let prg = &result.units[0];
    assert_eq!(0, prg.statements.len());
}

#[test]
fn empty_statements_are_ignored_before_a_statement() {
    let lexer = lexer::lex("PROGRAM buz ;;;;x; END_PROGRAM ");
    let result = super::parse(lexer).unwrap();
    
    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"Reference {
    name: "x",
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn empty_statements_are_ignored_after_a_statement() {
    let lexer = lexer::lex("PROGRAM buz x;;;; END_PROGRAM ");
    let result = super::parse(lexer).unwrap();
    
    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"Reference {
    name: "x",
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn simple_program_with_variable_can_be_parsed() {
    let lexer = lexer::lex("PROGRAM buz VAR x : INT; END_VAR END_PROGRAM");
    let result = super::parse(lexer).unwrap();

    let prg = &result.units[0];
    let variable = &prg.variable_blocks[0].variables[0];

    assert_eq!(variable.name, "x");
    assert_eq!(variable.data_type, Type::Primitive(PrimitiveType::Int));
}
