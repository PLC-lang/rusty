// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// A range on a UINT selector across the midpoint 32768 matches every value between its bounds.

FUNCTION classify : DINT
    VAR_INPUT
        x : UINT;
    END_VAR
    CASE x OF
        1..65534: classify := 1;
    ELSE
        classify := 0;
    END_CASE
END_FUNCTION

FUNCTION show
    VAR_INPUT
        x : UINT;
    END_VAR
    printf('%u: %d$N', x, classify(x));
END_FUNCTION

FUNCTION main
    show(0);
    show(1);
    show(32767);
    show(32768);
    show(65534);
    show(65535);
    // CHECK:      0: 0
    // CHECK-NEXT: 1: 1
    // CHECK-NEXT: 32767: 1
    // CHECK-NEXT: 32768: 1
    // CHECK-NEXT: 65534: 1
    // CHECK-NEXT: 65535: 0
END_FUNCTION
