// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// Two unsigned operands compare as unsigned values, also when a value has the highest bit set and
// no wider type is involved: UDINT with UDINT, and ULINT with ULINT or with a positive literal.

FUNCTION main
    VAR
        small_udint : UDINT := 5;
        large_udint : UDINT := 16#8000_0000;
        small_ulint : ULINT := 5;
        large_ulint : ULINT := 16#8000_0000_0000_0000;
    END_VAR
    printf('udint <: %d$N', small_udint < large_udint);
    printf('udint >: %d$N', large_udint > small_udint);
    printf('udint <=: %d$N', large_udint <= small_udint);
    printf('udint >=: %d$N', large_udint >= small_udint);
    // CHECK:      udint <: 1
    // CHECK-NEXT: udint >: 1
    // CHECK-NEXT: udint <=: 0
    // CHECK-NEXT: udint >=: 1

    printf('ulint <: %d$N', small_ulint < large_ulint);
    printf('ulint >: %d$N', large_ulint > small_ulint);
    printf('ulint <=: %d$N', large_ulint <= small_ulint);
    printf('ulint >=: %d$N', large_ulint >= small_ulint);
    // CHECK-NEXT: ulint <: 1
    // CHECK-NEXT: ulint >: 1
    // CHECK-NEXT: ulint <=: 0
    // CHECK-NEXT: ulint >=: 1

    printf('ulint > 5: %d$N', large_ulint > 5);
    // CHECK-NEXT: ulint > 5: 1
END_FUNCTION
