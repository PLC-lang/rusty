// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// Addition, subtraction and multiplication of a UDINT and an integer literal still run in DINT, so a
// subtraction below zero gives a negative result instead of wrapping around.

FUNCTION main
    VAR
        value : UDINT := 5;
        wide : LINT;
    END_VAR
    wide := value - 10;
    printf('value - 10: %lld$N', wide);
    printf('value - 10 < 0: %d$N', value - 10 < 0);
    // CHECK:      value - 10: -5
    // CHECK-NEXT: value - 10 < 0: 1
END_FUNCTION
