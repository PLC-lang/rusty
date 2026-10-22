// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A signed operand next to a UDINT makes the division signed, and next to a ULINT it makes the division unsigned,
// so one of the two operands is read with the wrong sign.
FUNCTION main : DINT
    VAR
        d5 : DINT := 5;
        sm1 : SINT := -1;
        dm1 : DINT := -1;
        lm1 : LINT := -1;
        s2 : SINT := 2;
        d2 : DINT := 2;
        ul2 : ULINT := 2;
        udmax : UDINT := 4294967295;
        ulmax : ULINT := 18446744073709551615;
        r : LINT;
    END_VAR

    // Signed operand next to a UDINT operand
    r := udmax / d2;
    printf('UDINT max / DINT 2 = %lld$N', r);               // CHECK: UDINT max / DINT 2 = 2147483647
    r := udmax MOD s2;
    printf('UDINT max MOD SINT 2 = %lld$N', r);             // CHECK-NEXT: UDINT max MOD SINT 2 = 1
    r := d5 / udmax;
    printf('DINT 5 / UDINT max = %lld$N', r);               // CHECK-NEXT: DINT 5 / UDINT max = 0

    // Signed operand next to a ULINT operand
    r := dm1 / ul2;
    printf('DINT -1 / ULINT 2 = %lld$N', r);                // CHECK-NEXT: DINT -1 / ULINT 2 = 0
    r := sm1 MOD ul2;
    printf('SINT -1 MOD ULINT 2 = %lld$N', r);              // CHECK-NEXT: SINT -1 MOD ULINT 2 = -1
    r := lm1 / ulmax;
    printf('LINT -1 / ULINT max = %lld$N', r);              // CHECK-NEXT: LINT -1 / ULINT max = 0
END_FUNCTION
