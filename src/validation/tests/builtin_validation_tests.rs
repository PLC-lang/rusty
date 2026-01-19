use crate::test_utils::tests::parse_and_validate_buffered;
use insta::assert_snapshot;

#[test]
fn arithmetic_builtins_allow_mixing_of_fp_and_int_params() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : LINT
        VAR
            i1, i2 : DINT;
            f1, f2 : LREAL;
            res_i : DINT;
            res_fp: LREAL;
        END_VAR
            res_i := ADD(i1, i2, f1, f2);
            res_fp := MUL(i1, i2, f1, f2);
            res_i := SUB(i1, f2);
            res_fp := DIV(i1, f2);
        END_FUNCTION
       ",
    );
    assert_snapshot!(diagnostics);
}

#[test]
#[ignore = "FIXME: no validation for incompatible types for arithmetic operations"]
fn arithmetic_builtins_called_with_incompatible_types() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
        VAR
            x1 : ARRAY[0..2] OF TOD;
            x2 : STRING;
        END_VAR
            x1 + x2; // will currently also validate without errors
            ADD(x1, x1);
            DIV(x1, x2);
            SUB(x2, x2);
        END_FUNCTION
       ",
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn arithmetic_builtins_called_with_invalid_param_count() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
        VAR
            x1 : DINT;
            x2 : REAL;
        END_VAR
            ADD();
            MUL(x1);
            DIV(x2, x2, x1, x2); // DIV and SUB are not extensible
            SUB(x2, x2, x1, x2);
        END_FUNCTION
       ",
    );

    assert_snapshot!(&diagnostics);
}

#[test]
#[ignore = "FIXME: no validation for incompatible type comparisons"]
fn comparison_builtins_called_with_incompatible_types() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
        VAR
            x1 : ARRAY[0..2] OF TOD;
            x2 : STRING;
        END_VAR
            x1 > x2;
            EQ(x1, x1);
            GT(x1, x2);
            NE(x2, x2);
        END_FUNCTION
       ",
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn comparison_builtins_called_with_invalid_param_count() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
        VAR
            x1 : DINT;
            x2 : REAL;
        END_VAR
            EQ();
            GT(x1);
            LE(x2, x2, x1, x2); // OK
            NE(x2, x2, x1, x2); // NE is not extensible
        END_FUNCTION
       ",
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn shl_must_validate_types() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main
        VAR
        END_VAR
            SHL('foo',2);
        END_FUNCTION
       ",
    );

    assert_snapshot!(&diagnostics);
}

#[test]
fn shr_must_validate_types() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main
        VAR
        END_VAR
            SHR('foo',2);
        END_FUNCTION
       ",
    );

    assert_snapshot!(&diagnostics);
}
