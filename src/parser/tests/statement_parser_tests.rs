use crate::{
    ast::{DataType, DataTypeDeclaration, SourceRange, Statement, Variable},
    parser::{parse, tests::lex},
};
use pretty_assertions::*;

#[test]
fn empty_statements_are_are_parsed() {
    let lexer = lex("PROGRAM buz ;;;; END_PROGRAM ");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    assert_eq!(
        format!("{:?}", prg.statements),
        format!(
            "{:?}",
            vec![
                Statement::EmptyStatement {
                    location: SourceRange::undefined()
                },
                Statement::EmptyStatement {
                    location: SourceRange::undefined()
                },
                Statement::EmptyStatement {
                    location: SourceRange::undefined()
                },
                Statement::EmptyStatement {
                    location: SourceRange::undefined()
                },
            ]
        ),
    );
}

#[test]
fn empty_statements_are_parsed_before_a_statement() {
    let lexer = lex("PROGRAM buz ;;;;x; END_PROGRAM ");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];

    assert_eq!(
        format!("{:?}", prg.statements),
        format!(
            "{:?}",
            vec![
                Statement::EmptyStatement {
                    location: SourceRange::undefined()
                },
                Statement::EmptyStatement {
                    location: SourceRange::undefined()
                },
                Statement::EmptyStatement {
                    location: SourceRange::undefined()
                },
                Statement::EmptyStatement {
                    location: SourceRange::undefined()
                },
                Statement::Reference {
                    name: "x".into(),
                    location: SourceRange::undefined()
                },
            ]
        ),
    );
}

#[test]
fn empty_statements_are_ignored_after_a_statement() {
    let lexer = lex("PROGRAM buz x;;;; END_PROGRAM ");
    let result = parse(lexer).unwrap().0;

    let prg = &result.implementations[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"Reference {
    name: "x",
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn inline_struct_declaration_can_be_parsed() {
    let (result, ..) = parse(lex(r#"
        VAR_GLOBAL
            my_struct : STRUCT
                One:INT;
                Two:INT;
                Three:INT;
            END_STRUCT
        END_VAR
        "#))
    .unwrap();

    let ast_string = format!("{:#?}", &result.global_vars[0].variables[0]);
    let expected_ast = r#"Variable {
    name: "my_struct",
    data_type: DataTypeDefinition {
        data_type: StructType {
            name: None,
            variables: [
                Variable {
                    name: "One",
                    data_type: DataTypeReference {
                        referenced_type: "INT",
                    },
                },
                Variable {
                    name: "Two",
                    data_type: DataTypeReference {
                        referenced_type: "INT",
                    },
                },
                Variable {
                    name: "Three",
                    data_type: DataTypeReference {
                        referenced_type: "INT",
                    },
                },
            ],
        },
    },
}"#;

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn inline_enum_declaration_can_be_parsed() {
    let (result, ..) = parse(lex(r#"
        VAR_GLOBAL
            my_enum : (red, yellow, green);
        END_VAR
        "#))
    .unwrap();

    let ast_string = format!("{:#?}", &result.global_vars[0].variables[0]);

    let v = Variable {
        name: "my_enum".to_string(),
        data_type: DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::EnumType {
                name: None,
                elements: vec!["red".to_string(), "yellow".to_string(), "green".to_string()],
            },
        },
        initializer: None,
        location: SourceRange::undefined(),
    };
    let expected_ast = format!("{:#?}", &v);
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn multilevel_inline_struct_and_enum_declaration_can_be_parsed() {
    let (result, ..) = parse(lex(r#"
        VAR_GLOBAL
            my_struct : STRUCT
                    inner_enum: (red, yellow, green);
                    inner_struct: STRUCT
                        field: INT;
                    END_STRUCT
                END_STRUCT
        END_VAR
        "#))
    .unwrap();

    let ast_string = format!("{:#?}", &result.global_vars[0].variables[0]);
    let expected_ast = r#"Variable {
    name: "my_struct",
    data_type: DataTypeDefinition {
        data_type: StructType {
            name: None,
            variables: [
                Variable {
                    name: "inner_enum",
                    data_type: DataTypeDefinition {
                        data_type: EnumType {
                            name: None,
                            elements: [
                                "red",
                                "yellow",
                                "green",
                            ],
                        },
                    },
                },
                Variable {
                    name: "inner_struct",
                    data_type: DataTypeDefinition {
                        data_type: StructType {
                            name: None,
                            variables: [
                                Variable {
                                    name: "field",
                                    data_type: DataTypeReference {
                                        referenced_type: "INT",
                                    },
                                },
                            ],
                        },
                    },
                },
            ],
        },
    },
}"#;

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn string_variable_declaration_can_be_parsed() {
    let lexer = lex("
            VAR_GLOBAL
                x : STRING;
                y : STRING[500];
                wx : WSTRING;
                wy : WSTRING[500];
            END_VAR
           ");
    let (parse_result, ..) = parse(lexer).unwrap();
    let x = &parse_result.global_vars[0].variables[0];
    let expected = r#"Variable {
    name: "x",
    data_type: DataTypeDefinition {
        data_type: StringType {
            name: None,
            is_wide: false,
            size: None,
        },
    },
}"#;
    assert_eq!(expected, format!("{:#?}", x).as_str());

    let x = &parse_result.global_vars[0].variables[1];
    let expected = r#"Variable {
    name: "y",
    data_type: DataTypeDefinition {
        data_type: StringType {
            name: None,
            is_wide: false,
            size: Some(
                LiteralInteger {
                    value: "500",
                },
            ),
        },
    },
}"#;
    assert_eq!(expected, format!("{:#?}", x).as_str());

    let x = &parse_result.global_vars[0].variables[2];
    let expected = r#"Variable {
    name: "wx",
    data_type: DataTypeDefinition {
        data_type: StringType {
            name: None,
            is_wide: true,
            size: None,
        },
    },
}"#;
    assert_eq!(expected, format!("{:#?}", x).as_str());

    let x = &parse_result.global_vars[0].variables[3];
    let expected = r#"Variable {
    name: "wy",
    data_type: DataTypeDefinition {
        data_type: StringType {
            name: None,
            is_wide: true,
            size: Some(
                LiteralInteger {
                    value: "500",
                },
            ),
        },
    },
}"#;
    assert_eq!(expected, format!("{:#?}", x).as_str());
}
