/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::lexer;
use crate::ast::*;
use pretty_assertions::*;

#[test]
fn empty_returns_empty_compilation_unit() {
    let result = super::parse(lexer::lex("")).unwrap();
    assert_eq!(result.units.len(), 0);
}

#[test]
fn empty_global_vars_can_be_parsed() {
    let lexer = lexer::lex("VAR_GLOBAL END_VAR");
    let result = super::parse(lexer).unwrap();

    let vars = &result.global_vars[0]; //globar_vars
    let ast_string = format!("{:#?}", vars);
    let expected_ast = 
r#"VariableBlock {
    variables: [],
    variable_block_type: Global,
}"#;
    assert_eq!(ast_string,expected_ast)

}

#[test]
fn global_vars_can_be_parsed() {
    let lexer = lexer::lex("VAR_GLOBAL x : INT; y : BOOL; END_VAR");
    let result = super::parse(lexer).unwrap();

    let vars = &result.global_vars[0]; //globar_vars
    let ast_string = format!("{:#?}", vars);
    let expected_ast = 
r#"VariableBlock {
    variables: [
        Variable {
            name: "x",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
        Variable {
            name: "y",
            data_type: DataTypeReference {
                referenced_type: "BOOL",
            },
        },
    ],
    variable_block_type: Global,
}"#;
    assert_eq!(ast_string,expected_ast)

}

#[test]
fn two_global_vars_can_be_parsed() {
    let lexer = lexer::lex("VAR_GLOBAL a: INT; END_VAR VAR_GLOBAL x : INT; y : BOOL; END_VAR");
    let result = super::parse(lexer).unwrap();

    let vars = &result.global_vars; //globar_vars
    let ast_string = format!("{:#?}", vars);
    let expected_ast = 
r#"[
    VariableBlock {
        variables: [
            Variable {
                name: "a",
                data_type: DataTypeReference {
                    referenced_type: "INT",
                },
            },
        ],
        variable_block_type: Global,
    },
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
                data_type: DataTypeReference {
                    referenced_type: "BOOL",
                },
            },
        ],
        variable_block_type: Global,
    },
]"#;
    assert_eq!(ast_string,expected_ast)

}

#[test]
fn simple_foo_program_can_be_parsed() {
    let lexer = lexer::lex("PROGRAM foo END_PROGRAM");
    let result = super::parse(lexer).unwrap();

    let prg = &result.units[0];
    assert_eq!(prg.pou_type, PouType::Program);
    assert_eq!(prg.name, "foo");
    assert!(prg.return_type.is_none());
}

#[test]
fn simple_foo_function_can_be_parsed() {
    let lexer = lexer::lex("FUNCTION foo : INT END_FUNCTION");
    let result = super::parse(lexer).unwrap();

    let prg = &result.units[0];
    assert_eq!(prg.pou_type, PouType::Function);
    assert_eq!(prg.name, "foo");
    assert_eq!(prg.return_type.as_ref().unwrap(), 
                &DataTypeDeclaration::DataTypeReference { 
                    referenced_type: "INT".to_string() } 
                );
}

#[test]
fn simple_foo_function_block_can_be_parsed() {
    let lexer = lexer::lex("FUNCTION_BLOCK foo END_FUNCTION_BLOCK");
    let result = super::parse(lexer).unwrap();

    let prg = &result.units[0];
    assert_eq!(prg.pou_type, PouType::FunctionBlock);
    assert_eq!(prg.name, "foo");
    assert!(prg.return_type.is_none());
}

#[test]
fn two_programs_can_be_parsed() {
    let lexer = lexer::lex("PROGRAM foo END_PROGRAM  PROGRAM bar END_PROGRAM");
    let result = super::parse(lexer).unwrap();

    let prg = &result.units[0];
    assert_eq!(prg.name, "foo");
    let prg2 = &result.units[1];
    assert_eq!(prg2.name, "bar");
}

#[test]
fn simple_program_with_varblock_can_be_parsed() {
    let lexer = lexer::lex("PROGRAM buz VAR END_VAR END_PROGRAM");
    let result = super::parse(lexer).unwrap();

    let prg = &result.units[0];

    assert_eq!(prg.variable_blocks.len(), 1);
}

#[test]
fn simple_program_with_two_varblocks_can_be_parsed() {
    let lexer = lexer::lex("PROGRAM buz VAR END_VAR VAR END_VAR END_PROGRAM");
    let result = super::parse(lexer).unwrap();

    let prg = &result.units[0];

    assert_eq!(prg.variable_blocks.len(), 2);
}

#[test]
fn a_program_needs_to_end_with_end_program() {
    let lexer = lexer::lex("PROGRAM buz ");
    let result = super::parse(lexer);
    assert_eq!(result, Err("unexpected end of body End, statements : []".to_string()));
}

#[test]
fn a_variable_declaration_block_needs_to_end_with_endvar() {
    let lexer = lexer::lex("PROGRAM buz VAR END_PROGRAM ");
    let result = super::parse(lexer);
    assert_eq!(
        result,
        Err("expected KeywordEndVar, but found KeywordEndProgram".to_string())
    );
}


#[test]
fn a_statement_without_a_semicolon_fails() {
    let lexer = lexer::lex("PROGRAM buz x END_PROGRAM ");
    let result = super::parse(lexer);
    assert_eq!(
        result,
        Err("expected End Statement, but found KeywordEndProgram".to_string())
    );
}

#[test]
fn empty_statements_are_ignored() {
    let lexer = lexer::lex("PROGRAM buz ;;;; END_PROGRAM ");
    let result = super::parse(lexer).unwrap();
    
    let prg = &result.units[0];
    assert_eq!(0, prg.statements.len());
}

#[test]
fn empty_statements_are_ignored_before_a_statement() {
    let lexer = lexer::lex("PROGRAM buz ;;;;x; END_PROGRAM ");
    let result = super::parse(lexer).unwrap();
    
    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"Reference {
    elements: [
        "x",
    ],
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn empty_statements_are_ignored_after_a_statement() {
    let lexer = lexer::lex("PROGRAM buz x;;;; END_PROGRAM ");
    let result = super::parse(lexer).unwrap();
    
    let prg = &result.units[0];
    let statement = &prg.statements[0];

    let ast_string = format!("{:#?}", statement);
    let expected_ast = r#"Reference {
    elements: [
        "x",
    ],
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn simple_program_with_variable_can_be_parsed() {
    let lexer = lexer::lex("PROGRAM buz VAR x : INT; END_VAR END_PROGRAM");
    let result = super::parse(lexer).unwrap();

    let prg = &result.units[0];
    let variable_block = &prg.variable_blocks[0];
    let ast_string = format!("{:#?}", variable_block);
    let expected_ast = 
r#"VariableBlock {
    variables: [
        Variable {
            name: "x",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
    ],
    variable_block_type: Local,
}"#;
    assert_eq!(ast_string,expected_ast);

}


#[test]
fn simple_program_with_var_input_can_be_parsed() {
    
    let lexer = lexer::lex("PROGRAM buz VAR_INPUT x : INT; END_VAR END_PROGRAM");
    let result = super::parse(lexer).unwrap();

    let prg = &result.units[0];
    let variable_block = &prg.variable_blocks[0];
    let ast_string = format!("{:#?}", variable_block);
    let expected_ast = 
r#"VariableBlock {
    variables: [
        Variable {
            name: "x",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
    ],
    variable_block_type: Input,
}"#;
    assert_eq!(ast_string,expected_ast);
}

#[test]
fn simple_struct_type_can_be_parsed() {
    let result = super::parse(lexer::lex(
        r#"
        TYPE SampleStruct :
            STRUCT
                One:INT;
                Two:INT;
                Three:INT;
            END_STRUCT
        END_TYPE 
        "#
    )).unwrap();

    let ast_string = format!("{:#?}", &result.types[0]);

    let expected_ast = 
r#"StructType {
    name: Some(
        "SampleStruct",
    ),
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
}"#;
    assert_eq!(ast_string, expected_ast);
}


#[test]
fn simple_enum_type_can_be_parsed() {
    let result = super::parse(lexer::lex(
        r#"
        TYPE SampleEnum : (red, yellow, green);
        END_TYPE 
        "#
    )).unwrap();

    let ast_string = format!("{:#?}", &result.types[0]);

    let expected_ast = 
r#"EnumType {
    name: Some(
        "SampleEnum",
    ),
    elements: [
        "red",
        "yellow",
        "green",
    ],
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn type_alias_can_be_parsed() {
    let result = super::parse(lexer::lex(
        r#"
        TYPE 
            MyInt : INT;
        END_TYPE
        "#
    )).unwrap();

    let ast_string = format!("{:#?}", &result.types[0]);

    let exptected_ast = 
r#"SubRangeType {
    name: Some(
        "MyInt",
    ),
    referenced_type: "INT",
}"#;

assert_eq!(ast_string, exptected_ast);

}

#[test]
fn array_type_can_be_parsed_test() {
    let result = super::parse(lexer::lex(
            r#"
            TYPE MyArray : ARRAY[0..8] OF INT; END_TYPE
            "#
    )).unwrap();

    let ast_string = format!("{:#?}", &result.types[0]);

    let expected_ast = 
r#"ArrayType {
    name: Some(
        "MyArray",
    ),
    bounds: RangeStatement {
        start: LiteralInteger {
            value: "0",
        },
        end: LiteralInteger {
            value: "8",
        },
    },
    referenced_type: DataTypeReference {
        referenced_type: "INT",
    },
}"#;

assert_eq!(ast_string, expected_ast);
}

#[test]
fn inline_struct_declaration_can_be_parsed() {
    let result = super::parse(lexer::lex(
        r#"
        VAR_GLOBAL
            my_struct : STRUCT
                One:INT;
                Two:INT;
                Three:INT;
            END_STRUCT
        END_VAR
        "#
    )).unwrap();

    let ast_string = format!("{:#?}", &result.global_vars[0].variables[0]);
    let expected_ast = 
r#"Variable {
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
    let result = super::parse(lexer::lex(
        r#"
        VAR_GLOBAL
            my_enum : (red, yellow, green);
        END_VAR
        "#
    )).unwrap();

    let ast_string = format!("{:#?}", &result.global_vars[0].variables[0]);

    let v = Variable{
        name: "my_enum".to_string(),
        data_type: DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::EnumType {
                name: None,
                elements: vec!["red".to_string(), "yellow".to_string(), "green".to_string()],
            }
        }
    };
    let expected_ast = format!("{:#?}", &v);
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn multilevel_inline_struct_and_enum_declaration_can_be_parsed() {
    let result = super::parse(lexer::lex(
        r#"
        VAR_GLOBAL
            my_struct : STRUCT
                    inner_enum: (red, yellow, green);
                    inner_struct: STRUCT
                        field: INT;
                    END_STRUCT
                END_STRUCT
        END_VAR
        "#
    )).unwrap();

    let ast_string = format!("{:#?}", &result.global_vars[0].variables[0]);
    let expected_ast = 
r#"Variable {
    name: "my_struct",
    data_type: DataTypeDefinition {
        data_type: StructType {
            name: None,
            variables: [
                Variable {
                    name: "inner_enum",
                    data_type: DataTypeDefinition {
                        data_type: EnumType {
                            name: None,
                            elements: [
                                "red",
                                "yellow",
                                "green",
                            ],
                        },
                    },
                },
                Variable {
                    name: "inner_struct",
                    data_type: DataTypeDefinition {
                        data_type: StructType {
                            name: None,
                            variables: [
                                Variable {
                                    name: "field",
                                    data_type: DataTypeReference {
                                        referenced_type: "INT",
                                    },
                                },
                            ],
                        },
                    },
                },
            ],
        },
    },
}"#;

    assert_eq!(ast_string, expected_ast);
}
