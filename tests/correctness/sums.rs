use super::super::*;

#[test]
fn adds_in_result() {
    let prog = 
    "
    FUNCTION main : INT
        main := 10 + 50;
    END_FUNCTION
    ";

    let (res, _) = compile_and_run(prog.to_string());
    assert_eq!(res,60)  
}