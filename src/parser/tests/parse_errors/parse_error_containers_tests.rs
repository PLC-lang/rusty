// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{
    ast::{PouType, SourceRange, Statement, Variable, VariableBlock, VariableBlockType},
    lexer::Token,
    parser::{parse, tests::lex},
    Diagnostic,
};
use pretty_assertions::*;

/*
 * These tests deal with parsing-behavior of containers like POUs, VAR-containers and Actions
 * in the presence of errors.
 * following scenarios will be tested:
 *  - missing pou names, missing function's return variable, illegal return-variable declaration
 *  - incomplete variable-declarations
 *  - incomplete containers, illegal close-keywords
 */

#[test]
fn missing_pou_name() {
    let lexer = lex(r"
            PROGRAM  
            VAR END_VAR
            a;
            END_PROGRAM
            ");

    let (compilation_unit, diagnostics) = parse(lexer).unwrap();
    //expected end of statement (e.g. ;), but found KeywordEndProgram at line: 1 offset: 14..25"
    //Expecting a missing semicolon message
    let expected = Diagnostic::unexpected_token_found(
        "Identifier".into(),
        "VAR".into(),
        SourceRange::new(35..38),
    );
    assert_eq!(diagnostics[0], expected);

    let pou = &compilation_unit.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements[0]),
        format!(
            "{:#?}",
            Statement::Reference {
                name: "a".into(),
                location: SourceRange::undefined()
            }
        )
    );
}

#[test]
fn missing_pou_name_2() {
    // in this case, a becomes the POU's name
    let lexer = lex(r"
            PROGRAM 
            a := 2;
            x;
            END_PROGRAM
            ");

    let (compilation_unit, diagnostics) = parse(lexer).unwrap();
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::syntax_error("Unexpected token: ':='".into(), (36..38).into()),
            Diagnostic::unexpected_token_found(
                "KeywordSemicolon".into(),
                "':= 2'".into(),
                (36..40).into()
            )
        ]
    );

    let pou = &compilation_unit.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements[1]),
        format!(
            "{:#?}",
            Statement::Reference {
                name: "x".into(),
                location: SourceRange::undefined()
            }
        )
    );
}

#[test]
fn illegal_end_pou_keyword() {
    let lexer = lex(r"
            PROGRAM foo
            a;
            END_FUNCTION
            PROGRAM baz
            b;
            END_PROGRAM
            ");

    let (compilation_unit, diagnostics) = parse(lexer).unwrap();
    let expected = Diagnostic::unexpected_token_found(
        format!("{:?}", Token::KeywordEndProgram),
        "END_FUNCTION".into(),
        SourceRange::new(52..64),
    );
    assert_eq!(diagnostics, vec![expected]);

    //check if baz was parsed successfully
    let pou = &compilation_unit.implementations[1];
    assert_eq!(
        format!("{:#?}", pou.statements),
        format!(
            "{:#?}",
            vec![Statement::Reference {
                name: "b".into(),
                location: SourceRange::undefined()
            }]
        )
    );
}

#[test]
#[ignore = "Semantic validation"]
fn function_without_return_variable_declaration() {
    // GIVEN a function without a return type
    let lexer = lex(r"
        FUNCTION foo
        a;
        END_FUNCTION
        ");

    // WHEN the function is parsed
    let (compilation_unit, diagnostics) = parse(lexer).unwrap();

    // THEN I expect a diagnostic complaining about a missing return type
    let expected = Diagnostic::unexpected_token_found(
        "COLON".into(),
        "'a', (Identifier)".into(),
        SourceRange::new(76..79),
    );
    assert_eq!(diagnostics, vec![expected]);

    // AND I expect the body to be parsed successfully
    let pou = &compilation_unit.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements),
        r#"[
            Reference {
                name: "a",
            },
            ]"#
    );
}

#[test]
fn function_with_illegal_return_variable_declaration() {
    let lexer = lex(r"
            FUNCTION foo :
            VAR END_VAR
            a;
            END_FUNCTION
            ");

    let (compilation_unit, diagnostics) = parse(lexer).unwrap();
    //expected end of statement (e.g. ;), but found KeywordEndProgram at line: 1 offset: 14..25"
    //Expecting a missing semicolon message
    let expected = Diagnostic::unexpected_token_found(
        "Datatype".into(),
        "VAR".into(),
        SourceRange::new(40..43),
    );
    assert_eq!(diagnostics[0], expected);

    //check if a was parsed successfully
    let pou = &compilation_unit.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements),
        r#"[
    Reference {
        name: "a",
    },
]"#
    );
}

#[test]
fn program_with_illegal_return_variable_declaration() {
    let lexer = lex(r"
                PROGRAM foo : INT
                VAR END_VAR
                a;
                END_PROGRAM
                ");

    let (compilation_unit, diagnostics) = parse(lexer).unwrap();
    //expected end of statement (e.g. ;), but found KeywordEndProgram at line: 1 offset: 14..25"
    //Expecting a missing semicolon message
    let expected =
        Diagnostic::return_type_not_supported(&PouType::Program, SourceRange::new(29..34));
    assert_eq!(diagnostics.get(0), Some(&expected));

    //check if a was parsed successfully
    let pou = &compilation_unit.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements),
        format!(
            "{:#?}",
            vec![Statement::Reference {
                name: "a".into(),
                location: SourceRange::undefined()
            }]
        )
    );
}

#[test]
fn unclosed_var_container() {
    let lexer = lex(r"
                PROGRAM foo
                    VAR a : INT;
                    VAR b : INT; END_VAR
                END_PROGRAM
                ");

    let (compilation_unit, diagnostics) = parse(lexer).unwrap();
    assert_eq!(
        vec![Diagnostic::unexpected_token_found(
            "KeywordEndVar".into(),
            "'VAR b : INT;'".into(),
            (82..94).into(),
        )],
        diagnostics
    );
    //check if b was parsed successfully
    let var_block = &compilation_unit.units[0].variable_blocks[0];
    assert_eq!(
        format!("{:#?}", var_block),
        format!(
            "{:#?}",
            VariableBlock {
                variable_block_type: VariableBlockType::Local,
                variables: vec![Variable {
                    name: "a".into(),
                    data_type: crate::ast::DataTypeDeclaration::DataTypeReference {
                        referenced_type: "INT".into(),
                    },
                    initializer: None,
                    location: SourceRange::undefined(),
                }]
            }
        )
    );
}

#[test]
fn test_unexpected_type_declaration_error_message() {
    let lexer = lex("TYPE MyType:
                PROGRAM
                END_PROGRAM
            END_TYPE
    ");
    let (_, diagnostics) = parse(lexer).unwrap();
    assert_eq!(
        vec![
            Diagnostic::unexpected_token_found(
                "DataTypeDefinition".into(),
                "KeywordProgram".into(),
                (29..36).into(),
            ),
            Diagnostic::unexpected_token_found(
                "KeywordSemicolon".into(),
                "'PROGRAM\n                END_PROGRAM\n            END_TYPE'".into(),
                (29..85).into(),
            ),
            Diagnostic::unexpected_token_found(
                "KeywordSemicolon".into(),
                "''".into(),
                (90..90).into(),
            ),
        ],
        diagnostics
    );
}

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
fn a_program_needs_to_end_with_end_program() {
    let lexer = lex("PROGRAM buz ");
    let (_, diagnostics) = parse(lexer).unwrap();
    assert_eq!(
        diagnostics,
        vec![Diagnostic::unexpected_token_found(
            "KeywordEndProgram".into(),
            "''".into(),
            (12..12).into()
        ),]
    );
}

#[test]
fn a_variable_declaration_block_needs_to_end_with_endvar() {
    let lexer = lex("PROGRAM buz VAR END_PROGRAM ");
    let (_, diagnostics) = parse(lexer).unwrap();

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_token("[KeywordEndVar]".into(), (16..27).into()),
            Diagnostic::unexpected_token_found(
                "KeywordEndVar".into(),
                "'END_PROGRAM'".into(),
                (16..27).into()
            ),
        ]
    );
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
