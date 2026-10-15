// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// A range on a UDINT selector across the midpoint 2^31 matches every value between its bounds.

FUNCTION classify : DINT
    VAR_INPUT
        x : UDINT;
    END_VAR
    CASE x OF
        16#7FFF_FFFF..16#8000_0001: classify := 1;
    ELSE
        classify := 0;
    END_CASE
END_FUNCTION

FUNCTION show
    VAR_INPUT
        x : UDINT;
    END_VAR
    printf('%u: %d$N', x, classify(x));
END_FUNCTION

FUNCTION main
    show(16#7FFF_FFFE);
    show(16#7FFF_FFFF);
    show(16#8000_0000);
    show(16#8000_0001);
    show(16#8000_0002);
    // CHECK:      2147483646: 0
    // CHECK-NEXT: 2147483647: 1
    // CHECK-NEXT: 2147483648: 1
    // CHECK-NEXT: 2147483649: 1
    // CHECK-NEXT: 2147483650: 0
END_FUNCTION
