// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// A range whose bounds are USINT constants compares like a range with literal bounds.

VAR_GLOBAL CONSTANT
    LOWER : USINT := 11;
    UPPER : USINT := 254;
END_VAR

FUNCTION classify : DINT
    VAR_INPUT
        x : USINT;
    END_VAR
    CASE x OF
        LOWER..UPPER: classify := 1;
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
    show(10);
    show(11);
    show(128);
    show(254);
    show(255);
    // CHECK:      10: 0
    // CHECK-NEXT: 11: 1
    // CHECK-NEXT: 128: 1
    // CHECK-NEXT: 254: 1
    // CHECK-NEXT: 255: 0
END_FUNCTION
