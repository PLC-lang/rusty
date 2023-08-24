use plc::DebugLevel;
use plc_diagnostics::diagnostics::Diagnostic;
use source_code::{source_location::SourceLocation, SourceCode};

use crate::tests::compile_to_string;

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
    insta::assert_snapshot!(results.join("\n"));
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
    insta::assert_snapshot!(results.join("\n"));
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
        assert_eq!(
            Diagnostic::codegen_error(
                r#"cannot generate call statement for "ReferenceExpr { kind: Member(Identifier { name: \"external\" }), base: None }""#,
                SourceLocation::in_file(30..38, "external_file.st")
            ),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}
