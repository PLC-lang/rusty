// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// The negation of a BYTE or WORD runs in the operand's own width, so a value from the signed midpoint on wraps.
FUNCTION main : DINT
    VAR
        b : BYTE := 16#FF;
        b200 : BYTE := 16#C8;
        w : WORD := 16#FFFF;
        w40000 : WORD := 16#9C40;
        r : LINT;
    END_VAR

    r := -b;
    printf('-BYTE 16#FF = %lld$N', r);                      // CHECK: -BYTE 16#FF = -255
    r := -b200;
    printf('-BYTE 16#C8 = %lld$N', r);                      // CHECK-NEXT: -BYTE 16#C8 = -200
    r := -w;
    printf('-WORD 16#FFFF = %lld$N', r);                    // CHECK-NEXT: -WORD 16#FFFF = -65535
    r := -w40000;
    printf('-WORD 16#9C40 = %lld$N', r);                    // CHECK-NEXT: -WORD 16#9C40 = -40000
    printf('-BYTE 16#FF < 0 = %d$N', -b < 0);               // CHECK-NEXT: -BYTE 16#FF < 0 = 1
END_FUNCTION
