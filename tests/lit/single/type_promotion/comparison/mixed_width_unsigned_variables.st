// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// Two unsigned operands of different widths are promoted to the wider UDINT or ULINT and then compared signed.
FUNCTION main : DINT
    VAR
        us5 : USINT := 5;
        ui5 : UINT := 5;
        ud5 : UDINT := 5;
        us2 : USINT := 2;
        ui2 : UINT := 2;
        ud2 : UDINT := 2;
        udmax : UDINT := 4294967295;
        ulmax : ULINT := 18446744073709551615;
    END_VAR

    // Narrow operand next to a UDINT maximum
    printf('USINT 5 < UDINT max = %d$N', us5 < udmax);      // CHECK: USINT 5 < UDINT max = 1
    printf('UINT 5 < UDINT max = %d$N', ui5 < udmax);       // CHECK-NEXT: UINT 5 < UDINT max = 1
    printf('UDINT max > USINT 2 = %d$N', udmax > us2);      // CHECK-NEXT: UDINT max > USINT 2 = 1
    printf('UDINT max > UINT 2 = %d$N', udmax > ui2);       // CHECK-NEXT: UDINT max > UINT 2 = 1

    // Narrow operand next to a ULINT maximum
    printf('USINT 5 < ULINT max = %d$N', us5 < ulmax);      // CHECK-NEXT: USINT 5 < ULINT max = 1
    printf('ULINT max > UINT 2 = %d$N', ulmax > ui2);       // CHECK-NEXT: ULINT max > UINT 2 = 1

    // UDINT operand next to a ULINT maximum
    printf('UDINT 5 < ULINT max = %d$N', ud5 < ulmax);      // CHECK-NEXT: UDINT 5 < ULINT max = 1
    printf('ULINT max > UDINT 2 = %d$N', ulmax > ud2);      // CHECK-NEXT: ULINT max > UDINT 2 = 1
END_FUNCTION
