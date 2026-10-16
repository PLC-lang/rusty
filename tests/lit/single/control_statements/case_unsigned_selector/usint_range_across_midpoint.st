// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// A range on a USINT selector whose bounds lie on both sides of the midpoint 128 matches every
// value between them. Single labels and ranges in the lower half keep working next to it.

FUNCTION classify : DINT
    VAR_INPUT
        x : USINT;
    END_VAR
    CASE x OF
        0:       classify := 1;
        1..9:    classify := 2;
        11..254: classify := 3;
        255:     classify := 4;
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
    show(9);
    show(10);
    show(11);
    show(127);
    show(128);
    show(254);
    show(255);
    // CHECK:      0: 1
    // CHECK-NEXT: 9: 2
    // CHECK-NEXT: 10: 0
    // CHECK-NEXT: 11: 3
    // CHECK-NEXT: 127: 3
    // CHECK-NEXT: 128: 3
    // CHECK-NEXT: 254: 3
    // CHECK-NEXT: 255: 4
END_FUNCTION
