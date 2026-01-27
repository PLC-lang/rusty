use crate::test_utils::tests::{parse, parse_and_validate_buffered, parse_buffered};
use insta::{assert_debug_snapshot, assert_snapshot};
use plc_ast::ast::PouType;
use plc_diagnostics::diagnostics::Diagnostic;
use pretty_assertions::*;

#[test]
fn simple_foo_function_can_be_parsed() {
    let src = "FUNCTION foo : INT END_FUNCTION";
    let result = parse(src).0;

    let prg = &result.pous[0];
    assert_eq!(prg.kind, PouType::Function);
    assert_eq!(prg.name, "foo");
    assert_debug_snapshot!(prg.return_type.as_ref().unwrap())
}

#[test]
fn simple_foo_function_block_can_be_parsed() {
    let src = "FUNCTION_BLOCK foo END_FUNCTION_BLOCK";
    let result = parse(src).0;

    let prg = &result.pous[0];
    assert_eq!(prg.kind, PouType::FunctionBlock);
    assert_eq!(prg.name, "foo");
    assert!(prg.return_type.is_none());
}

#[test]
fn a_function_with_varargs_can_be_parsed() {
    let src = "FUNCTION foo : INT VAR_INPUT x : INT; y : ...; END_VAR END_FUNCTION";
    let result = parse(src).0;

    let prg = &result.pous[0];
    let variable_block = &prg.variable_blocks[0];
    let ast_string = format!("{variable_block:#?}");
    insta::assert_snapshot!(ast_string,  @r#"
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
    "#);
}

#[test]
fn a_function_with_typed_varargs_can_be_parsed() {
    let src = "FUNCTION foo : INT VAR_INPUT x : INT; y : INT...; END_VAR END_FUNCTION";
    let result = parse(src).0;

    let prg = &result.pous[0];
    let variable_block = &prg.variable_blocks[0];
    let ast_string = format!("{variable_block:#?}");
    insta::assert_snapshot!(ast_string,@r#"
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
    "#);
}

#[test]
fn a_function_with_sized_varargs_can_be_parsed() {
    let src = "FUNCTION foo : INT VAR_INPUT x : INT; y : {sized} ...; END_VAR END_FUNCTION";
    let result = parse(src).0;

    let prg = &result.pous[0];
    let variable_block = &prg.variable_blocks[0];
    let ast_string = format!("{variable_block:#?}");
    insta::assert_snapshot!(ast_string,  @r#"
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
    "#);
}

#[test]
fn a_function_with_sized_typed_varargs_can_be_parsed() {
    let src = "FUNCTION foo : INT VAR_INPUT x : INT; y : {sized} INT...; END_VAR END_FUNCTION";
    let result = parse(src).0;

    let prg = &result.pous[0];
    let variable_block = &prg.variable_blocks[0];
    let ast_string = format!("{variable_block:#?}");
    insta::assert_snapshot!(ast_string,@r#"
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
    "#);
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
    assert_debug_snapshot!(parse_result.pous[0], @r#"
    POU {
        name: "foo",
        variable_blocks: [
            VariableBlock {
                variables: [
                    Variable {
                        name: "args1",
                        data_type: DataTypeDefinition {
                            data_type: VarArgs {
                                referenced_type: None,
                                sized: false,
                            },
                        },
                    },
                    Variable {
                        name: "args2",
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
            },
        ],
        pou_type: Function,
        return_type: Some(
            DataTypeReference {
                referenced_type: "DINT",
            },
        ),
        interfaces: [],
        properties: [],
    }
    "#);
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
    assert_debug_snapshot!(parse_result.pous[0], @r#"
    POU {
        name: "foo",
        variable_blocks: [
            VariableBlock {
                variables: [
                    Variable {
                        name: "args1",
                        data_type: DataTypeDefinition {
                            data_type: VarArgs {
                                referenced_type: None,
                                sized: true,
                            },
                        },
                    },
                    Variable {
                        name: "args2",
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
            },
        ],
        pou_type: Function,
        return_type: Some(
            DataTypeReference {
                referenced_type: "DINT",
            },
        ),
        interfaces: [],
        properties: [],
    }
    "#);
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

    let prg = &result.pous[0];
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

    let prg = &result.pous[0];
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

    let prg = &result.pous[0];
    let variable_block = &prg.variable_blocks[0];
    let ast_string = format!("{variable_block:#?}");

    insta::assert_snapshot!(ast_string, @r#"
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
    "#)
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

// TODO(volsa): https://github.com/PLC-lang/rusty/issues/1408
#[test]
fn reserved_keywords_as_variable_names_are_recognized_as_errors() {
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
    insta::assert_snapshot!(diagnostics, @r"
    error[E007]: Unexpected token: expected KeywordEndVar but found ': DINT;
                    public : DINT;
                    property : DINT;
                    get : DINT;
                    set : DINT;

                    end_property : DINT;
                    end_get : DINT;
                    end_set : DINT;'
       ┌─ <internal>:4:24
       │  
     4 │                   retain : DINT;
       │ ╭────────────────────────^
     5 │ │                 public : DINT;
     6 │ │                 property : DINT;
     7 │ │                 get : DINT;
     8 │ │                 set : DINT;
     9 │ │ 
    10 │ │                 end_property : DINT;
    11 │ │                 end_get : DINT;
    12 │ │                 end_set : DINT;
       │ ╰───────────────────────────────^ Unexpected token: expected KeywordEndVar but found ': DINT;
                    public : DINT;
                    property : DINT;
                    get : DINT;
                    set : DINT;

                    end_property : DINT;
                    end_get : DINT;
                    end_set : DINT;'
    ");
}
#[test]
fn use_incorrect_end_keyword() {
    let source = r"
        FUNCTION_BLOCK fb
                PROPERTY foo : DINT
                    GET
                    END_SET;
                    GET
                    END_GET;
                    GET
                    END_SET;
                    END_PROPERTY
        END_FUNCTION_BLOCK
    ";

    let (_, diagnostics) = parse_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E007]: Unexpected token: expected Literal but found END_SET
      ┌─ <internal>:5:21
      │
    5 │                     END_SET;
      │                     ^^^^^^^ Unexpected token: expected Literal but found END_SET

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'END_SET'
      ┌─ <internal>:5:21
      │
    5 │                     END_SET;
      │                     ^^^^^^^ Unexpected token: expected KeywordSemicolon but found 'END_SET'

    error[E007]: Unexpected token: expected Literal but found GET
      ┌─ <internal>:6:21
      │
    6 │                     GET
      │                     ^^^ Unexpected token: expected Literal but found GET

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'GET'
      ┌─ <internal>:6:21
      │
    6 │                     GET
      │                     ^^^ Unexpected token: expected KeywordSemicolon but found 'GET'

    error[E006]: Missing expected Token [KeywordSemicolon, KeywordColon]
      ┌─ <internal>:7:21
      │
    7 │                     END_GET;
      │                     ^^^^^^^ Missing expected Token [KeywordSemicolon, KeywordColon]

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'END_GET'
      ┌─ <internal>:7:21
      │
    7 │                     END_GET;
      │                     ^^^^^^^ Unexpected token: expected KeywordSemicolon but found 'END_GET'

    error[E006]: Missing expected Token KeywordEndProperty
      ┌─ <internal>:7:28
      │
    7 │                     END_GET;
      │                            ^ Missing expected Token KeywordEndProperty

    error[E007]: Unexpected token: expected Literal but found GET
      ┌─ <internal>:8:21
      │
    8 │                     GET
      │                     ^^^ Unexpected token: expected Literal but found GET

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'GET
                        END_SET'
      ┌─ <internal>:8:21
      │  
    8 │ ╭                     GET
    9 │ │                     END_SET;
      │ ╰───────────────────────────^ Unexpected token: expected KeywordSemicolon but found 'GET
                        END_SET'

    error[E007]: Unexpected token: expected Literal but found END_PROPERTY
       ┌─ <internal>:10:21
       │
    10 │                     END_PROPERTY
       │                     ^^^^^^^^^^^^ Unexpected token: expected Literal but found END_PROPERTY

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'END_PROPERTY'
       ┌─ <internal>:10:21
       │
    10 │                     END_PROPERTY
       │                     ^^^^^^^^^^^^ Unexpected token: expected KeywordSemicolon but found 'END_PROPERTY'

    error[E006]: Missing expected Token [KeywordSemicolon, KeywordColon]
       ┌─ <internal>:11:9
       │
    11 │         END_FUNCTION_BLOCK
       │         ^^^^^^^^^^^^^^^^^^ Missing expected Token [KeywordSemicolon, KeywordColon]

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'END_FUNCTION_BLOCK'
       ┌─ <internal>:11:9
       │
    11 │         END_FUNCTION_BLOCK
       │         ^^^^^^^^^^^^^^^^^^ Unexpected token: expected KeywordSemicolon but found 'END_FUNCTION_BLOCK'
    ");
}
