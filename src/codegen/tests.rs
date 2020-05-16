use crate::lexer;
use crate::parser;
use crate::index::Index;
use inkwell::context::Context;
use pretty_assertions::assert_eq;

macro_rules! codegen {
    ($code:tt) => {{
        let lexer = lexer::lex($code);
        let ast = parser::parse(lexer).unwrap();

        let context = Context::create();
        let mut index = Index::new();
        index.visit(&ast);
        let mut code_generator = super::CodeGen::new(&context, &mut index);
        code_generator.generate(ast)
    }};
}

macro_rules! generate_with_empty_program {
  ($code:tt) => (
    {
      let source = format!("{} {}", "PROGRAM main END_PROGRAM", $code);
      let str_source = source.as_str();
      codegen!(str_source)
    }
  )
}


fn generate_program_boiler_plate(pou_name : &str, type_list : &[(&str,&str)], return_type : &str, thread_mode : &str, global_variables : &str, body : &str) -> String{

  let mut interface : String = type_list.iter().map(|(t,_)| *t).collect::<Vec<&str>>().join(", ");
  if !interface.is_empty() { 
    interface = format!(" {} ",interface);
  }


  let mut type_references : Vec<String>= vec![];

  for (i,t)  in type_list.iter().enumerate() {
    let type_def = format!("  %{var_name} = getelementptr inbounds %{pou_name}_interface, %{pou_name}_interface* %0, i32 0, i32 {index}",
    var_name = t.1,
    index = i,
    pou_name = pou_name,
  );
    type_references.push(type_def);
  }

  if return_type != "void" {
    type_references.push(format!("  %{pou_name} = alloca {return_type}",pou_name = pou_name,return_type = return_type))
  }

  if !type_references.is_empty() {
    type_references.push("  ".to_string());
  }

format!(
r#"; ModuleID = 'main'
source_filename = "main"

%{pou_name}_interface = type {{{type}}}
{global_variables}
@{pou_name}_instance = common {thread_mode}global %{pou_name}_interface zeroinitializer

define {return_type} @{pou_name}(%{pou_name}_interface* %0) {{
entry:
{type_references}{body}}}
"#,
                pou_name = pou_name, 
                type = interface, 
                return_type = return_type, 
                thread_mode = thread_mode,  
                type_references = type_references.join("\n"),
                body = body,
                global_variables = global_variables
                )
}

fn generate_program_boiler_plate_globals(global_variables : &str) -> String {
  generate_program_boiler_plate("main", &[], "void", "", global_variables, "  ret void\n", )
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
@gX = common global i32 0
@gY = common global i1 false"#);

    assert_eq!(result, expected);
}

#[test]
fn two_global_variables_generates_in_separate_global_variables() {
    let result = generate_with_empty_program!("VAR_GLOBAL gX : INT; gY : BOOL; END_VAR VAR_GLOBAL gA : INT; END_VAR");
    let expected = generate_program_boiler_plate_globals(
r#"
@gX = common global i32 0
@gY = common global i1 false
@gA = common global i32 0"#);

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

    let expected = generate_program_boiler_plate("prg", &[("i32","x")], "void", "", 
r"
@gX = common global i32 0", //global vars
r"store i32 20, i32* @gX
  %load_gX = load i32, i32* @gX
  store i32 %load_gX, i32* %x
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

define i32 @foo(%foo_interface* %0) {
entry:
  %foo = alloca i32
  %foo_ret = load i32, i32* %foo
  ret i32 %foo_ret
}
"#;
 
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
x : INT;
y : INT;
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
x : INT;
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
y : INT;
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
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i1","x"),("i1","y"), ("i32","z")],
        "void",
        "",
        "",
        r#"%load_x = load i1, i1* %x
  %load_y = load i1, i1* %y
  %tmpVar = and i1 %load_x, %load_y
  %load_x1 = load i1, i1* %x
  %load_y2 = load i1, i1* %y
  %tmpVar3 = or i1 %load_x1, %load_y2
  %load_x4 = load i1, i1* %x
  %load_y5 = load i1, i1* %y
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
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i1","x"),("i1","y")],
        "void",
        "",
        "",
        r#"%load_x = load i1, i1* %x
  %tmpVar = xor i1 %load_x, true
  %load_x1 = load i1, i1* %x
  %load_y = load i1, i1* %y
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
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i32","z"),("i1","y")],
        "void",
        "",
        "",
        r#"%load_y = load i1, i1* %y
  %load_z = load i32, i32* %z
  %tmpVar = icmp sge i32 %load_z, 5
  %tmpVar1 = and i1 %load_y, %tmpVar
  %load_z2 = load i32, i32* %z
  %tmpVar3 = icmp sle i32 %load_z2, 6
  %tmpVar4 = xor i1 %tmpVar3, true
  %load_y5 = load i1, i1* %y
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
            x : INT;
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
            x : INT;
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
  %load_b1 = load i1, i1* %b1
  %tmpVar1 = or i1 %tmpVar, %load_b1
  br i1 %tmpVar1, label %condition_body, label %continue

condition_body:                                   ; preds = %entry
  %load_x2 = load i32, i32* %x
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
            x : INT;
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
            x : INT;
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
  %tmpVar = icmp eq i1 %load_x, i32 0
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
        FUNCTION foo : INT
        foo := 1;
        END_FUNCTION

        PROGRAM prg 
        VAR
            x : INT;
        END_VAR
        x := foo();
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%prg_interface = type { i32 }
%foo_interface = type {}

@prg_instance = common global %prg_interface zeroinitializer

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
  %call = call i32 @foo(%foo_interface* %foo_instance)
  store i32 %call, i32* %x
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn function_with_parameters_called_in_program() {
    let result = codegen!(
        "
        FUNCTION foo : INT
        VAR_INPUT
          bar : INT;
        END_VAR
        foo := 1;
        END_FUNCTION

        PROGRAM prg 
        VAR
        x : INT;
        END_VAR
        x := foo(2);
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%prg_interface = type { i32 }
%foo_interface = type { i32 }

@prg_instance = common global %prg_interface zeroinitializer

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
  %1 = getelementptr inbounds %foo_interface, %foo_interface* %foo_instance, i32 0, i32 0
  store i32 2, i32* %1
  %call = call i32 @foo(%foo_interface* %foo_instance)
  store i32 %call, i32* %x
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn function_with_two_parameters_called_in_program() {
    let result = codegen!(
        "
        FUNCTION foo : INT
        VAR_INPUT
          bar : INT;
          buz : BOOL;
        END_VAR
        foo := 1;
        END_FUNCTION

        PROGRAM prg 
        VAR
        x : INT;
        END_VAR
        x := foo(2, TRUE);
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%prg_interface = type { i32 }
%foo_interface = type { i32, i1 }

@prg_instance = common global %prg_interface zeroinitializer

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
  %1 = getelementptr inbounds %foo_interface, %foo_interface* %foo_instance, i32 0, i32 0
  store i32 2, i32* %1
  %2 = getelementptr inbounds %foo_interface, %foo_interface* %foo_instance, i32 0, i32 1
  store i1 true, i1* %2
  %call = call i32 @foo(%foo_interface* %foo_instance)
  store i32 %call, i32* %x
  ret void
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

@foo_instance = common global %foo_interface zeroinitializer
@prg_instance = common global %prg_interface zeroinitializer

define void @foo(%foo_interface* %0) {
entry:
  ret void
}

define void @prg(%prg_interface* %0) {
entry:
  call void @foo(%foo_interface* @foo_instance)
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
          bar : INT;
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

@foo_instance = common global %foo_interface zeroinitializer
@prg_instance = common global %prg_interface zeroinitializer

define void @foo(%foo_interface* %0) {
entry:
  %bar = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 0
  %buz = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 1
  ret void
}

define void @prg(%prg_interface* %0) {
entry:
  store i32 2, i32* getelementptr inbounds (%foo_interface, %foo_interface* @foo_instance, i32 0, i32 0)
  store i1 true, i1* getelementptr inbounds (%foo_interface, %foo_interface* @foo_instance, i32 0, i32 1)
  call void @foo(%foo_interface* @foo_instance)
  ret void
}
"#;

  assert_eq!(result, expected);
}

#[test]
fn function_called_when_shadowed() {
  let result = codegen!(
        "
        FUNCTION foo : INT
        foo := 1;
        END_FUNCTION

        PROGRAM prg 
        VAR
            foo : INT;
        END_VAR
        foo := foo();
        END_PROGRAM
        "
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%prg_interface = type { i32 }
%foo_interface = type {}

@prg_instance = common global %prg_interface zeroinitializer

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
  %call = call i32 @foo(%foo_interface* %foo_instance)
  store i32 %call, i32* %foo
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

@prg_instance = common global %prg_interface zeroinitializer

define void @foo(%foo_interface* %0) {
entry:
  ret void
}

define void @prg(%prg_interface* %0) {
entry:
  %fb_inst = getelementptr inbounds %prg_interface, %prg_interface* %0, i32 0, i32 0
  call void @foo(%foo_interface* %fb_inst)
  ret void
}
"#;

  assert_eq!(result, expected);
}
