// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// A FOR loop with a DWORD counter runs through the midpoint 2^31 like through any other value.

FUNCTION main
    VAR
        i : DWORD;
        first : DWORD := 16#7FFF_FFFE;
        last : DWORD := 16#8000_0001;
    END_VAR
    FOR i := first TO last DO
        printf('%u$N', i);
    END_FOR
    // CHECK:      2147483646
    // CHECK-NEXT: 2147483647
    // CHECK-NEXT: 2147483648
    // CHECK-NEXT: 2147483649
    // CHECK-NOT:  {{.+}}
END_FUNCTION
