// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Stress test for u8 (BYTE) FFI return values. ROL__BYTE / ROR__BYTE in the
// Rust stdlib take and return u8. Under the target-aware extension gate
// (#1146 follow-up), the rusty side either emits `zeroext i8` on both call
// and declaration (x86 family / Darwin) or skips the attribute entirely
// (AAPCS aarch64-linux). Either way the runtime behaviour must match the
// IEC semantics: BYTE 0xFF rotated by 0 stays 0xFF; rotating 0x80 left by
// one wraps to 0x01.

FUNCTION main : DINT
VAR
    r1 : BYTE;
    r2 : BYTE;
    r3 : BYTE;
    r4 : BYTE;
END_VAR
    r1 := ROL(BYTE#16#FF, 0);
    // CHECK: 255
    printf('%d$N', r1);

    r2 := ROL(BYTE#16#80, 1);
    // CHECK: 1
    printf('%d$N', r2);

    r3 := ROR(BYTE#16#01, 1);
    // CHECK: 128
    printf('%d$N', r3);

    r4 := ROR(BYTE#16#FF, 4);
    // CHECK: 255
    printf('%d$N', r4);

    main := 0;
END_FUNCTION
