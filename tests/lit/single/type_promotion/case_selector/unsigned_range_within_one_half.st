// RUN: %COMPILE %s && %RUN | %CHECK %s
// Range labels that stay below or above the signed midpoint of an unsigned selector.
FUNCTION branch_usint : DINT
    VAR_INPUT
        x : USINT;
    END_VAR

    CASE x OF
        1..100: branch_usint := 1;
        129..254: branch_usint := 2;
        255: branch_usint := 3;
    ELSE
        branch_usint := 9;
    END_CASE
END_FUNCTION

FUNCTION branch_uint : DINT
    VAR_INPUT
        x : UINT;
    END_VAR

    CASE x OF
        1..30000: branch_uint := 1;
        32769..65534: branch_uint := 2;
        65535: branch_uint := 3;
    ELSE
        branch_uint := 9;
    END_CASE
END_FUNCTION

FUNCTION branch_udint : DINT
    VAR_INPUT
        x : UDINT;
    END_VAR

    CASE x OF
        1..2000000000: branch_udint := 1;
        2147483649..4294967294: branch_udint := 2;
        4294967295: branch_udint := 3;
    ELSE
        branch_udint := 9;
    END_CASE
END_FUNCTION

FUNCTION branch_ulint : DINT
    VAR_INPUT
        x : ULINT;
    END_VAR

    CASE x OF
        1..9000000000000000000: branch_ulint := 1;
        9223372036854775809..18446744073709551614: branch_ulint := 2;
        18446744073709551615: branch_ulint := 3;
    ELSE
        branch_ulint := 9;
    END_CASE
END_FUNCTION

FUNCTION main : DINT
    // USINT
    printf('USINT 50 -> %d$N', branch_usint(50));                   // CHECK: USINT 50 -> 1
    printf('USINT 200 -> %d$N', branch_usint(200));                 // CHECK-NEXT: USINT 200 -> 2
    printf('USINT 255 -> %d$N', branch_usint(255));                 // CHECK-NEXT: USINT 255 -> 3
    printf('USINT 0 -> %d$N', branch_usint(0));                     // CHECK-NEXT: USINT 0 -> 9

    // UINT
    printf('UINT 100 -> %d$N', branch_uint(100));                   // CHECK-NEXT: UINT 100 -> 1
    printf('UINT 40000 -> %d$N', branch_uint(40000));               // CHECK-NEXT: UINT 40000 -> 2
    printf('UINT 65535 -> %d$N', branch_uint(65535));               // CHECK-NEXT: UINT 65535 -> 3

    // UDINT
    printf('UDINT 100 -> %d$N', branch_udint(100));                 // CHECK-NEXT: UDINT 100 -> 1
    printf('UDINT 3000000000 -> %d$N', branch_udint(3000000000));   // CHECK-NEXT: UDINT 3000000000 -> 2
    printf('UDINT 4294967295 -> %d$N', branch_udint(4294967295));   // CHECK-NEXT: UDINT 4294967295 -> 3

    // ULINT
    printf('ULINT 100 -> %d$N', branch_ulint(100));                 // CHECK-NEXT: ULINT 100 -> 1
    printf('ULINT 2^63 + 1 -> %d$N', branch_ulint(9223372036854775809));   // CHECK-NEXT: ULINT 2^63 + 1 -> 2
    printf('ULINT max -> %d$N', branch_ulint(18446744073709551615));       // CHECK-NEXT: ULINT max -> 3
END_FUNCTION
