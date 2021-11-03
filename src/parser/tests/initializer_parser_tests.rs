use crate::test_utils::tests::parse;

#[test]
fn initial_scalar_values_can_be_parsed() {
    let src = "
            VAR_GLOBAL
                x : INT := 7;
            END_VAR

            TYPE MyStruct :
                STRUCT
                    a: INT := 69;
                    b: BOOL := TRUE;
                    c: REAL := 5.25;
                END_STRUCT
            END_TYPE

            TYPE MyInt : INT := 789;
            END_TYPE

            PROGRAM MY_PRG
                VAR
                    y : REAL := 11.3;
                END_VAR
            END_PROGRAM
            ";
    let (parse_result, ..) = parse(src);

    let x = &parse_result.global_vars[0].variables[0];
    let expected = r#"Variable {
    name: "x",
    data_type: DataTypeReference {
        referenced_type: "INT",
    },
    initializer: Some(
        LiteralInteger {
            value: 7,
        },
    ),
}"#;
    assert_eq!(expected, format!("{:#?}", x).as_str());

    let struct_type = &parse_result.types[0];
    let expected = r#"UserTypeDeclaration {
    data_type: StructType {
        name: Some(
            "MyStruct",
        ),
        variables: [
            Variable {
                name: "a",
                data_type: DataTypeReference {
                    referenced_type: "INT",
                },
                initializer: Some(
                    LiteralInteger {
                        value: 69,
                    },
                ),
            },
            Variable {
                name: "b",
                data_type: DataTypeReference {
                    referenced_type: "BOOL",
                },
                initializer: Some(
                    LiteralBool {
                        value: true,
                    },
                ),
            },
            Variable {
                name: "c",
                data_type: DataTypeReference {
                    referenced_type: "REAL",
                },
                initializer: Some(
                    LiteralReal {
                        value: "5.25",
                    },
                ),
            },
        ],
    },
    initializer: None,
    scope: None,
}"#;
    assert_eq!(expected, format!("{:#?}", struct_type).as_str());

    let my_int_type = &parse_result.types[1];
    let expected = r#"UserTypeDeclaration {
    data_type: SubRangeType {
        name: Some(
            "MyInt",
        ),
        referenced_type: "INT",
        bounds: None,
    },
    initializer: Some(
        LiteralInteger {
            value: 789,
        },
    ),
    scope: None,
}"#;
    assert_eq!(expected, format!("{:#?}", my_int_type).as_str());

    let y = &parse_result.units[0].variable_blocks[0].variables[0];
    let expected = r#"Variable {
    name: "y",
    data_type: DataTypeReference {
        referenced_type: "REAL",
    },
    initializer: Some(
        LiteralReal {
            value: "11.3",
        },
    ),
}"#;

    assert_eq!(expected, format!("{:#?}", y).as_str());
}

#[test]
fn array_initializer_can_be_parsed() {
    let src = "
            VAR_GLOBAL
                x : ARRAY[0..2] OF INT := [7,8,9];
            END_VAR
           ";
    let (parse_result, ..) = parse(src);
    let x = &parse_result.global_vars[0].variables[0];
    let expected = r#"Variable {
    name: "x",
    data_type: DataTypeDefinition {
        data_type: ArrayType {
            name: None,
            bounds: RangeStatement {
                start: LiteralInteger {
                    value: 0,
                },
                end: LiteralInteger {
                    value: 2,
                },
            },
            referenced_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
    },
    initializer: Some(
        LiteralArray {
            elements: Some(
                ExpressionList {
                    expressions: [
                        LiteralInteger {
                            value: 7,
                        },
                        LiteralInteger {
                            value: 8,
                        },
                        LiteralInteger {
                            value: 9,
                        },
                    ],
                },
            ),
        },
    ),
}"#;
    assert_eq!(expected, format!("{:#?}", x).as_str());
}

#[test]
fn multi_dim_array_initializer_can_be_parsed() {
    let src = "
            VAR_GLOBAL
                x : MyMultiArray := [[1,2],[3,4],[5,6]];
            END_VAR
           ";
    let (parse_result, ..) = parse(src);
    let x = &parse_result.global_vars[0].variables[0];
    let expected = r#"Variable {
    name: "x",
    data_type: DataTypeReference {
        referenced_type: "MyMultiArray",
    },
    initializer: Some(
        LiteralArray {
            elements: Some(
                ExpressionList {
                    expressions: [
                        LiteralArray {
                            elements: Some(
                                ExpressionList {
                                    expressions: [
                                        LiteralInteger {
                                            value: 1,
                                        },
                                        LiteralInteger {
                                            value: 2,
                                        },
                                    ],
                                },
                            ),
                        },
                        LiteralArray {
                            elements: Some(
                                ExpressionList {
                                    expressions: [
                                        LiteralInteger {
                                            value: 3,
                                        },
                                        LiteralInteger {
                                            value: 4,
                                        },
                                    ],
                                },
                            ),
                        },
                        LiteralArray {
                            elements: Some(
                                ExpressionList {
                                    expressions: [
                                        LiteralInteger {
                                            value: 5,
                                        },
                                        LiteralInteger {
                                            value: 6,
                                        },
                                    ],
                                },
                            ),
                        },
                    ],
                },
            ),
        },
    ),
}"#;
    assert_eq!(expected, format!("{:#?}", x).as_str());
}

#[test]
fn array_initializer_multiplier_can_be_parsed() {
    let src = "
            VAR_GLOBAL
                x : ARRAY[0..2] OF INT := [3(7)];
            END_VAR
           ";
    let (parse_result, ..) = parse(src);
    let x = &parse_result.global_vars[0].variables[0];
    let expected = r#"Variable {
    name: "x",
    data_type: DataTypeDefinition {
        data_type: ArrayType {
            name: None,
            bounds: RangeStatement {
                start: LiteralInteger {
                    value: 0,
                },
                end: LiteralInteger {
                    value: 2,
                },
            },
            referenced_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
    },
    initializer: Some(
        LiteralArray {
            elements: Some(
                MultipliedStatement {
                    multiplier: 3,
                    element: LiteralInteger {
                        value: 7,
                    },
                },
            ),
        },
    ),
}"#;
    assert_eq!(expected, format!("{:#?}", x).as_str());
}

#[test]
fn struct_initializer_can_be_parsed() {
    let src = "
            VAR_GLOBAL
                x : Point := (x := 1, y:= 2);
            END_VAR
           ";
    let (parse_result, ..) = parse(src);
    let x = &parse_result.global_vars[0].variables[0];
    let expected = r#"Variable {
    name: "x",
    data_type: DataTypeReference {
        referenced_type: "Point",
    },
    initializer: Some(
        ExpressionList {
            expressions: [
                Assignment {
                    left: Reference {
                        name: "x",
                    },
                    right: LiteralInteger {
                        value: 1,
                    },
                },
                Assignment {
                    left: Reference {
                        name: "y",
                    },
                    right: LiteralInteger {
                        value: 2,
                    },
                },
            ],
        },
    ),
}"#;
    assert_eq!(expected, format!("{:#?}", x).as_str());
}

#[test]
fn array_initializer_in_pou_can_be_parsed() {
    let (result, ..) = parse(
        r#"
            PROGRAM main
            VAR
                my_array: ARRAY[0..2] OF INT := [5,6,7];
            END_VAR
            END_PROGRAM
            "#,
    );

    let member = &result.units[0].variable_blocks[0].variables[0];
    if let Some(initializer) = &member.initializer {
        let ast_string = format!("{:#?}", initializer);
        let expected_ast = r#"LiteralArray {
    elements: Some(
        ExpressionList {
            expressions: [
                LiteralInteger {
                    value: 5,
                },
                LiteralInteger {
                    value: 6,
                },
                LiteralInteger {
                    value: 7,
                },
            ],
        },
    ),
}"#;
        assert_eq!(ast_string, expected_ast);
    }
}

#[test]
fn array_type_initialization_with_literals_can_be_parsed_test() {
    let (result, ..) = parse(
        r#"
            TYPE MyArray : ARRAY[0..2] OF INT := [1,2,3]; END_TYPE
            "#,
    );

    let initializer = &result.types[0].initializer;
    let ast_string = format!("{:#?}", &initializer);

    let expected_ast = r#"Some(
    LiteralArray {
        elements: Some(
            ExpressionList {
                expressions: [
                    LiteralInteger {
                        value: 1,
                    },
                    LiteralInteger {
                        value: 2,
                    },
                    LiteralInteger {
                        value: 3,
                    },
                ],
            },
        ),
    },
)"#;
    assert_eq!(ast_string, expected_ast);
}
