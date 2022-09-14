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

    insta::assert_snapshot!(result);
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

    insta::assert_snapshot!(result);
}

#[test]
fn passing_a_string_to_a_function_as_reference() {
    let result = codegen(
        r#"FUNCTION func : DINT
            VAR_INPUT {ref} x : STRING[5]; END_VAR
        END_FUNCTION

        PROGRAM main
            VAR a : STRING[5]; END_VAR

            func(a);
            func('12345');
        END_PROGRAM
        "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn passing_arguments_to_functions_by_ref_and_val() {
    let result = codegen(
        r#"FUNCTION func : DINT
        VAR_INPUT {ref}
            byRef1 : INT;
            byRef2 : DINT;
        END_VAR
        VAR_INPUT
            byVal1 : INT;
            byVal2 : DINT;
        END_VAR
            func := byRef1 * byRef2 * byVal1 * byRef2;
        END_FUNCTION

        PROGRAM main
            func(1,2,3,4); //1 and 2 by ref, 3 and 4 by val
        END_PROGRAM
        "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn function_with_varargs_called_in_program() {
    let result = codegen(
        "
        @EXTERNAL
        FUNCTION foo : DINT
        VAR_INPUT
          args : ...;
        END_VAR
        END_FUNCTION

        PROGRAM prg 
        VAR
        x : DINT;
        END_VAR
        x := foo(FALSE, 3, (x + 1));
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn function_with_sized_varargs_called_in_program() {
    let result = codegen(
        "
        @EXTERNAL
        FUNCTION foo : DINT
        VAR_INPUT
          args : {sized} DINT...;
        END_VAR
        END_FUNCTION

        PROGRAM prg 
        VAR
        x : DINT;
        END_VAR
        x := foo(0, 3, (x + 1));
        END_PROGRAM
        ",
    );

    // The function definition contains a size and pointer for the parameters
    // The parameters are stored in a local vector (allocated in place)
    // Function call with 3 as first parameter (size) and the arguments array as pointer
    insta::assert_snapshot!(result);
}


#[test]
fn function_with_ref_sized_string_varargs_called_in_program() {
    let result = codegen(
        "
        {external}
        FUNCTION foo : DINT
        VAR_INPUT {ref}
          args : {sized} STRING...;
        END_VAR
        END_FUNCTION

        PROGRAM prg 
        VAR
        x : DINT;
        END_VAR
        x := foo('a', 'abc', 'abcdef');
        END_PROGRAM
        ",
    );

    // The function definition contains a size and pointer for the parameters
    // The parameters are stored in a local vector (allocated in place)
    // Function call with 3 as first parameter (size) and the arguments array as pointer
    insta::assert_snapshot!(result);


}
