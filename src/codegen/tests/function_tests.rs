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
            func(1, 2, 3, 4); // 1 and 2 by ref, 3 and 4 by val
        END_PROGRAM
        "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn autocast_argument_literals_for_function_call() {
    let result = codegen(
        r#"FUNCTION func : DINT
        VAR_INPUT {ref}
            byInt1 : SINT;
            byInt2 : INT;
            byInt3 : DINT;
            byInt4 : LINT;
            byReal1 : REAL;
            byReal2 : LREAL;
        END_VAR
            func := 1;
        END_FUNCTION
        PROGRAM main
            // Check if up- and downcasting works; the IR should not need additional instructions other
            // than a `alloca` and `store` instruction for their actual types, i.e. no casting needed
            func(DINT#1, DINT#2, SINT#3, DINT#4, LREAL#5.0, REAL#6.0);
        END_PROGRAM
        "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn bitcast_argument_references_for_function_call() {
    let result = codegen(
        r#"
        FUNCTION fn_sint : SINT
            VAR_INPUT {ref}
                in_ref : SINT;
            END_VAR
            VAR_IN_OUT
                in_out : SINT;
            END_VAR
        END_FUNCTION
        FUNCTION fn_lint : LINT
            VAR_INPUT {ref}
                in_ref : LINT;
            END_VAR
            VAR_IN_OUT
                in_out : LINT;
            END_VAR
        END_FUNCTION

        FUNCTION fn_real : LINT
            VAR_INPUT {ref}
                in_ref : REAL;
            END_VAR
            VAR_IN_OUT
                in_out : REAL;
            END_VAR
        END_FUNCTION

        FUNCTION fn_lreal : LINT
            VAR_INPUT {ref}
                in_ref : LREAL;
            END_VAR
            VAR_IN_OUT
                in_out : LREAL;
            END_VAR
        END_FUNCTION

        PROGRAM main
            VAR
                var1_sint, var2_sint : SINT := 1;
                var1_int,  var2_int : INT := 2;
                var1_dint, var2_dint : DINT := 3;
                var1_lint, var2_lint : LINT := 4;
                var1_real, var2_real : REAL := 5.0;
                var1_lreal, var2_lreal : LREAL := 6.0;
            END_VAR

            // Check if up- and downcasting (= bitcasts) works for **integers** references
            fn_sint(var1_sint, var2_sint);
            fn_sint(var1_int,  var2_int);
            fn_sint(var1_dint, var2_dint);
            fn_sint(var1_lint, var2_lint);

            fn_lint(var1_sint, var2_sint);
            fn_lint(var1_int,  var2_int);
            fn_lint(var1_dint, var2_dint);
            fn_lint(var1_lint, var2_lint);

            // Check if up- and downcasting (= bitcasts) works for **float** references
            fn_real(var1_real,  var2_real);
            fn_real(var1_lreal, var2_lreal);
            fn_lreal(var1_real,  var2_real);
            fn_lreal(var1_lreal, var2_lreal);
        END_PROGRAM"#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn literal_string_argument_passed_by_ref() {
    let result = codegen(
        "
        @EXTERNAL
        FUNCTION func : STRING
            VAR_INPUT {ref}
                in : STRING;
            END_VAR
        END_FUNCTION

        PROGRAM main
            VAR
                res : STRING;
            END_VAR

            res := func('hello');
        END_PROGRAM
    ",
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

#[test]
fn return_variable_in_nested_call() {
    // GIVEN a call statement where we take the adr of the return-variable
    let src = "
        FUNCTION main : DINT
            VAR
                x1, x2 : DINT;
            END_VAR
            x1 := SMC_Read(
                        ValAddr := ADR(main));
        END_FUNCTION

        FUNCTION SMC_Read : DINT
            VAR_INPUT
                ValAddr : LWORD;
            END_VAR
        END_FUNCTION
          ";

    // we want a call passing the return-variable as apointer (actually the adress as a LWORD)
    insta::assert_snapshot!(codegen(src));
}

#[test]
fn argument_fed_by_ref_then_by_val() {
    let result = codegen(
        "
        TYPE MyType : ARRAY[1..5] OF DWORD; END_TYPE

        FUNCTION main : DINT
            VAR
                arr : MyType;
            END_VAR

            fn_by_ref(arr);
        END_FUNCTION

        FUNCTION fn_by_ref : DINT
            VAR_IN_OUT
                arg_by_ref : MyType;
            END_VAR

            fn_by_val(arg_by_ref);
        END_FUNCTION

        FUNCTION fn_by_val : DINT
            VAR_INPUT
                arg_by_val : MyType;
            END_VAR
        END_FUNCTION
    ",
    );

    insta::assert_snapshot!(result)
}

#[test]
fn properties_are_not_registered_as_variables_in_a_pou() {
    // When dealing with properties, we currently create an internal variable named after the property into
    // the POU during the lowering stage. We do this because of ease of implementation. However, technically
    // properties aren't variables hence we want to make sure the generated IR does not contain any property
    // variable member.
    let result = codegen(
        "
        FUNCTION_BLOCK fb
            VAR
                privateVariable : STRING;
            END_VAR

            // Internally the compiler will also create the following variable block:
            // VAR_PROPERTY
            //   foo : INT;
            //   bar : DINT;
            // END_VAR

            PROPERTY foo : INT
                GET END_GET
                SET END_SET
            END_PROPERTY

            PROPERTY bar : DINT
                GET END_GET
                SET END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
    ",
    );

    assert_eq!(result.lines().nth(3).unwrap(), "%fb = type { [81 x i8] }");
}
