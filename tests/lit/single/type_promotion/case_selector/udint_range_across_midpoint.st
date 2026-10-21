// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A range label that crosses the signed midpoint of the UDINT selector never matches.
FUNCTION branch : DINT
    VAR_INPUT
        x : UDINT;
    END_VAR

    CASE x OF
        0:
            branch := 1;
        1..9:
            branch := 2;
        11..4294967294:
            branch := 3;
        4294967295:
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
    printf('2147483647 -> %d$N', branch(2147483647));           // CHECK-NEXT: 2147483647 -> 3
    printf('2147483648 -> %d$N', branch(2147483648));           // CHECK-NEXT: 2147483648 -> 3
    printf('3000000000 -> %d$N', branch(3000000000));           // CHECK-NEXT: 3000000000 -> 3
    printf('4294967294 -> %d$N', branch(4294967294));           // CHECK-NEXT: 4294967294 -> 3
    printf('4294967295 -> %d$N', branch(4294967295));           // CHECK-NEXT: 4294967295 -> 4
END_FUNCTION
