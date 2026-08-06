FUNCTION main : DINT
    // b := NOT a (source bubble ahead of the connector),
    // d := NOT c (sink bubble behind the continuation).
    negated_route.a := FALSE;
    negated_route.c := TRUE;
    negated_route();
    // CHECK: b = 1, d = 0
    printf('b = %d, d = %d$N', negated_route.b, negated_route.d);

    negated_route.a := TRUE;
    negated_route.c := FALSE;
    negated_route();
    // CHECK: b = 0, d = 1
    printf('b = %d, d = %d$N', negated_route.b, negated_route.d);
END_FUNCTION
