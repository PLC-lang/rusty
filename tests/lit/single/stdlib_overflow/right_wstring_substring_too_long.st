// RUN: (%COMPILE %s && %RUN 2>&1) | %CHECK %s
// Test that RIGHT with substring length > string length yields the whole string

FUNCTION main : DINT
VAR
    s : WSTRING;
END_VAR
    s := "sa𝄞ple text";
    // Length 12 exceeds string length 11 -> clamped to the whole string
    s := RIGHT(s, 12);
    // CHECK: sa𝄞ple text
    printf('%s$N', REF(TO_STRING(s)));
    main := 0;
END_FUNCTION
