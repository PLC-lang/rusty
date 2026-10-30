use insta::assert_snapshot;

use crate::test_utils::tests::{parse_and_validate_buffered, DATE_TIME_ARITHMETIC_FUNCTIONS};

const ALL_TYPES: [&str; 20] = [
    "SINT", "INT", "DINT", "LINT", "USINT", "UINT", "UDINT", "ULINT", "BYTE", "WORD", "DWORD", "LWORD",
    "BOOL", "REAL", "LREAL", "TIME", "LTIME", "DATE", "TOD", "DT",
];

fn declarations() -> String {
    let variables: String = ALL_TYPES.iter().map(|ty| format!("v{}: {ty};\n", camel(ty))).collect();
    format!(
        "
FUNCTION takesUdint : UDINT VAR_INPUT x : UDINT; END_VAR END_FUNCTION
FUNCTION takesDint : DINT VAR_INPUT x : DINT; END_VAR END_FUNCTION
FUNCTION takesReal : REAL VAR_INPUT x : REAL; END_VAR END_FUNCTION
FUNCTION main : DINT
VAR
{variables}
END_VAR
"
    )
}

fn camel(ty: &str) -> String {
    let lower = ty.to_lowercase();
    format!("{}{}", lower[..1].to_uppercase(), &lower[1..])
}

/// Runs the body inside a function that declares one variable per elementary type, named `v<Type>`
fn in_main(body: &str) -> String {
    conversions(&format!("{}{body}\nEND_FUNCTION", declarations()))
}

fn in_main_with_standard_functions(body: &str) -> String {
    conversions(&format!("{DATE_TIME_ARITHMETIC_FUNCTIONS}{}{body}\nEND_FUNCTION", declarations()))
}

fn conversions(src: &str) -> String {
    parse_and_validate_buffered(src)
}

/// Signed values assigned to unsigned or bit types may change their sign, even when the target is wider.
#[test]
fn signed_into_unsigned_or_bit_type_is_a_sign_change() {
    let diagnostics = in_main(
        "
        vUdint := vDint;
        vUlint := vLint;
        vUdint := vSint;
        vUint := vSint;
        vUlint := vDint;
        vDword := vDint;
        vLword := vDint;
        vByte := vSint;
        vWord := vInt;
        vUsint := vInt;
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:30:19
       │
    30 │         vUdint := vDint;
       │                   ^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'LINT' to 'ULINT' may change the sign.
       ┌─ <internal>:31:19
       │
    31 │         vUlint := vLint;
       │                   ^^^^^ Implicit conversion from 'LINT' to 'ULINT' may change the sign.

    warning[E160]: Implicit conversion from 'SINT' to 'UDINT' may change the sign.
       ┌─ <internal>:32:19
       │
    32 │         vUdint := vSint;
       │                   ^^^^^ Implicit conversion from 'SINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'SINT' to 'UINT' may change the sign.
       ┌─ <internal>:33:18
       │
    33 │         vUint := vSint;
       │                  ^^^^^ Implicit conversion from 'SINT' to 'UINT' may change the sign.

    warning[E160]: Implicit conversion from 'DINT' to 'ULINT' may change the sign.
       ┌─ <internal>:34:19
       │
    34 │         vUlint := vDint;
       │                   ^^^^^ Implicit conversion from 'DINT' to 'ULINT' may change the sign.

    warning[E160]: Implicit conversion from 'DINT' to 'DWORD' may change the sign.
       ┌─ <internal>:35:19
       │
    35 │         vDword := vDint;
       │                   ^^^^^ Implicit conversion from 'DINT' to 'DWORD' may change the sign.

    warning[E160]: Implicit conversion from 'DINT' to 'LWORD' may change the sign.
       ┌─ <internal>:36:19
       │
    36 │         vLword := vDint;
       │                   ^^^^^ Implicit conversion from 'DINT' to 'LWORD' may change the sign.

    warning[E160]: Implicit conversion from 'SINT' to 'BYTE' may change the sign.
       ┌─ <internal>:37:18
       │
    37 │         vByte := vSint;
       │                  ^^^^^ Implicit conversion from 'SINT' to 'BYTE' may change the sign.

    warning[E160]: Implicit conversion from 'INT' to 'WORD' may change the sign.
       ┌─ <internal>:38:18
       │
    38 │         vWord := vInt;
       │                  ^^^^ Implicit conversion from 'INT' to 'WORD' may change the sign.

    warning[E067]: Implicit downcast from 'INT' to 'USINT'.
       ┌─ <internal>:39:19
       │
    39 │         vUsint := vInt;
       │                   ^^^^ Implicit downcast from 'INT' to 'USINT'.

    warning[E160]: Implicit conversion from 'INT' to 'USINT' may change the sign.
       ┌─ <internal>:39:19
       │
    39 │         vUsint := vInt;
       │                   ^^^^ Implicit conversion from 'INT' to 'USINT' may change the sign.
    ");
}

/// Unsigned values that do not fit the signed target produce the downcast warning and the sign change warning.
#[test]
fn unsigned_into_signed_type_that_cannot_hold_it_is_a_downcast_and_a_sign_change() {
    let diagnostics = in_main(
        "
        vDint := vUdint;
        vLint := vUlint;
        vSint := vUsint;
        vDint := vDword;
        vSint := vUint;
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E067]: Implicit downcast from 'UDINT' to 'DINT'.
       ┌─ <internal>:30:18
       │
    30 │         vDint := vUdint;
       │                  ^^^^^^ Implicit downcast from 'UDINT' to 'DINT'.

    warning[E160]: Implicit conversion from 'UDINT' to 'DINT' may change the sign.
       ┌─ <internal>:30:18
       │
    30 │         vDint := vUdint;
       │                  ^^^^^^ Implicit conversion from 'UDINT' to 'DINT' may change the sign.

    warning[E067]: Implicit downcast from 'ULINT' to 'LINT'.
       ┌─ <internal>:31:18
       │
    31 │         vLint := vUlint;
       │                  ^^^^^^ Implicit downcast from 'ULINT' to 'LINT'.

    warning[E160]: Implicit conversion from 'ULINT' to 'LINT' may change the sign.
       ┌─ <internal>:31:18
       │
    31 │         vLint := vUlint;
       │                  ^^^^^^ Implicit conversion from 'ULINT' to 'LINT' may change the sign.

    warning[E067]: Implicit downcast from 'USINT' to 'SINT'.
       ┌─ <internal>:32:18
       │
    32 │         vSint := vUsint;
       │                  ^^^^^^ Implicit downcast from 'USINT' to 'SINT'.

    warning[E160]: Implicit conversion from 'USINT' to 'SINT' may change the sign.
       ┌─ <internal>:32:18
       │
    32 │         vSint := vUsint;
       │                  ^^^^^^ Implicit conversion from 'USINT' to 'SINT' may change the sign.

    warning[E067]: Implicit downcast from 'DWORD' to 'DINT'.
       ┌─ <internal>:33:18
       │
    33 │         vDint := vDword;
       │                  ^^^^^^ Implicit downcast from 'DWORD' to 'DINT'.

    warning[E160]: Implicit conversion from 'DWORD' to 'DINT' may change the sign.
       ┌─ <internal>:33:18
       │
    33 │         vDint := vDword;
       │                  ^^^^^^ Implicit conversion from 'DWORD' to 'DINT' may change the sign.

    warning[E067]: Implicit downcast from 'UINT' to 'SINT'.
       ┌─ <internal>:34:18
       │
    34 │         vSint := vUint;
       │                  ^^^^^ Implicit downcast from 'UINT' to 'SINT'.

    warning[E160]: Implicit conversion from 'UINT' to 'SINT' may change the sign.
       ┌─ <internal>:34:18
       │
    34 │         vSint := vUint;
       │                  ^^^^^ Implicit conversion from 'UINT' to 'SINT' may change the sign.
    ");
}

/// Conversions that keep every value (widening, or unsigned into a wider signed type) must not warn.
#[test]
fn lossless_integer_conversions_are_silent() {
    let diagnostics = in_main(
        "
        vUdint := vUint;
        vUdint := vByte;
        vDint := vUint;
        vLint := vUdint;
        vDint := vUsint;
        vInt := vUsint;
        vUint := vByte;
        vByte := vUsint;
        vDword := vUdint;
        vLword := vUlint;
        vDint := vInt;
        vLint := vDint;
        vDint := vDint;
        vUdint := vUdint;
        ",
    );

    assert_snapshot!(diagnostics, @r"");
}

/// Aliases are judged by the type they stand for.
#[test]
fn alias_types_are_transparent_for_sign_change() {
    let diagnostics = conversions(
        "
        TYPE Counter : UDINT; END_TYPE
        TYPE Offset : DINT; END_TYPE
        FUNCTION main : DINT
        VAR
            vCounter : Counter;
            vOffset : Offset;
            vDint : DINT;
            vUdint : UDINT;
        END_VAR
            vCounter := vDint;
            vCounter := vOffset;
            vUdint := vOffset;
            vOffset := vUdint;
            vCounter := vUdint;
            vOffset := vDint;
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:11:25
       │
    11 │             vCounter := vDint;
       │                         ^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:12:25
       │
    12 │             vCounter := vOffset;
       │                         ^^^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:13:23
       │
    13 │             vUdint := vOffset;
       │                       ^^^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.

    warning[E067]: Implicit downcast from 'UDINT' to 'DINT'.
       ┌─ <internal>:14:24
       │
    14 │             vOffset := vUdint;
       │                        ^^^^^^ Implicit downcast from 'UDINT' to 'DINT'.

    warning[E160]: Implicit conversion from 'UDINT' to 'DINT' may change the sign.
       ┌─ <internal>:14:24
       │
    14 │             vOffset := vUdint;
       │                        ^^^^^^ Implicit conversion from 'UDINT' to 'DINT' may change the sign.
    ");
}

/// Untyped literals are judged by their value.
#[test]
fn non_negative_literals_are_silent() {
    let diagnostics = in_main(
        "
        vUdint := 5;
        vUlint := 5;
        vByte := 255;
        vDword := 16#FFFF_FFFF;
        vUdint := 0;
        vDword := 16#FF;
        vReal := 1;
        ",
    );

    assert_snapshot!(diagnostics, @r"");
}

/// Negative literals and typed signed literals warn when assigned to unsigned or bit types.
#[test]
fn negative_and_typed_literals_into_unsigned_or_bit_types_are_a_sign_change() {
    let diagnostics = in_main(
        "
        vUdint := -1;
        vByte := -1;
        vUlint := -5;
        vUdint := DINT#-5;
        vDword := -1;
        vUdint := DINT#5;
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E160]: Implicit conversion from 'SINT' to 'UDINT' may change the sign.
       ┌─ <internal>:30:19
       │
    30 │         vUdint := -1;
       │                   ^^ Implicit conversion from 'SINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'SINT' to 'BYTE' may change the sign.
       ┌─ <internal>:31:18
       │
    31 │         vByte := -1;
       │                  ^^ Implicit conversion from 'SINT' to 'BYTE' may change the sign.

    warning[E160]: Implicit conversion from 'SINT' to 'ULINT' may change the sign.
       ┌─ <internal>:32:19
       │
    32 │         vUlint := -5;
       │                   ^^ Implicit conversion from 'SINT' to 'ULINT' may change the sign.

    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:33:19
       │
    33 │         vUdint := DINT#-5;
       │                   ^^^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'SINT' to 'DWORD' may change the sign.
       ┌─ <internal>:34:19
       │
    34 │         vDword := -1;
       │                   ^^ Implicit conversion from 'SINT' to 'DWORD' may change the sign.

    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:35:19
       │
    35 │         vUdint := DINT#5;
       │                   ^^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.
    ");
}

/// Negative literals into signed targets are valid.
#[test]
fn negative_literals_into_signed_types_are_silent() {
    let diagnostics = in_main(
        "
        vSint := -1;
        vDint := -1;
        vLint := -5;
        vDint := DINT#-5;
        ",
    );

    assert_snapshot!(diagnostics, @r"");
}

/// Arithmetic between an unsigned variable and an untyped literal must not warn.
#[test]
fn counting_idioms_on_unsigned_types_are_silent() {
    let diagnostics = in_main(
        "
        vUdint := vUdint + 1;
        vUdint := vUdint - 1;
        vUint := vUint - 3;
        vByte := vByte + 1;
        vUsint := vUsint + 1;
        vUdint := vUdint * 2;
        vUdint := vUdint / 2;
        vUdint := vUdint MOD 10;
        vUlint := vUlint + 1;
        vDword := vDword + 16#FF;
        vWord := vWord AND 16#FF;
        vDword := vDword OR 1;
        vUdint := 1 + vUdint;
        vUdint := (vUdint + 1) * 2;

        vUdint := ADD(vUdint, 1);
        vUdint := SUB(vUdint, 1);
        vUdint := SUB(IN1 := vUdint, IN2 := 1);
        vUint := SUB(IN1 := vUint, IN2 := 3);
        vUdint := MUL(vUdint, 2);
        vUdint := DIV(vUdint, 2);
        vUdint := DIV(IN1 := vUdint, IN2 := 2);
        vUlint := ADD(vUlint, 1);
        vUdint := ADD(1, vUdint);
        vUdint := MUL(ADD(vUdint, 1), 2);
        ",
    );

    assert_snapshot!(diagnostics, @r"");
}

/// Operands of the same signedness do not change sign when they are promoted.
#[test]
fn promoted_operands_of_the_same_signedness_are_silent() {
    let diagnostics = in_main(
        "
        vUdint := vByte + vByte;
        vUdint := vUsint + vUint;
        vDint := vSint + vSint;
        vDint := vInt + vSint;
        vLint := vDint + vDint;
        vUlint := vUdint + vUdint;
        vDint := vSint * vInt;

        vUdint := ADD(vUsint, vUint);
        vDint := ADD(vSint, vSint);
        vLint := ADD(vDint, vDint);
        vUlint := ADD(vUdint, vUdint);
        vDint := MUL(vSint, vInt);
        vUdint := SUB(IN1 := vUdint, IN2 := vUint);
        ",
    );

    assert_snapshot!(diagnostics, @"");
}

/// A signed and an unsigned operand are promoted to the signed type: the unsigned operand is reported, and the
/// signed result follows from it.
#[test]
fn mixed_signedness_is_reported_at_the_converted_operand() {
    let diagnostics = in_main(
        "
        vUdint := vUdint + vDint;
        vUdint := vDint + vUdint;
        vUlint := vUlint + vLint;
        vUdint := vUdint + vSint;
        vDword := vDword + vDint;
        vUdint := vUsint + vSint;

        vUdint := ADD(vUdint, vDint);
        vUdint := SUB(IN1 := vUdint, IN2 := vDint);
        vUlint := ADD(vUlint, vLint);
        vUdint := ADD(vUsint, vSint);
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:30:19
       │
    30 │         vUdint := vUdint + vDint;
       │                   ^^^^^^ Operand of type 'UDINT' is converted to 'DINT', which may change its sign.

    warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:31:27
       │
    31 │         vUdint := vDint + vUdint;
       │                           ^^^^^^ Operand of type 'UDINT' is converted to 'DINT', which may change its sign.

    warning[E161]: Operand of type 'ULINT' is converted to 'LINT', which may change its sign.
       ┌─ <internal>:32:19
       │
    32 │         vUlint := vUlint + vLint;
       │                   ^^^^^^ Operand of type 'ULINT' is converted to 'LINT', which may change its sign.

    warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:33:19
       │
    33 │         vUdint := vUdint + vSint;
       │                   ^^^^^^ Operand of type 'UDINT' is converted to 'DINT', which may change its sign.

    warning[E161]: Operand of type 'DWORD' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:34:19
       │
    34 │         vDword := vDword + vDint;
       │                   ^^^^^^ Operand of type 'DWORD' is converted to 'DINT', which may change its sign.

    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:35:19
       │
    35 │         vUdint := vUsint + vSint;
       │                   ^^^^^^^^^^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.

    warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:37:23
       │
    37 │         vUdint := ADD(vUdint, vDint);
       │                       ^^^^^^ Operand of type 'UDINT' is converted to 'DINT', which may change its sign.

    warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:38:30
       │
    38 │         vUdint := SUB(IN1 := vUdint, IN2 := vDint);
       │                              ^^^^^^ Operand of type 'UDINT' is converted to 'DINT', which may change its sign.

    warning[E161]: Operand of type 'ULINT' is converted to 'LINT', which may change its sign.
       ┌─ <internal>:39:23
       │
    39 │         vUlint := ADD(vUlint, vLint);
       │                       ^^^^^^ Operand of type 'ULINT' is converted to 'LINT', which may change its sign.

    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:40:19
       │
    40 │         vUdint := ADD(vUsint, vSint);
       │                   ^^^^^^^^^^^^^^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.
    ");
}

/// A signed expression assigned to an unsigned target is reported once, on the whole expression.
#[test]
fn a_signed_result_is_reported_once_at_the_expression() {
    let diagnostics = in_main(
        "
        vUdint := vDint + vDint;
        vUdint := -vDint;
        vUdint := (vDint);

        vUdint := ADD(vDint, vDint);
        vUdint := SUB(IN1 := vDint, IN2 := vDint);
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:30:19
       │
    30 │         vUdint := vDint + vDint;
       │                   ^^^^^^^^^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:31:19
       │
    31 │         vUdint := -vDint;
       │                   ^^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:32:20
       │
    32 │         vUdint := (vDint);
       │                    ^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:34:19
       │
    34 │         vUdint := ADD(vDint, vDint);
       │                   ^^^^^^^^^^^^^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:35:19
       │
    35 │         vUdint := SUB(IN1 := vDint, IN2 := vDint);
       │                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.
    ");
}

/// Arguments are converted to the parameter type like assignments.
#[test]
fn signed_arguments_into_unsigned_parameters_are_a_sign_change() {
    let diagnostics = in_main(
        "
        vUdint := takesUdint(vDint);
        vUdint := takesUdint(x := vDint);
        vUdint := takesUdint(vSint);
        vUdint := takesUdint(-1);
        vUdint := takesUdint(vUint);
        vUdint := takesUdint(vUdint);
        vUdint := takesUdint(5);
        vDint := takesDint(vUdint);
        vDint := takesDint(vDint);
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:30:30
       │
    30 │         vUdint := takesUdint(vDint);
       │                              ^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:31:35
       │
    31 │         vUdint := takesUdint(x := vDint);
       │                                   ^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'SINT' to 'UDINT' may change the sign.
       ┌─ <internal>:32:30
       │
    32 │         vUdint := takesUdint(vSint);
       │                              ^^^^^ Implicit conversion from 'SINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'SINT' to 'UDINT' may change the sign.
       ┌─ <internal>:33:30
       │
    33 │         vUdint := takesUdint(-1);
       │                              ^^ Implicit conversion from 'SINT' to 'UDINT' may change the sign.

    warning[E067]: Implicit downcast from 'UDINT' to 'DINT'.
       ┌─ <internal>:37:28
       │
    37 │         vDint := takesDint(vUdint);
       │                            ^^^^^^ Implicit downcast from 'UDINT' to 'DINT'.

    warning[E160]: Implicit conversion from 'UDINT' to 'DINT' may change the sign.
       ┌─ <internal>:37:28
       │
    37 │         vDint := takesDint(vUdint);
       │                            ^^^^^^ Implicit conversion from 'UDINT' to 'DINT' may change the sign.
    ");
}

/// Assigning to the function name converts to the return type.
#[test]
fn the_returned_value_is_converted_to_the_return_type() {
    let diagnostics = conversions(
        "
        FUNCTION fromSigned : UDINT
        VAR
            vDint : DINT;
            vUdint : UDINT;
        END_VAR
            fromSigned := vDint;
            fromSigned := vUdint;
            fromSigned := 5;
            fromSigned := -1;
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
      ┌─ <internal>:7:27
      │
    7 │             fromSigned := vDint;
      │                           ^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'SINT' to 'UDINT' may change the sign.
       ┌─ <internal>:10:27
       │
    10 │             fromSigned := -1;
       │                           ^^ Implicit conversion from 'SINT' to 'UDINT' may change the sign.
    ");
}

/// Output assignments convert from the output to the assigned variable.
#[test]
fn function_block_outputs_are_checked_in_the_direction_of_the_value() {
    let diagnostics = conversions(
        "
        FUNCTION_BLOCK Producer
        VAR_OUTPUT
            outDint : DINT;
            outUdint : UDINT;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION main : DINT
        VAR
            instance : Producer;
            vDint : DINT;
            vUdint : UDINT;
        END_VAR
            instance(outDint => vUdint);
            instance(outDint => vDint);
            instance(outUdint => vDint);
            instance(outUdint => vUdint);
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:15:22
       │
    15 │             instance(outDint => vUdint);
       │                      ^^^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.

    warning[E067]: Implicit downcast from 'UDINT' to 'DINT'.
       ┌─ <internal>:17:22
       │
    17 │             instance(outUdint => vDint);
       │                      ^^^^^^^^ Implicit downcast from 'UDINT' to 'DINT'.

    warning[E160]: Implicit conversion from 'UDINT' to 'DINT' may change the sign.
       ┌─ <internal>:17:22
       │
    17 │             instance(outUdint => vDint);
       │                      ^^^^^^^^ Implicit conversion from 'UDINT' to 'DINT' may change the sign.
    ");
}

/// Function block inputs and method arguments are converted like function arguments.
#[test]
fn function_block_inputs_and_methods_are_checked_like_functions() {
    let diagnostics = conversions(
        "
        FUNCTION_BLOCK Stepper
        VAR_INPUT step : UDINT; END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK Adder
        METHOD plus : UDINT
        VAR_INPUT amount : UDINT; END_VAR
        END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main : DINT
        VAR
            stepper : Stepper;
            adder : Adder;
            vDint : DINT;
            vUdint : UDINT;
        END_VAR
            stepper(step := vDint);
            stepper(step := vUdint);
            stepper(step := 1);
            vUdint := adder.plus(vDint);
            vUdint := adder.plus(vUdint);
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:19:29
       │
    19 │             stepper(step := vDint);
       │                             ^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:22:34
       │
    22 │             vUdint := adder.plus(vDint);
       │                                  ^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.
    ");
}

/// Variadic arguments have no parameter type, so nothing is converted.
#[test]
fn variadic_arguments_are_not_converted() {
    let diagnostics = conversions(
        "
        {external}
        FUNCTION printf : DINT
        VAR_INPUT {ref}
            format : STRING;
        END_VAR
        VAR_INPUT
            args : ...;
        END_VAR
        END_FUNCTION

        FUNCTION main : DINT
        VAR
            vDint : DINT;
            vUdint : UDINT;
        END_VAR
            printf('%d %u', vDint, vUdint);
            printf('%d', -1);
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @r"");
}

/// Variable, array and struct member initializers are converted to the declared type.
#[test]
fn initializers_are_converted_to_the_declared_type() {
    let diagnostics = conversions(
        "
        VAR_GLOBAL CONSTANT
            positive : DINT := 5;
            negative : DINT := -5;
        END_VAR

        VAR_GLOBAL
            fromPositive : UDINT := positive;
            fromNegative : UDINT := negative;
            fromLiteral : UDINT := 5;
            fromNegativeLiteral : UDINT := -1;
            elements : ARRAY[0..2] OF UDINT := [1, 2, -1];
        END_VAR

        TYPE Point : STRUCT
            x : UDINT := 1;
            y : UDINT := -1;
        END_STRUCT END_TYPE
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E039]: This will overflow for type UDINT
       ┌─ <internal>:11:44
       │
    11 │             fromNegativeLiteral : UDINT := -1;
       │                                            ^^ This will overflow for type UDINT

    warning[E039]: This will overflow for type UDINT
       ┌─ <internal>:12:55
       │
    12 │             elements : ARRAY[0..2] OF UDINT := [1, 2, -1];
       │                                                       ^^ This will overflow for type UDINT

    warning[E039]: This will overflow for type UDINT
       ┌─ <internal>:17:26
       │
    17 │             y : UDINT := -1;
       │                          ^^ This will overflow for type UDINT

    warning[E160]: Implicit conversion from 'SINT' to 'UDINT' may change the sign.
       ┌─ <internal>:17:26
       │
    17 │             y : UDINT := -1;
       │                          ^^ Implicit conversion from 'SINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
      ┌─ <internal>:9:37
      │
    9 │             fromNegative : UDINT := negative;
      │                                     ^^^^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'SINT' to 'UDINT' may change the sign.
       ┌─ <internal>:11:44
       │
    11 │             fromNegativeLiteral : UDINT := -1;
       │                                            ^^ Implicit conversion from 'SINT' to 'UDINT' may change the sign.

    warning[E160]: Implicit conversion from 'SINT' to 'UDINT' may change the sign.
       ┌─ <internal>:12:55
       │
    12 │             elements : ARRAY[0..2] OF UDINT := [1, 2, -1];
       │                                                       ^^ Implicit conversion from 'SINT' to 'UDINT' may change the sign.
    ");
}

/// A constant with a non-negative value converts to unsigned without a sign change.
#[test]
fn constants_are_judged_by_their_value() {
    let diagnostics = conversions(
        "
        VAR_GLOBAL CONSTANT
            size : DINT := 16;
            offset : DINT := -4;
        END_VAR

        FUNCTION main : DINT
        VAR
            vUdint : UDINT;
        END_VAR
            vUdint := size;
            vUdint := size + 1;
            vUdint := offset;
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:13:23
       │
    13 │             vUdint := offset;
       │                       ^^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.
    ");
}

/// Enums and subranges are not checked.
#[test]
fn subranges_and_enums_are_not_reported_as_sign_change_when_they_cannot_change_the_value() {
    let diagnostics = conversions(
        "
        TYPE Percent : DINT(0..100); END_TYPE
        TYPE Mode : (idle, running); END_TYPE

        FUNCTION main : DINT
        VAR
            vPercent : Percent;
            vMode : Mode;
            vUdint : UDINT;
            vDint : DINT;
        END_VAR
            vUdint := vPercent;
            vDint := vMode;
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @r"");
}

/// The selector of SEL and MUX is not part of the result conversion.
#[test]
fn builtin_selectors_are_not_converted() {
    let diagnostics = in_main(
        "
        vUdint := SEL(vBool, vUdint, 5);
        vUdint := SEL(vBool, vUdint, vUdint);
        vUdint := MUX(vDint, vUdint, vUdint);
        ",
    );

    assert_snapshot!(diagnostics, @r"");
}

/// Builtin operator calls, such as `ADD(a, b)`, are evaluated as their operator and convert like it: each call
/// is followed by the same operation written with the operator.
#[test]
fn builtin_operator_calls_convert_like_their_operators() {
    let diagnostics = in_main_with_standard_functions(
        "
        vUdint := ADD(vUdint, 1);
        vUdint := vUdint + 1;
        vUdint := ADD(vUdint, 1, vUdint);
        vUdint := vUdint + 1 + vUdint;
        vUdint := MUL(vUdint, 2);
        vUdint := vUdint * 2;
        vBool := GT(vUdint, 1);
        vBool := vUdint > 1;
        vTime := vTime + 5;
        vUdint := ADD(vUdint, vDint);
        vUdint := vUdint + vDint;
        vReal := ADD(vReal, vDint);
        vReal := vReal + vDint;
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E157]: The integer operand of `+` has no unit and is read as nanoseconds of `TIME`
       ┌─ <internal>:51:18
       │
    51 │         vTime := vTime + 5;
       │                  ^^^^^^^^^ The integer operand of `+` has no unit and is read as nanoseconds of `TIME`

    warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:52:23
       │
    52 │         vUdint := ADD(vUdint, vDint);
       │                       ^^^^^^ Operand of type 'UDINT' is converted to 'DINT', which may change its sign.

    warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:53:19
       │
    53 │         vUdint := vUdint + vDint;
       │                   ^^^^^^ Operand of type 'UDINT' is converted to 'DINT', which may change its sign.

    warning[E162]: Implicit conversion from 'DINT' to 'REAL' may lose precision.
       ┌─ <internal>:54:29
       │
    54 │         vReal := ADD(vReal, vDint);
       │                             ^^^^^ Implicit conversion from 'DINT' to 'REAL' may lose precision.

    warning[E162]: Implicit conversion from 'DINT' to 'REAL' may lose precision.
       ┌─ <internal>:55:26
       │
    55 │         vReal := vReal + vDint;
       │                          ^^^^^ Implicit conversion from 'DINT' to 'REAL' may lose precision.
    ");
}

/// Mixed signedness in builtin arguments and in MOD is reported.
#[test]
fn builtin_operands_of_mixed_signedness_are_reported() {
    let diagnostics = in_main(
        "
        vUdint := SEL(vBool, vUdint, vDint);
        vUdint := vUdint MOD vDint;
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E160]: Implicit conversion from 'DINT' to 'UDINT' may change the sign.
       ┌─ <internal>:30:38
       │
    30 │         vUdint := SEL(vBool, vUdint, vDint);
       │                                      ^^^^^ Implicit conversion from 'DINT' to 'UDINT' may change the sign.

    warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:31:19
       │
    31 │         vUdint := vUdint MOD vDint;
       │                   ^^^^^^ Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
    ");
}

/// Comparing signed with unsigned values promotes the unsigned operand to a signed type.
#[test]
fn mixed_signedness_in_comparisons_is_reported_at_the_converted_operand() {
    let diagnostics = in_main(
        "
        vBool := vDint < vUdint;
        vBool := vUdint < vDint;
        vBool := vLint < vUlint;
        vBool := vDint = vDword;
        vBool := vUdint > -1;

        vBool := LT(vDint, vUdint);
        vBool := EQ(vDint, vDword);
        vBool := GT(vUdint, -1);
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:30:26
       │
    30 │         vBool := vDint < vUdint;
       │                          ^^^^^^ Operand of type 'UDINT' is converted to 'DINT', which may change its sign.

    warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:31:18
       │
    31 │         vBool := vUdint < vDint;
       │                  ^^^^^^ Operand of type 'UDINT' is converted to 'DINT', which may change its sign.

    warning[E161]: Operand of type 'ULINT' is converted to 'LINT', which may change its sign.
       ┌─ <internal>:32:26
       │
    32 │         vBool := vLint < vUlint;
       │                          ^^^^^^ Operand of type 'ULINT' is converted to 'LINT', which may change its sign.

    warning[E161]: Operand of type 'DWORD' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:33:26
       │
    33 │         vBool := vDint = vDword;
       │                          ^^^^^^ Operand of type 'DWORD' is converted to 'DINT', which may change its sign.

    warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:34:18
       │
    34 │         vBool := vUdint > -1;
       │                  ^^^^^^ Operand of type 'UDINT' is converted to 'DINT', which may change its sign.

    warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:36:28
       │
    36 │         vBool := LT(vDint, vUdint);
       │                            ^^^^^^ Operand of type 'UDINT' is converted to 'DINT', which may change its sign.

    warning[E161]: Operand of type 'DWORD' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:37:28
       │
    37 │         vBool := EQ(vDint, vDword);
       │                            ^^^^^^ Operand of type 'DWORD' is converted to 'DINT', which may change its sign.

    warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:38:21
       │
    38 │         vBool := GT(vUdint, -1);
       │                     ^^^^^^ Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
    ");
}

/// Comparisons where the promotion keeps every value must not warn.
#[test]
fn comparisons_that_cannot_change_a_value_are_silent() {
    let diagnostics = in_main(
        "
        vBool := vUdint < 5;
        vBool := vDint < 5;
        vBool := vDint < -5;
        vBool := vUdint > 0;
        vBool := vUint < vInt;
        vBool := vUsint < vSint;
        vBool := vUdint < vLint;
        vBool := vUdint < vUdint;
        vBool := vDint < vDint;
        vBool := vByte = 16#FF;
        vBool := vSint < vDint;
        vBool := vUsint = -1;

        vBool := LT(vUdint, 5);
        vBool := GT(vUdint, 0);
        vBool := LT(vUint, vInt);
        vBool := LT(vUdint, vLint);
        vBool := EQ(vByte, 16#FF);
        vBool := EQ(vUsint, -1);
        ",
    );

    assert_snapshot!(diagnostics, @r"");
}

/// Bitwise operations with mixed signedness are reported like arithmetic.
#[test]
fn mixed_signedness_in_bitwise_operations_is_reported() {
    let diagnostics = in_main(
        "
        vDword := vDword AND vDint;
        vUdint := vUdint OR vSint;
        vDword := vDword AND 16#FF;
        vDword := vDword AND vUdint;
        vDint := vDint AND vDint;
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E161]: Operand of type 'DWORD' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:30:19
       │
    30 │         vDword := vDword AND vDint;
       │                   ^^^^^^ Operand of type 'DWORD' is converted to 'DINT', which may change its sign.

    warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
       ┌─ <internal>:31:19
       │
    31 │         vUdint := vUdint OR vSint;
       │                   ^^^^^^ Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
    ");
}

/// Integers wider than the mantissa of the float lose precision.
#[test]
fn integers_that_a_float_cannot_hold_are_reported() {
    let diagnostics = in_main(
        "
        vReal := vDint;
        vReal := vUdint;
        vReal := vDword;
        vLreal := vLint;
        vLreal := vUlint;
        vReal := takesReal(vDint);
        vReal := vDint + vReal;
        vReal := vLint;

        vReal := ADD(vDint, vReal);
        vReal := SUB(IN1 := vDint, IN2 := vReal);
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E162]: Implicit conversion from 'DINT' to 'REAL' may lose precision.
       ┌─ <internal>:30:18
       │
    30 │         vReal := vDint;
       │                  ^^^^^ Implicit conversion from 'DINT' to 'REAL' may lose precision.

    warning[E162]: Implicit conversion from 'UDINT' to 'REAL' may lose precision.
       ┌─ <internal>:31:18
       │
    31 │         vReal := vUdint;
       │                  ^^^^^^ Implicit conversion from 'UDINT' to 'REAL' may lose precision.

    warning[E162]: Implicit conversion from 'DWORD' to 'REAL' may lose precision.
       ┌─ <internal>:32:18
       │
    32 │         vReal := vDword;
       │                  ^^^^^^ Implicit conversion from 'DWORD' to 'REAL' may lose precision.

    warning[E162]: Implicit conversion from 'LINT' to 'LREAL' may lose precision.
       ┌─ <internal>:33:19
       │
    33 │         vLreal := vLint;
       │                   ^^^^^ Implicit conversion from 'LINT' to 'LREAL' may lose precision.

    warning[E162]: Implicit conversion from 'ULINT' to 'LREAL' may lose precision.
       ┌─ <internal>:34:19
       │
    34 │         vLreal := vUlint;
       │                   ^^^^^^ Implicit conversion from 'ULINT' to 'LREAL' may lose precision.

    warning[E162]: Implicit conversion from 'DINT' to 'REAL' may lose precision.
       ┌─ <internal>:35:28
       │
    35 │         vReal := takesReal(vDint);
       │                            ^^^^^ Implicit conversion from 'DINT' to 'REAL' may lose precision.

    warning[E162]: Implicit conversion from 'DINT' to 'REAL' may lose precision.
       ┌─ <internal>:36:18
       │
    36 │         vReal := vDint + vReal;
       │                  ^^^^^ Implicit conversion from 'DINT' to 'REAL' may lose precision.

    warning[E067]: Implicit downcast from 'LINT' to 'REAL'.
       ┌─ <internal>:37:18
       │
    37 │         vReal := vLint;
       │                  ^^^^^ Implicit downcast from 'LINT' to 'REAL'.

    warning[E162]: Implicit conversion from 'LINT' to 'REAL' may lose precision.
       ┌─ <internal>:37:18
       │
    37 │         vReal := vLint;
       │                  ^^^^^ Implicit conversion from 'LINT' to 'REAL' may lose precision.

    warning[E162]: Implicit conversion from 'DINT' to 'REAL' may lose precision.
       ┌─ <internal>:39:22
       │
    39 │         vReal := ADD(vDint, vReal);
       │                      ^^^^^ Implicit conversion from 'DINT' to 'REAL' may lose precision.

    warning[E162]: Implicit conversion from 'DINT' to 'REAL' may lose precision.
       ┌─ <internal>:40:29
       │
    40 │         vReal := SUB(IN1 := vDint, IN2 := vReal);
       │                             ^^^^^ Implicit conversion from 'DINT' to 'REAL' may lose precision.
    ");
}

/// Integers that fit the mantissa of the float are exact.
#[test]
fn integers_that_a_float_can_hold_are_silent() {
    let diagnostics = in_main(
        "
        vReal := vSint;
        vReal := vInt;
        vReal := vUsint;
        vReal := vUint;
        vReal := vByte;
        vLreal := vDint;
        vLreal := vUdint;
        vLreal := vInt;
        vReal := 1;
        vReal := 16777216;
        vReal := vReal + 1;
        vLreal := vReal;
        vReal := vInt + vReal;
        vReal := vReal * 2;

        vReal := ADD(vReal, 1);
        vReal := ADD(vInt, vReal);
        vReal := MUL(vReal, 2);
        vReal := DIV(IN1 := vReal, IN2 := 2);
        ",
    );

    assert_snapshot!(diagnostics, @r"");
}

/// Floats assigned to integers drop the fraction.
#[test]
fn floats_into_integers_are_reported() {
    let diagnostics = in_main(
        "
        vLint := vReal;
        vUlint := vReal;
        vLword := vReal;
        vDint := vReal;
        vDint := vLreal;
        vInt := vReal;
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E163]: Implicit conversion from 'REAL' to 'LINT' truncates the value.
       ┌─ <internal>:30:18
       │
    30 │         vLint := vReal;
       │                  ^^^^^ Implicit conversion from 'REAL' to 'LINT' truncates the value.

    warning[E163]: Implicit conversion from 'REAL' to 'ULINT' truncates the value.
       ┌─ <internal>:31:19
       │
    31 │         vUlint := vReal;
       │                   ^^^^^ Implicit conversion from 'REAL' to 'ULINT' truncates the value.

    warning[E163]: Implicit conversion from 'REAL' to 'LWORD' truncates the value.
       ┌─ <internal>:32:19
       │
    32 │         vLword := vReal;
       │                   ^^^^^ Implicit conversion from 'REAL' to 'LWORD' truncates the value.

    warning[E067]: Implicit downcast from 'REAL' to 'DINT'.
       ┌─ <internal>:33:18
       │
    33 │         vDint := vReal;
       │                  ^^^^^ Implicit downcast from 'REAL' to 'DINT'.

    warning[E163]: Implicit conversion from 'REAL' to 'DINT' truncates the value.
       ┌─ <internal>:33:18
       │
    33 │         vDint := vReal;
       │                  ^^^^^ Implicit conversion from 'REAL' to 'DINT' truncates the value.

    warning[E067]: Implicit downcast from 'LREAL' to 'DINT'.
       ┌─ <internal>:34:18
       │
    34 │         vDint := vLreal;
       │                  ^^^^^^ Implicit downcast from 'LREAL' to 'DINT'.

    warning[E163]: Implicit conversion from 'LREAL' to 'DINT' truncates the value.
       ┌─ <internal>:34:18
       │
    34 │         vDint := vLreal;
       │                  ^^^^^^ Implicit conversion from 'LREAL' to 'DINT' truncates the value.

    warning[E067]: Implicit downcast from 'REAL' to 'INT'.
       ┌─ <internal>:35:17
       │
    35 │         vInt := vReal;
       │                 ^^^^^ Implicit downcast from 'REAL' to 'INT'.

    warning[E163]: Implicit conversion from 'REAL' to 'INT' truncates the value.
       ┌─ <internal>:35:17
       │
    35 │         vInt := vReal;
       │                 ^^^^^ Implicit conversion from 'REAL' to 'INT' truncates the value.
    ");
}

/// Time and date types do not convert to or from numbers or each other.
#[test]
fn time_and_date_types_do_not_convert_implicitly() {
    let diagnostics = in_main(
        "
        vTime := vDint;
        vDate := vUdint;
        vDt := vDword;
        vDint := vTime;
        vLint := vLtime;
        vDate := vTod;
        vTime := vDt;
        vLtime := vTime;
        vTime := 5000;
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E164]: Implicit conversion from 'DINT' to 'TIME' mixes time or date types with other types.
       ┌─ <internal>:30:18
       │
    30 │         vTime := vDint;
       │                  ^^^^^ Implicit conversion from 'DINT' to 'TIME' mixes time or date types with other types.

    warning[E164]: Implicit conversion from 'UDINT' to 'DATE' mixes time or date types with other types.
       ┌─ <internal>:31:18
       │
    31 │         vDate := vUdint;
       │                  ^^^^^^ Implicit conversion from 'UDINT' to 'DATE' mixes time or date types with other types.

    warning[E164]: Implicit conversion from 'DWORD' to 'DATE_AND_TIME' mixes time or date types with other types.
       ┌─ <internal>:32:16
       │
    32 │         vDt := vDword;
       │                ^^^^^^ Implicit conversion from 'DWORD' to 'DATE_AND_TIME' mixes time or date types with other types.

    warning[E067]: Implicit downcast from 'TIME' to 'DINT'.
       ┌─ <internal>:33:18
       │
    33 │         vDint := vTime;
       │                  ^^^^^ Implicit downcast from 'TIME' to 'DINT'.

    warning[E164]: Implicit conversion from 'TIME' to 'DINT' mixes time or date types with other types.
       ┌─ <internal>:33:18
       │
    33 │         vDint := vTime;
       │                  ^^^^^ Implicit conversion from 'TIME' to 'DINT' mixes time or date types with other types.

    warning[E164]: Implicit conversion from 'TIME' to 'LINT' mixes time or date types with other types.
       ┌─ <internal>:34:18
       │
    34 │         vLint := vLtime;
       │                  ^^^^^^ Implicit conversion from 'TIME' to 'LINT' mixes time or date types with other types.

    warning[E164]: Implicit conversion from 'TIME_OF_DAY' to 'DATE' mixes time or date types with other types.
       ┌─ <internal>:35:18
       │
    35 │         vDate := vTod;
       │                  ^^^^ Implicit conversion from 'TIME_OF_DAY' to 'DATE' mixes time or date types with other types.

    warning[E164]: Implicit conversion from 'DATE_AND_TIME' to 'TIME' mixes time or date types with other types.
       ┌─ <internal>:36:18
       │
    36 │         vTime := vDt;
       │                  ^^^ Implicit conversion from 'DATE_AND_TIME' to 'TIME' mixes time or date types with other types.

    warning[E164]: Implicit conversion from 'INT' to 'TIME' mixes time or date types with other types.
       ┌─ <internal>:38:18
       │
    38 │         vTime := 5000;
       │                  ^^^^ Implicit conversion from 'INT' to 'TIME' mixes time or date types with other types.
    ");
}

/// Arithmetic that is defined for time and date types must not warn.
#[test]
fn time_and_date_arithmetic_is_silent() {
    let diagnostics = in_main_with_standard_functions(
        "
        vTime := vTime + vTime;
        vTime := vTime - vTime;
        vTime := vTime * 3;
        vTime := vTime / 2;
        vTod := vTod + vTime;
        vDt := vDt + vTime;
        vTime := vDate - vDate;
        vTime := vTod - vTod;
        vTime := vDt - vDt;
        vTime := T#5s;
        vBool := vTime < vTime;
        vLtime := vLtime + vLtime;
        ",
    );

    assert_snapshot!(diagnostics, @r"");
}

/// A duration is scaled by the number as it is: the standard functions take every integer and float type, so
/// the factor is never converted.
#[test]
fn durations_scaled_by_any_number_are_silent() {
    let diagnostics = in_main_with_standard_functions(
        "
        vTime := vTime * vDint;
        vTime := vDint * vTime;
        vTime := vTime * vUdint;
        vTime := vTime / vUlint;
        vTime := vTime * vReal;
        vTime := vTime / vLreal;
        vTime := vTime * 1.5;
        vTime := vTime * REAL#2.0;
        vLtime := vLtime * vUlint;
        vLtime := vLtime / 2.5;
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E067]: Implicit downcast from 'ULINT' to 'TIME'.
       ┌─ <internal>:46:26
       │
    46 │         vTime := vTime / vUlint;
       │                          ^^^^^^ Implicit downcast from 'ULINT' to 'TIME'.

    warning[E067]: Implicit downcast from 'ULINT' to 'TIME'.
       ┌─ <internal>:51:28
       │
    51 │         vLtime := vLtime * vUlint;
       │                            ^^^^^^ Implicit downcast from 'ULINT' to 'TIME'.
    ");
}

/// Comparing, MOD and bitwise operations on a time or date with another type are reported once, at the operand.
/// Adding a number to a duration is reported by E157, invalid combinations by E156.
#[test]
fn other_operations_mixing_time_with_another_type_are_reported() {
    let diagnostics = in_main_with_standard_functions(
        "
        vBool := vTime < vDint;
        vBool := vTime = 5;
        vBool := vTime > vLtime;
        vTime := vTime MOD vDint;
        vLtime := vLtime MOD vLint;
        vTime := vTime AND vDint;
        vBool := vTime < vTime;
        vBool := vTime = T#5s;

        vBool := GT(vTime, vDint);
        vBool := EQ(vTime, 5);
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E164]: Operand of type 'DINT' is used with 'TIME', which mixes time or date types with other types.
       ┌─ <internal>:43:26
       │
    43 │         vBool := vTime < vDint;
       │                          ^^^^^ Operand of type 'DINT' is used with 'TIME', which mixes time or date types with other types.

    warning[E164]: Operand of type 'SINT' is used with 'TIME', which mixes time or date types with other types.
       ┌─ <internal>:44:26
       │
    44 │         vBool := vTime = 5;
       │                          ^ Operand of type 'SINT' is used with 'TIME', which mixes time or date types with other types.

    warning[E164]: Operand of type 'DINT' is used with 'TIME', which mixes time or date types with other types.
       ┌─ <internal>:46:28
       │
    46 │         vTime := vTime MOD vDint;
       │                            ^^^^^ Operand of type 'DINT' is used with 'TIME', which mixes time or date types with other types.

    warning[E164]: Operand of type 'LINT' is used with 'TIME', which mixes time or date types with other types.
       ┌─ <internal>:47:30
       │
    47 │         vLtime := vLtime MOD vLint;
       │                              ^^^^^ Operand of type 'LINT' is used with 'TIME', which mixes time or date types with other types.

    warning[E164]: Operand of type 'DINT' is used with 'TIME', which mixes time or date types with other types.
       ┌─ <internal>:48:28
       │
    48 │         vTime := vTime AND vDint;
       │                            ^^^^^ Operand of type 'DINT' is used with 'TIME', which mixes time or date types with other types.

    warning[E164]: Operand of type 'DINT' is used with 'TIME', which mixes time or date types with other types.
       ┌─ <internal>:52:28
       │
    52 │         vBool := GT(vTime, vDint);
       │                            ^^^^^ Operand of type 'DINT' is used with 'TIME', which mixes time or date types with other types.

    warning[E164]: Operand of type 'SINT' is used with 'TIME', which mixes time or date types with other types.
       ┌─ <internal>:53:28
       │
    53 │         vBool := EQ(vTime, 5);
       │                            ^ Operand of type 'SINT' is used with 'TIME', which mixes time or date types with other types.
    ");
}

/// Comparing addresses does not convert a value.
#[test]
fn pointer_comparisons_are_silent() {
    let diagnostics = conversions(
        "
        FUNCTION main : DINT
        VAR
            vPointer : POINTER TO DINT;
            vDint : DINT;
            vBool : BOOL;
        END_VAR
            vBool := vPointer = ADR(vDint);
            vBool := vPointer <> REF(vDint);
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @r"");
}

/// The size of a type is never negative, so it keeps its sign; a narrower target is a downcast.
#[test]
fn sizes_do_not_change_sign() {
    let diagnostics = conversions(
        "
        FUNCTION main : DINT
        VAR
            vArray : ARRAY[0..3] OF DINT;
            vDint : DINT;
            vLint : LINT;
        END_VAR
            vLint := SIZEOF(vArray);
            vDint := SIZEOF(vArray);
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E067]: Implicit downcast from 'ULINT' to 'LINT'.
      ┌─ <internal>:8:22
      │
    8 │             vLint := SIZEOF(vArray);
      │                      ^^^^^^^^^^^^^^ Implicit downcast from 'ULINT' to 'LINT'.

    warning[E067]: Implicit downcast from 'ULINT' to 'DINT'.
      ┌─ <internal>:9:22
      │
    9 │             vDint := SIZEOF(vArray);
      │                      ^^^^^^^^^^^^^^ Implicit downcast from 'ULINT' to 'DINT'.
    ");
}

/// The shift count of a bit shift counts bits, it is not converted.
#[test]
fn shift_counts_are_not_converted() {
    let diagnostics = in_main(
        "
        vByte := SHL(vByte, vInt);
        vByte := SHR(vByte, SINT#1);
        vDword := SHL(vDword, vDint);
        ",
    );

    assert_snapshot!(diagnostics, @r"");
}

/// An argument passed by reference is not converted; a mismatching type is an invalid argument.
#[test]
fn arguments_passed_by_reference_are_not_converted() {
    let diagnostics = conversions(
        "
        FUNCTION takesInOut : DINT
        VAR_IN_OUT
            io : DINT;
        END_VAR
        END_FUNCTION

        FUNCTION main : DINT
        VAR
            vReal : REAL;
            vUdint : UDINT;
        END_VAR
            takesInOut(io := vReal);
            takesInOut(vUdint);
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @"
    error[E037]: Parameter io is passed by reference, it requires an argument of type DINT but got REAL
       ┌─ <internal>:13:30
       │
    13 │             takesInOut(io := vReal);
       │                              ^^^^^ Parameter io is passed by reference, it requires an argument of type DINT but got REAL

    warning[E067]: Implicit downcast from 'REAL' to 'DINT'.
       ┌─ <internal>:13:30
       │
    13 │             takesInOut(io := vReal);
       │                              ^^^^^ Implicit downcast from 'REAL' to 'DINT'.

    error[E037]: Parameter io is passed by reference, it requires an argument of type DINT but got UDINT
       ┌─ <internal>:14:24
       │
    14 │             takesInOut(vUdint);
       │                        ^^^^^^ Parameter io is passed by reference, it requires an argument of type DINT but got UDINT

    warning[E067]: Implicit downcast from 'UDINT' to 'DINT'.
       ┌─ <internal>:14:24
       │
    14 │             takesInOut(vUdint);
       │                        ^^^^^^ Implicit downcast from 'UDINT' to 'DINT'.
    ");
}

/// The bitwise complement of an untyped literal is judged by its value: `NOT -1234` is 1233.
#[test]
fn complements_of_literals_are_judged_by_their_value() {
    let diagnostics = in_main(
        "
        vUint := NOT -1234;
        vUint := NOT 5;
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E160]: Implicit conversion from 'SINT' to 'UINT' may change the sign.
       ┌─ <internal>:31:18
       │
    31 │         vUint := NOT 5;
       │                  ^^^^^ Implicit conversion from 'SINT' to 'UINT' may change the sign.
    ");
}

/// Numbers assigned to BOOL are not truth values.
#[test]
fn bool_and_integers_do_not_convert_implicitly() {
    let diagnostics = in_main(
        "
        vBool := vByte;
        vBool := vSint;
        vBool := vDint;
        vByte := vBool;
        vDint := vBool;
        vBool := vReal;
        vBool := 2;
        ",
    );

    assert_snapshot!(diagnostics, @"
    warning[E165]: Implicit conversion from 'BYTE' to 'BOOL' mixes BOOL with a numeric type.
       ┌─ <internal>:30:18
       │
    30 │         vBool := vByte;
       │                  ^^^^^ Implicit conversion from 'BYTE' to 'BOOL' mixes BOOL with a numeric type.

    warning[E165]: Implicit conversion from 'SINT' to 'BOOL' mixes BOOL with a numeric type.
       ┌─ <internal>:31:18
       │
    31 │         vBool := vSint;
       │                  ^^^^^ Implicit conversion from 'SINT' to 'BOOL' mixes BOOL with a numeric type.

    warning[E067]: Implicit downcast from 'DINT' to 'BOOL'.
       ┌─ <internal>:32:18
       │
    32 │         vBool := vDint;
       │                  ^^^^^ Implicit downcast from 'DINT' to 'BOOL'.

    warning[E165]: Implicit conversion from 'DINT' to 'BOOL' mixes BOOL with a numeric type.
       ┌─ <internal>:32:18
       │
    32 │         vBool := vDint;
       │                  ^^^^^ Implicit conversion from 'DINT' to 'BOOL' mixes BOOL with a numeric type.

    error[E037]: Invalid assignment: cannot assign 'BOOL' to 'BYTE'
       ┌─ <internal>:33:9
       │
    33 │         vByte := vBool;
       │         ^^^^^^^^^^^^^^ Invalid assignment: cannot assign 'BOOL' to 'BYTE'

    error[E037]: Invalid assignment: cannot assign 'BOOL' to 'DINT'
       ┌─ <internal>:34:9
       │
    34 │         vDint := vBool;
       │         ^^^^^^^^^^^^^^ Invalid assignment: cannot assign 'BOOL' to 'DINT'

    warning[E067]: Implicit downcast from 'REAL' to 'BOOL'.
       ┌─ <internal>:35:18
       │
    35 │         vBool := vReal;
       │                  ^^^^^ Implicit downcast from 'REAL' to 'BOOL'.

    warning[E165]: Implicit conversion from 'REAL' to 'BOOL' mixes BOOL with a numeric type.
       ┌─ <internal>:35:18
       │
    35 │         vBool := vReal;
       │                  ^^^^^ Implicit conversion from 'REAL' to 'BOOL' mixes BOOL with a numeric type.

    error[E037]: Invalid assignment: cannot assign 'DINT' to 'BOOL'
       ┌─ <internal>:36:9
       │
    36 │         vBool := 2;
       │         ^^^^^^^^^^ Invalid assignment: cannot assign 'DINT' to 'BOOL'
    ");
}

/// BOOL values, bit access and the literals 0 and 1 are valid.
#[test]
fn bool_values_and_the_literals_zero_and_one_are_silent() {
    let diagnostics = in_main(
        "
        vBool := TRUE;
        vBool := FALSE;
        vBool := 1;
        vBool := 0;
        vBool := vDint < vDint;
        vBool := vByte.0;
        vBool := vDword.31;
        vBool := vBool AND vBool;
        vBool := NOT vBool;
        vByte.3 := vBool;
        ",
    );

    assert_snapshot!(diagnostics, @r"");
}

/// Loop counters and CASE branches are not assignments.
#[test]
fn control_flow_statements_are_not_assignments() {
    let diagnostics = in_main(
        "
        FOR vUdint := 0 TO 10 DO END_FOR
        FOR vDint := 0 TO vUdint DO END_FOR
        CASE vUdint OF
            1: vUdint := 1;
            2..5: vUdint := 2;
        END_CASE
        ",
    );

    assert_snapshot!(diagnostics, @r"");
}
