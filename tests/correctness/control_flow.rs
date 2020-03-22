use super::super::*;

macro_rules! permutate_conditionals {
    ($code: tt, $condition : tt) => {{
       let true_1 = format!($code, $condition = "TRUE" );
       let false_1 = format!($code, $condition = "FALSE");
       (true_1, false_1)
    }};
}

#[test]
fn adding_through_conditions() {
    let function = permutate_conditionals!(r#"
    FUNCTION main : INT
    VAR
        inc : INT;
        cond : BOOL;
    END_VAR

    cond := {cond};
    inc := 0;

    IF cond THEN
        inc := inc + 10;
    ELSE
        inc := inc + 100;
    END_IF

    main := inc;

    END_FUNCTION

    "#, cond);

    let (func_true, func_false) = function;

    println!("Func True : {}",func_true);
    println!("Func False : {}",func_false);

    let (res, _) = compile_and_run(func_true.to_string());
    assert_eq!(res,10);
    let (res, _) = compile_and_run(func_false.to_string());
    assert_eq!(res,100);
}