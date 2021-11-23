use crate::{SourceCode, compile_to_string};

#[test]
fn multiple_source_files_generated() {
    //Given 2 sources
    let src1 : SourceCode = "
    FUNCTION main : INT
    VAR_INPUT

    END_VAR

    VAR

    END_VAR
    mainProg();
    END_FUNCTION

    ".into();
    let src2 : SourceCode = "
    PROGRAM mainProg
    VAR_TEMP
    END_VAR
    END_PROGRAM
    ".into();
    //When the are generated
    let res = compile_to_string(vec![src1,src2], None).unwrap();
    //The datatypes do not conflics
    //The functions are defined correctly
    insta::assert_snapshot!(res);
    
}