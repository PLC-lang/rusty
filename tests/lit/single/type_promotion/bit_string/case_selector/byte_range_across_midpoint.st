// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A range label that crosses the signed midpoint of the BYTE selector never matches.
FUNCTION branch : DINT
    VAR_INPUT
        x : BYTE;
    END_VAR

    CASE x OF
        16#00:
            branch := 1;
        16#01..16#09:
            branch := 2;
        16#0B..16#FE:
            branch := 3;
        16#FF:
            branch := 4;
    ELSE
        branch := 9;
    END_CASE
END_FUNCTION

FUNCTION main : DINT
    printf('16#00 -> %d$N', branch(16#00));                     // CHECK: 16#00 -> 1
    printf('16#05 -> %d$N', branch(16#05));                     // CHECK-NEXT: 16#05 -> 2
    printf('16#0A -> %d$N', branch(16#0A));                     // CHECK-NEXT: 16#0A -> 9
    printf('16#0B -> %d$N', branch(16#0B));                     // CHECK-NEXT: 16#0B -> 3
    printf('16#7F -> %d$N', branch(16#7F));                     // CHECK-NEXT: 16#7F -> 3
    printf('16#80 -> %d$N', branch(16#80));                     // CHECK-NEXT: 16#80 -> 3
    printf('16#C8 -> %d$N', branch(16#C8));                     // CHECK-NEXT: 16#C8 -> 3
    printf('16#FE -> %d$N', branch(16#FE));                     // CHECK-NEXT: 16#FE -> 3
    printf('16#FF -> %d$N', branch(16#FF));                     // CHECK-NEXT: 16#FF -> 4
END_FUNCTION
