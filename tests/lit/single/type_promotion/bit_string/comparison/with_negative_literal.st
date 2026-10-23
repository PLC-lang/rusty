// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A negative literal next to a DWORD or LWORD maximum compares signed, so the maximum reads as -1.
FUNCTION main : DINT
    VAR
        dwmax : DWORD := 16#FFFF_FFFF;
        lwmax : LWORD := 16#FFFF_FFFF_FFFF_FFFF;
    END_VAR

    printf('DWORD max > -1 = %d$N', dwmax > -1);            // CHECK: DWORD max > -1 = 1
    printf('LWORD max > -1 = %d$N', lwmax > -1);            // CHECK-NEXT: LWORD max > -1 = 1
    printf('-1 < DWORD max = %d$N', -1 < dwmax);            // CHECK-NEXT: -1 < DWORD max = 1
END_FUNCTION
