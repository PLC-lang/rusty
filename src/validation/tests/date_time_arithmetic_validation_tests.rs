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
    warning[E156]: Operator `+` is not defined for `DATE_AND_TIME` and `DATE_AND_TIME`, the next version rejects it
       ┌─ <internal>:25:19
       │
    25 │             dt := dt + dt;
       │                   ^^^^^^^ Operator `+` is not defined for `DATE_AND_TIME` and `DATE_AND_TIME`, the next version rejects it

    warning[E156]: Operator `+` is not defined for `DATE` and `TIME`, the next version rejects it
       ┌─ <internal>:26:18
       │
    26 │             d := d + t;
       │                  ^^^^^ Operator `+` is not defined for `DATE` and `TIME`, the next version rejects it

    warning[E156]: Operator `*` is not defined for `TIME` and `TIME`, the next version rejects it
       ┌─ <internal>:27:18
       │
    27 │             t := t * t;
       │                  ^^^^^ Operator `*` is not defined for `TIME` and `TIME`, the next version rejects it

    warning[E156]: Operator `/` is not defined for `DINT` and `TIME`, the next version rejects it
       ┌─ <internal>:28:18
       │
    28 │             t := n / t;
       │                  ^^^^^ Operator `/` is not defined for `DINT` and `TIME`, the next version rejects it

    warning[E156]: Operator `-` is not defined for `DINT` and `TIME`, the next version rejects it
       ┌─ <internal>:29:18
       │
    29 │             t := n - t;
       │                  ^^^^^ Operator `-` is not defined for `DINT` and `TIME`, the next version rejects it

    warning[E156]: Operator `+` is not defined for `TIME_OF_DAY` and `DINT`, the next version rejects it
       ┌─ <internal>:30:20
       │
    30 │             tod := tod + n;
       │                    ^^^^^^^ Operator `+` is not defined for `TIME_OF_DAY` and `DINT`, the next version rejects it

    warning[E156]: Operator `+` mixes `TIME` and `LTIME`, the next version, where the short types are 32-bit, rejects it
       ┌─ <internal>:31:19
       │
    31 │             lt := t + lt;
       │                   ^^^^^^ Operator `+` mixes `TIME` and `LTIME`, the next version, where the short types are 32-bit, rejects it

    warning[E156]: Operator `+` is not defined for `TIME` and `STRING`, the next version rejects it
       ┌─ <internal>:32:18
       │
    32 │             t := t + s;
       │                  ^^^^^ Operator `+` is not defined for `TIME` and `STRING`, the next version rejects it

    error[E031]: Invalid expression, types TIME and STRING are incompatible in the given context
       ┌─ <internal>:32:18
       │
    32 │             t := t + s;
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
            tod : TOD;
        END_VAR
            tod := tod + t;
            t := t * 2.5;
        END_FUNCTION
        "#,
    );

    assert_snapshot!(diagnostics, @"
    error[E073]: Missing function `ADD_TOD_TIME` for `TIME_OF_DAY + TIME`
      ┌─ <internal>:7:20
      │
    7 │             tod := tod + t;
      │                    ^^^^^^^ Missing function `ADD_TOD_TIME` for `TIME_OF_DAY + TIME`

    error[E073]: Missing function `MUL_TIME__REAL` for `TIME * REAL`
      ┌─ <internal>:8:18
      │
    8 │             t := t * 2.5;
      │                  ^^^^^^^ Missing function `MUL_TIME__REAL` for `TIME * REAL`
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
    warning[E157]: The integer operand of `+` has no unit and is read as nanoseconds of `TIME`
       ┌─ <internal>:21:18
       │
    21 │             t := t + 5;
       │                  ^^^^^ The integer operand of `+` has no unit and is read as nanoseconds of `TIME`

    warning[E157]: The integer operand of `-` has no unit and is read as nanoseconds of `TIME`
       ┌─ <internal>:22:18
       │
    22 │             t := t - n;
       │                  ^^^^^ The integer operand of `-` has no unit and is read as nanoseconds of `TIME`

    warning[E157]: The integer operand of `+` has no unit and is read as nanoseconds of `TIME`
       ┌─ <internal>:23:18
       │
    23 │             t := 5 + t;
       │                  ^^^^^ The integer operand of `+` has no unit and is read as nanoseconds of `TIME`

    warning[E157]: The integer operand of `+` has no unit and is read as nanoseconds of `LTIME`
       ┌─ <internal>:24:19
       │
    24 │             lt := lt + 5;
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
