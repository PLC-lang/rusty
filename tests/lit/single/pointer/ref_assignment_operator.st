// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// CHECK: 5
FUNCTION main : DINT
    VAR
        a : REF_TO DINT;
        b : DINT := 5;
    END_VAR
    a REF= b;

    printf('%d$N', a^);
END_FUNCTION