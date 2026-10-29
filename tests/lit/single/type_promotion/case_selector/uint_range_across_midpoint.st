// RUN: %COMPILE %s && %RUN | %CHECK %s
// A range label that crosses the signed midpoint of the UINT selector matches every value between its bounds.
FUNCTION branch : DINT
    VAR_INPUT
        x : UINT;
    END_VAR

    CASE x OF
        0:
            branch := 1;
        1..9:
            branch := 2;
        11..65534:
            branch := 3;
        65535:
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
    printf('32767 -> %d$N', branch(32767));                     // CHECK-NEXT: 32767 -> 3
    printf('32768 -> %d$N', branch(32768));                     // CHECK-NEXT: 32768 -> 3
    printf('40000 -> %d$N', branch(40000));                     // CHECK-NEXT: 40000 -> 3
    printf('65534 -> %d$N', branch(65534));                     // CHECK-NEXT: 65534 -> 3
    printf('65535 -> %d$N', branch(65535));                     // CHECK-NEXT: 65535 -> 4
END_FUNCTION
