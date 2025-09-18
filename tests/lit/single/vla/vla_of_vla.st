// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION foo
VAR_INPUT
    arr1: ARRAY[*] OF ARRAY[*] OF INT;
END_VAR
VAR_IN_OUT
    arr2: ARRAY[*] OF ARRAY[*] OF INT;
END_VAR
VAR
    i, j : INT;
END_VAR
    FOR i := LOWER_BOUND(arr1, 1) TO UPPER_BOUND(arr1, 1) DO
        FOR j := LOWER_BOUND(arr1[i], 1) TO UPPER_BOUND(arr1[i], 1) DO
            arr1[i][j] := j;
            arr2[i][j] := j + 1;
        END_FOR
    END_FOR
END_FUNCTION

FUNCTION main
VAR
    arr1: ARRAY[0..1] OF ARRAY[0..1] OF INT;
    arr2: ARRAY[0..1] OF ARRAY[0..1] OF INT;
    i, j : INT;
END_VAR
    foo(arr1, arr2);

    FOR i := 0 TO 1 DO
        FOR j := 0 TO 1 DO
            printf('%d$N', arr1[i][j]);
        END_FOR
    END_FOR
    // CHECK: arr1[0][0] = 0
    // CHECK-NEXT: arr1[0][1] = 1
    // CHECK-NEXT: arr1[1][0] = 2
    // CHECK-NEXT: arr1[1][1] = 3

    FOR i := 0 TO 1 DO
        FOR j := 0 TO 1 DO
            printf('%d$N', arr2[i][j]);
        END_FOR
    END_FOR
    // CHECK: arr2[0][0] = 1
    // CHECK-NEXT: arr2[0][1] = 2
    // CHECK-NEXT: arr2[1][0] = 3
    // CHECK-NEXT: arr2[1][1] = 4
END_FUNCTION
