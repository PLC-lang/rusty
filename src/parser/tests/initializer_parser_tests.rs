use insta::assert_debug_snapshot;

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
    assert_eq!(expected, format!("{x:#?}").as_str());

    let struct_type = &parse_result.user_types[0];
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
    assert_eq!(expected, format!("{struct_type:#?}").as_str());

    let my_int_type = &parse_result.user_types[1];
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
    assert_eq!(expected, format!("{my_int_type:#?}").as_str());

    let y = &parse_result.pous[0].variable_blocks[0].variables[0];
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

    assert_eq!(expected, format!("{y:#?}").as_str());
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
            is_variable_length: false,
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
    assert_eq!(expected, format!("{x:#?}").as_str());
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
    assert_eq!(expected, format!("{x:#?}").as_str());
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
            is_variable_length: false,
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
    assert_eq!(expected, format!("{x:#?}").as_str());
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
    assert_debug_snapshot!(x);
}

#[test]
fn parenthesized_expression_within_array() {
    let (result, _) = parse(
        "
        PROGRAM main
            VAR
                arr : ARRAY[1..5] OF DINT := [(1, 2, 3, 4, 5)];
            END_VAR
        END_PROGRAM
        ",
    );

    let member = &result.pous[0].variable_blocks[0].variables[0];
    assert_debug_snapshot!(&member.initializer);
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

    let member = &result.pous[0].variable_blocks[0].variables[0];
    if let Some(initializer) = &member.initializer {
        let ast_string = format!("{initializer:#?}");
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

    let initializer = &result.user_types[0].initializer;
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

#[test]
fn date_and_time_constants_can_be_parsed() {
    let src = r#"
    VAR_GLOBAL CONSTANT
        cT          : TIME              := TIME#1s;
        cT_SHORT    : TIME              := T#2h1s10ns;
        cLT         : LTIME             := LTIME#1h1s1ms1us1ns;
        cLT_SHORT   : LTIME             := LT#1000s;
        cD          : DATE              := DATE#1970-01-01;
        cD_SHORT    : DATE              := D#1970-01-01;
        cLD         : LDATE             := LDATE#1970-01-01;
        cLD_SHORT   : LDATE             := LD#1970-01-01;
        cTOD        : TIME_OF_DAY       := TIME_OF_DAY#00:00:00;
        cTOD_SHORT  : TOD               := TOD#00:00:00;
        cLTOD       : LTOD              := LTIME_OF_DAY#23:59:59.999999999;
        cLTOD_SHORT : LTOD              := LTOD#23:59:59.999999999;
        cDT         : DATE_AND_TIME     := DATE_AND_TIME#1970-01-01-23:59:59;
        cDT_SHORT   : DT                := DT#1970-01-01-23:59:59;
        cLDT        : LDT               := LDATE_AND_TIME#1970-01-01-23:59:59.123;
        cLDT_SHORT  : LDT               := LDT#1970-01-01-23:59:59.123;
    END_VAR"#;

    let (result, _) = parse(src);

    let vars = &result.global_vars[0].variables;
    let ast_string = format!("{:#?}", &vars);
    let expected_ast = r#"[
    Variable {
        name: "cT",
        data_type: DataTypeReference {
            referenced_type: "TIME",
        },
        initializer: Some(
            LiteralTime {
                day: 0.0,
                hour: 0.0,
                min: 0.0,
                sec: 1.0,
                milli: 0.0,
                micro: 0.0,
                nano: 0,
                negative: false,
            },
        ),
    },
    Variable {
        name: "cT_SHORT",
        data_type: DataTypeReference {
            referenced_type: "TIME",
        },
        initializer: Some(
            LiteralTime {
                day: 0.0,
                hour: 2.0,
                min: 0.0,
                sec: 1.0,
                milli: 0.0,
                micro: 0.0,
                nano: 10,
                negative: false,
            },
        ),
    },
    Variable {
        name: "cLT",
        data_type: DataTypeReference {
            referenced_type: "LTIME",
        },
        initializer: Some(
            LiteralTime {
                day: 0.0,
                hour: 1.0,
                min: 0.0,
                sec: 1.0,
                milli: 1.0,
                micro: 1.0,
                nano: 1,
                negative: false,
            },
        ),
    },
    Variable {
        name: "cLT_SHORT",
        data_type: DataTypeReference {
            referenced_type: "LTIME",
        },
        initializer: Some(
            LiteralTime {
                day: 0.0,
                hour: 0.0,
                min: 0.0,
                sec: 1000.0,
                milli: 0.0,
                micro: 0.0,
                nano: 0,
                negative: false,
            },
        ),
    },
    Variable {
        name: "cD",
        data_type: DataTypeReference {
            referenced_type: "DATE",
        },
        initializer: Some(
            LiteralDate {
                year: 1970,
                month: 1,
                day: 1,
            },
        ),
    },
    Variable {
        name: "cD_SHORT",
        data_type: DataTypeReference {
            referenced_type: "DATE",
        },
        initializer: Some(
            LiteralDate {
                year: 1970,
                month: 1,
                day: 1,
            },
        ),
    },
    Variable {
        name: "cLD",
        data_type: DataTypeReference {
            referenced_type: "LDATE",
        },
        initializer: Some(
            LiteralDate {
                year: 1970,
                month: 1,
                day: 1,
            },
        ),
    },
    Variable {
        name: "cLD_SHORT",
        data_type: DataTypeReference {
            referenced_type: "LDATE",
        },
        initializer: Some(
            LiteralDate {
                year: 1970,
                month: 1,
                day: 1,
            },
        ),
    },
    Variable {
        name: "cTOD",
        data_type: DataTypeReference {
            referenced_type: "TIME_OF_DAY",
        },
        initializer: Some(
            LiteralTimeOfDay {
                hour: 0,
                min: 0,
                sec: 0,
                nano: 0,
            },
        ),
    },
    Variable {
        name: "cTOD_SHORT",
        data_type: DataTypeReference {
            referenced_type: "TOD",
        },
        initializer: Some(
            LiteralTimeOfDay {
                hour: 0,
                min: 0,
                sec: 0,
                nano: 0,
            },
        ),
    },
    Variable {
        name: "cLTOD",
        data_type: DataTypeReference {
            referenced_type: "LTOD",
        },
        initializer: Some(
            LiteralTimeOfDay {
                hour: 23,
                min: 59,
                sec: 59,
                nano: 999999999,
            },
        ),
    },
    Variable {
        name: "cLTOD_SHORT",
        data_type: DataTypeReference {
            referenced_type: "LTOD",
        },
        initializer: Some(
            LiteralTimeOfDay {
                hour: 23,
                min: 59,
                sec: 59,
                nano: 999999999,
            },
        ),
    },
    Variable {
        name: "cDT",
        data_type: DataTypeReference {
            referenced_type: "DATE_AND_TIME",
        },
        initializer: Some(
            LiteralDateAndTime {
                year: 1970,
                month: 1,
                day: 1,
                hour: 23,
                min: 59,
                sec: 59,
                nano: 0,
            },
        ),
    },
    Variable {
        name: "cDT_SHORT",
        data_type: DataTypeReference {
            referenced_type: "DT",
        },
        initializer: Some(
            LiteralDateAndTime {
                year: 1970,
                month: 1,
                day: 1,
                hour: 23,
                min: 59,
                sec: 59,
                nano: 0,
            },
        ),
    },
    Variable {
        name: "cLDT",
        data_type: DataTypeReference {
            referenced_type: "LDT",
        },
        initializer: Some(
            LiteralDateAndTime {
                year: 1970,
                month: 1,
                day: 1,
                hour: 23,
                min: 59,
                sec: 59,
                nano: 123000000,
            },
        ),
    },
    Variable {
        name: "cLDT_SHORT",
        data_type: DataTypeReference {
            referenced_type: "LDT",
        },
        initializer: Some(
            LiteralDateAndTime {
                year: 1970,
                month: 1,
                day: 1,
                hour: 23,
                min: 59,
                sec: 59,
                nano: 123000000,
            },
        ),
    },
]"#;
    assert_eq!(ast_string, expected_ast)
}
