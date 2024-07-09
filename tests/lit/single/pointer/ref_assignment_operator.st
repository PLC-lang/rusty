// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main : DINT
    VAR
        a : REF_TO DINT;
        b : DINT := 5;
    END_VAR
    a REF= b;

    // CHECK: 5
    printf('%d$N', a^);
END_FUNCTION