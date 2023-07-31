// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::*;

#[test]
fn class_reference_in_pou() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MyClass {
        x: i16,
        y: i16,
    }

    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        cl: MyClass,
        x: i16,
    }

    let source = "
        CLASS MyClass
            VAR
                x, y : INT;
            END_VAR
        
            METHOD testMethod : INT
                VAR_INPUT myMethodArg : INT; END_VAR
                VAR myMethodLocalVar : INT; END_VAR
        
                x := myMethodArg;
                y := x + 1;
                myMethodLocalVar := y + 1;
                testMethod := myMethodLocalVar + 1;
            END_METHOD
        END_CLASS

        PROGRAM main 
        VAR
          cl : MyClass;
          x : INT := 0;
        END_VAR
        x := 1;
        cl.x := 1;
        x := x + cl.x;
        x := x + cl.testMethod(x);
        x := cl.testMethod(myMethodArg:= x);
        END_PROGRAM
        ";

    let mut m = MainType { cl: MyClass { x: 0, y: 0 }, x: 0 };
    let _: i32 = compile_and_run(source, &mut m);
    assert_eq!(m.x, 10);
}

#[test]
fn access_var_in_super_class() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x: i16,
        y: i16,
    }

    let source = "
        CLASS MyClass
            VAR
                x: INT;
            END_VAR
        END_CLASS

        CLASS MyClass2 EXTENDS MyCLASS
        VAR
            y: INT;
        END_VAR
        END_CLASS

        PROGRAM main 
        VAR
          x : INT := 0;
          y : INT := 0;
        END_VAR
        VAR_TEMP
            cl : MyClass2;
        END_VAR
        cl.y := 2;
        cl.x := 1;
        x := cl.x;
        y := cl.y;
        END_PROGRAM
        ";

    let mut m = MainType { x: 0, y: 0 };
    let _: i32 = compile_and_run(source, &mut m);
    assert_eq!(m.x, 1);
    assert_eq!(m.y, 2);
}
