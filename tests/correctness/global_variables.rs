use super::super::*;

#[test]
fn global_variable_can_be_referenced_in_fn() {
    let function = r"
    VAR_GLOBAL
        gX : INT;
    END_VAR
    FUNCTION main : INT
    VAR
        x : INT;
    END_VAR

    x := 10;
    gX := 20;

    gX := x + gX;

    main := gX;
    END_FUNCTION
    ";

    let (res, _) = compile_and_run(function.to_string());
    assert_eq!(res,30);
}

#[test]
fn global_variable_can_be_referenced_in_two_functions()  {

    let function = r"
    VAR_GLOBAL
        gX : INT;
    END_VAR
    FUNCTION main : INT
    VAR
        x : INT;
    END_VAR

    x := 10;
    gX := 20;

    gX := x + gX;

    main := gX;
    END_FUNCTION

    FUNCTION two : INT
    two := gX;
    END_FUNCTION
    ";
    let context = inkwell::context::Context::create();
    let exec_engine =compile(&context, function.to_string());

    let (res, _) = run(&exec_engine, "main");
    assert_eq!(res,30);
    let (res2, _) = run(&exec_engine, "two");
    assert_eq!(res2, 30)
}