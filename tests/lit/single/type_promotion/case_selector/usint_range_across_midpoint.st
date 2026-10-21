// RUN: %COMPILE %s && %RUN | %CHECK %s
// A range label that crosses the signed midpoint of the USINT selector matches every value between its bounds.
FUNCTION branch : DINT
    VAR_INPUT
        x : USINT;
    END_VAR

    CASE x OF
        0:
            branch := 1;
        1..9:
            branch := 2;
        11..254:
            branch := 3;
        255:
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
    printf('127 -> %d$N', branch(127));                         // CHECK-NEXT: 127 -> 3
    printf('128 -> %d$N', branch(128));                         // CHECK-NEXT: 128 -> 3
    printf('200 -> %d$N', branch(200));                         // CHECK-NEXT: 200 -> 3
    printf('254 -> %d$N', branch(254));                         // CHECK-NEXT: 254 -> 3
    printf('255 -> %d$N', branch(255));                         // CHECK-NEXT: 255 -> 4
END_FUNCTION
