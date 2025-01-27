use crate::test_utils::tests::{parse, parse_and_validate_buffered, parse_buffered};
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
    assert_eq!(prg.kind, PouType::Function);
    assert_eq!(prg.name, "foo");
    assert_debug_snapshot!(prg.return_type.as_ref().unwrap())
}

#[test]
fn simple_foo_function_block_can_be_parsed() {
    let src = "FUNCTION_BLOCK foo END_FUNCTION_BLOCK";
    let result = parse(src).0;

    let prg = &result.units[0];
    assert_eq!(prg.kind, PouType::FunctionBlock);
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
        kind: PouType::Function,
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
        interfaces: vec![],
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
        kind: PouType::Function,
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
        interfaces: vec![],
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
    let (_, diagnostics) = parse_buffered(src);

    insta::assert_snapshot!(diagnostics, @r"
    error[E105]: Pragma {constant} is not allowed in POU declarations
      ┌─ <internal>:2:9
      │  
    2 │ ╭         {constant}
    3 │ │         FUNCTION_BLOCK foo END_FUNCTION_BLOCK
      │ ╰──────────────────────^ Pragma {constant} is not allowed in POU declarations

    error[E105]: Pragma {constant} is not allowed in POU declarations
      ┌─ <internal>:4:9
      │  
    4 │ ╭         {constant}
    5 │ │         PROGRAM bar END_PROGRAM
      │ ╰───────────────^ Pragma {constant} is not allowed in POU declarations

    error[E105]: Pragma {constant} is not allowed in POU declarations
      ┌─ <internal>:6:9
      │  
    6 │ ╭         {constant}
    7 │ │         CLASS qux
      │ ╰─────────────^ Pragma {constant} is not allowed in POU declarations

    error[E105]: Pragma {constant} is not allowed in POU declarations
      ┌─ <internal>:8:13
      │  
    8 │ ╭             {constant}
    9 │ │             METHOD quux : DINT END_METHOD
      │ ╰──────────────────^ Pragma {constant} is not allowed in POU declarations

    error[E105]: Pragma {constant} is not allowed in POU declarations
       ┌─ <internal>:11:9
       │  
    11 │ ╭         {constant}
    12 │ │         FUNCTION corge  : BOOL END_FUNCTION
       │ ╰────────────────^ Pragma {constant} is not allowed in POU declarations
    ");
}

#[test]
#[ignore = "This is not a property related issue but more of a general issue with the parser"]
fn some_defined_tokens_are_not_reserved_keywords() {
    // TODO: Currently the parse_variable_line method checks if the line contains an Token::Identifier and if so parses
    //       the line. However, an identifier called (for example) "retain" will not be parseable as a variable because "retain"
    //       resolves to a `Token::Retain` by the lexer. Same goes for property related keywords.
    //       One way to solve this would be to get the raw value of the token and check if it is a reserved keyword, e.g.
    //       `if reserved_keywords.get(token.as_str_representation()) { /* parse... */ } else { /* return error */ }` in the parser
    let source = r"
        FUNCTION foo
            VAR
                retain : DINT;
                public : DINT;
                property : DINT;
                get : DINT;
                set : DINT;

                end_property : DINT;
                end_get : DINT;
                end_set : DINT;
            END_VAR
        END_FUNCTION
    ";

    let (_, diagnostics) = parse_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"");
}

#[test]
fn property_inside_function_block() {
    let source = r"
        FUNCTION_BLOCK foo
            PROPERTY bar : DINT
                GET
                    VAR
                        propertyLocalVariable : DINT;
                    END_VAR

                    propertyLocalVariable := 42;
                END_GET

                SET
                    VAR
                        propertyLocalVariable : DINT;
                    END_VAR

                    propertyLocalVariable := 42;
                    functionblockLocalVariable := propertyLocalVariable;
                END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
    ";

    let (result, diagnostics) = parse_buffered(source);

    insta::assert_snapshot!(diagnostics, @r"");
    // We expect one POU
    assert_eq!(result.units.len(), 1);
    assert_eq!(result.units[0].name.as_str(), "foo");

    // ...and one implementation
    assert_eq!(result.implementations.len(), 1);
    assert_eq!(result.implementations[0].name.as_str(), "foo");

    // Other than that we expect a property named "bar" with a getter and setter, which later will be desugared into a method
    insta::assert_debug_snapshot!(result.properties, @r#"
    [
        Property {
            name: "bar",
            name_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 2,
                        column: 21,
                        offset: 49,
                    }..TextLocation {
                        line: 2,
                        column: 24,
                        offset: 52,
                    },
                ),
            },
            kind_parent: FunctionBlock,
            name_parent: "foo",
            name_parent_location: SourceLocation {
                span: Range(
                    TextLocation {
                        line: 1,
                        column: 23,
                        offset: 24,
                    }..TextLocation {
                        line: 1,
                        column: 26,
                        offset: 27,
                    },
                ),
            },
            datatype: DataTypeReference {
                referenced_type: "DINT",
            },
            implementations: [
                PropertyImplementation {
                    kind: Get,
                    variables: [
                        VariableBlock {
                            variables: [
                                Variable {
                                    name: "propertyLocalVariable",
                                    data_type: DataTypeReference {
                                        referenced_type: "DINT",
                                    },
                                },
                            ],
                            variable_block_type: Local,
                        },
                    ],
                    statements: [
                        Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "propertyLocalVariable",
                                    },
                                ),
                                base: None,
                            },
                            right: LiteralInteger {
                                value: 42,
                            },
                        },
                    ],
                    location: SourceLocation {
                        span: Range(
                            TextLocation {
                                line: 3,
                                column: 16,
                                offset: 76,
                            }..TextLocation {
                                line: 3,
                                column: 19,
                                offset: 79,
                            },
                        ),
                    },
                },
                PropertyImplementation {
                    kind: Set,
                    variables: [
                        VariableBlock {
                            variables: [
                                Variable {
                                    name: "propertyLocalVariable",
                                    data_type: DataTypeReference {
                                        referenced_type: "DINT",
                                    },
                                },
                            ],
                            variable_block_type: Local,
                        },
                    ],
                    statements: [
                        Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "propertyLocalVariable",
                                    },
                                ),
                                base: None,
                            },
                            right: LiteralInteger {
                                value: 42,
                            },
                        },
                        Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "functionblockLocalVariable",
                                    },
                                ),
                                base: None,
                            },
                            right: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "propertyLocalVariable",
                                    },
                                ),
                                base: None,
                            },
                        },
                    ],
                    location: SourceLocation {
                        span: Range(
                            TextLocation {
                                line: 11,
                                column: 16,
                                offset: 277,
                            }..TextLocation {
                                line: 11,
                                column: 19,
                                offset: 280,
                            },
                        ),
                    },
                },
            ],
        },
    ]
    "#);
}

#[test]
#[ignore = "TODO: property parsing has no error handling currently"]
fn property_with_missing_name_and_return_type() {
    let source = r"
        FUNCTION_BLOCK foo
            PROPERTY
                GET END_GET
                SET END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
    ";

    let (_, diagnostics) = parse_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"");
}
