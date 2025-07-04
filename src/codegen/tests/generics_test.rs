use crate::test_utils::tests::codegen;
use plc_util::filtered_assert_snapshot;
#[test]
fn generic_function_has_no_declaration() {
    let prg = codegen(
        r"
        FUNCTION MAX<T : ANY_NUM> : T VAR_INPUT in1, in2 : T END_VAR END_FUNCTION
        ",
    );

    filtered_assert_snapshot!(prg);
}

#[test]
fn generic_function_call_generates_real_type_call() {
    let prg = codegen(
        r"
        @EXTERNAL FUNCTION MAX<T : ANY_NUM> : T VAR_INPUT in1, in2 : T END_VAR END_FUNCTION
        FUNCTION MAX__DINT : DINT VAR_INPUT in1, in2 : DINT END_VAR END_FUNCTION

        PROGRAM prg
        VAR
            a, b : INT;
        END_VAR

        MAX(1,2);
        MAX(a,b);
        END_PROGRAM
        ",
    );

    filtered_assert_snapshot!(prg);
}

#[test]
fn generic_codegen_with_aggregate_return() {
    let prg = codegen(
        r"
    FUNCTION main : STRING
    VAR_TEMP
        l : DINT;
        p : DINT;
    END_VAR
        l := 4;
        p := 6;
        main := MID(
            '     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.the                same           is   true                    for             this                     string.',
            l,
            p
        );
    END_FUNCTION

    FUNCTION MID < T: ANY_STRING >: T
    VAR_INPUT {ref}
        IN: T;
    END_VAR
    VAR_INPUT
        L: DINT;
        P: DINT;
    END_VAR
    END_FUNCTION

    {external}
    FUNCTION MID__STRING : STRING
    VAR_INPUT {ref}
        IN: STRING;
    END_VAR
    VAR_INPUT
        L: DINT;
        P: DINT;
    END_VAR
    END_FUNCTION

        ",
    );
    filtered_assert_snapshot!(prg);
}

#[test]
fn generic_output_parameter() {
    // GIVEN ... (see comments in st-code)
    let src = r"
        // ... a generic function FOO with a T, defined by a VAR_OUT
        // parameter (which will be interally treated as a pointer)
            FUNCTION foo <T: ANY_INT> : T
                VAR_INPUT   in1 : DATE; END_VAR
                VAR_OUTPUT  out1: T;    END_VAR
            END_FUNCTION

        // ...AND an implementatino for INT
            FUNCTION foo__INT : INT
                VAR_INPUT   in1 : DATE; END_VAR
                VAR_OUTPUT  out1: INT;  END_VAR
            END_FUNCTION

        // ... AND a program calling foo with an INT-parameter
            PROGRAM prg
            VAR
                theInt, iResult : INT;
                data : DATE;
            END_VAR

            iResult := foo(data, theInt);
            END_PROGRAM
        ";

    // THEN we expect a first call to foo__INT with out1 passed as a pointer
    filtered_assert_snapshot!(codegen(src));
}

#[test]
fn generic_call_gets_cast_to_biggest_type() {
    let src = r"

    {external}
    FUNCTION MAX<T : ANY> : T
        VAR_INPUT
            args : {sized} T...;
        END_VAR
    END_FUNCTION

 FUNCTION main : LREAL
    main := MAX(SINT#5,DINT#1,LREAL#1.5,1.2);
    END_FUNCTION";

    //Expecting all values to be LREAL
    filtered_assert_snapshot!(codegen(src));
}

#[test]
fn any_real_function_called_with_ints() {
    let src = r"
        FUNCTION foo <T: ANY_REAL> : T
            VAR_INPUT   in1 : T; END_VAR
        END_FUNCTION

        FUNCTION foo__REAL : REAL
            VAR_INPUT   in1 : REAL; END_VAR
        END_FUNCTION

        PROGRAM prg
        VAR
            res_sint : REAL;
            res_int : REAL;
            res_dint : REAL;
            res_lint : LREAL;
            res_usint : REAL;
            res_uint : REAL;
            res_udint : REAL;
            res_ulint : LREAL;
        END_VAR
        VAR_TEMP
            v_dint : DINT := 1;
            v_udint : DINT := 1;
        END_VAR
            res_sint := foo(SINT#1);
            res_int := foo(INT#1);
            res_dint := foo(v_dint);
            res_lint := foo(LINT#1);
            res_usint := foo(USINT#1);
            res_uint := foo(UINT#1);
            res_udint := foo(v_udint);
            res_ulint := foo(ULINT#1);
        END_PROGRAM";
    //Expecting to REAL/LREAL conversion for every call
    filtered_assert_snapshot!(codegen(src));
}

#[test]
fn generic_function_with_aggregate_return() {
    let src = r#"
    FUNCTION TO_STRING <T: ANY_STRING> : STRING[1024]
        VAR_INPUT {ref}
            in : T;
        END_VAR
    END_FUNCTION

    {external}
    FUNCTION TO_STRING__WSTRING : STRING[1024]
        VAR_INPUT {ref}
            in : WSTRING;
        END_VAR
    END_FUNCTION

    FUNCTION main
        TO_STRING(WSTRING#"Hello");
    END_FUNCTION

    "#;
    filtered_assert_snapshot!(codegen(src));
}
