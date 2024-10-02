// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    arr : ARRAY [0..10] OF INT;
END_VAR

FUNCTION write_to_array
VAR_EXTERNAL CONSTANT
    arr : ARRAY [0..10] OF INT;
END_VAR
VAR
    i: DINT;
END_VAR
    FOR i := 0 TO 10 DO
        arr[i] := i;
    END_FOR;
END_FUNCTION

FUNCTION main : DINT
    write_to_array();
    printf('%d$N', arr[0]); // CHECK: 0
    printf('%d$N', arr[1]); // CHECK: 1
    printf('%d$N', arr[2]); // CHECK: 2
    printf('%d$N', arr[3]); // CHECK: 3
    printf('%d$N', arr[4]); // CHECK: 4
    printf('%d$N', arr[5]); // CHECK: 5
    printf('%d$N', arr[6]); // CHECK: 6
    printf('%d$N', arr[7]); // CHECK: 7
    printf('%d$N', arr[8]); // CHECK: 8
    printf('%d$N', arr[9]); // CHECK: 9
    printf('%d$N', arr[10]); // CHECK: 10
END_FUNCTION