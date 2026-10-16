// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// MOD of an unsigned operand of 32 or 64 bits and an integer literal is unsigned, like MOD with a
// variable of the same type, also when the operand has the highest bit set.

FUNCTION main
    VAR
        udint_value : UDINT := 4_000_000_000;
        seven : UDINT := 7;
        dword_value : DWORD := 16#FFFF_FFFF;
        ulint_value : ULINT := 18_446_744_073_709_551_615;
        lword_value : LWORD := 16#FFFF_FFFF_FFFF_FFFF;
    END_VAR
    printf('udint MOD seven: %u$N', udint_value MOD seven);
    printf('udint MOD 7: %u$N', udint_value MOD 7);
    printf('dword MOD 3: %u$N', dword_value MOD 3);
    printf('ulint MOD 10: %llu$N', ulint_value MOD 10);
    printf('lword MOD 10: %llu$N', lword_value MOD 10);
    // CHECK:      udint MOD seven: 3
    // CHECK-NEXT: udint MOD 7: 3
    // CHECK-NEXT: dword MOD 3: 0
    // CHECK-NEXT: ulint MOD 10: 5
    // CHECK-NEXT: lword MOD 10: 5
END_FUNCTION
