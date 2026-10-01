// RUN: %COMPILE %s && %RUN | %CHECK %s
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
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: T#8s

    // SUB_TIME: TIME - TIME -> TIME (u32)
    t1 := TIME#10s;
    t2 := TIME#3s;
    result := SUB_TIME(t1, t2);
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: T#7s

    // The operators carry out the same functions
    result := t1 + t2;
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: T#13s
    result := t1 - t2;
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: T#7s
    result := t1 * 3;
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: T#30s
    result := 2 * t1;
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: T#20s
    result := t1 / 4;
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: T#2s500ms
    result := t1 + 5;
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: T#10s5ms

    // The builtins carry out the same functions as the operators
    result := ADD(t1, t2, T#1s);
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: T#14s
    result := SUB(t1, t2);
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: T#7s
    result := MUL(t1, 1.5);
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: T#15s
    result := DIV(t1, 2.5);
    printf('%s$N', REF(TIME_TO_STRING(result)));                                    // CHECK: T#4s

END_FUNCTION
