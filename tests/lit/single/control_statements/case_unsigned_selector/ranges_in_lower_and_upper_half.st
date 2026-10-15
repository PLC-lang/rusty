// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// Ranges that stay in one half of a USINT selector split the values at the midpoint 128.

FUNCTION classify : DINT
    VAR_INPUT
        x : USINT;
    END_VAR
    CASE x OF
        0..127:   classify := 1;
        128..255: classify := 2;
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
    // CHECK-NEXT: 128: 2
    // CHECK-NEXT: 255: 2
END_FUNCTION
