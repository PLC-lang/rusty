// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A range label that crosses the signed midpoint of the WORD selector never matches.
FUNCTION branch : DINT
    VAR_INPUT
        x : WORD;
    END_VAR

    CASE x OF
        16#0000:
            branch := 1;
        16#0001..16#0009:
            branch := 2;
        16#000B..16#FFFE:
            branch := 3;
        16#FFFF:
            branch := 4;
    ELSE
        branch := 9;
    END_CASE
END_FUNCTION

FUNCTION main : DINT
    printf('16#0000 -> %d$N', branch(16#0000));                 // CHECK: 16#0000 -> 1
    printf('16#0005 -> %d$N', branch(16#0005));                 // CHECK-NEXT: 16#0005 -> 2
    printf('16#000A -> %d$N', branch(16#000A));                 // CHECK-NEXT: 16#000A -> 9
    printf('16#000B -> %d$N', branch(16#000B));                 // CHECK-NEXT: 16#000B -> 3
    printf('16#7FFF -> %d$N', branch(16#7FFF));                 // CHECK-NEXT: 16#7FFF -> 3
    printf('16#8000 -> %d$N', branch(16#8000));                 // CHECK-NEXT: 16#8000 -> 3
    printf('16#9C40 -> %d$N', branch(16#9C40));                 // CHECK-NEXT: 16#9C40 -> 3
    printf('16#FFFE -> %d$N', branch(16#FFFE));                 // CHECK-NEXT: 16#FFFE -> 3
    printf('16#FFFF -> %d$N', branch(16#FFFF));                 // CHECK-NEXT: 16#FFFF -> 4
END_FUNCTION
