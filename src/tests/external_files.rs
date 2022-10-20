use crate::{
    ast::SourceRange,
    diagnostics::{Diagnostic, Diagnostician},
    test_utils::tests::compile_to_string,
    SourceCode,
};

#[test]
fn external_file_function_call() {
    //Given a program calling a function from an external file
    let prog: SourceCode = "
    FUNCTION main : INT
    	external();
    END_FUNCTION
    "
    .into();

    let ext: SourceCode = "
    FUNCTION external : INT
	END_FUNCTION
    "
    .into();
    //When they are generated
    let res = compile_to_string(
        vec![prog],
        vec![ext],
        None,
        Diagnostician::null_diagnostician(),
    )
    .unwrap();
    insta::assert_snapshot!(res);
}

#[test]
fn external_file_global_var() {
    //Given a program calling a function from an external file
    let prog: SourceCode = "
    FUNCTION main : INT
        x := 2;
        y := 2;
    	external();
    END_FUNCTION
    "
    .into();

    let ext: SourceCode = "
    VAR_GLOBAL
        x : INT;
    END_VAR
    FUNCTION external : INT
	END_FUNCTION
    VAR_GLOBAL
        y : INT;
    END_VAR
    "
    .into();
    //When they are generated
    let res = compile_to_string(
        vec![prog],
        vec![ext],
        None,
        Diagnostician::null_diagnostician(),
    )
    .unwrap();
    //x should be external
    insta::assert_snapshot!(res);
}

#[test]
fn calling_external_file_function_without_including_file_results_in_error() {
    //Given a program calling a function from an external file
    let prog: SourceCode = "
    FUNCTION main : INT
    	external();
    END_FUNCTION
    "
    .into();
    //External file is not included
    let res = compile_to_string(
        vec![prog],
        vec![],
        None,
        Diagnostician::null_diagnostician(),
    );

    if let Err(msg) = res {
        assert_eq!(
            Diagnostic::codegen_error(
                r#"cannot generate call statement for "Reference { name: \"external\" }""#,
                SourceRange::in_file(30..38, "external_file.st")
            ),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}
