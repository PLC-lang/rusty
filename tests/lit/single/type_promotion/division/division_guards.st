// RUN: %COMPILE %s && %RUN | %CHECK %s
// Divisions and MOD whose result does not depend on the type of the literal.
FUNCTION main : DINT
    VAR
        a : UDINT := 4000000000;
        two : UDINT := 2;
        seven : UDINT := 7;
        ul : ULINT := 18446744073709551615;
        ul2 : ULINT := 2;
        us : USINT := 200;
        ui : UINT := 60000;
        small : UDINT := 100;
        neg : DINT := -7;
        s5 : SINT := 5;
        d2 : DINT := 2;
        ud7 : UDINT := 7;
        q : UDINT;
        ql : ULINT;
        qs : LINT;
        d : DINT;
    END_VAR

    // Two unsigned variables
    q := a / two;
    printf('UDINT 4e9 / UDINT 2 = %u$N', q);                // CHECK: UDINT 4e9 / UDINT 2 = 2000000000
    q := a MOD seven;
    printf('UDINT 4e9 MOD UDINT 7 = %u$N', q);              // CHECK-NEXT: UDINT 4e9 MOD UDINT 7 = 3
    ql := ul / ul2;
    printf('ULINT max / ULINT 2 = %llu$N', ql);             // CHECK-NEXT: ULINT max / ULINT 2 = 9223372036854775807

    // A typed literal of the same type
    q := a / UDINT#2;
    printf('UDINT 4e9 / UDINT#2 = %u$N', q);                // CHECK-NEXT: UDINT 4e9 / UDINT#2 = 2000000000

    // ULINT operands by an untyped literal
    ql := ul / 2;
    printf('ULINT max / 2 = %llu$N', ql);                   // CHECK-NEXT: ULINT max / 2 = 9223372036854775807
    ql := ul MOD 7;
    printf('ULINT max MOD 7 = %llu$N', ql);                 // CHECK-NEXT: ULINT max MOD 7 = 1

    // A literal dividend too large for DINT is LINT
    qs := 4000000000 / two;
    printf('4000000000 / UDINT 2 = %lld$N', qs);            // CHECK-NEXT: 4000000000 / UDINT 2 = 2000000000

    // Values below the signed midpoint
    q := small / 3;
    printf('UDINT 100 / 3 = %u$N', q);                      // CHECK-NEXT: UDINT 100 / 3 = 33
    d := us / 3;
    printf('USINT 200 / 3 = %d$N', d);                      // CHECK-NEXT: USINT 200 / 3 = 66
    d := ui MOD 7;
    printf('UINT 60000 MOD 7 = %d$N', d);                   // CHECK-NEXT: UINT 60000 MOD 7 = 3

    // Signed division cuts towards zero
    d := neg / 2;
    printf('-7 / 2 = %d$N', d);                             // CHECK-NEXT: -7 / 2 = -3
    d := neg MOD 2;
    printf('-7 MOD 2 = %d$N', d);                           // CHECK-NEXT: -7 MOD 2 = -1

    // Mixed signedness and negative literals with non-negative or small operands
    qs := s5 / ul;
    printf('SINT 5 / ULINT max = %lld$N', qs);              // CHECK-NEXT: SINT 5 / ULINT max = 0
    qs := ul / d2;
    printf('ULINT max / DINT 2 = %lld$N', qs);              // CHECK-NEXT: ULINT max / DINT 2 = 9223372036854775807
    qs := ud7 / -2;
    printf('UDINT 7 / -2 = %lld$N', qs);                    // CHECK-NEXT: UDINT 7 / -2 = -3
END_FUNCTION
