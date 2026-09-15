// STRING_TO integer and bit-string conversions accept IEC based literals and nothing else.
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

    // Based literals are unsigned; a sign on either side of the radix is rejected.
    printf('negative_hex=%d$N', STRING_TO_DINT('-16#FF'));   // CHECK-NEXT: negative_hex=0
    printf('hex_negative=%d$N', STRING_TO_DINT('16#-FF'));   // CHECK-NEXT: hex_negative=0
    printf('binary_negative=%d$N', STRING_TO_SINT('2#-1'));  // CHECK-NEXT: binary_negative=0

    // Underscores group digits; they may not lead, trail or double up.
    printf('leading_underscore=%d$N', STRING_TO_UDINT('16#_FF'));  // CHECK-NEXT: leading_underscore=0
    printf('trailing_underscore=%d$N', STRING_TO_UDINT('16#FF_')); // CHECK-NEXT: trailing_underscore=0
    printf('double_underscore=%d$N', STRING_TO_UDINT('16#F__F')); // CHECK-NEXT: double_underscore=0
    printf('decimal_grouped=%d$N', STRING_TO_UDINT('1_000'));     // CHECK-NEXT: decimal_grouped=1000
    printf('decimal_leading_us=%d$N', STRING_TO_UDINT('_1'));     // CHECK-NEXT: decimal_leading_us=0
    printf('decimal_double_us=%d$N', STRING_TO_UDINT('1__0'));    // CHECK-NEXT: decimal_double_us=0

    // C-style radix prefixes are not Structured Text literals.
    printf('byte_c_binary=%d$N', STRING_TO_BYTE('0b1010'));   // CHECK-NEXT: byte_c_binary=0
    printf('lword_c_binary=%d$N', STRING_TO_LWORD('0B1010')); // CHECK-NEXT: lword_c_binary=0
    printf('dint_c_hex=%d$N', STRING_TO_DINT('0x7F'));        // CHECK-NEXT: dint_c_hex=0
    printf('ulint_c_hex=%d$N', STRING_TO_ULINT('0X7F'));      // CHECK-NEXT: ulint_c_hex=0

    // A hexadecimal bit pattern that does not fit the signed target is rejected, not reinterpreted.
    printf('int_hex_max=%d$N', STRING_TO_INT('16#7FFF'));          // CHECK-NEXT: int_hex_max=32767
    printf('int_hex_all_ones=%d$N', STRING_TO_INT('16#FFFF'));     // CHECK-NEXT: int_hex_all_ones=0
    printf('word_hex_all_ones=%d$N', STRING_TO_WORD('16#FFFF'));   // CHECK-NEXT: word_hex_all_ones=65535
END_FUNCTION
