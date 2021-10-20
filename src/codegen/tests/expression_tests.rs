// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::codegen;

#[test]
fn pointers_in_function_return() {
    let result = codegen!(
        r#"FUNCTION func : REF_TO INT
        END_FUNCTION"#
    );
    insta::assert_snapshot!(result);
}

#[test]
fn structs_in_function_return() {
    let result = codegen!(
        r#"
        TYPE myStruct : STRUCT
            x : INT;
            END_STRUCT
        END_TYPE
        FUNCTION func : myStruct
        END_FUNCTION"#
    );
    insta::assert_snapshot!(result);
}
