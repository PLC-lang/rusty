use crate::lexer;
use crate::parser;
use inkwell::context::Context;
use pretty_assertions::assert_eq;

macro_rules! codegen {
    ($code:tt) => {{
        let lexer = lexer::lex($code);
        let ast = parser::parse(lexer).unwrap();

        let context = Context::create();
        let mut code_generator = super::CodeGen::new(&context);
        code_generator.generate(ast)
    }};
}

macro_rules! generate_boiler_plate {
        ($pou_name:tt, $type:tt, $body:tt)  => (
            format!(
r#"; ModuleID = 'main'
source_filename = "main"

%prg_interface = type {{{type}}}

@prg_instance = common global %prg_interface zeroinitializer

define void @{pou_name}() {{
entry:
{body}}}
"#,
            pou_name = $pou_name, type = $type, body = $body)
        )
    }

#[test]
fn empty_program_with_name_generates_void_function() {
    let result = codegen!("PROGRAM prg END_PROGRAM");
    let expected = generate_boiler_plate!("prg", "", 
    r#"  ret void
"#);

    assert_eq!(result, expected);
}

#[test]
fn program_with_variables_generates_void_function_and_struct() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
x : INT;
y : INT;
END_VAR
END_PROGRAM
"#
    );
    let expected = generate_boiler_plate!("prg", " i32, i32 ",
    r#"  ret void
"#);

    assert_eq!(result, expected);
}

#[test]
fn program_with_variables_and_references_generates_void_function_and_struct_and_body() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
x : INT;
y : INT;
END_VAR
x;
y;
END_PROGRAM
"#
    );
    let expected = generate_boiler_plate!(
        "prg",
        " i32, i32 ",
        r#"  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %load_y = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  ret void
"#
    );

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
    let expected = generate_boiler_plate!(
        "prg",
        " i1, i1 ",
        r#"  %load_x = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %load_y = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
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
x : INT;
y : INT;
END_VAR
x + y;
END_PROGRAM
"#
    );
    let expected = generate_boiler_plate!(
        "prg",
        " i32, i32 ",
        r#"  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %load_y = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
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
x : INT;
END_VAR
x + 7;
END_PROGRAM
"#
    );
    let expected = generate_boiler_plate!(
        "prg",
        " i32 ",
        r#"  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
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
y : INT;
END_VAR
y := 7;
END_PROGRAM
"#
    );
    let expected = generate_boiler_plate!(
        "prg",
        " i32 ",
        r#"  store i32 7, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
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
x : INT;
y : INT;
END_VAR
y := x + 1;
y := x - 2;
y := x * 3;
y := x / 4;
y := x MOD 5;
END_PROGRAM
"#
    );
    let expected = generate_boiler_plate!(
        "prg",
        " i32, i32 ",
        r#"  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar = add i32 %load_x, 1
  store i32 %tmpVar, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x1 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar2 = sub i32 %load_x1, 2
  store i32 %tmpVar2, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x3 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar4 = mul i32 %load_x3, 3
  store i32 %tmpVar4, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x5 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar6 = sdiv i32 %load_x5, 4
  store i32 %tmpVar6, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x7 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar8 = srem i32 %load_x7, 5
  store i32 %tmpVar8, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
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
x : INT;
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
    let expected = generate_boiler_plate!(
        "prg",
        " i32, i1 ",
        r#"  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar = icmp eq i32 %load_x, 1
  store i1 %tmpVar, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x1 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar2 = icmp sgt i32 %load_x1, 2
  store i1 %tmpVar2, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x3 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar4 = icmp slt i32 %load_x3, 3
  store i1 %tmpVar4, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x5 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar6 = icmp ne i32 %load_x5, 4
  store i1 %tmpVar6, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x7 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar8 = icmp sge i32 %load_x7, 5
  store i1 %tmpVar8, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x9 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar10 = icmp sle i32 %load_x9, 6
  store i1 %tmpVar10, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  ret void
"#
    );

    assert_eq!(result, expected);
}

#[test]
fn program_with_variable_and_boolean_expressions_generates_void_function_and_struct_and_body() {
    let result = codegen!(
        r#"PROGRAM prg
VAR
x : BOOL;
y : BOOL;
z : INT;
END_VAR
x AND y;
x OR y;
x XOR y;
END_PROGRAM
"#
    );
    let expected = generate_boiler_plate!(
        "prg",
        " i1, i1, i32 ",
        r#"  %load_x = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %load_y = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %tmpVar = and i1 %load_x, %load_y
  %load_x1 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %load_y2 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %tmpVar3 = or i1 %load_x1, %load_y2
  %load_x4 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %load_y5 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %tmpVar6 = xor i1 %load_x4, %load_y5
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
    let expected = generate_boiler_plate!(
        "prg",
        " i1, i1 ",
        r#"  %load_x = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar = xor i1 %load_x, true
  %load_x1 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %load_y = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %tmpVar2 = xor i1 %load_y, true
  %tmpVar3 = and i1 %load_x1, %tmpVar2
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
z : INT;
y : BOOL;
END_VAR
y AND z >= 5;
NOT (z <= 6) OR y;
END_PROGRAM
"#
    );
    let expected = generate_boiler_plate!(
        "prg",
        " i32, i1 ",
        r#"  %load_y = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_z = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar = icmp sge i32 %load_z, 5
  %tmpVar1 = and i1 %load_y, %tmpVar
  %load_z2 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar3 = icmp sle i32 %load_z2, 6
  %tmpVar4 = xor i1 %tmpVar3, true
  %load_y5 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %tmpVar6 = or i1 %tmpVar4, %load_y5
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
            z : INT;
            y : INT;
            END_VAR
            -1 + z;
            2 +-z;
            -y + 3;
            END_PROGRAM
            "#
    );
    let expected = generate_boiler_plate!(
        "prg",
        " i32, i32 ",
        r#"  %load_z = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar = add i32 -1, %load_z
  %load_z1 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar2 = sub i32 0, %load_z1
  %tmpVar3 = add i32 2, %tmpVar2
  %load_y = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
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
            x : INT;
            y : INT;
            z : INT;
            u : INT;
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
    let expected = generate_boiler_plate!("prg"," i32, i32, i32, i32, i1, i1, i1 ", 
r#"  %load_b1 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 4)
  br i1 %load_b1, label %condition_body, label %branch

condition_body:                                   ; preds = %entry
  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  br label %continue

branch:                                           ; preds = %entry
  %load_b2 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 5)
  br i1 %load_b2, label %condition_body2, label %branch1

condition_body2:                                  ; preds = %branch
  %load_y = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  br label %continue

branch1:                                          ; preds = %branch
  %load_b3 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 6)
  br i1 %load_b3, label %condition_body3, label %else

condition_body3:                                  ; preds = %branch1
  %load_z = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 2)
  br label %continue

else:                                             ; preds = %branch1
  %load_u = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 3)
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
            x : INT;
            b1 : BOOL;
        END_VAR
        IF b1 THEN
            x;
        END_IF
        END_PROGRAM
        "
    );
    let expected = generate_boiler_plate!("prg"," i32, i1 ", 
r#"  %load_b1 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  br i1 %load_b1, label %condition_body, label %continue

condition_body:                                   ; preds = %entry
  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
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
            x : INT;
            b1 : BOOL;
        END_VAR
        IF (x > 1) OR b1 THEN
            x;
        END_IF
        END_PROGRAM
        "
    );
    let expected = generate_boiler_plate!("prg"," i32, i1 ", 
r#"  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar = icmp sgt i32 %load_x, 1
  %load_b1 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %tmpVar1 = or i1 %tmpVar, %load_b1
  br i1 %tmpVar1, label %condition_body, label %continue

condition_body:                                   ; preds = %entry
  %load_x2 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  br label %continue

continue:                                         ; preds = %condition_body, %entry
  ret void
"#);

    assert_eq!(result, expected);
}

#[test]
fn for_statement_with_steps_test() {
    let result = codegen!(
        "
        PROGRAM prg 
        VAR
            x : INT;
        END_VAR
        FOR x := 3 TO 10 BY 7 DO 
            x;
        END_FOR
        END_PROGRAM
        "
    );

    let expected = generate_boiler_plate!("prg"," i32 ", 
r#"  store i32 3, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)

condition_check:                                  ; preds = %for_body
  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar = icmp sle i32 %load_x, 10
  br i1 %tmpVar, label %for_body, label %continue

for_body:                                         ; preds = %condition_check
  %load_x1 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar2 = add i32 %load_x, 7
  store i32 %tmpVar2, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
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
            x : INT;
        END_VAR
        FOR x := 3 TO 10 DO 
            x;
        END_FOR
        END_PROGRAM
        "
    );

    let expected = generate_boiler_plate!("prg"," i32 ", 
r#"  store i32 3, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)

condition_check:                                  ; preds = %for_body
  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar = icmp sle i32 %load_x, 10
  br i1 %tmpVar, label %for_body, label %continue

for_body:                                         ; preds = %condition_check
  %load_x1 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar2 = add i32 %load_x, 1
  store i32 %tmpVar2, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
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
            x : INT;
        END_VAR
        FOR x := 3 TO 10 DO 
        END_FOR
        x;
        END_PROGRAM
        "
    );

    let expected = generate_boiler_plate!("prg"," i32 ", 
r#"  store i32 3, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)

condition_check:                                  ; preds = %for_body
  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar = icmp sle i32 %load_x, 10
  br i1 %tmpVar, label %for_body, label %continue

for_body:                                         ; preds = %condition_check
  %tmpVar1 = add i32 %load_x, 1
  store i32 %tmpVar1, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  br label %condition_check

continue:                                         ; preds = %condition_check
  %load_x2 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
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
            step: INT;
            x : INT;
            y : INT;
            z : INT;
        END_VAR
        FOR x := y TO z BY step DO 
            x;
        END_FOR
        END_PROGRAM
        "
    );

    let expected = generate_boiler_plate!("prg"," i32, i32, i32, i32 ", 
r#"  %load_y = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 2)
  store i32 %load_y, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)

condition_check:                                  ; preds = %for_body
  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_z = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 3)
  %tmpVar = icmp sle i32 %load_x, %load_z
  br i1 %tmpVar, label %for_body, label %continue

for_body:                                         ; preds = %condition_check
  %load_x1 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_step = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar2 = add i32 %load_x, %load_step
  store i32 %tmpVar2, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
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

    let expected = generate_boiler_plate!("prg"," i1 ", 
r#"
condition_check:                                  ; preds = %while_body
  %load_x = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  br i1 %load_x, label %while_body, label %continue

while_body:                                       ; preds = %condition_check
  %load_x1 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
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

    let expected = generate_boiler_plate!("prg"," i1 ", 
r#"
condition_check:                                  ; preds = %while_body
  %load_x = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar = icmp eq i1 %load_x, i32 0
  br i1 %tmpVar, label %while_body, label %continue

while_body:                                       ; preds = %condition_check
  %load_x1 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
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

    let expected = generate_boiler_plate!("prg"," i1 ", 
r#"  br label %while_body

condition_check:                                  ; preds = %while_body
  %load_x = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  br i1 %load_x, label %while_body, label %continue

while_body:                                       ; preds = %entry, %condition_check
  %load_x1 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
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
            x : INT;
            y : INT;
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

    let expected = generate_boiler_plate!("prg"," i32, i32 ", 
r#"  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  switch i32 %load_x, label %else [
    i32 1, label %case
    i32 2, label %case1
    i32 3, label %case2
  ]

case:                                             ; preds = %entry
  store i32 1, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  br label %continue

case1:                                            ; preds = %entry
  store i32 2, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  br label %continue

case2:                                            ; preds = %entry
  store i32 3, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  br label %continue

else:                                             ; preds = %entry
  store i32 -1, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  br label %continue

continue:                                         ; preds = %else, %case2, %case1, %case
  ret void
"#);

    assert_eq!(result, expected);
}