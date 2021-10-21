use crate::{ast::*, test_utils::tests::parse};

#[test]
fn simple_foo_program_can_be_parsed() {
    let src = "PROGRAM foo END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.units[0];
    assert_eq!(prg.pou_type, PouType::Program);
    assert_eq!(prg.name, "foo");
    assert!(prg.return_type.is_none());
}

#[test]
fn simple_program_with_variable_can_be_parsed() {
    let src = "PROGRAM buz VAR x : INT; END_VAR END_PROGRAM";
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
    ],
    variable_block_type: Local,
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn simple_program_with_var_input_can_be_parsed() {
    let src = "PROGRAM buz VAR_INPUT x : INT; END_VAR END_PROGRAM";
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
    ],
    variable_block_type: Input,
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn simple_program_with_var_output_can_be_parsed() {
    let src = "PROGRAM buz VAR_OUTPUT x : INT; END_VAR END_PROGRAM";
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
    ],
    variable_block_type: Output,
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn simple_program_with_var_inout_can_be_parsed() {
    let src = "PROGRAM buz VAR_IN_OUT x : INT; END_VAR END_PROGRAM";
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
    ],
    variable_block_type: InOut,
}"#;
    assert_eq!(ast_string, expected_ast);
}
