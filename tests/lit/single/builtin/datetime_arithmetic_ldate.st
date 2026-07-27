// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test LDATE arithmetic operations return correct types and values

FUNCTION main
    VAR
        ldate1 : LDATE;
        ldate2 : LDATE;
        lresult : LTIME;
    END_VAR

    // SUB_LDATE_LDATE: LDATE - LDATE -> LTIME (i64)
    ldate1 := LDATE#2024-01-15;
    ldate2 := LDATE#2024-01-10;
    lresult := SUB_LDATE_LDATE(ldate1, ldate2);
    printf('%s$N', REF(LTIME_TO_STRING(lresult)));                                  // CHECK: 5d

END_FUNCTION
