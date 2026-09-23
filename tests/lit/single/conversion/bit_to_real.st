// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// BOOL and the bit string types widen to REAL and LREAL by value. BOOL, BYTE and WORD fit a REAL
// exactly, and DWORD fits an LREAL exactly. A REAL keeps 24 bits of mantissa, so a DWORD or LWORD
// above 2^24 is rounded to the nearest representable value on the way in, without a diagnostic.

FUNCTION main
VAR
    umax : LWORD := 0;
END_VAR
umax := umax - 1;   // 2^64 - 1

// BOOL
printf('%.1f %.1f %.1f %.1f$N', BOOL_TO_REAL(FALSE), BOOL_TO_REAL(TRUE), BOOL_TO_LREAL(FALSE), BOOL_TO_LREAL(TRUE));  // CHECK: 0.0 1.0 0.0 1.0

// BYTE and WORD, exact over the whole range
printf('%.0f %.0f %.0f %.0f$N', BYTE_TO_REAL(BYTE#255), BYTE_TO_LREAL(BYTE#255), WORD_TO_REAL(WORD#65535), WORD_TO_LREAL(WORD#65535));  // CHECK: 255 255 65535 65535

// DWORD: exact into LREAL, rounded into REAL above 2^24
printf('%.0f %.0f$N', DWORD_TO_LREAL(DWORD#16777217), DWORD_TO_LREAL(DWORD#4294967295));  // CHECK: 16777217 4294967295
printf('%.0f %.0f$N', DWORD_TO_REAL(DWORD#16777216),  DWORD_TO_REAL(DWORD#16777217));     // CHECK: 16777216 16777216
printf('%.0f$N',      DWORD_TO_REAL(DWORD#4294967295));                                   // CHECK: 4294967296

// LWORD into REAL: 2^24 is the last exact integer
printf('%.0f %.0f$N', LWORD_TO_REAL(LWORD#16777216), LWORD_TO_REAL(LWORD#16777217));          // CHECK: 16777216 16777216
printf('%.0f %.0f$N', LWORD_TO_REAL(LWORD#4294967296), LWORD_TO_REAL(LWORD#9007199254740993));  // CHECK: 4294967296 9007199254740992
printf('%.0f$N',      LWORD_TO_REAL(umax));                                                    // CHECK: 18446744073709551616

// LWORD into LREAL: 2^53 is the last exact integer
printf('%.0f %.0f$N', LWORD_TO_LREAL(LWORD#4611686018427387904), LWORD_TO_LREAL(LWORD#9007199254740993));  // CHECK: 4611686018427387904 9007199254740992

// the generic form calls the named one
printf('%.0f %.0f %.1f$N', TO_REAL(BYTE#255), TO_LREAL(LWORD#16777217), TO_REAL(TRUE));  // CHECK: 255 16777217 1.0
END_FUNCTION
