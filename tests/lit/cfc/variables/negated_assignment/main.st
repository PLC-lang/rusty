FUNCTION main : DINT
    // b1 := NOT a1 (source bubble), b2 := NOT a2 (sink bubble),
    // b3 := NOT NOT a3 (both bubbles cancel).
    negated_assignment.a1 := FALSE;
    negated_assignment.a2 := FALSE;
    negated_assignment.a3 := FALSE;
    negated_assignment();
    // CHECK: all false: b1 = 1, b2 = 1, b3 = 0
    printf('all false: b1 = %d, b2 = %d, b3 = %d$N', negated_assignment.b1, negated_assignment.b2, negated_assignment.b3);

    negated_assignment.a1 := TRUE;
    negated_assignment.a2 := TRUE;
    negated_assignment.a3 := TRUE;
    negated_assignment();
    // CHECK: all true: b1 = 0, b2 = 0, b3 = 1
    printf('all true: b1 = %d, b2 = %d, b3 = %d$N', negated_assignment.b1, negated_assignment.b2, negated_assignment.b3);
END_FUNCTION
