// RUN: %COMPILE %s && %RUN | %CHECK %s

// REAL literals are converted to the declared integer type, both as scalar initializers and as
// elements of array literals in declarations and in the body.

FUNCTION main : DINT
VAR
    a : INT := 3.5;
    b : DINT := 2.25;
    ints : ARRAY[0..1] OF INT := [1.5, 2];
    dints : ARRAY[0..1] OF DINT := [2.5, 3.75];
    reals : ARRAY[0..1] OF REAL := [1, 2];
    later : ARRAY[0..1] OF INT;
END_VAR
    later := [4.5, 5];

    // CHECK: scalars: 3 2
    printf('scalars: %d %d$N', a, b);
    // CHECK: ints: 1 2
    printf('ints: %d %d$N', ints[0], ints[1]);
    // CHECK: dints: 2 3
    printf('dints: %d %d$N', dints[0], dints[1]);
    // CHECK: reals: 1.000000 2.000000
    printf('reals: %f %f$N', reals[0], reals[1]);
    // CHECK: later: 4 5
    printf('later: %d %d$N', later[0], later[1]);
END_FUNCTION
