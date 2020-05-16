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
    let mut index = rusty::create_index(); 
    let engine = compile(&context, &mut index, function);
    let mut case1 = MainType{theA : 4, theB: 7};
    let mut case2 = MainType{theA : 9, theB: -2};

    let (res, _) = run(&engine, "main", &mut case1);
    assert_eq!(res,7);
    let (res, _) = run(&engine, "main", &mut case2);
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
    let mut index = rusty::create_index(); 
    let engine = compile(&context, &mut index, function);
    let mut case1 = MainType{x : false,};
    let (res, _) = run(&engine, "main", &mut case1);
    assert_eq!(res,1);
}



#[test]
fn function_block_instances_save_state_per_instance() {
    #[allow(dead_code)]
    #[repr(C)]
    struct FooType {
        i : i32,
    }

    struct MainType {
        f : FooType,
        j : FooType,
    }
    let function = r#"
    FUNCTION_BLOCK foo : INT
    VAR_INPUT
        i : INT;
    END_VAR
    i := i + 1;
    END_FUNCTION_BLOCK

    PROGRAM main
    VAR 
        f : foo;
        j : foo;
    END_VAR 
    f();
    f();
    j(4);
    j();
    j();
    END_PROGRAM
    "#;
    
        let mut interface = MainType{ f: FooType{ i: 0}, j : FooType{ i: 0}};
        let (res, _) = compile_and_run(function.to_string(), &mut interface);
        assert_eq!(interface.f.i,2);
        assert_eq!(interface.j.i,7);
}
#[test]
fn program_instances_save_state_per() {
    #[allow(dead_code)]
    #[repr(C)]
    struct FooType {
        i : i32,
    }

    struct MainType {
        f : FooType,
    }
    let function = r#"
    PROGRAM main
    VAR_INPUT
        i : INT;
    END_VAR
    i := i + 1;
    END_PROGRAM
    "#;
    
        let mut interface = MainType{ f: FooType{ i: 4}};
        let context = inkwell::context::Context::create();
        let mut index = rusty::create_index(); 
        let exec_engine =compile(&context, &mut index, function.to_string());
        run(&exec_engine,"main", &mut interface);
        run(&exec_engine,"main", &mut interface);
        assert_eq!(interface.f.i,6);
}