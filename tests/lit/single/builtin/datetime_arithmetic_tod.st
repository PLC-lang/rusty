// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test TOD arithmetic operations return correct types and values

FUNCTION main
    VAR
        tod1 : TOD;
        tod2 : TOD;
        t1 : TIME;
        result : TIME;
    END_VAR

    // ADD_TOD_TIME: TOD + TIME -> TOD (wraps at day boundary)
    tod1 := TOD#23:59:50;
    t1 := TIME#20s;
    result := ADD_TOD_TIME(tod1, t1);
    printf('%s$N', REF(TOD_TO_STRING(result)));                                     // CHECK: 00:00:10

    // SUB_TOD_TIME: TOD - TIME -> TOD
    tod1 := TOD#00:01:00;
    t1 := TIME#30s;
    result := SUB_TOD_TIME(tod1, t1);
    printf('%s$N', REF(TOD_TO_STRING(result)));                                     // CHECK: 00:00:30

    // SUB_TOD_TOD: TOD - TOD -> TIME (u32)
    tod1 := TOD#10:30:00;
    tod2 := TOD#10:20:00;
    result := SUB_TOD_TOD(tod1, tod2);
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: 10m

    // Test ADD_TOD_TIME with simple arithmetic
    tod1 := TOD#12:00:00;
    t1 := TIME#1m;
    result := ADD_TOD_TIME(tod1, t1);
    printf('%s$N', REF(TOD_TO_STRING(result)));                                     // CHECK: 12:01:00

    // Test SUB_TOD_TIME with simple case
    tod1 := TOD#12:05:00;
    t1 := TIME#1m;
    result := SUB_TOD_TIME(tod1, t1);
    printf('%s$N', REF(TOD_TO_STRING(result)));                                     // CHECK: 12:04:00

END_FUNCTION
