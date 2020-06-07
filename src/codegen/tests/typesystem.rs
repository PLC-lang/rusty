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

#[test]
#[ignore]
fn float_and_double_mix_converted_to_double() {
    let result = codegen!(
        r#"
        PROGRAM prg
        VAR
            a : REAL;
            b : LREAL;
            c : LREAL;
        END_VAR

        c := b + c;
        END_PROGRAM
        "#
    );

    let expected = generate_program_boiler_plate(
        "prg", &[("float","a"),("double", "b"),("double","c")], 
        "void", 
        "", "",
        r#"
        "#
    );

    assert_eq!(result,expected)
}

#[test]
fn int_assigned_to_float_is_cast() {
    let result = codegen!(
        r#"
        PROGRAM prg
        VAR
            a : INT;
            b : UINT;
            c : REAL;
        END_VAR
        c := a;
        c := b;
        END_PROGRAM
        "#
    );

    let expected = generate_program_boiler_plate(
        "prg", &[("i32","a"), ("i32", "b"),("float","c")], 
        "void", 
        "", "",
        r#"%load_a = load i32, i32* %a
  %1 = sitofp i32 %load_a to float
  store float %1, float* %c
  %load_b = load i32, i32* %b
  %2 = uitofp i32 %load_b to float
  store float %2, float* %c
  ret void
"#
    );

    assert_eq!(result,expected)
}

#[test]
fn float_assigned_to_int_is_cast() {
    let result = codegen!(
        r#"
        PROGRAM prg
        VAR
            a : INT;
            b : UINT;
            c : REAL;
        END_VAR
        a := c;
        b := c;
        END_PROGRAM
        "#
    );

    let expected = generate_program_boiler_plate(
        "prg", &[("i32","a"), ("i32", "b"),("float","c")], 
        "void", 
        "", "",
        r#"%load_c = load float, float* %c
  %1 = fptosi float %load_c to i32
  store i32 %1, i32* %a
  %load_c1 = load float, float* %c
  %2 = fptoui float %load_c1 to i32
  store i32 %2, i32* %b
  ret void
"#
    );

    assert_eq!(result,expected)
}

#[test]
fn int_smaller_or_equal_to_float_converted_to_float() {
    let result = codegen!(
        r#"
        PROGRAM prg
        VAR
            a : REAL;
            b : INT;
            c : REAL;
        END_VAR

        c := b + a;
        END_PROGRAM
        "#
    );

    let expected = generate_program_boiler_plate(
        "prg", &[("float","a"),("i32", "b"),("float","c")], 
        "void", 
        "", "",
        r#"%load_b = load i32, i32* %b
  %load_a = load float, float* %a
  %1 = sitofp i32 %load_b to float
  %tmpVar = fadd float %1, %load_a
  store float %tmpVar, float* %c
  ret void
"#
    );

    assert_eq!(result,expected)
}

#[test]
#[ignore]
fn int_bigger_than_float_converted_to_double() {
    let result = codegen!(
        r#"
        PROGRAM prg
        VAR
            a : REAL;
            b : DINT;
        END_VAR

        b + a;
        END_PROGRAM
        "#
    );

    let expected = generate_program_boiler_plate(
        "prg", &[("float","a"),("i64", "b")], 
        "void", 
        "", "",
        r#"%load_b = load i64, i64* %b
  %load_a = load float, float* %a
  %1 = sitofp i64 %load_b to double
  %tmpVar = fadd double %1, %load_a
  ret void
"#
    );

    assert_eq!(result,expected)
}
