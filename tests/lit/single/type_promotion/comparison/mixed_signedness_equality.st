// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A negative signed operand and a UDINT or ULINT maximum have the same bit pattern, and = and <> compare the bits, not the values.
FUNCTION main : DINT
    VAR
        sm1 : SINT := -1;
        im1 : INT := -1;
        dm1 : DINT := -1;
        lm1 : LINT := -1;
        udmax : UDINT := 4294967295;
        ulmax : ULINT := 18446744073709551615;
    END_VAR

    printf('SINT -1 = UDINT max = %d$N', sm1 = udmax);      // CHECK: SINT -1 = UDINT max = 0
    printf('DINT -1 = UDINT max = %d$N', dm1 = udmax);      // CHECK-NEXT: DINT -1 = UDINT max = 0
    printf('DINT -1 <> UDINT max = %d$N', dm1 <> udmax);    // CHECK-NEXT: DINT -1 <> UDINT max = 1
    printf('INT -1 = ULINT max = %d$N', im1 = ulmax);       // CHECK-NEXT: INT -1 = ULINT max = 0
    printf('LINT -1 = ULINT max = %d$N', lm1 = ulmax);      // CHECK-NEXT: LINT -1 = ULINT max = 0
    printf('ULINT max <> LINT -1 = %d$N', ulmax <> lm1);    // CHECK-NEXT: ULINT max <> LINT -1 = 1
END_FUNCTION
