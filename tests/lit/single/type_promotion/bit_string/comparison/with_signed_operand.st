// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A signed operand next to a DWORD or LWORD makes the compare signed, so a bit string from the signed midpoint on compares as negative.
FUNCTION main : DINT
    VAR
        i5 : INT := 5;
        im1 : INT := -1;
        dm1 : DINT := -1;
        lm1 : LINT := -1;
        i2 : INT := 2;
        l2 : LINT := 2;
        dwmax : DWORD := 16#FFFF_FFFF;
        lwmax : LWORD := 16#FFFF_FFFF_FFFF_FFFF;
    END_VAR

    // Signed value below a bit string maximum
    printf('INT 5 < DWORD max = %d$N', i5 < dwmax);         // CHECK: INT 5 < DWORD max = 1
    printf('INT -1 < DWORD max = %d$N', im1 < dwmax);       // CHECK-NEXT: INT -1 < DWORD max = 1
    printf('LINT -1 < LWORD max = %d$N', lm1 < lwmax);      // CHECK-NEXT: LINT -1 < LWORD max = 1

    // Bit string maximum on the left
    printf('DWORD max > INT 2 = %d$N', dwmax > i2);         // CHECK-NEXT: DWORD max > INT 2 = 1
    printf('LWORD max > DINT -1 = %d$N', lwmax > dm1);      // CHECK-NEXT: LWORD max > DINT -1 = 1
    printf('LWORD max > LINT 2 = %d$N', lwmax > l2);        // CHECK-NEXT: LWORD max > LINT 2 = 1
END_FUNCTION
