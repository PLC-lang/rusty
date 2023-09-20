use driver::runner::compile_and_run;

use crate::get_test_file;

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

// TODO(volsa): Remove this once our `test_utils.rs` file has been polished to also support CFC.
// More specifically transform the following tests into simple codegen ones.
#[cfg(test)]
mod ir {
    use std::io::Read;

    use driver::compile;
    use insta::assert_snapshot;

    use crate::get_test_file;

    const NEWLINE: &str = if cfg!(windows) { "\r\n" } else { "\n" };

    #[test]
    fn conditional_return() {
        let cfc_file = get_test_file("cfc/conditional_return.cfc");

        let output_file = tempfile::NamedTempFile::new().unwrap();
        let output_file_path = output_file.path().to_string_lossy();
        compile(&["plc", &cfc_file, "--ir", "-o", &output_file_path]).unwrap();

        let mut output_file_handle = std::fs::File::open(output_file).unwrap();
        let mut output_file_content = String::new();
        output_file_handle.read_to_string(&mut output_file_content).unwrap();

        // We truncate the first 3 lines of the snapshot file because they contain file-metadata that changes
        // with each run. This is due to working with temporary files (i.e. tempfile::NamedTempFile::new())
        let output_file_content_without_headers = output_file_content.lines().skip(3).collect::<Vec<&str>>();
        assert_snapshot!(output_file_content_without_headers.join(NEWLINE));
    }

    #[test]
    fn conditional_return_evaluating_true() {
        let st_file = get_test_file("cfc/conditional_return_evaluating_true.st");
        let cfc_file = get_test_file("cfc/conditional_return.cfc");

        let output_file = tempfile::NamedTempFile::new().unwrap();
        let output_file_path = output_file.path().to_string_lossy();
        compile(&["plc", &st_file, &cfc_file, "--ir", "-o", &output_file_path]).unwrap();

        let mut output_file_handle = std::fs::File::open(output_file).unwrap();
        let mut output_file_content = String::new();
        output_file_handle.read_to_string(&mut output_file_content).unwrap();

        // We truncate the first 3 lines of the snapshot file because they contain file-metadata that changes
        // with each run. This is due to working with temporary files (i.e. tempfile::NamedTempFile::new())
        let output_file_content_without_headers = output_file_content.lines().skip(3).collect::<Vec<&str>>();
        assert_snapshot!(output_file_content_without_headers.join(NEWLINE));
    }

    #[test]
    fn conditional_return_evaluating_true_negated() {
        let st_file = get_test_file("cfc/conditional_return_evaluating_true_negated.st");
        let cfc_file = get_test_file("cfc/conditional_return_negated.cfc");

        let output_file = tempfile::NamedTempFile::new().unwrap();
        let output_file_path = output_file.path().to_string_lossy();
        compile(&["plc", &st_file, &cfc_file, "--ir", "-o", &output_file_path]).unwrap();

        let mut output_file_handle = std::fs::File::open(output_file).unwrap();
        let mut output_file_content = String::new();
        output_file_handle.read_to_string(&mut output_file_content).unwrap();

        // We truncate the first 3 lines of the snapshot file because they contain file-metadata that changes
        // with each run. This is due to working with temporary files (i.e. tempfile::NamedTempFile::new())
        let output_file_content_without_headers = output_file_content.lines().skip(3).collect::<Vec<&str>>();
        assert_snapshot!(output_file_content_without_headers.join(NEWLINE));
    }

    #[test]
    fn variable_source_to_variable_and_block_sink() {
        let st_file = get_test_file("cfc/connection.st");
        let cfc_file = get_test_file("cfc/connection_var_source_multi_sink.cfc");

        let result_file = tempfile::NamedTempFile::new().unwrap();
        let path = result_file.path();
        compile(&["plc", &st_file, &cfc_file, "-o", &path.to_str().unwrap(), "--ir"]).unwrap();
        let mut f = std::fs::File::open(path).expect("Temp-file should have been generated");
        let mut content = String::new();
        let _ = f.read_to_string(&mut content);
        let output_file_content_without_headers = content.lines().skip(3).collect::<Vec<&str>>();

        //Verify file content
        assert_snapshot!(output_file_content_without_headers.join(NEWLINE));

        //clean up
        let _ = std::fs::remove_file(path);
    }

    #[test]
    #[ignore = "block-to-block connections not yet implemented"]
    fn block_source_to_variable_and_block_sink() {
        let st_file = get_test_file("cfc/connection.st");
        let cfc_file = get_test_file("cfc/connection_block_source_multi_sink.cfc");

        let result_file = tempfile::NamedTempFile::new().unwrap();
        let path = result_file.path();
        compile(&["plc", &st_file, &cfc_file, "-o", &path.to_str().unwrap(), "--ir"]).unwrap();
        let mut f = std::fs::File::open(path).expect("Temp-file should have been generated");
        let mut content = String::new();
        let _ = f.read_to_string(&mut content);
        let output_file_content_without_headers = content.lines().skip(3).collect::<Vec<&str>>();

        //Verify file content
        assert_snapshot!(output_file_content_without_headers.join(NEWLINE));

        //clean up
        let _ = std::fs::remove_file(path);
    }
}
