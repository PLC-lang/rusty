// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// Two narrow unsigned operands are computed as UDINT, so a negative difference is zero-extended into a 64-bit or real target.
FUNCTION show_lint : DINT
    VAR_INPUT
        v : LINT;
    END_VAR
    printf('LINT parameter = %lld$N', v);
END_FUNCTION

FUNCTION show_lreal : DINT
    VAR_INPUT
        v : LREAL;
    END_VAR
    printf('LREAL parameter = %f$N', v);
END_FUNCTION

FUNCTION main : DINT
    VAR
        a : USINT := 5;
        b : USINT := 7;
        ua : UINT := 5;
        ub : UINT := 7;
        lz : LINT := 0;
        l : LINT;
        r : LREAL;
        rr : REAL;
    END_VAR

    // Assignment to a 64-bit or real target
    l := a - b;
    printf('LINT := USINT 5 - USINT 7 = %lld$N', l);        // CHECK: LINT := USINT 5 - USINT 7 = -2
    l := ua - ub;
    printf('LINT := UINT 5 - UINT 7 = %lld$N', l);          // CHECK-NEXT: LINT := UINT 5 - UINT 7 = -2
    printf('LINT := (USINT 5 - USINT 7) * 2 = %lld$N', l);  // CHECK-NEXT: LINT := (USINT 5 - USINT 7) * 2 = -4
    r := a - b;
    printf('LREAL := USINT 5 - USINT 7 = %f$N', r);         // CHECK-NEXT: LREAL := USINT 5 - USINT 7 = -2.000000
    rr := ua - ub;
    printf('REAL := UINT 5 - UINT 7 = %f$N', rr);           // CHECK-NEXT: REAL := UINT 5 - UINT 7 = -2.000000

    // Compare with a 64-bit operand
    printf('USINT 5 - USINT 7 < LINT 0 = %d$N', a - b < lz);    // CHECK-NEXT: USINT 5 - USINT 7 < LINT 0 = 1
    printf('UINT 5 - UINT 7 < LINT 0 = %d$N', ua - ub < lz);    // CHECK-NEXT: UINT 5 - UINT 7 < LINT 0 = 1

    // Argument for a 64-bit or real parameter
    show_lint(a - b);                                       // CHECK-NEXT: LINT parameter = -2
    show_lreal(ua - ub);                                    // CHECK-NEXT: LREAL parameter = -2.000000
END_FUNCTION
