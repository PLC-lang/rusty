// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// #1737 coverage: a PROGRAM-scoped VAR with `AT %M...` should bind its alias on the
// per-instance ctor, so the write below ends up at the same hardware slot a
// VAR_GLOBAL alias at the same address observes.

VAR_GLOBAL
    g AT %MX2.3.4.5 : BYTE;
END_VAR

PROGRAM probeProg
VAR
    local AT %MX2.3.4.5 : BYTE;
END_VAR
    local := 11;
END_PROGRAM

FUNCTION main : DINT
    probeProg();
    // CHECK: 11
    printf('%d$N', g);
END_FUNCTION
