use super::super::*;

#[test]
fn max_function() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        theA : i32, 
        theB : i32,
    }
    
    let function = r#"

    FUNCTION MAX : INT 
    VAR_INPUT 
        a : INT;
        b : INT;
    END_VAR

    IF a > b THEN
        MAX := a;
    ELSE 
        MAX := b;
    END_IF
    END_FUNCTION

    FUNCTION main : INT
    VAR
        theA : INT;
        theB : INT;
    END_VAR

    main := MAX(theA, theB);

    END_FUNCTION

    "#.to_string();

    let context : Context = Context::create(); 
    let engine = compile(&context, function);
    let case1 = MainType{theA : 4, theB: 7};
    let case2 = MainType{theA : 9, theB: -2};

    let (res, _) = run(&engine, "main", case1);
    assert_eq!(res,7);
    let (res, _) = run(&engine, "main", case2);
    assert_eq!(res,9);
}

#[test]
#[ignore]
fn test_or_sideeffects() {
     #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x : bool,
    }
    
    let function = r#"
    VAR_GLOBAL
        res : INT;
    END_VAR
    FUNCTION OR_BRANCH : BOOL 
    VAR_INPUT 
        a : BOOL;
        b : INT;
    END_VAR

    OR_BRANCH := a;
    res := res + b;

    END_FUNCTION

    FUNCTION main : INT
    VAR
        x : BOOL;
    END_VAR

    x := OR_BRANCH(TRUE,1) OR OR_BRANCH(FALSE,2);
    main := res;

    END_FUNCTION

    "#.to_string();

    let context : Context = Context::create(); 
    let engine = compile(&context, function);
    let case1 = MainType{x : false,};
    let (res, _) = run(&engine, "main", case1);
    assert_eq!(res,1);
}