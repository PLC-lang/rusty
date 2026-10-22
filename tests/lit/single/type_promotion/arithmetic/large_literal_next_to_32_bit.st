// RUN: %COMPILE %s && %RUN | %CHECK %s
// An untyped literal that does not fit DINT moves a UDINT or DWORD operand to ULINT or LWORD, so the result stays exact.
VAR_GLOBAL CONSTANT
    cu : UDINT := 5;
    cw : DWORD := 16#5;
END_VAR

FUNCTION main : DINT
    VAR
        u : UDINT := 5;
        w : DWORD := 16#5;
        r : ULINT;
    END_VAR
    VAR CONSTANT
        lc : UDINT := 5;
    END_VAR

    // Variables
    r := u + 16#FFFFFFFF;
    printf('UDINT 5 + 16#FFFFFFFF = %llu$N', r);            // CHECK: UDINT 5 + 16#FFFFFFFF = 4294967300
    r := u + 3000000000;
    printf('UDINT 5 + 3000000000 = %llu$N', r);             // CHECK-NEXT: UDINT 5 + 3000000000 = 3000000005
    r := 3000000000 * u;
    printf('3000000000 * UDINT 5 = %llu$N', r);             // CHECK-NEXT: 3000000000 * UDINT 5 = 15000000000
    r := w + 16#FFFFFFFF;
    printf('DWORD 5 + 16#FFFFFFFF = %llu$N', r);            // CHECK-NEXT: DWORD 5 + 16#FFFFFFFF = 4294967300
    printf('UDINT 5 < 16#1_0000_0000 = %d$N', u < 16#1_0000_0000);  // CHECK-NEXT: UDINT 5 < 16#1_0000_0000 = 1

    // Constants
    r := cu + 16#FFFFFFFF;
    printf('constant UDINT 5 + 16#FFFFFFFF = %llu$N', r);   // CHECK-NEXT: constant UDINT 5 + 16#FFFFFFFF = 4294967300
    r := lc + 3000000000;
    printf('local constant UDINT 5 + 3000000000 = %llu$N', r);  // CHECK-NEXT: local constant UDINT 5 + 3000000000 = 3000000005
    r := cw + 16#FFFFFFFF;
    printf('constant DWORD 5 + 16#FFFFFFFF = %llu$N', r);   // CHECK-NEXT: constant DWORD 5 + 16#FFFFFFFF = 4294967300
END_FUNCTION
