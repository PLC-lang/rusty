// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Probe for #1737 follow-up: a FUNCTION-scoped VAR with `AT %M...` should bind its alias
// pointer to the synthesised backing global on entry, so the write below ends up at the
// same hardware slot a VAR_GLOBAL alias would observe.

VAR_GLOBAL
    g AT %MX1.2.3.4 : BYTE;
END_VAR

FUNCTION writer : DINT
VAR
    local AT %MX1.2.3.4 : BYTE;
END_VAR
    local := 7;
    writer := DINT#0;
END_FUNCTION

FUNCTION main : DINT
    writer();
    // CHECK: 7
    printf('%d$N', g);
END_FUNCTION
