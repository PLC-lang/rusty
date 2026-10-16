// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// A negative integer literal does not fit into an unsigned type, so a division or MOD of a UDINT by
// it still runs in DINT and gives a signed result.

FUNCTION main
    VAR
        value : UDINT := 10;
    END_VAR
    printf('value / -2: %d$N', value / -2);
    printf('value MOD -3: %d$N', value MOD -3);
    // CHECK:      value / -2: -5
    // CHECK-NEXT: value MOD -3: 1
END_FUNCTION
