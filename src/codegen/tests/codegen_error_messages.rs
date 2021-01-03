/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::lexer;
use crate::parser;
use crate::index::Index;
use inkwell::context::Context;
use pretty_assertions::assert_eq;

#[test]
#[ignore]
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
        assert_eq!("Unknown reference 'y' at line: 6, offset: 17..18", msg);
    }else{
        panic!("expected code-gen error but got none")
    }

}