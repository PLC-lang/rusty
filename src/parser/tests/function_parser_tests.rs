use crate::{
    ast::*, parser::tests::ref_to, test_utils::tests::parse, typesystem::DINT_TYPE, Diagnostic,
};
use pretty_assertions::*;

#[test]
fn simple_foo_function_can_be_parsed() {
    let src = "FUNCTION foo : INT END_FUNCTION";
    let result = parse(src).0;

    let prg = &result.units[0];
    assert_eq!(prg.pou_type, PouType::Function);
    assert_eq!(prg.name, "foo");
    assert_eq!(
        prg.return_type.as_ref().unwrap(),
        &DataTypeDeclaration::DataTypeReference {
            referenced_type: "INT".to_string(),
            location: (15..18).into(),
        }
    );
}

#[test]
fn simple_foo_function_block_can_be_parsed() {
    let src = "FUNCTION_BLOCK foo END_FUNCTION_BLOCK";
    let result = parse(src).0;

    let prg = &result.units[0];
    assert_eq!(prg.pou_type, PouType::FunctionBlock);
    assert_eq!(prg.name, "foo");
    assert!(prg.return_type.is_none());
}

#[test]
fn a_function_with_varargs_can_be_parsed() {
    let src = "FUNCTION foo : INT VAR_INPUT x : INT; y : ...; END_VAR END_FUNCTION";
    let result = parse(src).0;

    let prg = &result.units[0];
    let variable_block = &prg.variable_blocks[0];
    let ast_string = format!("{:#?}", variable_block);
    let expected_ast = r#"VariableBlock {
    variables: [
        Variable {
            name: "x",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
        Variable {
            name: "y",
            data_type: DataTypeDefinition {
                data_type: VarArgs {
                    referenced_type: None,
                },
            },
        },
    ],
    variable_block_type: Input,
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn a_function_with_typed_varargs_can_be_parsed() {
    let src = "FUNCTION foo : INT VAR_INPUT x : INT; y : INT...; END_VAR END_FUNCTION";
    let result = parse(src).0;

    let prg = &result.units[0];
    let variable_block = &prg.variable_blocks[0];
    let ast_string = format!("{:#?}", variable_block);
    let expected_ast = r#"VariableBlock {
    variables: [
        Variable {
            name: "x",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
        Variable {
            name: "y",
            data_type: DataTypeDefinition {
                data_type: VarArgs {
                    referenced_type: Some(
                        DataTypeReference {
                            referenced_type: "INT",
                        },
                    ),
                },
            },
        },
    ],
    variable_block_type: Input,
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn varargs_parameters_can_be_parsed() {
    let src = "
            FUNCTION foo : DINT
            VAR_INPUT
            args1 : ...;
            args2 : INT...;
            END_VAR
            END_FUNCTION
           ";
    let (parse_result, diagnostics) = parse(src);

    assert_eq!(
        format!("{:#?}", diagnostics),
        format!("{:#?}", Vec::<Diagnostic>::new()).as_str()
    );

    let x = &parse_result.units[0];
    let expected = Pou {
        name: "foo".into(),
        pou_type: PouType::Function,
        return_type: Some(DataTypeDeclaration::DataTypeReference {
            referenced_type: "DINT".into(),
            location: SourceRange::undefined(),
        }),
        variable_blocks: vec![VariableBlock {
            constant: false,
            access: AccessModifier::Protected,
            retain: false,
            variable_block_type: VariableBlockType::Input,
            location: SourceRange::undefined(),
            variables: vec![
                Variable {
                    name: "args1".into(),
                    data_type: DataTypeDeclaration::DataTypeDefinition {
                        data_type: DataType::VarArgs {
                            referenced_type: None,
                        },
                        location: SourceRange::undefined(),
                        scope: Some("foo".into()),
                    },
                    initializer: None,
                    location: SourceRange::undefined(),
                },
                Variable {
                    name: "args2".into(),
                    data_type: DataTypeDeclaration::DataTypeDefinition {
                        data_type: DataType::VarArgs {
                            referenced_type: Some(Box::new(
                                DataTypeDeclaration::DataTypeReference {
                                    referenced_type: "INT".into(),
                                    location: SourceRange::undefined(),
                                },
                            )),
                        },
                        location: SourceRange::undefined(),
                        scope: Some("foo".into()),
                    },
                    initializer: None,
                    location: SourceRange::undefined(),
                },
            ],
        }],
        location: SourceRange::undefined(),
        poly_mode: None,
        generics: vec![],
    };
    assert_eq!(format!("{:#?}", expected), format!("{:#?}", x).as_str());
}

// Tests for function return types
// supported return types
#[test]
fn function_array_return_supported() {
    //GIVEN FUNCTION returning an ARRAY
    let function = "FUNCTION foo : ARRAY[0..3] OF INT VAR_INPUT END_VAR END_FUNCTION";
    //WHEN parsing is done
    let (_parse_result, diagnostics) = parse(function);
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn function_subrange_return_supported() {
    //GIVEN FUNCTION returning a SubRange
    let function = "FUNCTION foo : INT(0..10) VAR_INPUT END_VAR END_FUNCTION";
    //WHEN parsing is done
    let (_parse_result, diagnostics) = parse(function);
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn function_pointer_return_supported() {
    //GIVEN FUNCTION returning a POINTER
    let function = "FUNCTION foo : REF_TO INT VAR_INPUT END_VAR END_FUNCTION";
    //WHEN parsing is done
    let (_parse_result, diagnostics) = parse(function);
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

// STRING types
#[test]
fn function_string_return_supported() {
    //GIVEN FUNCTION returning a STRING
    let function = "FUNCTION foo : STRING VAR_INPUT END_VAR END_FUNCTION";
    //WHEN parsing is done
    let (_parse_result, diagnostics) = parse(function);
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn function_string_len_return_supported() {
    //GIVEN FUNCTION returning a STRING[10]
    let function = "FUNCTION foo : STRING[10] VAR_INPUT END_VAR END_FUNCTION";
    //WHEN parsing is done
    let (_parse_result, diagnostics) = parse(function);
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn function_wstring_return_supported() {
    //GIVEN FUNCTION returning a WSTRING
    let function = "FUNCTION foo : WSTRING VAR_INPUT END_VAR END_FUNCTION";
    //WHEN parsing is done
    let (_parse_result, diagnostics) = parse(function);
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn function_wstring_len_return_supported() {
    //GIVEN FUNCTION returning a WSTRING[10]
    let function = "FUNCTION foo : WSTRING[10] VAR_INPUT END_VAR END_FUNCTION";
    //WHEN parsing is done
    let (_parse_result, diagnostics) = parse(function);
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

// SCALAR types
#[test]
fn function_int_return_supported() {
    //GIVEN FUNCTION returning an INT
    let function = "FUNCTION foo : INT VAR_INPUT END_VAR END_FUNCTION";
    //WHEN parsing is done
    let (_parse_result, diagnostics) = parse(function);
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn function_bool_return_supported() {
    //GIVEN FUNCTION returning a BOOL
    let function = "FUNCTION foo : BOOL VAR_INPUT END_VAR END_FUNCTION";
    //WHEN parsing is done
    let (_parse_result, diagnostics) = parse(function);
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn function_type_enum_return_supported() {
    // GIVEN FUNCTION returning a type ENUM
    let function = "TYPE MyEnum: (green, yellow, red); END_TYPE
	FUNCTION foo : MyEnum VAR_INPUT END_VAR END_FUNCTION";
    //WHEN parsing is done
    let (_parse_result, diagnostics) = parse(function);
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn function_type_struct_return_supported() {
    // GIVEN FUNCTION returning a type STRUCT
    let function = "TYPE MyStruct: STRUCT x : INT; y : INT; END_STRUCT END_TYPE
	FUNCTION foo : MyStruct VAR_INPUT END_VAR END_FUNCTION";
    //WHEN parsing is done
    let (_parse_result, diagnostics) = parse(function);
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

// unsupported return types
#[test]
fn function_inline_enum_return_unsupported() {
    // GIVEN FUNCTION returning an inline ENUM
    let function = "FUNCTION foo : (green, yellow, red) VAR_INPUT END_VAR END_FUNCTION";
    // WHEN parsing is done
    let (_parse_result, diagnostics) = parse(function);
    // THEN there should be one diagnostic -> unsupported return type
    assert_eq!(
        diagnostics,
        vec![Diagnostic::function_unsupported_return_type(
            &DataTypeDeclaration::DataTypeDefinition {
                data_type: DataType::EnumType {
                    name: None,
                    numeric_type: DINT_TYPE.to_string(),
                    elements: AstStatement::ExpressionList {
                        expressions: vec![ref_to("green"), ref_to("yellow"), ref_to("red")],
                        id: 0,
                    }
                },
                location: (15..35).into(),
                scope: Some("foo".into()),
            }
        )]
    );
}

#[test]
fn function_inline_struct_return_unsupported() {
    // GIVEN FUNCTION returning an inline STRUCT
    let function =
        "FUNCTION foo : STRUCT x : INT; y : INT; END_STRUCT VAR_INPUT END_VAR END_FUNCTION";
    // WHEN parsing is done
    let (_parse_result, diagnostics) = parse(function);
    // THEN there should be one diagnostic -> unsupported return type
    assert_eq!(
        true,
        diagnostics.contains(&Diagnostic::function_unsupported_return_type(
            &DataTypeDeclaration::DataTypeDefinition {
                data_type: DataType::StructType {
                    name: None,
                    variables: vec![
                        Variable {
                            name: "x".into(),
                            location: SourceRange::undefined(),
                            data_type: DataTypeDeclaration::DataTypeReference {
                                location: SourceRange::undefined(),
                                referenced_type: "INT".into()
                            },
                            initializer: None
                        },
                        Variable {
                            name: "y".into(),
                            location: SourceRange::undefined(),
                            data_type: DataTypeDeclaration::DataTypeReference {
                                location: SourceRange::undefined(),
                                referenced_type: "INT".into()
                            },
                            initializer: None
                        }
                    ],
                },
                location: (15..50).into(),
                scope: Some("foo".into()),
            }
        ))
    );
}

#[test]
fn simple_fb_with_var_temp_can_be_parsed() {
    let function = "FUNCTION_BLOCK buz VAR_TEMP x : INT; END_VAR END_FUNCTION_BLOCK";
    let result = parse(function).0;

    let prg = &result.units[0];
    let variable_block = &prg.variable_blocks[0];
    let ast_string = format!("{:#?}", variable_block);
    let expected_ast = r#"VariableBlock {
    variables: [
        Variable {
            name: "x",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
    ],
    variable_block_type: Temp,
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn simple_function_with_var_temp_can_be_parsed() {
    let function = "FUNCTION buz VAR_TEMP x : INT; END_VAR END_FUNCTION";
    let result = parse(function).0;

    let prg = &result.units[0];
    let variable_block = &prg.variable_blocks[0];
    let ast_string = format!("{:#?}", variable_block);
    let expected_ast = r#"VariableBlock {
    variables: [
        Variable {
            name: "x",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
    ],
    variable_block_type: Temp,
}"#;
    assert_eq!(ast_string, expected_ast);
}
