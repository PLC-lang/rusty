use plc::DebugLevel;
use source_code::SourceCode;

use crate::tests::compile_to_string;
use plc_util::filtered_snapshot;

#[test]
fn external_file_function_call() {
    //Given a program calling a function from an external file
    let prog = SourceCode::new(
        "
    FUNCTION main : INT
        external();
    END_FUNCTION
    ",
        "main.st",
    );

    let ext = SourceCode::new(
        "
    FUNCTION external : INT
    END_FUNCTION
    ",
        "external.st",
    );
    //When they are generated
    let results = compile_to_string(vec![prog], vec![ext], None, DebugLevel::None).unwrap();
    //Expect external to only be declared in the result
    filtered_snapshot!(results.join("\n"));
}

#[test]
fn external_file_global_var() {
    //Given a program calling a function from an external file
    let prog = SourceCode::new(
        "
    FUNCTION main : INT
        x := 2;
        y := 2;
        external();
    END_FUNCTION
    ",
        "main.st",
    );

    let ext = SourceCode::new(
        "
    VAR_GLOBAL
        x : INT;
    END_VAR
    FUNCTION external : INT
    END_FUNCTION
    VAR_GLOBAL
        y : INT;
    END_VAR
    ",
        "external.st",
    );
    //When they are generated
    let results = compile_to_string(vec![prog], vec![ext], None, DebugLevel::None).unwrap();
    //x should be external
    filtered_snapshot!(results.join("\n"));
}

#[test]
fn calling_external_file_function_without_including_file_results_in_error() {
    //Given a program calling a function from an external file
    let prog = SourceCode::new(
        "
    FUNCTION main : INT
        external();
    END_FUNCTION
    ",
        "external_file.st",
    );
    //External file is not included
    let res = compile_to_string(vec![prog], vec![], None, DebugLevel::None);

    if let Err(msg) = res {
        filtered_snapshot!(msg.to_string())
    } else {
        panic!("expected code-gen error but got none")
    }
}
