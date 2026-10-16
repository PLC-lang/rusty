// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// An unsigned operand of 32 or 64 bits compares with an integer literal as an unsigned value, also
// when the operand has the highest bit set.

FUNCTION main
    VAR
        udint_high : UDINT := 16#8000_0000;
        dword_high : DWORD := 16#8000_0000;
        ulint_high : ULINT := 16#8000_0000_0000_0000;
        lword_high : LWORD := 16#8000_0000_0000_0000;
    END_VAR
    printf('udint > 5: %d$N', udint_high > 5);
    printf('udint < 5: %d$N', udint_high < 5);
    printf('dword > 16#7FFF_FFFF: %d$N', dword_high > 16#7FFF_FFFF);
    printf('ulint > 5: %d$N', ulint_high > 5);
    printf('lword > 5: %d$N', lword_high > 5);
    // CHECK:      udint > 5: 1
    // CHECK-NEXT: udint < 5: 0
    // CHECK-NEXT: dword > 16#7FFF_FFFF: 1
    // CHECK-NEXT: ulint > 5: 1
    // CHECK-NEXT: lword > 5: 1
END_FUNCTION
