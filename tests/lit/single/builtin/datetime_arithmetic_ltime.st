// RUN: %COMPILE %s && %RUN | %CHECK %s
// Test LTIME arithmetic operations return correct types and values

FUNCTION main
    VAR
        lt1 : LTIME;
        lt2 : LTIME;
        lresult : LTIME;
    END_VAR

    // ADD_LTIME: LTIME + LTIME -> LTIME (i64)
    lt1 := LTIME#5s;
    lt2 := LTIME#3s;
    lresult := ADD_LTIME(lt1, lt2);
    printf('%s$N', REF(LTIME_TO_STRING(lresult)));                                  // CHECK: LTIME#8s

    // SUB_LTIME: LTIME - LTIME -> LTIME (i64)
    lt1 := LTIME#10s;
    lt2 := LTIME#3s;
    lresult := SUB_LTIME(lt1, lt2);
    printf('%s$N', REF(LTIME_TO_STRING(lresult)));                                  // CHECK: LTIME#7s

    // The operators carry out the same functions
    lresult := lt1 + lt2;
    printf('%s$N', REF(LTIME_TO_STRING(lresult)));                                  // CHECK: LTIME#13s
    lresult := lt1 - lt2;
    printf('%s$N', REF(LTIME_TO_STRING(lresult)));                                  // CHECK: LTIME#7s
    lresult := lt1 * 2;
    printf('%s$N', REF(LTIME_TO_STRING(lresult)));                                  // CHECK: LTIME#20s
    lresult := 2 * lt1;
    printf('%s$N', REF(LTIME_TO_STRING(lresult)));                                  // CHECK: LTIME#20s
    lresult := lt1 / 4;
    printf('%s$N', REF(LTIME_TO_STRING(lresult)));                                  // CHECK: LTIME#2s500ms

    // The builtins carry out the same functions as the operators
    lresult := ADD(lt1, lt2, LTIME#1s);
    printf('%s$N', REF(LTIME_TO_STRING(lresult)));                                  // CHECK: LTIME#14s
    lresult := MUL(lt1, 1.5);
    printf('%s$N', REF(LTIME_TO_STRING(lresult)));                                  // CHECK: LTIME#15s

END_FUNCTION
