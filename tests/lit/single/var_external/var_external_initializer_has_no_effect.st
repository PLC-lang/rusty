// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    i: DINT := 42;
END_VAR

FUNCTION main: DINT
VAR_EXTERNAL
    i : DINT := 1337;
END_VAR
    printf('%d$N', i); // CHECK: 42
END_FUNCTION