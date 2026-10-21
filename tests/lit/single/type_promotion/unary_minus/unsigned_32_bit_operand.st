// RUN: %COMPILE %s && %RUN | %CHECK %s
// The negation of a UDINT is LINT, so a value from 2^31 on gives the negative value.
FUNCTION main : DINT
    VAR
        ud : UDINT := 4294967295;
        ud3e9 : UDINT := 3000000000;
        r : LINT;
    END_VAR

    r := -ud;
    printf('-UDINT 4294967295 = %lld$N', r);                // CHECK: -UDINT 4294967295 = -4294967295
    r := -ud3e9;
    printf('-UDINT 3000000000 = %lld$N', r);                // CHECK-NEXT: -UDINT 3000000000 = -3000000000
    printf('-UDINT 3000000000 < 0 = %d$N', -ud3e9 < 0);     // CHECK-NEXT: -UDINT 3000000000 < 0 = 1
END_FUNCTION
