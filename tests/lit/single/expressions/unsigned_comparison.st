// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE Direction : (backward := -1, stop := 0, forward := 1); END_TYPE

// Integers compare by their values, also unsigned values above 2^31 and 2^63 and negative signed values
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
        direction : Direction := backward;
        address : LWORD;
    END_VAR

    // CHECK: 1 0 1 0
    printf('%d %d %d %d$N', udintBig > udintSmall, udintBig < udintSmall, udintBig >= udintSmall, udintBig <= udintSmall);
    // CHECK: 1 0 1 0
    printf('%d %d %d %d$N', ulintBig > ulintSmall, ulintBig < ulintSmall, ulintBig >= ulintSmall, ulintBig <= ulintSmall);
    // CHECK: 1 0 1 0
    printf('%d %d %d %d$N', dwordBig > dwordSmall, dwordBig < dwordSmall, dwordBig >= dwordSmall, dwordBig <= dwordSmall);
    // CHECK: 1 0 1 0
    printf('%d %d %d %d$N', lwordBig > lwordSmall, lwordBig < lwordSmall, lwordBig >= lwordSmall, lwordBig <= lwordSmall);

    // Unsigned operands compared with a literal
    // CHECK: 1 0 1 0
    printf('%d %d %d %d$N', udintBig > 5, udintBig < 5, ulintBig >= 5, ulintBig <= 5);

    // Signed operands still compare as signed values
    // CHECK: 0 1 0 1
    printf('%d %d %d %d$N', dintNegative > dintPositive, dintNegative < dintPositive, dintNegative >= dintPositive, dintNegative <= dintPositive);
    // CHECK: 1 0
    printf('%d %d$N', direction < stop, direction > forward);

    // A negative signed value is less than every unsigned value and equal to none
    // CHECK: 1 0 1 0
    printf('%d %d %d %d$N', ulintSmall > dintNegative, ulintSmall < dintNegative, ulintBig >= -1, ulintBig <= -1);
    // CHECK: 1 0 1 0
    printf('%d %d %d %d$N', udintBig > dintNegative, udintBig < dintNegative, dintNegative < lwordBig, dintNegative > lwordBig);
    // CHECK: 0 1 0 1
    printf('%d %d %d %d$N', dwordBig = dintNegative, dwordBig <> dintNegative, ulintBig = -1, ulintBig <> -1);
    // CHECK: 0 1 0
    printf('%d %d %d$N', lwordBig = dintNegative, dintNegative <> lwordBig, udintBig = -294967296);

    // Equal values compare as equal
    // CHECK: 1 0 1 1
    printf('%d %d %d %d$N', udintBig = 4000000000, udintBig <> 4000000000, dwordSmall = dintPositive, ulintBig = lwordBig);

    // Addresses compare as unsigned values
    address := ADR(dintPositive);
    // CHECK: 1 1 0
    printf('%d %d %d$N', address = ADR(dintPositive), address >= ADR(dintPositive), address < ADR(dintPositive));
END_FUNCTION
