// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// #1737 coverage: a METHOD-scoped VAR_TEMP with `AT %M...` (temporary, per-call) should
// alias-bind on entry and round-trip a write to the backing hardware slot.

VAR_GLOBAL
    g AT %MX4.5.6.7 : BYTE;
END_VAR

FUNCTION_BLOCK probeFb
    METHOD writer
    VAR_TEMP
        local AT %MX4.5.6.7 : BYTE;
    END_VAR
        local := 23;
    END_METHOD
END_FUNCTION_BLOCK

PROGRAM mainProg
VAR
    p : probeFb;
END_VAR
    p.writer();
END_PROGRAM

FUNCTION main : DINT
    mainProg();
    // CHECK: 23
    printf('%d$N', g);
END_FUNCTION
