// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main : DINT
    VAR
        arr : ARRAY[0..2] OF STRING := ['apple', 'banana', 'cherry'];
    END_VAR

    // CHECK: apple
    printf('%s$N', arr[0]);

    // CHECK: banana
    printf('%s$N', arr[1]);

    // CHECK: cherry
    printf('%s$N', arr[2]);
END_FUNCTION
