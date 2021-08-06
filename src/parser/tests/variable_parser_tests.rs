use crate::parser::{parse, tests::lex};

#[test]
fn empty_global_vars_can_be_parsed() {
    let lexer = lex("VAR_GLOBAL END_VAR");
    let result = parse(lexer).0;

    let vars = &result.global_vars[0]; //globar_vars
    let ast_string = format!("{:#?}", vars);
    let expected_ast = r#"VariableBlock {
    variables: [],
    variable_block_type: Global,
}"#;
    assert_eq!(ast_string, expected_ast)
}

#[test]
fn global_vars_can_be_parsed() {
    let lexer = lex("VAR_GLOBAL x : INT; y : BOOL; END_VAR");
    let result = parse(lexer).0;

    let vars = &result.global_vars[0]; //globar_vars
    let ast_string = format!("{:#?}", vars);
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
            data_type: DataTypeReference {
                referenced_type: "BOOL",
            },
        },
    ],
    variable_block_type: Global,
}"#;
    assert_eq!(ast_string, expected_ast)
}

#[test]
fn global_single_line_vars_can_be_parsed() {
    let lexer = lex("VAR_GLOBAL x, y,z : INT; f : BOOL; b, c : SINT; END_VAR");
    let result = parse(lexer).0;

    let vars = &result.global_vars[0]; //globar_vars
    let ast_string = format!("{:#?}", vars);
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
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
        Variable {
            name: "z",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
        Variable {
            name: "f",
            data_type: DataTypeReference {
                referenced_type: "BOOL",
            },
        },
        Variable {
            name: "b",
            data_type: DataTypeReference {
                referenced_type: "SINT",
            },
        },
        Variable {
            name: "c",
            data_type: DataTypeReference {
                referenced_type: "SINT",
            },
        },
    ],
    variable_block_type: Global,
}"#;
    assert_eq!(ast_string, expected_ast)
}

#[test]
fn two_global_vars_can_be_parsed() {
    let lexer = lex("VAR_GLOBAL a: INT; END_VAR VAR_GLOBAL x : INT; y : BOOL; END_VAR");
    let result = parse(lexer).0;

    let vars = &result.global_vars; //global_vars
    let ast_string = format!("{:#?}", vars);
    let expected_ast = r#"[
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
    assert_eq!(ast_string, expected_ast)
}
