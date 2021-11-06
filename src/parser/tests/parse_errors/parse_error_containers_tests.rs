// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{Diagnostic, ast::*, lexer::Token, test_utils::tests::{ToRc, parse}};
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
    let src = r"
            PROGRAM  
            VAR END_VAR
            a;
            END_PROGRAM
            ";

    let (compilation_unit, diagnostics) = parse(src);
    //expected end of statement (e.g. ;), but found KeywordEndProgram at line: 1 offset: 14..25"
    //Expecting a missing semicolon message
    let expected =
        Diagnostic::unexpected_token_found("Identifier", "VAR", SourceRange::new(35..38));
    assert_eq!(diagnostics[0], expected);

    let pou = &compilation_unit.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements[0]),
        format!(
            "{:#?}",
            AstStatement::Reference {
                name: "a".into(),
                location: SourceRange::undefined(),
                id: 0
            }
        )
    );
}

#[test]
fn missing_pou_name_2() {
    // in this case, a becomes the POU's name
    let src = r"
            PROGRAM 
            a := 2;
            x;
            END_PROGRAM
            ";

    let (compilation_unit, diagnostics) = parse(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::unexpected_token_found("Literal", ":=", (36..38).into()),
            Diagnostic::unexpected_token_found("KeywordSemicolon", "':= 2'", (36..40).into())
        ]
    );

    let pou = &compilation_unit.implementations[0];
    assert_eq!(
        format!("{:#?}", pou.statements[1]),
        format!(
            "{:#?}",
            AstStatement::Reference {
                name: "x".into(),
                location: SourceRange::undefined(),
                id: 0
            }
        )
    );
}

#[test]
fn illegal_end_pou_keyword() {
    let src = r"
            PROGRAM foo
            a;
            END_FUNCTION
            PROGRAM baz
            b;
            END_PROGRAM
            ";

    let (compilation_unit, diagnostics) = parse(src);
    let expected = Diagnostic::unexpected_token_found(
        format!("{:?}", Token::KeywordEndProgram).as_str(),
        "END_FUNCTION",
        SourceRange::new(52..64),
    );
    assert_eq!(diagnostics, vec![expected]);

    //check if baz was parsed successfully
    let pou = &compilation_unit.implementations[1];
    assert_eq!(
        format!("{:#?}", pou.statements),
        format!(
            "{:#?}",
            vec![AstStatement::Reference {
                name: "b".into(),
                location: SourceRange::undefined(),
                id: 0
            }]
        )
    );
}

#[test]
#[ignore = "Semantic validation"]
fn function_without_return_variable_declaration() {
    // GIVEN a function without a return type
    let src = r"
        FUNCTION foo
        a;
        END_FUNCTION
        ";

    // WHEN the function is parsed
    let (compilation_unit, diagnostics) = parse(src);

    // THEN I expect a diagnostic complaining about a missing return type
    let expected =
        Diagnostic::unexpected_token_found("COLON", "'a', (Identifier)", SourceRange::new(76..79));
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
    let src = r"
            FUNCTION foo :
            VAR END_VAR
            a;
            END_FUNCTION
            ";

    let (compilation_unit, diagnostics) = parse(src);
    //expected end of statement (e.g. ;), but found KeywordEndProgram at line: 1 offset: 14..25"
    //Expecting a missing semicolon message
    let expected = Diagnostic::unexpected_token_found(
        "DataTypeDefinition",
        "KeywordVar",
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
fn function_return_type_with_initializer() {
    let src = r"
            FUNCTION foo : INT := 3
            VAR END_VAR
            a;
            END_FUNCTION
            ";

    let (compilation_unit, diagnostics) = parse(src);
    //expected end of statement (e.g. ;), but found KeywordEndProgram at line: 1 offset: 14..25"
    //Expecting a missing semicolon message
    let expected = Diagnostic::unexpected_initializer_on_function_return(SourceRange::new(35..36));
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
    let src = r"
                PROGRAM foo : INT
                VAR END_VAR
                a;
                END_PROGRAM
                ";

    let (compilation_unit, diagnostics) = parse(src);
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
            vec![AstStatement::Reference {
                name: "a".into(),
                location: SourceRange::undefined(),
                id: 0
            }]
        )
    );
}

#[test]
fn unclosed_var_container() {
    let src = r"
                PROGRAM foo
                    VAR a : INT;
                    VAR b : INT; END_VAR
                END_PROGRAM
                ";

    let (compilation_unit, diagnostics) = parse(src);
    assert_eq!(
        vec![Diagnostic::unexpected_token_found(
            "KeywordEndVar",
            "'VAR b : INT;'",
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
                constant: false,
                access: AccessModifier::Protected,
                retain: false,
                variable_block_type: VariableBlockType::Local,
                location: SourceRange::undefined(),
                variables: vec![Variable {
                    name: "a".to_rc(),
                    data_type: crate::ast::DataTypeDeclaration::DataTypeReference {
                        referenced_type: "INT".to_rc(),
                        location: SourceRange::undefined(),
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
    let src = "TYPE MyType:
                PROGRAM
                END_PROGRAM
            END_TYPE
    ";
    let (_, diagnostics) = parse(src);
    assert_eq!(
        vec![
            Diagnostic::unexpected_token_found(
                "DataTypeDefinition",
                "KeywordProgram",
                (29..36).into(),
            ),
            Diagnostic::unexpected_token_found(
                "KeywordSemicolon",
                "'PROGRAM\n                END_PROGRAM\n            END_TYPE'",
                (29..85).into(),
            ),
            Diagnostic::unexpected_token_found("KeywordSemicolon", "''", (90..90).into(),),
        ],
        diagnostics
    );
}

#[test]
fn a_program_needs_to_end_with_end_program() {
    let src = "PROGRAM buz ";
    let (_, diagnostics) = parse(src);
    assert_eq!(
        diagnostics,
        vec![Diagnostic::unexpected_token_found(
            "KeywordEndProgram",
            "''",
            (12..12).into()
        ),]
    );
}

#[test]
fn a_variable_declaration_block_needs_to_end_with_endvar() {
    let src = "PROGRAM buz VAR END_PROGRAM ";
    let (_, diagnostics) = parse(src);

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::missing_token("[KeywordEndVar]", (16..27).into()),
            Diagnostic::unexpected_token_found("KeywordEndVar", "'END_PROGRAM'", (16..27).into()),
        ]
    );
}
