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
                    AstFactory::create_identifier("x", SourceLocation::internal(), 0),
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
            AstFactory::create_identifier("x", SourceLocation::internal(), 0),
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
        data_type_declaration: DataTypeDeclaration::Definition {
            data_type: Box::new(DataType::EnumType {
                name: None,
                numeric_type: DINT_TYPE.to_string(),
                elements: AstFactory::create_expression_list(
                    vec![ref_to("red"), ref_to("yellow"), ref_to("green")],
                    SourceLocation::internal(),
                    0,
                ),
            }),
            location: SourceLocation::internal(),
            scope: None,
        },
        initializer: None,
        address: None,
        location: SourceLocation::internal(),
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
    assert_snapshot!(format!("{x:#?}").as_str(), @r#"
    Variable {
        name: "x",
        data_type: DataTypeReference {
            referenced_type: "STRING",
        },
    }
    "#);

    let x = &parse_result.global_vars[0].variables[1];
    assert_snapshot!(format!("{x:#?}").as_str(), @r#"
    Variable {
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
    }
    "#);

    let x = &parse_result.global_vars[0].variables[2];
    assert_snapshot!(format!("{x:#?}").as_str(), @r#"
    Variable {
        name: "wx",
        data_type: DataTypeReference {
            referenced_type: "WSTRING",
        },
    }
    "#);

    let x = &parse_result.global_vars[0].variables[3];
    assert_snapshot!(format!("{x:#?}").as_str(), @r#"
    Variable {
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
    }
    "#);
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

#[test]
fn ref_assignment() {
    let result = &parse("PROGRAM main x REF= y END_PROGRAM").0.implementations[0];
    insta::assert_debug_snapshot!(result.statements, @r#"
    [
        ReferenceAssignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: None,
            },
            right: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "y",
                    },
                ),
                base: None,
            },
        },
    ]
    "#)
}

#[test]
fn reference_to_dint_declaration() {
    let (result, diagnostics) = parse(
        r"
        FUNCTION foo
            VAR
                bar : DINT;
                baz : REFERENCE TO DINT;
                qux : DINT;
            END_VAR
        END_FUNCTION
        ",
    );

    assert!(diagnostics.is_empty());
    insta::assert_debug_snapshot!(result.pous[0].variable_blocks[0], @r#"
    VariableBlock {
        variables: [
            Variable {
                name: "bar",
                data_type: DataTypeReference {
                    referenced_type: "DINT",
                },
            },
            Variable {
                name: "baz",
                data_type: DataTypeDefinition {
                    data_type: PointerType {
                        name: None,
                        referenced_type: DataTypeReference {
                            referenced_type: "DINT",
                        },
                        auto_deref: Some(
                            Reference,
                        ),
                        type_safe: true,
                        is_function: false,
                    },
                },
            },
            Variable {
                name: "qux",
                data_type: DataTypeReference {
                    referenced_type: "DINT",
                },
            },
        ],
        variable_block_type: Local,
    }
    "#);
}

#[test]
fn reference_to_string_declaration() {
    let (result, diagnostics) = parse(
        r"
        FUNCTION foo
            VAR
                foo : REFERENCE TO STRING;
            END_VAR
        END_FUNCTION
        ",
    );

    assert!(diagnostics.is_empty());
    insta::assert_debug_snapshot!(result.pous[0].variable_blocks[0], @r#"
    VariableBlock {
        variables: [
            Variable {
                name: "foo",
                data_type: DataTypeDefinition {
                    data_type: PointerType {
                        name: None,
                        referenced_type: DataTypeReference {
                            referenced_type: "STRING",
                        },
                        auto_deref: Some(
                            Reference,
                        ),
                        type_safe: true,
                        is_function: false,
                    },
                },
            },
        ],
        variable_block_type: Local,
    }
    "#);
}

#[test]
fn aliasing_dint_variable() {
    let (result, diagnostics) = parse(
        "
        FUNCTION main
            VAR
                a AT b : DINT; // equivalent to `a : REFERENCE TO DINT REF= b`
            END_VAR
        END_FUNCTION
        ",
    );

    assert_eq!(diagnostics, vec![]);
    insta::assert_debug_snapshot!(result.pous[0].variable_blocks[0], @r#"
    VariableBlock {
        variables: [
            Variable {
                name: "a",
                data_type: DataTypeDefinition {
                    data_type: PointerType {
                        name: None,
                        referenced_type: DataTypeReference {
                            referenced_type: "DINT",
                        },
                        auto_deref: Some(
                            Alias,
                        ),
                        type_safe: true,
                        is_function: false,
                    },
                },
                initializer: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "b",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        ],
        variable_block_type: Local,
    }
    "#);
}
