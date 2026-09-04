// STRING_TO integer and bit-string conversions share one parser at every width.
// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main
    // Empty input returns zero for every target type.
    printf('byte_empty=%d$N', STRING_TO_BYTE(''));       // CHECK: byte_empty=0
    printf('word_empty=%d$N', STRING_TO_WORD(''));       // CHECK-NEXT: word_empty=0
    printf('dword_empty=%d$N', STRING_TO_DWORD(''));     // CHECK-NEXT: dword_empty=0
    printf('lword_empty=%d$N', STRING_TO_LWORD(''));     // CHECK-NEXT: lword_empty=0
    printf('sint_empty=%d$N', STRING_TO_SINT(''));       // CHECK-NEXT: sint_empty=0
    printf('usint_empty=%d$N', STRING_TO_USINT(''));     // CHECK-NEXT: usint_empty=0
    printf('int_empty=%d$N', STRING_TO_INT(''));         // CHECK-NEXT: int_empty=0
    printf('uint_empty=%d$N', STRING_TO_UINT(''));       // CHECK-NEXT: uint_empty=0
    printf('dint_empty=%d$N', STRING_TO_DINT(''));       // CHECK-NEXT: dint_empty=0
    printf('udint_empty=%d$N', STRING_TO_UDINT(''));     // CHECK-NEXT: udint_empty=0
    printf('lint_empty=%d$N', STRING_TO_LINT(''));       // CHECK-NEXT: lint_empty=0
    printf('ulint_empty=%d$N', STRING_TO_ULINT(''));     // CHECK-NEXT: ulint_empty=0

    // Negative input is preserved by signed targets and rejected by unsigned targets.
    printf('byte_negative=%d$N', STRING_TO_BYTE('-1'));  // CHECK-NEXT: byte_negative=0
    printf('word_negative=%d$N', STRING_TO_WORD('-1'));  // CHECK-NEXT: word_negative=0
    printf('dword_negative=%d$N', STRING_TO_DWORD('-1'));// CHECK-NEXT: dword_negative=0
    printf('lword_negative=%d$N', STRING_TO_LWORD('-1'));// CHECK-NEXT: lword_negative=0
    printf('sint_negative=%d$N', STRING_TO_SINT('-1'));  // CHECK-NEXT: sint_negative=-1
    printf('usint_negative=%d$N', STRING_TO_USINT('-1'));// CHECK-NEXT: usint_negative=0
    printf('int_negative=%d$N', STRING_TO_INT('-1'));    // CHECK-NEXT: int_negative=-1
    printf('uint_negative=%d$N', STRING_TO_UINT('-1'));  // CHECK-NEXT: uint_negative=0
    printf('dint_negative=%d$N', STRING_TO_DINT('-1'));  // CHECK-NEXT: dint_negative=-1
    printf('udint_negative=%d$N', STRING_TO_UDINT('-1'));// CHECK-NEXT: udint_negative=0
    printf('lint_negative=%d$N', STRING_TO_LINT('-1'));  // CHECK-NEXT: lint_negative=-1
    printf('ulint_negative=%d$N', STRING_TO_ULINT('-1'));// CHECK-NEXT: ulint_negative=0

    // Malformed input returns zero for every target type.
    printf('byte_malformed=%d$N', STRING_TO_BYTE('12abc'));   // CHECK-NEXT: byte_malformed=0
    printf('word_malformed=%d$N', STRING_TO_WORD('12abc'));   // CHECK-NEXT: word_malformed=0
    printf('dword_malformed=%d$N', STRING_TO_DWORD('12abc')); // CHECK-NEXT: dword_malformed=0
    printf('lword_malformed=%d$N', STRING_TO_LWORD('12abc')); // CHECK-NEXT: lword_malformed=0
    printf('sint_malformed=%d$N', STRING_TO_SINT('12abc'));   // CHECK-NEXT: sint_malformed=0
    printf('usint_malformed=%d$N', STRING_TO_USINT('12abc')); // CHECK-NEXT: usint_malformed=0
    printf('int_malformed=%d$N', STRING_TO_INT('12abc'));     // CHECK-NEXT: int_malformed=0
    printf('uint_malformed=%d$N', STRING_TO_UINT('12abc'));   // CHECK-NEXT: uint_malformed=0
    printf('dint_malformed=%d$N', STRING_TO_DINT('12abc'));   // CHECK-NEXT: dint_malformed=0
    printf('udint_malformed=%d$N', STRING_TO_UDINT('12abc')); // CHECK-NEXT: udint_malformed=0
    printf('lint_malformed=%d$N', STRING_TO_LINT('12abc'));   // CHECK-NEXT: lint_malformed=0
    printf('ulint_malformed=%d$N', STRING_TO_ULINT('12abc')); // CHECK-NEXT: ulint_malformed=0

    // Fractional input truncates toward zero.
    printf('byte_fraction=%d$N', STRING_TO_BYTE('1.9'));      // CHECK-NEXT: byte_fraction=1
    printf('word_fraction=%d$N', STRING_TO_WORD('1.9'));      // CHECK-NEXT: word_fraction=1
    printf('dword_fraction=%d$N', STRING_TO_DWORD('1.9'));    // CHECK-NEXT: dword_fraction=1
    printf('lword_fraction=%d$N', STRING_TO_LWORD('1.9'));    // CHECK-NEXT: lword_fraction=1
    printf('sint_fraction=%d$N', STRING_TO_SINT('1.9'));      // CHECK-NEXT: sint_fraction=1
    printf('usint_fraction=%d$N', STRING_TO_USINT('1.9'));    // CHECK-NEXT: usint_fraction=1
    printf('int_fraction=%d$N', STRING_TO_INT('1.9'));        // CHECK-NEXT: int_fraction=1
    printf('uint_fraction=%d$N', STRING_TO_UINT('1.9'));      // CHECK-NEXT: uint_fraction=1
    printf('dint_fraction=%d$N', STRING_TO_DINT('1.9'));      // CHECK-NEXT: dint_fraction=1
    printf('udint_fraction=%d$N', STRING_TO_UDINT('1.9'));    // CHECK-NEXT: udint_fraction=1
    printf('lint_fraction=%d$N', STRING_TO_LINT('1.9'));      // CHECK-NEXT: lint_fraction=1
    printf('ulint_fraction=%d$N', STRING_TO_ULINT('1.9'));    // CHECK-NEXT: ulint_fraction=1

    // Surrounding whitespace is ignored for every target type.
    printf('byte_trimmed=%d$N', STRING_TO_BYTE('  12  '));    // CHECK-NEXT: byte_trimmed=12
    printf('word_trimmed=%d$N', STRING_TO_WORD('  12  '));    // CHECK-NEXT: word_trimmed=12
    printf('dword_trimmed=%d$N', STRING_TO_DWORD('  12  '));  // CHECK-NEXT: dword_trimmed=12
    printf('lword_trimmed=%d$N', STRING_TO_LWORD('  12  '));  // CHECK-NEXT: lword_trimmed=12
    printf('sint_trimmed=%d$N', STRING_TO_SINT('  12  '));    // CHECK-NEXT: sint_trimmed=12
    printf('usint_trimmed=%d$N', STRING_TO_USINT('  12  '));  // CHECK-NEXT: usint_trimmed=12
    printf('int_trimmed=%d$N', STRING_TO_INT('  12  '));      // CHECK-NEXT: int_trimmed=12
    printf('uint_trimmed=%d$N', STRING_TO_UINT('  12  '));    // CHECK-NEXT: uint_trimmed=12
    printf('dint_trimmed=%d$N', STRING_TO_DINT('  12  '));    // CHECK-NEXT: dint_trimmed=12
    printf('udint_trimmed=%d$N', STRING_TO_UDINT('  12  '));  // CHECK-NEXT: udint_trimmed=12
    printf('lint_trimmed=%d$N', STRING_TO_LINT('  12  '));    // CHECK-NEXT: lint_trimmed=12
    printf('ulint_trimmed=%d$N', STRING_TO_ULINT('  12  '));  // CHECK-NEXT: ulint_trimmed=12
END_FUNCTION
