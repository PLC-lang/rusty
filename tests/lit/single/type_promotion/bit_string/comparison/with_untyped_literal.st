// RUN: %COMPILE %s && %RUN | %CHECK %s
// An untyped integer literal takes the type of a DWORD or LWORD operand, so the compare is unsigned.
FUNCTION main : DINT
    VAR
        dw : DWORD := 16#FFFF_FFFF;
        lw : LWORD := 16#FFFF_FFFF_FFFF_FFFF;
    END_VAR

    // Literal on the right
    printf('DWORD > 1 = %d$N', dw > 1);                     // CHECK: DWORD > 1 = 1
    printf('DWORD > 16#7FFF_FFFF = %d$N', dw > 16#7FFF_FFFF);   // CHECK-NEXT: DWORD > 16#7FFF_FFFF = 1
    printf('DWORD < 0 = %d$N', dw < 0);                     // CHECK-NEXT: DWORD < 0 = 0
    printf('LWORD > 1 = %d$N', lw > 1);                     // CHECK-NEXT: LWORD > 1 = 1
    printf('LWORD > 3000000000 = %d$N', lw > 3000000000);   // CHECK-NEXT: LWORD > 3000000000 = 1

    // Literal on the left
    printf('1 < DWORD = %d$N', 1 < dw);                     // CHECK-NEXT: 1 < DWORD = 1
    printf('1 < LWORD = %d$N', 1 < lw);                     // CHECK-NEXT: 1 < LWORD = 1
END_FUNCTION
