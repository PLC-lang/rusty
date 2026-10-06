// RUN: %COMPILE %s && %RUN | %CHECK %s
// A variable length array output is not reset at entry: it keeps the bounds and the data
// pointer the caller passed, so the function can write into the caller's array.
FUNCTION fill
    VAR_OUTPUT
        arr : ARRAY[*] OF INT;
    END_VAR
    VAR
        i : DINT;
    END_VAR
    FOR i := LOWER_BOUND(arr, 1) TO UPPER_BOUND(arr, 1) DO
        arr[i] := i * 10;
    END_FOR
END_FUNCTION

FUNCTION main
    VAR
        data : ARRAY[2..4] OF INT;
    END_VAR
    fill(arr => data);
    printf('%d %d %d$N', data[2], data[3], data[4]);
    // CHECK: 20 30 40
END_FUNCTION
