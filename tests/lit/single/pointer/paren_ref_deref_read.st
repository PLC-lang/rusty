// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Regression for #1443: reading through `(REF(x))^` must dereference the
// returned pointer, not load through the variable holding it. Prior to the
// fix this silently miscompiled to a segfault at runtime.

FUNCTION main : DINT
VAR
    x : DINT := 42;
    y : DINT;
END_VAR
    y := (REF(x))^;
    // CHECK: 42
    printf('%d$N', y);
    main := 0;
END_FUNCTION
