// STRING_TO integer and bit-string conversions accept IEC and C-style radix literals.
// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main
    printf('byte_hex=%d$N', STRING_TO_BYTE('16#FF'));        // CHECK: byte_hex=255
    printf('word_octal=%d$N', STRING_TO_WORD('8#777'));      // CHECK-NEXT: word_octal=511
    printf('dword_binary=%d$N', STRING_TO_DWORD('2#1010'));  // CHECK-NEXT: dword_binary=10
    printf('dint_binary=%d$N', STRING_TO_DINT('2#1010'));    // CHECK-NEXT: dint_binary=10
    printf('lint_binary=%d$N', STRING_TO_LINT('2#1010'));    // CHECK-NEXT: lint_binary=10
    printf('lword_lower_hex=%d$N', STRING_TO_LWORD('16#ff'));// CHECK-NEXT: lword_lower_hex=255
    printf('dint_hex=%d$N', STRING_TO_DINT('16#FF'));        // CHECK-NEXT: dint_hex=255
    printf('lint_hex=%d$N', STRING_TO_LINT('16#FF'));        // CHECK-NEXT: lint_hex=255
    printf('dint_octal=%d$N', STRING_TO_DINT('8#77'));       // CHECK-NEXT: dint_octal=63
    printf('lint_octal=%d$N', STRING_TO_LINT('8#77'));       // CHECK-NEXT: lint_octal=63
    printf('udint_grouped_hex=%d$N', STRING_TO_UDINT('16#FF_FF')); // CHECK-NEXT: udint_grouped_hex=65535
    printf('ulint_trimmed_hex=%d$N', STRING_TO_ULINT(' 16#FF ')); // CHECK-NEXT: ulint_trimmed_hex=255
    printf('missing_digits=%d$N', STRING_TO_UDINT('16#'));   // CHECK-NEXT: missing_digits=0
    printf('invalid_digit=%d$N', STRING_TO_UDINT('16#GG'));  // CHECK-NEXT: invalid_digit=0
    printf('unsupported_radix=%d$N', STRING_TO_UDINT('10#42')); // CHECK-NEXT: unsupported_radix=0
    printf('signed_hex=%d$N', STRING_TO_UDINT('+16#FF'));    // CHECK-NEXT: signed_hex=0

    // C-style binary literals parse for every target type.
    printf('byte_c_binary=%d$N', STRING_TO_BYTE('0b1010'));      // CHECK-NEXT: byte_c_binary=10
    printf('word_c_binary=%d$N', STRING_TO_WORD('0B1010'));      // CHECK-NEXT: word_c_binary=10
    printf('dword_c_binary=%d$N', STRING_TO_DWORD('0b1010'));    // CHECK-NEXT: dword_c_binary=10
    printf('lword_c_binary=%d$N', STRING_TO_LWORD('0B1010'));    // CHECK-NEXT: lword_c_binary=10
    printf('sint_c_binary=%d$N', STRING_TO_SINT('0b1010'));      // CHECK-NEXT: sint_c_binary=10
    printf('usint_c_binary=%d$N', STRING_TO_USINT('0B1010'));    // CHECK-NEXT: usint_c_binary=10
    printf('int_c_binary=%d$N', STRING_TO_INT('0b1010'));        // CHECK-NEXT: int_c_binary=10
    printf('uint_c_binary=%d$N', STRING_TO_UINT('0B1010'));      // CHECK-NEXT: uint_c_binary=10
    printf('dint_c_binary=%d$N', STRING_TO_DINT('0b1010'));      // CHECK-NEXT: dint_c_binary=10
    printf('udint_c_binary=%d$N', STRING_TO_UDINT('0B1010'));    // CHECK-NEXT: udint_c_binary=10
    printf('lint_c_binary=%d$N', STRING_TO_LINT('0b1010'));      // CHECK-NEXT: lint_c_binary=10
    printf('ulint_c_binary=%d$N', STRING_TO_ULINT('0B1010'));    // CHECK-NEXT: ulint_c_binary=10

    // C-style hexadecimal literals parse for every target type.
    printf('byte_c_hex=%d$N', STRING_TO_BYTE('0x7F'));           // CHECK-NEXT: byte_c_hex=127
    printf('word_c_hex=%d$N', STRING_TO_WORD('0X7F'));           // CHECK-NEXT: word_c_hex=127
    printf('dword_c_hex=%d$N', STRING_TO_DWORD('0x7F'));         // CHECK-NEXT: dword_c_hex=127
    printf('lword_c_hex=%d$N', STRING_TO_LWORD('0X7F'));         // CHECK-NEXT: lword_c_hex=127
    printf('sint_c_hex=%d$N', STRING_TO_SINT('0x7F'));           // CHECK-NEXT: sint_c_hex=127
    printf('usint_c_hex=%d$N', STRING_TO_USINT('0X7F'));         // CHECK-NEXT: usint_c_hex=127
    printf('int_c_hex=%d$N', STRING_TO_INT('0x7F'));             // CHECK-NEXT: int_c_hex=127
    printf('uint_c_hex=%d$N', STRING_TO_UINT('0X7F'));           // CHECK-NEXT: uint_c_hex=127
    printf('dint_c_hex=%d$N', STRING_TO_DINT('0x7F'));           // CHECK-NEXT: dint_c_hex=127
    printf('udint_c_hex=%d$N', STRING_TO_UDINT('0X7F'));         // CHECK-NEXT: udint_c_hex=127
    printf('lint_c_hex=%d$N', STRING_TO_LINT('0x7F'));           // CHECK-NEXT: lint_c_hex=127
    printf('ulint_c_hex=%d$N', STRING_TO_ULINT('0X7F'));         // CHECK-NEXT: ulint_c_hex=127
END_FUNCTION
