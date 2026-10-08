// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Unsigned operands of 32 and 64 bits compare as unsigned values, also above 2^31 and 2^63
FUNCTION main : DINT
    VAR
        udintBig : UDINT := 4000000000;
        udintSmall : UDINT := 5;
        ulintBig : ULINT := 18446744073709551615;
        ulintSmall : ULINT := 1;
        dwordBig : DWORD := 16#FFFFFFFF;
        dwordSmall : DWORD := 1;
        lwordBig : LWORD := 16#FFFFFFFFFFFFFFFF;
        lwordSmall : LWORD := 1;
        dintNegative : DINT := -1;
        dintPositive : DINT := 1;
    END_VAR

    // CHECK: 1 0 1 0
    printf('%d %d %d %d$N', udintBig > udintSmall, udintBig < udintSmall, udintBig >= udintSmall, udintBig <= udintSmall);
    // CHECK: 1 0 1 0
    printf('%d %d %d %d$N', ulintBig > ulintSmall, ulintBig < ulintSmall, ulintBig >= ulintSmall, ulintBig <= ulintSmall);
    // CHECK: 1 0 1 0
    printf('%d %d %d %d$N', dwordBig > dwordSmall, dwordBig < dwordSmall, dwordBig >= dwordSmall, dwordBig <= dwordSmall);
    // CHECK: 1 0 1 0
    printf('%d %d %d %d$N', lwordBig > lwordSmall, lwordBig < lwordSmall, lwordBig >= lwordSmall, lwordBig <= lwordSmall);

    // Signed operands still compare as signed values
    // CHECK: 0 1 0 1
    printf('%d %d %d %d$N', dintNegative > dintPositive, dintNegative < dintPositive, dintNegative >= dintPositive, dintNegative <= dintPositive);
END_FUNCTION
