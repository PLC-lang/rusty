// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main : DINT
    printf('%s$N', REF(BYTE_TO_STRING(BYTE#255)));       // CHECK: 255
    printf('%s$N', REF(USINT_TO_STRING(USINT#255)));     // CHECK-NEXT: 255
    printf('%s$N', REF(WORD_TO_STRING(WORD#65535)));     // CHECK-NEXT: 65535
    printf('%s$N', REF(UINT_TO_STRING(UINT#65535)));     // CHECK-NEXT: 65535
    printf('%s$N', REF(DWORD_TO_STRING(DWORD#4294967295))); // CHECK-NEXT: 4294967295
    printf('%s$N', REF(UDINT_TO_STRING(UDINT#4294967295))); // CHECK-NEXT: 4294967295
    printf('%s$N', REF(LWORD_TO_STRING(LWORD#18446744073709551615))); // CHECK-NEXT: 18446744073709551615
    printf('%s$N', REF(ULINT_TO_STRING(ULINT#18446744073709551615))); // CHECK-NEXT: 18446744073709551615
    printf('%s$N', REF(SINT_TO_STRING(SINT#-128)));      // CHECK-NEXT: -128
    printf('%s$N', REF(INT_TO_STRING(INT#-32768)));      // CHECK-NEXT: -32768
    printf('%s$N', REF(INT_TO_STRING(INT#0)));            // CHECK-NEXT: 0
    // Untyped out-of-range numeric arguments are coerced before formatting and must not panic.
    printf('%s$N', REF(BYTE_TO_STRING(256)));                 // CHECK-NEXT: 0
    printf('%s$N', REF(USINT_TO_STRING(256)));                // CHECK-NEXT: 0
    printf('%s$N', REF(WORD_TO_STRING(65536)));               // CHECK-NEXT: 0
    printf('%s$N', REF(UINT_TO_STRING(65536)));               // CHECK-NEXT: 0
    printf('%s$N', REF(DWORD_TO_STRING(4294967296)));         // CHECK-NEXT: 0
    printf('%s$N', REF(UDINT_TO_STRING(4294967296)));         // CHECK-NEXT: 0
    printf('%s$N', REF(LWORD_TO_STRING(-1)));                 // CHECK-NEXT: 18446744073709551615
    printf('%s$N', REF(ULINT_TO_STRING(-1)));                 // CHECK-NEXT: 18446744073709551615
    printf('%s$N', REF(SINT_TO_STRING(128)));                 // CHECK-NEXT: -128
    printf('%s$N', REF(INT_TO_STRING(32768)));                // CHECK-NEXT: -32768
    printf('%s$N', REF(DINT_TO_STRING(2147483648)));          // CHECK-NEXT: -2147483648
    printf('%s$N', REF(LINT_TO_STRING(1.5)));                 // CHECK-NEXT: 1
END_FUNCTION
