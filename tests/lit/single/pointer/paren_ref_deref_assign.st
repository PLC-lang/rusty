// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Regression for #1463: writing through `(REF(x))^ := value` must reach `x`
// instead of segfaulting. The codegen change extracts the pointer directly
// from the RValue produced by REF instead of loading through it.

FUNCTION main : DINT
VAR
    x : DINT := 0;
END_VAR
    (REF(x))^ := 99;
    // CHECK: 99
    printf('%d$N', x);
    main := 0;
END_FUNCTION
