// RUN: %COMPILE %s && %RUN | %CHECK %s
// An untyped integer literal takes the type of a DWORD operand, so the division is unsigned.
FUNCTION main : DINT
    VAR
        w : DWORD := 16#FFFF_FFFF;
        q : DWORD;
    END_VAR

    q := w / 2;
    printf('DWORD 16#FFFF_FFFF / 2 = %u$N', q);             // CHECK: DWORD 16#FFFF_FFFF / 2 = 2147483647
    q := w / 16#10;
    printf('DWORD 16#FFFF_FFFF / 16#10 = %u$N', q);         // CHECK-NEXT: DWORD 16#FFFF_FFFF / 16#10 = 268435455
    q := w MOD 3;
    printf('DWORD 16#FFFF_FFFF MOD 3 = %u$N', q);           // CHECK-NEXT: DWORD 16#FFFF_FFFF MOD 3 = 0
    q := w MOD 16#100;
    printf('DWORD 16#FFFF_FFFF MOD 16#100 = %u$N', q);      // CHECK-NEXT: DWORD 16#FFFF_FFFF MOD 16#100 = 255
END_FUNCTION
