use crate::test_utils::tests::parse_and_validate_buffered;
use insta::assert_snapshot;

// The standard library functions the operators are carried out by, declared without bodies
const STANDARD_FUNCTIONS: &str = r#"
    FUNCTION ADD_TIME : TIME VAR_INPUT IN1 : TIME; IN2 : TIME; END_VAR END_FUNCTION
    FUNCTION ADD_TOD_TIME : TOD VAR_INPUT IN1 : TOD; IN2 : TIME; END_VAR END_FUNCTION
    FUNCTION ADD_DT_TIME : DT VAR_INPUT IN1 : DT; IN2 : TIME; END_VAR END_FUNCTION
    FUNCTION SUB_TIME : TIME VAR_INPUT IN1 : TIME; IN2 : TIME; END_VAR END_FUNCTION
    FUNCTION SUB_DATE_DATE : TIME VAR_INPUT IN1 : DATE; IN2 : DATE; END_VAR END_FUNCTION
    FUNCTION SUB_TOD_TIME : TOD VAR_INPUT IN1 : TOD; IN2 : TIME; END_VAR END_FUNCTION
    FUNCTION SUB_TOD_TOD : TIME VAR_INPUT IN1 : TOD; IN2 : TOD; END_VAR END_FUNCTION
    FUNCTION SUB_DT_TIME : DT VAR_INPUT IN1 : DT; IN2 : TIME; END_VAR END_FUNCTION
    FUNCTION SUB_DT_DT : TIME VAR_INPUT IN1 : DT; IN2 : DT; END_VAR END_FUNCTION
    FUNCTION MUL_TIME<T: ANY_NUM> : TIME VAR_INPUT IN1 : TIME; IN2 : T; END_VAR END_FUNCTION
    FUNCTION DIV_TIME<T: ANY_NUM> : TIME VAR_INPUT IN1 : TIME; IN2 : T; END_VAR END_FUNCTION

    FUNCTION ADD_LTIME : LTIME VAR_INPUT IN1 : LTIME; IN2 : LTIME; END_VAR END_FUNCTION
    FUNCTION ADD_LTOD_LTIME : LTOD VAR_INPUT IN1 : LTOD; IN2 : LTIME; END_VAR END_FUNCTION
    FUNCTION ADD_LDT_LTIME : LDT VAR_INPUT IN1 : LDT; IN2 : LTIME; END_VAR END_FUNCTION
    FUNCTION SUB_LTIME : LTIME VAR_INPUT IN1 : LTIME; IN2 : LTIME; END_VAR END_FUNCTION
    FUNCTION SUB_LDATE_LDATE : LTIME VAR_INPUT IN1 : LDATE; IN2 : LDATE; END_VAR END_FUNCTION
    FUNCTION SUB_LTOD_LTIME : LTOD VAR_INPUT IN1 : LTOD; IN2 : LTIME; END_VAR END_FUNCTION
    FUNCTION SUB_LTOD_LTOD : LTIME VAR_INPUT IN1 : LTOD; IN2 : LTOD; END_VAR END_FUNCTION
    FUNCTION SUB_LDT_LTIME : LDT VAR_INPUT IN1 : LDT; IN2 : LTIME; END_VAR END_FUNCTION
    FUNCTION SUB_LDT_LDT : LTIME VAR_INPUT IN1 : LDT; IN2 : LDT; END_VAR END_FUNCTION
    FUNCTION MUL_LTIME<T: ANY_NUM> : LTIME VAR_INPUT IN1 : LTIME; IN2 : T; END_VAR END_FUNCTION
    FUNCTION DIV_LTIME<T: ANY_NUM> : LTIME VAR_INPUT IN1 : LTIME; IN2 : T; END_VAR END_FUNCTION
"#;

fn validate_with_standard_functions(body: &str) -> String {
    parse_and_validate_buffered(&format!("{STANDARD_FUNCTIONS}{body}"))
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

    assert_snapshot!(diagnostics, @"
    warning[E067]: Implicit downcast from 'LREAL' to 'TIME'.
       ┌─ <internal>:35:22
       │
    35 │             t := t * r;
       │                      ^ Implicit downcast from 'LREAL' to 'TIME'.

    warning[E067]: Implicit downcast from 'REAL' to 'TIME'.
       ┌─ <internal>:37:22
       │
    37 │             t := t / 2.5;
       │                      ^^^ Implicit downcast from 'REAL' to 'TIME'.

    warning[E067]: Implicit downcast from 'LREAL' to 'LTIME'.
       ┌─ <internal>:39:19
       │
    39 │             lt := r * lt;
       │                   ^ Implicit downcast from 'LREAL' to 'LTIME'.
    ");
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
       ┌─ <internal>:36:19
       │
    36 │             dt := dt + dt;
       │                   ^^^^^^^ Operator `+` is not defined for `DATE_AND_TIME` and `DATE_AND_TIME`

    error[E156]: Operator `+` is not defined for `DATE` and `TIME`
       ┌─ <internal>:37:18
       │
    37 │             d := d + t;
       │                  ^^^^^ Operator `+` is not defined for `DATE` and `TIME`

    error[E156]: Operator `*` is not defined for `TIME` and `TIME`
       ┌─ <internal>:38:18
       │
    38 │             t := t * t;
       │                  ^^^^^ Operator `*` is not defined for `TIME` and `TIME`

    error[E156]: Operator `/` is not defined for `DINT` and `TIME`
       ┌─ <internal>:39:18
       │
    39 │             t := n / t;
       │                  ^^^^^ Operator `/` is not defined for `DINT` and `TIME`

    error[E156]: Operator `-` is not defined for `DINT` and `TIME`
       ┌─ <internal>:40:18
       │
    40 │             t := n - t;
       │                  ^^^^^ Operator `-` is not defined for `DINT` and `TIME`

    error[E156]: Operator `+` is not defined for `TIME_OF_DAY` and `DINT`
       ┌─ <internal>:41:20
       │
    41 │             tod := tod + n;
       │                    ^^^^^^^ Operator `+` is not defined for `TIME_OF_DAY` and `DINT`

    error[E156]: Operator `+` is not defined for `TIME` and `LTIME`
       ┌─ <internal>:42:19
       │
    42 │             lt := t + lt;
       │                   ^^^^^^ Operator `+` is not defined for `TIME` and `LTIME`

    error[E156]: Operator `+` is not defined for `TIME` and `STRING`
       ┌─ <internal>:43:18
       │
    43 │             t := t + s;
       │                  ^^^^^ Operator `+` is not defined for `TIME` and `STRING`

    error[E031]: Invalid expression, types TIME and STRING are incompatible in the given context
       ┌─ <internal>:43:18
       │
    43 │             t := t + s;
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

    error[E073]: Missing function `MUL_TIME` for `TIME * DINT`
      ┌─ <internal>:8:18
      │
    8 │             t := t * 2;
      │                  ^^^^^ Missing function `MUL_TIME` for `TIME * DINT`
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
       ┌─ <internal>:32:18
       │
    32 │             t := t + 5;
       │                  ^^^^^ The integer operand of `+` has no unit and is read as milliseconds of `TIME`

    warning[E157]: The integer operand of `-` has no unit and is read as milliseconds of `TIME`
       ┌─ <internal>:33:18
       │
    33 │             t := t - n;
       │                  ^^^^^ The integer operand of `-` has no unit and is read as milliseconds of `TIME`

    warning[E157]: The integer operand of `+` has no unit and is read as milliseconds of `TIME`
       ┌─ <internal>:34:18
       │
    34 │             t := 5 + t;
       │                  ^^^^^ The integer operand of `+` has no unit and is read as milliseconds of `TIME`

    warning[E157]: The integer operand of `+` has no unit and is read as nanoseconds of `LTIME`
       ┌─ <internal>:35:19
       │
    35 │             lt := lt + 5;
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
