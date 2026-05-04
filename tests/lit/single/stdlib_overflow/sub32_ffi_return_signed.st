// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Stress test for the sub-32-bit FFI integer ABI fix (#1146).
// Calls Rust stdlib `MUL_LTIME` with negative `SINT` arguments and asserts
// the i8 sign-extension on the parameter side reaches the callee correctly
// under release optimizations. The original repro: passing SINT#-120 used
// to read with the upper bits unset on x86_64 release; this test pins the
// fix and lets the matching CI lanes prove the target-aware gate is right.

FUNCTION main : DINT
VAR
    a : LTIME;
    b : LTIME;
    c : LTIME;
END_VAR
    a := MUL_LTIME(LTIME#1d, SINT#-120);
    // CHECK: -10368000000000000
    printf('%lld$N', a);

    b := MUL_LTIME(LTIME#1d, SINT#-1);
    // CHECK: -86400000000000
    printf('%lld$N', b);

    c := MUL_LTIME(LTIME#1d, SINT#127);
    // CHECK: 10972800000000000
    printf('%lld$N', c);

    main := 0;
END_FUNCTION
