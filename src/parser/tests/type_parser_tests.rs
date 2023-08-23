use crate::test_utils::tests::parse;
use plc_ast::{
    ast::{AstStatement, DataType, DataTypeDeclaration, SourceRange, UserTypeDeclaration, Variable},
    literals::AstLiteral,
};
use plc_diagnostics::diagnostics::Diagnostic;
use pretty_assertions::*;

#[test]
fn multi_type_declaration() {
    let (result, ..) = parse(
        r#"
        TYPE
            Point2D : STRUCT
                x,y : INT;
            END_STRUCT
            Point3D : STRUCT
                x,y,z : INT;
            END_STRUCT
        END_TYPE
        "#,
    );
    insta::assert_debug_snapshot!(result);
}

#[test]
fn simple_struct_type_can_be_parsed() {
    let (result, ..) = parse(
        r#"
        TYPE SampleStruct :
            STRUCT
                One:INT;
                Two:INT;
                Three:INT;
            END_STRUCT
        END_TYPE 
        "#,
    );

    let ast_string = format!("{:#?}", &result.user_types[0]);

    let expected_ast = format!(
        "{:#?}",
        &UserTypeDeclaration {
            data_type: DataType::StructType {
                name: Some("SampleStruct".to_string(),),
                variables: vec!(
                    Variable {
                        name: "One".to_string(),
                        data_type_declaration: DataTypeDeclaration::DataTypeReference {
                            referenced_type: "INT".to_string(),
                            location: SourceRange::undefined(),
                        },
                        initializer: None,
                        address: None,
                        location: SourceRange::undefined(),
                    },
                    Variable {
                        name: "Two".to_string(),
                        data_type_declaration: DataTypeDeclaration::DataTypeReference {
                            referenced_type: "INT".to_string(),
                            location: SourceRange::undefined(),
                        },
                        initializer: None,
                        address: None,
                        location: SourceRange::undefined(),
                    },
                    Variable {
                        name: "Three".to_string(),
                        data_type_declaration: DataTypeDeclaration::DataTypeReference {
                            referenced_type: "INT".to_string(),
                            location: SourceRange::undefined(),
                        },
                        initializer: None,
                        address: None,
                        location: SourceRange::undefined(),
                    },
                ),
            },
            initializer: None,
            location: SourceRange::undefined(),
            scope: None,
        }
    );
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn simple_enum_type_can_be_parsed() {
    let (result, ..) = parse(
        r#"
        TYPE SampleEnum : (red, yellow, green);
        END_TYPE 
        "#,
    );
    insta::assert_debug_snapshot!(result.user_types[0]);
}

#[test]
fn simple_enum_with_numeric_type_can_be_parsed() {
    let (result, ..) = parse(
        r#"
        TYPE SampleEnum : INT (red, yellow, green);
        END_TYPE 
        "#,
    );
    insta::assert_debug_snapshot!(result.user_types[0]);
}

#[test]
fn simple_enum_with_one_element_numeric_type_can_be_parsed() {
    let (result, ..) = parse(
        r#"
        TYPE SampleEnum : INT (red);
        END_TYPE 
        "#,
    );
    insta::assert_debug_snapshot!(result.user_types[0]);
}

#[test]
fn typed_enum_with_initial_values_can_be_parsed() {
    let (result, ..) = parse(
        r#"
        TYPE SampleEnum : INT (red := 1, yellow := 2, green := 4);
        END_TYPE 
        "#,
    );
    insta::assert_debug_snapshot!(result.user_types[0]);
}

#[test]
fn typed_inline_enum_with_initial_values_can_be_parsed() {
    let (result, ..) = parse(
        r#"
        PROGRAM prg
        VAR
            x : INT (red := 1, yellow := 2, green := 4);
        END_VAR
        END_PROGRAM 
        "#,
    );
    insta::assert_debug_snapshot!(result.units[0]);
}

#[test]
fn type_alias_can_be_parsed() {
    let (result, ..) = parse(
        r#"
        TYPE 
            MyInt : INT;
        END_TYPE
        "#,
    );

    let ast_string = format!("{:#?}", &result.user_types[0]);
    let exptected_ast = format!(
        "{:#?}",
        &UserTypeDeclaration {
            data_type: DataType::SubRangeType {
                name: Some("MyInt".to_string()),
                referenced_type: "INT".to_string(),
                bounds: None,
            },
            initializer: None,
            location: SourceRange::undefined(),
            scope: None,
        }
    );

    assert_eq!(ast_string, exptected_ast);
}

#[test]
fn array_type_can_be_parsed_test() {
    let (result, ..) = parse(
        r#"
            TYPE MyArray : ARRAY[0..8] OF INT; END_TYPE
            "#,
    );

    let ast_string = format!("{:#?}", &result.user_types[0]);

    let expected_ast = format!(
        "{:#?}",
        &UserTypeDeclaration {
            data_type: DataType::ArrayType {
                name: Some("MyArray".to_string()),
                bounds: AstStatement::RangeStatement {
                    start: Box::new(AstStatement::Literal {
                        kind: AstLiteral::new_integer(0),
                        location: SourceRange::undefined(),
                        id: 0,
                    }),
                    end: Box::new(AstStatement::Literal {
                        kind: AstLiteral::new_integer(8),
                        location: SourceRange::undefined(),
                        id: 0,
                    }),
                    id: 0,
                },
                referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                    referenced_type: "INT".to_string(),
                    location: SourceRange::undefined(),
                }),
                is_variable_length: false,
            },
            initializer: None,
            location: SourceRange::undefined(),
            scope: None,
        }
    );

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn string_type_can_be_parsed_test() {
    let (result, ..) = parse(
        r#"
            TYPE MyString : STRING[253]; END_TYPE
            TYPE MyString : STRING[253] := 'abc'; END_TYPE
            "#,
    );

    let ast_string = format!("{:#?}", &result.user_types);

    let expected_ast = format!(
        "{:#?}",
        vec![
            UserTypeDeclaration {
                data_type: DataType::StringType {
                    name: Some("MyString".to_string()),
                    size: Some(AstStatement::Literal {
                        kind: AstLiteral::new_integer(253),
                        location: (10..11).into(),
                        id: 0
                    }),
                    is_wide: false,
                },
                initializer: None,
                location: SourceRange::undefined(),
                scope: None,
            },
            UserTypeDeclaration {
                data_type: DataType::StringType {
                    name: Some("MyString".to_string()),
                    size: Some(AstStatement::Literal {
                        kind: AstLiteral::new_integer(253),
                        location: (10..11).into(),
                        id: 0
                    }),
                    is_wide: false,
                },
                initializer: Some(AstStatement::Literal {
                    kind: AstLiteral::new_string("abc".into(), false),
                    location: SourceRange::undefined(),
                    id: 0,
                }),
                location: SourceRange::undefined(),
                scope: None,
            }
        ]
    );

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn wide_string_type_can_be_parsed_test() {
    let (result, ..) = parse(
        r#"
            TYPE MyString : WSTRING[253]; END_TYPE
            "#,
    );

    let ast_string = format!("{:#?}", &result.user_types[0]);

    let expected_ast = format!(
        "{:#?}",
        &UserTypeDeclaration {
            data_type: DataType::StringType {
                name: Some("MyString".to_string()),
                size: Some(AstStatement::Literal {
                    kind: AstLiteral::new_integer(253),
                    location: (10..11).into(),
                    id: 0
                }),
                is_wide: true,
            },
            initializer: None,
            location: SourceRange::undefined(),
            scope: None,
        }
    );

    assert_eq!(ast_string, expected_ast);
}

#[test]
fn subrangetype_can_be_parsed() {
    let src = "
            VAR_GLOBAL
                x : UINT(0..1000);
            END_VAR
           ";
    let (parse_result, ..) = parse(src);

    let x = &parse_result.global_vars[0].variables[0];
    let expected = Variable {
        name: "x".to_string(),
        data_type_declaration: DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::SubRangeType {
                name: None,
                bounds: Some(AstStatement::RangeStatement {
                    start: Box::new(AstStatement::Literal {
                        kind: AstLiteral::new_integer(0),
                        location: SourceRange::undefined(),
                        id: 0,
                    }),
                    end: Box::new(AstStatement::Literal {
                        kind: AstLiteral::new_integer(1000),
                        location: SourceRange::undefined(),
                        id: 0,
                    }),
                    id: 0,
                }),
                referenced_type: "UINT".to_string(),
            },
            location: SourceRange::undefined(),
            scope: None,
        },
        initializer: None,
        address: None,
        location: (0..0).into(),
    };
    assert_eq!(format!("{expected:#?}"), format!("{x:#?}").as_str());
}

#[test]
fn struct_with_inline_array_can_be_parsed() {
    let (result, ..) = parse(
        r#"
        TYPE SampleStruct :
            STRUCT
                One: ARRAY[0..1] OF INT;
            END_STRUCT
        END_TYPE 
        "#,
    );

    let ast_string = format!("{:#?}", &result.user_types[0]);

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
                        is_variable_length: false,
                    },
                },
            },
        ],
    },
    initializer: None,
    scope: None,
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn pointer_type_test() {
    let (result, diagnostics) = parse(
        r#"
        TYPE SamplePointer :
            POINTER TO INT;
        END_TYPE 
        "#,
    );
    let pointer_type = &result.user_types[0];
    let expected = UserTypeDeclaration {
        data_type: DataType::PointerType {
            name: Some("SamplePointer".into()),
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "INT".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        location: SourceRange::undefined(),
        initializer: None,
        scope: None,
    };
    assert_eq!(format!("{expected:#?}"), format!("{pointer_type:#?}").as_str());
    assert_eq!(diagnostics.len(), 1);
    let diagnostic = Diagnostic::ImprovementSuggestion {
        message: "'POINTER TO' is not a standard keyword, use REF_TO instead".to_string(),
        range: vec![(42..49).into()],
    };
    assert_eq!(diagnostics[0], diagnostic);
}

#[test]
fn ref_type_test() {
    let (result, diagnostics) = parse(
        r#"
        TYPE SampleReference :
            REF_TO INT;
        END_TYPE 
        "#,
    );
    let reference_type = &result.user_types[0];
    let expected = UserTypeDeclaration {
        data_type: DataType::PointerType {
            name: Some("SampleReference".into()),
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "INT".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        location: SourceRange::undefined(),
        initializer: None,
        scope: None,
    };
    assert_eq!(format!("{expected:#?}"), format!("{reference_type:#?}").as_str());
    assert_eq!(diagnostics.len(), 0)
}

#[test]
fn global_pointer_declaration() {
    let (result, diagnostics) = parse(
        r#"
        VAR_GLOBAL 
            SampleReference : REF_TO INT;
            SamplePointer : POINTER TO INT;
        END_VAR 
        "#,
    );
    let reference_type = &result.global_vars[0].variables[0];
    let expected = Variable {
        name: "SampleReference".into(),
        data_type_declaration: DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::PointerType {
                name: None,
                referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                    referenced_type: "INT".to_string(),
                    location: SourceRange::undefined(),
                }),
            },
            location: SourceRange::undefined(),
            scope: None,
        },
        initializer: None,
        address: None,
        location: (0..0).into(),
    };
    assert_eq!(format!("{expected:#?}"), format!("{reference_type:#?}").as_str());
    let pointer_type = &result.global_vars[0].variables[1];
    let expected = Variable {
        name: "SamplePointer".into(),
        data_type_declaration: DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::PointerType {
                name: None,
                referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                    referenced_type: "INT".to_string(),
                    location: SourceRange::undefined(),
                }),
            },
            location: SourceRange::undefined(),
            scope: None,
        },
        initializer: None,
        address: None,
        location: (0..0).into(),
    };
    assert_eq!(format!("{expected:#?}"), format!("{pointer_type:#?}").as_str());
    assert_eq!(diagnostics.len(), 1);
    let diagnostic = Diagnostic::ImprovementSuggestion {
        message: "'POINTER TO' is not a standard keyword, use REF_TO instead".to_string(),
        range: vec![(91..98).into()],
    };
    assert_eq!(diagnostics[0], diagnostic);
}

#[test]
fn variable_length_array_can_be_parsed() {
    let (parse_result, diagnostics) = parse(
        r#"
    VAR_GLOBAL
        x : ARRAY[*] OF INT;
    END_VAR
    "#,
    );
    assert_eq!(diagnostics.len(), 0);

    let x = &parse_result.global_vars[0].variables[0];
    let expected = Variable {
        name: "x".to_string(),
        data_type_declaration: DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::ArrayType {
                name: None,
                bounds: AstStatement::VlaRangeStatement { id: 0 },
                referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                    referenced_type: "INT".to_string(),
                    location: SourceRange::undefined(),
                }),
                is_variable_length: true,
            },
            location: SourceRange::undefined(),
            scope: None,
        },
        initializer: None,
        address: None,
        location: (0..0).into(),
    };
    assert_eq!(format!("{expected:#?}"), format!("{x:#?}").as_str());
}

#[test]
fn multi_dimensional_variable_length_arrays_can_be_parsed() {
    let (parse_result, diagnostics) = parse(
        r#"
    VAR_GLOBAL
        x : ARRAY[*, *] OF INT;
        y : ARRAY[*, *, *, *] OF INT;
    END_VAR
    "#,
    );

    assert_eq!(diagnostics.len(), 0);

    let var = &parse_result.global_vars[0].variables[0];
    let expected = Variable {
        name: "x".to_string(),
        data_type_declaration: DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::ArrayType {
                name: None,
                bounds: AstStatement::ExpressionList {
                    expressions: vec![
                        AstStatement::VlaRangeStatement { id: 0 },
                        AstStatement::VlaRangeStatement { id: 0 },
                    ],
                    id: 0,
                },
                referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                    referenced_type: "INT".to_string(),
                    location: SourceRange::undefined(),
                }),
                is_variable_length: true,
            },
            location: SourceRange::undefined(),
            scope: None,
        },
        initializer: None,
        address: None,
        location: (0..0).into(),
    };
    assert_eq!(format!("{expected:#?}"), format!("{var:#?}").as_str());

    let var = &parse_result.global_vars[0].variables[1];
    let expected = Variable {
        name: "y".to_string(),
        data_type_declaration: DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::ArrayType {
                name: None,
                bounds: AstStatement::ExpressionList {
                    expressions: vec![
                        AstStatement::VlaRangeStatement { id: 0 },
                        AstStatement::VlaRangeStatement { id: 0 },
                        AstStatement::VlaRangeStatement { id: 0 },
                        AstStatement::VlaRangeStatement { id: 0 },
                    ],
                    id: 0,
                },
                referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                    referenced_type: "INT".to_string(),
                    location: SourceRange::undefined(),
                }),
                is_variable_length: true,
            },
            location: SourceRange::undefined(),
            scope: None,
        },
        initializer: None,
        address: None,
        location: (0..0).into(),
    };
    assert_eq!(format!("{expected:#?}"), format!("{var:#?}").as_str());
}
