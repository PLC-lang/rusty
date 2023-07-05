use plc::DebugLevel;
use source_code::SourceCode;

use crate::tests::compile_with_root;

#[test]
fn multiple_source_files_generated() {
    //Given 2 sources
    let src1 = SourceCode::new(
        "
    FUNCTION main : INT
    VAR_INPUT

    END_VAR

    VAR

    END_VAR
    mainProg();
    END_FUNCTION

    ",
        "external_file1.st",
    );
    let src2 = SourceCode::new(
        "
    PROGRAM mainProg
    VAR_TEMP
    END_VAR
    END_PROGRAM
    ",
        "external_file2.st",
    );
    //When the are generated
    let results = compile_with_root(vec![src1, src2], vec![], "root", DebugLevel::None).unwrap();
    assert_eq!(results.len(), 2);
    //The datatypes do not conflics
    //The functions are defined correctly
    insta::assert_snapshot!(results.join("\n"));
}

#[test]
fn multiple_files_with_debug_info() {
    //Given 2 sources
    let src1: SourceCode = SourceCode::new(
        "
    FUNCTION main : INT
    VAR_INPUT

    END_VAR

    VAR

    END_VAR
    mainProg();
    END_FUNCTION

    ",
        "file1.st",
    );

    let src2: SourceCode = SourceCode::new(
        "
    PROGRAM mainProg
    VAR_TEMP
    END_VAR
    END_PROGRAM
    ",
        "file2.st",
    );
    //When the are generated
    let results = compile_with_root(vec![src1, src2], vec![], "root", DebugLevel::Full).unwrap();
    assert_eq!(results.len(), 2);
    //The datatypes do not conflics
    //The functions are defined correctly
    insta::assert_snapshot!(results.join("\n"));
}

#[test]
fn multiple_files_in_different_locations_with_debug_info() {
    //Given 2 sources
    let src1: SourceCode = SourceCode::new(
        "
    FUNCTION main : INT
    VAR_INPUT

    END_VAR

    VAR

    END_VAR
    mainProg();
    END_FUNCTION

    ",
        "app/file1.st",
    );

    let src2: SourceCode = SourceCode::new(
        "
    PROGRAM mainProg
    VAR_TEMP
    END_VAR
    END_PROGRAM
    ",
        "lib/file2.st",
    );
    //When the are generated
    let results = compile_with_root(vec![src1, src2], vec![], "root", DebugLevel::Full).unwrap();
    assert_eq!(results.len(), 2);
    //The datatypes do not conflics
    //The functions are defined correctly
    insta::assert_snapshot!(results.join("\n"));
}
