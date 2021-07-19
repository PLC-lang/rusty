use crate::{
    ast::*,
    parser::{parse, tests::lex},
    Diagnostic,
};
use pretty_assertions::*;

#[test]
fn simple_foo_function_can_be_parsed() {
    let lexer = lex("FUNCTION foo : INT END_FUNCTION");
    let result = parse(lexer).unwrap().0;

    let prg = &result.units[0];
    assert_eq!(prg.pou_type, PouType::Function);
    assert_eq!(prg.name, "foo");
    assert_eq!(
        prg.return_type.as_ref().unwrap(),
        &DataTypeDeclaration::DataTypeReference {
            referenced_type: "INT".to_string()
        }
    );
}

#[test]
fn simple_foo_function_block_can_be_parsed() {
    let lexer = lex("FUNCTION_BLOCK foo END_FUNCTION_BLOCK");
    let result = parse(lexer).unwrap().0;

    let prg = &result.units[0];
    assert_eq!(prg.pou_type, PouType::FunctionBlock);
    assert_eq!(prg.name, "foo");
    assert!(prg.return_type.is_none());
}

#[test]
fn a_function_with_varargs_can_be_parsed() {
    let lexer = lex("FUNCTION foo : INT VAR_INPUT x : INT; y : ...; END_VAR END_FUNCTION");
    let result = parse(lexer).unwrap().0;

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
    let lexer = lex("FUNCTION foo : INT VAR_INPUT x : INT; y : INT...; END_VAR END_FUNCTION");
    let result = parse(lexer).unwrap().0;

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
    let lexer = lex("
            FUNCTION foo : DINT
            VAR_INPUT
            args1 : ...;
            args2 : INT...;
            END_VAR
            END_FUNCTION
           ");
    let (parse_result, diagnostics) = parse(lexer).unwrap();

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
        }),
        variable_blocks: vec![VariableBlock {
            variable_block_type: VariableBlockType::Input,
            variables: vec![
                Variable {
                    name: "args1".into(),
                    data_type: DataTypeDeclaration::DataTypeDefinition {
                        data_type: DataType::VarArgs {
                            referenced_type: None,
                        },
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
                                },
                            )),
                        },
                    },
                    initializer: None,
                    location: SourceRange::undefined(),
                },
            ],
        }],
        location: SourceRange::undefined(),
    };
    assert_eq!(format!("{:#?}", expected), format!("{:#?}", x).as_str());
}
