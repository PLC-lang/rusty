// RUN: %COMPILE %s && %RUN | %CHECK %s
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
    printf('%s$N', REF(DT_TO_STRING(dt_result)));                                   // CHECK: DT#2024-01-15-12:30:05

    // SUB_DT_TIME: DT - TIME -> DT (u32)
    dt1 := DT#2024-01-15-12:30:05;
    t1 := TIME#3s;
    dt_result := SUB_DT_TIME(dt1, t1);
    printf('%s$N', REF(DT_TO_STRING(dt_result)));                                   // CHECK: DT#2024-01-15-12:30:02

    // SUB_DT_DT: DT - DT -> TIME (u32)
    dt1 := DT#2024-01-15-12:30:10;
    dt2 := DT#2024-01-15-12:30:00;
    result := SUB_DT_DT(dt1, dt2);
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: T#10s

    // The operators carry out the same functions
    dt_result := dt1 + t1;
    printf('%s$N', REF(DT_TO_STRING(dt_result)));                                   // CHECK: DT#2024-01-15-12:30:13
    dt_result := dt1 - t1;
    printf('%s$N', REF(DT_TO_STRING(dt_result)));                                   // CHECK: DT#2024-01-15-12:30:07
    result := dt1 - dt2;
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: T#10s

    // The builtins carry out the same functions as the operators
    dt_result := ADD(dt1, t1, T#1m);
    printf('%s$N', REF(DT_TO_STRING(dt_result)));                                   // CHECK: DT#2024-01-15-12:31:13
    result := SUB(dt1, dt2);
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: T#10s

END_FUNCTION
