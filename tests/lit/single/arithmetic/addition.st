// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main
    VAR
        x : DINT;
        y : DINT;
    END_VAR

    x := 5;
    y := 5;
    printf('%d$N', x + y);
    // CHECK: 10
END_FUNCTION
