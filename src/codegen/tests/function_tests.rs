use crate::test_utils::tests::codegen;

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

    insta::assert_snapshot!(result, @"");
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

    insta::assert_snapshot!(result, @"");
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

    insta::assert_snapshot!(result, @"");
}
