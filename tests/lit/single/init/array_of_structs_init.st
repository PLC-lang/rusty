// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE Point : STRUCT
    x : DINT;
    y : DINT;
END_STRUCT END_TYPE

FUNCTION main : DINT
    VAR
        points : ARRAY[0..1] OF Point := [(x := 1, y := 2), (x := 3, y := 4)];
    END_VAR

    // CHECK: 1
    printf('%d$N', points[0].x);

    // CHECK: 2
    printf('%d$N', points[0].y);

    // CHECK: 3
    printf('%d$N', points[1].x);

    // CHECK: 4
    printf('%d$N', points[1].y);
END_FUNCTION
