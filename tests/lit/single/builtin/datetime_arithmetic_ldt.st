// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test LDT arithmetic operations return correct types and values

FUNCTION main
    VAR
        ldt1 : LDT;
        ldt2 : LDT;
        lt1 : LTIME;
        lresult : LTIME;
        ldt_result : LDT;
    END_VAR

    // ADD_LDT_LTIME: LDT + LTIME -> LDT (i64, nanosecond precision)
    ldt1 := LDT#2024-01-15-12:30:00;
    lt1 := LTIME#5s;
    ldt_result := ADD_LDT_LTIME(ldt1, lt1);
    printf('%s$N', REF(LDT_TO_STRING(ldt_result)));                                 // CHECK: 2024-01-15-12:30:05

    // SUB_LDT_LTIME: LDT - LTIME -> LDT (i64)
    ldt1 := LDT#2024-01-15-12:30:05;
    lt1 := LTIME#3s;
    ldt_result := SUB_LDT_LTIME(ldt1, lt1);
    printf('%s$N', REF(LDT_TO_STRING(ldt_result)));                                 // CHECK: 2024-01-15-12:30:02

    // SUB_LDT_LDT: LDT - LDT -> LTIME (i64, result is difference in nanoseconds)
    ldt1 := LDT#2024-01-15-12:30:10;
    ldt2 := LDT#2024-01-15-12:30:00;
    lresult := SUB_LDT_LDT(ldt1, ldt2);
    printf('%s$N', REF(LTIME_TO_STRING(lresult)));                                  // CHECK: 10s

END_FUNCTION
