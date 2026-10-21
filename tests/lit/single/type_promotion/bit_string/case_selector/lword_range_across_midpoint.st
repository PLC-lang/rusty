// RUN: %COMPILE %s && %RUN | %CHECK %s
// A range label that crosses the signed midpoint of the LWORD selector matches every value between its bounds.
FUNCTION branch : DINT
    VAR_INPUT
        x : LWORD;
    END_VAR

    CASE x OF
        16#0:
            branch := 1;
        16#1..16#9:
            branch := 2;
        16#B..16#FFFF_FFFF_FFFF_FFFE:
            branch := 3;
        16#FFFF_FFFF_FFFF_FFFF:
            branch := 4;
    ELSE
        branch := 9;
    END_CASE
END_FUNCTION

FUNCTION main : DINT
    printf('16#0 -> %d$N', branch(16#0));                       // CHECK: 16#0 -> 1
    printf('16#5 -> %d$N', branch(16#5));                       // CHECK-NEXT: 16#5 -> 2
    printf('16#A -> %d$N', branch(16#A));                       // CHECK-NEXT: 16#A -> 9
    printf('16#B -> %d$N', branch(16#B));                       // CHECK-NEXT: 16#B -> 3
    printf('16#7FFF_FFFF_FFFF_FFFF -> %d$N', branch(16#7FFF_FFFF_FFFF_FFFF));  // CHECK-NEXT: 16#7FFF_FFFF_FFFF_FFFF -> 3
    printf('16#8000_0000_0000_0000 -> %d$N', branch(16#8000_0000_0000_0000));  // CHECK-NEXT: 16#8000_0000_0000_0000 -> 3
    printf('16#A000_0000_0000_0000 -> %d$N', branch(16#A000_0000_0000_0000));  // CHECK-NEXT: 16#A000_0000_0000_0000 -> 3
    printf('16#FFFF_FFFF_FFFF_FFFE -> %d$N', branch(16#FFFF_FFFF_FFFF_FFFE));  // CHECK-NEXT: 16#FFFF_FFFF_FFFF_FFFE -> 3
    printf('16#FFFF_FFFF_FFFF_FFFF -> %d$N', branch(16#FFFF_FFFF_FFFF_FFFF));  // CHECK-NEXT: 16#FFFF_FFFF_FFFF_FFFF -> 4
END_FUNCTION
