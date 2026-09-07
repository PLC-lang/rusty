// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that RIGHT with a substring length above the string length clamps
// to the whole string, matching CODESYS

FUNCTION main : DINT
VAR
    s : STRING;
END_VAR
    s := RIGHT('sample text', 12);
    // CHECK: sample text
    printf('%s$N', REF(s));

    s := RIGHT('sample text', -1);
    // CHECK: negative_length_yields_empty
    IF s = '' THEN
        printf('negative_length_yields_empty$N');
    END_IF

    main := 0;
END_FUNCTION
