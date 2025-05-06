// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::*;

#[test]
fn class_reference_in_pou() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MyClass {
        __vtable: usize,
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

    let mut m = MainType { cl: MyClass { __vtable: 0, x: 0, y: 0 }, x: 0 };
    let _: i32 = compile_and_run(source, &mut m);
    assert_eq!(m.x, 10);
}
