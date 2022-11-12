// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{diagnostics::Diagnostic, test_utils::tests::codegen_without_unwrap};
use pretty_assertions::assert_eq;

#[test]
fn unknown_reference_should_be_reported_with_line_number() {
    let result = codegen_without_unwrap(
        "
        PROGRAM prg 
            VAR
                x : INT;
            END_VAR
            x := y;
        END_PROGRAM
        ",
    );
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::unresolved_reference("y", (100..101).into()),
            msg
        );
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn exit_not_in_loop() {
    let result = codegen_without_unwrap(
        "
        PROGRAM prg 
            VAR
                x : INT;
            END_VAR
            EXIT;
        END_PROGRAM
        ",
    );
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::codegen_error(
                "Cannot break out of loop when not inside a loop",
                (95..99).into()
            ),
            msg
        );
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn continue_not_in_loop() {
    let result = codegen_without_unwrap(
        "
        PROGRAM prg 
            VAR
                x : INT;
            END_VAR
            CONTINUE;
        END_PROGRAM
        ",
    );
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::codegen_error(
                "Cannot continue loop when not inside a loop",
                (95..103).into()
            ),
            msg
        );
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[ignore = "will be covered by validation"]
#[test]
fn unknown_struct_field_should_be_reported_with_line_number() {
    let result = codegen_without_unwrap(
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
        ",
    );
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::unresolved_reference("MyStruct.c", (264..265).into()),
            msg
        );
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn invalid_array_access_should_be_reported_with_line_number() {
    let result = codegen_without_unwrap(
        "
        PROGRAM prg 
            VAR
                x : INT;
            END_VAR
            x[3] := 3;
        END_PROGRAM
        ",
    );
    if let Err(msg) = result {
        // that's not perfect yet, the error is reported for the region of the variable
        // but better than nothing
        assert_eq!(
            Diagnostic::codegen_error("Invalid array access", (97..98).into()),
            msg
        );
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn invalid_array_access_in_struct_should_be_reported_with_line_number() {
    let result = codegen_without_unwrap(
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
        ",
    );
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::codegen_error("Invalid array access", (228..229).into()),
            msg
        );
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[ignore = "will be covered by validation"]
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

    let result = codegen_without_unwrap(src);
    if let Err(msg) = result {
        // that's not perfect yet, we need display-names for generated datatypes
        assert_eq!(
            Diagnostic::unresolved_reference("INT.a", (114..115).into()),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[ignore = "will be covered by validation"]
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

    let result = codegen_without_unwrap(src);
    if let Err(msg) = result {
        // that's not perfect yet, we need display-names for generated datatypes
        assert_eq!(
            Diagnostic::unresolved_reference("INT.index", (139..144).into()),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn invalid_initial_constant_values_in_pou_variables() {
    let result = codegen_without_unwrap(
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
 
        "#,
    );

    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::codegen_error(
                "Cannot generate literal initializer for 'prg.my_len': Value can not be derived",
                (214..221).into()
            ),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn recursive_initial_constant_values() {
    let result = codegen_without_unwrap(
        r#"
        VAR_GLOBAL CONSTANT
            a : INT := b;
            b : INT := a;
        END_VAR
        "#,
    );

    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::codegen_error("Cannot propagate constant value for 'b'", (52..53).into()),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn char_assigning_wstring_literal_results_in_casting_error() {
    // GIVEN program with char variable trying to assign a wide string literal
    // WHEN codegen
    let result = codegen_without_unwrap(
        r#"
	PROGRAM mainProg
	VAR
		x : CHAR;
	END_VAR
		x := "A";
	END_PROGRAM"#,
    );
    // THEN result should be a casting error
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::casting_error("WSTRING", "CHAR", (52..55).into()),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn wchar_assigning_string_literal_results_in_casting_error() {
    // GIVEN program with wchar variable trying to assing a string literal
    // WHEN codegen
    let result = codegen_without_unwrap(
        r#"
	PROGRAM mainProg
	VAR
		x : WCHAR;
	END_VAR
		x := 'A';
	END_PROGRAM"#,
    );
    // THEN result shoud be a casting error
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::casting_error("STRING", "WCHAR", (53..56).into()),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn assigning_string_literal_to_int_variable_results_in_casting_error() {
    // GIVEN program with an int variable trying to assing a string literal
    // WHEN codegen
    let result = codegen_without_unwrap(
        r#"
	PROGRAM mainProg
	VAR
		x : INT;
	END_VAR
		x := 'A';
	END_PROGRAM"#,
    );
    // THEN result shoud be a casting error
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::codegen_error(
                "Cannot generate String-Literal for type INT",
                (51..54).into(),
            ),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn assigning_empty_string_literal_to_char_results_in_error() {
    // GIVEN program with a char variable trying to assing an empty string literal
    // WHEN codegen
    let result = codegen_without_unwrap(
        r#"
	PROGRAM mainProg
	VAR
		x : CHAR;
	END_VAR
		x := '';
	END_PROGRAM"#,
    );
    // THEN result shoud be an error
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::cannot_generate_from_empty_literal("CHAR", (52..54).into()),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn assigning_empty_string_literal_to_wide_char_results_in_error() {
    // GIVEN program with a wide-char variable trying to assing an empty string literal
    // WHEN codegen
    let result = codegen_without_unwrap(
        r#"
	PROGRAM mainProg
	VAR
		x : WCHAR;
	END_VAR
		x := "";
	END_PROGRAM"#,
    );
    // THEN result shoud be an error
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::cannot_generate_from_empty_literal("WCHAR", (53..55).into()),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn pointer_binary_expression_adding_two_pointers() {
    let result = codegen_without_unwrap(
        r#"
	PROGRAM mainProg
	VAR
		x : INT;
		ptr : REF_TO INT;
	END_VAR
		ptr := &(x);
		ptr := ptr + ptr;
	END_PROGRAM"#,
    );
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::codegen_error("'+' operation must contain one int type", (88..97).into()),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn pointer_binary_expression_multiplication() {
    let result = codegen_without_unwrap(
        r#"
	PROGRAM mainProg
	VAR
		x : INT;
		ptr : REF_TO INT;
	END_VAR
		ptr := &(x);
		ptr := ptr * ptr;
	END_PROGRAM"#,
    );
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::codegen_error("Operator '*' unimplemented for pointers", (88..97).into()),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn pointer_binary_expression_division() {
    let result = codegen_without_unwrap(
        r#"
	PROGRAM mainProg
	VAR
		x : INT;
		ptr : REF_TO INT;
	END_VAR
		ptr := &(x);
		ptr := ptr / ptr;
	END_PROGRAM"#,
    );
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::codegen_error("Operator '/' unimplemented for pointers", (88..97).into()),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn pointer_binary_expression_modulo() {
    let result = codegen_without_unwrap(
        r#"
	PROGRAM mainProg
	VAR
		x : INT;
		ptr : REF_TO INT;
	END_VAR
		ptr := &(x);
		ptr := ptr MOD ptr;
	END_PROGRAM"#,
    );
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::codegen_error("Operator 'MOD' unimplemented for pointers", (88..99).into()),
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}
