// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A negative literal next to a UDINT or ULINT maximum compares signed, so the maximum reads as -1.
FUNCTION main : DINT
    VAR
        udmax : UDINT := 4294967295;
        ulmax : ULINT := 18446744073709551615;
    END_VAR

    // Relational compares
    printf('UDINT max > -1 = %d$N', udmax > -1);            // CHECK: UDINT max > -1 = 1
    printf('ULINT max > -1 = %d$N', ulmax > -1);            // CHECK-NEXT: ULINT max > -1 = 1
    printf('-1 < UDINT max = %d$N', -1 < udmax);            // CHECK-NEXT: -1 < UDINT max = 1
    printf('-1 < ULINT max = %d$N', -1 < ulmax);            // CHECK-NEXT: -1 < ULINT max = 1

    // Equality
    printf('UDINT max = -1 = %d$N', udmax = -1);            // CHECK-NEXT: UDINT max = -1 = 0
    printf('ULINT max = -1 = %d$N', ulmax = -1);            // CHECK-NEXT: ULINT max = -1 = 0
    printf('UDINT max <> -1 = %d$N', udmax <> -1);          // CHECK-NEXT: UDINT max <> -1 = 1
END_FUNCTION
