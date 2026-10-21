// RUN: %COMPILE %s && %RUN | %CHECK %s
// An untyped integer literal takes the type of a UDINT or ULINT operand, so the compare is unsigned.
FUNCTION main : DINT
    VAR
        ud : UDINT := 4000000000;
        ul : ULINT := 18446744073709551615;
    END_VAR

    // Literal on the right
    printf('UDINT > 5 = %d$N', ud > 5);                     // CHECK: UDINT > 5 = 1
    printf('UDINT >= 2147483647 = %d$N', ud >= 2147483647); // CHECK-NEXT: UDINT >= 2147483647 = 1
    printf('UDINT < 5 = %d$N', ud < 5);                     // CHECK-NEXT: UDINT < 5 = 0
    printf('UDINT < 0 = %d$N', ud < 0);                     // CHECK-NEXT: UDINT < 0 = 0
    printf('ULINT > 1 = %d$N', ul > 1);                     // CHECK-NEXT: ULINT > 1 = 1
    printf('ULINT > 3000000000 = %d$N', ul > 3000000000);   // CHECK-NEXT: ULINT > 3000000000 = 1

    // Literal on the left
    printf('5 < UDINT = %d$N', 5 < ud);                     // CHECK-NEXT: 5 < UDINT = 1
    printf('1 < ULINT = %d$N', 1 < ul);                     // CHECK-NEXT: 1 < ULINT = 1
END_FUNCTION
