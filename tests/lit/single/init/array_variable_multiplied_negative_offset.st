// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Tests that a non-constant variable multiplier works correctly with
// negative array bounds.

FUNCTION main : DINT
VAR
    n : DINT := 11;
    arr : ARRAY [-10..0] OF DINT := [(n)(99)];
END_VAR
    printf('%d$N', arr[-10]);  // CHECK: 99
    printf('%d$N', arr[-5]);   // CHECK: 99
    printf('%d$N', arr[0]);    // CHECK: 99

    main := 0;
END_FUNCTION
