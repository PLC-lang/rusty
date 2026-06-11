use plc_ast::ast::{LinkageType, PouType};

use crate::test_utils::tests::parse;

#[test]
fn simple_foo_program_can_be_parsed() {
    let src = "PROGRAM foo END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.pous[0];
    assert_eq!(prg.kind, PouType::Program);
    assert_eq!(prg.name, "foo");
    assert!(prg.return_type.is_none());
    assert_eq!(prg.linkage, LinkageType::Internal);
}

#[test]
fn external_simple_foo_program_can_be_parsed() {
    let src = "@EXTERNAL PROGRAM foo END_PROGRAM";
    let result = parse(src).0;

    let prg = &result.pous[0];
    assert_eq!(prg.kind, PouType::Program);
    assert_eq!(prg.name, "foo");
    assert!(prg.return_type.is_none());
    assert_eq!(prg.linkage, LinkageType::External);
}

#[test]
fn simple_program_with_variable_can_be_parsed() {
    let src = "PROGRAM buz VAR x : INT; END_VAR END_PROGRAM";
    let result = parse(src).0;

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
    variable_block_type: Local,
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn simple_program_with_var_input_can_be_parsed() {
    let src = "PROGRAM buz VAR_INPUT x : INT; END_VAR END_PROGRAM";
    let result = parse(src).0;

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
            ByVal,
        ),
    }
    "#);
}

#[test]
fn simple_program_with_var_output_can_be_parsed() {
    let src = "PROGRAM buz VAR_OUTPUT x : INT; END_VAR END_PROGRAM";
    let result = parse(src).0;

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
    variable_block_type: Output,
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn simple_program_with_var_inout_can_be_parsed() {
    let src = "PROGRAM buz VAR_IN_OUT x : INT; END_VAR END_PROGRAM";
    let result = parse(src).0;

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
    variable_block_type: InOut,
}"#;
    assert_eq!(ast_string, expected_ast);
}

#[test]
fn simple_program_with_var_temp_can_be_parsed() {
    let program = "PROGRAM buz VAR_TEMP x : INT; END_VAR END_PROGRAM";
    let result = parse(program).0;

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
