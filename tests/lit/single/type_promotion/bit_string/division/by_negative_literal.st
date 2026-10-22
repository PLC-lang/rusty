// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A negative literal next to a DWORD makes the division signed in 32 bits, and next to an LWORD it is read as unsigned.
FUNCTION main : DINT
    VAR
        dwmax : DWORD := 16#FFFF_FFFF;
        lw5 : LWORD := 16#5;
        r : LINT;
    END_VAR

    r := dwmax / -2;
    printf('DWORD max / -2 = %lld$N', r);                   // CHECK: DWORD max / -2 = -2147483647
    r := lw5 / -2;
    printf('LWORD 5 / -2 = %lld$N', r);                     // CHECK-NEXT: LWORD 5 / -2 = -2
END_FUNCTION
