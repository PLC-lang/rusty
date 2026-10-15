// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// A selector whose type is a subrange of USINT compares like a USINT in a CASE range.

FUNCTION classify : DINT
    VAR_INPUT
        x : USINT(10..250);
    END_VAR
    CASE x OF
        100..200: classify := 1;
    ELSE
        classify := 0;
    END_CASE
END_FUNCTION

FUNCTION show
    VAR_INPUT
        x : USINT(10..250);
    END_VAR
    printf('%u: %d$N', x, classify(x));
END_FUNCTION

FUNCTION main
    show(99);
    show(100);
    show(150);
    show(200);
    show(201);
    // CHECK:      99: 0
    // CHECK-NEXT: 100: 1
    // CHECK-NEXT: 150: 1
    // CHECK-NEXT: 200: 1
    // CHECK-NEXT: 201: 0
END_FUNCTION
