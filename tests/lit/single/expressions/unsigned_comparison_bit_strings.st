// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// Two bit strings of the same type compare as unsigned values, also when a value has the highest
// bit set.

FUNCTION main
    VAR
        byte_low : BYTE := 1;
        byte_high : BYTE := 16#FF;
        word_low : WORD := 1;
        word_high : WORD := 16#FFFF;
        dword_low : DWORD := 1;
        dword_high : DWORD := 16#FFFF_FFFF;
        lword_low : LWORD := 1;
        lword_high : LWORD := 16#FFFF_FFFF_FFFF_FFFF;
    END_VAR
    printf('byte: %d$N', byte_high > byte_low);
    printf('word: %d$N', word_high > word_low);
    printf('dword: %d$N', dword_high > dword_low);
    printf('lword: %d$N', lword_high > lword_low);
    // CHECK:      byte: 1
    // CHECK-NEXT: word: 1
    // CHECK-NEXT: dword: 1
    // CHECK-NEXT: lword: 1
END_FUNCTION
