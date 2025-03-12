// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION foo: ARRAY[1..5] OF DINT
    foo := [5, 4, 3, 2, 1];
END_FUNCTION

FUNCTION alwaysOne: DINT
    alwaysOne := 1;
END_FUNCTION

FUNCTION main
    VAR
        value: DINT;
        one: DINT := 1;
        two: DINT := 2;
    END_VAR

    // CHECK: 5
    value := foo()[1];
    printf('%d$N', value);

    // CHECK: 3
    value := foo()[1 + 2];
    printf('%d$N', value);

    // CHECK: 5
    value := foo()[one];
    printf('%d$N', value);

    // CHECK: 3
    value := foo()[one + two];
    printf('%d$N', value);

    // CHECK: 5
    value := foo()[alwaysOne()];
    printf('%d$N', value);
END_FUNCTION
