// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// NOTE: The runtime monitoring write API depends on these exact values.
// If this test fails, check whether the runtime parser needs a matching update.

FUNCTION main : DINT
VAR
    // Two's complement INT hex literals
    i_ffff      : INT  := 16#FFFF;
    i_8000      : INT  := 16#8000;
    i_7fff      : INT  := 16#7FFF;
    i_neg_ffff  : INT  := -16#FFFF;
    i_neg_000f  : INT  := -16#000F;

    // Two's complement DINT hex literals
    d_ffffffff  : DINT := 16#FFFFFFFF;
    d_80000000  : DINT := 16#80000000;

    // Two's complement INT binary literal
    i_bin_ffff  : INT  := 2#1111111111111111;
END_VAR
    // --- INT hex ---
    // CHECK: 16#FFFF as INT: -1
    printf('16#FFFF as INT: %d$N', i_ffff);
    // CHECK: 16#8000 as INT: -32768
    printf('16#8000 as INT: %d$N', i_8000);
    // CHECK: 16#7FFF as INT: 32767
    printf('16#7FFF as INT: %d$N', i_7fff);
    // CHECK: -16#FFFF as INT: 1
    printf('-16#FFFF as INT: %d$N', i_neg_ffff);
    // CHECK: -16#000F as INT: -15
    printf('-16#000F as INT: %d$N', i_neg_000f);

    // --- DINT hex ---
    // CHECK: 16#FFFFFFFF as DINT: -1
    printf('16#FFFFFFFF as DINT: %d$N', d_ffffffff);
    // CHECK: 16#80000000 as DINT: -2147483648
    printf('16#80000000 as DINT: %d$N', d_80000000);

    // --- INT binary ---
    // CHECK: 2#1111111111111111 as INT: -1
    printf('2#1111111111111111 as INT: %d$N', i_bin_ffff);
END_FUNCTION
