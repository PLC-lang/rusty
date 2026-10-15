// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// A range over all values of a USINT selector, 0..255, matches every value, so ELSE never runs.

FUNCTION classify : DINT
    VAR_INPUT
        x : USINT;
    END_VAR
    CASE x OF
        0..255: classify := 1;
    ELSE
        classify := 0;
    END_CASE
END_FUNCTION

FUNCTION show
    VAR_INPUT
        x : USINT;
    END_VAR
    printf('%u: %d$N', x, classify(x));
END_FUNCTION

FUNCTION main
    show(0);
    show(127);
    show(128);
    show(255);
    // CHECK:      0: 1
    // CHECK-NEXT: 127: 1
    // CHECK-NEXT: 128: 1
    // CHECK-NEXT: 255: 1
END_FUNCTION
