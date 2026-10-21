// RUN: %COMPILE %s && %RUN | %CHECK %s
// Narrow integer arithmetic whose result is correct in a 32-bit target.
FUNCTION main : DINT
    VAR
        a : USINT := 5;
        b : USINT := 7;
        ua : UINT := 5;
        ub : UINT := 7;
        big : USINT := 200;
        big2 : USINT := 100;
        s7 : SINT := 7;
        i : INT := 300;
        j : INT := 200;
        u2 : UDINT := 2;
        d : DINT;
        k : INT;
        l : LINT;
    END_VAR

    // Unsigned difference stored in DINT
    d := a - b;
    printf('DINT := USINT 5 - USINT 7 = %d$N', d);          // CHECK: DINT := USINT 5 - USINT 7 = -2
    d := ua - ub;
    printf('DINT := UINT 5 - UINT 7 = %d$N', d);            // CHECK-NEXT: DINT := UINT 5 - UINT 7 = -2

    // Results beyond the operand width
    d := big + big2;
    printf('USINT 200 + USINT 100 = %d$N', d);              // CHECK-NEXT: USINT 200 + USINT 100 = 300
    d := i * j;
    printf('DINT := INT 300 * INT 200 = %d$N', d);          // CHECK-NEXT: DINT := INT 300 * INT 200 = 60000
    k := i * j;
    printf('INT := INT 300 * INT 200 = %d$N', k);           // CHECK-NEXT: INT := INT 300 * INT 200 = -5536

    // A signed operand makes the operation signed
    l := a - s7;
    printf('LINT := USINT 5 - SINT 7 = %lld$N', l);         // CHECK-NEXT: LINT := USINT 5 - SINT 7 = -2

    // Two narrow unsigned operands are computed as UDINT, so a negative difference wraps in 32 bits
    printf('USINT 5 - USINT 7 < 0 = %d$N', a - b < 0);      // CHECK-NEXT: USINT 5 - USINT 7 < 0 = 0
    printf('UINT 5 - UINT 7 < 0 = %d$N', ua - ub < 0);      // CHECK-NEXT: UINT 5 - UINT 7 < 0 = 0
    l := (a - b) * 2;
    printf('LINT := (USINT 5 - USINT 7) * 2 = %lld$N', l);  // CHECK-NEXT: LINT := (USINT 5 - USINT 7) * 2 = 4294967292

    // An untyped literal next to a UDINT takes the UDINT type, so the difference wraps in 32 bits
    l := u2 - 3;
    printf('LINT := UDINT 2 - 3 = %lld$N', l);              // CHECK-NEXT: LINT := UDINT 2 - 3 = 4294967295
    l := 1 - u2;
    printf('LINT := 1 - UDINT 2 = %lld$N', l);              // CHECK-NEXT: LINT := 1 - UDINT 2 = 4294967295
    d := u2 - 3;
    printf('DINT := UDINT 2 - 3 = %d$N', d);                // CHECK-NEXT: DINT := UDINT 2 - 3 = -1
END_FUNCTION
