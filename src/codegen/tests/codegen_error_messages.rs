/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::lexer;
use crate::parser;
use crate::index::Index;
use inkwell::context::Context;
use pretty_assertions::assert_eq;

#[test]
fn unknown_reference_should_be_reported_with_line_number() {
    let result = codegen_wihout_unwrap!(
        "
        PROGRAM prg 
            VAR
                x : INT;
            END_VAR
            x := y;
        END_PROGRAM
        "
    );
    if let Err(msg) = result {
        assert_eq!("Unknown reference 'y' at line: 6, offset: 18..19", msg);
    }else{
        panic!("expected code-gen error but got none")
    }
}


#[test]
fn unknown_type_should_be_reported_with_line_number() {
    let result = codegen_wihout_unwrap!(
        "
        PROGRAM prg 
            VAR
                x : unknown_type;
            END_VAR
            x := 7;
        END_PROGRAM
        "
    );
    if let Err(msg) = result {
        // that's not perfect yet, the error is reported for the region of the variable
        // but better than nothing
        assert_eq!("Unknown datatype 'unknown_type' at line: 4, offset: 17..18", msg);
    }else{
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn unknown_struct_field_should_be_reported_with_line_number() {
    let result = codegen_wihout_unwrap!(
        "
        TYPE MyStruct:
        STRUCT 
            a : INT;
            b : INT;
        END_STRUCT
        END_TYPE

        PROGRAM prg 
            VAR
                x : MyStruct;
            END_VAR
            x.a := 7;
            x.b := 8;
            x.c := 9;
        END_PROGRAM
        "
    );
    if let Err(msg) = result {
        assert_eq!("Unknown reference 'MyStruct.c' at line: 15, offset: 15..16", msg);
    }else{
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn invalid_array_access_should_be_reported_with_line_number() {
    let result = codegen_wihout_unwrap!(
        "
        PROGRAM prg 
            VAR
                x : INT;
            END_VAR
            x[3] := 3;
        END_PROGRAM
        "
    );
    if let Err(msg) = result {
        // that's not perfect yet, the error is reported for the region of the variable
        // but better than nothing
        assert_eq!("Invalid array access at line: 6, offset: 15..16", msg);
    }else{
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn invalid_array_access_in_struct_should_be_reported_with_line_number() {
    let result = codegen_wihout_unwrap!(
        "
        TYPE MyStruct:
        STRUCT 
            a : INT;
            b : INT;
        END_STRUCT
        END_TYPE

       PROGRAM prg 
            VAR
                x : MyStruct;
            END_VAR
            x.a := x.b[3];
        END_PROGRAM
        "
    );
    if let Err(msg) = result {
        assert_eq!("Invalid array access at line: 13, offset: 24..25", msg);
    }else{
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn invalid_struct_access_in_array_should_be_reported_with_line_number() {
    let src = "
       PROGRAM prg 
            VAR
                x : ARRAY[0..1] OF INT;
            END_VAR
            x[3].a := 2;
        END_PROGRAM
        ";

    let result = codegen_wihout_unwrap!(
            src);
    if let Err(msg) = result {
        // that's not perfect yet, we need display-names for generated datatypes
        assert_eq!("Unknown reference '__prg_x.a' at line: 6, offset: 18..19", msg);
    }else{
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn invalid_struct_access_in_array_access_should_be_reported_with_line_number() {
    let src = "
        PROGRAM prg 
            VAR
                x : ARRAY[0..1] OF INT;
                y : INT;
            END_VAR
            x[y.index] := 2;
        END_PROGRAM
        ";

    let result = codegen_wihout_unwrap!(
            src);
    if let Err(msg) = result {
        // that's not perfect yet, we need display-names for generated datatypes
        assert_eq!("Unknown reference 'INT.index' at line: 7, offset: 17..22", msg);
    }else{
        panic!("expected code-gen error but got none")
    }
}