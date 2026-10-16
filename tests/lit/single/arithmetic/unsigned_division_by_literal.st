// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// A division of an unsigned operand of 32 or 64 bits by an integer literal is unsigned, like a
// division by a variable of the same type, also when the operand has the highest bit set.

FUNCTION main
    VAR
        udint_value : UDINT := 4_000_000_000;
        two : UDINT := 2;
        dword_value : DWORD := 16#FFFF_FFFF;
        ulint_value : ULINT := 18_446_744_073_709_551_615;
        lword_value : LWORD := 16#FFFF_FFFF_FFFF_FFFF;
    END_VAR
    printf('udint / two: %u$N', udint_value / two);
    printf('udint / 2: %u$N', udint_value / 2);
    printf('dword / 2: %u$N', dword_value / 2);
    printf('ulint / 2: %llu$N', ulint_value / 2);
    printf('lword / 2: %llu$N', lword_value / 2);
    // CHECK:      udint / two: 2000000000
    // CHECK-NEXT: udint / 2: 2000000000
    // CHECK-NEXT: dword / 2: 2147483647
    // CHECK-NEXT: ulint / 2: 9223372036854775807
    // CHECK-NEXT: lword / 2: 9223372036854775807
END_FUNCTION
