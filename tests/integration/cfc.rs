use driver::runner::compile_and_run;

use crate::get_test_file;
mod resolver_tests;
mod validation_tests;

#[test]
fn variables_assigned() {
    // GIVEN a ST program with hardcoded values and CFC program with two variables but without a body
    let st_file = get_test_file("cfc/assigning.st");
    let cfc_file = get_test_file("cfc/assigning.cfc");

    // WHEN assigning these values to the CFC-POU variables and adding them together
    let res: i32 = compile_and_run(vec![st_file, cfc_file], &mut {});

    // THEN we get the correct result
    assert_eq!(res, 300);
}

#[test]
fn simple_assignment() {
    // GIVEN a CFC program which assigns one variable to another
    let st_file = get_test_file("cfc/variable_assignment.st");
    let cfc_file = get_test_file("cfc/variable_assignment.cfc");
    // WHEN assigning values to them and then calling the program
    let res: i32 = compile_and_run(vec![st_file, cfc_file], &mut {});
    // THEN the second variable will have the value of the first variable
    assert_eq!(res, 10);
}

#[test]
fn select_call_in_function_block_with_input_variables() {
    // GIVEN a CFC program which selects a variable based on a predicate
    let st_file = get_test_file("cfc/select.st");
    let cfc_file = get_test_file("cfc/select.cfc");
    // WHEN assigning values to them and then calling the program
    let res: i32 = compile_and_run(vec![st_file, cfc_file], &mut {});
    // THEN the correct value is selected
    assert_eq!(res, 1);
}

#[test]
fn custom_function_call_in_function_block() {
    // GIVEN a CFC program which calls a subroutine
    let st_file = get_test_file("cfc/my_add.st");
    let cfc_file = get_test_file("cfc/my_add.cfc");
    // WHEN calling the
    let res: i32 = compile_and_run(vec![st_file, cfc_file], &mut {});
    // THEN the second variable will have the value of the first variable
    assert_eq!(res, 4);
}

#[test]
fn chained_calls() {
    // GIVEN a CFC program which assigns a variable
    let st_file = get_test_file("cfc/chained_calls.st");
    let cfc_file = get_test_file("cfc/chained_calls.cfc");
    // WHEN assigning values to them and then calling the program
    let res: i32 = compile_and_run(vec![st_file, cfc_file], &mut {});
    // THEN the second variable will have the value of the first variable
    assert_eq!(res, 10);
}

#[test]
fn chained_calls_galore() {
    // GIVEN a CFC program which assigns a variable
    let st_file = get_test_file("cfc/chained_calls_galore.st");
    let cfc_file = get_test_file("cfc/chained_calls_galore.cfc");
    // WHEN assigning values to them and then calling the program
    let res: i32 = compile_and_run(vec![st_file, cfc_file], &mut {});
    // THEN the second variable will have the value of the first variable
    assert_eq!(res, 88);
}

#[test]
fn function_returns() {
    // GIVEN a CFC function which doubles a value
    let st_file = get_test_file("cfc/function_returns.st");
    let cfc_file = get_test_file("cfc/function_returns.cfc");
    // WHEN passing a value into the function
    let res: i32 = compile_and_run(vec![st_file, cfc_file], &mut {});
    // THEN it will return the correct value
    assert_eq!(res, 222);
}

#[test]
fn conditional_return_evaluating_true() {
    // GIVEN a CFC function which returns early if a given argument is 5 and
    // otherwise modifies the argument to be 10
    let st_file = get_test_file("cfc/conditional_return_evaluating_true.st");
    let cfc_file = get_test_file("cfc/conditional_return.cfc");

    // WHEN passing 5 as an argument
    let res: i32 = compile_and_run(vec![st_file, cfc_file], &mut {});

    // THEN it will early return, leaving the argument unmodified (i.e. 5)
    assert_eq!(res, 5);
}

#[test]
fn conditional_return_evaluating_false() {
    // GIVEN a CFC function which returns early if a given argument is 5 and
    // otherwise modifies the argument to be 10
    let st_file = get_test_file("cfc/conditional_return_evaluating_false.st");
    let cfc_file = get_test_file("cfc/conditional_return.cfc");

    // WHEN passing 0 as an argument
    let res: i32 = compile_and_run(vec![st_file, cfc_file], &mut {});

    // THEN it will NOT return early and modify the argument to be 10
    assert_eq!(res, 10);
}

#[test]
fn conditional_return_evaluating_true_negated() {
    // GIVEN a CFC function which returns early if a given argument is NOT 5 and
    // otherwise modifies the argument to be 10
    let st_file = get_test_file("cfc/conditional_return_evaluating_true_negated.st");
    let cfc_file = get_test_file("cfc/conditional_return_negated.cfc");

    // WHEN passing 5 as an argument
    let res: i32 = compile_and_run(vec![st_file, cfc_file], &mut {});

    // THEN it will NOT return early, modifying the argument to be 10
    assert_eq!(res, 10);
}

#[test]
fn conditional_return_block_evaluating_true() {
    // GIVEN a CFC function which returns early if variable argument `a` is bigger than `b` and otherwise
    // modifies an argument `res` to be 10
    let st_file = get_test_file("cfc/conditional_return_block_evaluating_true.st");
    let cfc_file = get_test_file("cfc/conditional_return_block.cfc");

    // WHEN passing variables a = 1 and b = 0
    let res: i32 = compile_and_run(vec![st_file, cfc_file], &mut {});

    // THEN it will return early, leaving `res` unmodified
    assert_eq!(res, 5);
}

#[test]
fn conditional_return_block_evaluating_false() {
    // GIVEN a CFC function which returns early if variable argument `a` is bigger than `b` and otherwise
    // modifies an argument `res` to be 10
    let st_file = get_test_file("cfc/conditional_return_block_evaluating_false.st");
    let cfc_file = get_test_file("cfc/conditional_return_block.cfc");

    // WHEN passing variables a = 0 and b = 1
    let res: i32 = compile_and_run(vec![st_file, cfc_file], &mut {});

    // THEN it will NOT return early, modifying `res` to be 10
    assert_eq!(res, 10);
}

#[test]
fn connection_sink_source() {
    // GIVEN a CFC program which assigns variables through a sink-source-pair and adds them together
    let st_file = get_test_file("cfc/connection.st");
    let cfc_file = get_test_file("cfc/connection_var_source_multi_sink.cfc");
    // WHEN calling the program
    let res: i32 = compile_and_run(vec![st_file, cfc_file], &mut {});
    // THEN the result will have double the value of the initial value
    assert_eq!(res, 4);
}

#[test]
fn jump_to_label_with_true() {
    let cfc_file = get_test_file("cfc/jump_true.cfc");

    let res: i32 = compile_and_run(vec![cfc_file], &mut {});
    assert_eq!(res, 3);
}

#[test]
fn jump_to_label_with_false() {
    let cfc_file = get_test_file("cfc/jump_false.cfc");

    let res: i32 = compile_and_run(vec![cfc_file], &mut {});
    assert_eq!(res, 5);
}

#[test]
fn actions_called_correctly() {
    #[derive(Default)]
    #[repr(C)]
    struct MainType {
        a: i32,
        b: i32,
    }

    let mut main = MainType::default();

    let file = get_test_file("cfc/actions.cfc");
    let _: i32 = compile_and_run(vec![file], &mut main);

    assert_eq!(main.a, 1);
    assert_eq!(main.b, 2);
}

// TODO(volsa): Remove this once our `test_utils.rs` file has been polished to also support CFC.
// More specifically transform the following tests into simple codegen ones.
#[cfg(test)]
mod ir {
    use driver::{compile, generate_to_string, generate_to_string_debug};
    use plc_source::SourceCode;
    use plc_util::filtered_assert_snapshot;
    use plc_xmlgen::serializer::{
        SAction, SBlock, SConnector, SContinuation, SInVariable, SJump, SLabel, SOutVariable, SPou, SReturn,
    };

    use crate::get_test_file;

    const NEWLINE: &str = if cfg!(windows) { "\r\n" } else { "\n" };

    #[test]
    fn conditional_return() {
        let cfc_file = get_test_file("cfc/conditional_return.cfc");

        let output_file = tempfile::NamedTempFile::new().unwrap();
        let output_file_path = output_file.path().to_string_lossy();
        compile(&["plc", &cfc_file, "--ir", "-o", &output_file_path]).unwrap();
        let res = generate_to_string("plc", vec![cfc_file]).unwrap();

        // We truncate the first 3 lines of the snapshot file because they contain file-metadata that changes
        // with each run. This is due to working with temporary files (i.e. tempfile::NamedTempFile::new())
        let output_file_content_without_headers = res.lines().skip(3).collect::<Vec<&str>>().join(NEWLINE);
        filtered_assert_snapshot!(output_file_content_without_headers);
    }

    #[test]
    fn conditional_return_evaluating_true() {
        let st_file = get_test_file("cfc/conditional_return_evaluating_true.st");
        let cfc_file = get_test_file("cfc/conditional_return.cfc");

        let res = generate_to_string("plc", vec![st_file, cfc_file]).unwrap();

        // We truncate the first 3 lines of the snapshot file because they contain file-metadata that changes
        // with each run. This is due to working with temporary files (i.e. tempfile::NamedTempFile::new())
        let output_file_content_without_headers = res.lines().skip(3).collect::<Vec<&str>>().join(NEWLINE);
        filtered_assert_snapshot!(output_file_content_without_headers);
    }

    #[test]
    fn conditional_return_evaluating_true_negated() {
        let st_file = get_test_file("cfc/conditional_return_evaluating_true_negated.st");
        let cfc_file = get_test_file("cfc/conditional_return_negated.cfc");

        let res = generate_to_string("plc", vec![st_file, cfc_file]).unwrap();

        // We truncate the first 3 lines of the snapshot file because they contain file-metadata that changes
        // with each run. This is due to working with temporary files (i.e. tempfile::NamedTempFile::new())
        let output_file_content_without_headers = res.lines().skip(3).collect::<Vec<&str>>().join(NEWLINE);
        filtered_assert_snapshot!(output_file_content_without_headers);
    }

    #[test]
    fn variable_source_to_variable_and_block_sink() {
        let st_file = get_test_file("cfc/connection.st");
        let cfc_file = get_test_file("cfc/connection_var_source_multi_sink.cfc");

        let res = generate_to_string("plc", vec![st_file, cfc_file]).unwrap();
        // We truncate the first 3 lines of the snapshot file because they contain file-metadata that changes
        // with each run. This is due to working with temporary files (i.e. tempfile::NamedTempFile::new())
        let output_file_content_without_headers = res.lines().skip(3).collect::<Vec<&str>>().join(NEWLINE);
        filtered_assert_snapshot!(output_file_content_without_headers);
    }

    #[test]
    #[ignore = "block-to-block connections not yet implemented"]
    fn block_source_to_variable_and_block_sink() {
        let st_file = get_test_file("cfc/connection.st");
        let cfc_file = get_test_file("cfc/connection_block_source_multi_sink.cfc");

        let res = generate_to_string("plc", vec![st_file, cfc_file]).unwrap();

        // We truncate the first 3 lines of the snapshot file because they contain file-metadata that changes
        // with each run. This is due to working with temporary files (i.e. tempfile::NamedTempFile::new())
        let output_file_content_without_headers = res.lines().skip(3).collect::<Vec<&str>>().join(NEWLINE);
        filtered_assert_snapshot!(output_file_content_without_headers);
    }

    #[test]
    fn jump_to_label_with_true() {
        let cfc_file = get_test_file("cfc/jump_true.cfc");

        let res = generate_to_string("plc", vec![cfc_file]).unwrap();

        // We truncate the first 3 lines of the snapshot file because they contain file-metadata that changes
        // with each run. This is due to working with temporary files (i.e. tempfile::NamedTempFile::new())
        let output_file_content_without_headers = res.lines().skip(3).collect::<Vec<&str>>().join(NEWLINE);
        filtered_assert_snapshot!(output_file_content_without_headers);
    }

    #[test]
    fn jump_to_label_with_false() {
        let cfc_file = get_test_file("cfc/jump_false.cfc");

        let res = generate_to_string("plc", vec![cfc_file]).unwrap();

        // We truncate the first 3 lines of the snapshot file because they contain file-metadata that changes
        // with each run. This is due to working with temporary files (i.e. tempfile::NamedTempFile::new())
        let output_file_content_without_headers = res.lines().skip(3).collect::<Vec<&str>>().join(NEWLINE);
        filtered_assert_snapshot!(output_file_content_without_headers);
    }

    #[test]
    // TODO: Transfer this test to `codegen/tests/debug_tests/cfc.rs` once `test_utils.rs` has been refactored
    fn conditional_return_debug() {
        let declaration = "FUNCTION foo : DINT VAR_INPUT val : DINT; END_VAR";
        let content = SPou::init_str("foo", "function", declaration).with_fbd(vec![
            // IF val = 1 THEN RETURN
            Box::new(SInVariable::id(2).with_expression_str("val = 5")),
            Box::new(SReturn::id(3).with_execution_id(2).connect(2).negate(false)),
            // ELSE val := 10
            Box::new(SInVariable::id(4).with_expression_str("10")),
            Box::new(SInVariable::id(5).with_execution_id(3).connect(4).with_expression_str("val")),
        ]);

        let mut source = SourceCode::from(content.serialize());
        source.path = Some("<internal>.cfc".into());
        let res = generate_to_string_debug("plc", vec![source]).unwrap();

        // We truncate the first 3 lines of the snapshot file because they contain file-metadata that changes
        // with each run. This is due to working with temporary files (i.e. tempfile::NamedTempFile::new())
        let output_file_content_without_headers = res.lines().skip(3).collect::<Vec<&str>>().join(NEWLINE);

        // We expect two different !dbg statements for the return statement and its condition
        filtered_assert_snapshot!(output_file_content_without_headers);
    }

    #[test]
    // TODO: Transfer this test to `codegen/tests/debug_tests/cfc.rs` once `test_utils.rs` has been refactored
    fn jump_debug() {
        let declaration = "PROGRAM foo VAR val : DINT := 0; END_VAR";
        let content = SPou::init_str("foo", "program", declaration).with_fbd(vec![
            // IF TRUE THEN GOTO lbl
            Box::new(SInVariable::id(1).with_expression_str("val = 0")), // condition
            Box::new(SLabel::id(2).with_name_str("lbl").with_execution_id(1)), // label
            Box::new(SJump::id(3).with_name_str("lbl").with_execution_id(2).connect(1)), // statement
            // ELSE x := FALSE
            Box::new(SOutVariable::id(4).with_execution_id(3).with_expression_str("val").connect(5)), // assignment
            Box::new(SInVariable::id(5).with_expression_str("1")),
        ]);

        let mut source = SourceCode::from(content.serialize());
        source.path = Some("<internal>.cfc".into());
        let res = generate_to_string_debug("plc", vec![source]).unwrap();

        // We truncate the first 3 lines of the snapshot file because they contain file-metadata that changes
        // with each run. This is due to working with temporary files (i.e. tempfile::NamedTempFile::new())
        let output_file_content_without_headers = res.lines().skip(3).collect::<Vec<&str>>().join(NEWLINE);

        // We expect four different !dbg statement for the condition, label, statement and the assignment
        filtered_assert_snapshot!(output_file_content_without_headers);
    }

    #[test]
    // TODO: Transfer this test to `codegen/tests/debug_tests/cfc.rs` once `test_utils.rs` has been refactored
    fn actions_debug() {
        let content = SPou::init_str("main", "program", "PROGRAM main VAR a, b : DINT; END_VAR")
            .with_actions(vec![
                Box::new(SAction::name_str("newAction").with_fbd(vec![
                    Box::new(SOutVariable::id(1).with_expression_str("a").with_execution_id(0).connect(2)),
                    Box::new(SInVariable::id(2).with_expression_str("a + 1")),
                ])),
                Box::new(SAction::name_str("newAction2").with_fbd(vec![
                    Box::new(SInVariable::id(1).with_expression_str("b + 2")),
                    Box::new(SOutVariable::id(2).with_expression_str("b").with_execution_id(0).connect(1)),
                ])),
            ])
            .with_fbd(vec![
                Box::new(SBlock::id(1).with_name_str("newAction").with_execution_id(1)),
                Box::new(SBlock::id(2).with_name_str("newAction2").with_execution_id(2)),
                Box::new(SInVariable::id(4).with_expression_str("0")),
                Box::new(SOutVariable::id(3).with_expression_str("a").with_execution_id(0).connect(4)),
            ]);

        let mut source = SourceCode::from(content.serialize());
        source.path = Some("<internal>.cfc".into());
        let res = generate_to_string_debug("plc", vec![source]).unwrap();

        // We truncate the first 3 lines of the snapshot file because they contain file-metadata that changes
        // with each run. This is due to working with temporary files (i.e. tempfile::NamedTempFile::new())
        let output_file_content_without_headers = res.lines().skip(3).collect::<Vec<&str>>().join(NEWLINE);

        filtered_assert_snapshot!(output_file_content_without_headers);
    }

    #[test]
    // TODO: Transfer this test to `codegen/tests/debug_tests/cfc.rs` once `test_utils.rs` has been refactored
    fn sink_source_debug() {
        let content = SPou::init_str("main", "program", "PROGRAM main VAR x: DINT; END_VAR").with_fbd(vec![
            Box::new(SInVariable::id(1).with_expression_str("5")),
            Box::new(SConnector::id(2).with_name_str("s1").connect(1)),
            Box::new(SContinuation::id(3).with_name_str("s1")),
            Box::new(SOutVariable::id(4).with_expression_str("x").with_execution_id(1).connect(3)),
        ]);

        let mut source = SourceCode::from(content.serialize());
        source.path = Some("<internal>.cfc".into());
        let res = generate_to_string_debug("plc", vec![source]).unwrap();

        // We truncate the first 3 lines of the snapshot file because they contain file-metadata that changes
        // with each run. This is due to working with temporary files (i.e. tempfile::NamedTempFile::new())
        let output_file_content_without_headers = res.lines().skip(3).collect::<Vec<&str>>().join(NEWLINE);

        filtered_assert_snapshot!(output_file_content_without_headers);
    }
}
