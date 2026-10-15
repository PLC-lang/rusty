// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// A label list that mixes single values and a range across the midpoint of a USINT selector
// matches all of them; the values that the list leaves out go to ELSE.

FUNCTION classify : DINT
    VAR_INPUT
        x : USINT;
    END_VAR
    CASE x OF
        0, 11..254, 255: classify := 1;
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
    show(10);
    show(11);
    show(200);
    show(255);
    // CHECK:      0: 1
    // CHECK-NEXT: 10: 0
    // CHECK-NEXT: 11: 1
    // CHECK-NEXT: 200: 1
    // CHECK-NEXT: 255: 1
END_FUNCTION
