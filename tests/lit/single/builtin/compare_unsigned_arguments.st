// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// GT, LT, GE and LE compare two unsigned arguments as unsigned values, like the operators do.

FUNCTION main
    VAR
        small : UDINT := 5;
        large : UDINT := 4000000000;
    END_VAR
    printf('GT: %d$N', GT(large, small));
    printf('LT: %d$N', LT(small, large));
    printf('GE: %d$N', GE(large, small));
    printf('LE: %d$N', LE(small, large));
    // CHECK:      GT: 1
    // CHECK-NEXT: LT: 1
    // CHECK-NEXT: GE: 1
    // CHECK-NEXT: LE: 1
END_FUNCTION
