use crate::{
    parser::{parse, tests::lex},
    Diagnostic,
};
use pretty_assertions::*;

#[test]
fn action_container_parsed() {
    let lexer = lex("ACTIONS foo ACTION bar END_ACTION END_ACTIONS");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    assert_eq!(prg.name, "foo.bar");
    assert_eq!(prg.type_name, "foo");
}

#[test]
fn two_action_containers_parsed() {
    let lexer = lex("ACTIONS foo ACTION bar END_ACTION ACTION buz END_ACTION END_ACTIONS");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    assert_eq!(prg.name, "foo.bar");
    assert_eq!(prg.type_name, "foo");

    let prg2 = &result.implementations[1];
    assert_eq!(prg2.name, "foo.buz");
    assert_eq!(prg2.type_name, "foo");
}

#[test]
fn mixed_action_types_parsed() {
    let lexer = lex("PROGRAM foo END_PROGRAM ACTIONS foo ACTION bar END_ACTION END_ACTIONS ACTION foo.buz END_ACTION");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[1];
    assert_eq!(prg.name, "foo.bar");
    assert_eq!(prg.type_name, "foo");

    let prg2 = &result.implementations[2];
    assert_eq!(prg2.name, "foo.buz");
    assert_eq!(prg2.type_name, "foo");
}

#[test]
fn actions_with_no_container_error() {
    let lexer = lex("ACTIONS ACTION bar END_ACTION ACTION buz END_ACTION END_ACTIONS");
    let err = parse(lexer).expect_err("Expecting parser failure");
    assert_eq!(
        err,
        Diagnostic::unexpected_token_found("Identifier".into(), "ACTION".into(), (8..14).into())
    );
}

#[test]
fn actions_with_invalid_token() {
    let lexer = lex("ACTIONS LIMA BRAVO END_ACTIONS");
    let err = parse(lexer).expect_err("Expecting parser failure");
    assert_eq!(
        err,
        Diagnostic::unexpected_token_found(
            "KeywordAction".to_string(),
            "BRAVO".into(),
            (13..18).into()
        )
    );
}

#[test]
fn two_programs_can_be_parsed() {
    let lexer = lex("PROGRAM foo END_PROGRAM  PROGRAM bar END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.units[0];
    assert_eq!(prg.name, "foo");
    let prg2 = &result.units[1];
    assert_eq!(prg2.name, "bar");
}

#[test]
fn simple_program_with_varblock_can_be_parsed() {
    let lexer = lex("PROGRAM buz VAR END_VAR END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.units[0];

    assert_eq!(prg.variable_blocks.len(), 1);
}

#[test]
fn simple_program_with_two_varblocks_can_be_parsed() {
    let lexer = lex("PROGRAM buz VAR END_VAR VAR END_VAR END_PROGRAM");
    let result = parse(lexer).unwrap().0;

    let prg = &result.units[0];

    assert_eq!(prg.variable_blocks.len(), 2);
}

#[test]
fn single_action_parsed() {
    let lexer = lex("ACTION foo.bar END_ACTION");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    assert_eq!(prg.name, "foo.bar");
    assert_eq!(prg.type_name, "foo");
}

#[test]
fn two_actions_parsed() {
    let lexer = lex("ACTION foo.bar END_ACTION ACTION fuz.bar END_ACTION");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    assert_eq!(prg.name, "foo.bar");
    assert_eq!(prg.type_name, "foo");

    let prg2 = &result.implementations[1];
    assert_eq!(prg2.name, "fuz.bar");
    assert_eq!(prg2.type_name, "fuz");
}
