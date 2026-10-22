// RUN: %COMPILE %s && %RUN | %CHECK %s
// Bit string compares whose result does not depend on the signedness of the predicate.
FUNCTION main : DINT
    VAR
        dw : DWORD := 16#FFFF_FFFF;
        lw : LWORD := 16#FFFF_FFFF_FFFF_FFFF;
        b : BYTE := 16#FF;
        w : WORD := 16#FFFF;
        im1 : INT := -1;
    END_VAR

    // Equality compares the bit pattern
    printf('DWORD 16#FFFF_FFFF = -1 = %d$N', dw = -1);      // CHECK: DWORD 16#FFFF_FFFF = -1 = 1
    printf('LWORD max = 16#FFFF_FFFF_FFFF_FFFF = %d$N', lw = 16#FFFF_FFFF_FFFF_FFFF);    // CHECK-NEXT: LWORD max = 16#FFFF_FFFF_FFFF_FFFF = 1

    // A narrow bit string next to an untyped literal is widened to DINT
    printf('BYTE 16#FF > 1 = %d$N', b > 1);                 // CHECK-NEXT: BYTE 16#FF > 1 = 1
    printf('WORD 16#FFFF > 1 = %d$N', w > 1);               // CHECK-NEXT: WORD 16#FFFF > 1 = 1

    // Results that hold under either predicate
    printf('INT -1 > DWORD max = %d$N', im1 > dw);          // CHECK-NEXT: INT -1 > DWORD max = 0
END_FUNCTION
