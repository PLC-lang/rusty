// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test DATE arithmetic operations return correct types and values

FUNCTION main
    VAR
        date1 : DATE;
        date2 : DATE;
        result : TIME;
    END_VAR

    // SUB_DATE_DATE: DATE - DATE -> TIME (u32)
    date1 := DATE#2024-01-15;
    date2 := DATE#2024-01-10;
    result := SUB_DATE_DATE(date1, date2);
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: 5d

END_FUNCTION
