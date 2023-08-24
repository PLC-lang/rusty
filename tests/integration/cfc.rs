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
fn early_return() {
    // GIVEN a CFC function which returns early if a given argument is 5 and
    // otherwise modifies the argument to be 10
    let st_file = get_test_file("cfc/early_return.st");
    let cfc_file = get_test_file("cfc/early_return.cfc");

    // WHEN passing 5 as an argument
    let res: i32 = compile_and_run(vec![st_file, cfc_file], &mut {});

    // THEN it will early return, leaving the argument unmodified (i.e. 5)
    assert_eq!(res, 5);
}

#[test]
fn no_early_return() {
    // GIVEN a CFC function which returns early if a given argument is 5 and
    // otherwise modifies the argument to be 10
    let st_file = get_test_file("cfc/no_early_return.st");
    let cfc_file = get_test_file("cfc/early_return.cfc");

    // WHEN passing 0 as an argument
    let res: i32 = compile_and_run(vec![st_file, cfc_file], &mut {});

    // THEN it will NOT return early and modify the argument to be 10
    assert_eq!(res, 10);
}

#[test]
fn early_return_negated() {
    // GIVEN a CFC function which returns early if a given argument is NOT 5 and
    // otherwise modifies the argument to be 10
    let st_file = get_test_file("cfc/early_return_negated.st");
    let cfc_file = get_test_file("cfc/early_return_negated.cfc");

    // WHEN passing 5 as an argument
    let res: i32 = compile_and_run(vec![st_file, cfc_file], &mut {});

    // THEN it will NOT return early, modifying the argument to be 10
    assert_eq!(res, 10);
}
