use super::super::*;

#[test]
fn max_function() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        the_a: i32, 
        the_b: i32,
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
    let mut case1 = MainType{the_a : 4, the_b: 7};
    let mut case2 = MainType{the_a : 9, the_b: -2};

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
    FUNCTION_BLOCK foo
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
        let (_, _) = compile_and_run(function.to_string(), &mut interface);
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


#[test]
fn function_block_instances_save_state_per_instance_2() {
    
    #[allow(dead_code)]
    #[repr(C)]
    struct BazType {
        i : i32,
    }
    
    #[allow(dead_code)]
    #[repr(C)]
    struct FooType {
        i : i32,
        baz : BazType,
    }

    struct MainType {
        f : FooType,
        j : FooType,
    }
    let function = r#"
    FUNCTION_BLOCK Baz 
    VAR_INPUT
        i : INT;
    END_VAR
    i := i+1;
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK foo
    VAR_INPUT
        i : INT;
        baz: Baz; 
    END_VAR
    
    END_FUNCTION_BLOCK

    PROGRAM main
    VAR 
        f : foo;
        j : foo;
    END_VAR 
    f.baz.i := f.baz.i + 1;
    f.baz.i := f.baz.i + 1;


    j.baz.i := j.baz.i + 1;
    j.baz.i := j.baz.i + 1;
    j.baz.i := j.baz.i + 1;
    j.baz.i := j.baz.i + 1;
    END_PROGRAM
    "#;
    
        let mut interface = MainType{ f: FooType{ i: 0, baz: BazType{ i: 0}}, j : FooType{ i: 0, baz: BazType{i:0}}};
        let (_, _) = compile_and_run(function.to_string(), &mut interface);

        assert_eq!(2, interface.f.baz.i);
        assert_eq!(4, interface.j.baz.i);


}