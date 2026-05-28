// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// #1737 coverage: a METHOD-scoped VAR with `AT %M...` should alias-bind on method
// entry and round-trip a write to the backing hardware slot.

VAR_GLOBAL
    g AT %MX3.4.5.6 : BYTE;
END_VAR

FUNCTION_BLOCK probeFb
    METHOD writer
    VAR
        local AT %MX3.4.5.6 : BYTE;
    END_VAR
        local := 17;
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
    // CHECK: 17
    printf('%d$N', g);
END_FUNCTION
