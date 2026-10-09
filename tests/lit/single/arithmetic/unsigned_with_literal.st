// RUN: %COMPILE %s && %RUN | %CHECK %s

// An integer literal takes the type of an unsigned operand of 32 or 64 bits, so the operation stays unsigned
FUNCTION main : DINT
    VAR
        udintValue : UDINT := 4000000000;
        dwordValue : DWORD := 16#FFFFFFFF;
        ulintValue : ULINT := 18446744073709551615;
        ulintResult : ULINT;
        usintValue : USINT := 0;
        dintValue : DINT := -7;
    END_VAR

    // CHECK: 2000000000
    printf('%u$N', udintValue / 2);
    // CHECK: 3
    printf('%u$N', udintValue MOD 7);
    // CHECK: 1
    printf('%u$N', 4000000001 MOD udintValue);
    // CHECK: 2147483647
    printf('%u$N', dwordValue / 2);
    // CHECK: 0
    printf('%u$N', dwordValue MOD 3);
    // CHECK: 9223372036854775807
    printf('%llu$N', ulintValue / 2);

    ulintResult := udintValue + 1;
    // CHECK: 4000000001
    printf('%llu$N', ulintResult);

    // Smaller unsigned and signed operands are not affected
    // CHECK: -1
    printf('%d$N', usintValue - 1);
    // CHECK: -3
    printf('%d$N', dintValue / 2);
END_FUNCTION
