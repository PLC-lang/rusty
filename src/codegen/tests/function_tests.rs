use crate::test_utils::tests::codegen;

#[test]
fn var_output_in_function_call() {
    let result = codegen(
        r#"FUNCTION func : DINT
            VAR_OUTPUT  o   : INT;      END_VAR
            o := 6;
            func := 4;
        END_FUNCTION

        PROGRAM main
            VAR
                x : INT := 4;
            END_VAR

            func(o => x);
        END_PROGRAM
        "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
#[ignore = "duplicate"]
fn on_functions_var_in_out_should_be_passed_as_a_pointer() {
    let result = codegen(
        r#"
        FUNCTION bump : DINT
            VAR_IN_OUT  v  : SINT;      END_VAR
            bump := v;
            v := 7;
        END_FUNCTION
        "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn on_functions_var_output_should_be_passed_as_a_pointer() {
    let result = codegen(
        r#"
        FUNCTION bump : DINT
            VAR_OUTPUT  v  : SINT;      END_VAR
            bump := 1;
            v := 2;
        END_FUNCTION
        "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn member_variables_in_body() {
    let result = codegen(
        r#"FUNCTION func : DINT
            VAR_INPUT   i   : INT := 6 END_VAR
            VAR_IN_OUT  io  : SINT;      END_VAR
            VAR_OUTPUT  o   : LINT;      END_VAR
            VAR         v   : INT := 1; END_VAR
            VAR_TEMP    vt  : INT := 2; END_VAR
            
            func := i * io - o + v * vt;
        END_FUNCTION
        "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn simple_call() {
    let result = codegen(
        r#"FUNCTION func : DINT
            VAR_INPUT x : DINT; END_VAR
        END_FUNCTION

        PROGRAM main
            VAR a : DINT; END_VAR

            func(a);
            func(1);
            func(1+a);
        END_PROGRAM
        "#,
    );

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %main_interface = type { i32 }

    @main_instance = global %main_interface zeroinitializer

    define i32 @func(i32 %0) {
    entry:
      %x = alloca i32, align 4
      store i32 %0, i32* %x, align 4
      %func = alloca i32, align 4
      store i32 0, i32* %func, align 4
      %func_ret = load i32, i32* %func, align 4
      ret i32 %func_ret
    }

    define void @main(%main_interface* %0) {
    entry:
      %a = getelementptr inbounds %main_interface, %main_interface* %0, i32 0, i32 0
      %load_a = load i32, i32* %a, align 4
      %call = call i32 @func(i32 %load_a)
      %call1 = call i32 @func(i32 1)
      %load_a2 = load i32, i32* %a, align 4
      %tmpVar = add i32 1, %load_a2
      %call3 = call i32 @func(i32 %tmpVar)
      ret void
    }
    "###);
}

#[test]
fn passing_a_string_to_a_function() {
    let result = codegen(
        r#"FUNCTION func : DINT
            VAR_INPUT x : STRING[5]; END_VAR
        END_FUNCTION

        PROGRAM main
            VAR a : STRING[5]; END_VAR

            func(a);
            func('12345');
        END_PROGRAM
        "#,
    );

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %main_interface = type { [6 x i8] }

    @main_instance = global %main_interface zeroinitializer
    @utf08_literal_0 = unnamed_addr constant [6 x i8] c"12345\00"

    define i32 @func([6 x i8] %0) {
    entry:
      %x = alloca [6 x i8], align 1
      store [6 x i8] %0, [6 x i8]* %x, align 1
      %func = alloca i32, align 4
      store i32 0, i32* %func, align 4
      %func_ret = load i32, i32* %func, align 4
      ret i32 %func_ret
    }

    define void @main(%main_interface* %0) {
    entry:
      %a = getelementptr inbounds %main_interface, %main_interface* %0, i32 0, i32 0
      %load_a = load [6 x i8], [6 x i8]* %a, align 1
      %call = call i32 @func([6 x i8] %load_a)
      %1 = load [6 x i8], [6 x i8]* @utf08_literal_0, align 1
      %call1 = call i32 @func([6 x i8] %1)
      ret void
    }
    "###);
}
