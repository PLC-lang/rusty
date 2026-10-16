// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// A FOR loop with a UDINT counter and a negative step counts down through the midpoint 2^31.

FUNCTION main
    VAR
        i : UDINT;
    END_VAR
    FOR i := 16#8000_0001 TO 16#7FFF_FFFE BY -1 DO
        printf('%u$N', i);
    END_FOR
    // CHECK:      2147483649
    // CHECK-NEXT: 2147483648
    // CHECK-NEXT: 2147483647
    // CHECK-NEXT: 2147483646
    // CHECK-NOT:  {{.+}}
END_FUNCTION
