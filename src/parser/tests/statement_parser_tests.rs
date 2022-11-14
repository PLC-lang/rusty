use crate::{
    ast::{AstStatement, DataType, DataTypeDeclaration, SourceRange, Variable},
    parser::tests::ref_to,
    test_utils::tests::parse,
    typesystem::DINT_TYPE,
};
use pretty_assertions::*;

#[test]
fn empty_statements_are_are_parsed() {
    let src = "PROGRAM buz ;;;; END_PROGRAM ";
    let result = parse(src).0;

    let prg = &result.implementations[0];
    assert_eq!(
        format!("{:?}", prg.statements),
        format!(
            "{:?}",
            vec![
                AstStatement::EmptyStatement {
                    location: SourceRange::undefined(),
                    id: 0
                },
                AstStatement::EmptyStatement {
                    location: SourceRange::undefined(),
                    id: 0
                },
                AstStatement::EmptyStatement {
                    location: SourceRange::undefined(),
                    id: 0
                },
                AstStatement::EmptyStatement {
                    location: SourceRange::undefined(),
                    id: 0
                },
            ]
        ),
    );
}

#[test]
fn empty_statements_are_parsed_before_a_statement() {
    let src = "PROGRAM buz ;;;;x; END_PROGRAM ";
    let result = parse(src).0;

    let prg = &result.implementations[0];

    assert_eq!(
        format!("{:?}", prg.statements),
        format!(
            "{:?}",
            vec![
                AstStatement::EmptyStatement {
                    location: SourceRange::undefined(),
                    id: 0
                },
                AstStatement::EmptyStatement {
                    location: SourceRange::undefined(),
                    id: 0
                },
                AstStatement::EmptyStatement {
                    location: SourceRange::undefined(),
                    id: 0
                },
                AstStatement::EmptyStatement {
                    location: SourceRange::undefined(),
                    id: 0
                },
                AstStatement::Reference {
                    name: "x".into(),
                    location: SourceRange::undefined(),
                    id: 0
                },
            ]
        ),
    );
}

#[test]
fn empty_statements_are_ignored_after_a_statement() {
    let src = "PROGRAM buz x;;;; END_PROGRAM ";
    let result = parse(src).0;

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
    let (result, ..) = parse(
        r#"
        VAR_GLOBAL
            my_struct : STRUCT
                One:INT;
                Two:INT;
                Three:INT;
            END_STRUCT
        END_VAR
        "#,
    );

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
    let (result, ..) = parse(
        r#"
        VAR_GLOBAL
            my_enum : (red, yellow, green);
        END_VAR
        "#,
    );

    let ast_string = format!("{:#?}", &result.global_vars[0].variables[0]);

    let v = Variable {
        name: "my_enum".to_string(),
        data_type: DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::EnumType {
                name: None,
                numeric_type: DINT_TYPE.to_string(),
                elements: AstStatement::ExpressionList {
                    expressions: vec![ref_to("red"), ref_to("yellow"), ref_to("green")],
                    id: 0,
                },
            },
            location: SourceRange::undefined(),
            scope: None,
        },
        initializer: None,
        address: None,
        location: SourceRange::undefined(),
    };
    let expected_ast = format!("{:#?}", &v);
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn multilevel_inline_struct_and_enum_declaration_can_be_parsed() {
    let (result, ..) = parse(
        r#"
        VAR_GLOBAL
            my_struct : STRUCT
                    inner_enum: (red, yellow, green);
                    inner_struct: STRUCT
                        field: INT;
                    END_STRUCT
                END_STRUCT
        END_VAR
        "#,
    );

    let ast_string = format!("{:#?}", &result.global_vars[0].variables[0]);
    insta::assert_snapshot!(ast_string);
}

#[test]
fn string_variable_declaration_can_be_parsed() {
    let src = "
            VAR_GLOBAL
                x : STRING;
                y : STRING[500];
                wx : WSTRING;
                wy : WSTRING[500];
            END_VAR
           ";
    let (parse_result, ..) = parse(src);
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
                    value: 500,
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
                    value: 500,
                },
            ),
        },
    },
}"#;
    assert_eq!(expected, format!("{:#?}", x).as_str());
}

#[test]
fn empty_parameter_assignments_in_call_statement() {
    let (result, diagnostics) = parse(
        r#"
        FUNCTION foo : INT
		VAR_INPUT
			input1 : INT;
		END_VAR
		VAR_OUTPUT
			output1 : INT;
		END_VAR
		VAR_IN_OUT
			inout1 : INT;
		END_VAR
		END_FUNCTION

		PROGRAM main
		VAR
			a, b, c : INT;
		END_VAR
		foo(input1 := , output1 => , inout1 => );
		END_PROGRAM
        "#,
    );

    assert_eq!(diagnostics, vec![]);

    let ast_string = format!("{:#?}", &result);
    insta::assert_snapshot!(ast_string);
}
