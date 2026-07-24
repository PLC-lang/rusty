// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test LTOD arithmetic operations return correct types and values

FUNCTION main
    VAR
        ltod1 : LTOD;
        ltod2 : LTOD;
        lt1 : LTIME;
        lresult : LTIME;
        ltod_result : LTOD;
    END_VAR

    // ADD_LTOD_LTIME: LTOD + LTIME -> LTOD (wraps at day boundary)
    ltod1 := LTOD#23:59:50;
    lt1 := LTIME#20s;
    ltod_result := ADD_LTOD_LTIME(ltod1, lt1);
    printf('%s$N', REF(LTOD_TO_STRING(ltod_result)));                               // CHECK: 00:00:10

    // SUB_LTOD_LTIME: LTOD - LTIME -> LTOD
    ltod1 := LTOD#00:01:00;
    lt1 := LTIME#30s;
    ltod_result := SUB_LTOD_LTIME(ltod1, lt1);
    printf('%s$N', REF(LTOD_TO_STRING(ltod_result)));                               // CHECK: 00:00:30

    // SUB_LTOD_LTOD: LTOD - LTOD -> LTIME (i64)
    ltod1 := LTOD#10:30:00;
    ltod2 := LTOD#10:20:00;
    lresult := SUB_LTOD_LTOD(ltod1, ltod2);
    printf('%s$N', REF(LTIME_TO_STRING(lresult)));                                  // CHECK: 10m

END_FUNCTION
