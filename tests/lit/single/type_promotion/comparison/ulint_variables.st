// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A relational compare of two ULINT values is signed, so a value from the signed midpoint on compares as negative.
FUNCTION main : DINT
    VAR
        hi : ULINT := 18446744073709551615;
        lo : ULINT := 5;
        mid : ULINT := 9223372036854775808;
        below : ULINT := 9223372036854775807;
        result : BOOL;
    END_VAR

    // Every relational operator, both operand orders
    printf('hi > lo = %d$N', hi > lo);             // CHECK: hi > lo = 1
    printf('hi >= lo = %d$N', hi >= lo);           // CHECK-NEXT: hi >= lo = 1
    printf('hi < lo = %d$N', hi < lo);             // CHECK-NEXT: hi < lo = 0
    printf('hi <= lo = %d$N', hi <= lo);           // CHECK-NEXT: hi <= lo = 0
    printf('lo < hi = %d$N', lo < hi);             // CHECK-NEXT: lo < hi = 1
    printf('lo <= hi = %d$N', lo <= hi);           // CHECK-NEXT: lo <= hi = 1
    printf('lo > hi = %d$N', lo > hi);             // CHECK-NEXT: lo > hi = 0
    printf('lo >= hi = %d$N', lo >= hi);           // CHECK-NEXT: lo >= hi = 0

    // The two values next to the signed midpoint
    printf('mid > below = %d$N', mid > below);     // CHECK-NEXT: mid > below = 1

    // A typed literal operand of the same type
    printf('hi > ULINT#5 = %d$N', hi > ULINT#5);  // CHECK-NEXT: hi > ULINT#5 = 1

    // The compare as a stored value and as a condition
    result := hi > lo;
    printf('stored = %d$N', result);               // CHECK-NEXT: stored = 1
    IF hi > lo THEN
        printf('IF taken$N');                      // CHECK-NEXT: IF taken
    ELSE
        printf('IF not taken$N');
    END_IF
END_FUNCTION
