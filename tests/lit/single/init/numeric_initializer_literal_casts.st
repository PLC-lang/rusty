// RUN: %COMPILE %s && %RUN | %CHECK %s

// BOOL and REAL literals are converted to the declared numeric type, both as scalar initializers
// and as elements of array literals in declarations and in the body.

FUNCTION main : DINT
VAR
    a : DINT := TRUE;
    b : INT := TRUE;
    c : REAL := TRUE;
    d : LREAL := FALSE;

    dints : ARRAY[0..1] OF DINT := [TRUE, FALSE];
    lints : ARRAY[0..1] OF LINT := [FALSE, TRUE];
    reals : ARRAY[0..1] OF REAL := [TRUE, FALSE];
    lreals : ARRAY[0..1] OF LREAL := [FALSE, TRUE];
    ints : ARRAY[0..1] OF INT := [1.5, 2];
    later : ARRAY[0..1] OF REAL;
END_VAR
    later := [TRUE, FALSE];

    // CHECK: scalars: 1 1 1.000000 0.000000
    printf('scalars: %d %d %f %f$N', a, b, c, d);
    // CHECK: dints: 1 0
    printf('dints: %d %d$N', dints[0], dints[1]);
    // CHECK: lints: 0 1
    printf('lints: %d %d$N', lints[0], lints[1]);
    // CHECK: reals: 1.000000 0.000000
    printf('reals: %f %f$N', reals[0], reals[1]);
    // CHECK: lreals: 0.000000 1.000000
    printf('lreals: %f %f$N', lreals[0], lreals[1]);
    // CHECK: ints: 1 2
    printf('ints: %d %d$N', ints[0], ints[1]);
    // CHECK: later: 1.000000 0.000000
    printf('later: %f %f$N', later[0], later[1]);
END_FUNCTION
