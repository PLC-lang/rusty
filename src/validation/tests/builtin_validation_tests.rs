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

/// Mimics the generic ABS declaration the standard library includes provide
const ABS_DECLARATION: &str = "
        FUNCTION ABS<T: ANY_NUM> : T
        VAR_INPUT
            IN : T;
        END_VAR
        END_FUNCTION
";

#[test]
fn abs_on_unsigned_arguments_reports_a_warning() {
    let diagnostics = parse_and_validate_buffered(&format!(
        "{ABS_DECLARATION}
        FUNCTION main : DINT
        VAR
            a : USINT;
            b : UINT;
            c : UDINT;
            d : ULINT;
        END_VAR
            ABS(a);
            ABS(b);
            ABS(c);
            ABS(d);
            ABS(b - UINT#10);
        END_FUNCTION
       ",
    ));

    assert_snapshot!(diagnostics, @"
    warning[E150]: ABS on a value of unsigned type 'USINT' has no effect
       ┌─ <internal>:15:13
       │
    15 │             ABS(a);
       │             ^^^ ABS on a value of unsigned type 'USINT' has no effect

    warning[E150]: ABS on a value of unsigned type 'UINT' has no effect
       ┌─ <internal>:16:13
       │
    16 │             ABS(b);
       │             ^^^ ABS on a value of unsigned type 'UINT' has no effect

    warning[E150]: ABS on a value of unsigned type 'UDINT' has no effect
       ┌─ <internal>:17:13
       │
    17 │             ABS(c);
       │             ^^^ ABS on a value of unsigned type 'UDINT' has no effect

    warning[E150]: ABS on a value of unsigned type 'ULINT' has no effect
       ┌─ <internal>:18:13
       │
    18 │             ABS(d);
       │             ^^^ ABS on a value of unsigned type 'ULINT' has no effect

    warning[E150]: ABS on a value of unsigned type 'UDINT' has no effect
       ┌─ <internal>:19:13
       │
    19 │             ABS(b - UINT#10);
       │             ^^^ ABS on a value of unsigned type 'UDINT' has no effect
    ");
}

#[test]
fn abs_on_signed_or_float_arguments_reports_nothing() {
    let diagnostics = parse_and_validate_buffered(&format!(
        "{ABS_DECLARATION}
        FUNCTION main : DINT
        VAR
            a : SINT;
            b : INT;
            c : DINT;
            d : LINT;
            e : REAL;
            f : LREAL;
            u : UINT;
        END_VAR
            ABS(a);
            ABS(b);
            ABS(c);
            ABS(d);
            ABS(e);
            ABS(f);
            // mixing an unsigned with a signed argument derives a signed type
            ABS(u + b);
        END_FUNCTION
       ",
    ));

    assert!(diagnostics.is_empty(), "expected no diagnostics but got:\n{diagnostics}");
}

#[test]
fn abs_on_a_bit_type_reports_no_unsigned_warning() {
    let diagnostics = parse_and_validate_buffered(&format!(
        "{ABS_DECLARATION}
        FUNCTION main : DINT
        VAR
            a : BYTE;
        END_VAR
            ABS(a);
        END_FUNCTION
       ",
    ));

    assert_snapshot!(diagnostics, @"
    error[E062]: Invalid type nature for generic argument. BYTE is no ANY_NUMBER
       ┌─ <internal>:12:17
       │
    12 │             ABS(a);
       │                 ^ Invalid type nature for generic argument. BYTE is no ANY_NUMBER
    ");
}

#[test]
fn abs_with_invalid_argument_count_on_an_unsigned_monomorph() {
    let diagnostics = parse_and_validate_buffered(&format!(
        "{ABS_DECLARATION}
        FUNCTION main : DINT
        VAR
            u : UINT;
        END_VAR
            ABS(u, u);
        END_FUNCTION
       ",
    ));

    assert_snapshot!(diagnostics, @"
    error[E032]: this POU takes 1 argument but 2 arguments were supplied
       ┌─ <internal>:12:13
       │
    12 │             ABS(u, u);
       │             ^^^ this POU takes 1 argument but 2 arguments were supplied

    warning[E150]: ABS on a value of unsigned type 'UINT' has no effect
       ┌─ <internal>:12:13
       │
    12 │             ABS(u, u);
       │             ^^^ ABS on a value of unsigned type 'UINT' has no effect
    ");
}

#[test]
fn abs_monomorph_called_directly_reports_the_warning() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
        VAR
            u : UDINT;
        END_VAR
            ABS__UDINT(u);
        END_FUNCTION
       ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E150]: ABS on a value of unsigned type 'UDINT' has no effect
      ┌─ <internal>:6:13
      │
    6 │             ABS__UDINT(u);
      │             ^^^^^^^^^^ ABS on a value of unsigned type 'UDINT' has no effect
    ");
}
