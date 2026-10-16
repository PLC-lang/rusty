// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// Expressions with three and four operands compare an unsigned operand of 32 bits as an unsigned
// value in every comparison, with literals and with other unsigned operands.

FUNCTION main
    VAR
        a : UDINT := 16#8000_0000;
        b : UDINT := 5;
        c : UDINT := 1;
        d : UDINT := 0;
    END_VAR
    printf('a > 1 AND a < 16#FFFF_FFFF: %d$N', a > 1 AND a < 16#FFFF_FFFF);
    printf('a > b AND b > c AND c > d: %d$N', a > b AND b > c AND c > d);
    printf('a >= 16#8000_0000 OR a < 5: %d$N', a >= 16#8000_0000 OR a < 5);
    printf('((a > b) < c) > d: %d$N', ((a > b) < c) > d);
    printf('((a > b) = (b < a)) > d: %d$N', ((a > b) = (b < a)) > d);
    // CHECK:      a > 1 AND a < 16#FFFF_FFFF: 1
    // CHECK-NEXT: a > b AND b > c AND c > d: 1
    // CHECK-NEXT: a >= 16#8000_0000 OR a < 5: 1
    // CHECK-NEXT: ((a > b) < c) > d: 0
    // CHECK-NEXT: ((a > b) = (b < a)) > d: 1
END_FUNCTION
