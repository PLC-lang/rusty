// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// An untyped integer literal is DINT, so a UDINT divided by it is converted to DINT and the division is signed.
FUNCTION main : DINT
    VAR
        a : UDINT := 4000000000;
        q : UDINT;
    END_VAR

    q := a / 2;
    printf('UDINT 4e9 / 2 = %u$N', q);                      // CHECK: UDINT 4e9 / 2 = 2000000000
    q := a / 3;
    printf('UDINT 4e9 / 3 = %u$N', q);                      // CHECK-NEXT: UDINT 4e9 / 3 = 1333333333
    q := a / 2147483647;
    printf('UDINT 4e9 / 2147483647 = %u$N', q);             // CHECK-NEXT: UDINT 4e9 / 2147483647 = 1
    q := a MOD 7;
    printf('UDINT 4e9 MOD 7 = %u$N', q);                    // CHECK-NEXT: UDINT 4e9 MOD 7 = 3
    q := a MOD 2147483647;
    printf('UDINT 4e9 MOD 2147483647 = %u$N', q);           // CHECK-NEXT: UDINT 4e9 MOD 2147483647 = 1852516353
    q := (a / 2) MOD 7;
    printf('(UDINT 4e9 / 2) MOD 7 = %u$N', q);              // CHECK-NEXT: (UDINT 4e9 / 2) MOD 7 = 5
END_FUNCTION
