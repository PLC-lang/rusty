// RUN: %COMPILE %s && %RUN | %CHECK %s
// A FOR loop over a property computes the next value in the property's type, as it does for a variable.
FUNCTION_BLOCK fb
VAR
    real_value : REAL;
    big_value : ULINT;
    small_value : USINT;
END_VAR
    PROPERTY_GET r : REAL
        r := real_value;
    END_PROPERTY
    PROPERTY_SET r : REAL
        real_value := r;
    END_PROPERTY

    PROPERTY_GET big : ULINT
        big := big_value;
    END_PROPERTY
    PROPERTY_SET big : ULINT
        big_value := big;
    END_PROPERTY

    PROPERTY_GET small : USINT
        small := small_value;
    END_PROPERTY
    PROPERTY_SET small : USINT
        small_value := small;
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        f : fb;
        n : DINT;
    END_VAR

    n := 0;
    FOR f.r := 0.0 TO 1.0 BY 0.25 DO
        n := n + 1;
        IF n > 100 THEN EXIT; END_IF
    END_FOR
    printf('REAL property 0.0 TO 1.0 BY 0.25 = %d, after = %.2f$N', n, f.r);
    // CHECK: REAL property 0.0 TO 1.0 BY 0.25 = 5, after = 1.25

    n := 0;
    FOR f.big := 9223372036854775806 TO 9223372036854775810 DO
        n := n + 1;
        IF n > 100 THEN EXIT; END_IF
    END_FOR
    printf('ULINT property across 2^63 = %d$N', n);
    // CHECK-NEXT: ULINT property across 2^63 = 5

    n := 0;
    FOR f.small := 10 TO 1 BY -3 DO
        n := n + 1;
        IF n > 100 THEN EXIT; END_IF
    END_FOR
    printf('USINT property 10 TO 1 BY -3 = %d, after = %d$N', n, f.small);
    // CHECK-NEXT: USINT property 10 TO 1 BY -3 = 4, after = 254
END_FUNCTION
