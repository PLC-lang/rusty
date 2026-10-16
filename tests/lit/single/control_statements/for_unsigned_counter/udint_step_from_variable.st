// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// A FOR loop with a UDINT counter and a UDINT step variable runs through the midpoint 2^31.

FUNCTION main
    VAR
        i : UDINT;
        first : UDINT := 16#7FFF_FFFE;
        last : UDINT := 16#8000_0002;
        step : UDINT := 2;
    END_VAR
    FOR i := first TO last BY step DO
        printf('%u$N', i);
    END_FOR
    // CHECK:      2147483646
    // CHECK-NEXT: 2147483648
    // CHECK-NEXT: 2147483650
    // CHECK-NOT:  {{.+}}
END_FUNCTION
