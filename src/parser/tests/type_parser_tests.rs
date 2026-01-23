use crate::test_utils::tests::{parse, parse_buffered};
use insta::{assert_debug_snapshot, assert_snapshot};
use plc_ast::ast::{DataType, DataTypeDeclaration, UserTypeDeclaration, Variable};
use plc_source::source_location::SourceLocation;
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
                        data_type_declaration: DataTypeDeclaration::Reference {
                            referenced_type: "INT".to_string(),
                            location: SourceLocation::internal(),
                        },
                        initializer: None,
                        address: None,
                        location: SourceLocation::internal(),
                    },
                    Variable {
                        name: "Two".to_string(),
                        data_type_declaration: DataTypeDeclaration::Reference {
                            referenced_type: "INT".to_string(),
                            location: SourceLocation::internal(),
                        },
                        initializer: None,
                        address: None,
                        location: SourceLocation::internal(),
                    },
                    Variable {
                        name: "Three".to_string(),
                        data_type_declaration: DataTypeDeclaration::Reference {
                            referenced_type: "INT".to_string(),
                            location: SourceLocation::internal(),
                        },
                        initializer: None,
                        address: None,
                        location: SourceLocation::internal(),
                    },
                ),
            },
            initializer: None,
            location: SourceLocation::internal(),
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
    insta::assert_debug_snapshot!(result.pous[0]);
}

#[test]
fn enum_with_equality_operator_instead_of_assignment_should_error() {
    let (_, diagnostics) = parse_buffered(
        r#"
        TYPE State : (Idle := 0, Running = 1);
        END_TYPE
        "#,
    );
    assert!(!diagnostics.is_empty(), "Expected parse error for enum using = instead of :=");
    assert_snapshot!(diagnostics);
}

#[test]
fn enum_with_call_statement_as_variant_should_error() {
    let (_, diagnostics) = parse_buffered(
        r#"
        TYPE State : (Idle := 0, foo());
        END_TYPE
        "#,
    );
    assert!(!diagnostics.is_empty(), "Expected parse error for invalid enum variant");
    assert_snapshot!(diagnostics);
}

#[test]
fn enum_with_qualified_member_variant_should_error() {
    let (_, diagnostics) = parse_buffered(
        r#"
        TYPE State : (Idle := 0, Running.Fast, Running.Slow);
        END_TYPE
        "#,
    );
    assert!(!diagnostics.is_empty(), "Expected parse error for invalid enum variant");
    assert_snapshot!(diagnostics);
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
            location: SourceLocation::internal(),
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
    assert_snapshot!(ast_string);
}

#[test]
fn string_type_can_be_parsed_test() {
    let (result, ..) = parse(
        r#"
            TYPE MyString : STRING[253]; END_TYPE
            TYPE MyString : STRING[253] := 'abc'; END_TYPE
            "#,
    );

    assert_debug_snapshot!(result.user_types);
}

#[test]
fn wide_string_type_can_be_parsed_test() {
    let (result, ..) = parse(
        r#"
            TYPE MyString : WSTRING[253]; END_TYPE
            "#,
    );

    assert_debug_snapshot!(result.user_types[0]);
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

    assert_debug_snapshot!(x);
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

    assert_debug_snapshot!(result.user_types[0]);
}

#[test]
fn pointer_type_test() {
    let (result, _) = parse(
        r#"
        TYPE SamplePointer :
            POINTER TO INT;
        END_TYPE
        "#,
    );
    assert_debug_snapshot!(result.user_types[0]);
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
    assert_debug_snapshot!(result.user_types[0]);
    assert_eq!(diagnostics.len(), 0)
}

#[test]
fn global_pointer_declaration() {
    let (result, diagnostics) = parse_buffered(
        r#"
        VAR_GLOBAL
            SampleReference : REF_TO INT;
            SamplePointer : POINTER TO INT;
        END_VAR
        "#,
    );
    let reference_type = &result.global_vars[0].variables[0];
    assert_debug_snapshot!(reference_type);
    let pointer_type = &result.global_vars[0].variables[1];
    assert_debug_snapshot!(pointer_type);
    assert_snapshot!(diagnostics)
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
    assert_debug_snapshot!(x);
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
    assert_debug_snapshot!(var);

    let var = &parse_result.global_vars[0].variables[1];
    assert_debug_snapshot!(var);
}

#[test]
fn optional_semicolon_at_end_of_endstruct_keyword_is_consumed() {
    let (_, diagnostics) = parse(
        r#"
        TYPE Position : STRUCT
            x : DINT;
            y : DINT;
        END_STRUCT; END_TYPE
        "#,
    );

    assert!(diagnostics.is_empty())
}

#[test]
fn enum_61131_standard_style_type_before_list() {
    // TYPE COLOR : DWORD (...) := default;
    let (result, diagnostics) = parse(
        r#"
        TYPE COLOR : DWORD (
            white := 16#FFFFFF00,
            yellow := 16#FFFF0000,
            green := 16#FF00FF00,
            blue := 16#FF0000FF,
            black := 16#88000000
        ) := black;
        END_TYPE
        "#,
    );
    assert_debug_snapshot!(result.user_types[0], @r#"
    UserTypeDeclaration {
        data_type: EnumType {
            name: Some(
                "COLOR",
            ),
            numeric_type: "DWORD",
            elements: ExpressionList {
                expressions: [
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "white",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 4294967040,
                        },
                    },
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "yellow",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 4294901760,
                        },
                    },
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "green",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 4278255360,
                        },
                    },
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "blue",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 4278190335,
                        },
                    },
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "black",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 2281701376,
                        },
                    },
                ],
            },
        },
        initializer: Some(
            ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "black",
                    },
                ),
                base: None,
            },
        ),
        scope: None,
    }
    "#);
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn enum_mixed_style_type_before_and_after_list() {
    // TYPE COLOR : INT (...) DWORD - invalid mixed syntax
    let (_result, diagnostics) = parse_buffered(
        r#"
        TYPE MyEnum : INT (a := 1, b := 2) DWORD;
        END_TYPE
        "#,
    );
    // This should produce a diagnostic since types are specified twice
    assert!(!diagnostics.is_empty(), "Expected diagnostic for mixed enum type syntax");
    assert_snapshot!(diagnostics, @r"
    error[E007]: Unexpected token: expected KeywordSemicolon but found 'DWORD'
      ┌─ <internal>:2:44
      │
    2 │         TYPE MyEnum : INT (a := 1, b := 2) DWORD;
      │                                            ^^^^^ Unexpected token: expected KeywordSemicolon but found 'DWORD'
    ");
}

#[test]
fn enum_codesys_style_type_after_list() {
    // TYPE COLOR : (...) DWORD := default;
    let (result, diagnostics) = parse(
        r#"
        TYPE COLOR : (
            white := 16#FFFFFF00,
            yellow := 16#FFFF0000,
            green := 16#FF00FF00,
            blue := 16#FF0000FF,
            black := 16#88000000
        ) DWORD := black;
        END_TYPE
        "#,
    );
    assert_debug_snapshot!(result.user_types[0], @r#"
    UserTypeDeclaration {
        data_type: EnumType {
            name: Some(
                "COLOR",
            ),
            numeric_type: "DWORD",
            elements: ExpressionList {
                expressions: [
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "white",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 4294967040,
                        },
                    },
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "yellow",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 4294901760,
                        },
                    },
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "green",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 4278255360,
                        },
                    },
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "blue",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 4278190335,
                        },
                    },
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "black",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 2281701376,
                        },
                    },
                ],
            },
        },
        initializer: Some(
            ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "black",
                    },
                ),
                base: None,
            },
        ),
        scope: None,
    }
    "#);
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn enum_with_default_value_no_type_specified() {
    let (result, diagnostics) = parse(
        r#"
        TYPE STATE : (idle := 0, running := 1, stopped := 2) := running;
        END_TYPE
        "#,
    );
    assert_debug_snapshot!(result.user_types[0], @r#"
    UserTypeDeclaration {
        data_type: EnumType {
            name: Some(
                "STATE",
            ),
            numeric_type: "DINT",
            elements: ExpressionList {
                expressions: [
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "idle",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 0,
                        },
                    },
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "running",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 1,
                        },
                    },
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "stopped",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 2,
                        },
                    },
                ],
            },
        },
        initializer: Some(
            ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "running",
                    },
                ),
                base: None,
            },
        ),
        scope: None,
    }
    "#);
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn enum_61131_style_with_byte_type_and_default() {
    let (result, diagnostics) = parse(
        r#"
        TYPE STATE : BYTE (idle := 0, running := 1, stopped := 2) := running;
        END_TYPE
        "#,
    );
    assert_debug_snapshot!(result.user_types[0], @r#"
    UserTypeDeclaration {
        data_type: EnumType {
            name: Some(
                "STATE",
            ),
            numeric_type: "BYTE",
            elements: ExpressionList {
                expressions: [
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "idle",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 0,
                        },
                    },
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "running",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 1,
                        },
                    },
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "stopped",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 2,
                        },
                    },
                ],
            },
        },
        initializer: Some(
            ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "running",
                    },
                ),
                base: None,
            },
        ),
        scope: None,
    }
    "#);
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn enum_codesys_style_with_int_type_and_default() {
    let (result, diagnostics) = parse(
        r#"
        TYPE PRIORITY : (low := 10, medium := 20, high := 30) INT := medium;
        END_TYPE
        "#,
    );
    assert_debug_snapshot!(result.user_types[0], @r#"
    UserTypeDeclaration {
        data_type: EnumType {
            name: Some(
                "PRIORITY",
            ),
            numeric_type: "INT",
            elements: ExpressionList {
                expressions: [
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "low",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 10,
                        },
                    },
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "medium",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 20,
                        },
                    },
                    Assignment {
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "high",
                                },
                            ),
                            base: None,
                        },
                        right: LiteralInteger {
                            value: 30,
                        },
                    },
                ],
            },
        },
        initializer: Some(
            ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "medium",
                    },
                ),
                base: None,
            },
        ),
        scope: None,
    }
    "#);
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn enum_with_no_elements_produces_syntax_error() {
    // Empty enums are syntactically invalid - parser requires at least one element
    let (result, diagnostics) = parse_buffered(
        r#"
        TYPE EMPTY_ENUM : INT ();
        END_TYPE

        TYPE ANOTHER_EMPTY_ENUM : () INT;
        "#,
    );
    assert!(!diagnostics.is_empty());
    assert_snapshot!(diagnostics, @r"
    error[E007]: Unexpected token: expected Literal but found )
      ┌─ <internal>:2:32
      │
    2 │         TYPE EMPTY_ENUM : INT ();
      │                                ^ Unexpected token: expected Literal but found )

    error[E007]: Unexpected token: expected KeywordEndType but found ''
      ┌─ <internal>:6:9
      │
    6 │         
      │         ^ Unexpected token: expected KeywordEndType but found ''
    ");
    // User type should still be created despite the error (error recovery)
    assert_debug_snapshot!(result.user_types[0], @r#"
    UserTypeDeclaration {
        data_type: SubRangeType {
            name: Some(
                "EMPTY_ENUM",
            ),
            referenced_type: "INT",
            bounds: Some(
                EmptyStatement,
            ),
        },
        initializer: None,
        scope: None,
    }
    "#);
    assert_debug_snapshot!(result.user_types[1], @r#"
    UserTypeDeclaration {
        data_type: EnumType {
            name: Some(
                "ANOTHER_EMPTY_ENUM",
            ),
            numeric_type: "INT",
            elements: EmptyStatement,
        },
        initializer: None,
        scope: None,
    }
    "#);
}
