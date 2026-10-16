// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// A range on a ULINT selector across the midpoint 2^63 matches every value between its bounds.

FUNCTION classify : DINT
    VAR_INPUT
        x : ULINT;
    END_VAR
    CASE x OF
        16#7FFF_FFFF_FFFF_FFFF..16#8000_0000_0000_0001: classify := 1;
    ELSE
        classify := 0;
    END_CASE
END_FUNCTION

FUNCTION show
    VAR_INPUT
        x : ULINT;
    END_VAR
    printf('%llu: %d$N', x, classify(x));
END_FUNCTION

FUNCTION main
    show(16#7FFF_FFFF_FFFF_FFFE);
    show(16#7FFF_FFFF_FFFF_FFFF);
    show(16#8000_0000_0000_0000);
    show(16#8000_0000_0000_0001);
    show(16#8000_0000_0000_0002);
    // CHECK:      9223372036854775806: 0
    // CHECK-NEXT: 9223372036854775807: 1
    // CHECK-NEXT: 9223372036854775808: 1
    // CHECK-NEXT: 9223372036854775809: 1
    // CHECK-NEXT: 9223372036854775810: 0
END_FUNCTION
