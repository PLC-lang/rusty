use crate::{test_utils::tests::compile_to_string, DebugLevel, SourceCode};

#[test]
fn multiple_source_files_generated() {
    //Given 2 sources
    let src1: SourceCode = "
    FUNCTION main : INT
    VAR_INPUT

    END_VAR

    VAR

    END_VAR
    mainProg();
    END_FUNCTION

    "
    .into();
    let src2: SourceCode = "
    PROGRAM mainProg
    VAR_TEMP
    END_VAR
    END_PROGRAM
    "
    .into();
    //When the are generated
    let res = compile_to_string(vec![src1, src2], vec![], None, DebugLevel::None).unwrap();
    //The datatypes do not conflics
    //The functions are defined correctly
    insta::assert_snapshot!(res);
}

#[test]
fn multiple_files_with_debug_info() {
    //Given 2 sources
    let src1: SourceCode = SourceCode {
        source: "
    FUNCTION main : INT
    VAR_INPUT

    END_VAR

    VAR

    END_VAR
    mainProg();
    END_FUNCTION

    "
        .to_string(),
        path: "file1.st".to_string(),
    };

    let src2: SourceCode = SourceCode {
        source: "
    PROGRAM mainProg
    VAR_TEMP
    END_VAR
    END_PROGRAM
    "
        .to_string(),
        path: "file2.st".to_string(),
    };
    //When the are generated
    let res = compile_to_string(vec![src1, src2], vec![], None, DebugLevel::Full).unwrap();
    //The datatypes do not conflics
    //The functions are defined correctly
    insta::assert_snapshot!(res);
}