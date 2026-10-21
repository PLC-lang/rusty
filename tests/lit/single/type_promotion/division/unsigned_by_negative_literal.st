// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A negative literal next to a UDINT makes the division signed in 32 bits, and next to a ULINT it is read as unsigned.
FUNCTION main : DINT
    VAR
        udmax : UDINT := 4294967295;
        ul5 : ULINT := 5;
        r : LINT;
    END_VAR

    // UDINT dividend
    r := udmax / -2;
    printf('UDINT max / -2 = %lld$N', r);                   // CHECK: UDINT max / -2 = -2147483647
    r := udmax MOD -2;
    printf('UDINT max MOD -2 = %lld$N', r);                 // CHECK-NEXT: UDINT max MOD -2 = 1

    // ULINT dividend
    r := ul5 / -2;
    printf('ULINT 5 / -2 = %lld$N', r);                     // CHECK-NEXT: ULINT 5 / -2 = -2
END_FUNCTION
