// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Stress test for u16 (WORD) FFI return values. Same shape as the BYTE
// regression but at i16 width — exercises the second sub-32-bit slot the
// extension gate must handle consistently across targets.

FUNCTION main : DINT
VAR
    r1 : WORD;
    r2 : WORD;
    r3 : WORD;
END_VAR
    r1 := ROL(WORD#16#FFFF, 0);
    // CHECK: 65535
    printf('%d$N', r1);

    r2 := ROL(WORD#16#8000, 1);
    // CHECK: 1
    printf('%d$N', r2);

    r3 := ROR(WORD#16#0001, 1);
    // CHECK: 32768
    printf('%d$N', r3);

    main := 0;
END_FUNCTION
