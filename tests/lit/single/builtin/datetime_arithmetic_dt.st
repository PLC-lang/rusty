// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test DT arithmetic operations return correct types and values

FUNCTION main
    VAR
        dt1 : DT;
        dt2 : DT;
        t1 : TIME;
        result : TIME;
        dt_result : DT;
    END_VAR

    // ADD_DT_TIME: DT + TIME -> DT (u32)
    dt1 := DT#2024-01-15-12:30:00;
    t1 := TIME#5s;
    dt_result := ADD_DT_TIME(dt1, t1);
    printf('%s$N', REF(DT_TO_STRING(dt_result)));                                   // CHECK: 2024-01-15-12:30:05

    // SUB_DT_TIME: DT - TIME -> DT (u32)
    dt1 := DT#2024-01-15-12:30:05;
    t1 := TIME#3s;
    dt_result := SUB_DT_TIME(dt1, t1);
    printf('%s$N', REF(DT_TO_STRING(dt_result)));                                   // CHECK: 2024-01-15-12:30:02

    // SUB_DT_DT: DT - DT -> TIME (u32)
    dt1 := DT#2024-01-15-12:30:10;
    dt2 := DT#2024-01-15-12:30:00;
    result := SUB_DT_DT(dt1, dt2);
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: 10s

END_FUNCTION
