use crate::test_utils::tests::{parse_and_validate_buffered, DATE_TIME_ARITHMETIC_FUNCTIONS};
use insta::assert_snapshot;

fn validate_with_standard_functions(body: &str) -> String {
    parse_and_validate_buffered(&format!("{DATE_TIME_ARITHMETIC_FUNCTIONS}{body}"))
}

#[test]
fn short_family_operators_defined_by_the_standard_pass() {
    let diagnostics = validate_with_standard_functions(
        r#"
        FUNCTION main : DINT
        VAR
            t1, t2 : TIME;
            tod1, tod2 : TOD;
            d1, d2 : DATE;
            dt1, dt2 : DT;
        END_VAR
            t1 := t1 + t2;
            tod1 := tod1 + t1;
            dt1 := dt1 + t1;
            t1 := t1 - t2;
            t1 := d1 - d2;
            tod1 := tod1 - t1;
            t1 := tod1 - tod2;
            dt1 := dt1 - t1;
            t1 := dt1 - dt2;
        END_FUNCTION
        "#,
    );

    assert_snapshot!(diagnostics, @"");
}

#[test]
fn long_family_operators_defined_by_the_standard_pass() {
    let diagnostics = validate_with_standard_functions(
        r#"
        FUNCTION main : DINT
        VAR
            t1, t2 : LTIME;
            tod1, tod2 : LTOD;
            d1, d2 : LDATE;
            dt1, dt2 : LDT;
        END_VAR
            t1 := t1 + t2;
            tod1 := tod1 + t1;
            dt1 := dt1 + t1;
            t1 := t1 - t2;
            t1 := d1 - d2;
            tod1 := tod1 - t1;
            t1 := tod1 - tod2;
            dt1 := dt1 - t1;
            t1 := dt1 - dt2;
        END_FUNCTION
        "#,
    );

    assert_snapshot!(diagnostics, @"");
}

#[test]
fn duration_scaled_by_a_number_passes() {
    let diagnostics = validate_with_standard_functions(
        r#"
        FUNCTION main : DINT
        VAR
            t : TIME;
            lt : LTIME;
            n : DINT;
            r : LREAL;
        END_VAR
            t := t * 2;
            t := 2 * t;
            t := t * r;
            t := t / n;
            t := t / 2.5;
            lt := lt * n;
            lt := r * lt;
            lt := lt / 4;
        END_FUNCTION
        "#,
    );

    assert_snapshot!(diagnostics, @"");
}

#[test]
fn undefined_combinations_report_e156() {
    let diagnostics = validate_with_standard_functions(
        r#"
        FUNCTION main : DINT
        VAR
            t : TIME;
            lt : LTIME;
            tod : TOD;
            d : DATE;
            dt : DT;
            n : DINT;
            s : STRING;
        END_VAR
            dt := dt + dt;
            d := d + t;
            t := t * t;
            t := n / t;
            t := n - t;
            tod := tod + n;
            lt := t + lt;
            t := t + s;
        END_FUNCTION
        "#,
    );

    assert_snapshot!(diagnostics, @"
    error[E156]: Operator `+` is not defined for `DATE_AND_TIME` and `DATE_AND_TIME`
       ┌─ <internal>:49:19
       │
    49 │             dt := dt + dt;
       │                   ^^^^^^^ Operator `+` is not defined for `DATE_AND_TIME` and `DATE_AND_TIME`

    error[E156]: Operator `+` is not defined for `DATE` and `TIME`
       ┌─ <internal>:50:18
       │
    50 │             d := d + t;
       │                  ^^^^^ Operator `+` is not defined for `DATE` and `TIME`

    error[E156]: Operator `*` is not defined for `TIME` and `TIME`
       ┌─ <internal>:51:18
       │
    51 │             t := t * t;
       │                  ^^^^^ Operator `*` is not defined for `TIME` and `TIME`

    error[E156]: Operator `/` is not defined for `DINT` and `TIME`
       ┌─ <internal>:52:18
       │
    52 │             t := n / t;
       │                  ^^^^^ Operator `/` is not defined for `DINT` and `TIME`

    error[E156]: Operator `-` is not defined for `DINT` and `TIME`
       ┌─ <internal>:53:18
       │
    53 │             t := n - t;
       │                  ^^^^^ Operator `-` is not defined for `DINT` and `TIME`

    error[E156]: Operator `+` is not defined for `TIME_OF_DAY` and `DINT`
       ┌─ <internal>:54:20
       │
    54 │             tod := tod + n;
       │                    ^^^^^^^ Operator `+` is not defined for `TIME_OF_DAY` and `DINT`

    error[E156]: Operator `+` is not defined for `TIME` and `LTIME`
       ┌─ <internal>:55:19
       │
    55 │             lt := t + lt;
       │                   ^^^^^^ Operator `+` is not defined for `TIME` and `LTIME`

    error[E156]: Operator `+` is not defined for `TIME` and `STRING`
       ┌─ <internal>:56:18
       │
    56 │             t := t + s;
       │                  ^^^^^ Operator `+` is not defined for `TIME` and `STRING`

    error[E031]: Invalid expression, types TIME and STRING are incompatible in the given context
       ┌─ <internal>:56:18
       │
    56 │             t := t + s;
       │                  ^^^^^ Invalid expression, types TIME and STRING are incompatible in the given context
    ");
}

#[test]
fn missing_standard_functions_report_e073() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION main : DINT
        VAR
            t : TIME;
            dt : DT;
        END_VAR
            dt := dt + t;
            t := t * 2;
        END_FUNCTION
        "#,
    );

    assert_snapshot!(diagnostics, @"
    error[E073]: Missing function `ADD_DT_TIME` for `DATE_AND_TIME + TIME`
      ┌─ <internal>:7:19
      │
    7 │             dt := dt + t;
      │                   ^^^^^^ Missing function `ADD_DT_TIME` for `DATE_AND_TIME + TIME`

    error[E073]: Missing function `MUL_TIME__LINT` for `TIME * DINT`
      ┌─ <internal>:8:18
      │
    8 │             t := t * 2;
      │                  ^^^^^ Missing function `MUL_TIME__LINT` for `TIME * DINT`
    ");
}

#[test]
fn bare_integer_with_a_duration_reports_e157() {
    let diagnostics = validate_with_standard_functions(
        r#"
        FUNCTION main : DINT
        VAR
            t : TIME;
            lt : LTIME;
            n : DINT;
        END_VAR
            t := t + 5;
            t := t - n;
            t := 5 + t;
            lt := lt + 5;
        END_FUNCTION
        "#,
    );

    assert_snapshot!(diagnostics, @"
    warning[E157]: The integer operand of `+` has no unit and is read as milliseconds of `TIME`
       ┌─ <internal>:45:18
       │
    45 │             t := t + 5;
       │                  ^^^^^ The integer operand of `+` has no unit and is read as milliseconds of `TIME`

    warning[E157]: The integer operand of `-` has no unit and is read as milliseconds of `TIME`
       ┌─ <internal>:46:18
       │
    46 │             t := t - n;
       │                  ^^^^^ The integer operand of `-` has no unit and is read as milliseconds of `TIME`

    warning[E157]: The integer operand of `+` has no unit and is read as milliseconds of `TIME`
       ┌─ <internal>:47:18
       │
    47 │             t := 5 + t;
       │                  ^^^^^ The integer operand of `+` has no unit and is read as milliseconds of `TIME`

    warning[E157]: The integer operand of `+` has no unit and is read as nanoseconds of `LTIME`
       ┌─ <internal>:48:19
       │
    48 │             lt := lt + 5;
       │                   ^^^^^^ The integer operand of `+` has no unit and is read as nanoseconds of `LTIME`
    ");
}

#[test]
fn other_operators_on_date_and_time_are_not_checked() {
    let diagnostics = validate_with_standard_functions(
        r#"
        FUNCTION main : DINT
        VAR
            t1, t2 : TIME;
            dt1, dt2 : DT;
            b : BOOL;
        END_VAR
            t1 := t1 MOD t2;
            t1 := -t1;
            b := dt1 < dt2;
            b := t1 = t2;
        END_FUNCTION
        "#,
    );

    assert_snapshot!(diagnostics, @"");
}

#[test]
fn builtin_calls_with_date_and_time_arguments_pass() {
    let diagnostics = validate_with_standard_functions(
        r#"
        FUNCTION main : DINT
        VAR
            t, t2 : TIME;
            tod : TOD;
            dt : DT;
            d1, d2 : DATE;
            n : DINT;
            r : REAL;
        END_VAR
            dt := ADD(dt, t, t2);
            tod := ADD(tod, t);
            dt := SUB(IN2 := t, IN1 := dt);
            t := SUB(d1, d2);
            t := MUL(2, t, 3);
            t := MUL(t, r);
            t := DIV(t, n);
        END_FUNCTION
        "#,
    );

    assert_snapshot!(diagnostics, @"");
}

#[test]
fn builtin_calls_fold_their_arguments_from_the_left() {
    let diagnostics = validate_with_standard_functions(
        r#"
        FUNCTION main : DINT
        VAR
            t : TIME;
            dt : DT;
            n : DINT;
            s : STRING;
        END_VAR
            dt := ADD(dt, dt);
            dt := ADD(dt, t, dt);
            t := SUB(IN2 := dt, IN1 := t);
            t := DIV(n, t);
            t := ADD(t, 5);
            t := ADD(t, s);
        END_FUNCTION
        "#,
    );

    assert_snapshot!(diagnostics, @"
    error[E156]: Operator `+` is not defined for `DATE_AND_TIME` and `DATE_AND_TIME`
       ┌─ <internal>:46:23
       │
    46 │             dt := ADD(dt, dt);
       │                       ^^^^^^ Operator `+` is not defined for `DATE_AND_TIME` and `DATE_AND_TIME`

    error[E156]: Operator `+` is not defined for `DATE_AND_TIME` and `DATE_AND_TIME`
       ┌─ <internal>:47:23
       │
    47 │             dt := ADD(dt, t, dt);
       │                       ^^^^^^^^^ Operator `+` is not defined for `DATE_AND_TIME` and `DATE_AND_TIME`

    error[E156]: Operator `-` is not defined for `TIME` and `DATE_AND_TIME`
       ┌─ <internal>:48:40
       │
    48 │             t := SUB(IN2 := dt, IN1 := t);
       │                                        ^ Operator `-` is not defined for `TIME` and `DATE_AND_TIME`

    error[E156]: Operator `/` is not defined for `DINT` and `TIME`
       ┌─ <internal>:49:22
       │
    49 │             t := DIV(n, t);
       │                      ^^^^ Operator `/` is not defined for `DINT` and `TIME`

    warning[E067]: Implicit downcast from 'TIME' to 'DINT'.
       ┌─ <internal>:49:25
       │
    49 │             t := DIV(n, t);
       │                         ^ Implicit downcast from 'TIME' to 'DINT'.

    warning[E157]: The integer operand of `+` has no unit and is read as milliseconds of `TIME`
       ┌─ <internal>:50:22
       │
    50 │             t := ADD(t, 5);
       │                      ^^^^ The integer operand of `+` has no unit and is read as milliseconds of `TIME`

    error[E156]: Operator `+` is not defined for `TIME` and `STRING`
       ┌─ <internal>:51:22
       │
    51 │             t := ADD(t, s);
       │                      ^^^^ Operator `+` is not defined for `TIME` and `STRING`
    ");
}

#[test]
fn builtin_calls_without_the_standard_functions_report_e073() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION main : DINT
        VAR
            t : TIME;
            dt : DT;
        END_VAR
            dt := ADD(dt, t);
            t := MUL(t, 2);
        END_FUNCTION
        "#,
    );

    assert_snapshot!(diagnostics, @"
    error[E073]: Missing function `ADD_DT_TIME` for `DATE_AND_TIME + TIME`
      ┌─ <internal>:7:23
      │
    7 │             dt := ADD(dt, t);
      │                       ^^^^^ Missing function `ADD_DT_TIME` for `DATE_AND_TIME + TIME`

    error[E073]: Missing function `MUL_TIME__LINT` for `TIME * DINT`
      ┌─ <internal>:8:22
      │
    8 │             t := MUL(t, 2);
      │                      ^^^^ Missing function `MUL_TIME__LINT` for `TIME * DINT`
    ");
}
