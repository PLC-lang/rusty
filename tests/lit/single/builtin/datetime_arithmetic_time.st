// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test TIME arithmetic operations return correct types and values

FUNCTION main
    VAR
        t1 : TIME;
        t2 : TIME;
        result : TIME;
    END_VAR

    // ADD_TIME: TIME + TIME -> TIME (u32)
    t1 := TIME#5s;
    t2 := TIME#3s;
    result := ADD_TIME(t1, t2);
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: 8s

    // SUB_TIME: TIME - TIME -> TIME (u32)
    t1 := TIME#10s;
    t2 := TIME#3s;
    result := SUB_TIME(t1, t2);
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: 7s

    // SUB_TIME with wrapping (u32 underflow, printed as signed)
    t1 := TIME#2s;
    t2 := TIME#5s;
    result := SUB_TIME(t1, t2);
    printf('%d$N', result);                                                         // CHECK: -3000

END_FUNCTION
