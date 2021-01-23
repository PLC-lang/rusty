/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::{codegen, generate_with_empty_program, lexer };
use crate::parser;
use crate::index::Index;
use inkwell::context::Context;
use pretty_assertions::assert_eq;

use super::{generate_program_boiler_plate, generate_program_boiler_plate_globals};



#[test]
fn program_with_variables_and_references_generates_void_function_and_struct_and_body() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
x : DINT;
y : DINT;
END_VAR
x;
y;
END_PROGRAM
"#
    );
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i32","x"),("i32","y")],
        "void",
        "",
        "",
        r#"%load_x = load i32, i32* %x
  %load_y = load i32, i32* %y
  ret void
"#
    );

    assert_eq!(result, expected);
}

#[test]
fn empty_global_variable_list_generates_nothing() {
    let result = generate_with_empty_program!("VAR_GLOBAL END_VAR");
    let expected = generate_program_boiler_plate_globals("");

    assert_eq!(result, expected);
}

#[test]
fn a_global_variables_generates_in_separate_global_variables() {
    let result = generate_with_empty_program!("VAR_GLOBAL gX : INT; gY : BOOL; END_VAR");
    let expected = generate_program_boiler_plate_globals(
r#"
@gX = global i16 0
@gY = global i1 false"#);

    assert_eq!(result, expected);
}

#[test]
fn two_global_variables_generates_in_separate_global_variables() {
    let result = generate_with_empty_program!("VAR_GLOBAL gX : INT; gY : BOOL; END_VAR VAR_GLOBAL gA : INT; END_VAR");
    let expected = generate_program_boiler_plate_globals(
r#"
@gX = global i16 0
@gY = global i1 false
@gA = global i16 0"#);

    assert_eq!(result, expected);
}

#[test]
fn global_variable_reference_is_generated() {
    let function = codegen!(r"
    VAR_GLOBAL
        gX : INT;
    END_VAR
    PROGRAM prg
    VAR
      x : INT;
    END_VAR
    gX := 20;
    x := gX;
    END_PROGRAM
    ");

    let expected = generate_program_boiler_plate("prg", &[("i16","x")], "void", "", 
r"
@gX = global i16 0", //global vars
r"store i16 20, i16* @gX
  %load_gX = load i16, i16* @gX
  store i16 %load_gX, i16* %x
  ret void
", //body
    );

    assert_eq!(function,expected)

}

#[test]
fn empty_program_with_name_generates_void_function() {
    let result = codegen!("PROGRAM prg END_PROGRAM");
    let expected = generate_program_boiler_plate("prg", &[], "void", "", "",
    r#"  ret void
"#);

    assert_eq!(result, expected);
}

#[test]
fn empty_function_with_name_generates_int_function() {
    let result = codegen!("FUNCTION foo : INT END_FUNCTION");
    let expected = 
    r#"; ModuleID = 'main'
source_filename = "main"

%foo_interface = type {}

define i16 @foo(%foo_interface* %0) {
entry:
  %foo = alloca i16
  %foo_ret = load i16, i16* %foo
  ret i16 %foo_ret
}
"#;
 
    assert_eq!(result, expected);
}
#[test]
fn program_with_variables_generates_void_function_and_struct() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
x : DINT;
y : DINT;
END_VAR
END_PROGRAM
"#
    );
    let expected = generate_program_boiler_plate("prg", &[("i32","x"),("i32","y")],"void","","",
    r#"ret void
"#);

    assert_eq!(result, expected);
}



#[test]
fn program_with_bool_variables_and_references_generates_void_function_and_struct_and_body() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
x : BOOL;
y : BOOL;
END_VAR
x;
y;
END_PROGRAM
"#
    );
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i1","x"),("i1","y")],
        "void",
        "",
        "",
        r#"%load_x = load i1, i1* %x
  %load_y = load i1, i1* %y
  ret void
"#
    );

    assert_eq!(result, expected);
}

#[test]
fn program_with_variables_and_additions_generates_void_function_and_struct_and_body() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
x : DINT;
y : DINT;
END_VAR
x + y;
END_PROGRAM
"#
    );
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i32","x"),("i32","y")],
        "void",
        "",
        "",
        r#"%load_x = load i32, i32* %x
  %load_y = load i32, i32* %y
  %tmpVar = add i32 %load_x, %load_y
  ret void
"#
    );

    assert_eq!(result, expected);
}

#[test]
fn program_with_variable_and_addition_literal_generates_void_function_and_struct_and_body() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
x : DINT;
END_VAR
x + 7;
END_PROGRAM
"#
    );
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i32","x")],
        "void",
        "",
        "",
        r#"%load_x = load i32, i32* %x
  %tmpVar = add i32 %load_x, 7
  ret void
"#
    );

    assert_eq!(result, expected);
}

#[test]
fn program_with_variable_assignment_generates_void_function_and_struct_and_body() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
y : DINT;
END_VAR
y := 7;
END_PROGRAM
"#
    );
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i32","y")],
        "void",
        "",
        "",
        r#"store i32 7, i32* %y
  ret void
"#
    );

    assert_eq!(result, expected);
}


#[test]
fn program_with_real_assignment() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
y : REAL;
END_VAR
y := 0.15625;
END_PROGRAM
"#
    );
    let expected = generate_program_boiler_plate(
        "prg",
        &[("float","y")],
        "void",
        "",
        "",
        r#"store float 1.562500e-01, float* %y
  ret void
"#
    );

    assert_eq!(result, expected);
}

#[test]
fn program_with_string_assignment() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
y : STRING;
END_VAR
y := 'im a genius';
END_PROGRAM
"#
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%prg_interface = type { [81 x i8] }

@prg_instance = global %prg_interface zeroinitializer

define void @prg(%prg_interface* %0) {
entry:
  %y = getelementptr inbounds %prg_interface, %prg_interface* %0, i32 0, i32 0
  store [12 x i8] c"im a genius\00", [81 x i8]* %y
  ret void
}
"#;

    assert_eq!(result, expected);
}

#[test]
fn program_with_real_additions() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
x : REAL;
y : REAL;
z : REAL;
END_VAR
x := 12.375;
y := 0.25;
z := x + y;
END_PROGRAM
"#
    );
    let expected = generate_program_boiler_plate(
        "prg",
        &[("float", "x"),("float","y"), ("float", "z")],
        "void",
        "",
        "",
        r#"store float 1.237500e+01, float* %x
  store float 2.500000e-01, float* %y
  %load_x = load float, float* %x
  %load_y = load float, float* %y
  %tmpVar = fadd float %load_x, %load_y
  store float %tmpVar, float* %z
  ret void
"#
    );

    assert_eq!(result, expected);
}


#[test]
fn program_with_boolean_assignment_generates_void_function_and_struct_and_body() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
y : BOOL;
END_VAR
y := TRUE;
y := FALSE;
END_PROGRAM
"#
    );
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i1","y")],
        "void",
        "",
        "",
        r#"store i1 true, i1* %y
  store i1 false, i1* %y
  ret void
"#
    );

    assert_eq!(result, expected);
}

#[test]
fn program_with_variable_and_arithmatic_assignment_generates_void_function_and_struct_and_body() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
x : DINT;
y : DINT;
END_VAR
y := x + 1;
y := x - 2;
y := x * 3;
y := x / 4;
y := x MOD 5;
END_PROGRAM
"#
    );
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i32","x"),("i32","y")],
        "void",
        "",
        "",
        r#"%load_x = load i32, i32* %x
  %tmpVar = add i32 %load_x, 1
  store i32 %tmpVar, i32* %y
  %load_x1 = load i32, i32* %x
  %tmpVar2 = sub i32 %load_x1, 2
  store i32 %tmpVar2, i32* %y
  %load_x3 = load i32, i32* %x
  %tmpVar4 = mul i32 %load_x3, 3
  store i32 %tmpVar4, i32* %y
  %load_x5 = load i32, i32* %x
  %tmpVar6 = sdiv i32 %load_x5, 4
  store i32 %tmpVar6, i32* %y
  %load_x7 = load i32, i32* %x
  %tmpVar8 = srem i32 %load_x7, 5
  store i32 %tmpVar8, i32* %y
  ret void
"#
    );

    assert_eq!(result, expected);
}

#[test]
fn program_with_variable_and_comparison_assignment_generates_void_function_and_struct_and_body() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
x : DINT;
y : BOOL;
END_VAR
y := x = 1;
y := x > 2;
y := x < 3;
y := x <> 4;
y := x >= 5;
y := x <= 6;
END_PROGRAM
"#
    );
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i32","x"),("i1","y")],
        "void",
        "",
        "",
        r#"%load_x = load i32, i32* %x
  %tmpVar = icmp eq i32 %load_x, 1
  store i1 %tmpVar, i1* %y
  %load_x1 = load i32, i32* %x
  %tmpVar2 = icmp sgt i32 %load_x1, 2
  store i1 %tmpVar2, i1* %y
  %load_x3 = load i32, i32* %x
  %tmpVar4 = icmp slt i32 %load_x3, 3
  store i1 %tmpVar4, i1* %y
  %load_x5 = load i32, i32* %x
  %tmpVar6 = icmp ne i32 %load_x5, 4
  store i1 %tmpVar6, i1* %y
  %load_x7 = load i32, i32* %x
  %tmpVar8 = icmp sge i32 %load_x7, 5
  store i1 %tmpVar8, i1* %y
  %load_x9 = load i32, i32* %x
  %tmpVar10 = icmp sle i32 %load_x9, 6
  store i1 %tmpVar10, i1* %y
  ret void
"#
    );

    assert_eq!(result, expected);
}

#[test]
fn program_with_and_statement() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
x : BOOL;
y : BOOL;
END_VAR
x AND y;
END_PROGRAM
"#
    );
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i1","x"),("i1","y")],
        "void",
        "",
        "",
        r#"%load_x = load i1, i1* %x
  %1 = icmp ne i1 %load_x, false
  br i1 %1, label %2, label %3

2:                                                ; preds = %entry
  %load_y = load i1, i1* %y
  br label %3

3:                                                ; preds = %2, %entry
  %4 = phi i1 [ %load_x, %entry ], [ %load_y, %2 ]
  ret void
"#
    );

    assert_eq!(result, expected);
}

#[test]
fn program_with_or_statement() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
x : BOOL;
y : BOOL;
END_VAR
x OR y;
END_PROGRAM
"#
    );
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i1","x"),("i1","y")],
        "void",
        "",
        "",
        r#"%load_x = load i1, i1* %x
  %1 = icmp ne i1 %load_x, false
  br i1 %1, label %3, label %2

2:                                                ; preds = %entry
  %load_y = load i1, i1* %y
  br label %3

3:                                                ; preds = %2, %entry
  %4 = phi i1 [ %load_x, %entry ], [ %load_y, %2 ]
  ret void
"#
    );

    assert_eq!(result, expected);
}

#[test]
fn program_with_xor_statement() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
x : BOOL;
y : BOOL;
END_VAR
x XOR y;
END_PROGRAM
"#
    );
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i1","x"),("i1","y")],
        "void",
        "",
        "",
        r#"%load_x = load i1, i1* %x
  %load_y = load i1, i1* %y
  %tmpVar = xor i1 %load_x, %load_y
  ret void
"#
    );

    assert_eq!(result, expected);
}

#[test]
fn program_with_negated_expressions_generates_void_function_and_struct_and_body() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
x : BOOL;
y : BOOL;
END_VAR
NOT x;
x AND NOT y;
END_PROGRAM
"#
    );
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i1","x"),("i1","y")],
        "void",
        "",
        "",
        r#"%load_x = load i1, i1* %x
  %tmpVar = xor i1 %load_x, true
  %load_x1 = load i1, i1* %x
  %1 = icmp ne i1 %load_x1, false
  br i1 %1, label %2, label %3

2:                                                ; preds = %entry
  %load_y = load i1, i1* %y
  %tmpVar2 = xor i1 %load_y, true
  br label %3

3:                                                ; preds = %2, %entry
  %4 = phi i1 [ %load_x1, %entry ], [ %tmpVar2, %2 ]
  ret void
"#
    );

    assert_eq!(result, expected);
}

#[test]
fn program_with_negated_combined_expressions_generates_void_function_and_struct_and_body() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
z : DINT;
y : BOOL;
END_VAR
y AND z >= 5;
NOT (z <= 6) OR y;
END_PROGRAM
"#
    );
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i32","z"),("i1","y")],
        "void",
        "",
        "",
        r#"%load_y = load i1, i1* %y
  %1 = icmp ne i1 %load_y, false
  br i1 %1, label %2, label %3

2:                                                ; preds = %entry
  %load_z = load i32, i32* %z
  %tmpVar = icmp sge i32 %load_z, 5
  br label %3

3:                                                ; preds = %2, %entry
  %4 = phi i1 [ %load_y, %entry ], [ %tmpVar, %2 ]
  %load_z1 = load i32, i32* %z
  %tmpVar2 = icmp sle i32 %load_z1, 6
  %tmpVar3 = xor i1 %tmpVar2, true
  %5 = icmp ne i1 %tmpVar3, false
  br i1 %5, label %7, label %6

6:                                                ; preds = %3
  %load_y4 = load i1, i1* %y
  br label %7

7:                                                ; preds = %6, %3
  %8 = phi i1 [ %tmpVar3, %3 ], [ %load_y4, %6 ]
  ret void
"#
    );

    assert_eq!(result, expected);
}

#[test]
fn program_with_signed_combined_expressions() {
    let result = codegen!(
        r#"PROGRAM prg
            VAR
            z : DINT;
            y : DINT;
            END_VAR
            -1 + z;
            2 +-z;
            -y + 3;
            END_PROGRAM
            "#
    );
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i32","z"),("i32","y")],
        "void",
        "",
        "",
        r#"%load_z = load i32, i32* %z
  %tmpVar = add i32 -1, %load_z
  %load_z1 = load i32, i32* %z
  %tmpVar2 = sub i32 0, %load_z1
  %tmpVar3 = add i32 2, %tmpVar2
  %load_y = load i32, i32* %y
  %tmpVar4 = sub i32 0, %load_y
  %tmpVar5 = add i32 %tmpVar4, 3
  ret void
"#
    );

    assert_eq!(result, expected);
}

#[test]
fn if_elsif_else_generator_test() {
    let result = codegen!(
        "
        PROGRAM prg 
        VAR
            x : DINT;
            y : DINT;
            z : DINT;
            u : DINT;
            b1 : BOOL;
            b2 : BOOL;
            b3 : BOOL;
        END_VAR
        IF b1 THEN
            x;
        ELSIF b2 THEN
            y;
        ELSIF b3 THEN
            z;
        ELSE
            u;
        END_IF
        END_PROGRAM
        "
    );
    let expected = generate_program_boiler_plate("prg",
    &[("i32","x"),("i32","y"),("i32", "z"), ("i32", "u"), ("i1", "b1"), ("i1", "b2"), ("i1", "b3")],
    "void",
    "",
    "",
r#"%load_b1 = load i1, i1* %b1
  br i1 %load_b1, label %condition_body, label %branch

condition_body:                                   ; preds = %entry
  %load_x = load i32, i32* %x
  br label %continue

branch:                                           ; preds = %entry
  %load_b2 = load i1, i1* %b2
  br i1 %load_b2, label %condition_body2, label %branch1

condition_body2:                                  ; preds = %branch
  %load_y = load i32, i32* %y
  br label %continue

branch1:                                          ; preds = %branch
  %load_b3 = load i1, i1* %b3
  br i1 %load_b3, label %condition_body3, label %else

condition_body3:                                  ; preds = %branch1
  %load_z = load i32, i32* %z
  br label %continue

else:                                             ; preds = %branch1
  %load_u = load i32, i32* %u
  br label %continue

continue:                                         ; preds = %else, %condition_body3, %condition_body2, %condition_body
  ret void
"#
    );

    assert_eq!(result, expected);
}

#[test]
fn if_generator_test() {
    let result = codegen!(
        "
        PROGRAM prg 
        VAR
            x : DINT;
            b1 : BOOL;
        END_VAR
        IF b1 THEN
            x;
        END_IF
        END_PROGRAM
        "
    );
    let expected = generate_program_boiler_plate("prg",
    &[("i32","x"),("i1","b1")],
    "void",
    "",
    "",
r#"%load_b1 = load i1, i1* %b1
  br i1 %load_b1, label %condition_body, label %continue

condition_body:                                   ; preds = %entry
  %load_x = load i32, i32* %x
  br label %continue

continue:                                         ; preds = %condition_body, %entry
  ret void
"#);

    assert_eq!(result, expected);
}

#[test]
fn if_with_expression_generator_test() {
    let result = codegen!(
        "
        PROGRAM prg 
        VAR
            x : DINT;
            b1 : BOOL;
        END_VAR
        IF (x > 1) OR b1 THEN
            x;
        END_IF
        END_PROGRAM
        "
    );
    let expected = generate_program_boiler_plate("prg",
    &[("i32","x"),("i1","b1")],
    "void",
    "",
    "",
r#"%load_x = load i32, i32* %x
  %tmpVar = icmp sgt i32 %load_x, 1
  %1 = icmp ne i1 %tmpVar, false
  br i1 %1, label %3, label %2

condition_body:                                   ; preds = %3
  %load_x1 = load i32, i32* %x
  br label %continue

continue:                                         ; preds = %condition_body, %3
  ret void

2:                                                ; preds = %entry
  %load_b1 = load i1, i1* %b1
  br label %3

3:                                                ; preds = %2, %entry
  %4 = phi i1 [ %tmpVar, %entry ], [ %load_b1, %2 ]
  br i1 %4, label %condition_body, label %continue
"#);

    assert_eq!(result, expected);
}

#[test]
fn for_statement_with_steps_test() {
    let result = codegen!(
        "
        PROGRAM prg 
        VAR
            x : DINT;
        END_VAR
        FOR x := 3 TO 10 BY 7 DO 
            x;
        END_FOR
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",
    &[("i32","x")],
    "void",
    "",
    "",
r#"store i32 3, i32* %x
  br label %condition_check

condition_check:                                  ; preds = %for_body, %entry
  %load_x = load i32, i32* %x
  %tmpVar = icmp sle i32 %load_x, 10
  br i1 %tmpVar, label %for_body, label %continue

for_body:                                         ; preds = %condition_check
  %load_x1 = load i32, i32* %x
  %tmpVar2 = add i32 %load_x, 7
  store i32 %tmpVar2, i32* %x
  br label %condition_check

continue:                                         ; preds = %condition_check
  ret void
"#);

    assert_eq!(result, expected);
}

#[test]
fn for_statement_without_steps_test() {
    let result = codegen!(
        "
        PROGRAM prg 
        VAR
            x : DINT;
        END_VAR
        FOR x := 3 TO 10 DO 
            x;
        END_FOR
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",&[("i32","x")],
    "void",
    "",
    "",
r#"store i32 3, i32* %x
  br label %condition_check

condition_check:                                  ; preds = %for_body, %entry
  %load_x = load i32, i32* %x
  %tmpVar = icmp sle i32 %load_x, 10
  br i1 %tmpVar, label %for_body, label %continue

for_body:                                         ; preds = %condition_check
  %load_x1 = load i32, i32* %x
  %tmpVar2 = add i32 %load_x, 1
  store i32 %tmpVar2, i32* %x
  br label %condition_check

continue:                                         ; preds = %condition_check
  ret void
"#);

    assert_eq!(result, expected);
}

#[test]
fn for_statement_continue() {
    let result = codegen!(
        "
        PROGRAM prg 
        VAR
            x : DINT;
        END_VAR
        FOR x := 3 TO 10 DO 
        END_FOR
        x;
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",&[("i32","x")],
    "void",
    "",
    "",
r#"store i32 3, i32* %x
  br label %condition_check

condition_check:                                  ; preds = %for_body, %entry
  %load_x = load i32, i32* %x
  %tmpVar = icmp sle i32 %load_x, 10
  br i1 %tmpVar, label %for_body, label %continue

for_body:                                         ; preds = %condition_check
  %tmpVar1 = add i32 %load_x, 1
  store i32 %tmpVar1, i32* %x
  br label %condition_check

continue:                                         ; preds = %condition_check
  %load_x2 = load i32, i32* %x
  ret void
"#);

    assert_eq!(result, expected);
}

#[test]
fn for_statement_with_references_steps_test() {
    let result = codegen!(
        "
        PROGRAM prg 
        VAR
            step: DINT;
            x : DINT;
            y : DINT;
            z : DINT;
        END_VAR
        FOR x := y TO z BY step DO 
            x;
        END_FOR
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",
    &[("i32","step"),("i32","x"),("i32","y"),("i32","z")],
    "void",
    "",
    "",
r#"%load_y = load i32, i32* %y
  store i32 %load_y, i32* %x
  br label %condition_check

condition_check:                                  ; preds = %for_body, %entry
  %load_x = load i32, i32* %x
  %load_z = load i32, i32* %z
  %tmpVar = icmp sle i32 %load_x, %load_z
  br i1 %tmpVar, label %for_body, label %continue

for_body:                                         ; preds = %condition_check
  %load_x1 = load i32, i32* %x
  %load_step = load i32, i32* %step
  %tmpVar2 = add i32 %load_x, %load_step
  store i32 %tmpVar2, i32* %x
  br label %condition_check

continue:                                         ; preds = %condition_check
  ret void
"#);

    assert_eq!(result, expected);
}

#[test]
fn while_statement() {
    let result = codegen!(
        "
        PROGRAM prg 
        VAR
            x : BOOL;
        END_VAR
        WHILE x DO
            x;
        END_WHILE
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",&[("i1","x")], 
    "void",
    "",
    "",
r#"br label %condition_check

condition_check:                                  ; preds = %entry, %while_body
  %load_x = load i1, i1* %x
  br i1 %load_x, label %while_body, label %continue

while_body:                                       ; preds = %condition_check
  %load_x1 = load i1, i1* %x
  br label %condition_check

continue:                                         ; preds = %condition_check
  ret void
"#);

    assert_eq!(result, expected);
}

#[test]
fn while_with_expression_statement() {
    let result = codegen!(
        "
        PROGRAM prg 
        VAR
            x : BOOL;
        END_VAR
        WHILE x = 0 DO
            x;
        END_WHILE
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",&[("i1","x")], 
    "void",
    "",
    "",
r#"br label %condition_check

condition_check:                                  ; preds = %entry, %while_body
  %load_x = load i1, i1* %x
  %1 = sext i1 %load_x to i32
  %tmpVar = icmp eq i32 %1, 0
  br i1 %tmpVar, label %while_body, label %continue

while_body:                                       ; preds = %condition_check
  %load_x1 = load i1, i1* %x
  br label %condition_check

continue:                                         ; preds = %condition_check
  ret void
"#);

    assert_eq!(result, expected);
}

#[test]
fn repeat_statement() {
    let result = codegen!(
        "
        PROGRAM prg 
        VAR
            x : BOOL;
        END_VAR
        REPEAT
            x;
        UNTIL x 
        END_REPEAT
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",&[("i1","x")], 
    "void",
    "",
    "",
r#"br label %while_body

condition_check:                                  ; preds = %while_body
  %load_x = load i1, i1* %x
  br i1 %load_x, label %while_body, label %continue

while_body:                                       ; preds = %entry, %condition_check
  %load_x1 = load i1, i1* %x
  br label %condition_check

continue:                                         ; preds = %condition_check
  ret void
"#);

    assert_eq!(result, expected);
}


#[test]
fn simple_case_statement() {
    let result = codegen!(
        "
        PROGRAM prg 
        VAR
            x : DINT;
            y : DINT;
        END_VAR
        CASE x OF
        1: y := 1;
        2: y := 2;
        3: y := 3;
        ELSE
            y := -1;
        END_CASE
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",&[("i32","x"),("i32","y")], 
    "void",
    "",
    "",
r#"%load_x = load i32, i32* %x
  switch i32 %load_x, label %else [
    i32 1, label %case
    i32 2, label %case1
    i32 3, label %case2
  ]

case:                                             ; preds = %entry
  store i32 1, i32* %y
  br label %continue

case1:                                            ; preds = %entry
  store i32 2, i32* %y
  br label %continue

case2:                                            ; preds = %entry
  store i32 3, i32* %y
  br label %continue

else:                                             ; preds = %entry
  store i32 -1, i32* %y
  br label %continue

continue:                                         ; preds = %else, %case2, %case1, %case
  ret void
"#);

    assert_eq!(result, expected);
}

#[test]
fn function_called_in_program() {
    let result = codegen!(
        "
        FUNCTION foo : DINT
        foo := 1;
        END_FUNCTION

        PROGRAM prg 
        VAR
            x : DINT;
        END_VAR
        x := foo();
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%prg_interface = type { i32 }
%foo_interface = type {}

@prg_instance = global %prg_interface zeroinitializer

define i32 @foo(%foo_interface* %0) {
entry:
  %foo = alloca i32
  store i32 1, i32* %foo
  %foo_ret = load i32, i32* %foo
  ret i32 %foo_ret
}

define void @prg(%prg_interface* %0) {
entry:
  %x = getelementptr inbounds %prg_interface, %prg_interface* %0, i32 0, i32 0
  %foo_instance = alloca %foo_interface
  br label %input

input:                                            ; preds = %entry
  br label %call

call:                                             ; preds = %input
  %call1 = call i32 @foo(%foo_interface* %foo_instance)
  br label %output

output:                                           ; preds = %call
  br label %continue

continue:                                         ; preds = %output
  store i32 %call1, i32* %x
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn real_function_called_in_program() {
    let result = codegen!(
        "
        FUNCTION foo : REAL
        foo := 1.0;
        END_FUNCTION

        PROGRAM prg 
        VAR
            x : DINT;
        END_VAR
        x := foo();
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%prg_interface = type { i32 }
%foo_interface = type {}

@prg_instance = global %prg_interface zeroinitializer

define float @foo(%foo_interface* %0) {
entry:
  %foo = alloca float
  store float 1.000000e+00, float* %foo
  %foo_ret = load float, float* %foo
  ret float %foo_ret
}

define void @prg(%prg_interface* %0) {
entry:
  %x = getelementptr inbounds %prg_interface, %prg_interface* %0, i32 0, i32 0
  %foo_instance = alloca %foo_interface
  br label %input

input:                                            ; preds = %entry
  br label %call

call:                                             ; preds = %input
  %call1 = call float @foo(%foo_interface* %foo_instance)
  br label %output

output:                                           ; preds = %call
  br label %continue

continue:                                         ; preds = %output
  %1 = fptosi float %call1 to i32
  store i32 %1, i32* %x
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn external_function_called_in_program() {
    let result = codegen!(
        "
        @EXTERNAL FUNCTION foo : DINT
        END_FUNCTION

        PROGRAM prg 
        foo();
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%prg_interface = type {}
%foo_interface = type {}

@prg_instance = global %prg_interface zeroinitializer

declare i32 @foo(%foo_interface*)

define void @prg(%prg_interface* %0) {
entry:
  %foo_instance = alloca %foo_interface
  br label %input

input:                                            ; preds = %entry
  br label %call

call:                                             ; preds = %input
  %call1 = call i32 @foo(%foo_interface* %foo_instance)
  br label %output

output:                                           ; preds = %call
  br label %continue

continue:                                         ; preds = %output
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn nested_function_called_in_program() {
    let result = codegen!(
        "
        FUNCTION bar : DINT
        bar := 1;
        END_FUNCTION

        FUNCTION foo : DINT
        VAR_INPUT
            in : DINT;
        END_VAR

        foo := 1;
        END_FUNCTION

        PROGRAM prg 
        VAR
            x : DINT;
        END_VAR
        x := foo(bar());
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%prg_interface = type { i32 }
%bar_interface = type {}
%foo_interface = type { i32 }

@prg_instance = global %prg_interface zeroinitializer

define i32 @bar(%bar_interface* %0) {
entry:
  %bar = alloca i32
  store i32 1, i32* %bar
  %bar_ret = load i32, i32* %bar
  ret i32 %bar_ret
}

define i32 @foo(%foo_interface* %0) {
entry:
  %in = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 0
  %foo = alloca i32
  store i32 1, i32* %foo
  %foo_ret = load i32, i32* %foo
  ret i32 %foo_ret
}

define void @prg(%prg_interface* %0) {
entry:
  %x = getelementptr inbounds %prg_interface, %prg_interface* %0, i32 0, i32 0
  %foo_instance = alloca %foo_interface
  br label %input

input:                                            ; preds = %entry
  %bar_instance = alloca %bar_interface
  br label %input1

call:                                             ; preds = %continue4
  %call6 = call i32 @foo(%foo_interface* %foo_instance)
  br label %output

output:                                           ; preds = %call
  br label %continue

continue:                                         ; preds = %output
  store i32 %call6, i32* %x
  ret void

input1:                                           ; preds = %input
  br label %call2

call2:                                            ; preds = %input1
  %call5 = call i32 @bar(%bar_interface* %bar_instance)
  br label %output3

output3:                                          ; preds = %call2
  br label %continue4

continue4:                                        ; preds = %output3
  %1 = getelementptr inbounds %foo_interface, %foo_interface* %foo_instance, i32 0, i32 0
  store i32 %call5, i32* %1
  br label %call
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn function_with_parameters_called_in_program() {
    let result = codegen!(
        "
        FUNCTION foo : DINT
        VAR_INPUT
          bar : DINT;
        END_VAR
        foo := 1;
        END_FUNCTION

        PROGRAM prg 
        VAR
        x : DINT;
        END_VAR
        x := foo(2);
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%prg_interface = type { i32 }
%foo_interface = type { i32 }

@prg_instance = global %prg_interface zeroinitializer

define i32 @foo(%foo_interface* %0) {
entry:
  %bar = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 0
  %foo = alloca i32
  store i32 1, i32* %foo
  %foo_ret = load i32, i32* %foo
  ret i32 %foo_ret
}

define void @prg(%prg_interface* %0) {
entry:
  %x = getelementptr inbounds %prg_interface, %prg_interface* %0, i32 0, i32 0
  %foo_instance = alloca %foo_interface
  br label %input

input:                                            ; preds = %entry
  %1 = getelementptr inbounds %foo_interface, %foo_interface* %foo_instance, i32 0, i32 0
  store i32 2, i32* %1
  br label %call

call:                                             ; preds = %input
  %call1 = call i32 @foo(%foo_interface* %foo_instance)
  br label %output

output:                                           ; preds = %call
  br label %continue

continue:                                         ; preds = %output
  store i32 %call1, i32* %x
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn function_with_two_parameters_called_in_program() {
    let result = codegen!(
        "
        FUNCTION foo : DINT
        VAR_INPUT
          bar : DINT;
          buz : BOOL;
        END_VAR
        foo := 1;
        END_FUNCTION

        PROGRAM prg 
        VAR
        x : DINT;
        END_VAR
        x := foo(2, TRUE);
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%prg_interface = type { i32 }
%foo_interface = type { i32, i1 }

@prg_instance = global %prg_interface zeroinitializer

define i32 @foo(%foo_interface* %0) {
entry:
  %bar = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 0
  %buz = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 1
  %foo = alloca i32
  store i32 1, i32* %foo
  %foo_ret = load i32, i32* %foo
  ret i32 %foo_ret
}

define void @prg(%prg_interface* %0) {
entry:
  %x = getelementptr inbounds %prg_interface, %prg_interface* %0, i32 0, i32 0
  %foo_instance = alloca %foo_interface
  br label %input

input:                                            ; preds = %entry
  %1 = getelementptr inbounds %foo_interface, %foo_interface* %foo_instance, i32 0, i32 0
  store i32 2, i32* %1
  %2 = getelementptr inbounds %foo_interface, %foo_interface* %foo_instance, i32 0, i32 1
  store i1 true, i1* %2
  br label %call

call:                                             ; preds = %input
  %call1 = call i32 @foo(%foo_interface* %foo_instance)
  br label %output

output:                                           ; preds = %call
  br label %continue

continue:                                         ; preds = %output
  store i32 %call1, i32* %x
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn function_with_local_var_initialization() {
    let result = codegen!(
        "
        FUNCTION foo : DINT
        VAR_INPUT
          in1 : DINT;
        END_VAR
        VAR
          x : INT := 7;
          y : INT;
          z : INT := 9;
        END_VAR
        foo := 1;
        END_FUNCTION
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%foo_interface = type { i32, i16, i16, i16 }

define i32 @foo(%foo_interface* %0) {
entry:
  %in1 = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 0
  %x = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 1
  %y = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 2
  %z = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 3
  %foo = alloca i32
  store i16 7, i16* %x
  store i16 9, i16* %z
  store i32 1, i32* %foo
  %foo_ret = load i32, i32* %foo
  ret i32 %foo_ret
}
"#;

  assert_eq!(result, expected);
}


#[test]
fn program_called_in_program() {
    let result = codegen!(
        "
        PROGRAM foo
        END_PROGRAM

        PROGRAM prg 
        foo();
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%foo_interface = type {}
%prg_interface = type {}

@foo_instance = global %foo_interface zeroinitializer
@prg_instance = global %prg_interface zeroinitializer

define void @foo(%foo_interface* %0) {
entry:
  ret void
}

define void @prg(%prg_interface* %0) {
entry:
  br label %input

input:                                            ; preds = %entry
  br label %call

call:                                             ; preds = %input
  call void @foo(%foo_interface* @foo_instance)
  br label %output

output:                                           ; preds = %call
  br label %continue

continue:                                         ; preds = %output
  ret void
}
"#;

  assert_eq!(result, expected);
}


#[test]
fn program_with_two_parameters_called_in_program() {
    let result = codegen!(
        "
        PROGRAM foo 
        VAR_INPUT
          bar : DINT;
          buz : BOOL;
        END_VAR
        END_PROGRAM

        PROGRAM prg 
          foo(2, TRUE);
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%foo_interface = type { i32, i1 }
%prg_interface = type {}

@foo_instance = global %foo_interface zeroinitializer
@prg_instance = global %prg_interface zeroinitializer

define void @foo(%foo_interface* %0) {
entry:
  %bar = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 0
  %buz = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 1
  ret void
}

define void @prg(%prg_interface* %0) {
entry:
  br label %input

input:                                            ; preds = %entry
  store i32 2, i32* getelementptr inbounds (%foo_interface, %foo_interface* @foo_instance, i32 0, i32 0)
  store i1 true, i1* getelementptr inbounds (%foo_interface, %foo_interface* @foo_instance, i32 0, i32 1)
  br label %call

call:                                             ; preds = %input
  call void @foo(%foo_interface* @foo_instance)
  br label %output

output:                                           ; preds = %call
  br label %continue

continue:                                         ; preds = %output
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn program_with_two_explicit_parameters_called_in_program() {
    let result = codegen!(
        "
        PROGRAM foo 
        VAR_INPUT
          bar : DINT;
          buz : BOOL;
        END_VAR
        END_PROGRAM

        PROGRAM prg 
          foo(buz := TRUE, bar := 2);
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%foo_interface = type { i32, i1 }
%prg_interface = type {}

@foo_instance = global %foo_interface zeroinitializer
@prg_instance = global %prg_interface zeroinitializer

define void @foo(%foo_interface* %0) {
entry:
  %bar = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 0
  %buz = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 1
  ret void
}

define void @prg(%prg_interface* %0) {
entry:
  br label %input

input:                                            ; preds = %entry
  store i1 true, i1* getelementptr inbounds (%foo_interface, %foo_interface* @foo_instance, i32 0, i32 1)
  store i32 2, i32* getelementptr inbounds (%foo_interface, %foo_interface* @foo_instance, i32 0, i32 0)
  br label %call

call:                                             ; preds = %input
  call void @foo(%foo_interface* @foo_instance)
  br label %output

output:                                           ; preds = %call
  br label %continue

continue:                                         ; preds = %output
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn program_with_var_out_called_in_program() {
    let result = codegen!(
        "
        PROGRAM foo 
        VAR_INPUT
          bar : DINT;
        END_VAR
        VAR_OUTPUT
          buz : BOOL;
        END_VAR
        END_PROGRAM

        PROGRAM prg 
        VAR
            baz : BOOL;
        END_VAR
          foo(bar := 2, buz => baz);
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%foo_interface = type { i32, i1 }
%prg_interface = type { i1 }

@foo_instance = global %foo_interface zeroinitializer
@prg_instance = global %prg_interface zeroinitializer

define void @foo(%foo_interface* %0) {
entry:
  %bar = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 0
  %buz = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 1
  ret void
}

define void @prg(%prg_interface* %0) {
entry:
  %baz = getelementptr inbounds %prg_interface, %prg_interface* %0, i32 0, i32 0
  br label %input

input:                                            ; preds = %entry
  store i32 2, i32* getelementptr inbounds (%foo_interface, %foo_interface* @foo_instance, i32 0, i32 0)
  br label %call

call:                                             ; preds = %input
  call void @foo(%foo_interface* @foo_instance)
  br label %output

output:                                           ; preds = %call
  %buz = load i1, i1* getelementptr inbounds (%foo_interface, %foo_interface* @foo_instance, i32 0, i32 1)
  store i1 %buz, i1* %baz
  br label %continue

continue:                                         ; preds = %output
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn program_with_var_out_called_mixed_in_program() {
    let result = codegen!(
        "
        PROGRAM foo 
        VAR_INPUT
          bar : DINT;
        END_VAR
        VAR_OUTPUT
          buz : BOOL;
        END_VAR
        END_PROGRAM

        PROGRAM prg 
        VAR
            baz : BOOL;
        END_VAR
          foo(buz => baz, bar := 2);
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%foo_interface = type { i32, i1 }
%prg_interface = type { i1 }

@foo_instance = global %foo_interface zeroinitializer
@prg_instance = global %prg_interface zeroinitializer

define void @foo(%foo_interface* %0) {
entry:
  %bar = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 0
  %buz = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 1
  ret void
}

define void @prg(%prg_interface* %0) {
entry:
  %baz = getelementptr inbounds %prg_interface, %prg_interface* %0, i32 0, i32 0
  br label %input

input:                                            ; preds = %entry
  store i32 2, i32* getelementptr inbounds (%foo_interface, %foo_interface* @foo_instance, i32 0, i32 0)
  br label %call

call:                                             ; preds = %input
  call void @foo(%foo_interface* @foo_instance)
  br label %output

output:                                           ; preds = %call
  %buz = load i1, i1* getelementptr inbounds (%foo_interface, %foo_interface* @foo_instance, i32 0, i32 1)
  store i1 %buz, i1* %baz
  br label %continue

continue:                                         ; preds = %output
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn function_called_when_shadowed() {
  let result = codegen!(
        "
        FUNCTION foo : DINT
        foo := 1;
        END_FUNCTION

        PROGRAM prg 
        VAR
            foo : DINT;
        END_VAR
        foo := foo();
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%prg_interface = type { i32 }
%foo_interface = type {}

@prg_instance = global %prg_interface zeroinitializer

define i32 @foo(%foo_interface* %0) {
entry:
  %foo = alloca i32
  store i32 1, i32* %foo
  %foo_ret = load i32, i32* %foo
  ret i32 %foo_ret
}

define void @prg(%prg_interface* %0) {
entry:
  %foo = getelementptr inbounds %prg_interface, %prg_interface* %0, i32 0, i32 0
  %foo_instance = alloca %foo_interface
  br label %input

input:                                            ; preds = %entry
  br label %call

call:                                             ; preds = %input
  %call1 = call i32 @foo(%foo_interface* %foo_instance)
  br label %output

output:                                           ; preds = %call
  br label %continue

continue:                                         ; preds = %output
  store i32 %call1, i32* %foo
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn function_block_instance_call() {
  let result = codegen!(
        "
        FUNCTION_BLOCK foo
        END_FUNCTION_BLOCK

        PROGRAM prg 
        VAR
            fb_inst : foo;
        END_VAR
        fb_inst();
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%prg_interface = type { %foo_interface }
%foo_interface = type {}

@prg_instance = global %prg_interface zeroinitializer

define void @foo(%foo_interface* %0) {
entry:
  ret void
}

define void @prg(%prg_interface* %0) {
entry:
  %fb_inst = getelementptr inbounds %prg_interface, %prg_interface* %0, i32 0, i32 0
  br label %input

input:                                            ; preds = %entry
  br label %call

call:                                             ; preds = %input
  call void @foo(%foo_interface* %fb_inst)
  br label %output

output:                                           ; preds = %call
  br label %continue

continue:                                         ; preds = %output
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn reference_qualified_name() {
  let result = codegen!(
        "
        FUNCTION_BLOCK fb
        VAR_INPUT
          x :DINT;
        END_VAR
        END_FUNCTION_BLOCK
        PROGRAM foo
        VAR_INPUT
            x : DINT;
            y : DINT;
            baz : fb;
        END_VAR
        END_PROGRAM
        PROGRAM prg 
        VAR
            x : DINT;
        END_VAR
            x := foo.x;
            x := foo.y;
            x := foo.baz.x;    
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%foo_interface = type { i32, i32, %fb_interface }
%fb_interface = type { i32 }
%prg_interface = type { i32 }

@foo_instance = global %foo_interface zeroinitializer
@prg_instance = global %prg_interface zeroinitializer

define void @fb(%fb_interface* %0) {
entry:
  %x = getelementptr inbounds %fb_interface, %fb_interface* %0, i32 0, i32 0
  ret void
}

define void @foo(%foo_interface* %0) {
entry:
  %x = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 0
  %y = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 1
  %baz = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 2
  ret void
}

define void @prg(%prg_interface* %0) {
entry:
  %x = getelementptr inbounds %prg_interface, %prg_interface* %0, i32 0, i32 0
  %load_ = load i32, i32* getelementptr inbounds (%foo_interface, %foo_interface* @foo_instance, i32 0, i32 0)
  store i32 %load_, i32* %x
  %load_1 = load i32, i32* getelementptr inbounds (%foo_interface, %foo_interface* @foo_instance, i32 0, i32 1)
  store i32 %load_1, i32* %x
  %load_2 = load i32, i32* getelementptr inbounds (%foo_interface, %foo_interface* @foo_instance, i32 0, i32 2, i32 0)
  store i32 %load_2, i32* %x
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn structs_are_generated() {
  let result = codegen!(
        "
        TYPE MyStruct: STRUCT
          a: DINT;
          b: DINT;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL
          x : MyStruct;
        END_VAR
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%MyStruct = type { i32, i32 }

@x = global %MyStruct zeroinitializer
"#;

  assert_eq!(result, expected);
}

#[test]
fn arrays_are_generated() {
  let result = codegen!(
        "
        TYPE MyArray: ARRAY[0..9] OF INT; END_TYPE

        VAR_GLOBAL
          x : MyArray;
        END_VAR
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

@x = external global [10 x i16]
"#;

  assert_eq!(result, expected);
}

#[test]
fn structs_members_can_be_referenced() {
  let result = codegen!(
        "
        TYPE MyStruct: STRUCT
          a: DINT;
          b: DINT;
        END_STRUCT
        END_TYPE

        PROGRAM MainProg 
        VAR
          Cord: MyStruct; 
        END_VAR
          Cord.a := 0;
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%MainProg_interface = type { %MyStruct }
%MyStruct = type { i32, i32 }

@MainProg_instance = global %MainProg_interface zeroinitializer

define void @MainProg(%MainProg_interface* %0) {
entry:
  %Cord = getelementptr inbounds %MainProg_interface, %MainProg_interface* %0, i32 0, i32 0
  %a = getelementptr inbounds %MyStruct, %MyStruct* %Cord, i32 0, i32 0
  store i32 0, i32* %a
  ret void
}
"#;

  assert_eq!(result, expected);
}


#[test]
fn enums_are_generated() {
  let result = codegen!(
        "
        TYPE MyEnum: (red, yellow, green);
        END_TYPE

        VAR_GLOBAL
          x : MyEnum;
        END_VAR
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

@red = global i32 0
@yellow = global i32 1
@green = global i32 2
@x = global i32 0
"#;

  assert_eq!(result, expected);
}


#[test]
fn enum_members_can_be_used_in_asignments() {
  let result = codegen!(
      "
      TYPE MyEnum: (red, yellow, green);
      END_TYPE

      PROGRAM main
      VAR
        color : MyEnum;
      END_VAR
      color := red;
      color := yellow;
      color := green;
      END_PROGRAM
      "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%main_interface = type { i32 }

@red = global i32 0
@yellow = global i32 1
@green = global i32 2
@main_instance = global %main_interface zeroinitializer

define void @main(%main_interface* %0) {
entry:
  %color = getelementptr inbounds %main_interface, %main_interface* %0, i32 0, i32 0
  %load_red = load i32, i32* @red
  store i32 %load_red, i32* %color
  %load_yellow = load i32, i32* @yellow
  store i32 %load_yellow, i32* %color
  %load_green = load i32, i32* @green
  store i32 %load_green, i32* %color
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn inline_structs_are_generated() {
  let result = codegen!(
        "
        
        VAR_GLOBAL
         x: STRUCT
              a: DINT;
              b: DINT;
            END_STRUCT
        END_VAR
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%__global_x = type { i32, i32 }

@x = global %__global_x zeroinitializer
"#;

  assert_eq!(result, expected);
}

#[test]
fn accessing_nested_structs() {
  let result = codegen!(
        "
        TYPE InnerStruct:
        STRUCT 
          inner1 : INT;
          inner2 : INT;
        END_STRUCT
        END_TYPE
        
        TYPE OuterStruct:
        STRUCT 
          out1 : InnerStruct;
          out2 : InnerStruct;
        END_STRUCT
        END_TYPE
        
        PROGRAM Main
        VAR
          m : OuterStruct;
        END_VAR

          m.out1.inner1 := 3;
          m.out2.inner2 := 7;
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%Main_interface = type { %OuterStruct }
%OuterStruct = type { %InnerStruct, %InnerStruct }
%InnerStruct = type { i16, i16 }

@Main_instance = global %Main_interface zeroinitializer

define void @Main(%Main_interface* %0) {
entry:
  %m = getelementptr inbounds %Main_interface, %Main_interface* %0, i32 0, i32 0
  %out1 = getelementptr inbounds %OuterStruct, %OuterStruct* %m, i32 0, i32 0
  %inner1 = getelementptr inbounds %InnerStruct, %InnerStruct* %out1, i32 0, i32 0
  store i16 3, i16* %inner1
  %out2 = getelementptr inbounds %OuterStruct, %OuterStruct* %m, i32 0, i32 1
  %inner2 = getelementptr inbounds %InnerStruct, %InnerStruct* %out2, i32 0, i32 1
  store i16 7, i16* %inner2
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn inline_enums_are_generated() {
  let result = codegen!(
        "
        VAR_GLOBAL
          x : (red, yellow, green);
        END_VAR
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

@red = global i32 0
@yellow = global i32 1
@green = global i32 2
@x = global i32 0
"#;

  assert_eq!(result, expected);
}

#[test]
fn basic_datatypes_generated() {
    let result = codegen!(
        "
        VAR_GLOBAL
            bool_1   : BOOL;
            byte_2   : BYTE;
            sint_3   : SINT;
            usint_4  : USINT;
            word_5   : WORD;
            int_6    : INT;
            uint_7   : UINT;
            dword_8  : DWORD;
            dint_9   : DINT;
            udint_10 : UDINT;
            lword_11 : LWORD;
            lint_12  : LINT;
            ulint_13 : ULINT;
        END_VAR
        "
    );
    let expected = r#"; ModuleID = 'main'
source_filename = "main"

@bool_1 = global i1 false
@byte_2 = global i8 0
@sint_3 = global i8 0
@usint_4 = global i8 0
@word_5 = global i16 0
@int_6 = global i16 0
@uint_7 = global i16 0
@dword_8 = global i32 0
@dint_9 = global i32 0
@udint_10 = global i32 0
@lword_11 = global i64 0
@lint_12 = global i64 0
@ulint_13 = global i64 0
"#;

    assert_eq!(result, expected);
}

#[test]
fn array_of_int_type_generated() {
    let result = codegen!(
        "
        PROGRAM prg 
            VAR
                x : ARRAY[0..10] OF INT;
            END_VAR
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",
    &[("[11 x i16]","x")],
    "void",
    "",
    "",
r#"ret void
"#);
    assert_eq!(result, expected);
}

#[test]
fn array_of_int_type_used() {
    let result = codegen!(
        "
        PROGRAM prg 
            VAR
                x : ARRAY[0..3] OF DINT;
            END_VAR
            x[1] := 3;
            x[2] := x[3] + 3;
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",
    &[("[4 x i32]","x")],
    "void",
    "",
    "",
r#"%tmpVar = getelementptr inbounds [4 x i32], [4 x i32]* %x, i32 0, i32 1
  store i32 3, i32* %tmpVar
  %tmpVar1 = getelementptr inbounds [4 x i32], [4 x i32]* %x, i32 0, i32 2
  %tmpVar2 = getelementptr inbounds [4 x i32], [4 x i32]* %x, i32 0, i32 3
  %load_tmpVar = load i32, i32* %tmpVar2
  %tmpVar3 = add i32 %load_tmpVar, 3
  store i32 %tmpVar3, i32* %tmpVar1
  ret void
"#);
    assert_eq!(result, expected);
}

#[test]
fn array_of_int_non_zero_type_generated() {
    let result = codegen!(
        "
        PROGRAM prg 
            VAR
                x : ARRAY[10..20] OF INT;
            END_VAR
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",
    &[("[11 x i16]","x")],
    "void",
    "",
    "",
r#"ret void
"#);
    assert_eq!(result, expected);
}

#[test]
fn array_of_int_type_with_non_zero_start_used() {
    let result = codegen!(
        "
        PROGRAM prg 
            VAR
                x : ARRAY[1..3] OF DINT;
            END_VAR
            x[1] := 3;
            x[2] := x[3] + 3;
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",
    &[("[3 x i32]","x")],
    "void",
    "",
    "",
r#"%tmpVar = getelementptr inbounds [3 x i32], [3 x i32]* %x, i32 0, i32 0
  store i32 3, i32* %tmpVar
  %tmpVar1 = getelementptr inbounds [3 x i32], [3 x i32]* %x, i32 0, i32 1
  %tmpVar2 = getelementptr inbounds [3 x i32], [3 x i32]* %x, i32 0, i32 2
  %load_tmpVar = load i32, i32* %tmpVar2
  %tmpVar3 = add i32 %load_tmpVar, 3
  store i32 %tmpVar3, i32* %tmpVar1
  ret void
"#);
    assert_eq!(result, expected);
}

#[test]
fn array_of_int_non_zero_negative_type_generated() {
    let result = codegen!(
        "
        PROGRAM prg 
            VAR
                x : ARRAY[-10..20] OF INT;
            END_VAR
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",
    &[("[31 x i16]","x")],
    "void",
    "",
    "",
r#"ret void
"#);
    assert_eq!(result, expected);
}

#[test]
fn array_of_int_type_with_non_zero_negative_start_used() {
    let result = codegen!(
        "
        PROGRAM prg 
            VAR
                x : ARRAY[-2..3] OF DINT;
            END_VAR
            x[-1] := 3;
            x[2] := x[3] + 3;
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",
    &[("[6 x i32]","x")],
    "void",
    "",
    "",
r#"%tmpVar = getelementptr inbounds [6 x i32], [6 x i32]* %x, i32 0, i32 1
  store i32 3, i32* %tmpVar
  %tmpVar1 = getelementptr inbounds [6 x i32], [6 x i32]* %x, i32 0, i32 4
  %tmpVar2 = getelementptr inbounds [6 x i32], [6 x i32]* %x, i32 0, i32 5
  %load_tmpVar = load i32, i32* %tmpVar2
  %tmpVar3 = add i32 %load_tmpVar, 3
  store i32 %tmpVar3, i32* %tmpVar1
  ret void
"#);
    assert_eq!(result, expected);
}

#[test]
fn multidim_array_declaration() {
    let result = codegen!(
        "
        PROGRAM prg 
            VAR
                x : ARRAY[0..1, 2..4] OF INT;
            END_VAR
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",
    &[("[2 x [3 x i16]]","x")],
    "void",
    "",
    "",
r#"ret void
"#);
    assert_eq!(result, expected);
}

#[test]
fn multidim_array_access() {
    let result = codegen!(
        "
        PROGRAM prg 
            VAR
                x : ARRAY[0..3, 1..2] OF DINT;
            END_VAR
            x[2, 1] := 3;
            x[3, 2] := x[1, 2] + 3;
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",
    &[("[4 x [2 x i32]]","x")],
    "void",
    "",
    "",
r#"%tmpVar = getelementptr inbounds [4 x [2 x i32]], [4 x [2 x i32]]* %x, i32 0, i32 2, i32 0
  store i32 3, i32* %tmpVar
  %tmpVar1 = getelementptr inbounds [4 x [2 x i32]], [4 x [2 x i32]]* %x, i32 0, i32 3, i32 1
  %tmpVar2 = getelementptr inbounds [4 x [2 x i32]], [4 x [2 x i32]]* %x, i32 0, i32 1, i32 1
  %load_tmpVar = load i32, i32* %tmpVar2
  %tmpVar3 = add i32 %load_tmpVar, 3
  store i32 %tmpVar3, i32* %tmpVar1
  ret void
"#);
    assert_eq!(result, expected);
}

#[test]
fn nested_array_declaration() {
    let result = codegen!(
        "
        PROGRAM prg 
            VAR
                x : ARRAY[2..4] OF ARRAY[0..1] OF INT;
            END_VAR
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",
    &[("[3 x [2 x i16]]","x")],
    "void",
    "",
    "",
r#"ret void
"#);
    assert_eq!(result, expected);
}

#[test]
fn nested_array_access() {
    let result = codegen!(
        "
        PROGRAM prg 
            VAR
                x : ARRAY[0..3] OF ARRAY[1..2] OF DINT;
            END_VAR
            x[2][1] := 3;
            x[3][2] := x[1][2] + 3;
        END_PROGRAM
        "
    );

    let expected = generate_program_boiler_plate("prg",
    &[("[4 x [2 x i32]]","x")],
    "void",
    "",
    "",
r#"%tmpVar = getelementptr inbounds [4 x [2 x i32]], [4 x [2 x i32]]* %x, i32 0, i32 2
  %tmpVar1 = getelementptr inbounds [2 x i32], [2 x i32]* %tmpVar, i32 0, i32 0
  store i32 3, i32* %tmpVar1
  %tmpVar2 = getelementptr inbounds [4 x [2 x i32]], [4 x [2 x i32]]* %x, i32 0, i32 3
  %tmpVar3 = getelementptr inbounds [2 x i32], [2 x i32]* %tmpVar2, i32 0, i32 1
  %tmpVar4 = getelementptr inbounds [4 x [2 x i32]], [4 x [2 x i32]]* %x, i32 0, i32 1
  %tmpVar5 = getelementptr inbounds [2 x i32], [2 x i32]* %tmpVar4, i32 0, i32 1
  %load_tmpVar = load i32, i32* %tmpVar5
  %tmpVar6 = add i32 %load_tmpVar, 3
  store i32 %tmpVar6, i32* %tmpVar3
  ret void
"#);
    assert_eq!(result, expected);
}

#[test]
fn accessing_nested_array_in_struct() {
  let result = codegen!(
        "
        TYPE MyStruct:
        STRUCT 
          field1 : ARRAY[0..4] OF INT;
        END_STRUCT
        END_TYPE
        
        PROGRAM Main
        VAR
          m : MyStruct;
        END_VAR

          m.field1[3] := 7;
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%Main_interface = type { %MyStruct }
%MyStruct = type { [5 x i16] }

@Main_instance = global %Main_interface zeroinitializer

define void @Main(%Main_interface* %0) {
entry:
  %m = getelementptr inbounds %Main_interface, %Main_interface* %0, i32 0, i32 0
  %field1 = getelementptr inbounds %MyStruct, %MyStruct* %m, i32 0, i32 0
  %tmpVar = getelementptr inbounds [5 x i16], [5 x i16]* %field1, i32 0, i32 3
  store i16 7, i16* %tmpVar
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn initial_values_in_global_variables() {
  let result = codegen!(
        "
        VAR_GLOBAL
          x : INT := 7;
          y : BOOL := TRUE;
          z : REAL := 3.1415;
        END_VAR
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

@x = global i16 7
@y = global i1 true
@z = global float 0x400921CAC0000000
"#;

  assert_eq!(result, expected);
}

#[test]
fn initial_values_in_program_pou() {
  let result = codegen!(
        "
        PROGRAM Main
        VAR
          x : INT := 7;
          xx : INT;
          y : BOOL := TRUE;
          yy : BOOL;
          z : REAL := 3.1415;
          zz : REAL;
        END_VAR
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%Main_interface = type { i16, i16, i1, i1, float, float }

@Main_instance = global %Main_interface { i32 7, i16 0, i1 true, i1 false, float 0x400921CAC0000000, float 0.000000e+00 }

define void @Main(%Main_interface* %0) {
entry:
  %x = getelementptr inbounds %Main_interface, %Main_interface* %0, i32 0, i32 0
  %xx = getelementptr inbounds %Main_interface, %Main_interface* %0, i32 0, i32 1
  %y = getelementptr inbounds %Main_interface, %Main_interface* %0, i32 0, i32 2
  %yy = getelementptr inbounds %Main_interface, %Main_interface* %0, i32 0, i32 3
  %z = getelementptr inbounds %Main_interface, %Main_interface* %0, i32 0, i32 4
  %zz = getelementptr inbounds %Main_interface, %Main_interface* %0, i32 0, i32 5
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn initial_values_in_function_block_pou() {
  let result = codegen!(
        "
        FUNCTION_BLOCK FB
        VAR
          x : INT := 7;
          xx : INT;
          y : BOOL := TRUE;
          yy : BOOL;
          z : REAL := 3.1415;
          zz : REAL;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM main
        VAR
          fb : FB;
        END_VAR
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%main_interface = type { %FB_interface }
%FB_interface = type { i16, i16, i1, i1, float, float }

@main_instance = global %main_interface { %FB_interface { i32 7, i16 0, i1 true, i1 false, float 0x400921CAC0000000, float 0.000000e+00 } }

define void @FB(%FB_interface* %0) {
entry:
  %x = getelementptr inbounds %FB_interface, %FB_interface* %0, i32 0, i32 0
  %xx = getelementptr inbounds %FB_interface, %FB_interface* %0, i32 0, i32 1
  %y = getelementptr inbounds %FB_interface, %FB_interface* %0, i32 0, i32 2
  %yy = getelementptr inbounds %FB_interface, %FB_interface* %0, i32 0, i32 3
  %z = getelementptr inbounds %FB_interface, %FB_interface* %0, i32 0, i32 4
  %zz = getelementptr inbounds %FB_interface, %FB_interface* %0, i32 0, i32 5
  ret void
}

define void @main(%main_interface* %0) {
entry:
  %fb = getelementptr inbounds %main_interface, %main_interface* %0, i32 0, i32 0
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn initial_values_in_struct_types() {
  let result = codegen!(
        "
        TYPE MyStruct:
        STRUCT
          x : INT := 7;
          xx : INT;
          y : BOOL := TRUE;
          yy : BOOL;
          z : REAL := 3.1415;
          zz : REAL;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL x : MyStruct; END_VAR
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%MyStruct = type { i16, i16, i1, i1, float, float }

@x = global %MyStruct { i32 7, i16 0, i1 true, i1 false, float 0x400921CAC0000000, float 0.000000e+00 }
"#;

  assert_eq!(result, expected);
}

#[test]
fn initial_values_in_type_alias() {
  let result = codegen!(
        "
        TYPE MyInt: INT := 7; END_TYPE 
        VAR_GLOBAL x : MyInt; END_VAR
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

@x = global i16 7
"#;

  assert_eq!(result, expected);
}