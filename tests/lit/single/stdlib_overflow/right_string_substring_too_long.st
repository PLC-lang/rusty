// RUN: (%COMPILE %s && %RUN 2>&1) | %CHECK %s
// Test that RIGHT with substring length > string length yields the whole string

FUNCTION main : DINT
VAR
    s : STRING;
END_VAR
    s := 'sample text';
    // Length 12 exceeds string length 11 -> clamped to the whole string
    s := RIGHT(s, 12);
    // CHECK: sample text
    printf('%s$N', REF(s));
    main := 0;
END_FUNCTION
