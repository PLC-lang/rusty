// RUN: %COMPILE %s && %RUN | %CHECK %s
// A range label that crosses the signed midpoint of the ULINT selector matches every value between its bounds.
FUNCTION branch : DINT
    VAR_INPUT
        x : ULINT;
    END_VAR

    CASE x OF
        0:
            branch := 1;
        1..9:
            branch := 2;
        11..18446744073709551614:
            branch := 3;
        18446744073709551615:
            branch := 4;
    ELSE
        branch := 9;
    END_CASE
END_FUNCTION

FUNCTION main : DINT
    printf('0 -> %d$N', branch(0));                             // CHECK: 0 -> 1
    printf('5 -> %d$N', branch(5));                             // CHECK-NEXT: 5 -> 2
    printf('10 -> %d$N', branch(10));                           // CHECK-NEXT: 10 -> 9
    printf('11 -> %d$N', branch(11));                           // CHECK-NEXT: 11 -> 3
    printf('9223372036854775807 -> %d$N', branch(9223372036854775807));  // CHECK-NEXT: 9223372036854775807 -> 3
    printf('9223372036854775808 -> %d$N', branch(9223372036854775808));  // CHECK-NEXT: 9223372036854775808 -> 3
    printf('12000000000000000000 -> %d$N', branch(12000000000000000000));  // CHECK-NEXT: 12000000000000000000 -> 3
    printf('18446744073709551614 -> %d$N', branch(18446744073709551614));  // CHECK-NEXT: 18446744073709551614 -> 3
    printf('18446744073709551615 -> %d$N', branch(18446744073709551615));  // CHECK-NEXT: 18446744073709551615 -> 4
END_FUNCTION
