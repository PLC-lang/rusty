// RUN: %COMPILE %s && %RUN | %CHECK %s
// Bit string divisions whose result does not depend on the type of the literal.
FUNCTION main : DINT
    VAR
        w : DWORD := 16#FFFF_FFFF;
        w2 : DWORD := 16#2;
        lw : LWORD := 16#FFFF_FFFF_FFFF_FFFF;
        qw : DWORD;
        ql : ULINT;
    END_VAR

    qw := w / w2;
    printf('DWORD max / DWORD 2 = %u$N', qw);               // CHECK: DWORD max / DWORD 2 = 2147483647
    ql := lw / 2;
    printf('LWORD max / 2 = %llu$N', ql);                   // CHECK-NEXT: LWORD max / 2 = 9223372036854775807
END_FUNCTION
