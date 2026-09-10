// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that LREAL_TO_STRING uses scientific notation for huge negative values
// instead of overflowing the 80-byte string buffer

FUNCTION main : DINT
VAR
    s : STRING;
END_VAR
    s := LREAL_TO_STRING(LREAL#-1.0E300);
    // CHECK: -1.000000e300
    printf('%s$N', REF(s));

    s := LREAL_TO_STRING(LREAL#1.0E300);
    // CHECK: 1.000000e300
    printf('%s$N', REF(s));

    s := LREAL_TO_STRING(LREAL#-42.5);
    // CHECK: -42.500000
    printf('%s$N', REF(s));

    main := 0;
END_FUNCTION
