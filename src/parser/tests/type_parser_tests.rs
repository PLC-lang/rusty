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
