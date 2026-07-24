// RUN: (%COMPILE %s && %RUN) | %CHECK %s
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
    printf('%s$N', REF(LTIME_TO_STRING(lresult)));                                  // CHECK: 8s

    // SUB_LTIME: LTIME - LTIME -> LTIME (i64)
    lt1 := LTIME#10s;
    lt2 := LTIME#3s;
    lresult := SUB_LTIME(lt1, lt2);
    printf('%s$N', REF(LTIME_TO_STRING(lresult)));                                  // CHECK: 7s

END_FUNCTION
