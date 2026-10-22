// RUN: %COMPILE %s && %RUN | %CHECK %s
// The negation of a DWORD is LINT, so a value from 2^31 on gives the negative value.
FUNCTION main : DINT
    VAR
        dw : DWORD := 16#FFFF_FFFF;
        dw3e9 : DWORD := 16#B2D0_5E00;
        r : LINT;
    END_VAR

    r := -dw;
    printf('-DWORD 16#FFFF_FFFF = %lld$N', r);              // CHECK: -DWORD 16#FFFF_FFFF = -4294967295
    r := -dw3e9;
    printf('-DWORD 16#B2D0_5E00 = %lld$N', r);              // CHECK-NEXT: -DWORD 16#B2D0_5E00 = -3000000000
END_FUNCTION
