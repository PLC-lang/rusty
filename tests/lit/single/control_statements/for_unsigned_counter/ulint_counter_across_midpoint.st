// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// A FOR loop with a ULINT counter runs through the midpoint 2^63 like through any other value.

FUNCTION main
    VAR
        i : ULINT;
        first : ULINT := 16#7FFF_FFFF_FFFF_FFFE;
        last : ULINT := 16#8000_0000_0000_0001;
    END_VAR
    FOR i := first TO last DO
        printf('%llu$N', i);
    END_FOR
    // CHECK:      9223372036854775806
    // CHECK-NEXT: 9223372036854775807
    // CHECK-NEXT: 9223372036854775808
    // CHECK-NEXT: 9223372036854775809
    // CHECK-NOT:  {{.+}}
END_FUNCTION
