// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Issue #1736: a complete hardware address (AT %MX1.2.3.4) on a POU-scoped variable used to be
// silently dropped at codegen — the FB field stayed null and the first access SIGSEGV'd at
// runtime. The preprocessor now synthesises the backing __M_1_2_3_4 global and the existing
// alias-pointer lowering binds the FB field in the per-instance ctor. The VAR_GLOBAL `slot`
// and the FB `shadow` must end up bound to the *same* `__M_1_2_3_4` slot.

VAR_GLOBAL
    slot AT %MX1.2.3.4 : BYTE;
END_VAR

FUNCTION_BLOCK probe
VAR_INPUT
    shadow AT %MX1.2.3.4 : BYTE;
END_VAR
    shadow := 42;
END_FUNCTION_BLOCK

PROGRAM mainProg
VAR
    p : probe;
END_VAR
    p();
    // CHECK: 42
    printf('%d$N', slot);
END_PROGRAM

FUNCTION main : DINT
    mainProg();
END_FUNCTION
