// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// CHECK: 10
FUNCTION main
    VAR
        x : DINT;
        y : DINT;
    END_VAR

    x := 5;
    y := 5;
    printf('%d$N', x + y);
END_FUNCTION
