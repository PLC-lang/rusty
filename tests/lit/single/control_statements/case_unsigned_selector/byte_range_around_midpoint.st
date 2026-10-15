// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// A narrow range around the midpoint of a BYTE selector, 127..129, matches its three values and
// none of the neighbors.

FUNCTION classify : DINT
    VAR_INPUT
        x : BYTE;
    END_VAR
    CASE x OF
        127..129: classify := 1;
    ELSE
        classify := 0;
    END_CASE
END_FUNCTION

FUNCTION show
    VAR_INPUT
        x : BYTE;
    END_VAR
    printf('%u: %d$N', x, classify(x));
END_FUNCTION

FUNCTION main
    show(126);
    show(127);
    show(128);
    show(129);
    show(130);
    // CHECK:      126: 0
    // CHECK-NEXT: 127: 1
    // CHECK-NEXT: 128: 1
    // CHECK-NEXT: 129: 1
    // CHECK-NEXT: 130: 0
END_FUNCTION
