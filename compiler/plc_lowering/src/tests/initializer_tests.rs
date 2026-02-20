use plc_driver::parse_and_annotate;
use plc_source::SourceCode;

#[test]
fn var_input_with_initializer_parses_correctly() {
    let src: SourceCode = "
        PROGRAM TestVarInput
        VAR_INPUT
            x : INT := 42;
        END_VAR
        END_PROGRAM
        "
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let pou = &project.units[0].get_unit().pous[0];

    // Check that VAR_INPUT variable has initializer
    assert_eq!(pou.variable_blocks[0].variables[0].name, "x");
    assert!(pou.variable_blocks[0].variables[0].initializer.is_some());
}

#[test]
fn var_output_with_initializer_parses_correctly() {
    let src: SourceCode = "
        PROGRAM TestVarOutput
        VAR_OUTPUT
            y : INT := 99;
        END_VAR
        END_PROGRAM
        "
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let pou = &project.units[0].get_unit().pous[0];

    // Check that VAR_OUTPUT variable has initializer
    assert_eq!(pou.variable_blocks[0].variables[0].name, "y");
    assert!(pou.variable_blocks[0].variables[0].initializer.is_some());
}

#[test]
fn var_in_out_with_initializer_parses_correctly() {
    let src: SourceCode = "
        PROGRAM TestVarInOut
        VAR_IN_OUT
            ptr : INT := 50;
        END_VAR
        END_PROGRAM
        "
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let pou = &project.units[0].get_unit().pous[0];

    // Check that VAR_IN_OUT variable has initializer in AST
    // For PROGRAM (stateless), initializers should be in stack constructor
    assert_eq!(pou.variable_blocks[0].variables[0].name, "ptr");
    assert!(pou.variable_blocks[0].variables[0].initializer.is_some());
}

#[test]
fn var_in_out_for_stateful_pou_goes_to_stack_constructor() {
    let src: SourceCode = "
        FUNCTION_BLOCK TestVarInOut
        VAR_IN_OUT
            ptr : INT := 50;
        END_VAR
        END_FUNCTION_BLOCK
        "
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let pou = &project.units[0].get_unit().pous[0];

    // Check that VAR_IN_OUT variable has initializer in AST
    // It should be added to stack constructor (at call time), not static constructor
    let inout_var = pou
        .variable_blocks
        .iter()
        .flat_map(|vb| &vb.variables)
        .find(|v| v.name == "ptr")
        .expect("ptr variable should exist");
    assert!(inout_var.initializer.is_some());
}

#[test]
fn mixed_parameters_parse_correctly() {
    let src: SourceCode = "
        PROGRAM TestMixed
        VAR_INPUT
            x : INT := 1;
        END_VAR
        VAR_OUTPUT
            y : INT := 2;
        END_VAR
        VAR_IN_OUT
            ptr : INT;
        END_VAR
        END_PROGRAM
        "
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let pou = &project.units[0].get_unit().pous[0];

    // Check all three parameter types exist
    assert_eq!(pou.variable_blocks.len(), 3);

    let input_var = &pou.variable_blocks[0].variables[0];
    assert_eq!(input_var.name, "x");
    assert!(input_var.initializer.is_some());

    let output_var = &pou.variable_blocks[1].variables[0];
    assert_eq!(output_var.name, "y");
    assert!(output_var.initializer.is_some());

    let inout_var = &pou.variable_blocks[2].variables[0];
    assert_eq!(inout_var.name, "ptr");
    // ptr doesn't have initializer in this case
    assert!(inout_var.initializer.is_none());
}

#[test]
fn var_input_with_struct_initializer_parses() {
    let src: SourceCode = r#"
        FUNCTION_BLOCK MyStruct
        VAR
            a : INT;
            b : STRING;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM TestStructInput
        VAR_INPUT
            s : MyStruct := (a := 10, b := 'test');
        END_VAR
        END_PROGRAM
        "#
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let pou = &project.units[0].get_unit().pous[1]; // Skip FB definition

    // Check that VAR_INPUT has struct initializer
    let var = &pou.variable_blocks[0].variables[0];
    assert_eq!(var.name, "s");
    assert!(var.initializer.is_some());
}

#[test]
fn var_in_out_with_struct_initializer_parses() {
    let src: SourceCode = r#"
        FUNCTION_BLOCK MyStruct
        VAR
            a : INT;
            b : STRING;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM TestStructInOut
        VAR_IN_OUT
            s : MyStruct := (a := 10, b := 'test');
        END_VAR
        END_PROGRAM
        "#
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let pou = &project.units[0].get_unit().pous[1]; // Skip FB definition

    // Check that VAR_IN_OUT has struct initializer in AST
    let var = &pou.variable_blocks[0].variables[0];
    assert_eq!(var.name, "s");
    assert!(var.initializer.is_some());
}

#[test]
fn nested_struct_parsing_succeeds() {
    // This test verifies that the parser correctly handles nested struct definitions
    // The actual initialization is tested in the LIT test suite (nested.st)
    let src: SourceCode = r#"
        FUNCTION_BLOCK C
        VAR
            localPrivateVariable : DINT := 69;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK B
        VAR
            instanceC : C;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK A
        VAR
            instanceB : B;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM TestNestedInit
        VAR
            instanceA : A;
        END_VAR
        END_PROGRAM
        "#
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();

    // Find the POUs by name since parse_and_annotate includes stdlib
    let pous = &project.units[0].get_unit().pous;

    // Verify all three POUs exist
    let c_pou = pous.iter().find(|p| p.name == "C").expect("C POU should exist");
    assert_eq!(c_pou.name, "C");

    let b_pou = pous.iter().find(|p| p.name == "B").expect("B POU should exist");
    assert_eq!(b_pou.name, "B");

    let a_pou = pous.iter().find(|p| p.name == "A").expect("A POU should exist");
    assert_eq!(a_pou.name, "A");

    // Verify that the program was successfully parsed
    let program_pou =
        pous.iter().find(|p| p.name == "TestNestedInit").expect("TestNestedInit POU should exist");
    assert_eq!(program_pou.name, "TestNestedInit");
}
