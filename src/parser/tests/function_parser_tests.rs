use crate::test_utils::tests::{parse, parse_and_validate_buffered};
use insta::{assert_debug_snapshot, assert_snapshot};
use plc_ast::ast::{
    AccessModifier, ArgumentProperty, DataType, DataTypeDeclaration, LinkageType, Pou, PouType, Variable,
    VariableBlock, VariableBlockType,
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;
use pretty_assertions::*;

#[test]
fn simple_foo_function_can_be_parsed() {
    let src = "FUNCTION foo : INT END_FUNCTION";
    let result = parse(src).0;

    let prg = &result.units[0];
    assert_eq!(prg.pou_type, PouType::Function);
    assert_eq!(prg.name, "foo");
    assert_debug_snapshot!(prg.return_type.as_ref().unwrap())
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
    let ast_string = format!("{variable_block:#?}");
    insta::assert_snapshot!(ast_string,  @r###"
    VariableBlock {
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
                        sized: false,
                    },
                },
            },
        ],
        variable_block_type: Input(
            ByVal,
        ),
    }
    "###);
}

#[test]
fn a_function_with_typed_varargs_can_be_parsed() {
    let src = "FUNCTION foo : INT VAR_INPUT x : INT; y : INT...; END_VAR END_FUNCTION";
    let result = parse(src).0;

    let prg = &result.units[0];
    let variable_block = &prg.variable_blocks[0];
    let ast_string = format!("{variable_block:#?}");
    insta::assert_snapshot!(ast_string,@r###"
    VariableBlock {
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
                        sized: false,
                    },
                },
            },
        ],
        variable_block_type: Input(
            ByVal,
        ),
    }
    "###);
}

#[test]
fn a_function_with_sized_varargs_can_be_parsed() {
    let src = "FUNCTION foo : INT VAR_INPUT x : INT; y : {sized} ...; END_VAR END_FUNCTION";
    let result = parse(src).0;

    let prg = &result.units[0];
    let variable_block = &prg.variable_blocks[0];
    let ast_string = format!("{variable_block:#?}");
    insta::assert_snapshot!(ast_string,  @r###"
    VariableBlock {
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
                        sized: true,
                    },
                },
            },
        ],
        variable_block_type: Input(
            ByVal,
        ),
    }
    "###);
}

#[test]
fn a_function_with_sized_typed_varargs_can_be_parsed() {
    let src = "FUNCTION foo : INT VAR_INPUT x : INT; y : {sized} INT...; END_VAR END_FUNCTION";
    let result = parse(src).0;

    let prg = &result.units[0];
    let variable_block = &prg.variable_blocks[0];
    let ast_string = format!("{variable_block:#?}");
    insta::assert_snapshot!(ast_string,@r###"
    VariableBlock {
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
                        sized: true,
                    },
                },
            },
        ],
        variable_block_type: Input(
            ByVal,
        ),
    }
    "###);
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

    assert_eq!(format!("{diagnostics:#?}"), format!("{:#?}", Vec::<Diagnostic>::new()).as_str());

    let x = &parse_result.units[0];
    let expected = Pou {
        name: "foo".into(),
        pou_type: PouType::Function,
        return_type: Some(DataTypeDeclaration::DataTypeReference {
            referenced_type: "DINT".into(),
            location: SourceLocation::internal(),
        }),
        variable_blocks: vec![VariableBlock {
            constant: false,
            access: AccessModifier::Protected,
            retain: false,
            variable_block_type: VariableBlockType::Input(ArgumentProperty::ByVal),
            location: SourceLocation::internal(),
            linkage: LinkageType::Internal,
            variables: vec![
                Variable {
                    name: "args1".into(),
                    data_type_declaration: DataTypeDeclaration::DataTypeDefinition {
                        data_type: DataType::VarArgs { referenced_type: None, sized: false },
                        location: SourceLocation::internal(),
                        scope: Some("foo".into()),
                    },
                    initializer: None,
                    address: None,
                    location: SourceLocation::internal(),
                },
                Variable {
                    name: "args2".into(),
                    data_type_declaration: DataTypeDeclaration::DataTypeDefinition {
                        data_type: DataType::VarArgs {
                            referenced_type: Some(Box::new(DataTypeDeclaration::DataTypeReference {
                                referenced_type: "INT".into(),
                                location: SourceLocation::internal(),
                            })),
                            sized: false,
                        },
                        location: SourceLocation::internal(),
                        scope: Some("foo".into()),
                    },
                    initializer: None,
                    address: None,
                    location: SourceLocation::internal(),
                },
            ],
        }],
        location: SourceLocation::internal(),
        name_location: SourceLocation::internal(),
        poly_mode: None,
        generics: vec![],
        linkage: LinkageType::Internal,
        super_class: None,
        is_const: false,
    };
    assert_eq!(format!("{expected:#?}"), format!("{x:#?}").as_str());
}

#[test]
fn sized_varargs_parameters_can_be_parsed() {
    let src = "
            FUNCTION foo : DINT
            VAR_INPUT
            args1 : {sized} ...;
            args2 : {sized} INT...;
            END_VAR
            END_FUNCTION
           ";
    let (parse_result, diagnostics) = parse(src);

    assert_eq!(format!("{diagnostics:#?}"), format!("{:#?}", Vec::<Diagnostic>::new()).as_str());

    let x = &parse_result.units[0];
    let expected = Pou {
        name: "foo".into(),
        pou_type: PouType::Function,
        return_type: Some(DataTypeDeclaration::DataTypeReference {
            referenced_type: "DINT".into(),
            location: SourceLocation::internal(),
        }),
        variable_blocks: vec![VariableBlock {
            constant: false,
            access: AccessModifier::Protected,
            retain: false,
            variable_block_type: VariableBlockType::Input(ArgumentProperty::ByVal),
            location: SourceLocation::internal(),
            linkage: LinkageType::Internal,
            variables: vec![
                Variable {
                    name: "args1".into(),
                    data_type_declaration: DataTypeDeclaration::DataTypeDefinition {
                        data_type: DataType::VarArgs { referenced_type: None, sized: true },
                        location: SourceLocation::internal(),
                        scope: Some("foo".into()),
                    },
                    initializer: None,
                    address: None,
                    location: SourceLocation::internal(),
                },
                Variable {
                    name: "args2".into(),
                    data_type_declaration: DataTypeDeclaration::DataTypeDefinition {
                        data_type: DataType::VarArgs {
                            referenced_type: Some(Box::new(DataTypeDeclaration::DataTypeReference {
                                referenced_type: "INT".into(),
                                location: SourceLocation::internal(),
                            })),
                            sized: true,
                        },
                        location: SourceLocation::internal(),
                        scope: Some("foo".into()),
                    },
                    initializer: None,
                    address: None,
                    location: SourceLocation::internal(),
                },
            ],
        }],
        location: SourceLocation::internal(),
        name_location: SourceLocation::internal(),
        poly_mode: None,
        generics: vec![],
        linkage: LinkageType::Internal,
        super_class: None,
        is_const: false,
    };
    assert_eq!(format!("{expected:#?}"), format!("{x:#?}").as_str());
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
    let diagnostics = parse_and_validate_buffered(function);
    // THEN there should be one diagnostic -> unsupported return type
    assert_snapshot!(diagnostics);
}

#[test]
fn function_inline_struct_return_unsupported() {
    // GIVEN FUNCTION returning an inline STRUCT
    let function = "FUNCTION foo : STRUCT x : INT; y : INT; END_STRUCT VAR_INPUT END_VAR END_FUNCTION";
    // WHEN parsing is done
    let diagnostics = parse_and_validate_buffered(function);
    // THEN there should be one diagnostic -> unsupported return type
    assert_snapshot!(diagnostics);
}

#[test]
fn simple_fb_with_var_temp_can_be_parsed() {
    let function = "FUNCTION_BLOCK buz VAR_TEMP x : INT; END_VAR END_FUNCTION_BLOCK";
    let result = parse(function).0;

    let prg = &result.units[0];
    let variable_block = &prg.variable_blocks[0];
    let ast_string = format!("{variable_block:#?}");
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
    let ast_string = format!("{variable_block:#?}");
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
fn var_input_by_ref_parsed() {
    let function = "FUNCTION buz VAR_INPUT {ref} x : INT; END_VAR END_FUNCTION";
    let result = parse(function).0;

    let prg = &result.units[0];
    let variable_block = &prg.variable_blocks[0];
    let ast_string = format!("{variable_block:#?}");

    insta::assert_snapshot!(ast_string, @r###"
    VariableBlock {
        variables: [
            Variable {
                name: "x",
                data_type: DataTypeReference {
                    referenced_type: "INT",
                },
            },
        ],
        variable_block_type: Input(
            ByRef,
        ),
    }
    "###)
}

#[test]
fn constant_pragma_can_be_parsed_but_errs() {
    let src = r#"
        {constant}
        FUNCTION_BLOCK foo END_FUNCTION_BLOCK
        {constant}
        PROGRAM bar END_PROGRAM 
        {constant}
        CLASS qux 
            {constant}
            METHOD quux : DINT END_METHOD 
        END_CLASS 
        {constant}
        FUNCTION corge  : BOOL END_FUNCTION
        // {constant} pragma in comment does not cause validation
        FUNCTION corge  : BOOL END_FUNCTION
    "#;
    let (_, diagnostics) = parse(src);

    insta::assert_debug_snapshot!(diagnostics, @r###"
    [
        Diagnostic {
            message: "Pragma {constant} is not allowed in POU declaration",
            primary_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 1,
                        column: 8,
                        offset: 9,
                    }..TextLocation {
                        line: 2,
                        column: 22,
                        offset: 42,
                    },
                ),
            },
            secondary_locations: None,
            error_code: "E105",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Pragma {constant} is not allowed in POU declaration",
            primary_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 3,
                        column: 8,
                        offset: 74,
                    }..TextLocation {
                        line: 4,
                        column: 15,
                        offset: 100,
                    },
                ),
            },
            secondary_locations: None,
            error_code: "E105",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Pragma {constant} is not allowed in POU declaration",
            primary_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 5,
                        column: 8,
                        offset: 126,
                    }..TextLocation {
                        line: 6,
                        column: 13,
                        offset: 150,
                    },
                ),
            },
            secondary_locations: None,
            error_code: "E105",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Pragma {constant} is not allowed in POU declaration",
            primary_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 7,
                        column: 12,
                        offset: 168,
                    }..TextLocation {
                        line: 8,
                        column: 18,
                        offset: 197,
                    },
                ),
            },
            secondary_locations: None,
            error_code: "E105",
            sub_diagnostics: [],
            internal_error: None,
        },
        Diagnostic {
            message: "Pragma {constant} is not allowed in POU declaration",
            primary_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 10,
                        column: 8,
                        offset: 249,
                    }..TextLocation {
                        line: 11,
                        column: 16,
                        offset: 276,
                    },
                ),
            },
            secondary_locations: None,
            error_code: "E105",
            sub_diagnostics: [],
            internal_error: None,
        },
    ]
    "###);
}
