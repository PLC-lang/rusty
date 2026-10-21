// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A bit string next to an unsigned integer or another bit string of a different width is promoted to the wider 32-bit or 64-bit type
// and then compared signed.
FUNCTION main : DINT
    VAR
        us5 : USINT := 5;
        ui5 : UINT := 5;
        b5 : BYTE := 16#05;
        w5 : WORD := 16#0005;
        ud5 : UDINT := 5;
        dw5 : DWORD := 16#5;
        us2 : USINT := 2;
        ui2 : UINT := 2;
        ud2 : UDINT := 2;
        dw2 : DWORD := 16#2;
        lw2 : LWORD := 16#2;
        udmax : UDINT := 4294967295;
        dwmax : DWORD := 16#FFFF_FFFF;
        ulmax : ULINT := 18446744073709551615;
        lwmax : LWORD := 16#FFFF_FFFF_FFFF_FFFF;
    END_VAR

    // Narrow operand next to a 32-bit maximum
    printf('UINT 5 < DWORD max = %d$N', ui5 < dwmax);       // CHECK: UINT 5 < DWORD max = 1
    printf('BYTE 5 < UDINT max = %d$N', b5 < udmax);        // CHECK-NEXT: BYTE 5 < UDINT max = 1
    printf('WORD 5 < DWORD max = %d$N', w5 < dwmax);        // CHECK-NEXT: WORD 5 < DWORD max = 1
    printf('DWORD max > UINT 2 = %d$N', dwmax > ui2);       // CHECK-NEXT: DWORD max > UINT 2 = 1

    // Narrow operand next to a 64-bit maximum
    printf('BYTE 5 < LWORD max = %d$N', b5 < lwmax);        // CHECK-NEXT: BYTE 5 < LWORD max = 1
    printf('LWORD max > USINT 2 = %d$N', lwmax > us2);      // CHECK-NEXT: LWORD max > USINT 2 = 1

    // 32-bit operand next to a 64-bit maximum
    printf('DWORD 5 < LWORD max = %d$N', dw5 < lwmax);      // CHECK-NEXT: DWORD 5 < LWORD max = 1
    printf('UDINT 5 < LWORD max = %d$N', ud5 < lwmax);      // CHECK-NEXT: UDINT 5 < LWORD max = 1
    printf('ULINT max > DWORD 2 = %d$N', ulmax > dw2);      // CHECK-NEXT: ULINT max > DWORD 2 = 1
    printf('USINT 5 < DWORD max = %d$N', us5 < dwmax);      // CHECK-NEXT: USINT 5 < DWORD max = 1

    // Unsigned integer next to a bit string of the same width
    printf('UDINT max > DWORD 2 = %d$N', udmax > dw2);      // CHECK-NEXT: UDINT max > DWORD 2 = 1
    printf('DWORD max > UDINT 2 = %d$N', dwmax > ud2);      // CHECK-NEXT: DWORD max > UDINT 2 = 1
    printf('ULINT max > LWORD 2 = %d$N', ulmax > lw2);      // CHECK-NEXT: ULINT max > LWORD 2 = 1
END_FUNCTION
