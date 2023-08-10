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

#[test]
fn use_method_to_change_field_in_super() {
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

        METHOD change_y
            y := 55; 
        END_METHOD

        METHOD change_x
            x := 44;
        END_METHOD
        END_CLASS

        PROGRAM main 
        VAR
          x : INT := 0;
          y : INT := 0;
        END_VAR
        VAR_TEMP
            cl : MyClass2;
        END_VAR
        cl.change_y();
        cl.change_x();
        x := cl.x;
        y := cl.y;
        END_PROGRAM
        ";

    let mut m = MainType { x: 0, y: 0 };
    let _: i32 = compile_and_run(source, &mut m);
    assert_eq!(m.x, 44);
    assert_eq!(m.y, 55);
}

#[test]
fn call_method_in_parent() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x: i16,
    }

    let source = "
        CLASS MyClass
            VAR
                x: INT;
            END_VAR
            METHOD change_x
                x := 10;
            END_METHOD
        END_CLASS

        CLASS MyClass2 EXTENDS MyCLASS

        END_CLASS

        PROGRAM main 
        VAR
          x : INT := 0;
        END_VAR
        VAR_TEMP
            cl : MyClass2;
        END_VAR
        cl.change_x();
        x := cl.x;
        END_PROGRAM
        ";

    let mut m = MainType { x: 0 };
    let _: i32 = compile_and_run(source, &mut m);
    assert_eq!(m.x, 10);
}

#[test]
fn call_overridden_method() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x: i16,
    }

    let source = "
        CLASS MyClass
            VAR
                x: INT;
            END_VAR
            METHOD change_x
                x := 10;
            END_METHOD
        END_CLASS

        CLASS MyClass2 EXTENDS MyCLASS
        METHOD OVERRIDE change_x
            x := 44;
        END_METHOD
        
        END_CLASS

        PROGRAM main 
        VAR
          x : INT := 0;
        END_VAR
        VAR_TEMP
            cl : MyClass2;
        END_VAR
        cl.change_x();
        x := cl.x;
        END_PROGRAM
        ";

    let mut m = MainType { x: 0 };
    let _: i32 = compile_and_run(source, &mut m);
    assert_eq!(m.x, 44);
}

#[test]
#[ignore]
//TODO this test can be used once methods are implemented using fp
fn call_method_from_class_given_by_ref() {
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
                y: INT;
            END_VAR

            METHOD change_x
                x := 10;
            END_METHOD

            METHOD change_y
                y := 15;
            END_METHOD
        END_CLASS

        CLASS MyClass2 EXTENDS MyCLASS
            METHOD OVERRIDE change_x
                x := 44;
            END_METHOD

            METHOD OVERRIDE change_y
                y := 55;
            END_METHOD
        END_CLASS

        CLASS MyClass3 
            METHOD callFuncX
            VAR_IN_OUT
                cls : MyClass;
            END_VAR
                cls.change_x();
            END_METHOD

            METHOD callFuncY
            VAR_IN_OUT
                cls : MyClass;
            END_VAR
                cls.change_y();
            END_METHOD
        END_CLASS

        PROGRAM main 
        VAR
          x : INT := 0;
          y : INT := 0;
        END_VAR
        VAR_TEMP
            cls2 : MyClass;
            cl : MyClass2;
            callcls : MyClass3;
        END_VAR

        callcls.callFuncX(cl);
        x := cl.x;

        callcls.callFuncY(cls2);
        y := cls2;

        END_PROGRAM
        ";

    let mut m = MainType { x: 0, y: 0 };
    let _: i32 = compile_and_run(source, &mut m);
    assert_eq!(m.x, 44);
    assert_eq!(m.y, 15);
}

#[test]
fn access_cls_fields_using_fp() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x: i16,
        y: i16,
    }

    let source = "
    CLASS MyClass
    VAR
        x : INT;
        y : INT;
    END_VAR
    END_CLASS

    PROGRAM MyProg
    VAR_IN_OUT
        cls : MyClass;
    END_VAR
    VAR_OUTPUT
        x : INT;
        y : INT;
    END_VAR
        x := cls.x;
        cls.y := y;
    END_PROGRAM

    PROGRAM main
    VAR_TEMP
        cls : MyClass;
    END_VAR
    VAR
        x : INT;
        y : INT;
    END_VAR
        cls.x := 2;
        MyProg.y := 3;
        MyProg(cls);
        x := MyProg.x;
        y := cls.y;
    END_PROGRAM
    ";

    let mut m = MainType { x: 0, y: 0 };
    let _: i32 = compile_and_run(source, &mut m);
    assert_eq!(m.x, 2);
    assert_eq!(m.y, 3);
}

#[test]
fn access_cls_fields_using_fp_via_function() {
    #[allow(dead_code)]
    #[repr(C)]
    struct MainType {
        x: i16,
        y: i16,
    }

    let source = "
    CLASS MyClass
    VAR
        x : INT;
        y : INT;
    END_VAR
    END_CLASS

    FUNCTION MyFunc : DINT
    VAR_IN_OUT
        cls : MyClass;
    END_VAR
    VAR_INPUT
        y : INT;
    END_VAR 
        MyFunc := cls.x;
        cls.y := y;
    END_FUNCTION

    PROGRAM main
    VAR_TEMP
        cls : MyClass;
    END_VAR
    VAR
        x : INT;
        y : INT;
    END_VAR
        cls.x := 2;
        x := MyFunc(cls,3);
        y := cls.y;
    END_PROGRAM
    ";

    let mut m = MainType { x: 0, y: 0 };
    let _: i32 = compile_and_run(source, &mut m);
    assert_eq!(m.x, 2);
    assert_eq!(m.y, 3);
}
