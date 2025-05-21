use plc_util::filtered_assert_snapshot;

use crate::test_utils::tests::codegen_multi;
use plc_source::SourceCodeFactory;

#[test]
fn datatype_defined_in_external_file_in_module() {
    let units = vec![
        "
        TYPE myStruct : STRUCT
            x : DINT;
            z : REF_TO INT;
        END_STRUCT
        END_TYPE
        "
        .create_source("myStruct.st"),
        "
        PROGRAM prog
            VAR
                x : myStruct;
            END_VAR
        END_PROGRAM
        "
        .create_source("prog.st"),
    ];
    //Expecting struct in prog module
    filtered_assert_snapshot!(codegen_multi(units, crate::DebugLevel::None).join("\n"));
}

#[test]
fn datatype_defined_in_external_file_no_deps_in_module() {
    let units = vec![
        "
        TYPE myStruct : STRUCT
            x : DINT := 20;
            z : REF_TO INT;
        END_STRUCT
        END_TYPE
        "
        .create_source("myStruct.st"),
        "
        PROGRAM prog
            VAR
                x : DINT;
            END_VAR
        END_PROGRAM
        "
        .create_source("prog.st"),
    ];
    //Expecting no reference to struct or its initializers in prog
    filtered_assert_snapshot!(codegen_multi(units, crate::DebugLevel::None).join("\n"));
}

#[test]
fn datatype_initialized_in_external_file_in_module() {
    let units = vec![
        "
        TYPE MyInt : INT(1..10) := 5;
        "
        .create_source("MyInt.st"),
        "
        PROGRAM prog
            VAR
                x : MyInt;
            END_VAR
        END_PROGRAM
        "
        .create_source("prog.st"),
    ];
    //Expect the myInt initial value to be in prog
    filtered_assert_snapshot!(codegen_multi(units, crate::DebugLevel::None).join("\n"));
}

#[test]
fn global_value_from_different_file() {
    let units = vec![
        "
        VAR_GLOBAL
            x : DINT := d;
            y : DINT := e;
        END_VAR
        "
        .create_source("g1.st"),
        "
        VAR_GLOBAL CONSTANT
            c : DINT := 5;
            d : DINT := 6;
            e : DINT := 7;
        END_VAR
        "
        .create_source("g2.st"),
        "
        PROGRAM prog
            x := c + 2;
        END_PROGRAM
        "
        .create_source("prog.st"),
    ];
    //Expect x,c and d in prog
    filtered_assert_snapshot!(codegen_multi(units, crate::DebugLevel::None).join("\n"));
}

#[test]
fn struct_with_custom_init_in_different_file() {
    let units = vec![
        "
        TYPE myStruct : STRUCT
            x : DINT := 6;
            z : INT := 2;
        END_STRUCT
        END_TYPE
        "
        .create_source("myStruct.st"),
        "
        TYPE myStruct2 : STRUCT
            x : DINT := 6;
            z : INT := 2;
        END_STRUCT
        END_TYPE
        "
        .create_source("myStruct2.st"),
        "
        PROGRAM prog
            VAR
                x : myStruct := (x := 5);
                y : myStruct2;
            END_VAR
        END_PROGRAM
        "
        .create_source("prog.st"),
    ];
    //Expect the struct initializer in prog to have correct values
    filtered_assert_snapshot!(codegen_multi(units, crate::DebugLevel::None).join("\n"));
}

#[test]
fn function_defined_in_external_file() {
    let units = vec![
        "
        FUNCTION func : DINT
        END_FUNCTION
        "
        .create_source("func.st"),
        "
        FUNCTION_BLOCK fb
        END_FUNCTION_BLOCK
        "
        .create_source("fb.st"),
        "
        PROGRAM prg
        VAR a : DINT; END_VAR
        END_PROGRAM
        "
        .create_source("prg.st"),
        "
        PROGRAM prg2
        VAR b : DINT; END_VAR
        END_PROGRAM
        "
        .create_source("prg2.st"),
        "
        PROGRAM prog
            VAR
                myFb : fb;
            END_VAR
            prg.a;
            prg2();
            func();
            myFb();
        END_PROGRAM
        "
        .create_source("prog.st"),
    ];
    filtered_assert_snapshot!(codegen_multi(units, crate::DebugLevel::None).join("\n"));
}

#[test]
fn enum_referenced_in_fb_nested() {
    let units = vec![
        "
        TYPE myEnum : (a := 1,b := 2,c := 3) END_TYPE
        "
        .create_source("myEnum.st"),
        "
        FUNCTION_BLOCK fb
            VAR
                x : myEnum;
            END_VAR
        END_FUNCTION_BLOCK
        "
        .create_source("fb.st"),
        "TYPE myStruct STRUCT
            f : fb;
        END_STRUCT
        "
        .create_source("myStruct.st"),
        "
        FUNCTION_BLOCK fb2
            VAR
                x : myStruct;
            END_VAR
        END_FUNCTION_BLOCK
        "
        .create_source("fb2.st"),
        "
        FUNCTION_BLOCK fb3
        VAR
        END_VAR
        END_FUNCTION_BLOCK
        "
        .create_source("fb3.st"),
    ];
    //Expecting fb3 to have no enum references, but fb1 and 2 to have enums
    filtered_assert_snapshot!(codegen_multi(units, crate::DebugLevel::None).join("\n"));
}
