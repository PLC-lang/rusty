// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{ast::SourceRange, codegen_wihout_unwrap, compile_error::CompileError};
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
        assert_eq!(CompileError::invalid_reference("y", (100..101).into()), msg);
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn exit_not_in_loop() {
    let result = codegen_wihout_unwrap!(
        "
        PROGRAM prg 
            VAR
                x : INT;
            END_VAR
            EXIT;
        END_PROGRAM
        "
    );
    if let Err(msg) = result {
        assert_eq!(
            CompileError::CodeGenError {
                message: "Cannot break out of loop when not inside a loop".into(),
                location: crate::ast::SourceRange::new(95..99),
            },
            msg
        );
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn continue_not_in_loop() {
    let result = codegen_wihout_unwrap!(
        "
        PROGRAM prg 
            VAR
                x : INT;
            END_VAR
            CONTINUE;
        END_PROGRAM
        "
    );
    if let Err(msg) = result {
        assert_eq!(
            CompileError::CodeGenError {
                message: "Cannot continue loop when not inside a loop".into(),
                location: crate::ast::SourceRange::new(95..103),
            },
            msg
        );
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
#[ignore]
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
        assert_eq!(
            CompileError::unknown_type("unknown_type", (17..18).into()),
            msg
        );
    } else {
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
        assert_eq!(
            CompileError::invalid_reference("MyStruct.c", (264..265).into()),
            msg
        );
    } else {
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
        assert_eq!(
            CompileError::codegen_error("Invalid array access".to_string(), (97..98).into()),
            msg
        );
    } else {
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
        assert_eq!(
            CompileError::codegen_error("Invalid array access".to_string(), (228..229).into()),
            msg
        );
    } else {
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

    let result = codegen_wihout_unwrap!(src);
    if let Err(msg) = result {
        // that's not perfect yet, we need display-names for generated datatypes
        assert_eq!(
            CompileError::invalid_reference("INT.a", (114..115).into()),
            msg
        )
    } else {
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

    let result = codegen_wihout_unwrap!(src);
    if let Err(msg) = result {
        // that's not perfect yet, we need display-names for generated datatypes
        assert_eq!(
            CompileError::invalid_reference("INT.index", (139..144).into()),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}


#[test]
fn invalid_initial_constant_values_in_pou_variables() {
    let result = codegen_wihout_unwrap!(
        r#"
        VAR_GLOBAL CONSTANT
            MAX_LEN : INT := 99;
        END_VAR

        VAR_GLOBAL
            LEN := MAX_LEN - 2;
        END_VAR
 
        PROGRAM prg
      	  VAR_INPUT
            my_len: INT := LEN + 4;  //cannot be evaluated at compile time!
          END_VAR
        END_PROGRAM
 
        "#
    );

    if let Err(msg) = result {
        assert_eq!(
            CompileError::codegen_error("Cannot generate literal initializer for 'prg.my_len': Value can not be derived".to_string(), (214..221).into()),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn constants_without_initialization() {
    let result = codegen_wihout_unwrap!(
        r#"
        VAR_GLOBAL CONSTANT
            a : INT;
            b : INT := a;
        END_VAR
        "#
    );

    if let Err(msg) = result {
        assert_eq!(
            CompileError::codegen_error("Cannot generate literal initializer for 'b': Value can not be derived".to_string(), SourceRange::undefined()),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}


#[test]
fn recursive_initial_constant_values() {
    let result = codegen_wihout_unwrap!(
        r#"
        VAR_GLOBAL CONSTANT
            a : INT := b;
            b : INT := a;
        END_VAR
        "#
    );

    if let Err(msg) = result {
        assert_eq!(
            CompileError::codegen_error("Cannot generate literal initializer for 'a': Value can not be derived".to_string(), SourceRange::undefined()),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}