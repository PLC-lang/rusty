use super::*;

//Same size operations remain the same
// Different types smaller than int converted to int (expanded according to sign)
// Different types with one element bigger than int converts all elements to its size
// Unary operator on an element equal to or bigger than int converts it to the next bigger size (if available)
// Expansions follow the sign of the original datatype 

/*
                                            x       x
        +-------+-------+-------+-------+-------+-------+
        |       | <=Int | DINT  | LINT  | REAL  | LREAL |
        +-------+-------+-------+-------+-------+-------+
        | <=INT | INT   | DINT  | LINT  | REAL  | LREAL |
        +-------+-------+-------+-------+-------+-------+
        | DINT  | DINT  | DINT  | LINT  | REAL  | LREAL |
        +-------+-------+-------+-------+-------+-------+
        | LINT  | LINT  | LINT  | LINT  | LREAL | LREAL |
        +-------+-------+-------+-------+-------+-------+
      x | REAL  | REAL  | REAL  | LREAL | REAL  | LREAL |
        +-------+-------+-------+-------+-------+-------+
      x | LREAL | LREAL | LREAL | LREAL | LREAL | LREAL |
        +-------+-------+-------+-------+-------+-------++

*/

#[test]
fn no_type_conversion_if_datatypes_are_the_same() {

    let result = codegen!(
        r#"PROGRAM prg
        VAR
        b : SINT;
        c : SINT;
        x : SINT;
        END_VAR

        x := b + c;

        END_PROGRAM
        "#
    );
    
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i16", "b"), ("i16","c"),("i16","x")],
        "void",
        "",
        "",
        r#"%load_b = load i16, i16* %b
  %load_c = load i16, i16* %c
  %tmpVar = add i16 %load_b, %load_c
  store i16 %tmpVar, i16* %x
  ret void
"#
    );

    assert_eq!(result,expected)
}

#[test]
fn datatypes_smaller_than_int_promoted_to_int() {
    let result = codegen!(
        r#"PROGRAM prg
        VAR
        b : SINT;
        c : INT;
        x : INT;
        END_VAR

        x := b + c;

        END_PROGRAM
        "#
    );
    
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i16", "b"), ("i32","c"),("i32","x")],
        "void",
        "",
        "",
        r#"%load_b = load i16, i16* %b
  %load_c = load i32, i32* %c
  %1 = sext i16 %load_b to i32
  %tmpVar = add i32 %1, %load_c
  store i32 %tmpVar, i32* %x
  ret void
"#
    );

    assert_eq!(result,expected)

}

#[test]
fn unsingned_datatypes_smaller_than_int_promoted_to_int() {
    let result = codegen!(
        r#"PROGRAM prg
        VAR
        b : BYTE;
        c : INT;
        x : INT;
        END_VAR

        x := b + c;

        END_PROGRAM
        "#
    );
    
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i8", "b"), ("i32","c"),("i32","x")],
        "void",
        "",
        "",
        r#"%load_b = load i8, i8* %b
  %load_c = load i32, i32* %c
  %1 = zext i8 %load_b to i32
  %tmpVar = add i32 %1, %load_c
  store i32 %tmpVar, i32* %x
  ret void
"#
    );

    assert_eq!(result,expected)

}

#[test]
fn datatypes_larger_than_int_promote_the_second_operand() {
    let result = codegen!(
        r#"PROGRAM prg
        VAR
        b : INT;
        c : DINT;
        x : LINT;
        END_VAR

        x := b + c;

        END_PROGRAM
        "#
    );
    
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i32", "b"), ("i64","c"),("i128","x")],
        "void",
        "",
        "",
        r#"%load_b = load i32, i32* %b
  %load_c = load i64, i64* %c
  %1 = sext i32 %load_b to i64
  %tmpVar = add i64 %1, %load_c
  %2 = sext i64 %tmpVar to i128
  store i128 %2, i128* %x
  ret void
"#
    );

    assert_eq!(result,expected)

}
