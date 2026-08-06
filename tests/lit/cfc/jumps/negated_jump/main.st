FUNCTION main : DINT
    // The source bubble inverts the guard: a false guard takes the jump,
    // skipping the assignment.
    negated_jump.guard := FALSE;
    negated_jump.result := 0;
    negated_jump();
    // CHECK: result = 0
    printf('result = %d$N', negated_jump.result);

    // A true guard does not jump, so the assignment runs.
    negated_jump.guard := TRUE;
    negated_jump.result := 0;
    negated_jump();
    // CHECK: result = 42
    printf('result = %d$N', negated_jump.result);
END_FUNCTION
