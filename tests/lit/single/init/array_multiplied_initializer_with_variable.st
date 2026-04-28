// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main : INT
    VAR
        arr : ARRAY[1..32] OF INT := [32(1)];
        x : INT;
    END_VAR

    x := 5;
    arr := [32(x)];


    // CHECK: 15
    printf('%d$N', arr[1] + arr[16] + arr[32]);
END_FUNCTION
