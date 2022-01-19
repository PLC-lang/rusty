use crate::{
    compile_to_string,
    diagnostics::{Diagnostic, Diagnostician},
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
    FUNCTION external() : INT
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
                "cannot generate call statement for Reference { name: \"external\" }",
                (30..38).into()
            ),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}
