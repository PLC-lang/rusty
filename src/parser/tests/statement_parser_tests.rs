use crate::{
    parser::tests::{empty_stmt, ref_to},
    test_utils::tests::parse,
    typesystem::DINT_TYPE,
};
use insta::assert_snapshot;
use plc_ast::ast::{AstFactory, DataType, DataTypeDeclaration, Variable};
use plc_source::source_location::SourceLocation;
use pretty_assertions::*;

#[test]
fn empty_statements_are_are_parsed() {
    let src = "PROGRAM buz ;;;; END_PROGRAM ";
    let result = parse(src).0;

    let prg = &result.implementations[0];

    assert_eq!(
        format!("{:?}", prg.statements),
        format!("{:?}", vec![empty_stmt(), empty_stmt(), empty_stmt(), empty_stmt(),]),
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
                empty_stmt(),
                empty_stmt(),
                empty_stmt(),
                empty_stmt(),
                AstFactory::create_member_reference(
                    AstFactory::create_identifier("x", &SourceLocation::undefined(), 0),
                    None,
                    0
                ),
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

    let ast_string = format!("{statement:#?}");
    let expected_ast = format!(
        "{:#?}",
        AstFactory::create_member_reference(
            AstFactory::create_identifier("x", &SourceLocation::undefined(), 0),
            None,
            0
        )
    );
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
        data_type_declaration: DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::EnumType {
                name: None,
                numeric_type: DINT_TYPE.to_string(),
                elements: AstFactory::create_expression_list(
                    vec![ref_to("red"), ref_to("yellow"), ref_to("green")],
                    SourceLocation::undefined(),
                    0,
                ),
            },
            location: SourceLocation::undefined(),
            scope: None,
        },
        initializer: None,
        address: None,
        location: SourceLocation::undefined(),
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
    assert_snapshot!(format!("{x:#?}").as_str(), @r###"
    Variable {
        name: "x",
        data_type: DataTypeReference {
            referenced_type: "STRING",
        },
    }
    "###);

    let x = &parse_result.global_vars[0].variables[1];
    assert_snapshot!(format!("{x:#?}").as_str(), @r#"Variable {
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
}"#);

    let x = &parse_result.global_vars[0].variables[2];
    assert_snapshot!(format!("{x:#?}").as_str(), @r###"
    Variable {
        name: "wx",
        data_type: DataTypeReference {
            referenced_type: "WSTRING",
        },
    }
    "###);

    let x = &parse_result.global_vars[0].variables[3];
    assert_snapshot!(format!("{x:#?}").as_str(), @r#"Variable {
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
}"#);
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
