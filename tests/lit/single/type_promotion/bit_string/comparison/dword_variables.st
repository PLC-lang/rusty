// RUN: %COMPILE %s && %RUN | %CHECK %s
// A relational compare of two DWORD values is unsigned, so a value from the signed midpoint on compares as large.
FUNCTION main : DINT
    VAR
        hi : DWORD := 16#FFFF_FFFF;
        lo : DWORD := 16#1;
        mid : DWORD := 16#8000_0000;
        below : DWORD := 16#7FFF_FFFF;
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
    printf('hi > DWORD#16#1 = %d$N', hi > DWORD#16#1);  // CHECK-NEXT: hi > DWORD#16#1 = 1

    // The compare as a stored value and as a condition
    result := hi > lo;
    printf('stored = %d$N', result);               // CHECK-NEXT: stored = 1
    IF hi > lo THEN
        printf('IF taken$N');                      // CHECK-NEXT: IF taken
    ELSE
        printf('IF not taken$N');
    END_IF
END_FUNCTION
