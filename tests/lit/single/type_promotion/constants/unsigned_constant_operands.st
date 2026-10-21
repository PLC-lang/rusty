// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// Compares, divisions and MOD on UDINT constants follow the same signed rules as on variables.
VAR_GLOBAL CONSTANT
    big : UDINT := 4000000000;
    small : UDINT := 5;
END_VAR

VAR_GLOBAL
    half : UDINT := big / 2;
    rest : UDINT := big MOD 7;
END_VAR

FUNCTION main : DINT
    VAR
        q : UDINT;
    END_VAR

    // Folded initializers
    printf('initializer big / 2 = %u$N', half);             // CHECK: initializer big / 2 = 2000000000
    printf('initializer big MOD 7 = %u$N', rest);           // CHECK-NEXT: initializer big MOD 7 = 3

    // Body expressions on constants
    q := big / 2;
    printf('body big / 2 = %u$N', q);                       // CHECK-NEXT: body big / 2 = 2000000000
    printf('big > small = %d$N', big > small);              // CHECK-NEXT: big > small = 1
    printf('big > 5 = %d$N', big > 5);                      // CHECK-NEXT: big > 5 = 1
END_FUNCTION
