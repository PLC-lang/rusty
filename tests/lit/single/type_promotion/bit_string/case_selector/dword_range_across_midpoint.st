// RUN: %COMPILE %s && %RUN | %CHECK %s
// A range label that crosses the signed midpoint of the DWORD selector matches every value between its bounds.
FUNCTION branch : DINT
    VAR_INPUT
        x : DWORD;
    END_VAR

    CASE x OF
        16#0000_0000:
            branch := 1;
        16#0000_0001..16#0000_0009:
            branch := 2;
        16#0000_000B..16#FFFF_FFFE:
            branch := 3;
        16#FFFF_FFFF:
            branch := 4;
    ELSE
        branch := 9;
    END_CASE
END_FUNCTION

FUNCTION main : DINT
    printf('16#0000_0000 -> %d$N', branch(16#0000_0000));       // CHECK: 16#0000_0000 -> 1
    printf('16#0000_0005 -> %d$N', branch(16#0000_0005));       // CHECK-NEXT: 16#0000_0005 -> 2
    printf('16#0000_000A -> %d$N', branch(16#0000_000A));       // CHECK-NEXT: 16#0000_000A -> 9
    printf('16#0000_000B -> %d$N', branch(16#0000_000B));       // CHECK-NEXT: 16#0000_000B -> 3
    printf('16#7FFF_FFFF -> %d$N', branch(16#7FFF_FFFF));       // CHECK-NEXT: 16#7FFF_FFFF -> 3
    printf('16#8000_0000 -> %d$N', branch(16#8000_0000));       // CHECK-NEXT: 16#8000_0000 -> 3
    printf('16#B2D0_5E00 -> %d$N', branch(16#B2D0_5E00));       // CHECK-NEXT: 16#B2D0_5E00 -> 3
    printf('16#FFFF_FFFE -> %d$N', branch(16#FFFF_FFFE));       // CHECK-NEXT: 16#FFFF_FFFE -> 3
    printf('16#FFFF_FFFF -> %d$N', branch(16#FFFF_FFFF));       // CHECK-NEXT: 16#FFFF_FFFF -> 4
END_FUNCTION
