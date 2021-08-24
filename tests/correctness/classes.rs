// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::super::*;

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
        cl : MyClass,
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
                y := x;
                myMethodLocalVar = y;
                testMethod := myMethodLocalVar;
            END_METHOD
        END_CLASS

        FUNCTION main : DINT
        VAR
          cl : MyClass;
          x : INT := 0;
        END_VAR
        cl.x := 1;
        x := x + cl.x;
        x := x + cl.testMethod(x);
        x := cl.testMethod(myMethodArg:= x);
        END_FUNCTION
        ".into();

        let (res, _) = compile_and_run(source, &mut MainType{cl: MyClass{x: 0, y: 0}, x : 0});
        assert_eq!(res, 4);
}