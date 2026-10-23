// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A signed operand next to a UDINT or ULINT operand of the same or larger width makes the compare signed,
// so an unsigned value from the signed midpoint on compares as negative.
FUNCTION main : DINT
    VAR
        s5 : SINT := 5;
        d5 : DINT := 5;
        l5 : LINT := 5;
        sm1 : SINT := -1;
        dm1 : DINT := -1;
        lm1 : LINT := -1;
        i2 : INT := 2;
        d2 : DINT := 2;
        l2 : LINT := 2;
        udmax : UDINT := 4294967295;
        ulmax : ULINT := 18446744073709551615;
    END_VAR

    // Signed value below a UDINT maximum
    printf('SINT 5 < UDINT max = %d$N', s5 < udmax);        // CHECK: SINT 5 < UDINT max = 1
    printf('DINT 5 < UDINT max = %d$N', d5 < udmax);        // CHECK-NEXT: DINT 5 < UDINT max = 1
    printf('DINT -1 < UDINT max = %d$N', dm1 < udmax);      // CHECK-NEXT: DINT -1 < UDINT max = 1

    // Signed value below a ULINT maximum
    printf('DINT 5 < ULINT max = %d$N', d5 < ulmax);        // CHECK-NEXT: DINT 5 < ULINT max = 1
    printf('LINT 5 < ULINT max = %d$N', l5 < ulmax);        // CHECK-NEXT: LINT 5 < ULINT max = 1
    printf('LINT -1 < ULINT max = %d$N', lm1 < ulmax);      // CHECK-NEXT: LINT -1 < ULINT max = 1
    printf('SINT -1 < ULINT max = %d$N', sm1 < ulmax);      // CHECK-NEXT: SINT -1 < ULINT max = 1

    // Unsigned maximum on the left
    printf('UDINT max > DINT 2 = %d$N', udmax > d2);        // CHECK-NEXT: UDINT max > DINT 2 = 1
    printf('UDINT max > SINT -1 = %d$N', udmax > sm1);      // CHECK-NEXT: UDINT max > SINT -1 = 1
    printf('ULINT max > LINT 2 = %d$N', ulmax > l2);        // CHECK-NEXT: ULINT max > LINT 2 = 1
    printf('ULINT max > DINT -1 = %d$N', ulmax > dm1);      // CHECK-NEXT: ULINT max > DINT -1 = 1
    printf('ULINT max < INT 2 = %d$N', ulmax < i2);         // CHECK-NEXT: ULINT max < INT 2 = 0
END_FUNCTION
