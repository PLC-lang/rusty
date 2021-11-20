use crate::test_utils::tests::codegen;

#[test]
#[ignore]
fn generic_function_has_no_declaration() {
    let prg = codegen(
        r"
        @EXTERNAL FUNCTION MAX<T : ANY_NUMBER> : T VAR_INPUT in1, in2 : T END_VAR END_FUNCTION
        ",
    );

    insta::assert_snapshot!(prg);
}

#[test]
#[ignore]
fn generic_function_call_generates_real_type_call() {
    let prg = codegen(
        r"
        @EXTERNAL FUNCTION MAX<T : ANY_NUMBER> : T VAR_INPUT in1, in2 : T END_VAR END_FUNCTION

        PROGRAM prg 
        VAR
            a, b : INT;
        END_VAR

        MAX(1,2);
        MAX(a,b);
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(prg);
}
