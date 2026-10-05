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

#[test]
fn abs_on_unsigned_arguments_reports_a_warning() {
    let diagnostics = parse_and_validate_buffered(
        "
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
    );

    assert_snapshot!(diagnostics, @"
    warning[E150]: ABS on a value of unsigned type 'USINT' has no effect
      ┌─ <internal>:9:13
      │
    9 │             ABS(a);
      │             ^^^ ABS on a value of unsigned type 'USINT' has no effect

    warning[E150]: ABS on a value of unsigned type 'UINT' has no effect
       ┌─ <internal>:10:13
       │
    10 │             ABS(b);
       │             ^^^ ABS on a value of unsigned type 'UINT' has no effect

    warning[E150]: ABS on a value of unsigned type 'UDINT' has no effect
       ┌─ <internal>:11:13
       │
    11 │             ABS(c);
       │             ^^^ ABS on a value of unsigned type 'UDINT' has no effect

    warning[E150]: ABS on a value of unsigned type 'ULINT' has no effect
       ┌─ <internal>:12:13
       │
    12 │             ABS(d);
       │             ^^^ ABS on a value of unsigned type 'ULINT' has no effect

    warning[E150]: ABS on a value of unsigned type 'UDINT' has no effect
       ┌─ <internal>:13:13
       │
    13 │             ABS(b - UINT#10);
       │             ^^^ ABS on a value of unsigned type 'UDINT' has no effect
    ");
}

#[test]
fn abs_on_signed_or_float_arguments_reports_nothing() {
    let diagnostics = parse_and_validate_buffered(
        "
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
    );

    assert!(diagnostics.is_empty(), "expected no diagnostics but got:\n{diagnostics}");
}

#[test]
fn abs_on_a_bit_type_reports_no_unsigned_warning() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
        VAR
            a : BYTE;
        END_VAR
            ABS(a);
        END_FUNCTION
       ",
    );

    assert_snapshot!(diagnostics, @"
    error[E062]: Invalid type nature for generic argument. BYTE is no ANY_NUMBER
      ┌─ <internal>:6:17
      │
    6 │             ABS(a);
      │                 ^ Invalid type nature for generic argument. BYTE is no ANY_NUMBER
    ");
}

#[test]
fn abs_with_invalid_argument_count() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
        VAR
            a : DINT;
        END_VAR
            ABS();
            ABS(a, a);
        END_FUNCTION
       ",
    );

    assert_snapshot!(diagnostics, @"
    error[E032]: this POU takes 1 argument but 0 arguments were supplied
      ┌─ <internal>:6:13
      │
    6 │             ABS();
      │             ^^^ this POU takes 1 argument but 0 arguments were supplied

    error[E064]: Could not resolve generic type T with ANY_NUMBER
      ┌─ <internal>:6:13
      │
    6 │             ABS();
      │             ^^^^^^ Could not resolve generic type T with ANY_NUMBER

    error[E032]: this POU takes 1 argument but 2 arguments were supplied
      ┌─ <internal>:7:13
      │
    7 │             ABS(a, a);
      │             ^^^ this POU takes 1 argument but 2 arguments were supplied
    ");
}

#[test]
fn mod_with_a_zero_divisor_reports_an_error() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
        VAR
            x : DINT;
            t : TIME;
            lt : LTIME;
        END_VAR
        VAR CONSTANT
            zero : DINT := 0;
        END_VAR
            x := MOD(x, 0);
            x := MOD(IN1 := x, IN2 := 0);
            x := MOD(IN2 := 0, IN1 := x);
            x := MOD(x, zero);
            x := MOD(0, x); // a zero dividend is valid
            t := MOD(t, T#0s);
            lt := MOD(IN2 := LTIME#0s, IN1 := lt);
        END_FUNCTION
       ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E123]: Division by Zero
       ┌─ <internal>:11:22
       │
    11 │             x := MOD(x, 0);
       │                      ^^^^ Division by Zero

    error[E123]: Division by Zero
       ┌─ <internal>:12:22
       │
    12 │             x := MOD(IN1 := x, IN2 := 0);
       │                      ^^^^^^^^^^^^^^^^^^ Division by Zero

    error[E123]: Division by Zero
       ┌─ <internal>:13:22
       │
    13 │             x := MOD(IN2 := 0, IN1 := x);
       │                      ^^^^^^^^^^^^^^^^^^ Division by Zero

    error[E123]: Division by Zero
       ┌─ <internal>:14:22
       │
    14 │             x := MOD(x, zero);
       │                      ^^^^^^^ Division by Zero

    error[E123]: Division by Zero
       ┌─ <internal>:16:22
       │
    16 │             t := MOD(t, T#0s);
       │                      ^^^^^^^ Division by Zero

    error[E123]: Division by Zero
       ┌─ <internal>:17:23
       │
    17 │             lt := MOD(IN2 := LTIME#0s, IN1 := lt);
       │                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ Division by Zero
    ");
}

#[test]
fn operator_builtins_called_with_invalid_argument_count() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
        VAR
            a : BOOL;
            x : DINT;
        END_VAR
            a := AND(a);
            a := OR(a);
            a := XOR(a);
            x := MOD(x, x, x); // MOD is not extensible
            a := NOT(a, a);
        END_FUNCTION
       ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E032]: this POU takes 2 arguments but 1 argument was supplied
      ┌─ <internal>:7:18
      │
    7 │             a := AND(a);
      │                  ^^^ this POU takes 2 arguments but 1 argument was supplied

    error[E032]: this POU takes 2 arguments but 1 argument was supplied
      ┌─ <internal>:8:18
      │
    8 │             a := OR(a);
      │                  ^^ this POU takes 2 arguments but 1 argument was supplied

    error[E032]: this POU takes 2 arguments but 1 argument was supplied
      ┌─ <internal>:9:18
      │
    9 │             a := XOR(a);
      │                  ^^^ this POU takes 2 arguments but 1 argument was supplied

    error[E032]: this POU takes 2 arguments but 3 arguments were supplied
       ┌─ <internal>:10:18
       │
    10 │             x := MOD(x, x, x); // MOD is not extensible
       │                  ^^^ this POU takes 2 arguments but 3 arguments were supplied

    error[E032]: this POU takes 1 argument but 2 arguments were supplied
       ┌─ <internal>:11:18
       │
    11 │             a := NOT(a, a);
       │                  ^^^ this POU takes 1 argument but 2 arguments were supplied
    ");
}

#[test]
fn mod_rejects_arguments_other_than_numbers_and_equal_durations() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
        VAR
            i : DINT;
            s : STRING;
            t : TIME;
            lt : LTIME;
            d : DATE;
        END_VAR
            s := MOD(s, s);
            d := MOD(IN1 := d, IN2 := d);
            t := MOD(t, lt);
            i := MOD(IN2 := t, IN1 := i);
            t := MOD(t, t); // OK
            lt := MOD(IN2 := lt, IN1 := lt); // OK
        END_FUNCTION
       ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E062]: Invalid type nature for generic argument. STRING is no ANY_NUMBER or ANY_DURATION
       ┌─ <internal>:10:22
       │
    10 │             s := MOD(s, s);
       │                      ^ Invalid type nature for generic argument. STRING is no ANY_NUMBER or ANY_DURATION

    error[E062]: Invalid type nature for generic argument. STRING is no ANY_NUMBER or ANY_DURATION
       ┌─ <internal>:10:25
       │
    10 │             s := MOD(s, s);
       │                         ^ Invalid type nature for generic argument. STRING is no ANY_NUMBER or ANY_DURATION

    error[E062]: Invalid type nature for generic argument. DATE is no ANY_NUMBER or ANY_DURATION
       ┌─ <internal>:11:29
       │
    11 │             d := MOD(IN1 := d, IN2 := d);
       │                             ^ Invalid type nature for generic argument. DATE is no ANY_NUMBER or ANY_DURATION

    error[E062]: Invalid type nature for generic argument. DATE is no ANY_NUMBER or ANY_DURATION
       ┌─ <internal>:11:39
       │
    11 │             d := MOD(IN1 := d, IN2 := d);
       │                                       ^ Invalid type nature for generic argument. DATE is no ANY_NUMBER or ANY_DURATION

    error[E156]: Operator `MOD` is not defined for `TIME` and `LTIME`
       ┌─ <internal>:12:22
       │
    12 │             t := MOD(t, lt);
       │                      ^^^^^ Operator `MOD` is not defined for `TIME` and `LTIME`

    warning[E067]: Implicit downcast from 'LTIME' to 'TIME'.
       ┌─ <internal>:12:18
       │
    12 │             t := MOD(t, lt);
       │                  ^^^^^^^^^^ Implicit downcast from 'LTIME' to 'TIME'.

    error[E156]: Operator `MOD` is not defined for `DINT` and `TIME`
       ┌─ <internal>:13:29
       │
    13 │             i := MOD(IN2 := t, IN1 := i);
       │                             ^^^^^^^^^^^ Operator `MOD` is not defined for `DINT` and `TIME`

    warning[E067]: Implicit downcast from 'TIME' to 'DINT'.
       ┌─ <internal>:13:29
       │
    13 │             i := MOD(IN2 := t, IN1 := i);
       │                             ^ Implicit downcast from 'TIME' to 'DINT'.
    ");
}

#[test]
fn bitwise_builtins_reject_arguments_other_than_bits_and_integers() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
        VAR
            b : BOOL;
            i : DINT;
            r : REAL;
            s : STRING;
            t : TIME;
        END_VAR
            r := AND(r, r);
            s := OR(s, s);
            t := XOR(t, t);
            r := NOT(IN := r);
            s := NOT(s, s);
            b := AND(b, r); // only the REAL is reported
            i := XOR(i, i, r); // only the REAL is reported
        END_FUNCTION
       ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E062]: Invalid type nature for generic argument. REAL is no ANY_BIT or ANY_INT
       ┌─ <internal>:10:22
       │
    10 │             r := AND(r, r);
       │                      ^ Invalid type nature for generic argument. REAL is no ANY_BIT or ANY_INT

    error[E062]: Invalid type nature for generic argument. REAL is no ANY_BIT or ANY_INT
       ┌─ <internal>:10:25
       │
    10 │             r := AND(r, r);
       │                         ^ Invalid type nature for generic argument. REAL is no ANY_BIT or ANY_INT

    error[E062]: Invalid type nature for generic argument. STRING is no ANY_BIT or ANY_INT
       ┌─ <internal>:11:21
       │
    11 │             s := OR(s, s);
       │                     ^ Invalid type nature for generic argument. STRING is no ANY_BIT or ANY_INT

    error[E062]: Invalid type nature for generic argument. STRING is no ANY_BIT or ANY_INT
       ┌─ <internal>:11:24
       │
    11 │             s := OR(s, s);
       │                        ^ Invalid type nature for generic argument. STRING is no ANY_BIT or ANY_INT

    error[E062]: Invalid type nature for generic argument. TIME is no ANY_BIT or ANY_INT
       ┌─ <internal>:12:22
       │
    12 │             t := XOR(t, t);
       │                      ^ Invalid type nature for generic argument. TIME is no ANY_BIT or ANY_INT

    error[E062]: Invalid type nature for generic argument. TIME is no ANY_BIT or ANY_INT
       ┌─ <internal>:12:25
       │
    12 │             t := XOR(t, t);
       │                         ^ Invalid type nature for generic argument. TIME is no ANY_BIT or ANY_INT

    error[E062]: Invalid type nature for generic argument. REAL is no ANY_BIT or ANY_INT
       ┌─ <internal>:13:28
       │
    13 │             r := NOT(IN := r);
       │                            ^ Invalid type nature for generic argument. REAL is no ANY_BIT or ANY_INT

    error[E032]: this POU takes 1 argument but 2 arguments were supplied
       ┌─ <internal>:14:18
       │
    14 │             s := NOT(s, s);
       │                  ^^^ this POU takes 1 argument but 2 arguments were supplied

    error[E062]: Invalid type nature for generic argument. STRING is no ANY_BIT or ANY_INT
       ┌─ <internal>:14:22
       │
    14 │             s := NOT(s, s);
       │                      ^ Invalid type nature for generic argument. STRING is no ANY_BIT or ANY_INT

    error[E062]: Invalid type nature for generic argument. STRING is no ANY_BIT or ANY_INT
       ┌─ <internal>:14:25
       │
    14 │             s := NOT(s, s);
       │                         ^ Invalid type nature for generic argument. STRING is no ANY_BIT or ANY_INT

    error[E062]: Invalid type nature for generic argument. REAL is no ANY_BIT or ANY_INT
       ┌─ <internal>:15:25
       │
    15 │             b := AND(b, r); // only the REAL is reported
       │                         ^ Invalid type nature for generic argument. REAL is no ANY_BIT or ANY_INT

    error[E062]: Invalid type nature for generic argument. REAL is no ANY_BIT or ANY_INT
       ┌─ <internal>:16:28
       │
    16 │             i := XOR(i, i, r); // only the REAL is reported
       │                            ^ Invalid type nature for generic argument. REAL is no ANY_BIT or ANY_INT
    ");
}

#[test]
fn operator_builtins_report_missing_and_unresolved_arguments() {
    let diagnostics = parse_and_validate_buffered(
        "
        FUNCTION main : DINT
        VAR
            a : BOOL;
            x : DINT;
        END_VAR
            AND(0(TRUE));
            MOD(0(1));
            NOT(IN := );
            MOD(IN1 := x, IN2 := );
            NOT(IN => a);
            NOT(IN := missing);
            MOD(missing, x);
            MOD(IN1 := x, IN2 := missing);
            XOR(missing, a);
        END_FUNCTION
       ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E032]: this POU takes 2 arguments but 0 arguments were supplied
      ┌─ <internal>:7:13
      │
    7 │             AND(0(TRUE));
      │             ^^^ this POU takes 2 arguments but 0 arguments were supplied

    error[E064]: Could not resolve generic type T with ANY
      ┌─ <internal>:7:13
      │
    7 │             AND(0(TRUE));
      │             ^^^^^^^^^^^^ Could not resolve generic type T with ANY

    error[E032]: this POU takes 2 arguments but 0 arguments were supplied
      ┌─ <internal>:8:13
      │
    8 │             MOD(0(1));
      │             ^^^ this POU takes 2 arguments but 0 arguments were supplied

    error[E064]: Could not resolve generic type T1 with ANY
      ┌─ <internal>:8:13
      │
    8 │             MOD(0(1));
      │             ^^^^^^^^^ Could not resolve generic type T1 with ANY

    error[E062]: Invalid type nature for generic argument. VOID is no ANY_BIT or ANY_INT
      ┌─ <internal>:9:23
      │
    9 │             NOT(IN := );
      │                       ^ Invalid type nature for generic argument. VOID is no ANY_BIT or ANY_INT

    error[E062]: Invalid type nature for generic argument. VOID is no ANY_NUMBER or ANY_DURATION
       ┌─ <internal>:10:34
       │
    10 │             MOD(IN1 := x, IN2 := );
       │                                  ^ Invalid type nature for generic argument. VOID is no ANY_NUMBER or ANY_DURATION

    error[E062]: Invalid type nature for generic argument. VOID is no ANY_BIT or ANY_INT
       ┌─ <internal>:11:17
       │
    11 │             NOT(IN => a);
       │                 ^^^^^^^ Invalid type nature for generic argument. VOID is no ANY_BIT or ANY_INT

    warning[E135]: 'IN' is an input parameter; use ':=' instead of '=>'
       ┌─ <internal>:11:17
       │
    11 │             NOT(IN => a);
       │                 ^^^^^^^ 'IN' is an input parameter; use ':=' instead of '=>'

    warning[E067]: Implicit downcast from 'BOOL' to 'T'.
       ┌─ <internal>:11:23
       │
    11 │             NOT(IN => a);
       │                       ^ Implicit downcast from 'BOOL' to 'T'.

    error[E048]: Could not resolve reference to missing
       ┌─ <internal>:12:23
       │
    12 │             NOT(IN := missing);
       │                       ^^^^^^^ Could not resolve reference to missing

    error[E048]: Could not resolve reference to missing
       ┌─ <internal>:13:17
       │
    13 │             MOD(missing, x);
       │                 ^^^^^^^ Could not resolve reference to missing

    error[E048]: Could not resolve reference to missing
       ┌─ <internal>:14:34
       │
    14 │             MOD(IN1 := x, IN2 := missing);
       │                                  ^^^^^^^ Could not resolve reference to missing

    error[E048]: Could not resolve reference to missing
       ┌─ <internal>:15:17
       │
    15 │             XOR(missing, a);
       │                 ^^^^^^^ Could not resolve reference to missing
    ");
}
