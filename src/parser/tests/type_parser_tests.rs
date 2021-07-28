use crate::{
    ast::*,
    parser::{parse, tests::lex, Statement::LiteralInteger},
};
use pretty_assertions::*;

#[test]
fn simple_struct_type_can_be_parsed() {
    let (result, ..) = parse(lex(r#"
        TYPE SampleStruct :
            STRUCT
                One:INT;
                Two:INT;
                Three:INT;
            END_STRUCT
        END_TYPE 
        "#))
    .unwrap();

    let ast_string = format!("{:#?}", &result.types[0]);

    let expected_ast = format!(
        "{:#?}",
        &UserTypeDeclaration {
            data_type: DataType::StructType {
                name: Some("SampleStruct".to_string(),),
                variables: vec!(
                    Variable {
                        name: "One".to_string(),
                        data_type: DataTypeDeclaration::DataTypeReference {
                            referenced_type: "INT".to_string(),
                        },
                        initializer: None,
                        location: SourceRange::undefined(),
                    },
                    Variable {
                        name: "Two".to_string(),
                        data_type: DataTypeDeclaration::DataTypeReference {
                            referenced_type: "INT".to_string(),
                        },
                        initializer: None,
                        location: SourceRange::undefined(),
                    },
                    Variable {
                        name: "Three".to_string(),
                        data_type: DataTypeDeclaration::DataTypeReference {
                            referenced_type: "INT".to_string(),
                        },
                        initializer: None,
                        location: SourceRange::undefined(),
                    },
                ),
            },
            initializer: None,
        }
    );
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn simple_enum_type_can_be_parsed() {
    let (result, ..) = parse(lex(r#"
        TYPE SampleEnum : (red, yellow, green);
        END_TYPE 
        "#))
    .unwrap();

    let ast_string = format!("{:#?}", &result.types[0]);

    let epxtected_ast = &UserTypeDeclaration {
        data_type: DataType::EnumType {
            name: Some("SampleEnum".to_string()),
            elements: vec!["red".to_string(), "yellow".to_string(), "green".to_string()],
        },
        initializer: None,
    };
    let expected_string = format!("{:#?}", epxtected_ast);
    assert_eq!(ast_string, expected_string);
}

#[test]
fn type_alias_can_be_parsed() {
    let (result, ..) = parse(lex(r#"
        TYPE 
            MyInt : INT;
        END_TYPE
        "#))
    .unwrap();

    let ast_string = format!("{:#?}", &result.types[0]);
    let exptected_ast = format!(
        "{:#?}",
        &UserTypeDeclaration {
            data_type: DataType::SubRangeType {
                name: Some("MyInt".to_string()),
                referenced_type: "INT".to_string(),
                bounds: None,
            },
            initializer: None,
        }
    );

    assert_eq!(ast_string, exptected_ast);
}

#[test]
fn array_type_can_be_parsed_test() {
    let (result, ..) = parse(lex(r#"
            TYPE MyArray : ARRAY[0..8] OF INT; END_TYPE
            "#))
    .unwrap();

    let ast_string = format!("{:#?}", &result.types[0]);

    let expected_ast = format!(
        "{:#?}",
        &UserTypeDeclaration {
            data_type: DataType::ArrayType {
                name: Some("MyArray".to_string()),
                bounds: Statement::RangeStatement {
                    start: Box::new(Statement::LiteralInteger {
                        value: 0,
                        location: SourceRange::undefined(),
                    }),
                    end: Box::new(Statement::LiteralInteger {
                        value: 8,
                        location: SourceRange::undefined(),
                    }),
                },
                referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                    referenced_type: "INT".to_string(),
                }),
            },
            initializer: None,
        }
    );

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn string_type_can_be_parsed_test() {
    let (result, ..) = parse(lex(r#"
            TYPE MyString : STRING[253]; END_TYPE
            TYPE MyString : STRING[253] := 'abc'; END_TYPE
            "#))
    .unwrap();

    let ast_string = format!("{:#?}", &result.types);

    let expected_ast = format!(
        "{:#?}",
        vec![
            UserTypeDeclaration {
                data_type: DataType::StringType {
                    name: Some("MyString".to_string()),
                    size: Some(LiteralInteger {
                        value: 253,
                        location: (10..11).into(),
                    }),
                    is_wide: false,
                },
                initializer: None,
            },
            UserTypeDeclaration {
                data_type: DataType::StringType {
                    name: Some("MyString".to_string()),
                    size: Some(LiteralInteger {
                        value: 253,
                        location: (10..11).into(),
                    }),
                    is_wide: false,
                },
                initializer: Some(Statement::LiteralString {
                    is_wide: false,
                    location: SourceRange::undefined(),
                    value: "abc".into(),
                }),
            }
        ]
    );

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn wide_string_type_can_be_parsed_test() {
    let (result, ..) = parse(lex(r#"
            TYPE MyString : WSTRING[253]; END_TYPE
            "#))
    .unwrap();

    let ast_string = format!("{:#?}", &result.types[0]);

    let expected_ast = format!(
        "{:#?}",
        &UserTypeDeclaration {
            data_type: DataType::StringType {
                name: Some("MyString".to_string()),
                size: Some(LiteralInteger {
                    value: 253,
                    location: (10..11).into(),
                }),
                is_wide: true,
            },
            initializer: None,
        }
    );

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn subrangetype_can_be_parsed() {
    let lexer = lex("
            VAR_GLOBAL
                x : UINT(0..1000);
            END_VAR
           ");
    let (parse_result, ..) = parse(lexer).unwrap();

    let x = &parse_result.global_vars[0].variables[0];
    let expected = Variable {
        name: "x".to_string(),
        data_type: DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::SubRangeType {
                name: None,
                bounds: Some(Statement::RangeStatement {
                    start: Box::new(LiteralInteger {
                        value: 0,
                        location: SourceRange::undefined(),
                    }),
                    end: Box::new(LiteralInteger {
                        value: 1000,
                        location: SourceRange::undefined(),
                    }),
                }),
                referenced_type: "UINT".to_string(),
            },
        },
        initializer: None,
        location: (0..0).into(),
    };
    assert_eq!(format!("{:#?}", expected), format!("{:#?}", x).as_str());
}

#[test]
fn struct_with_inline_array_can_be_parsed() {
    let (result, ..) = parse(lex(r#"
        TYPE SampleStruct :
            STRUCT
                One: ARRAY[0..1] OF INT;
            END_STRUCT
        END_TYPE 
        "#))
    .unwrap();

    let ast_string = format!("{:#?}", &result.types[0]);

    let expected_ast = r#"UserTypeDeclaration {
    data_type: StructType {
        name: Some(
            "SampleStruct",
        ),
        variables: [
            Variable {
                name: "One",
                data_type: DataTypeDefinition {
                    data_type: ArrayType {
                        name: None,
                        bounds: RangeStatement {
                            start: LiteralInteger {
                                value: 0,
                            },
                            end: LiteralInteger {
                                value: 1,
                            },
                        },
                        referenced_type: DataTypeReference {
                            referenced_type: "INT",
                        },
                    },
                },
            },
        ],
    },
    initializer: None,
}"#;
    assert_eq!(ast_string, expected_ast);
}
