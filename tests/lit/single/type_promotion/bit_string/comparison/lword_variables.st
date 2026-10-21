// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A relational compare of two LWORD values is signed, so a value from the signed midpoint on compares as negative.
FUNCTION main : DINT
    VAR
        hi : LWORD := 16#FFFF_FFFF_FFFF_FFFF;
        lo : LWORD := 16#1;
        mid : LWORD := 16#8000_0000_0000_0000;
        below : LWORD := 16#7FFF_FFFF_FFFF_FFFF;
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
    printf('hi > LWORD#16#1 = %d$N', hi > LWORD#16#1);  // CHECK-NEXT: hi > LWORD#16#1 = 1

    // The compare as a stored value and as a condition
    result := hi > lo;
    printf('stored = %d$N', result);               // CHECK-NEXT: stored = 1
    IF hi > lo THEN
        printf('IF taken$N');                      // CHECK-NEXT: IF taken
    ELSE
        printf('IF not taken$N');
    END_IF
END_FUNCTION
