// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// A selector whose type is an alias of USINT compares like a USINT in a CASE range.

TYPE Count : USINT; END_TYPE

FUNCTION classify : DINT
    VAR_INPUT
        x : Count;
    END_VAR
    CASE x OF
        11..254: classify := 1;
    ELSE
        classify := 0;
    END_CASE
END_FUNCTION

FUNCTION show
    VAR_INPUT
        x : Count;
    END_VAR
    printf('%u: %d$N', x, classify(x));
END_FUNCTION

FUNCTION main
    show(10);
    show(11);
    show(200);
    show(254);
    show(255);
    // CHECK:      10: 0
    // CHECK-NEXT: 11: 1
    // CHECK-NEXT: 200: 1
    // CHECK-NEXT: 254: 1
    // CHECK-NEXT: 255: 0
END_FUNCTION
