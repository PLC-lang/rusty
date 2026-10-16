// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// An integer literal as the left operand of a division or MOD takes the type of an unsigned divisor of
// 32 bits, so a divisor with the highest bit set is not read as a negative number. As a DINT,
// 16#FFFF_FFFF would be -1, and the results would be -100 and 0.

FUNCTION main
    VAR
        divisor : UDINT := 16#FFFF_FFFF;
    END_VAR
    printf('100 / divisor: %u$N', 100 / divisor);
    printf('100 MOD divisor: %u$N', 100 MOD divisor);
    // CHECK:      100 / divisor: 0
    // CHECK-NEXT: 100 MOD divisor: 100
END_FUNCTION
