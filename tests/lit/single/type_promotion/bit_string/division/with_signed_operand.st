// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A signed operand next to a DWORD makes the division signed, and next to an LWORD the bit string is read as negative,
// so a bit string from the signed midpoint on gives a wrong quotient.
FUNCTION main : DINT
    VAR
        i5 : INT := 5;
        i2 : INT := 2;
        l2 : LINT := 2;
        dwmax : DWORD := 16#FFFF_FFFF;
        lwmax : LWORD := 16#FFFF_FFFF_FFFF_FFFF;
        r : LINT;
    END_VAR

    r := dwmax / i2;
    printf('DWORD max / INT 2 = %lld$N', r);                // CHECK: DWORD max / INT 2 = 2147483647
    r := i5 MOD dwmax;
    printf('INT 5 MOD DWORD max = %lld$N', r);              // CHECK-NEXT: INT 5 MOD DWORD max = 5
    r := lwmax / l2;
    printf('LWORD max / LINT 2 = %lld$N', r);               // CHECK-NEXT: LWORD max / LINT 2 = 9223372036854775807
END_FUNCTION
